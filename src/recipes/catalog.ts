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
  {
    id: "carve-and-emphasize",
    title: "Carve and emphasize",
    summary: "Cut or boost a fixed frequency area, reinforce the attack, then set a final level.",
    outcome: "A more focused sound with a clearer onset and a controlled peak level.",
    sourceGuidance:
      "Use a recording whose tonal area and transient both need a little definition; leave headroom before emphasizing the attack.",
    tags: ["filter", "attack", "finishing"],
    cautions: [
      "Attack gain and a positive filter boost can create clipping; normalize last and retain mix headroom.",
    ],
    steps: [
      {
        processId: "filter-fixed",
        modeId: "fixed",
        instruction:
          "Start with a modest cut or boost around the frequency you want to de-emphasize or bring forward.",
        handoff: "Use the filtered soundfile as the source for attack shaping.",
      },
      {
        processId: "envel-attack",
        modeId: "attack",
        instruction:
          "Use a restrained gain and short onset so the transient becomes clearer without sounding detached.",
        handoff: "Use the attack-shaped soundfile for the final level pass.",
      },
      {
        processId: "modify-loudness",
        modeId: "normalise",
        instruction:
          "Normalize conservatively, lowering the target when the sound needs room alongside other material.",
        handoff: "The normalized soundfile is the finished result.",
      },
    ],
  },
  {
    id: "loop-then-pan",
    title: "Loop, then pan",
    summary: "Turn a mono fragment into a repeating gesture and position it in stereo.",
    outcome:
      "A looping stereo gesture with its repetitions placed at a deliberate lateral position.",
    sourceGuidance:
      "Start with a mono source that contains a useful region to repeat; choose a loop boundary that can tolerate a short splice.",
    tags: ["loop", "repeat", "stereo"],
    cautions: [
      "Pan mono accepts mono input only; use a short splice and audition for clicks at the loop boundary.",
    ],
    steps: [
      {
        processId: "extend-loop",
        modeId: "advance",
        instruction:
          "Set the loop start, length, and step around the fragment you want to repeat; begin with a short splice.",
        handoff: "Use the looped mono soundfile as the source for stereo placement.",
      },
      {
        processId: "modify-space",
        modeId: "pan",
        instruction:
          "Choose a pan position and keep the default prescale unless a louder source needs more attenuation.",
        handoff: "The panned stereo soundfile is the finished gesture.",
      },
    ],
  },
  {
    id: "smooth-and-exaggerate-spectrum",
    title: "Smooth and exaggerate spectrum",
    summary:
      "Analyze a mono sound, smooth adjacent spectral channels, then emphasize its contour before rendering.",
    outcome:
      "A resynthesized sound with softened local detail and a more pronounced spectral shape.",
    sourceGuidance:
      "Use a mono sound with evolving timbre; material with clearly changing harmonics makes both spectral stages easier to hear.",
    tags: ["spectral", "blur", "contour"],
    cautions: [
      "Both middle stages work on .ana analysis data; retain it if you want to revisit the spectral decisions later.",
    ],
    steps: [
      {
        processId: "pvoc-analyze",
        modeId: "analyze",
        instruction:
          "Begin with the default analysis size and overlap to make an analysis file from the source.",
        handoff: "Use the resulting .ana file for spectral smoothing.",
      },
      {
        processId: "blur-avrg",
        modeId: "average",
        instruction:
          "Use a small odd channel count first so neighboring spectral energy is smoothed without losing all definition.",
        handoff: "Use the averaged .ana file to shape the broader spectral contour.",
      },
      {
        processId: "focus-exag",
        modeId: "exaggerate",
        instruction:
          "Increase the amount gradually; positive values widen troughs and negative values widen peaks.",
        handoff: "Use the contour-shaped .ana file as the synthesis input.",
      },
      {
        processId: "pvoc-synthesize",
        modeId: "synthesize",
        instruction:
          "Render the analysis file back to audio and compare it with the original source.",
        handoff: "The rendered mono soundfile is the finished result.",
      },
    ],
  },
  {
    id: "step-pan-render",
    title: "Step, pan, render",
    summary:
      "Lay out several sounds at a fixed interval, pan the arrangement, and render the mixfile.",
    outcome:
      "A rendered sequence whose timing and global spatial position remain editable until the final step.",
    sourceGuidance:
      "Prepare two or more soundfiles in playback order and choose an interval that leaves the desired amount of overlap or silence.",
    tags: ["mix", "sequence", "pan"],
    cautions: [
      "The first two steps create mixfiles, not audio; keep the source soundfiles available when reopening the arrangement.",
    ],
    steps: [
      {
        processId: "submix-atstep",
        modeId: "create",
        instruction: "Add the source files in order and set the fixed time step for their entries.",
        handoff: "Use the stepped .mix file to adjust the arrangement's pan position.",
      },
      {
        processId: "submix-pan",
        modeId: "pan",
        instruction:
          "Choose a global pan value for the mixfile, then retain the panned mixfile for rendering.",
        handoff: "Use the panned .mix file as the render input.",
      },
      {
        processId: "submix-mix",
        modeId: "render",
        instruction: "Render the mixfile once the timing and spatial position are satisfactory.",
        handoff: "The rendered soundfile is the finished sequence.",
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
