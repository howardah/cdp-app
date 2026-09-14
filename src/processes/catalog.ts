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
import cut from "./definitions/sfedit-cut.json";
import cutend from "./definitions/sfedit-cutend.json";
import excise from "./definitions/sfedit-excise.json";
import insil from "./definitions/sfedit-insil.json";
import insert from "./definitions/sfedit-insert.json";
import replace from "./definitions/sfedit-replace.json";
import cutmany from "./definitions/sfedit-cutmany.json";
import masks from "./definitions/sfedit-masks.json";
import space from "./definitions/modify-space.json";
import revecho from "./definitions/modify-revecho.json";
import zcut from "./definitions/sfedit-zcut.json";
import zcuts from "./definitions/sfedit-zcuts.json";
import excises from "./definitions/sfedit-excises.json";
import noisecut from "./definitions/sfedit-noisecut.json";
import radical from "./definitions/modify-radical.json";
import convolve from "./definitions/modify-convolve.json";
import stack from "./definitions/modify-stack.json";
import shudder from "./definitions/modify-shudder.json";
import scaledpan from "./definitions/modify-scaledpan.json";
import brassage from "./definitions/modify-brassage.json";
import sausage from "./definitions/modify-sausage.json";
import spaceform from "./definitions/modify-spaceform.json";
import findpan from "./definitions/modify-findpan.json";
import reverb from "./definitions/reverb.json";
import filterFixed from "./definitions/filter-fixed.json";
import filterLohi from "./definitions/filter-lohi.json";
import filterVariable from "./definitions/filter-variable.json";
import filterSweeping from "./definitions/filter-sweeping.json";
import filterPhasing from "./definitions/filter-phasing.json";
import filterIterated from "./definitions/filter-iterated.json";
import filterBank from "./definitions/filter-bank.json";
import filterUserbank from "./definitions/filter-userbank.json";
import filterVaribank from "./definitions/filter-varibank.json";
import phasor from "./definitions/phasor.json";
import blurAvrg from "./definitions/blur-avrg.json";
import blurBlur from "./definitions/blur-blur.json";
import blurChorus from "./definitions/blur-chorus.json";
import blurDrunk from "./definitions/blur-drunk.json";
import blurNoise from "./definitions/blur-noise.json";
import blurScatter from "./definitions/blur-scatter.json";
import blurSpread from "./definitions/blur-spread.json";
import extendDrunk from "./definitions/extend-drunk.json";
import extendRepetitions from "./definitions/extend-repetitions.json";
import extendScramble from "./definitions/extend-scramble.json";
import extendSequence from "./definitions/extend-sequence.json";
import extendZigzag from "./definitions/extend-zigzag.json";
import hover from "./definitions/hover.json";
import hover2 from "./definitions/hover2.json";
import sfecho from "./definitions/sfecho-echo.json";
import spectstr from "./definitions/spectstr.json";
import stretchSpectrum from "./definitions/stretch-spectrum.json";
import stretchTime from "./definitions/stretch-time.json";
import submixBalance from "./definitions/submix-balance.json";
import submixMerge from "./definitions/submix-merge.json";
import submixMergemany from "./definitions/submix-mergemany.json";
import submixCrossfade from "./definitions/submix-crossfade.json";
import submixMix from "./definitions/submix-mix.json";
import submixInterleave from "./definitions/submix-interleave.json";
import submixPan from "./definitions/submix-pan.json";
import submixSpacewarp from "./definitions/submix-spacewarp.json";
import blurShuffle from "./definitions/blur-shuffle.json";
import iterline from "./definitions/iterline.json";
import iterlinef from "./definitions/iterlinef.json";
import submixInbetween from "./definitions/submix-inbetween.json";
import submixInbetween2 from "./definitions/submix-inbetween2.json";
import submixSync from "./definitions/submix-sync.json";
import submixSyncattack from "./definitions/submix-syncattack.json";
import submixTimewarp from "./definitions/submix-timewarp.json";
import submixFaders from "./definitions/submix-faders.json";
import submixAddtomix from "./definitions/submix-addtomix.json";
import blurSuppress from "./definitions/blur-suppress.json";
import blurWeave from "./definitions/blur-weave.json";
import combineCross from "./definitions/combine-cross.json";
import combineDiff from "./definitions/combine-diff.json";
import combineInterleave from "./definitions/combine-interleave.json";
import combineMax from "./definitions/combine-max.json";
import combineMean from "./definitions/combine-mean.json";
import combineSum from "./definitions/combine-sum.json";
import focusAccu from "./definitions/focus-accu.json";
import focusExag from "./definitions/focus-exag.json";
import focusFocus from "./definitions/focus-focus.json";
import focusFold from "./definitions/focus-fold.json";
import focusFreeze from "./definitions/focus-freeze.json";
import focusHold from "./definitions/focus-hold.json";
import envelAttack from "./definitions/envel-attack.json";
import envelCurtail from "./definitions/envel-curtail.json";
import envelDovetail from "./definitions/envel-dovetail.json";
import envelTremolo from "./definitions/envel-tremolo.json";
import envelCyclic from "./definitions/envel-cyclic.json";
import envelSwell from "./definitions/envel-swell.json";
import envelPluck from "./definitions/envel-pluck.json";
import envelWarp from "./definitions/envel-warp.json";
import envelImpose from "./definitions/envel-impose.json";
import envelReplace from "./definitions/envel-replace.json";
import envelExtract from "./definitions/envel-extract.json";
import envelCreate from "./definitions/envel-create.json";
import envnuExpdecay from "./definitions/envnu-expdecay.json";
import envnuPeakchop from "./definitions/envnu-peakchop.json";
import flatten from "./definitions/flatten.json";
import joinseq from "./definitions/sfedit-joinseq.json";
import joindyn from "./definitions/sfedit-joindyn.json";
import randchunks from "./definitions/sfedit-randchunks.json";
import twixt from "./definitions/sfedit-twixt.json";
import sphinx from "./definitions/sfedit-sphinx.json";
import syllables from "./definitions/sfedit-syllables.json";
import baktobak from "./definitions/extend-baktobak.json";
import bounce from "./definitions/bounce.json";
import freeze from "./definitions/extend-freeze.json";
import iterate from "./definitions/extend-iterate.json";
import loop from "./definitions/extend-loop.json";
import envelBrktoenv from "./definitions/envel-brktoenv.json";
import envelDbtoenv from "./definitions/envel-dbtoenv.json";
import extendDoublets from "./definitions/extend-doublets.json";
import focusStep from "./definitions/focus-step.json";
import envelDbtogain from "./definitions/envel-dbtogain.json";
import envelEnvtobrk from "./definitions/envel-envtobrk.json";
import envelEnvtodb from "./definitions/envel-envtodb.json";
import envelGaintodb from "./definitions/envel-gaintodb.json";
import envelReshape from "./definitions/envel-reshape.json";
import envelReplot from "./definitions/envel-replot.json";
import envelScaled from "./definitions/envel-scaled.json";
import envelTimegrid from "./definitions/envel-timegrid.json";
import filterBankfrqs from "./definitions/filter-bankfrqs.json";
import filterVfilters from "./definitions/filter-vfilters.json";
import submixAtstep from "./definitions/submix-atstep.json";
import submixAttenuate from "./definitions/submix-attenuate.json";
import submixDummy from "./definitions/submix-dummy.json";
import submixGetlevel from "./definitions/submix-getlevel.json";
import submixModel from "./definitions/submix-model.json";
import submixOngrid from "./definitions/submix-ongrid.json";
import submixShuffle from "./definitions/submix-shuffle.json";
import submixTest from "./definitions/submix-test.json";
import combineMake from "./definitions/combine-make.json";
import combineMake2 from "./definitions/combine-make2.json";

