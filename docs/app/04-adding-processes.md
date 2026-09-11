# Adding processes to CDP Desktop

## Purpose

A user-facing process is a reviewed catalog entry, not merely an executable.
Every visible mode must have verified command semantics, safe defaults, complete
input/output rules, frontend and Rust validation, and a packaged binary on each
supported platform. Do not expose partially documented operations as disabled
or experimental controls.

## 1. Gather evidence

1. Find the process in `cdpr8/docs/html/ccdpndex.htm` and read its detailed
   reference page, including every mode being proposed.
2. Run the executable with no arguments and with its operation-specific help
   arguments. Record the CDP version and the exact commands used.
3. Compare the reference with the executable output. Resolve discrepancies by
   testing the binary; do not copy the shape of a neighboring process.
4. Write down the executable and operation, CLI mode number, ordered inputs,
   parameter order, flags, valid ranges, defaults, output behavior, and any
   relationships between fields.
5. Confirm that the process works with packaged CDP rather than relying on
   `PATH`, shell aliases, or a user's CDP environment.

## 2. Author the manifest

Create `src/processes/definitions/<process-id>.json` against
`catalog.schema.json`. Use stable kebab-case IDs because process, mode, input,
and parameter IDs become route, recipe, form-state, and test references.

- Explain the audible or file-level outcome before CDP terminology.
- Include concrete use cases and search tags.
- Describe every input's accepted CDP file types, quantity, order, channel
  limits, and compatibility requirements.
- Represent optional flags and values explicitly. Never place user-authored
  command fragments in `argumentOrder`.
- Set documented, conservative defaults and declarative cross-field constraints.
- Declare the real output shape: single file, generic multi-output root, stdout
  report, or no output.
- Record local documentation provenance, help arguments, and verified version.

Register the JSON in both catalog entry points:

1. Import and append it in `src/processes/catalog.ts` for the Vue application.
2. Add an `include_str!` entry in `src-tauri/src/catalog/mod.rs` and
   `src-tauri/tests/catalog_contract.rs` for the Rust runtime and contract tests.

This duplication is intentional until catalog generation has a reviewed build
step. A process is not shipped unless both sides load the same manifest.

## 3. Add new contract vocabulary only when required

Prefer the existing categories, file types, parameter kinds, constraints, and
output rules. If the process cannot be described without a contract change:

1. Update `catalog.types.ts`, `catalog.schema.json`, and the matching Rust types.
2. Implement the same validation and serialization behavior in TypeScript and
   Rust.
3. Add a fixture proving both sides accept the new shape and reject invalid
   variants.
4. Update [the catalog contract](02-process-catalog-contract.md) before using the
   new vocabulary in a manifest.

Do not add an expression language, arbitrary regular expressions, shell text,
or executable paths to the catalog.

## 4. Package a new executable

If the process uses an existing `BinaryId`, no new executable registration is
needed. For a new executable:

1. Add the ID to the TypeScript `BinaryId`, JSON Schema enum, and Rust
   `BinaryId`; keep their serialized spelling identical.
2. Add its unsuffixed logical path to `bundle.externalBin` in
   `src-tauri/tauri.conf.json`.
3. Add it to the staging allowlist and produce one correctly named binary for
   every supported target described in
   [the cross-platform build guide](05-cross-platform-builds.md).
4. Verify executable permissions on Unix, the `.exe` suffix on Windows, and
   that the runtime resolves only the packaged allowlisted binary.
5. Include the executable's license and source-revision record in the release
   materials.

Never silently fall back to an executable from `PATH` or a user-selected path.

## 5. Test before making it visible

Add all of the following in the same change:

- Catalog tests for search, defaults, mode coverage, argument order, output
  rules, valid boundaries, and representative invalid requests.
- Rust tests for exact `OsString` argument vectors, missing/extra fields,
  unsafe paths, constraints, output collision behavior, and discovery rules.
- A disposable smoke fixture that invokes the actual staged binary, verifies
  outputs with `sfprops`, and never writes into `cdpr8` or the repository.
- Manual generated-form checks for every mode, including keyboard operation,
  errors, advanced controls, mode changes, preview, cancellation, and results.

Run:

```sh
bun run typecheck
bun test
cargo test --manifest-path src-tauri/Cargo.toml
bun run fmt:check
bun run lint
```

Only add the definition to the visible frontend and Rust catalog lists after
these checks pass. Then review `src/recipes/` for new compatible handoffs and
run the recipe tests; adding a process does not require adding a recipe.

## Definition of done

- The process completes end to end for every visible mode.
- Vue and Rust agree on all IDs, values, constraints, and output types.
- Command preview and execution use the same Rust-owned compiler.
- Missing binaries and invalid material produce actionable errors.
- Every advertised platform packages and smoke-tests the required executable.
- Documentation evidence and the verified CDP revision are recorded.
