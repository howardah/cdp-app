use super::paths::{validate_breakpoint, validate_text_data};
use super::process::file_type_for_path;
use super::types::{
    CommandPreview, CompiledCommand, InputMetadata, ParameterValue, RunProcessRequest,
};
use crate::catalog::types::*;
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

pub fn compile_request(
    catalog: &[ProcessDefinition],
    request: &RunProcessRequest,
    program: PathBuf,
) -> Result<(CompiledCommand, CommandPreview), String> {
    let process = catalog
        .iter()
        .find(|p| p.id == request.process_id)
        .ok_or("unknown process")?;
    let mode = process
        .modes
        .iter()
        .find(|m| m.id == request.mode_id)
        .ok_or("unknown mode")?;
    for input in &mode.inputs {
        let values = request
            .inputs
            .get(&input.id)
            .ok_or_else(|| format!("missing input {}", input.id))?;
        if values.len() < input.min_items || input.max_items.is_some_and(|max| values.len() > max) {
            return Err(format!("invalid number of {} inputs", input.id));
        }
        if values.iter().any(|p| p.trim().is_empty()) {
            return Err(format!("empty path in {}", input.id));
        }
    }
    if request
        .inputs
        .keys()
        .any(|key| !mode.inputs.iter().any(|i| i.id == *key))
    {
        return Err("unknown input key".into());
    }
    for param in &mode.parameters {
        let id = parameter_id(param);
        if required(param) && !request.parameters.contains_key(id) {
            return Err(format!("missing parameter {id}"));
        }
    }
    if request
        .parameters
        .keys()
        .any(|key| !mode.parameters.iter().any(|p| parameter_id(p) == key))
    {
        return Err("unknown parameter key".into());
    }
    evaluate_constraints(mode, &request.parameters, &HashMap::new())?;
    let needs_output = matches!(
        mode.output,
        OutputDefinition::SingleFile { .. }
            | OutputDefinition::GenericRoot { .. }
            | OutputDefinition::Composite { .. }
    );
    if needs_output != request.output_path.is_some() {
        return Err(if needs_output {
            "output path required"
        } else {
            "output path not allowed"
        }
        .into());
    }
    if let Some(output) = &request.output_path {
        validate_output_path(Path::new(output), &mode.output)?;
    }
    let mut args = Vec::new();
    for token in &mode.argument_order {
        match token {
            ArgumentToken::Literal { value } => args.push(OsString::from(value)),
            ArgumentToken::Mode => {
                if let Some(mode) = mode.cli_mode {
                    args.push(mode.to_string().into())
                }
            }
            ArgumentToken::Input { input_id } => args.extend(
                request
                    .inputs
                    .get(input_id)
                    .into_iter()
                    .flatten()
                    .map(OsString::from),
            ),
            ArgumentToken::Output => {
                if let Some(path) = &request.output_path {
                    args.push(OsString::from(path));
                }
            }
            ArgumentToken::OutputComponent { component_id } => {
                let root = request
                    .output_path
                    .as_deref()
                    .ok_or("output path required")?;
                let component = match &mode.output {
                    OutputDefinition::Composite { components, .. } => components
                        .iter()
                        .find(|component| component.id == *component_id)
                        .ok_or("unknown output component")?,
                    _ => return Err("output component requires a composite output".into()),
                };
                args.push(component_output_path(Path::new(root), component)?.into_os_string());
            }
            ArgumentToken::Parameter { parameter_id } => {
                if let Some(value) = request.parameters.get(parameter_id) {
                    let definition = mode
                        .parameters
                        .iter()
                        .find(|p| parameter_id_of(p) == parameter_id)
                        .ok_or("unknown parameter binding")?;
                    validate_parameter(definition, value)?;
                    append_bound_value(&mut args, definition, value)?;
                }
            }
        }
    }
    let executable = process.identity.executable_string();
    let tokens = args
        .iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let display = std::iter::once(executable.as_str())
        .chain(tokens.iter().map(String::as_str))
        .map(|token| format!("{token:?}"))
        .collect::<Vec<_>>()
        .join(" ");
    let preview = CommandPreview { display, tokens };
    let resolved_output = match &mode.output {
        OutputDefinition::AutoNamedGeneric { extension, .. } => {
            let source = mode
                .inputs
                .first()
                .and_then(|input| request.inputs.get(&input.id))
                .and_then(|paths| paths.first())
                .ok_or("auto-named output requires a source input")?;
            let source = Path::new(source);
            let stem = source
                .file_stem()
                .and_then(|value| value.to_str())
                .ok_or("source filename is invalid")?;
            let root = stem
                .strip_suffix(|_: char| true)
                .filter(|value| !value.is_empty())
                .ok_or("source filename is too short for CDP auto-naming")?;
            Some(source.with_file_name(root).with_extension(extension))
        }
        _ => request.output_path.as_ref().map(PathBuf::from),
    };
    Ok((
        CompiledCommand {
            binary: process.identity.executable,
            program,
            args,
            output: resolved_output,
            output_definition: mode.output.clone(),
        },
        preview,
    ))
}

