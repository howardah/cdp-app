import { findMode, findProcess, type CdpFileType } from "../processes";
import type { RecipeDefinition } from "./catalog.types";

export const recipeCatalog: RecipeDefinition[] = [
  {
    id: "pitch-then-polish",
    title: "Pitch, then polish",
    summary: "Transpose a sound, then bring the result to a dependable final level.",
    outcome: "A pitch-shifted gesture with a controlled peak level, ready to place in a mix.",
    sourceGuidance:
      "Start with a mono or stereo sound whose pitch and duration can change together.",
    tags: ["pitch", "level", "finishing"],
    cautions: ["Semitone transposition changes duration as well as pitch."],
    steps: [
      {
        processId: "modify-speed",
        modeId: "semitones",
        instruction:
          "Choose the interval by ear. Negative values transpose down; positive values transpose up.",
        handoff: "Use the transposed soundfile as the input to the next step.",
      },
      {
        processId: "modify-loudness",
        modeId: "normalise",
        instruction: "Keep the safe starting level or lower it when the result needs mix headroom.",
        handoff: "The normalized soundfile is the finished result.",
      },
    ],
  },
  {
    id: "cut-reorder-reconnect",
    title: "Cut, reorder, reconnect",
    summary:
      "Extract regions from a source, arrange the pieces, and splice them into a new phrase.",
    outcome: "A re-sequenced sound assembled from selected fragments of one source.",
    sourceGuidance:
      "Use a source with clear events and prepare a cuts data file describing the regions to extract.",
    tags: ["edit", "resequence", "fragments"],
    cautions: ["The Join inputs must share sample rate, channel count, and sample format."],
    steps: [
      {
        processId: "isolate",
        modeId: "single-segments",
        instruction: "Extract one output per marked segment, then audition the resulting files.",
        handoff: "Choose the fragments you want and note the order in which they should play.",
      },
      {
        processId: "sfedit-join",
        modeId: "join",
        instruction:
          "Add the extracted fragments in playback order and adjust the splice length if joins click.",
        handoff: "The joined soundfile is the new phrase.",
      },
    ],
  },
  {
    id: "equalise-and-assemble",
    title: "Equalise and assemble",
    summary: "Bring a group of sounds to a common level before joining them.",
    outcome: "A continuous sequence whose source files have a more consistent perceived level.",
    sourceGuidance:
      "Choose two or more compatible sounds that belong in one sequence but differ in level.",
    tags: ["level", "join", "sequence"],
    cautions: ["Equalise produces several files; keep their intended playback order when joining."],
    steps: [
      {
        processId: "modify-loudness",
        modeId: "equalise",
        instruction: "Process the complete source set together so CDP can equalise it as a group.",
        handoff: "Collect every equalised output and preserve the order you want to hear.",
      },
      {
        processId: "sfedit-join",
        modeId: "join",
        instruction:
          "Add all equalised outputs in order, then use a short splice to smooth each boundary.",
        handoff: "The joined soundfile is the finished sequence.",
      },
    ],
  },
  {
    id: "spectral-round-trip",
    title: "Spectral round trip",
    summary: "Convert a sound to spectral analysis data and synthesize it back into audio.",
    outcome:
      "A reconstructed sound plus an analysis file that can support future spectral workflows.",
    sourceGuidance:
      "Start with a mono soundfile; a sustained or evolving source makes spectral behavior easiest to hear.",
    tags: ["spectral", "analysis", "resynthesis"],
    cautions: [
      "This pair does not transform the analysis data between steps; it demonstrates the analysis/resynthesis path.",
    ],
    steps: [
      {
        processId: "pvoc-analyze",
        modeId: "analyze",
        instruction:
          "Use the default analysis settings first; larger analysis sizes favor frequency detail over time detail.",
        handoff: "Use the resulting .ana analysis file as the next step's input.",
      },
      {
        processId: "pvoc-synthesize",
        modeId: "synthesize",
        instruction: "Synthesize the analysis file without changing its format or extension.",
        handoff: "Audition the reconstructed mono soundfile against the source.",
      },
    ],
  },
];

const idPattern = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/;

function outputTypes(processId: string, modeId: string): CdpFileType[] {
  const process = findProcess(processId);
  const mode = process && findMode(process, modeId);
  if (!mode || mode.output.kind === "none" || mode.output.kind === "stdoutReport") return [];
  if (mode.output.kind === "composite")
    return mode.output.components.map((component) => component.fileType);
  return [mode.output.fileType];
}

function inputTypes(processId: string, modeId: string): CdpFileType[] {
  const process = findProcess(processId);
  const mode = process && findMode(process, modeId);
  return mode ? [...new Set(mode.inputs.flatMap((input) => input.fileTypes))] : [];
}

export function validateRecipes(recipes: readonly RecipeDefinition[]): void {
  const ids = new Set<string>();
  for (const recipe of recipes) {
    if (!idPattern.test(recipe.id) || ids.has(recipe.id))
      throw new Error(`invalid or duplicate recipe id: ${recipe.id}`);
    ids.add(recipe.id);
    if (recipe.steps.length < 2) throw new Error(`recipe needs at least two steps: ${recipe.id}`);
    recipe.steps.forEach((step, index) => {
      const process = findProcess(step.processId);
      if (!process) throw new Error(`unknown recipe process: ${step.processId}`);
      if (!findMode(process, step.modeId))
        throw new Error(`unknown recipe mode: ${step.processId}/${step.modeId}`);
      const next = recipe.steps[index + 1];
      if (!next) return;
      const outputs = outputTypes(step.processId, step.modeId);
      const inputs = inputTypes(next.processId, next.modeId);
      if (!outputs.some((type) => inputs.includes(type)))
        throw new Error(`incompatible recipe handoff: ${recipe.id}/${index + 1}`);
    });
  }
}

validateRecipes(recipeCatalog);

export function findRecipe(id: string): RecipeDefinition | undefined {
  return recipeCatalog.find((recipe) => recipe.id === id);
}

export function searchRecipes(query: string): RecipeDefinition[] {
  const needle = query.trim().toLocaleLowerCase();
  if (!needle) return recipeCatalog;
  return recipeCatalog.filter((recipe) =>
    [recipe.title, recipe.summary, recipe.outcome, ...recipe.tags]
      .join(" ")
      .toLocaleLowerCase()
      .includes(needle),
  );
}
