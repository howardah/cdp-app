import { invoke } from "@tauri-apps/api/core";
import type { RunProcessRequest } from "../processes";

export interface CommandPreview { display: string; tokens: string[]; }

/** The webview requests a preview; it never constructs executable arguments. */
export async function requestCommandPreview(request: RunProcessRequest): Promise<CommandPreview | null> {
  try { return await invoke<CommandPreview>("preview_process", { request }); }
  catch { return null; }
}
