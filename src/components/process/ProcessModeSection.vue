<script setup lang="ts">
import type { ModeDefinition, ProcessDefinition } from "../../processes";
defineProps<{ process: ProcessDefinition; mode: ModeDefinition; modeId: string }>();
defineEmits<{ preserveMode: [modeId: string] }>();
</script>
<template>
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
          @change="$emit('preserveMode', candidate.id)"
        /><span
          ><strong>{{ candidate.title }}</strong
          ><small>{{ candidate.summary }}</small></span
        ></label
      >
    </div>
  </section>
</template>

<style scoped>
@reference "../../styles.css";

.mode-options {
  @apply grid grid-cols-[repeat(auto-fit,minmax(190px,1fr))] gap-2;
}

.mode-option {
  @apply flex cursor-pointer items-start gap-2.5 rounded-sm border border-muted p-3 text-muted;

  &.selected {
    @apply border-primary bg-elevated text-default;
  }

  & input {
    @apply mt-1 accent-primary;
  }

  & strong,
  & small {
    @apply block;
  }

  & strong {
    @apply text-sm;
  }

  & small {
    @apply mt-1 text-xs leading-snug text-muted;
  }
}
</style>
