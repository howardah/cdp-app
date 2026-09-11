import type {
  CdpFileType,
  ModeDefinition,
  ParameterDefinition,
  ProcessDefinition,
} from "./catalog.types";
import speed from "./definitions/modify-speed.json";
import loudness from "./definitions/modify-loudness.json";
import join from "./definitions/sfedit-join.json";
import analyze from "./definitions/pvoc-analyze.json";
import synthesize from "./definitions/pvoc-synthesize.json";
import isolate from "./definitions/isolate.json";

export const processCatalog = [
  speed,
  loudness,
  join,
  analyze,
  synthesize,
  isolate,
] as unknown as ProcessDefinition[];
export const categories = ["time-domain", "spectral", "edit-and-mix", "utilities"] as const;

const idPattern = /^[a-z][a-zA-Z0-9]*(?:-[a-zA-Z0-9]+)*$/;
const safeFlag = /^-[A-Za-z0-9][A-Za-z0-9-]*$/;

/** Validate invariants which JSON Schema cannot express (references and uniqueness). */
export function validateCatalog(catalog: readonly ProcessDefinition[]): void {
  const processIds = new Set<string>();
  for (const process of catalog) {
    if (!idPattern.test(process.id) || processIds.has(process.id))
      throw new Error(`invalid or duplicate process id: ${process.id}`);
    processIds.add(process.id);
    const modeIds = new Set<string>();
    for (const mode of process.modes) {
      if (!idPattern.test(mode.id) || modeIds.has(mode.id))
        throw new Error(`invalid or duplicate mode id: ${mode.id}`);
      modeIds.add(mode.id);
      const inputIds = new Set(mode.inputs.map((input) => input.id));
      if (
        inputIds.size !== mode.inputs.length ||
        mode.inputs.some(
          (input) =>
            !idPattern.test(input.id) ||
            (input.maxItems !== null && input.maxItems < input.minItems),
        )
      )
        throw new Error(`invalid inputs in ${process.id}/${mode.id}`);
      const parameterIds = new Set(mode.parameters.map((parameter) => parameter.id));
      if (
        parameterIds.size !== mode.parameters.length ||
        mode.parameters.some((parameter) => !idPattern.test(parameter.id))
      )
        throw new Error(`invalid parameters in ${process.id}/${mode.id}`);
      for (const parameter of mode.parameters) {
        if (parameter.cli.kind !== "positional" && !safeFlag.test(parameter.cli.flag))
          throw new Error(`unsafe CLI flag in ${parameter.id}`);
        if (
          "default" in parameter &&
          (parameter.kind === "number" || parameter.kind === "integer")
        ) {
          const value = parameter.default;
          if (
            !Number.isFinite(value) ||
            ("min" in parameter && parameter.min !== undefined && value < parameter.min) ||
            ("max" in parameter && parameter.max !== undefined && value > parameter.max) ||
            (parameter.kind === "integer" && !Number.isInteger(value))
          )
            throw new Error(`invalid numeric default in ${parameter.id}`);
        }
      }
      for (const token of mode.argumentOrder) {
        if (token.kind === "input" && !inputIds.has(token.inputId))
          throw new Error(`unknown input reference: ${token.inputId}`);
        if (token.kind === "parameter" && !parameterIds.has(token.parameterId))
          throw new Error(`unknown parameter reference: ${token.parameterId}`);
        if (token.kind === "literal" && token.value.includes("\0"))
          throw new Error("literal contains NUL");
      }
      for (const constraint of mode.constraints) {
        const refs =
          constraint.kind === "powerOfTwo"
            ? [constraint.value]
            : [constraint.left, constraint.right];
        for (const ref of refs) {
          if (ref.source === "parameter" && !parameterIds.has(ref.id))
            throw new Error(`unknown constraint parameter: ${ref.id}`);
          if (ref.source === "inputMetadata" && !inputIds.has(ref.inputId))
            throw new Error(`unknown constraint input: ${ref.inputId}`);
        }
      }
    }
  }
}

validateCatalog(processCatalog);

export function findProcess(id: string): ProcessDefinition | undefined {
  return processCatalog.find((process) => process.id === id);
}
export function findMode(process: ProcessDefinition, id: string): ModeDefinition | undefined {
  return process.modes.find((mode) => mode.id === id);
}
export function searchCatalog(
  query: string,
  category?: ProcessDefinition["category"],
): ProcessDefinition[] {
  const needle = query.trim().toLocaleLowerCase();
  return processCatalog.filter(
    (process) =>
      (!category || process.category === category) &&
      (!needle ||
        [
          process.title,
          process.summary,
          process.id,
          ...process.tags,
          process.identity.executable,
          process.identity.operation ?? "",
        ]
          .join(" ")
          .toLocaleLowerCase()
          .includes(needle)),
  );
}
export function defaultParameterValues(mode: ModeDefinition): Record<string, unknown> {
  return Object.fromEntries(
    mode.parameters
      .filter((parameter: ParameterDefinition) => "default" in parameter)
      .map((parameter) => [parameter.id, parameter.default]),
  );
}
export function acceptsFileType(mode: ModeDefinition, fileType: CdpFileType): boolean {
  return mode.inputs.some((input) => input.fileTypes.includes(fileType));
}
