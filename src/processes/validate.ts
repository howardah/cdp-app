import type {
  ModeDefinition,
  ParameterValue,
  ProcessDefinition,
  RunProcessRequest,
} from "./catalog.types";

export interface ValidationIssue {
  field: string;
  message: string;
}
const pathValue = (request: RunProcessRequest, ref: string) => request.inputs[ref]?.[0];

export function validateRequest(
  process: ProcessDefinition,
  mode: ModeDefinition,
  request: RunProcessRequest,
): ValidationIssue[] {
  const issues: ValidationIssue[] = [];
  for (const input of mode.inputs) {
    const values = request.inputs[input.id] ?? [];
    if (values.length < input.minItems)
      issues.push({
        field: `input.${input.id}`,
        message: `${input.label} needs at least ${input.minItems} file${input.minItems === 1 ? "" : "s"}.`,
      });
    if (input.maxItems !== null && values.length > input.maxItems)
      issues.push({
        field: `input.${input.id}`,
        message: `${input.label} accepts at most ${input.maxItems} files.`,
      });
  }
  const parameterIds = new Set(mode.parameters.map((parameter) => parameter.id));
  for (const [id, value] of Object.entries(request.parameters)) {
    if (!parameterIds.has(id))
      issues.push({
        field: `parameter.${id}`,
        message: "This parameter is not available in the selected mode.",
      });
    validateParameter(mode, id, value, issues);
  }
  for (const parameter of mode.parameters)
    if (parameter.required && request.parameters[parameter.id] === undefined)
      issues.push({
        field: `parameter.${parameter.id}`,
        message: `${parameter.label} is required.`,
      });
  if (mode.output.kind === "singleFile" || mode.output.kind === "genericRoot") {
    if (!request.outputPath)
      issues.push({ field: "output", message: "Choose an output path before running." });
  } else if (request.outputPath !== null)
    issues.push({ field: "output", message: "This mode does not accept an output path." });
  for (const constraint of mode.constraints) {
    if (constraint.kind === "powerOfTwo") {
      const value = resolveValue(request, constraint.value);
      if (value !== undefined && (value < 2 || value & (value - 1)))
        issues.push({ field: "parameters", message: constraint.message });
    }
    if (
      constraint.kind === "lessThan" ||
      constraint.kind === "lessThanOrEqual" ||
      constraint.kind === "greaterThan"
    ) {
      const left = resolveValue(request, constraint.left);
      const right = resolveValue(request, constraint.right);
      if (
        left !== undefined &&
        right !== undefined &&
        !{ lessThan: left < right, lessThanOrEqual: left <= right, greaterThan: left > right }[
          constraint.kind
        ]
      )
        issues.push({ field: "parameters", message: constraint.message });
    }
  }
  void process;
  void pathValue;
  return issues;
}
function resolveValue(
  request: RunProcessRequest,
  ref: { source: string; id?: string; value?: number },
): number | undefined {
  if (ref.source === "literal") return ref.value;
  const value = ref.id ? request.parameters[ref.id] : undefined;
  return value?.kind === "number" ? value.value : undefined;
}
function validateParameter(
  mode: ModeDefinition,
  id: string,
  value: ParameterValue,
  issues: ValidationIssue[],
): void {
  const parameter = mode.parameters.find((item) => item.id === id);
  if (!parameter) return;
  if ((parameter.kind === "number" || parameter.kind === "integer") && value.kind === "number") {
    if (
      !Number.isFinite(value.value) ||
      (parameter.kind === "integer" && !Number.isInteger(value.value))
    )
      issues.push({
        field: `parameter.${id}`,
        message: `${parameter.label} must be a valid ${parameter.kind}.`,
      });
    if (parameter.min !== undefined && value.value < parameter.min)
      issues.push({
        field: `parameter.${id}`,
        message: `${parameter.label} must be at least ${parameter.min}.`,
      });
    if (parameter.max !== undefined && value.value > parameter.max)
      issues.push({
        field: `parameter.${id}`,
        message: `${parameter.label} must be at most ${parameter.max}.`,
      });
  }
  if (
    parameter.kind === "choice" &&
    value.kind === "choice" &&
    !parameter.choices.some((choice) => choice.value === value.value)
  )
    issues.push({ field: `parameter.${id}`, message: `${parameter.label} has an invalid choice.` });
}
