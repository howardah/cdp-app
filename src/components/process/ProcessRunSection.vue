<script setup lang="ts">
defineProps<{ previewOpen: boolean; preview: { display: string } | null }>();
defineEmits<{ togglePreview: [] }>();
</script>
<template>
  <section class="form-section run-section">
    <div class="section-heading">
      <span class="step-index">05</span>
      <div>
        <h2>Run</h2>
        <p>Review the backend-produced command preview, then submit to the queue.</p>
      </div>
    </div>
    <button type="button" class="secondary-button" @click="$emit('togglePreview')">
      {{ previewOpen ? "Hide" : "Show" }} command preview
    </button>
    <pre v-if="previewOpen" class="command-preview">{{
      preview?.display ?? "Preview becomes available when the Rust runtime is connected."
    }}</pre>
  </section>
</template>

<style scoped>
@reference "../../styles.css";

.command-preview {
  @apply mt-3 overflow-auto border border-muted bg-inverted p-3.5 text-xs whitespace-pre-wrap text-inverted;
  font-family: "IBM Plex Mono", monospace;
}
</style>