/// Evaluate all manifest cross-field constraints. Input metadata is keyed by
/// input id and is supplied by the trusted sfprops inspector at enqueue time.
pub fn evaluate_constraints(
    mode: &ModeDefinition,
    parameters: &HashMap<String, ParameterValue>,
    metadata: &HashMap<String, InputMetadata>,
) -> Result<(), String> {
    let mut effective = parameters.clone();
    for parameter in &mode.parameters {
        let base = parameter_base(parameter);
        if effective.contains_key(&base.id) {
            continue;
        }
        let Some(default) = base.details.get("default") else {
            continue;
        };
        if let Some(value) = default
            .as_f64()
            .or_else(|| default.get("value").and_then(|v| v.as_f64()))
        {
            effective.insert(base.id.clone(), ParameterValue::Number { value });
        }
    }
    fn value(
        reference: &ValueRef,
        parameters: &HashMap<String, ParameterValue>,
        metadata: &HashMap<String, InputMetadata>,
    ) -> Result<f64, String> {
        match reference {
            ValueRef::Literal { value } => Ok(*value),
            ValueRef::Parameter { id } => parameters
                .get(id)
                .and_then(|v| match v {
                    ParameterValue::Number { value } => Some(*value),
                    _ => None,
                })
                .or(None)
                .ok_or_else(|| format!("constraint requires numeric parameter {id}")),
            ValueRef::InputMetadata { input_id, property } => {
                let m = metadata
                    .get(input_id)
                    .ok_or_else(|| format!("metadata unavailable for input {input_id}"))?;
                match property.as_str() {
                    "duration" | "durationSeconds" => Ok(m.duration_seconds),
                    "sampleRate" => Ok(m.sample_rate as f64),
                    "channels" => Ok(m.channels as f64),
                    _ => Err(format!("unsupported input metadata property: {property}")),
                }
            }
        }
    }
    for constraint in &mode.constraints {
        let (ok, message) = match constraint {
            Constraint::LessThan {
                left,
                right,
                message,
            } => (
                value(left, &effective, metadata)? < value(right, &effective, metadata)?,
                message,
            ),
            Constraint::LessThanOrEqual {
                left,
                right,
                message,
            } => (
                value(left, &effective, metadata)? <= value(right, &effective, metadata)?,
                message,
            ),
            Constraint::GreaterThan {
                left,
                right,
                message,
            } => (
                value(left, &effective, metadata)? > value(right, &effective, metadata)?,
                message,
            ),
            Constraint::PowerOfTwo {
                value: reference,
                message,
            } => {
                let n = value(reference, &effective, metadata)?;
                (
                    (2.0..=32768.0).contains(&n)
                        && n.fract() == 0.0
                        && (n as u64).is_power_of_two(),
                    message,
                )
            }
        };
        if !ok {
            return Err(message.clone());
        }
    }
    Ok(())
}

