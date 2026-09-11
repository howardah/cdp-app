<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { defaultParameterValues, findProcess, findMode, validateRequest } from "../processes";
import type { InspectedFile, ParameterDefinition } from "../processes";
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
  process,
  (next) => {
    if (next) resetMode(next.modes[0]?.id ?? "");
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
  const request = {
    processId: process.value.id,
    modeId: mode.value.id,
    inputs,
    parameters: values,
    outputPath: outputPath.value || null,
  };
  preview.value = await requestCommandPreview(request);
  if (!preview.value) {
    runtimeMessage.value =
      "Runtime preview is unavailable. Start the desktop runtime to run this process.";
    status.value = "failed";
    return;
  }
  const accepted = await enqueueProcess(request);
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
    artifactMessage.value = "Artifact revealed in Finder.";
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
  <main class="process-shell" v-if="process && mode">
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
    <form class="process-form" @submit.prevent="request" @keydown="onFormKey" novalidate>
      <section class="form-section">
        <div class="section-heading">
          <span class="step-index">01</span>
          <div>
            <h2>Input</h2>
            <p>Choose paths already on your computer. Browser file objects are not used.</p>
          </div>
        </div>
        <div v-for="input in mode.inputs" :key="input.id" class="field">
          <label
            >{{ input.label }} <small>{{ input.fileTypes.join(" · ") }}</small></label
          >
          <div
            v-for="(_, index) in inputs[input.id]"
            :key="`${input.id}-${index}`"
            class="file-picker"
          >
            <input
              :id="`input-${input.id}-${index}`"
              v-model="inputs[input.id][index]"
              type="text"
              :aria-describedby="
                issueFor(`input.${input.id}`) ? `error-input-${input.id}` : undefined
              "
              placeholder="/path/to/source.wav"
              @blur="touched = true"
            /><button type="button" class="secondary-button" @click="chooseInput(input.id, false)">
              Choose…</button
            ><button
              v-if="input.ordered && inputs[input.id].length > 1"
              type="button"
              class="icon-button"
              :disabled="index === 0"
              aria-label="Move input up"
              @click="moveInput(input.id, index, -1)"
            >
              ↑</button
            ><button
              v-if="input.ordered && inputs[input.id].length > 1"
              type="button"
              class="icon-button"
              :disabled="index === inputs[input.id].length - 1"
              aria-label="Move input down"
              @click="moveInput(input.id, index, 1)"
            >
              ↓</button
            ><button
              v-if="inputs[input.id].length > input.minItems"
              type="button"
              class="icon-button"
              aria-label="Remove input"
              @click="removeInput(input.id, index)"
            >
              ×
            </button>
          </div>
          <button
            v-if="input.maxItems === null || inputs[input.id].length < input.maxItems"
            type="button"
            class="text-button"
            @click="addInput(input.id)"
          >
            + Add another file
          </button>
          <p
            v-if="issueFor(`input.${input.id}`)"
            class="field-error"
            :id="`error-input-${input.id}`"
          >
            {{ issueFor(`input.${input.id}`) }}
          </p>
          <p class="field-help">{{ input.description }}</p>
          <div
            v-if="inspectedFile && inputs[input.id]?.includes(inspectedFile.path)"
            class="inspection-meta"
            :class="{ 'mono-warning': inspectedWarnings.length }"
            role="status"
          >
            <span v-if="inspectedFile.durationSeconds !== undefined"
              >{{ inspectedFile.durationSeconds.toFixed(2) }} s</span
            ><span v-if="inspectedFile.sampleRate"
              >{{ (inspectedFile.sampleRate / 1000).toFixed(1) }} kHz</span
            ><span v-if="inspectedFile.channels">{{ inspectedFile.channels }} ch</span
            ><span v-if="inspectedFile.sampleFormat">{{ inspectedFile.sampleFormat }}</span
            ><strong v-if="inspectedWarnings.length">{{ inspectedWarnings[0] }}</strong>
          </div>
        </div>
      </section>
      <section class="form-section">
        <div class="section-heading">
          <span class="step-index">02</span>
          <div>
            <h2>Mode</h2>
            <p>Select the musical operation you want to perform.</p>
          </div>
        </div>
        <div class="mode-options">
          <label
            v-for="candidate in process.modes"
            :key="candidate.id"
            class="mode-option"
            :class="{ selected: candidate.id === mode.id }"
            ><input
              :checked="candidate.id === modeId"
              type="radio"
              :value="candidate.id"
              @change="preserveMode(candidate.id)"
            /><span
              ><strong>{{ candidate.title }}</strong
              ><small>{{ candidate.summary }}</small></span
            ></label
          >
        </div>
      </section>
      <section class="form-section">
        <div class="section-heading">
          <span class="step-index">03</span>
          <div>
            <h2>Parameters</h2>
            <p>Safe starting values are prefilled; adjust them for your material.</p>
          </div>
        </div>
        <template v-for="group in [false, true]" :key="String(group)"
          ><details
            v-if="group && mode.parameters.some((parameter) => parameter.advanced)"
            class="advanced-parameters"
          >
            <summary>
              Advanced parameters
              <small>{{ mode.parameters.filter((parameter) => parameter.advanced).length }}</small>
            </summary>
            <div
              v-for="parameter in mode.parameters.filter((parameter) => parameter.advanced)"
              :key="parameter.id"
              class="field"
            >
              <label :for="`parameter-${parameter.id}`"
                >{{ parameter.label }}
                <small v-if="parameter.unit">{{ parameter.unit }}</small></label
              ><template v-if="parameter.kind === 'flag'"
                ><label class="toggle"
                  ><input
                    :id="`parameter-${parameter.id}`"
                    type="checkbox"
                    :checked="values[parameter.id]?.kind === 'flag' && values[parameter.id].value"
                    @change="
                      values[parameter.id] = {
                        kind: 'flag',
                        value: ($event.target as HTMLInputElement).checked,
                      }
                    "
                  /><span>Enable this option</span></label
                ></template
              ><template v-else
                ><input
                  :id="`parameter-${parameter.id}`"
                  type="number"
                  :min="'min' in parameter ? parameter.min : undefined"
                  :max="'max' in parameter ? parameter.max : undefined"
                  :step="'step' in parameter ? parameter.step : undefined"
                  :value="values[parameter.id]?.kind === 'number' ? values[parameter.id].value : ''"
                  @input="setNumber(parameter, ($event.target as HTMLInputElement).value)"
                  @blur="touched = true"
                  :aria-describedby="
                    issueFor(`parameter.${parameter.id}`)
                      ? `error-parameter-${parameter.id}`
                      : undefined
                  "
              /></template>
              <p class="field-help">{{ parameter.description }}</p>
              <p
                v-if="issueFor(`parameter.${parameter.id}`)"
                class="field-error"
                :id="`error-parameter-${parameter.id}`"
              >
                {{ issueFor(`parameter.${parameter.id}`) }}
              </p>
            </div>
          </details>
          <div
            v-else-if="!group"
            v-for="parameter in mode.parameters.filter((parameter) => !parameter.advanced)"
            :key="parameter.id"
            class="field"
          >
            <label :for="`parameter-${parameter.id}`"
              >{{ parameter.label }}
              <small v-if="parameter.unit">{{ parameter.unit }}</small></label
            ><template v-if="parameter.kind === 'choice'"
              ><select
                :id="`parameter-${parameter.id}`"
                :value="values[parameter.id]?.kind === 'choice' ? values[parameter.id].value : ''"
                @change="
                  values[parameter.id] = {
                    kind: 'choice',
                    value: ($event.target as HTMLSelectElement).value,
                  }
                "
              >
                <option
                  v-for="choice in parameter.choices"
                  :key="choice.value"
                  :value="choice.value"
                >
                  {{ choice.label }}
                </option>
              </select></template
            ><template v-else-if="parameter.kind === 'flag'"
              ><label class="toggle"
                ><input
                  :id="`parameter-${parameter.id}`"
                  type="checkbox"
                  :checked="values[parameter.id]?.kind === 'flag' && values[parameter.id].value"
                  @change="
                    values[parameter.id] = {
                      kind: 'flag',
                      value: ($event.target as HTMLInputElement).checked,
                    }
                  "
                /><span>Enable this option</span></label
              ></template
            ><template v-else-if="parameter.kind === 'numberOrBreakpoint'"
              ><div class="parameter-toggle">
                <button
                  type="button"
                  :class="{ selected: values[parameter.id]?.kind !== 'file' }"
                  @click="
                    setNumber(
                      parameter,
                      String(
                        values[parameter.id]?.kind === 'number'
                          ? values[parameter.id].value
                          : parameter.default.value,
                      ),
                    )
                  "
                >
                  Scalar</button
                ><button
                  type="button"
                  :class="{ selected: values[parameter.id]?.kind === 'file' }"
                  @click="chooseBreakpoint(parameter)"
                >
                  Breakpoint file
                </button>
              </div>
              <input
                v-if="values[parameter.id]?.kind !== 'file'"
                :id="`parameter-${parameter.id}`"
                type="number"
                :min="parameter.min"
                :max="parameter.max"
                :step="parameter.step"
                :value="values[parameter.id]?.value"
                @input="setNumber(parameter, ($event.target as HTMLInputElement).value)"
                @blur="touched = true"
                :aria-describedby="
                  issueFor(`parameter.${parameter.id}`)
                    ? `error-parameter-${parameter.id}`
                    : undefined
                "
              />
              <div v-else class="file-picker">
                <input
                  :id="`parameter-${parameter.id}`"
                  :value="values[parameter.id].path"
                  readonly
                  :aria-describedby="
                    issueFor(`parameter.${parameter.id}`)
                      ? `error-parameter-${parameter.id}`
                      : undefined
                  "
                /><button
                  type="button"
                  class="secondary-button"
                  @click="chooseBreakpoint(parameter)"
                >
                  Choose…
                </button>
              </div></template
            ><template v-else
              ><input
                :id="`parameter-${parameter.id}`"
                type="number"
                :min="'min' in parameter ? parameter.min : undefined"
                :max="'max' in parameter ? parameter.max : undefined"
                :step="'step' in parameter ? parameter.step : undefined"
                :value="values[parameter.id]?.kind === 'number' ? values[parameter.id].value : ''"
                @input="setNumber(parameter, ($event.target as HTMLInputElement).value)"
                @blur="touched = true"
                :aria-describedby="
                  issueFor(`parameter.${parameter.id}`)
                    ? `error-parameter-${parameter.id}`
                    : undefined
                "
            /></template>
            <p class="field-help">{{ parameter.description }}</p>
            <p
              v-if="issueFor(`parameter.${parameter.id}`)"
              class="field-error"
              :id="`error-parameter-${parameter.id}`"
            >
              {{ issueFor(`parameter.${parameter.id}`) }}
            </p>
          </div></template
        >
      </section>
      <section class="form-section">
        <div class="section-heading">
          <span class="step-index">04</span>
          <div>
            <h2>Output</h2>
            <p>Choose a new destination. Existing files are never overwritten.</p>
          </div>
        </div>
        <div class="field">
          <label for="output-path">Output path</label>
          <div class="file-picker">
            <input
              id="output-path"
              v-model="outputPath"
              type="text"
              placeholder="/path/to/result.wav"
              @blur="touched = true"
              :aria-describedby="issueFor('output') ? 'error-output' : undefined"
            /><button type="button" class="secondary-button" @click="chooseOutput">Choose…</button>
          </div>
          <p v-if="issueFor('output')" id="error-output" class="field-error">
            {{ issueFor("output") }}
          </p>
        </div>
      </section>
      <section class="form-section run-section">
        <div class="section-heading">
          <span class="step-index">05</span>
          <div>
            <h2>Run</h2>
            <p>Review the backend-produced command preview, then submit to the queue.</p>
          </div>
        </div>
        <button type="button" class="secondary-button" @click="previewOpen = !previewOpen">
          {{ previewOpen ? "Hide" : "Show" }} command preview
        </button>
        <pre v-if="previewOpen" class="command-preview">{{
          preview?.display ?? "Preview becomes available when the Rust runtime is connected."
        }}</pre>
      </section>
      <p v-if="runtimeMessage" class="field-error" role="alert">{{ runtimeMessage }}</p>
      <section v-if="artifacts.length" class="result-section" aria-live="polite">
        <h2>Results</h2>
        <p v-if="artifactMessage" class="field-help">{{ artifactMessage }}</p>
        <div
          v-for="(artifact, index) in artifacts"
          :key="`${artifact.path}-${index}`"
          class="result-artifact"
        >
          <div>
            <strong>{{ artifact.path }}</strong
            ><small
              >{{ artifact.fileType ?? "artifact"
              }}<span v-if="artifact.sizeBytes">
                · {{ Math.max(1, Math.round(artifact.sizeBytes / 1024)) }} KB</span
              ></small
            >
          </div>
          <div class="artifact-actions">
            <button
              v-if="artifact.playable && !artifact.unavailable"
              type="button"
              class="secondary-button"
              @click="void playArtifact(index)"
            >
              Play</button
            ><button
              v-if="!artifact.unavailable"
              type="button"
              class="secondary-button"
              @click="void revealResult(index)"
            >
              Reveal in Finder</button
            ><button
              v-if="!artifact.unavailable"
              type="button"
              class="secondary-button"
              @click="reuseArtifact(artifact)"
            >
              Use as input</button
            ><button
              v-else
              type="button"
              class="secondary-button"
              @click="void locateArtifact(index)"
            >
              Locate file
            </button>
          </div>
        </div>
        <audio
          v-if="audioUrl"
          :src="audioUrl"
          controls
          preload="none"
          @play="audioPlaying = true"
          @pause="audioPlaying = false"
          @ended="audioPlaying = false"
        ></audio>
      </section>
      <footer class="process-footer">
        <span :class="`status status-${status}`">{{
          status === "queued"
            ? "Queued for processing"
            : status === "running"
              ? "Processing…"
              : status === "cancelling"
                ? "Cancelling…"
                : status === "completed"
                  ? "Completed"
                  : touched && issues?.length
                    ? `${issues.length} issue(s) to fix`
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
