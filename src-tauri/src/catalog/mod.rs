//! Catalog boundary shared by the runtime commands.
//! Manifests are data, never executable command fragments.
pub mod types;

use std::collections::HashSet;
use types::ProcessDefinition;

pub fn load_catalog(json_documents: &[&str]) -> Result<Vec<ProcessDefinition>, String> {
    let mut ids = HashSet::new();
    let mut catalog = Vec::with_capacity(json_documents.len());
    for document in json_documents {
        let definition: ProcessDefinition = serde_json::from_str(document)
            .map_err(|error| format!("invalid process manifest: {error}"))?;
        if definition.schema_version != 1 { return Err(format!("unsupported catalog schema for {}", definition.id)); }
        if !ids.insert(definition.id.clone()) { return Err(format!("duplicate process id: {}", definition.id)); }
        if definition.modes.is_empty() { return Err(format!("process {} has no modes", definition.id)); }
        catalog.push(definition);
    }
    Ok(catalog)
}

#[cfg(test)]
mod tests {
    use super::load_catalog;
    #[test]
    fn rejects_duplicate_process_ids() {
        let manifest = r#"{"schemaVersion":1,"id":"x","title":"X","category":"utilities","summary":"x","description":"x","useCases":[],"tags":[],"identity":{"executable":"modify"},"documentation":{"localPath":"x","helpCommand":[],"verifiedWithVersion":"x"},"modes":[{"id":"m","title":"m","summary":"m","inputs":[],"parameters":[],"output":{"kind":"none"},"argumentOrder":[],"constraints":[]}] }"#;
        assert!(load_catalog(&[manifest, manifest]).is_err());
    }
}
