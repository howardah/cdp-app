<script setup lang="ts">
import type { InspectedFile, ModeDefinition } from "../../processes";

defineProps<{
  mode: ModeDefinition;
  inputs: Record<string, string[]>;
  issueFor: (field: string) => string | undefined;
  inspectedFile: InspectedFile | null;
  inspectedWarnings: string[];
}>();
const emit = defineEmits<{
  chooseInput: [inputId: string, multiple: boolean];
  updateInput: [inputId: string, index: number, value: string];
  addInput: [inputId: string];
  removeInput: [inputId: string, index: number];
  moveInput: [inputId: string, index: number, delta: number];
  touched: [];
}>();
</script>

<template>
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
      <div v-for="(_, index) in inputs[input.id]" :key="`${input.id}-${index}`" class="file-picker">
        <input
          :id="`input-${input.id}-${index}`"
          :value="inputs[input.id][index]"
          type="text"
          :aria-describedby="issueFor(`input.${input.id}`) ? `error-input-${input.id}` : undefined"
          placeholder="/path/to/source.wav"
          @input="emit('updateInput', input.id, index, ($event.target as HTMLInputElement).value)"
          @blur="emit('touched')"
        />
        <button
          type="button"
          class="secondary-button"
          @click="emit('chooseInput', input.id, false)"
        >
          Choose…
        </button>
        <button
          v-if="input.ordered && inputs[input.id].length > 1"
          type="button"
          class="icon-button"
          :disabled="index === 0"
          aria-label="Move input up"
          @click="emit('moveInput', input.id, index, -1)"
        >
          ↑
        </button>
        <button
          v-if="input.ordered && inputs[input.id].length > 1"
          type="button"
          class="icon-button"
          :disabled="index === inputs[input.id].length - 1"
          aria-label="Move input down"
          @click="emit('moveInput', input.id, index, 1)"
        >
          ↓
        </button>
        <button
          v-if="inputs[input.id].length > input.minItems"
          type="button"
          class="icon-button"
          aria-label="Remove input"
          @click="emit('removeInput', input.id, index)"
        >
          ×
        </button>
      </div>
      <button
        v-if="input.maxItems === null || inputs[input.id].length < input.maxItems"
        type="button"
        class="text-button"
        @click="emit('addInput', input.id)"
      >
        + Add another file
      </button>
      <p v-if="issueFor(`input.${input.id}`)" class="field-error" :id="`error-input-${input.id}`">
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
</template>
