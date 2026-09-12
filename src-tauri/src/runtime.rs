//! Safe command compilation and runtime abstractions shared by IPC.
//!
//! Each responsibility lives in a conventional Rust submodule. The explicit
//! re-exports preserve the existing `runtime::...` API used by Tauri commands
//! and integration tests.

mod compiler;
mod paths;
mod process;
mod registry;
mod types;

pub use compiler::{
    compile_request, evaluate_constraints, resolve_development, resolve_packaged,
    validate_metadata_constraints, validate_request_files, BinaryResolver,
};
pub use paths::{
    discover_generic_outputs, inspect_audio_metadata, parse_sfprops, suggest_output_path,
    validate_breakpoint, validate_text_data, SfProps,
};
pub use process::{
    file_type_for_path, spawn_worker, PackagedRunner, ProcessRunner, RunningProcess,
};
pub use registry::{RunRegistry, SharedRunRegistry};
pub use types::{
    CommandPreview, CompiledCommand, InputMetadata, InspectedMetadata, OutputArtifact,
    ParameterValue, RunAccepted, RunProcessRequest, RunSnapshot, RunStatus,
};

#[cfg(test)]
mod tests;
