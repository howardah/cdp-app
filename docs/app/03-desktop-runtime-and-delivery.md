# CDP Desktop runtime and delivery architecture

## Architecture

```text
Vue + Nuxt UI webviews
  main navigator                     process-* windows
          │ typed Tauri commands and per-run Channel events │
          └──────────────────────┬───────────────────────────┘
                                 ▼
Rust application service
  catalog → validation → command compiler → FIFO queue → process runner
                                                │
                                                ▼
                                  packaged, allowlisted CDP sidecars
```

Rust owns catalog loading, path and file validation, command compilation,
execution, cancellation, run state, and output discovery. Vue owns presentation
and immediate form feedback. Validation is repeated in Rust because webview data
is untrusted.

The initial queue has one worker. This avoids multiple CPU-heavy legacy tools
making the application unresponsive and provides deterministic ordering. The
queue implementation accepts a configurable worker count internally so a later
preference can add bounded concurrency without changing IPC.

## Foundation migration

1. Run the supported Tauri 1-to-2 migration, then review rather than blindly
   accept generated configuration.
2. Move Rust startup into the Tauri 2 `lib.rs`/thin `main.rs` structure.
3. Update JavaScript and Rust packages to mutually compatible Tauri 2 releases.
4. Add the dialog, opener, shell, and window-state plugins. Shell is used only
   from Rust for packaged sidecars and receives no webview permission.
5. Add `@nuxt/ui`, Tailwind CSS, local Lucide icon data, Vue Router, Vitest, Vue
   Test Utils, and the chosen schema validation dependency.
6. Configure `@nuxt/ui/vite`, install `@nuxt/ui/vue-plugin`, import Tailwind and
   Nuxt UI in the root stylesheet, wrap routes in `UApp`, and add `isolate` to
   the root HTML element.
7. Use `createWebHashHistory()` so production asset URLs can load both the main
   and process routes without a server fallback.

Routes are fixed:

| Route | Window | Purpose |
| --- | --- | --- |
| `/#/` | `main` | Navigator and queue access |
| `/#/process/:processId?instance=:uuid` | `process-<uuid>` | One process form/run |

Only Rust creates process windows. `open_process_window` validates the catalog
ID, generates the UUID and window label, and opens the encoded route. Closing a
window removes its webview state but not the Rust run record.

## IPC contract

### Commands

```text
get_process_catalog() -> ProcessDefinition[]
open_process_window(processId: String, prefill?: InputPrefill) -> WindowOpened
inspect_file(path: PathBuf, expectedTypes: CdpFileType[]) -> InspectedFile
suggest_output_path(processId: String, modeId: String, primaryInput: PathBuf) -> PathBuf
preview_process(request: RunProcessRequest) -> CommandPreview
enqueue_process(request: RunProcessRequest, events: Channel<RunEvent>) -> RunAccepted
get_run(runId: Uuid) -> RunSnapshot
list_runs() -> RunSnapshot[]
cancel_process(runId: Uuid) -> CancelResult
reveal_artifact(runId: Uuid, artifactIndex: usize) -> ()
allow_artifact_playback(runId: Uuid, artifactIndex: usize) -> AssetUrl
```

`reveal_artifact` and `allow_artifact_playback` use a run-owned artifact index,
not an arbitrary frontend path. File pickers may return paths to the frontend,
but every processing command canonicalizes and revalidates them.

### File inspection

```ts
interface InspectedFile {
  path: string
  fileType: CdpFileType
  sizeBytes: number
  durationSeconds?: number
  sampleRate?: number
  channels?: number
  sampleFormat?: string
}
```

Use the packaged `sfprops` utility behind a Rust parser for CDP-recognized binary
formats. Treat its stdout as untrusted text: require known field labels, reject
non-zero exit status, and return a typed error when parsing is incomplete. Text
and breakpoint files receive bounded-size Rust validation before use.

### Runs and events