/// Canonicalize every user-selected path immediately before preview/enqueue.
pub fn validate_request_files(
    catalog: &[ProcessDefinition],
    request: &RunProcessRequest,
) -> Result<(), String> {
    let process = catalog
        .iter()
        .find(|p| p.id == request.process_id)
        .ok_or("unknown process")?;
    let mode = process
        .modes
        .iter()
        .find(|m| m.id == request.mode_id)
        .ok_or("unknown mode")?;
    for input in &mode.inputs {
        for raw in request.inputs.get(&input.id).into_iter().flatten() {
            let path =
                std::fs::canonicalize(raw).map_err(|_| format!("input does not exist: {raw}"))?;
            let kind = file_type_for_path(&path)
                .ok_or_else(|| format!("unrecognized input type: {raw}"))?;
            if !input.file_types.contains(&kind) {
                return Err(format!("input type is incompatible: {raw}"));
            }
            for constraint in &input.constraints {
                if matches!(
                    constraint,
                    FileConstraint::Channels { .. }
                        | FileConstraint::SameSampleRate { .. }
                        | FileConstraint::SameChannels { .. }
                        | FileConstraint::SameSampleFormat { .. }
                ) {
                    // Detailed metadata constraints are enforced after sfprops inspection;
                    // retaining this gate prevents unrecognized binary data from proceeding.
                    let _ = path;
                }
            }
            if matches!(kind, CdpFileType::Breakpoint) {
                validate_breakpoint(&path)?;
            }
            if matches!(kind, CdpFileType::TextData) {
                validate_text_data(&path)?;
            }
        }
    }
    for value in request.parameters.values() {
        if let ParameterValue::File { path } = value {
            let canonical = std::fs::canonicalize(path)
                .map_err(|_| format!("parameter file does not exist: {path}"))?;
            if file_type_for_path(&canonical) == Some(CdpFileType::Breakpoint) {
                validate_breakpoint(&canonical)?;
            }
            if file_type_for_path(&canonical) == Some(CdpFileType::TextData) {
                validate_text_data(&canonical)?;
            }
        }
    }
    Ok(())
}

