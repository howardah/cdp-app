use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BinaryId {
    Modify,
    Sfedit,
    Pvoc,
    Isolate,
    Sfprops,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessCategory {
    TimeDomain,
    Spectral,
    EditAndMix,
    Utilities,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CdpFileType {
    Soundfile,
    AnalysisAna,
    AnalysisPvx,
    Breakpoint,
    CutsData,
    SliceData,
    TextData,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessDefinition {
    pub schema_version: u8,
    pub id: String,
    pub title: String,
    pub category: ProcessCategory,
    pub summary: String,
    pub description: String,
    pub use_cases: Vec<String>,
    pub tags: Vec<String>,
    pub identity: Identity,
    pub documentation: Documentation,
    pub modes: Vec<ModeDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub executable: BinaryId,
    pub operation: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    pub local_path: String,
    pub anchor: Option<String>,
    pub help_command: Vec<String>,
    pub verified_with_version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    pub cli_mode: Option<u8>,
    pub title: String,
    pub summary: String,
    pub inputs: Vec<InputDefinition>,
    pub parameters: Vec<ParameterDefinition>,
    pub output: OutputDefinition,
    pub argument_order: Vec<ArgumentToken>,
    pub constraints: Vec<Constraint>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputDefinition {
    pub id: String,
    pub label: String,
    pub description: String,
    pub file_types: Vec<CdpFileType>,
    pub min_items: usize,
    pub max_items: Option<usize>,
    pub ordered: bool,
    pub constraints: Vec<FileConstraint>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ParameterDefinition {
    Number(ParameterBase),
    Integer(ParameterBase),
    Choice(ParameterBase),
    Flag(ParameterBase),
    File(ParameterBase),
    NumberOrBreakpoint(ParameterBase),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterBase {
    pub id: String,
    pub label: String,
    pub description: String,
    pub required: bool,
    pub advanced: bool,
    pub unit: Option<String>,
    pub cli: CliBinding,
    #[serde(flatten, default)]
    pub details: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CliBinding {
    Positional,
    Option { flag: String, join: JoinStyle },
    BooleanFlag { flag: String },
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JoinStyle {
    Concatenated,
    Separate,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum OutputDefinition {
    SingleFile {
        file_type: CdpFileType,
        extension: String,
        name_suffix: String,
    },
    GenericRoot {
        file_type: CdpFileType,
        extension: String,
        name_suffix: String,
        discovery: OutputDiscoveryRule,
    },
    StdoutReport,
    None,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDiscoveryRule {
    pub prefix_style: String,
    pub may_produce_remnant: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ArgumentToken {
    Literal { value: String },
    Mode,
    Input { input_id: String },
    Output,
    Parameter { parameter_id: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum FileConstraint {
    Channels {
        min: u32,
        max: u32,
    },
    SameSampleRate {
        #[serde(rename = "inputId")]
        input_id: String,
    },
    SameChannels {
        #[serde(rename = "inputId")]
        input_id: String,
    },
    SameSampleFormat {
        #[serde(rename = "inputId")]
        input_id: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Constraint {
    LessThan {
        left: ValueRef,
        right: ValueRef,
        message: String,
    },
    LessThanOrEqual {
        left: ValueRef,
        right: ValueRef,
        message: String,
    },
    GreaterThan {
        left: ValueRef,
        right: ValueRef,
        message: String,
    },
    PowerOfTwo {
        value: ValueRef,
        message: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "source",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ValueRef {
    Parameter {
        id: String,
    },
    InputMetadata {
        #[serde(rename = "inputId")]
        input_id: String,
        property: String,
    },
    Literal {
        value: f64,
    },
}
