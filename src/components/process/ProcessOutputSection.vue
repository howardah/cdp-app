<script setup lang="ts">
defineProps<{ outputPath: string; issueFor: (field: string) => string | undefined }>();
defineEmits<{ "update:outputPath": [value: string]; chooseOutput: []; touched: [] }>();
</script>
<template>
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
      <div class="flex gap-2 [&>.secondary-button]:shrink-0 [&>input]:flex-1">
        <input
          id="output-path"
          :value="outputPath"
          type="text"
          placeholder="/path/to/result.wav"
          @input="$emit('update:outputPath', ($event.target as HTMLInputElement).value)"
          @blur="$emit('touched')"
          :aria-describedby="issueFor('output') ? 'error-output' : undefined"
        /><button type="button" class="secondary-button" @click="$emit('chooseOutput')">
          Choose…
        </button>
      </div>
      <p v-if="issueFor('output')" id="error-output" class="field-error">
        {{ issueFor("output") }}
      </p>
    </div>
  </section>
</template>
