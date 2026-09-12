# CDP process catalog contract

## Goals

The catalog is the single source of truth for what CDP Desktop can display and
execute. It must express processes with different modes, file types, parameter
shapes, input cardinalities, and output behaviors without process-specific Vue
components or arbitrary runtime code.

Catalog manifests are version-controlled JSON files validated during tests and
application startup. Human-readable CLI output is evidence used while authoring
a manifest, never a runtime API. Rust deserializes the manifests and returns the
same data to the frontend through `get_process_catalog`.

## Files and versioning

Use this eventual source layout:

```text
src/processes/
  catalog.schema.json
  catalog.types.ts
  definitions/
    modify-speed.json
    modify-loudness.json
    sfedit-join.json
    pvoc-analyze.json
    pvoc-synthesize.json
    isolate.json
src-tauri/src/catalog/
  mod.rs
  types.rs
```

Every definition contains `schemaVersion: 1`. A schema change that invalidates
an existing definition increments the version and adds an explicit migration or
updates every definition in the same change. Stable process, mode, input, and
parameter IDs must not change when display copy changes.

TypeScript and Rust maintain matching serializable types. A checked-in fixture
containing every union variant must deserialize in Rust and validate against the
JSON Schema in frontend tests, preventing silent drift.

## Core model

```ts
interface ProcessDefinition {
  schemaVersion: 1
  id: string
  title: string
  category: ProcessCategory
  summary: string
  description: string
  useCases: string[]
  tags: string[]
  identity: {
    executable: BinaryId
    operation?: string
  }
  documentation: {
    localPath: string
    anchor?: string
    helpCommand: string[]
    verifiedWithVersion: string
  }
  modes: ModeDefinition[]
}

interface ModeDefinition {
  id: string
  cliMode?: number
  title: string
  summary: string
  inputs: InputDefinition[]
  parameters: ParameterDefinition[]
  output: OutputDefinition
  argumentOrder: ArgumentToken[]
  constraints: Constraint[]
}

type ProcessCategory =
  | 'time-domain'
  | 'spectral'
  | 'edit-and-mix'
  | 'utilities'

type BinaryId = 'modify' | 'sfedit' | 'pvoc' | 'isolate'
```

`BinaryId` is a closed set mapped to packaged binaries in Rust. It is not a path.
`helpCommand` is documentation provenance and is never executed in response to a
frontend request.

The catalog also supports the following reviewed file and output vocabulary:

- `binary-envelope` (`.env`) and `mixfile` (`.mix`) are typed CDP data files;
  they are validated by extension and never treated as sound output.
- `sameDuration` is an input metadata constraint. Runtime validation allows a
  tolerance of one microsecond for inspected durations.
- `autoNamedGeneric` is a generic numbered output whose root name is suggested
  by the app and whose numbered artifacts are discovered by the Rust runtime.
- `composite` describes a fixed set of typed components emitted beneath one
  output root. Components are declarative; manifests cannot provide shell text
  or arbitrary naming expressions.

These variants are serialized identically by TypeScript and Rust. Composite
outputs use the declared root extension for path validation and are discovered
through the same packaged-runtime boundary as generic outputs.

### Inputs

```ts
interface InputDefinition {
  id: string
  label: string
  description: string
  fileTypes: CdpFileType[]
  minItems: number
  maxItems: number | null
  ordered: boolean
  constraints: FileConstraint[]
}

type CdpFileType =
  | 'soundfile'
  | 'analysis-ana'
  | 'analysis-pvx'
  | 'breakpoint'
  | 'cuts-data'
  | 'slice-data'
  | 'text-data'

type FileConstraint =
  | { kind: 'channels'; min: number; max: number }
  | { kind: 'sameSampleRate'; inputId: string }
  | { kind: 'sameChannels'; inputId: string }
  | { kind: 'sameSampleFormat'; inputId: string }
```

An ordered multi-input definition preserves the visible drag/reorder sequence in
the generated command. `maxItems: null` means unbounded. SFEdit Join sets
`minItems: 2`, while Modify Loudness modes 7 and 8 also accept two or more files.

### Parameters

```ts
interface ParameterBase {
  id: string
  label: string
  description: string
  required: boolean
  advanced: boolean
  unit?: string
  cli: CliBinding
}

type ParameterDefinition =
  | (ParameterBase & {
      kind: 'number' | 'integer'
      default: number
      min?: number
      max?: number
      step?: number
    })
  | (ParameterBase & {
      kind: 'choice'
      default: string
      choices: Array<{ value: string; label: string; cliValue: string }>
    })
  | (ParameterBase & { kind: 'flag'; default: boolean })
  | (ParameterBase & {
      kind: 'file'
      fileTypes: CdpFileType[]
    })
  | (ParameterBase & {
      kind: 'numberOrBreakpoint'
      default: { kind: 'number'; value: number }
      min?: number
      max?: number
      step?: number
      breakpointType: 'time-value-pairs'
    })

type CliBinding =
  | { kind: 'positional' }
  | { kind: 'option'; flag: string; join: 'concatenated' | 'separate' }
  | { kind: 'booleanFlag'; flag: string }
```

A `numberOrBreakpoint` value serializes as either a number or a selected file
path in the same positional slot. Breakpoint content validation is process-aware:
times must be finite and strictly increasing; values must satisfy the parameter's
range. The original file is passed to CDP after validation rather than rewritten.

### Outputs

