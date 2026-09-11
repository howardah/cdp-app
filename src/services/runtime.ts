import { invoke } from "@tauri-apps/api/core";
import type { RunProcessRequest } from "../processes";

export type RunStatus = "queued" | "running" | "cancelling" | "completed" | "failed" | "cancelled";
export interface RunSnapshot {
  runId: string;
  processId?: string;
  modeId?: string;
  status: RunStatus;
  queuePosition?: number;
  error?: string;
  artifacts?: { path: string; fileType?: string; sizeBytes?: number; playable?: boolean }[];
}
export interface RunAccepted {
  runId: string;
  queuePosition: number;
}
const desktop = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
export async function openProcessWindow(processId: string): Promise<boolean> {
  if (!desktop()) return false;
  await invoke("open_process_window", { processId });
  return true;
}
export async function enqueueProcess(request: RunProcessRequest): Promise<RunAccepted | null> {
  if (!desktop()) return null;
  return invoke<RunAccepted>("enqueue_process", { request });
}
export async function getRunSnapshot(runId: string): Promise<RunSnapshot | null> {
  if (!desktop()) return null;
  return invoke<RunSnapshot | null>("get_run", { runId });
}
export async function listRuns(): Promise<RunSnapshot[]> {
  if (!desktop()) return [];
  return invoke<RunSnapshot[]>("list_runs");
}
export async function cancelRun(runId: string): Promise<void> {
  if (desktop()) await invoke("cancel_process", { runId });
}
export async function suggestOutputPath(
  processId: string,
  modeId: string,
  primaryInput: string,
): Promise<string | null> {
  if (!desktop() || !primaryInput) return null;
  return invoke<string>("suggest_output_path", { processId, modeId, primaryInput });
}
export async function revealArtifact(runId: string, artifactIndex: number): Promise<void> {
  if (desktop()) await invoke("reveal_artifact", { runId, artifactIndex });
}
export async function allowArtifactPlayback(
  runId: string,
  artifactIndex: number,
): Promise<string | null> {
  if (!desktop()) return null;
  return invoke<string>("allow_artifact_playback", { runId, artifactIndex });
}
