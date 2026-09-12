<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { defaultParameterValues, findProcess, findMode, validateRequest } from "../processes";
import type { InspectedFile, ParameterDefinition, ParameterValue } from "../processes";
import { requestCommandPreview, type CommandPreview } from "../services/preview";
import { useRoute, useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import {
  allowArtifactPlayback,
  cancelRun,
  enqueueProcess,
  getRunSnapshot,
  revealArtifact,
  suggestOutputPath,
  type RunStatus,
} from "../services/runtime";
import ProcessInputSection from "./process/ProcessInputSection.vue";
import ProcessModeSection from "./process/ProcessModeSection.vue";
import ProcessParametersSection from "./process/ProcessParametersSection.vue";
import ProcessOutputSection from "./process/ProcessOutputSection.vue";
import ProcessRunSection from "./process/ProcessRunSection.vue";
import ProcessResultsSection from "./process/ProcessResultsSection.vue";

const props = defineProps<{ processId: string }>();
const router = useRouter();
const route = useRoute();
const process = computed(() => findProcess(props.processId));
const modeId = ref("");
const inputs = reactive<Record<string, string[]>>({});
const values = reactive<Record<string, any>>({});
const breakpointMode = reactive<Record<string, boolean>>({});
const outputPath = ref("");
const touched = ref(false);
const preview = ref<CommandPreview | null>(null);
const previewOpen = ref(false);
const status = ref<RunStatus | "ready">("ready");
const runId = ref<string | null>(null);
const artifacts = ref<
  {
    path: string;
    fileType?: string;
    sizeBytes?: number;
    playable?: boolean;
    unavailable?: boolean;
  }[]
>([]);
const runtimeMessage = ref("");
const audioUrl = ref("");
const audioPlaying = ref(false);
const artifactMessage = ref("");
const inspectedWarnings = ref<string[]>([]);
const inspectedFile = ref<InspectedFile | null>(null);
const mode = computed(() => process.value && findMode(process.value, modeId.value));
const issues = computed(() =>
  process.value && mode.value
    ? validateRequest(process.value, mode.value, {
        processId: process.value.id,
        modeId: mode.value.id,
        inputs,
        parameters: values,
        outputPath: outputPath.value || null,
      })
    : [],
);
const issueFor = (field: string) =>
  touched.value ? issues.value?.find((issue) => issue.field === field)?.message : undefined;

function resetMode(id: string) {
  const next = process.value && findMode(process.value, id);
  if (!next) return;
  modeId.value = id;
  Object.keys(inputs).forEach((key) => delete inputs[key]);
  Object.keys(values).forEach((key) => delete values[key]);
  Object.keys(breakpointMode).forEach((key) => delete breakpointMode[key]);
  next.inputs.forEach((input) => {
    inputs[input.id] = Array.from({ length: input.minItems }, () => "");
  });
  Object.entries(defaultParameterValues(next)).forEach(([key, value]) => {
    const parameter = next.parameters.find((item) => item.id === key);
    if (parameter?.kind === "numberOrBreakpoint")
      values[key] = {
        kind: "number",
        value:
          typeof value === "object" && value !== null && "value" in value ? Number(value.value) : 0,
      };
    else if (parameter?.kind === "number" || parameter?.kind === "integer")
      values[key] = { kind: "number", value: Number(value) };
    else if (parameter?.kind === "choice") values[key] = { kind: "choice", value: String(value) };
    else if (parameter?.kind === "flag") values[key] = { kind: "flag", value: Boolean(value) };
    else if (parameter?.kind === "file") values[key] = { kind: "file", path: String(value ?? "") };
  });
  outputPath.value = "";
  preview.value = null;
}
watch(
  [process, () => route.query.mode],
  ([next, requestedMode]) => {
    if (!next) return;
    const requested =
      typeof requestedMode === "string" && findMode(next, requestedMode)
        ? requestedMode
        : (next.modes[0]?.id ?? "");
    resetMode(requested);
  },
  { immediate: true },
);
watch(
  [process, () => route.query.reusePath],
  ([next, path]) => {
    if (!next || typeof path !== "string" || !path) return;
    const accepted = next.modes.find((candidate) =>
      candidate.inputs.some((input) =>
        input.fileTypes.includes(String(route.query.reuseType) as never),
      ),
    );
    const input = accepted?.inputs[0] ?? next.modes[0]?.inputs[0];
    if (input) inputs[input.id] = [path];
  },
  { immediate: true },
);
watch(
  inputs,
  () => {
    void suggestOutput();
    const first = Object.values(inputs).flat().find(Boolean);
    const expected = mode.value?.inputs[0]?.fileTypes ?? [];
    if (first) void inspectInput(first, expected);
  },
  { deep: true },
);

function setNumber(parameter: ParameterDefinition, value: string) {
  if (
    parameter.kind === "number" ||
    parameter.kind === "integer" ||
    parameter.kind === "numberOrBreakpoint"
  )
    values[parameter.id] = { kind: "number", value: value === "" ? NaN : Number(value) };
}
function setInput(inputId: string, index: number, value: string) {
  inputs[inputId][index] = value;
}
function setParameterValue(parameterId: string, value: ParameterValue) {
  values[parameterId] = value;
}
function setBreakpoint(parameter: ParameterDefinition, path: string) {
  if (parameter.kind === "numberOrBreakpoint") values[parameter.id] = { kind: "file", path };
}
async function chooseBreakpoint(parameter: ParameterDefinition) {
  if (parameter.kind !== "numberOrBreakpoint") return;
  const picked = await open({ multiple: false, directory: false });
  if (typeof picked === "string") setBreakpoint(parameter, picked);
}
async function chooseInput(inputId: string, multiple: boolean) {
  const picked = await open({ multiple, directory: false });
  if (typeof picked === "string") inputs[inputId] = [picked];
  else if (Array.isArray(picked)) inputs[inputId] = picked;
}
async function inspectInput(path: string, expectedTypes: string[]) {
  if (!path || !("__TAURI_INTERNALS__" in window)) return;
  try {
    const result = await invoke<InspectedFile>("inspect_file", { path, expectedTypes });
    inspectedFile.value = result;
    inspectedWarnings.value =
      result.channels !== undefined && expectedTypes.includes("soundfile") && result.channels !== 1
        ? [
            `This process needs a mono soundfile. The selected file has ${result.channels} channels.`,
          ]
        : [];
    if (inspectedWarnings.value.length) runtimeMessage.value = inspectedWarnings.value[0];
  } catch (error) {
    inspectedFile.value = null;
    inspectedWarnings.value = [String(error)];
    runtimeMessage.value = inspectedWarnings.value[0];
  }
}
async function chooseOutput() {
  const picked = await open({ directory: false, multiple: false, save: true });
  if (typeof picked === "string") outputPath.value = picked;
}
function addInput(id: string) {
  inputs[id] = [...(inputs[id] ?? []), ""];
}
function removeInput(id: string, index: number) {
  inputs[id].splice(index, 1);
}
function moveInput(id: string, index: number, delta: number) {
  const list = inputs[id];
  const target = index + delta;
  if (target < 0 || target >= list.length) return;
  [list[index], list[target]] = [list[target], list[index]];
}
function preserveMode(nextId: string) {
  const next = process.value && findMode(process.value, nextId);
  const old = process.value && findMode(process.value, modeId.value);
  if (!next || !old || next.id === old.id) return;
  const shared: Record<string, any> = {};
  const incompatible = old.parameters.some((oldParameter) => {
    const nextParameter = next.parameters.find((parameter) => parameter.id === oldParameter.id);
    if (!nextParameter || values[oldParameter.id] === undefined) return false;
    if (oldParameter.kind !== nextParameter.kind) return true;
    shared[oldParameter.id] = values[oldParameter.id];
    return false;
  });
  if (
    incompatible &&
    !window.confirm(
      "Some parameter values are not compatible with this mode and will be reset. Continue?",
    )
  )
    return;
  modeId.value = nextId;
  resetMode(nextId);
  Object.assign(values, shared);
}
function onFormKey(event: KeyboardEvent) {
  if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
    event.preventDefault();
    request();
  }
}
async function request() {
  touched.value = true;
  if (!process.value || !mode.value || issues.value?.length) return;
  const runRequest = {
    processId: process.value.id,
    modeId: mode.value.id,
    inputs,
    parameters: values,
    outputPath: outputPath.value || null,
  };
  preview.value = await requestCommandPreview(runRequest);
  if (!preview.value) {
    runtimeMessage.value =
      "Runtime preview is unavailable. Start the desktop runtime to run this process.";
    status.value = "failed";
    return;
  }
  const accepted = await enqueueProcess(runRequest);
  if (!accepted) {
    runtimeMessage.value =
      "This process is ready, but the desktop queue is unavailable in browser preview.";
    status.value = "failed";
    return;
  }
  runId.value = accepted.runId;
  status.value = "queued";
  void poll();
}
async function togglePreview() {
  previewOpen.value = !previewOpen.value;
  if (!previewOpen.value || preview.value || !process.value || !mode.value) return;
  if (issues.value?.length) {
    runtimeMessage.value = "Complete the required fields to preview this command.";
    return;
  }
  preview.value = await requestCommandPreview({
    processId: process.value.id,
    modeId: mode.value.id,
    inputs,
    parameters: values,
    outputPath: outputPath.value || null,
  });
  if (!preview.value)
    runtimeMessage.value = "The desktop runtime could not create a command preview.";
}
async function suggestOutput() {
  const first = Object.values(inputs).flat().find(Boolean);
  if (process.value && mode.value && first && !outputPath.value)
    outputPath.value = (await suggestOutputPath(process.value.id, mode.value.id, first)) ?? "";
}
async function poll() {
  if (!runId.value) return;
  const snapshot = await getRunSnapshot(runId.value);
  if (!snapshot) return;
  status.value = snapshot.status;
  runtimeMessage.value = snapshot.error ?? "";
  artifacts.value = snapshot.artifacts ?? [];
  if (snapshot.status === "queued" || snapshot.status === "running")
    window.setTimeout(() => void poll(), 500);
}
async function cancel() {
  if (runId.value && (status.value === "queued" || status.value === "running")) {
    status.value = "cancelling";
    await cancelRun(runId.value);
    void poll();
  }
}
async function playArtifact(index: number) {
  if (!runId.value) return;
  try {
    audioUrl.value = (await allowArtifactPlayback(runId.value, index)) ?? "";
    artifactMessage.value = audioUrl.value ? "" : "Playback is available in the desktop app.";
  } catch (error) {
    artifactMessage.value = error instanceof Error ? error.message : String(error);
  }
}
async function revealResult(index: number) {
  if (!runId.value) return;
  try {
    await revealArtifact(runId.value, index);
    artifactMessage.value = "Artifact revealed in the file manager.";
  } catch (error) {
    artifactMessage.value = error instanceof Error ? error.message : String(error);
  }
}
async function locateArtifact(index: number) {
  const picked = await open({ multiple: false, directory: false });
  if (typeof picked !== "string") return;
  const artifact = artifacts.value[index];
  if (artifact) {
    artifact.path = picked;
    artifact.unavailable = false;
    artifactMessage.value = "Replacement selected. Use it as an input when ready.";
  }
}
function reuseArtifact(artifact: { path: string; fileType?: string }) {
  router.push({
    name: "navigator",
    query: { reusePath: artifact.path, reuseType: artifact.fileType ?? "soundfile" },
  });
}
</script>

