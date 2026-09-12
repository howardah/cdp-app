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

<style scoped>
@reference "../styles.css";

.queue-bar {
  @apply relative z-10 flex h-18 items-center justify-between border-t border-muted px-7;
  background: var(--surface);
  flex-shrink: 0;
}

.signal-rail {
  @apply flex items-center -mt-3;
  width: min(590px, 65%);
}

.rail-node {
  @apply flex size-7 shrink-0 flex-col items-center justify-center rounded-full border border-muted text-[11px] text-dimmed;
  font-family: "IBM Plex Mono", monospace;

  & small {
    @apply absolute mt-12 text-[8px] tracking-widest text-dimmed;
  }

  &.done,
  &.active {
    @apply border-primary text-primary;
  }
}

.rail-line {
  @apply h-px flex-1 bg-muted;

  &.done {
    @apply bg-primary;
  }
}

.queue-toggle,
.job,
.popover-head {
  @apply flex items-center;
}

.queue-toggle {
  @apply gap-3 border-0 bg-transparent text-left text-default transition-colors;

  & strong,
  & small {
    @apply block;
  }

  & strong {
    @apply text-sm;
  }

  & small {
    @apply mt-0.5 text-[10px] text-muted;
    font-family: "IBM Plex Mono", monospace;
  }
}

.queue-pulse {
  @apply size-2 rounded-full bg-warning ring-4 ring-warning/15;
}

.chevron {
  @apply ml-2 text-muted;
}

.queue-popover {
  @apply fixed right-7 bottom-18 z-3 w-83 border border-muted bg-elevated p-4 shadow-2xl;
}

.popover-head {
  @apply justify-between border-b border-muted pb-3;
}

.close-button {
  @apply border-0 bg-transparent text-xl text-muted;
}

.job {
  @apply gap-3 border-b border-muted py-3.5;

  &:last-child {
    @apply border-b-0 pb-0;
  }

  & > div {
    @apply flex-1;
  }

  & strong,
  & small {
    @apply block;
  }

  & strong {
    @apply text-sm;
  }

  & small {
    @apply mt-1 text-[10px] text-muted;
    font-family: "IBM Plex Mono", monospace;
  }
}

.job-number {
  @apply w-4 text-center text-[11px] text-dimmed;
  font-family: "IBM Plex Mono", monospace;
}

.job-spinner {
  @apply size-3.5 animate-spin rounded-full border-2 border-muted border-t-primary;
}

.cancel-button {
  @apply rounded-sm border border-error/50 bg-transparent px-2 py-1 text-[11px] text-error;
}

@media (prefers-reduced-motion: reduce) {
  .job-spinner {
    @apply animate-none;
  }
}
</style>
