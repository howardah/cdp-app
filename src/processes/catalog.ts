import type { CdpFileType, ModeDefinition, ParameterDefinition, ProcessDefinition } from "./catalog.types";
import speed from "./definitions/modify-speed.json";
import loudness from "./definitions/modify-loudness.json";
import join from "./definitions/sfedit-join.json";
import analyze from "./definitions/pvoc-analyze.json";
import synthesize from "./definitions/pvoc-synthesize.json";
import isolate from "./definitions/isolate.json";

export const processCatalog = [speed, loudness, join, analyze, synthesize, isolate] as unknown as ProcessDefinition[];
export const categories = ["time-domain", "spectral", "edit-and-mix", "utilities"] as const;

export function findProcess(id: string): ProcessDefinition | undefined { return processCatalog.find((process) => process.id === id); }
export function findMode(process: ProcessDefinition, id: string): ModeDefinition | undefined { return process.modes.find((mode) => mode.id === id); }
export function searchCatalog(query: string, category?: ProcessDefinition["category"]): ProcessDefinition[] {
  const needle = query.trim().toLocaleLowerCase();
  return processCatalog.filter((process) => (!category || process.category === category) && (!needle || [process.title, process.summary, process.id, ...process.tags, process.identity.executable, process.identity.operation ?? ""].join(" ").toLocaleLowerCase().includes(needle)));
}
export function defaultParameterValues(mode: ModeDefinition): Record<string, unknown> {
  return Object.fromEntries(mode.parameters.filter((parameter: ParameterDefinition) => "default" in parameter).map((parameter) => [parameter.id, parameter.default]));
}
export function acceptsFileType(mode: ModeDefinition, fileType: CdpFileType): boolean { return mode.inputs.some((input) => input.fileTypes.includes(fileType)); }