```ts
interface RunAccepted {
  runId: string
  queuePosition: number
}

type RunEvent =
  | { type: 'queued'; position: number }
  | { type: 'started'; startedAt: string }
  | { type: 'log'; stream: 'stdout' | 'stderr'; line: string }
  | { type: 'completed'; finishedAt: string; artifacts: OutputArtifact[] }
  | { type: 'failed'; finishedAt: string; code: RunErrorCode; message: string; details?: string }
  | { type: 'cancelled'; finishedAt: string }

type RunErrorCode =
  | 'invalid-request'
  | 'invalid-input'
  | 'output-exists'
  | 'binary-missing'
  | 'spawn-failed'
  | 'process-failed'
  | 'output-missing'
  | 'cancel-failed'
```

`enqueue_process` validates and compiles before returning. The queued object is
immutable, preventing a later form edit from changing an existing job. Events
are sent to the requesting webview and snapshots are retained in application
state for all windows. History lasts only for the current application session.

## Queue and cancellation

- Maintain a FIFO `VecDeque` plus one active child in Tauri managed state.
- Queued cancellation removes the request and updates the positions of remaining
  requests.
- Active cancellation sends the platform's normal termination signal, waits a
  short bounded interval, then force-kills if necessary.
- A completed cancellation must not report partially created artifacts as
  results. It may report their paths in technical cleanup details.
- Do not automatically delete partial outputs: legacy tools may have produced
  valuable data, and deletion could target an unexpected file. Mark matching new
  files as incomplete and let the user reveal them.
- On application quit, reject new work, cancel queued work, terminate the active
  child, and only then exit. A forced operating-system termination cannot promise
  cleanup.

## Binary resolution and packaging

Create a platform-neutral Rust abstraction:

```rust
trait BinaryResolver {
    fn resolve(&self, id: BinaryId) -> Result<ResolvedBinary, RuntimeError>;
}

trait ProcessRunner {
    async fn spawn(&self, command: CompiledCommand) -> Result<RunningChild, RuntimeError>;
}
```

Production uses packaged sidecars; tests use a fake resolver and runner. The
initial staging step copies these binaries without modifying `cdpr8`:

- `modify`
- `sfedit`
- `pvoc`
- `isolate`
- `sfprops`

After documentation redistribution rights are confirmed, also package
`cdpr8/docs/html/` as a read-only Tauri resource so process reference links work
offline with their relative styles, images, and cross-links. Until that gate is
cleared, the in-app manifest descriptions remain complete and the Reference
action is omitted from release builds rather than linking to a developer path.

Stage them under `src-tauri/binaries/` with Tauri's
`-x86_64-apple-darwin` target suffix and list the unsuffixed logical paths in
`bundle.externalBin`. Preserve executable permissions. Never resolve tools from
`PATH`, the current working directory, or user-controlled environment variables.

Future platform bundles provide the same `BinaryId` set with their target triple
and optional `.exe` suffix. Catalog definitions and frontend behavior must not
contain platform paths or suffixes.

## Capabilities and filesystem boundary

Use separate Tauri 2 capabilities for `main` and `process-*` windows:

- `main`: approved custom catalog/window/run-list commands and the minimum
  window-state permission.
- `process-*`: approved catalog, inspection, preview, enqueue, cancel, result,
  open/save dialog, reveal, and playback commands.
- No webview receives arbitrary shell execute/spawn, broad filesystem read/write,
  or arbitrary opener permissions.
- Native file dialogs provide user-selected paths. Rust commands remain the
  authority for reading those paths.
- Enable the asset protocol only if required for WebView audio. Add only exact,
  successfully produced soundfile paths to runtime scope and return their asset
  URLs through `allow_artifact_playback`; never configure `**/*` access.
- Set an explicit production Content Security Policy covering only bundled
  application assets and the local asset protocol. Remove the starter's null CSP.

Each custom Rust command also receives the calling `WebviewWindow` and checks its
label against the command's allowed window class. Plugin capabilities and this
application-level caller check are both required; a route loaded in the wrong
window must not gain process execution authority.

