//! Catalog boundary shared by the runtime commands.
//! Manifests are data, never executable command fragments.
pub mod types;

use std::collections::HashSet;
use types::ProcessDefinition;

/// Embed the reviewed manifests so catalog behavior is independent of cwd or
/// user-controlled files.
pub fn bundled_catalog() -> Result<Vec<ProcessDefinition>, String> {
    load_catalog(&[
        include_str!("../../../src/processes/definitions/modify-speed.json"),
        include_str!("../../../src/processes/definitions/modify-loudness.json"),
        include_str!("../../../src/processes/definitions/sfedit-join.json"),
        include_str!("../../../src/processes/definitions/pvoc-analyze.json"),
        include_str!("../../../src/processes/definitions/pvoc-synthesize.json"),
        include_str!("../../../src/processes/definitions/isolate.json"),
    ])
}

pub fn load_catalog(json_documents: &[&str]) -> Result<Vec<ProcessDefinition>, String> {
    let mut ids = HashSet::new();
    let mut catalog = Vec::with_capacity(json_documents.len());
    for document in json_documents {
        let definition: ProcessDefinition = serde_json::from_str(document)
            .map_err(|error| format!("invalid process manifest: {error}"))?;
        if definition.schema_version != 1 {
            return Err(format!("unsupported catalog schema for {}", definition.id));
        }
        if definition.id.trim().is_empty() {
            return Err("process id cannot be empty".into());
        }
        if !ids.insert(definition.id.clone()) {
            return Err(format!("duplicate process id: {}", definition.id));
        }
        if definition.modes.is_empty() {
            return Err(format!("process {} has no modes", definition.id));
        }
        if definition
            .modes
            .iter()
            .any(|mode| mode.id.trim().is_empty())
        {
            return Err(format!("process {} has an empty mode id", definition.id));
        }
        validate_definition(&definition)?;
        catalog.push(definition);
    }
    Ok(catalog)
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .split('-')
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric()))
        && id.chars().next().is_some_and(|c| c.is_ascii_lowercase())
}

fn validate_definition(definition: &ProcessDefinition) -> Result<(), String> {
    let mut modes = HashSet::new();
    for mode in &definition.modes {
        if !valid_id(&mode.id) || !modes.insert(&mode.id) {
            return Err(format!(
                "invalid or duplicate mode id in {}: {}",
                definition.id, mode.id
            ));
        }
        let mut inputs = HashSet::new();
        for input in &mode.inputs {
            if !valid_id(&input.id) || !inputs.insert(&input.id) {
                return Err(format!(
                    "invalid or duplicate input id in {}: {}",
                    mode.id, input.id
                ));
            }
            if input.max_items.is_some_and(|max| max < input.min_items) {
                return Err(format!("input {} has maxItems below minItems", input.id));
            }
            for constraint in &input.constraints {
                if let types::FileConstraint::Channels { min, max } = constraint {
                    if min > max {
                        return Err(format!("invalid channel range for {}", input.id));
                    }
                }
            }
        }
        let mut params = HashSet::new();
        for parameter in &mode.parameters {
            let base = parameter_base(parameter);
            if !valid_id(&base.id) || !params.insert(&base.id) {
                return Err(format!(
                    "invalid or duplicate parameter id in {}: {}",
                    mode.id, base.id
                ));
            }
            validate_binding(&base.cli)?;
            let d = &base.details;
            if let (Some(min), Some(max)) = (
                d.get("min").and_then(|v| v.as_f64()),
                d.get("max").and_then(|v| v.as_f64()),
            ) {
                if !min.is_finite() || !max.is_finite() || min > max {
                    return Err(format!("invalid numeric bounds for {}", base.id));
                }
            }
            if let Some(default) = d.get("default") {
                validate_default(parameter, default)?;
            }
        }
        for token in &mode.argument_order {
            match token {
                types::ArgumentToken::Input { input_id } if !inputs.contains(input_id) => {
                    return Err(format!("unknown input reference: {input_id}"))
                }
                types::ArgumentToken::Parameter { parameter_id }
                    if !params.contains(parameter_id) =>
                {
                    return Err(format!("unknown parameter reference: {parameter_id}"))
                }
                types::ArgumentToken::Literal { value } if value.contains('\0') => {
                    return Err("literal contains NUL".into())
                }
                _ => {}
            }
        }
        for constraint in &mode.constraints {
            for reference in constraint_refs(constraint) {
                match reference {
                    types::ValueRef::Parameter { id } if !params.contains(id) => {
                        return Err(format!("unknown constraint parameter: {id}"))
                    }
                    types::ValueRef::InputMetadata { input_id, .. }
                        if !inputs.contains(input_id) =>
                    {
                        return Err(format!("unknown constraint input: {input_id}"))
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn validate_binding(binding: &types::CliBinding) -> Result<(), String> {
    let flag = match binding {
        types::CliBinding::Positional => return Ok(()),
        types::CliBinding::Option { flag, .. } | types::CliBinding::BooleanFlag { flag } => flag,
    };
    if !flag.starts_with('-')
        || flag.len() < 2
        || !flag[1..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(format!("unsafe CLI flag: {flag}"));
    }
    Ok(())
}

fn validate_default(
    parameter: &types::ParameterDefinition,
    value: &serde_json::Value,
) -> Result<(), String> {
    let base = parameter_base(parameter);
    let numeric = |v: &serde_json::Value| v.as_f64().filter(|n| n.is_finite());
    let number = match parameter {
        types::ParameterDefinition::Number(_) | types::ParameterDefinition::Integer(_) => {
            numeric(value)
        }
        types::ParameterDefinition::NumberOrBreakpoint(_) => value.get("value").and_then(numeric),
        _ => None,
    };
    if matches!(
        parameter,
        types::ParameterDefinition::Number(_)
            | types::ParameterDefinition::Integer(_)
            | types::ParameterDefinition::NumberOrBreakpoint(_)
    ) && number.is_none()
    {
        return Err(format!("impossible numeric default for {}", base.id));
    }
    if let Some(n) = number {
        if let Some(min) = base.details.get("min").and_then(|v| v.as_f64()) {
            if n < min {
                return Err(format!("default below minimum for {}", base.id));
            }
        }
        if let Some(max) = base.details.get("max").and_then(|v| v.as_f64()) {
            if n > max {
                return Err(format!("default above maximum for {}", base.id));
            }
        }
        if matches!(parameter, types::ParameterDefinition::Integer(_)) && n.fract() != 0.0 {
            return Err(format!("integer default is fractional for {}", base.id));
        }
    }
    Ok(())
}

fn constraint_refs(constraint: &types::Constraint) -> Vec<&types::ValueRef> {
    match constraint {
        types::Constraint::LessThan { left, right, .. }
        | types::Constraint::LessThanOrEqual { left, right, .. }
        | types::Constraint::GreaterThan { left, right, .. } => vec![left, right],
        types::Constraint::PowerOfTwo { value, .. } => vec![value],
    }
}

fn parameter_base(parameter: &types::ParameterDefinition) -> &types::ParameterBase {
    match parameter {
        types::ParameterDefinition::Number(b)
        | types::ParameterDefinition::Integer(b)
        | types::ParameterDefinition::Choice(b)
        | types::ParameterDefinition::Flag(b)
        | types::ParameterDefinition::File(b)
        | types::ParameterDefinition::NumberOrBreakpoint(b) => b,
    }
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
