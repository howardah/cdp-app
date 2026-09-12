<script setup lang="ts">
type Artifact = {
  path: string;
  fileType?: string;
  sizeBytes?: number;
  playable?: boolean;
  unavailable?: boolean;
};
defineProps<{ artifacts: Artifact[]; artifactMessage: string; audioUrl: string }>();
defineEmits<{
  play: [index: number];
  reveal: [index: number];
  locate: [index: number];
  reuse: [artifact: Artifact];
  audioPlay: [];
  audioPause: [];
}>();
</script>
<template>
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
          @click="$emit('play', index)"
        >
          Play</button
        ><button
          v-if="!artifact.unavailable"
          type="button"
          class="secondary-button"
          @click="$emit('reveal', index)"
        >
          Reveal in Finder</button
        ><button
          v-if="!artifact.unavailable"
          type="button"
          class="secondary-button"
          @click="$emit('reuse', artifact)"
        >
          Use as input</button
        ><button v-else type="button" class="secondary-button" @click="$emit('locate', index)">
          Locate file
        </button>
      </div>
    </div>
    <audio
      v-if="audioUrl"
      :src="audioUrl"
      controls
      preload="none"
      @play="$emit('audioPlay')"
      @pause="$emit('audioPause')"
      @ended="$emit('audioPause')"
    ></audio>
  </section>
</template>
