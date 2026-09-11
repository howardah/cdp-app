use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessDefinition {
    pub schema_version: u8,
    pub id: String,
    pub title: String,
    pub category: String,
    pub summary: String,
    pub description: String,
    pub use_cases: Vec<String>,
    pub tags: Vec<String>,
    pub identity: Identity,
    pub documentation: Documentation,
    pub modes: Vec<ModeDefinition>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Identity { pub executable: String, pub operation: Option<String> }
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Documentation { pub local_path: String, pub anchor: Option<String>, pub help_command: Vec<String>, pub verified_with_version: String }
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition { pub id: String, pub cli_mode: Option<u8>, pub title: String, pub summary: String, pub inputs: Vec<InputDefinition>, pub parameters: Vec<serde_json::Value>, pub output: serde_json::Value, pub argument_order: Vec<serde_json::Value>, pub constraints: Vec<serde_json::Value> }
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputDefinition { pub id: String, pub label: String, pub description: String, pub file_types: Vec<String>, pub min_items: usize, pub max_items: Option<usize>, pub ordered: bool, pub constraints: Vec<serde_json::Value> }