```ts
type OutputDefinition =
  | {
      kind: 'singleFile'
      fileType: CdpFileType
      extension: string
      nameSuffix: string
    }
  | {
      kind: 'genericRoot'
      fileType: CdpFileType
      extension: string
      nameSuffix: string
      discovery: OutputDiscoveryRule
    }
  | { kind: 'stdoutReport' }
  | { kind: 'none' }

interface OutputDiscoveryRule {
  prefixStyle: 'root-number' | 'root-number-padded'
  mayProduceRemnant: boolean
}

interface OutputArtifact {
  path: string
  fileType: CdpFileType
  sizeBytes: number
  playable: boolean
  metadata?: InspectedFile
}
```

The runner snapshots only the output directory entries matching the trusted
discovery rule before execution, then returns matching new files afterward. A
process is successful only if its exit status is zero and its required output
shape is present.

### Constraints

Constraints are declarative and evaluated by both frontend and Rust. They never
contain JavaScript, regular expressions supplied by users, or expression text.

```ts
type ValueRef =
  | { source: 'parameter'; id: string }
  | { source: 'inputMetadata'; inputId: string; property: 'duration' | 'channels' }
  | { source: 'literal'; value: number }

type Constraint =
  | { kind: 'lessThan'; left: ValueRef; right: ValueRef; message: string }
  | { kind: 'lessThanOrEqual'; left: ValueRef; right: ValueRef; message: string }
  | { kind: 'greaterThan'; left: ValueRef; right: ValueRef; message: string }
  | { kind: 'powerOfTwo'; value: ValueRef; message: string }
```

Examples include Isolate mode 3 requiring `dBoff < dBon`, its minimum accepted
segment duration exceeding twice the splice length, and PVOC analysis points
being a power of two between 2 and 32768.

## Command compilation

The frontend submits typed values, never command tokens:

```ts
type ParameterValue =
  | { kind: 'number'; value: number }
  | { kind: 'choice'; value: string }
  | { kind: 'flag'; value: boolean }
  | { kind: 'file'; path: string }

interface RunProcessRequest {
  processId: string
  modeId: string
  inputs: Record<string, string[]>
  parameters: Record<string, ParameterValue>
  outputPath: string | null
}
```

Rust compiles the request as follows:

1. Look up `processId` and `modeId` in the loaded immutable catalog.
2. Reject missing required, extra, or mismatched input and parameter keys; allow
   an optional parameter to be omitted.
3. Canonicalize and inspect input paths; validate types and relationships.
4. Validate all parameter ranges, breakpoint contents, and cross-field rules.
5. Require an output path for file-producing modes, require `null` for report or
   no-output modes, validate the output parent directory, and reject an existing
   path or any existing file matching a multi-output root.
6. Resolve `BinaryId` through the platform binary resolver.
7. Walk `argumentOrder`, emitting operation, mode, input, output, positional,
   option, and flag tokens as individual `OsString` values.
8. Return a redacted preview and enqueue the immutable compiled command.

`ArgumentToken` is a closed union of trusted references:

```ts
type ArgumentToken =
  | { kind: 'literal'; value: string }
  | { kind: 'mode' }
  | { kind: 'input'; inputId: string }
  | { kind: 'output' }
  | { kind: 'parameter'; parameterId: string }
```

Paths are never quoted manually and the runner never invokes a shell.

## Initial definition decisions

### Modify Speed

- Modes 1 and 2 accept a scalar or time/value breakpoint file and optional
  `-o`. Safe defaults are ratio `1.0` and semitones `0.0`.
- Mode 5 requires acceleration ratio and goal time; start time is an optional
  concatenated `-s` value.
- Mode 6 requires vibrato rate `0–120` Hz and depth `0–96` semitones; both accept
  scalar or breakpoint files.
- All supported modes consume one soundfile and produce one soundfile.

### Modify Loudness

- Mode 1 is linear gain; mode 2 is dB gain in `-96–96` dB.
- Modes 3 and 4 expose optional output level `0–1`; absence means CDP's maximum
  default.
- Mode 5 consumes exactly two soundfiles and produces one.
- Mode 6 consumes one soundfile and produces one without parameters.
- Mode 7 consumes two or more soundfiles and returns a stdout report without an
  output path.
- Mode 8 consumes two or more soundfiles and uses a generic multi-output root.

### SFEdit Join

- Accept two or more ordered, mutually compatible soundfiles.
- Default splice length is 15 ms; flags control first-file fade-in and last-file
  fade-out.
- Produce one soundfile.

### PVOC

- Analyze mode 1 accepts one mono soundfile and produces one `.ana` file.
- Analysis points are a power of two from 2 through 32768, defaulting to 1024.
- Overlap is an integer from 1 through 4, defaulting to 3.
- Synthesize accepts one `.ana` analysis file and produces one mono soundfile.

### Isolate

- Support all five documented modes with their mode-specific cuts/slice file or
  dB threshold controls.
- Splice defaults to 15 ms and is constrained to `0–500` ms. Mode 5 dovetail
  defaults to 5 ms and is constrained to `0–20` ms.
- Modes 1–3 may produce a remnant; all modes use a generic multi-output root.
- Reverse and equal-length/silence flags remain explicit advanced controls.

## Authoring checklist

The operational, file-by-file procedure is maintained in
[Adding processes to CDP Desktop](04-adding-processes.md). The checklist below
summarizes the evidence required for every process or mode.

For every new process or mode:

1. Locate its user-facing entry in `cdpr8/docs/html/ccdpndex.htm` and its detailed
   local reference page.
2. Capture no-argument and operation-specific executable help and record the
   executable version.
3. Resolve discrepancies explicitly; do not guess from neighboring processes.
4. Author plain-language outcome, use cases, safe defaults, file rules, complete
   argument order, output discovery, and documentation provenance.
5. Add valid, boundary, and invalid catalog fixtures.
6. Add an exact expected-argument test and a disposable real-binary smoke test.
7. Only then add the definition to the visible catalog index.