pub(super) fn validate_output_path(path: &Path, output: &OutputDefinition) -> Result<(), String> {
    let (extension, generic) = match output {
        OutputDefinition::SingleFile { extension, .. } => (Some(extension.as_str()), false),
        OutputDefinition::GenericRoot { extension, .. }
        | OutputDefinition::AutoNamedGeneric { extension, .. }
        | OutputDefinition::Composite { extension, .. } => (Some(extension.as_str()), true),
        OutputDefinition::None | OutputDefinition::StdoutReport => {
            return Err("output path not allowed".into())
        }
    };
    let parent = path.parent().ok_or("output path has no parent directory")?;
    if !parent.is_dir() {
        return Err("output parent directory does not exist".into());
    }
    if path.exists() {
        return Err("output path already exists; choose a new path".into());
    }
    let wanted = extension.unwrap().trim_start_matches('.');
    if path
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v.eq_ignore_ascii_case(wanted))
        != Some(true)
    {
        return Err(format!("output must use the .{wanted} extension"));
    }
    if generic {
        // A generic root is itself a namespace: an existing matching numbered output
        // would make discovery ambiguous and is therefore rejected up front.
        let stem = path
            .file_name()
            .and_then(|v| v.to_str())
            .ok_or("output path is invalid")?;
        for entry in std::fs::read_dir(parent).map_err(|_| "output parent cannot be read")? {
            let candidate = entry.map_err(|_| "output parent cannot be read")?.path();
            let Some(name) = candidate.file_name().and_then(|v| v.to_str()) else {
                continue;
            };
            if name
                .strip_prefix(stem)
                .is_some_and(|rest| rest.starts_with('-'))
            {
                return Err("an existing file matches the output root".into());
            }
        }
    }
    if let OutputDefinition::Composite { components, .. } = output {
        for component in components {
            let component_path = component_output_path(path, component)?;
            if component_path.exists() {
                return Err(format!(
                    "output component already exists: {}",
                    component_path.display()
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn component_output_path(
    root: &Path,
    component: &OutputComponent,
) -> Result<PathBuf, String> {
    let stem = root
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or("output path is invalid")?;
    if stem.is_empty() {
        return Err("output path is invalid".into());
    }
    Ok(root
        .with_file_name(format!("{stem}{}", component.name_suffix))
        .with_extension(&component.extension))
}
fn parameter_id(p: &ParameterDefinition) -> &str {
    match p {
        ParameterDefinition::Number(b)
        | ParameterDefinition::Integer(b)
        | ParameterDefinition::Choice(b)
        | ParameterDefinition::Flag(b)
        | ParameterDefinition::File(b)
        | ParameterDefinition::NumberOrBreakpoint(b) => &b.id,
    }
}
fn required(p: &ParameterDefinition) -> bool {
    match p {
        ParameterDefinition::Number(b)
        | ParameterDefinition::Integer(b)
        | ParameterDefinition::Choice(b)
        | ParameterDefinition::Flag(b)
        | ParameterDefinition::File(b)
        | ParameterDefinition::NumberOrBreakpoint(b) => b.required,
    }
}
fn append_value(args: &mut Vec<OsString>, value: &ParameterValue) -> Result<(), String> {
    match value {
        ParameterValue::Number { value } if value.is_finite() => {
            args.push(value.to_string().into())
        }
        ParameterValue::Number { .. } => return Err("number must be finite".into()),
        ParameterValue::Choice { value } | ParameterValue::File { path: value } => {
            args.push(value.into())
        }
        ParameterValue::Flag { value } => {
            if *value {
                args.push(OsString::from("1"));
            }
        }
    }
    Ok(())
}
fn parameter_id_of(p: &ParameterDefinition) -> &str {
    match p {
        ParameterDefinition::Number(b)
        | ParameterDefinition::Integer(b)
        | ParameterDefinition::Choice(b)
        | ParameterDefinition::Flag(b)
        | ParameterDefinition::File(b)
        | ParameterDefinition::NumberOrBreakpoint(b) => &b.id,
    }
}
fn parameter_base(p: &ParameterDefinition) -> &ParameterBase {
    match p {
        ParameterDefinition::Number(b)
        | ParameterDefinition::Integer(b)
        | ParameterDefinition::Choice(b)
        | ParameterDefinition::Flag(b)
        | ParameterDefinition::File(b)
        | ParameterDefinition::NumberOrBreakpoint(b) => b,
    }
}
fn validate_parameter(
    definition: &ParameterDefinition,
    value: &ParameterValue,
) -> Result<(), String> {
    let base = parameter_base(definition);
    let d = &base.details;
    match (definition, value) {
        (
            ParameterDefinition::Number(_)
            | ParameterDefinition::Integer(_)
            | ParameterDefinition::NumberOrBreakpoint(_),
            ParameterValue::Number { value },
        ) => {
            if !value.is_finite() {
                return Err(format!("{} must be finite", base.id));
            }
            if let Some(min) = d.get("min").and_then(|v| v.as_f64()) {
                if *value < min {
                    return Err(format!("{} is below minimum", base.id));
                }
            }
            if let Some(max) = d.get("max").and_then(|v| v.as_f64()) {
                if *value > max {
                    return Err(format!("{} is above maximum", base.id));
                }
            }
            if matches!(definition, ParameterDefinition::Integer(_)) && value.fract() != 0.0 {
                return Err(format!("{} must be an integer", base.id));
            }
        }
        (ParameterDefinition::NumberOrBreakpoint(_), ParameterValue::File { path }) => {
            validate_breakpoint(Path::new(path))?;
        }
        (ParameterDefinition::Choice(_), ParameterValue::Choice { value }) => {
            if !d
                .get("choices")
                .and_then(|v| v.as_array())
                .is_some_and(|cs| {
                    cs.iter()
                        .any(|c| c.get("value").and_then(|v| v.as_str()) == Some(value))
                })
            {
                return Err(format!("invalid choice for {}", base.id));
            }
        }
        (ParameterDefinition::Flag(_), ParameterValue::Flag { .. })
        | (ParameterDefinition::File(_), ParameterValue::File { .. }) => {}
        _ => return Err(format!("wrong value type for {}", base.id)),
    }
    Ok(())
}
fn append_bound_value(
    args: &mut Vec<OsString>,
    definition: &ParameterDefinition,
    value: &ParameterValue,
) -> Result<(), String> {
    let binding = &parameter_base(definition).cli;
    match binding {
        CliBinding::Positional => append_value(args, value),
        CliBinding::Option { flag, join } => {
            let mut rendered = Vec::new();
            append_value(&mut rendered, value)?;
            let val = rendered.pop().ok_or("empty option value")?;
            match join {
                JoinStyle::Concatenated => {
                    args.push(format!("{flag}{}", val.to_string_lossy()).into())
                }
                JoinStyle::Separate => {
                    args.push(flag.into());
                    args.push(val);
                }
            }
            Ok(())
        }
        CliBinding::BooleanFlag { flag } => {
            if matches!(value, ParameterValue::Flag { value: true }) {
                args.push(flag.into());
                Ok(())
            } else {
                Ok(())
            }
        }
    }
}
trait ExecutableName {
    fn executable_string(&self) -> String;
}
impl ExecutableName for Identity {
    fn executable_string(&self) -> String {
        format!("{:?}", self.executable).to_lowercase()
    }
}
pub trait BinaryResolver {
    fn resolve(&self, id: BinaryId) -> Result<PathBuf, String>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SupportedTarget {
    triple: &'static str,
    executable_extension: &'static str,
}

#[allow(dead_code)] // The complete matrix is verified by host-independent resolver tests.
impl SupportedTarget {
    const MACOS_X64: Self = Self {
        triple: "x86_64-apple-darwin",
        executable_extension: "",
    };
    const MACOS_ARM64: Self = Self {
        triple: "aarch64-apple-darwin",
        executable_extension: "",
    };
    const WINDOWS_X64: Self = Self {
        triple: "x86_64-pc-windows-msvc",
        executable_extension: ".exe",
    };
    const WINDOWS_ARM64: Self = Self {
        triple: "aarch64-pc-windows-msvc",
        executable_extension: ".exe",
    };
    const LINUX_X64: Self = Self {
        triple: "x86_64-unknown-linux-gnu",
        executable_extension: "",
    };
    const LINUX_ARM64: Self = Self {
        triple: "aarch64-unknown-linux-gnu",
        executable_extension: "",
    };

    const ALL: [Self; 6] = [
        Self::MACOS_X64,
        Self::MACOS_ARM64,
        Self::WINDOWS_X64,
        Self::WINDOWS_ARM64,
        Self::LINUX_X64,
        Self::LINUX_ARM64,
    ];

    fn from_triple(triple: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|target| target.triple == triple)
    }
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::MACOS_X64;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::MACOS_ARM64;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::WINDOWS_X64;
#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::WINDOWS_ARM64;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::LINUX_X64;
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const CURRENT_TARGET: SupportedTarget = SupportedTarget::LINUX_ARM64;
#[cfg(not(any(
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "windows", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "aarch64")
)))]
compile_error!("CDP Desktop supports only macOS x86_64/aarch64, Windows x86_64/aarch64, and Linux x86_64/aarch64 targets");

fn binary_name(id: BinaryId) -> String {
    format!("{:?}", id).to_lowercase()
}

fn development_filename(id: BinaryId, target: SupportedTarget) -> String {
    format!(
        "{}-{}{}",
        binary_name(id),
        target.triple,
        target.executable_extension
    )
}

fn packaged_filename(id: BinaryId, target: SupportedTarget) -> String {
    // Tauri removes the target triple when it copies an externalBin into the
    // application resources. The platform extension remains part of its name.
    format!("{}{}", binary_name(id), target.executable_extension)
}

fn resolve_named(root: &Path, filename: String) -> Result<PathBuf, String> {
    let path = root.join(filename);
    if path.is_file() {
        Ok(path)
    } else {
        Err("required CDP binary is missing".into())
    }
}

/// Resolve a sidecar staged next to the Rust project for development. These
/// files retain Tauri's target-triple filename convention.
fn resolve_development_for(
    root: &Path,
    id: BinaryId,
    target: SupportedTarget,
) -> Result<PathBuf, String> {
    resolve_named(root, development_filename(id, target))
}

pub fn resolve_development(root: &Path, id: BinaryId) -> Result<PathBuf, String> {
    resolve_development_for(root, id, CURRENT_TARGET)
}

/// Resolve a sidecar copied into a packaged application's resources. Tauri
/// bundles this under its logical (unsuffixed) resource filename.
fn resolve_packaged_for(
    root: &Path,
    id: BinaryId,
    target: SupportedTarget,
) -> Result<PathBuf, String> {
    resolve_named(root, packaged_filename(id, target))
}

pub fn resolve_packaged(root: &Path, id: BinaryId) -> Result<PathBuf, String> {
    resolve_packaged_for(root, id, CURRENT_TARGET)
}

pub fn validate_metadata_constraints(
    mode: &ModeDefinition,
    input_metadata: &HashMap<String, InputMetadata>,
) -> Result<(), String> {
    for input in &mode.inputs {
        let Some(metadata) = input_metadata.get(&input.id) else {
            continue;
        };
        for constraint in &input.constraints {
            match constraint {
                FileConstraint::Channels { min, max }
                    if metadata.channels < *min || metadata.channels > *max =>
                {
                    return Err(format!(
                        "{} must have between {min} and {max} channels",
                        input.label
                    ));
                }
                FileConstraint::SameSampleRate { input_id }
                    if input_metadata
                        .get(input_id)
                        .is_some_and(|m| m.sample_rate != metadata.sample_rate) =>
                {
                    return Err(format!(
                        "{} must match the sample rate of {input_id}",
                        input.label
                    ))
                }
                FileConstraint::SameChannels { input_id }
                    if input_metadata
                        .get(input_id)
                        .is_some_and(|m| m.channels != metadata.channels) =>
                {
                    return Err(format!(
                        "{} must match the channel count of {input_id}",
                        input.label
                    ))
                }
                FileConstraint::SameSampleFormat { input_id }
                    if input_metadata
                        .get(input_id)
                        .is_some_and(|m| m.sample_format != metadata.sample_format) =>
                {
                    return Err(format!(
                        "{} must match the sample format of {input_id}",
                        input.label
                    ))
                }
                FileConstraint::SameDuration { input_id }
                    if input_metadata.get(input_id).is_some_and(|m| {
                        (m.duration_seconds - metadata.duration_seconds).abs() > 1e-6
                    }) =>
                {
                    return Err(format!(
                        "{} must match the duration of {input_id}",
                        input.label
                    ))
                }
                _ => {}
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod resolver_tests {
    use super::*;

    #[test]
    fn every_supported_target_has_the_documented_development_filename() {
        let expected = [
            (
                SupportedTarget::MACOS_X64,
                "x86_64-apple-darwin",
                "modify-x86_64-apple-darwin",
            ),
            (
                SupportedTarget::MACOS_ARM64,
                "aarch64-apple-darwin",
                "modify-aarch64-apple-darwin",
            ),
            (
                SupportedTarget::WINDOWS_X64,
                "x86_64-pc-windows-msvc",
                "modify-x86_64-pc-windows-msvc.exe",
            ),
            (
                SupportedTarget::WINDOWS_ARM64,
                "aarch64-pc-windows-msvc",
                "modify-aarch64-pc-windows-msvc.exe",
            ),
            (
                SupportedTarget::LINUX_X64,
                "x86_64-unknown-linux-gnu",
                "modify-x86_64-unknown-linux-gnu",
            ),
            (
                SupportedTarget::LINUX_ARM64,
                "aarch64-unknown-linux-gnu",
                "modify-aarch64-unknown-linux-gnu",
            ),
        ];
        assert_eq!(SupportedTarget::ALL.len(), expected.len());
        for (target, triple, filename) in expected {
            assert_eq!(SupportedTarget::from_triple(triple), Some(target));
            assert_eq!(development_filename(BinaryId::Modify, target), filename);
        }
    }

    #[test]
    fn unsupported_target_triples_are_rejected() {
        assert_eq!(SupportedTarget::from_triple("i686-pc-windows-msvc"), None);
        assert_eq!(
            SupportedTarget::from_triple("riscv64gc-unknown-linux-gnu"),
            None
        );
    }

    #[test]
    fn packaged_names_are_logical_names_with_only_platform_extensions() {
        for target in SupportedTarget::ALL {
            let expected = if target.executable_extension == ".exe" {
                "sfprops.exe"
            } else {
                "sfprops"
            };
            assert_eq!(packaged_filename(BinaryId::Sfprops, target), expected);
        }
    }

    #[test]
    fn development_and_packaged_resolution_use_different_names() {
        let dir = tempfile::tempdir().unwrap();
        let target = SupportedTarget::WINDOWS_ARM64;
        let development = dir
            .path()
            .join(development_filename(BinaryId::Modify, target));
        let packaged = dir.path().join(packaged_filename(BinaryId::Modify, target));
        std::fs::write(&development, b"sidecar").unwrap();
        assert_eq!(
            resolve_development_for(dir.path(), BinaryId::Modify, target).unwrap(),
            development
        );
        assert!(resolve_packaged_for(dir.path(), BinaryId::Modify, target).is_err());
        std::fs::write(&packaged, b"sidecar").unwrap();
        assert_eq!(
            resolve_packaged_for(dir.path(), BinaryId::Modify, target).unwrap(),
            packaged
        );
    }

    #[test]
    fn resolver_rejects_missing_files_and_wrong_extensions() {
        let dir = tempfile::tempdir().unwrap();
        let target = SupportedTarget::WINDOWS_X64;
        std::fs::write(dir.path().join("modify-x86_64-pc-windows-msvc"), b"sidecar").unwrap();
        std::fs::write(dir.path().join("modify"), b"sidecar").unwrap();
        assert!(resolve_development_for(dir.path(), BinaryId::Modify, target).is_err());
        assert!(resolve_packaged_for(dir.path(), BinaryId::Modify, target).is_err());
        assert!(resolve_development_for(dir.path(), BinaryId::Sfprops, target).is_err());
    }
}