<template>
  <main v-if="process && mode" class="process-shell">
    <header class="process-header">
      <button class="back-button" @click="router.push({ name: 'navigator' })">← Navigator</button>
      <div>
        <p class="eyebrow">
          {{ process.identity.executable
          }}<span v-if="process.identity.operation"> · {{ process.identity.operation }}</span>
        </p>
        <h1>
          {{ process.title }} <span>· {{ mode.title }}</span>
        </h1>
        <p>{{ process.summary }}</p>
      </div>
    </header>
    <ol class="flow-rail" aria-label="Process steps">
      <li class="active">Input</li>
      <li :class="{ valid: !issues?.some((issue) => issue.field.startsWith('input.')) }">Mode</li>
      <li :class="{ valid: !issues?.some((issue) => issue.field.startsWith('parameter.')) }">
        Parameters
      </li>
      <li :class="{ valid: Boolean(outputPath) }">Output</li>
      <li>Run</li>
    </ol>
    <form class="process-form" novalidate @submit.prevent="request" @keydown="onFormKey">
      <ProcessInputSection
        :mode="mode"
        :inputs="inputs"
        :issue-for="issueFor"
        :inspected-file="inspectedFile"
        :inspected-warnings="inspectedWarnings"
        @choose-input="chooseInput"
        @update-input="setInput"
        @add-input="addInput"
        @remove-input="removeInput"
        @move-input="moveInput"
        @touched="touched = true"
      />
      <ProcessModeSection
        :process="process"
        :mode="mode"
        :mode-id="modeId"
        @preserve-mode="preserveMode"
      />
      <ProcessParametersSection
        :mode="mode"
        :values="values"
        :issue-for="issueFor"
        @set-number="setNumber"
        @set-value="setParameterValue"
        @choose-breakpoint="chooseBreakpoint"
        @touched="touched = true"
      />
      <ProcessOutputSection
        v-model:output-path="outputPath"
        :issue-for="issueFor"
        @choose-output="chooseOutput"
        @touched="touched = true"
      />
      <ProcessRunSection
        :preview-open="previewOpen"
        :preview="preview"
        @toggle-preview="togglePreview"
      />
      <p v-if="runtimeMessage" class="field-error" role="alert">{{ runtimeMessage }}</p>
      <ProcessResultsSection
        :artifacts="artifacts"
        :artifact-message="artifactMessage"
        :audio-url="audioUrl"
        @play="playArtifact"
        @reveal="revealResult"
        @locate="locateArtifact"
        @reuse="reuseArtifact"
        @audio-play="audioPlaying = true"
        @audio-pause="audioPlaying = false"
      />
      <footer class="process-footer">
        <span :class="'status status-' + status">{{
          status === "queued"
            ? "Queued for processing"
            : status === "running"
              ? "Processing…"
              : status === "cancelling"
                ? "Cancelling…"
                : status === "completed"
                  ? "Completed"
                  : touched && issues?.length
                    ? issues.length + " issue(s) to fix"
                    : "Ready to configure"
        }}</span
        ><button
          v-if="status === 'queued' || status === 'running'"
          class="cancel-button"
          type="button"
          @click="cancel"
        >
          Cancel processing</button
        ><button v-else class="primary-button" type="submit" :disabled="status === 'cancelling'">
          Run process <span>↗</span>
        </button>
      </footer>
    </form>
  </main>
  <main v-else class="not-found">
    <h1>Process not found</h1>
    <button class="primary-button" @click="router.push({ name: 'navigator' })">
      Back to navigator
    </button>
  </main>
