#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ParameterValue {
    Number { value: f64 },
    Choice { value: String },
    Flag { value: bool },
    File { path: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunProcessRequest {
    pub process_id: String,
    pub mode_id: String,
    pub inputs: HashMap<String, Vec<String>>,
    pub parameters: HashMap<String, ParameterValue>,
    pub output_path: Option<String>,
}
#[derive(Debug, Clone)]
pub struct CompiledCommand {
    pub binary: BinaryId,
    pub program: PathBuf,
    pub args: Vec<OsString>,
    pub output: Option<PathBuf>,
    pub output_definition: OutputDefinition,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandPreview {
    pub display: String,
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunSnapshot {
    pub run_id: Uuid,
    pub status: RunStatus,
    pub queue_position: Option<usize>,
    pub error: Option<String>,
    pub artifacts: Vec<OutputArtifact>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputArtifact {
    pub path: String,
    pub file_type: CdpFileType,
    pub size_bytes: u64,
    pub playable: bool,
    /// False when the result was moved or deleted after the run completed.
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InspectedMetadata>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InspectedMetadata {
    pub duration_seconds: Option<f64>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub sample_format: Option<String>,
}

/// Metadata used by declarative manifest constraints. Kept separate from the
/// wire representation so tests and alternate inspectors can provide it.
#[derive(Debug, Clone, PartialEq)]
pub struct InputMetadata {
    pub duration_seconds: f64,
    pub sample_rate: u32,
    pub channels: u32,
    pub sample_format: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAccepted {
    pub run_id: Uuid,
    pub queue_position: usize,
}
#[derive(Debug, Clone)]
pub(super) struct QueuedRun {
    pub(super) id: Uuid,
    pub(super) command: CompiledCommand,
}
use crate::catalog::types::{BinaryId, CdpFileType, OutputDefinition};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::OsString, path::PathBuf};
use uuid::Uuid;
