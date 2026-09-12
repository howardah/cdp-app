export type ProcessCategory = "time-domain" | "spectral" | "edit-and-mix" | "utilities";
export type BinaryId =
  | "modify"
  | "sfedit"
  | "pvoc"
  | "isolate"
  | "sfprops"
  | "blur"
  | "bounce"
  | "combine"
  | "envel"
  | "envnu"
  | "extend"
  | "filter"
  | "flatten"
  | "focus"
  | "hover"
  | "hover2"
  | "iterline"
  | "iterlinef"
  | "phasor"
  | "reverb"
  | "sfecho"
  | "spectstr"
  | "stretch"
  | "submix";
export type CdpFileType =
  | "soundfile"
  | "analysis-ana"
  | "analysis-pvx"
  | "breakpoint"
  | "cuts-data"
  | "slice-data"
  | "text-data"
  | "binary-envelope"
  | "mixfile"
  | "domain-image";
export interface InspectedFile {
  path: string;
  fileType: CdpFileType;
  sizeBytes: number;
  durationSeconds?: number;
  sampleRate?: number;
  channels?: number;
  sampleFormat?: string;
}

export interface FileConstraint {
  kind: "channels" | "sameSampleRate" | "sameChannels" | "sameSampleFormat" | "sameDuration";
  min?: number;
  max?: number;
  inputId?: string;
}
export interface InputDefinition {
  id: string;
  label: string;
  description: string;
  fileTypes: CdpFileType[];
  minItems: number;
  maxItems: number | null;
  ordered: boolean;
  constraints: FileConstraint[];
}
export type CliBinding =
  | { kind: "positional" }
  | { kind: "option"; flag: string; join: "concatenated" | "separate" }
  | { kind: "booleanFlag"; flag: string };
export interface ParameterBase {
  id: string;
  label: string;
  description: string;
  required: boolean;
  advanced: boolean;
  unit?: string;
  cli: CliBinding;
}
export type ParameterDefinition =
  | (ParameterBase & {
      kind: "number" | "integer";
      default: number;
      min?: number;
      max?: number;
      step?: number;
    })
  | (ParameterBase & {
      kind: "choice";
      default: string;
      choices: { value: string; label: string; cliValue: string }[];
    })
  | (ParameterBase & { kind: "flag"; default: boolean })
  | (ParameterBase & { kind: "file"; fileTypes: CdpFileType[] })
  | (ParameterBase & {
      kind: "numberOrBreakpoint";
      default: { kind: "number"; value: number };
      min?: number;
      max?: number;
      step?: number;
      breakpointType: "time-value-pairs";
    });
export type ValueRef =
  | { source: "parameter"; id: string }
  | { source: "inputMetadata"; inputId: string; property: "duration" | "channels" }
  | { source: "literal"; value: number };
export type Constraint =
  | {
      kind: "lessThan" | "lessThanOrEqual" | "greaterThan";
      left: ValueRef;
      right: ValueRef;
      message: string;
    }
  | { kind: "powerOfTwo"; value: ValueRef; message: string };
export type ArgumentToken =
  | { kind: "literal"; value: string }
  | { kind: "mode" }
  | { kind: "input"; inputId: string }
  | { kind: "output" }
  | { kind: "parameter"; parameterId: string };
export type OutputDefinition =
  | { kind: "singleFile"; fileType: CdpFileType; extension: string; nameSuffix: string }
  | {
      kind: "genericRoot";
      fileType: CdpFileType;
      extension: string;
      nameSuffix: string;
      discovery: { prefixStyle: "root-number" | "root-number-padded"; mayProduceRemnant: boolean };
    }
  | {
      kind: "autoNamedGeneric";
      fileType: CdpFileType;
      extension: string;
      nameSuffix: string;
      discovery: { prefixStyle: "root-number" | "root-number-padded"; mayProduceRemnant: boolean };
    }
  | {
      kind: "composite";
      extension: string;
      nameSuffix: string;
      components: { fileType: CdpFileType; extension: string; nameSuffix: string }[];
    }
  | { kind: "stdoutReport" }
  | { kind: "none" };
export interface ModeDefinition {
  id: string;
  cliMode?: number;
  title: string;
  summary: string;
  inputs: InputDefinition[];
  parameters: ParameterDefinition[];
  output: OutputDefinition;
  argumentOrder: ArgumentToken[];
  constraints: Constraint[];
}
export interface ProcessDefinition {
  schemaVersion: 1;
  id: string;
  title: string;
  category: ProcessCategory;
  summary: string;
  description: string;
  useCases: string[];
  tags: string[];
  identity: { executable: BinaryId; operation?: string };
  documentation: {
    localPath: string;
    anchor?: string;
    helpCommand: string[];
    verifiedWithVersion: string;
  };
  modes: ModeDefinition[];
}
export type ParameterValue =
  | { kind: "number"; value: number }
  | { kind: "choice"; value: string }
  | { kind: "flag"; value: boolean }
  | { kind: "file"; path: string; value?: string };
export interface RunProcessRequest {
  processId: string;
  modeId: string;
  inputs: Record<string, string[]>;
  parameters: Record<string, ParameterValue>;
  outputPath: string | null;
}
