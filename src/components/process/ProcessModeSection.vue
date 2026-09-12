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
