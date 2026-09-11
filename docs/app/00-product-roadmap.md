# CDP Desktop product roadmap

## Purpose

CDP Desktop will make a useful subset of the Composers' Desktop Project (CDP)
command-line tools approachable from a modern desktop interface. A user should
be able to find a process by musical intent, understand its effect, open a
dedicated process window, configure valid inputs and parameters, run it, and
work with the resulting files without knowing CDP command syntax.

The initial audience includes experienced CDP users, but the interface favors
newcomers through plain-language descriptions, safe defaults, contextual help,
and progressive disclosure. Exact CDP terminology and the generated command
remain available for users who need them.

## Current baseline

- The application is the unmodified Vue 3, TypeScript, Vite, and Tauri 1
  starter. It has no router, UI framework, application state, or processing
  integration.
- `cdpr8/_cdp/_cdprogs/` contains about 230 x86_64 Mach-O executables. A single
  executable can contain several operations and modes, so an executable is not
  equivalent to a user-facing process.
- `cdpr8/docs/html/` contains the most useful local process descriptions and
  command-line references. Some executable help and HTML documentation differ;
  those discrepancies must be resolved during catalog authoring.
- The bundled executables are unsigned Intel macOS binaries. They are suitable
  for the first development target, but redistribution, signing, notarization,
  quarantine behavior, and Apple Silicon compatibility remain release gates.

## Product workflow

1. The navigator opens with process categories, a searchable process list, and
   details for the current selection.
2. Search matches friendly titles, summaries, tags, executable names, operation
   names, and familiar CDP terms.
3. Selecting a process explains what it does, its required input types, its
   possible outputs, its modes, and important limitations.
4. **Open process** creates a new native Tauri window. Multiple windows, including
   multiple instances of the same process, may remain open.
5. The process window guides the user through Input, Mode, Parameters, Output,
   and Run in that order. Advanced controls and the exact command are collapsed
   by default.
6. A valid request enters the global queue. One CDP process runs at a time;
   queued work remains visible and cancellable.
7. Completed artifacts can be auditioned when they are soundfiles, revealed in
   Finder, or passed into another compatible process.

## Initial process catalog

Only processes with complete, tested definitions appear in the navigator.
Unavailable CDP operations are not shown as disabled controls.

| Catalog entry | Supported modes | Contract exercised | Primary local source |
| --- | --- | --- | --- |
| Modify Speed | 1, 2, 5, 6 | Mode-dependent parameters, optional flags, scalar-or-breakpoint values | `cdpr8/docs/html/cgromody.htm#SPEED` |
| Modify Loudness | 1–8 | Optional values, zero/one/many outputs, fixed and variable input counts | `cdpr8/docs/html/cgromody.htm#LOUDNESS` |
| SFEdit Join | None | Ordered variable-length input list and splice flags | `cdpr8/docs/html/cgroedit.htm#JOIN` |
| PVOC Analyze | 1 | Soundfile-to-analysis conversion and mono constraint | `cdpr8/docs/html/cpvocman.htm#USAGEANAL` |
| PVOC Synthesize | None | Analysis-to-soundfile conversion | `cdpr8/docs/html/cpvocman.htm#USAGESYNTH` |
| Isolate | 1–5 | Mode-dependent data files and generic multi-output roots | `cdpr8/docs/html/cgroedit.htm#ISOLATE` |

The following variants are deliberately deferred because their output or
parameter semantics are inconsistent or insufficiently documented:

- Modify Speed modes 3 and 4.
- Modify Loudness modes 11 and 12.
- PVOC Analyze modes 2 and 3.

Adding one of these later requires the same evidence, manifest, command fixture,
and smoke test as a new process.

## Delivery milestones

### 1. Application foundation

- Migrate Tauri 1 to Tauri 2 before feature development.
- Add Vue Router with hash history, Nuxt UI's Vite and Vue plugins, Tailwind CSS,
  locally bundled fonts and icons, and the required `UApp` root.
- Replace the starter screen and global CSS with the navigator shell and design
  tokens defined in [the experience specification](01-experience-and-visual-design.md).
- Establish frontend unit tests and Rust unit/integration test modules.

Exit criterion: the main window and a route-driven placeholder process window
open in development and production builds with keyboard focus and theme support.

### 2. Catalog and generated forms

- Define and validate the manifest contract in
  [the catalog specification](02-process-catalog-contract.md).
- Author the initial process definitions from both local documentation and
  executable help output.
- Implement navigator search, categories, compatibility filtering, process
  details, schema-driven forms, and client-side validation.
- Render a read-only command preview from backend-produced preview data; the
  frontend must not compile executable arguments.

Exit criterion: every initial mode renders the correct inputs, output rule,
parameters, defaults, constraints, and help without process-specific Vue forms.

### 3. Processing runtime

- Implement the Rust-owned binary resolver, command compiler, file inspection,
  output suggestion, FIFO queue, cancellation, event channel, and run registry.
- Stage and invoke the approved sidecars without exposing shell execution to a
  webview.
- Preserve run status and results if a process window closes.

Exit criterion: all command shapes can be exercised through a fake runner, and
real CDP smoke tests produce expected artifacts in temporary directories.

### 4. Output workflow and hardening

- Add result auditioning, reveal-in-Finder, compatible-process reuse, queue
  visibility, actionable errors, and exact-path asset scoping.
- Test cancellation, window lifecycle, unusual paths, multi-output discovery,
  missing binaries, malformed input files, and application shutdown.
- Perform accessibility and light/dark visual passes.

Exit criterion: a newcomer can complete the full workflow without reading CLI
documentation, while an experienced user can verify the exact CDP invocation.

### 5. Intel macOS release candidate

- Produce an x86_64 macOS application that includes only the required binaries.
- Verify executable permissions, signing order for nested executables, hardened
  runtime behavior, notarization, and a clean-machine launch.
- Resolve CDP redistribution rights before distributing any build.
- Record known Rosetta behavior on Apple Silicon without claiming native
  support.

Exit criterion: the signed/notarized artifact passes all automated and manual
acceptance checks on the supported Intel macOS baseline.

## Later phases

- Supply native Apple Silicon, Windows, and Linux binary sets through the same
  binary resolver and staging manifest by following the
  [cross-platform build guide](05-cross-platform-builds.md).
- Expand the catalog by complete operation/mode definitions, not by blindly
  exposing executables; use the [process authoring guide](04-adding-processes.md)
  for every addition.
- Guided recipes may recommend compatible process sequences and open a chosen
  process mode. Automated chains, saved presets, persistent run history, batch
  processing, waveform visualization, and editable breakpoint curves remain
  deferred until the core contracts have proven stable.

## Definition of done

- Every visible process can complete end to end with its documented inputs.
- The UI never accepts an executable name or arbitrary argument array from a
  user or webview.
- Existing output files are never overwritten in the first release.
- All generated artifacts are reported, including multi-file outputs.
- Errors state what failed and what the user can do next; raw output is available
  as optional technical detail.
- All controls work by keyboard, all form errors are associated with fields, and
  no status depends on color alone.
- The app runs without a separately installed CDP environment.

## Non-goals for the first release

- Complete coverage of CDP Release 8.
- Mobile layouts or mobile Tauri targets.
- Real-time processing or live parameter preview.
- Waveform editing or a digital audio workstation timeline.
- Automatic runtime parsing of CLI help or legacy Sound Loom metadata tools.
- Persistent projects, processing history, or presets.
