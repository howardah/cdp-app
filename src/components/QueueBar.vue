<script setup lang="ts">
import { useQueueState } from "../composables/useQueueState";

const { runs, queueOpen, queueError, runTitle, runState, cancelQueued, reveal } = useQueueState();
</script>

<template>
  <footer class="queue-bar">
    <div class="signal-rail" aria-label="Process flow">
      <span class="rail-node active">1<small>INPUT</small></span
      ><span class="rail-line"></span><span class="rail-node">2<small>MODE</small></span>
      <span class="rail-line"></span><span class="rail-node">3<small>PARAMETERS</small></span
      ><span class="rail-line"></span> <span class="rail-node">4<small>OUTPUT</small></span
      ><span class="rail-line"></span><span class="rail-node">5<small>RUN</small></span>
    </div>
    <button
      class="queue-toggle"
      aria-haspopup="dialog"
      :aria-expanded="queueOpen"
      @click="queueOpen = !queueOpen"
    >
      <span class="queue-pulse" aria-hidden="true"></span
      ><span
        ><strong
          >Queue ·
          {{ runs.filter((run) => ["queued", "running"].includes(run.status)).length }}
          active</strong
        >
        <small
          >{{ runs.filter((run) => run.status === "completed").length }} completed this
          session</small
        ></span
      >
      <span class="chevron" aria-hidden="true">{{ queueOpen ? "⌄" : "⌃" }}</span>
    </button>
    <section v-if="queueOpen" class="queue-popover" role="dialog" aria-label="Processing queue">
      <div class="popover-head">
        <strong>Queue and results</strong
        ><button class="close-button" aria-label="Close queue" @click="queueOpen = false">×</button>
      </div>
      <p v-if="queueError" class="field-error">{{ queueError }}</p>
      <p v-if="!runs.length" class="empty-state">
        <strong>No runs yet</strong><span>Submit a process to see its status here.</span>
      </p>
      <div v-for="run in runs" :key="run.runId" class="job">
        <span class="job-number" aria-hidden="true">{{
          run.queuePosition !== undefined ? run.queuePosition + 1 : "·"
        }}</span>
        <span v-if="run.status === 'running'" class="job-spinner" aria-hidden="true"></span
        ><span v-else class="status-dot" aria-hidden="true"></span>
        <div>
          <strong>{{ runTitle(run) }}</strong
          ><small>{{ runState(run) }}</small
          ><small v-if="run.error" class="field-error">{{ run.error }}</small>
          <div v-if="run.artifacts?.length" class="job-artifacts">
            <button
              v-for="(artifact, index) in run.artifacts"
              :key="artifact.path"
              class="text-button"
              @click="void reveal(run, index)"
            >
              Reveal {{ artifact.fileType ?? "artifact" }}
            </button>
          </div>
        </div>
        <button
          v-if="run.status === 'queued'"
          class="cancel-button"
          @click="void cancelQueued(run)"
        >
          Cancel
        </button>
      </div>
    </section>
  </footer>
</template>