export const processCatalog = [
  speed,
  loudness,
  join,
  analyze,
  synthesize,
  isolate,
  cut,
  cutend,
  excise,
  insil,
  insert,
  replace,
  cutmany,
  masks,
  space,
  revecho,
  zcut,
  zcuts,
  excises,
  noisecut,
  radical,
  convolve,
  stack,
  shudder,
  scaledpan,
  brassage,
  sausage,
  spaceform,
  findpan,
  reverb,
  filterFixed,
  filterLohi,
  filterVariable,
  filterSweeping,
  filterPhasing,
  filterIterated,
  filterBank,
  filterUserbank,
  filterVaribank,
  phasor,
  blurAvrg,
  blurBlur,
  blurChorus,
  blurDrunk,
  blurNoise,
  blurScatter,
  blurSpread,
  extendDrunk,
  extendRepetitions,
  extendScramble,
  extendSequence,
  extendZigzag,
  hover,
  hover2,
  sfecho,
  spectstr,
  stretchSpectrum,
  stretchTime,
  submixBalance,
  submixMerge,
  submixMergemany,
  submixCrossfade,
  submixMix,
  submixInterleave,
  submixPan,
  submixSpacewarp,
  blurSuppress,
  blurWeave,
  blurShuffle,
  iterline,
  iterlinef,
  submixInbetween,
  submixInbetween2,
  submixSync,
  submixSyncattack,
  submixTimewarp,
  submixFaders,
  submixAddtomix,
  combineCross,
  combineDiff,
  combineInterleave,
  combineMax,
  combineMean,
  combineSum,
  focusAccu,
  focusExag,
  focusFocus,
  focusFold,
  focusFreeze,
  focusHold,
  envelAttack,
  envelCurtail,
  envelDovetail,
  envelTremolo,
  envelCyclic,
  envelSwell,
  envelPluck,
  envelWarp,
  envelImpose,
  envelReplace,
  envelExtract,
  envelCreate,
  envnuExpdecay,
  envnuPeakchop,
  flatten,
  joinseq,
  joindyn,
  randchunks,
  twixt,
  sphinx,
  syllables,
  baktobak,
  bounce,
  freeze,
  iterate,
  loop,
  envelBrktoenv,
  envelDbtoenv,
  extendDoublets,
  focusStep,
  envelDbtogain,
  envelEnvtobrk,
  envelEnvtodb,
  envelGaintodb,
  envelReshape,
  envelReplot,
  envelScaled,
  envelTimegrid,
  filterBankfrqs,
  filterVfilters,
  submixAtstep,
  submixAttenuate,
  submixDummy,
  submixGetlevel,
  submixModel,
  submixOngrid,
  submixShuffle,
  submixTest,
  combineMake,
  combineMake2,
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
