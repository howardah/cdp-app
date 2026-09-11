import { describe, expect, it } from "vitest";
import {
  acceptsFileType,
  defaultParameterValues,
  findMode,
  findProcess,
  processCatalog,
  searchCatalog,
  validateRequest,
} from "./index";
import type { RunProcessRequest } from "./catalog.types";

describe("process catalog", () => {
  it("contains unique, searchable process definitions", () => {
    const ids = processCatalog.map((process) => process.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(searchCatalog("transpose").map((process) => process.id)).toContain("modify-speed");
    expect(
      searchCatalog("spectral", "spectral").every((process) => process.category === "spectral"),
    ).toBe(true);
  });

  it("exposes the documented MVP mode coverage", () => {
    expect(findProcess("modify-speed")!.modes.map((mode) => mode.cliMode)).toEqual([1, 2, 5, 6]);
    expect(findProcess("modify-loudness")!.modes.map((mode) => mode.cliMode)).toEqual([
      1, 2, 3, 4, 5, 6, 7, 8,
    ]);
    expect(findProcess("isolate")!.modes.map((mode) => mode.cliMode)).toEqual([1, 2, 3, 4, 5]);
    expect(findProcess("sfedit-join")!.modes).toHaveLength(1);
    expect(findProcess("pvoc-analyze")!.modes).toHaveLength(1);
    expect(findProcess("pvoc-synthesize")!.modes).toHaveLength(1);
  });

  it("keeps documented argument order and output behavior", () => {
    const speed = findMode(findProcess("modify-speed")!, "accelerate")!;
    expect(speed.argumentOrder).toEqual([
      { kind: "literal", value: "speed" },
      { kind: "mode" },
      { kind: "input", inputId: "source" },
      { kind: "output" },
      { kind: "parameter", parameterId: "acceleration" },
      { kind: "parameter", parameterId: "goalTime" },
      { kind: "parameter", parameterId: "startTime" },
    ]);
    const report = findMode(findProcess("modify-loudness")!, "find-loudest")!;
    expect(report.output).toEqual({ kind: "stdoutReport" });
    expect(report.inputs[0].minItems).toBe(2);
    const isolate = findMode(findProcess("isolate")!, "dovetail")!;
    expect(isolate.parameters.find((parameter) => parameter.id === "dovetailMs")).toMatchObject({
      default: 5,
      min: 0,
      max: 20,
    });
    const analyze = findMode(findProcess("pvoc-analyze")!, "analyze")!;
    expect(analyze.argumentOrder[0]).toEqual({ kind: "literal", value: "anal" });
    expect(analyze.parameters.map((parameter) => parameter.cli)).toEqual([
      { kind: "option", flag: "-c", join: "concatenated" },
      { kind: "option", flag: "-o", join: "concatenated" },
    ]);
    const synthesize = findMode(findProcess("pvoc-synthesize")!, "synthesize")!;
    expect(synthesize.argumentOrder[0]).toEqual({ kind: "literal", value: "synth" });
  });

  it("returns defaults and file compatibility from a mode", () => {
    const process = findProcess("modify-speed");
    expect(process).toBeDefined();
    const mode = findMode(process!, "ratio");
    expect(mode).toBeDefined();
    expect(defaultParameterValues(mode!).ratio).toEqual({ kind: "number", value: 1 });
    expect(acceptsFileType(mode!, "soundfile")).toBe(true);
    expect(acceptsFileType(mode!, "analysis-ana")).toBe(false);
  });

  it("reports missing required input and output immediately", () => {
    const process = findProcess("modify-speed")!;
    const mode = findMode(process, "ratio")!;
    const request: RunProcessRequest = {
      processId: process.id,
      modeId: mode.id,
      inputs: { source: [] },
      parameters: { ratio: { kind: "number", value: 1 } },
      outputPath: null,
    };
    expect(validateRequest(process, mode, request).map((issue) => issue.field)).toEqual([
      "input.source",
      "output",
    ]);
  });

  it("enforces numeric bounds, integer constraints, and cross-field constraints", () => {
    const process = findProcess("isolate")!;
    const mode = findMode(process, process.modes[0].id)!;
    const request: RunProcessRequest = {
      processId: process.id,
      modeId: mode.id,
      inputs: Object.fromEntries(mode.inputs.map((input) => [input.id, ["source.wav"]])),
      parameters: Object.fromEntries(
        mode.parameters.map((parameter) => [parameter.id, { kind: "number", value: 999.5 }]),
      ),
      outputPath: "/tmp/result.wav",
    } as RunProcessRequest;
    const issues = validateRequest(process, mode, request);
    expect(issues.length).toBeGreaterThan(0);
    expect(issues.some((issue) => issue.field.startsWith("parameter."))).toBe(true);
  });
});