</template>

<style scoped>
@reference "../styles.css";

.process-shell {
  @apply mx-auto min-h-screen max-w-[820px] px-8 pt-7.5 pb-24;
}

.process-header {
  @apply grid grid-cols-[auto_1fr_auto] items-start gap-6 border-b border-muted pb-6;

  & h1 {
    @apply my-1.5 text-[28px] tracking-tight;

    & span {
      @apply text-lg font-normal text-muted;
    }
  }

  & p:last-child {
    @apply m-0 text-sm text-muted;
  }

  & .eyebrow {
    @apply m-0;
  }
}

.back-button {
  @apply border-0 bg-transparent py-1 text-sm text-primary;
}

.flow-rail {
  @apply m-0 flex list-none justify-between border-b border-muted py-6 text-[10px] tracking-widest text-dimmed uppercase;
  font-family: "IBM Plex Mono", monospace;

  & li {
    @apply relative;
  }

  & li.active,
  & li.valid {
    @apply text-primary;
  }

  & li.active::after,
  & li.valid::after {
    @apply absolute right-0 -bottom-[25px] left-0 h-0.5 bg-primary;
    content: "";
  }
}

:deep(.form-section) {
  @apply border-b border-muted py-7;
}

:deep(.section-heading) {
  @apply mb-5 flex items-start gap-4;

  & h2 {
    @apply mb-1 text-xl;
  }

  & p {
    @apply m-0 text-sm text-muted;
  }
}

:deep(.step-index) {
  @apply pt-1 text-[11px] text-primary;
  font-family: "IBM Plex Mono", monospace;
}

@media (max-width: 700px) {
  .process-shell {
    @apply px-4.5 pt-5 pb-25;
  }

  .process-header {
    @apply grid-cols-1 gap-2.5;
  }
}
</style>
