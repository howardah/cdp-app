# Extending CDP Desktop to new platforms

## Support contract

CDP Desktop consists of two native layers: the Tauri application and the CDP
sidecars it invokes. A target is supported only when both layers are built for
the target, packaged together, and pass clean-machine smoke tests. Renaming a
foreign-architecture binary to a Tauri target suffix does not make it native.

The intended matrix is:

| Platform | Rust/Tauri target | Required sidecar suffix | Status |
| --- | --- | --- | --- |
| macOS Intel | `x86_64-apple-darwin` | `-x86_64-apple-darwin` | `building` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `-aarch64-apple-darwin` | `planned` |
| Windows x64 | `x86_64-pc-windows-msvc` | `-x86_64-pc-windows-msvc.exe` | `planned` |
| Windows ARM64 | `aarch64-pc-windows-msvc` | `-aarch64-pc-windows-msvc.exe` | `planned` |
| Linux x64 | `x86_64-unknown-linux-gnu` | `-x86_64-unknown-linux-gnu` | `planned` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `-aarch64-unknown-linux-gnu` | `planned` |

These statuses describe release qualification, not resolver availability. Intel
macOS remains `building` until its unsigned bundled sidecars pass signing and
clean-machine qualification; no target is currently `released`.

Build separate macOS artifacts for Intel and Apple Silicon first. A universal
macOS artifact is optional and requires universal CDP sidecars as well as the
Tauri shell.

## 1. Pin and audit CDP source

Use the upstream [ComposersDesktop/CDP8 repository](https://github.com/ComposersDesktop/CDP8)
as the source of record. Record the commit SHA, source URL, license, local
patches, compiler, CMake version, and target triple with every staged binary.
Build in a separate checkout or CI workspace; do not modify the bundled
`cdpr8` release tree.

The upstream [building guide](https://github.com/ComposersDesktop/CDP8/blob/main/building.txt)
uses CMake and writes programs to `NewRelease/`:

```sh
cmake -S . -B build
cmake --build build --config Release
```

Install the platform dependencies described upstream, including the supported
C toolchain and any libraries required by the selected programs. This app needs
only the allowlisted executables in `bundle.externalBin`; do not make unrelated
playback or recording programs a release dependency.

Windows needs a separate feasibility pass. The upstream guide currently
documents an older 32-bit MinGW build, while this application's Windows targets
use Rust's x64 and ARM64 MSVC triples. Do not claim native Windows support until
the required CDP programs have been compiled and smoke-tested for the matching
architecture. Keep any required portability patches small, documented, and
submitted upstream where practical.

## 2. Build and verify the CDP sidecars

Build on the target architecture when possible. Native macOS and Windows
runners avoid installer and SDK cross-compilation gaps; native Linux ARM64
runners avoid AppImage cross-compilation limitations.

For each target:

1. Build `modify`, `sfedit`, `pvoc`, `isolate`, and `sfprops`, plus any later
   executable added through the process-authoring checklist.
2. Confirm the binary architecture and dynamic-library dependencies with the
   platform tools (`file`/`otool`, `dumpbin`, or `file`/`ldd`).
3. Run executable help and an actual command from each command-shape family.
4. Verify output existence and type with the target build of `sfprops`.
5. Record SHA-256 hashes and copy only approved executables into
   `src-tauri/binaries/` using the exact matrix suffix.
6. Preserve executable bits on macOS/Linux. On Windows, place `.exe` after the
   target suffix.

Generalize `scripts/stage-cdp-binaries.sh` before using it outside Intel macOS:
accept an explicit source directory and target triple, map the expected object
format/architecture per target, refuse unknown targets, and stage atomically
only after every required source passes validation. Never infer a release target
from an untrusted filename.

## 3. Extend runtime resolution

Tauri's `bundle.externalBin` entries remain unsuffixed logical names. During a
build, Tauri selects files named with the target triple documented in its
[sidecar guide](https://v2.tauri.app/develop/sidecar/).

Before enabling the full matrix:

1. Add `aarch64-pc-windows-msvc` and `aarch64-unknown-linux-gnu` cases beside
   the existing target constants in `runtime.rs`.
2. Make development fallback resolution append `.exe` on Windows.
3. Make packaged resolution handle Tauri's bundled resource filename while
   retaining target-suffixed lookup for development binaries.
4. Fail compilation for an unsupported target instead of using an empty suffix.
5. Keep platform-specific termination, reveal, signing, and packaging behavior
   behind Rust adapters; catalog manifests and Vue forms remain platform-neutral.
6. Add resolver tests for every matrix name, missing files, wrong extensions,
   and unsupported targets.

The application must still reject arbitrary binary paths and must not search
`PATH`, the current working directory, or user-controlled environment variables.

## 4. Build the Tauri application

Install the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) on
the target host, install the matching Rust target, stage all required sidecars,
then build explicitly:

```sh
rustup target add <target-triple>
bun run typecheck
bun test
cargo test --manifest-path src-tauri/Cargo.toml
bun tauri build --target <target-triple>
```

Use native packaging jobs for macOS DMG/app bundles, Windows MSI/NSIS, and Linux
Deb/AppImage/RPM as appropriate. Tauri documents native Windows ARM64 builds and
native/cross Linux ARM64 considerations in its
[Windows installer](https://v2.tauri.app/distribute/windows-installer/) and
[AppImage](https://v2.tauri.app/distribute/appimage/) guides.

Platform-facing copy must also be neutralized before Windows/Linux release:
replace Finder-only wording with the operating system's reveal terminology and
verify path separators, executable extensions, cancellation, file dialogs,
artifact URLs, and installer icons on the real platform.

## 5. Sign, package, and qualify

For each matrix entry:

1. Run unit and integration tests with the exact staged binaries.
2. Exercise every supported process family using disposable fixtures.
3. Inspect the finished bundle to confirm all and only allowlisted sidecars are
   present and have the expected architecture.
4. Test install, first launch, processing, cancellation, playback, reveal, and
   uninstall on a clean machine with no separate CDP installation.
5. Sign/notarize macOS nested executables before the outer app, and sign Windows
   binaries/installers using the chosen release service. Verify signatures after
   packaging.
6. Archive checksums, test results, CDP source revision, patches, licenses, and
   signing metadata with the artifact.

Add a native-runner CI matrix only after a target passes this procedure
manually. A green Tauri compile without real CDP smoke tests is not sufficient.

## Promotion rule

Track targets as `planned`, `building`, `verified`, or `released`. The UI,
README, and downloads page may advertise only `released` targets. A new CDP
process returns every platform to at least `building` until its new executable
coverage passes the same staging, smoke, and packaging checks.