Tauri 2 uses explicit capabilities, target-suffixed external binaries, and
channels for streamed IPC. Consult the current official references while
implementing:

- [Tauri 1 to 2 migration](https://v2.tauri.app/start/migrate/from-tauri-1/)
- [Embedding external binaries](https://v2.tauri.app/develop/sidecar/)
- [Calling Rust and channel IPC](https://v2.tauri.app/develop/calling-rust/)
- [Dialog plugin](https://v2.tauri.app/plugin/dialog/)
- [Embedding resources and resolving paths](https://v2.tauri.app/develop/resources/)

## Output handling

- Suggest `<input-stem>-<process-slug>.<extension>` in the primary input's
  directory. If unavailable, append `-2`, `-3`, and so on.
- Sanitize generated filename components but preserve the user-selected parent
  path. A manual output choice must use the mode's required extension.
- Recheck collisions immediately before spawn to close the gap between preview
  and execution. V1 never replaces an existing file.
- For generic roots, snapshot only entries matching the trusted manifest rule,
  then inspect matching new entries after success.
- Return stdout as an artifact-like report for information-only modes; do not
  create an undocumented file.
- Playback never starts automatically. Stop playback before allowing an artifact
  to be reused or its owning window to close.

## Test strategy

### Catalog and frontend tests

- Validate every JSON definition and the all-variants contract fixture.
- Reject duplicate IDs, unknown binaries, broken references, impossible defaults,
  invalid output rules, and unsafe flags.
- Test category/search matching, keyboard selection, process launching, prefilled
  inputs, mode switching, scalar/breakpoint switching, multi-input ordering,
  advanced disclosure, validation timing, and result rendering.
- Test every run state and ensure accessible names, descriptions, errors, and
  focus restoration.

### Rust unit tests

- Compile an exact expected argument vector for every supported mode.
- Cover paths with spaces, quotes, Unicode, leading hyphens, and very long names;
  these must remain single arguments and never alter the executable.
- Cover missing/extra values, numeric bounds, power-of-two checks, cross-field
  constraints, input compatibility, output collisions, and generic-root matching.
- Test FIFO order, queued cancellation, active graceful/forced cancellation,
  event ordering, closed listeners, and shutdown.
- Test `sfprops` parsing with supported output, unknown labels, truncated output,
  non-UTF-8 data, and non-zero exit status.

### Integration and real-binary tests

- Use a fake `ProcessRunner` for deterministic stdout, stderr, slow execution,
  non-zero exit, missing output, extra multi-output, and cancellation scenarios.
- On x86_64 macOS, run each command-shape family against disposable copies of
  `cdpr8/_cdp/_cdpenv/testfile.wav` in a temporary directory.
- Verify file existence and type with `sfprops`, not only process exit status.
- Run process tests serially so the queue and fixed fixtures remain deterministic.
- Never write smoke-test output into `cdpr8` or the repository.

### Window and packaging tests

- Verify unique labels and independent state for two instances of one process.
- Close a running process window, observe it through the main queue, and reopen
  its snapshot/results.
- Verify hash routes after a production build and application restart.
- Test the bundled app on a clean Intel macOS account with no CDP path variables.
- Verify only allowlisted sidecars can execute and arbitrary frontend paths fail.
- Confirm nested binary signing, hardened runtime, notarization, quarantine launch,
  and artifact playback from scoped paths.

## Release gates and portability

- Confirm licensing and redistribution rights for CDP binaries and bundled docs.
- Determine whether unsigned upstream binaries can be re-signed inside the app
  bundle and notarized; document the exact signing order.
- Record minimum macOS version and actual behavior on Apple Silicon under Rosetta.
- Do not advertise Apple Silicon, Windows, or Linux support until the same smoke
  and packaging suites pass with native target binaries.
- Keep all OS-specific behavior behind binary resolution, termination, reveal,
  and packaging adapters. Process manifests, IPC payloads, and Vue forms remain
  platform-neutral.
