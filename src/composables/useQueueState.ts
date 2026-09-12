import { onMounted, onUnmounted, ref } from "vue";
import { cancelRun, listRuns, revealArtifact, type RunSnapshot } from "../services/runtime";
import { processCatalog } from "../processes";

export function useQueueState() {
  const runs = ref<RunSnapshot[]>([]);
  const queueOpen = ref(false);
  const queueError = ref("");
  let timer: number | undefined;
  async function refreshRuns() {
    try {
      runs.value = await listRuns();
      queueError.value = "";
    } catch (error) {
      queueError.value = error instanceof Error ? error.message : String(error);
    }
  }
  function runTitle(run: RunSnapshot) {
    const process = run.processId
      ? processCatalog.find((item) => item.id === run.processId)
      : undefined;
    return process?.title ?? `Run ${run.runId.slice(0, 8)}`;
  }
  function runState(run: RunSnapshot) {
    if (run.status === "queued")
      return run.queuePosition === undefined ? "Waiting" : `Waiting · #${run.queuePosition + 1}`;
    if (run.status === "running") return "Active · processing";
    if (run.status === "completed") return "Completed";
    if (run.status === "failed") return "Failed";
    return run.status;
  }
  async function cancelQueued(run: RunSnapshot) {
    try {
      await cancelRun(run.runId);
      await refreshRuns();
    } catch (error) {
      queueError.value = error instanceof Error ? error.message : String(error);
    }
  }
  async function reveal(run: RunSnapshot, index: number) {
    try {
      await revealArtifact(run.runId, index);
    } catch (error) {
      queueError.value = error instanceof Error ? error.message : String(error);
    }
  }
  onMounted(() => {
    void refreshRuns();
    timer = window.setInterval(() => void refreshRuns(), 1000);
  });
  onUnmounted(() => {
    if (timer) window.clearInterval(timer);
  });
  return { runs, queueOpen, queueError, runTitle, runState, cancelQueued, reveal };
}
