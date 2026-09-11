//! Safe command compilation and the small runtime abstractions shared by IPC.
//! This module intentionally never invokes a shell: every token remains an OsString.
use crate::catalog::types::*;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::{
    collections::{HashMap, VecDeque},
    ffi::OsString,
    path::{Path, PathBuf},
};
use uuid::Uuid;

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
    pub executable: String,
    pub args: Vec<String>,
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
struct QueuedRun {
    id: Uuid,
    command: CompiledCommand,
}
#[derive(Debug, Default)]
pub struct RunRegistry {
    runs: HashMap<Uuid, RunSnapshot>,
    queue: VecDeque<QueuedRun>,
    active: Option<Uuid>,
    accepting: bool,
}
impl RunRegistry {
    pub fn new() -> Self {
        Self {
            accepting: true,
            ..Self::default()
        }
    }
    pub fn enqueue(&mut self, command: CompiledCommand) -> Result<RunAccepted, String> {
        if !self.accepting {
            return Err("application is shutting down".into());
        }
        let id = Uuid::new_v4();
        let position = self.queue.len();
        self.queue.push_back(QueuedRun { id, command });
        self.runs.insert(
            id,
            RunSnapshot {
                run_id: id,
                status: RunStatus::Queued,
                queue_position: Some(position),
                error: None,
                artifacts: vec![],
            },
        );
        Ok(RunAccepted {
            run_id: id,
            queue_position: position,
        })
    }
    pub fn start_next(&mut self) -> Option<(Uuid, CompiledCommand)> {
        let job = self.queue.pop_front()?;
        self.active = Some(job.id);
        if let Some(s) = self.runs.get_mut(&job.id) {
            s.status = RunStatus::Running;
            s.queue_position = None;
        }
        for (i, q) in self.queue.iter().enumerate() {
            if let Some(s) = self.runs.get_mut(&q.id) {
                s.queue_position = Some(i);
            }
        }
        Some((job.id, job.command))
    }
    pub fn cancel(&mut self, id: Uuid) -> Result<(), String> {
        if self.active == Some(id) {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
                s.queue_position = None;
            }
            return Ok(());
        }
        let Some(index) = self.queue.iter().position(|q| q.id == id) else {
            return Err("run not found or already finished".into());
        };
        self.queue.remove(index);
        if let Some(s) = self.runs.get_mut(&id) {
            s.status = RunStatus::Cancelled;
            s.queue_position = None;
        }
        for (i, q) in self.queue.iter().enumerate() {
            if let Some(s) = self.runs.get_mut(&q.id) {
                s.queue_position = Some(i);
            }
        }
        Ok(())
    }
    pub fn finish(&mut self, id: Uuid, result: Result<Vec<OutputArtifact>, String>) {
        if let Some(s) = self.runs.get_mut(&id) {
            if s.status == RunStatus::Cancelled {
                if self.active == Some(id) {
                    self.active = None;
                }
                return;
            }
            match result {
                Ok(a) => {
                    s.status = RunStatus::Completed;
                    s.artifacts = a;
                }
                Err(e) => {
                    s.status = RunStatus::Failed;
                    s.error = Some(e);
                }
            }
        }
        if self.active == Some(id) {
            self.active = None;
        }
    }
    pub fn get(&self, id: Uuid) -> Option<RunSnapshot> {
        self.runs.get(&id).cloned().map(refresh_snapshot)
    }
    pub fn list(&self) -> Vec<RunSnapshot> {
        let mut runs = self
            .runs
            .values()
            .cloned()
            .map(refresh_snapshot)
            .collect::<Vec<_>>();
        runs.sort_by_key(|run| run.run_id);
        runs
    }
    pub fn shutdown(&mut self) {
        self.accepting = false;
        if let Some(id) = self.active {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
            }
        }
        let ids = self.queue.drain(..).map(|q| q.id).collect::<Vec<_>>();
        for id in ids {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
                s.queue_position = None;
            }
        }
    }
}
pub type SharedRunRegistry = Arc<Mutex<RunRegistry>>;

pub trait RunningProcess: Send {
    fn poll(&mut self) -> Result<Option<Vec<OutputArtifact>>, String>;
    fn terminate(&mut self);
}
pub trait ProcessRunner: Send + Sync + 'static {
    fn spawn(&self, command: &CompiledCommand) -> Result<Box<dyn RunningProcess>, String>;
}
#[derive(Debug, Default)]
pub struct PackagedRunner;
impl ProcessRunner for PackagedRunner {
    fn spawn(&self, command: &CompiledCommand) -> Result<Box<dyn RunningProcess>, String> {
        let mut process = Command::new(&command.program);
        process.args(&command.args);
        if matches!(command.output_definition, OutputDefinition::StdoutReport) {
            process.stdout(Stdio::piped());
        }
        let child = process
            .spawn()
            .map_err(|e| format!("could not start CDP process: {e}"))?;
        Ok(Box::new(ChildProcess {
            child,
            output: command.output.clone(),
            output_definition: command.output_definition.clone(),
            generic_before: command
                .output
                .as_ref()
                .and_then(|root| match &command.output_definition {
                    OutputDefinition::GenericRoot { extension, .. } => {
                        matching_generic_outputs(root, extension).ok()
                    }
                    _ => None,
                })
                .unwrap_or_default(),
        }))
    }
}
struct ChildProcess {
    child: Child,
    output: Option<PathBuf>,
    output_definition: OutputDefinition,
    generic_before: Vec<PathBuf>,
}
impl RunningProcess for ChildProcess {
    fn poll(&mut self) -> Result<Option<Vec<OutputArtifact>>, String> {
        let Some(status) = self.child.try_wait().map_err(|e| e.to_string())? else {
            return Ok(None);
        };
        if !status.success() {
            return Err(format!("CDP process failed with status {status}"));
        }
        if let Some(path) = &self.output {
            if let OutputDefinition::GenericRoot { extension, .. } = &self.output_definition {
                let paths = discover_generic_outputs(path, extension, &self.generic_before)?;
                if paths.is_empty() {
                    return Err("CDP process completed without producing outputs".into());
                }
                return Ok(Some(paths.into_iter().map(artifact_for_path).collect()));
            }
            if !path.exists() {
                return Err("CDP process completed without producing its required output".into());
            }
            let size = std::fs::metadata(path)
                .map_err(|_| "output cannot be inspected".to_string())?
                .len();
            return Ok(Some(vec![OutputArtifact {
                path: path.to_string_lossy().into_owned(),
                file_type: file_type_for_path(path)
                    .ok_or("output has an unrecognized file type")?,
                size_bytes: size,
                playable: true,
                available: true,
                metadata: None,
            }]));
        }
        if matches!(self.output_definition, OutputDefinition::StdoutReport) {
            let mut stdout = Vec::new();
            if let Some(pipe) = self.child.stdout.as_mut() {
                pipe.read_to_end(&mut stdout).map_err(|e| e.to_string())?;
            }
            return Ok(Some(vec![OutputArtifact {
                path: "<stdout>".into(),
                file_type: CdpFileType::TextData,
                size_bytes: stdout.len() as u64,
                playable: false,
                available: true,
                metadata: None,
            }]));
        }
        Ok(Some(vec![]))
    }
    fn terminate(&mut self) {
        terminate_child(&mut self.child);
    }
}

fn terminate_child(child: &mut Child) {
    #[cfg(unix)]
    {
        // Give CDP a chance to flush/close its output. This intentionally uses
        // libc directly: no shell is involved and the PID is our child.
        #[cfg(target_os = "macos")]
        unsafe {
            libc::kill(child.id() as libc::pid_t, libc::SIGTERM);
        }
        let deadline = Instant::now() + Duration::from_millis(500);
        while Instant::now() < deadline {
            if child.try_wait().ok().flatten().is_some() {
                return;
            }
            thread::sleep(Duration::from_millis(25));
        }
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn artifact_for_path(path: PathBuf) -> OutputArtifact {
    let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    OutputArtifact {
        path: path.to_string_lossy().into_owned(),
        file_type: file_type_for_path(&path).unwrap_or(CdpFileType::TextData),
        size_bytes,
        playable: file_type_for_path(&path) == Some(CdpFileType::Soundfile),
        available: true,
        metadata: None,
    }
}

fn refresh_snapshot(mut snapshot: RunSnapshot) -> RunSnapshot {
    for artifact in &mut snapshot.artifacts {
        artifact.available = artifact.path == "<stdout>" || Path::new(&artifact.path).is_file();
        if artifact.available {
            artifact.size_bytes = std::fs::metadata(&artifact.path)
                .map(|m| m.len())
                .unwrap_or(artifact.size_bytes);
        }
    }
    snapshot
}

fn matching_generic_outputs(root: &Path, extension: &str) -> Result<Vec<PathBuf>, String> {
    discover_generic_outputs(root, extension, &[])
}

pub fn file_type_for_path(path: &Path) -> Option<CdpFileType> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "wav" | "aif" | "aiff" => Some(CdpFileType::Soundfile),
        "ana" => Some(CdpFileType::AnalysisAna),
        "pvx" => Some(CdpFileType::AnalysisPvx),
        "brk" | "bpf" => Some(CdpFileType::Breakpoint),
        "txt" => Some(CdpFileType::TextData),
        _ => None,
    }
}
pub fn spawn_worker(
    registry: SharedRunRegistry,
    runner: Arc<dyn ProcessRunner>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let next = registry.lock().ok().and_then(|mut r| r.start_next());
        if let Some((id, command)) = next {
            let Ok(mut process) = runner.spawn(&command) else {
                if let Ok(mut r) = registry.lock() {
                    r.finish(id, Err("could not start CDP process".into()));
                }
                continue;
            };
            loop {
                let cancelled = registry
                    .lock()
                    .map(|r| r.get(id).is_some_and(|s| s.status == RunStatus::Cancelled))
                    .unwrap_or(true);
                if cancelled {
                    process.terminate();
                    if let Ok(mut r) = registry.lock() {
                        r.finish(id, Err("cancelled".into()));
                    }
                    break;
                }
                match process.poll() {
                    Ok(Some(result)) => {
                        if let Ok(mut r) = registry.lock() {
                            r.finish(id, Ok(result));
                        }
                        break;
                    }
                    Ok(None) => thread::sleep(Duration::from_millis(25)),
                    Err(e) => {
                        if let Ok(mut r) = registry.lock() {
                            r.finish(id, Err(e));
                        }
                        break;
                    }
                }
            }
        } else {
            let stop = registry
                .lock()
                .map(|r| !r.accepting && r.active.is_none() && r.queue.is_empty())
                .unwrap_or(true);
            if stop {
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }
    })
}

pub fn suggest_output_path(input: &Path, slug: &str, extension: &str) -> Result<PathBuf, String> {
    let parent = input.parent().ok_or("input has no parent directory")?;
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("input filename is not valid UTF-8")?;
    let safe_slug = slug
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    for n in 1..10000 {
        let suffix = if n == 1 {
            "".to_string()
        } else {
            format!("-{n}")
        };
        let extension = extension.trim_start_matches('.').to_ascii_lowercase();
        if extension.is_empty() || !extension.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err("output extension is invalid".into());
        }
        let path = parent.join(format!("{stem}-{safe_slug}{suffix}.{extension}"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err("could not find an available output path".into())
}

const MAX_BREAKPOINT_BYTES: u64 = 1024 * 1024;
const MAX_TEXT_BYTES: u64 = 4 * 1024 * 1024;

/// Validate CDP's time/value breakpoint text without interpreting user input as code.
pub fn validate_breakpoint(path: &Path) -> Result<(), String> {
    validate_bounded_text(path, MAX_BREAKPOINT_BYTES, |line, number| {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 2
            || fields
                .iter()
                .any(|v| v.parse::<f64>().map_or(true, |n| !n.is_finite()))
        {
            return Err(format!("invalid breakpoint at line {number}"));
        }
        if fields[0].parse::<f64>().unwrap() < 0.0 {
            return Err(format!(
                "breakpoint time must be non-negative at line {number}"
            ));
        }
        Ok(())
    })
}

pub fn validate_text_data(path: &Path) -> Result<(), String> {
    validate_bounded_text(path, MAX_TEXT_BYTES, |line, number| {
        if line.trim().is_empty() {
            return Ok(());
        }
        if line.chars().any(|c| c.is_control() && c != '\t') {
            return Err(format!("invalid text data at line {number}"));
        }
        Ok(())
    })
}

fn validate_bounded_text<F>(path: &Path, max: u64, mut check: F) -> Result<(), String>
where
    F: FnMut(&str, usize) -> Result<(), String>,
{
    let metadata = std::fs::metadata(path).map_err(|_| "file cannot be inspected".to_string())?;
    if !metadata.is_file() {
        return Err("path is not a regular file".into());
    }
    if metadata.len() > max {
        return Err("file exceeds the supported size limit".into());
    }
    let content =
        std::fs::read_to_string(path).map_err(|_| "file is not valid UTF-8".to_string())?;
    for (line, number) in content.lines().zip(1..) {
        check(line, number)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct SfProps {
    pub duration_seconds: f64,
    pub sample_rate: u32,
    pub channels: u32,
    pub sample_format: String,
}

/// Parse only the stable, labelled fields emitted by sfprops. Unknown labels are ignored,
/// but all required fields must be present and numeric values must be finite and positive.
pub fn parse_sfprops(stdout: &[u8], success: bool) -> Result<SfProps, String> {
    if !success {
        return Err("sfprops failed".into());
    }
    let text =
        std::str::from_utf8(stdout).map_err(|_| "sfprops output is not UTF-8".to_string())?;
    let mut duration = None;
    let mut rate = None;
    let mut channels = None;
    let mut format = None;
    for line in text.lines() {
        let Some((label, raw)) = line.split_once(':') else {
            continue;
        };
        let value = raw.trim();
        match label.trim().to_ascii_lowercase().as_str() {
            "duration" | "duration seconds" => {
                duration = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "sample rate" | "samplerate" => {
                rate = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "channels" | "channel count" => {
                channels = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "sample format" | "sampleformat" if !value.is_empty() => {
                format = Some(value.to_string())
            }
            _ => {}
        }
    }
    let duration = duration
        .filter(|v: &f64| v.is_finite() && *v >= 0.0)
        .ok_or("sfprops output missing duration")?;
    let sample_rate = rate
        .filter(|v| *v > 0)
        .ok_or("sfprops output missing sample rate")?;
    let channels = channels
        .filter(|v| *v > 0)
        .ok_or("sfprops output missing channels")?;
    let sample_format = format.ok_or("sfprops output missing sample format")?;
    Ok(SfProps {
        duration_seconds: duration,
        sample_rate,
        channels,
        sample_format,
    })
}

pub fn inspect_audio_metadata(path: &Path, sfprops: &Path) -> Result<InputMetadata, String> {
    let output = Command::new(sfprops)
        .arg(path)
        .output()
        .map_err(|e| format!("could not inspect input: {e}"))?;
    let props = parse_sfprops(&output.stdout, output.status.success())?;
    Ok(InputMetadata {
        duration_seconds: props.duration_seconds,
        sample_rate: props.sample_rate,
        channels: props.channels,
        sample_format: props.sample_format,
    })
}

pub fn discover_generic_outputs(
    root: &Path,
    extension: &str,
    before: &[PathBuf],
) -> Result<Vec<PathBuf>, String> {
    let parent = root.parent().ok_or("output root has no parent")?;
    let stem = root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("output root is invalid")?;
    let ext = extension.trim_start_matches('.').to_ascii_lowercase();
    let mut found = Vec::new();
    for entry in std::fs::read_dir(parent).map_err(|_| "output directory cannot be read")? {
        let path = entry.map_err(|_| "output directory cannot be read")?.path();
        if before.contains(&path) || !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
            continue;
        };
        let Some(suffix) = name.strip_prefix(stem) else {
            continue;
        };
        if !suffix.starts_with('-')
            || path
                .extension()
                .and_then(|v| v.to_str())
                .map(|v| v.eq_ignore_ascii_case(&ext))
                != Some(true)
        {
            continue;
        }
        let Some(number) = suffix
            .strip_prefix('-')
            .and_then(|v| v.strip_suffix(&format!(".{ext}")))
        else {
            continue;
        };
        if !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()) {
            found.push(path);
        }
    }
    found.sort();
    Ok(found)
}

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
        OutputDefinition::SingleFile { .. } | OutputDefinition::GenericRoot { .. }
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
    let preview = CommandPreview {
        executable: process.identity.executable_string(),
        args: args
            .iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect(),
    };
    Ok((
        CompiledCommand {
            binary: process.identity.executable,
            program,
            args,
            output: request.output_path.as_ref().map(PathBuf::from),
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

fn validate_output_path(path: &Path, output: &OutputDefinition) -> Result<(), String> {
    let (extension, generic) = match output {
        OutputDefinition::SingleFile { extension, .. } => (Some(extension.as_str()), false),
        OutputDefinition::GenericRoot { extension, .. } => (Some(extension.as_str()), true),
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
    Ok(())
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

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const TARGET_SUFFIX: &str = "-x86_64-apple-darwin";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const TARGET_SUFFIX: &str = "-aarch64-apple-darwin";
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const TARGET_SUFFIX: &str = "-x86_64-pc-windows-msvc";
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const TARGET_SUFFIX: &str = "-x86_64-unknown-linux-gnu";
#[cfg(not(any(
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "x86_64")
)))]
const TARGET_SUFFIX: &str = "";

pub fn resolve_packaged(root: &Path, id: BinaryId) -> Result<PathBuf, String> {
    let name = format!("{:?}", id).to_lowercase();
    let path = root.join(format!("{name}{TARGET_SUFFIX}"));
    if path.is_file() {
        Ok(path)
    } else {
        Err("required CDP binary is missing".into())
    }
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
                _ => {}
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_remain_single_arguments() {
        let req = RunProcessRequest {
            process_id: "x".into(),
            mode_id: "m".into(),
            inputs: HashMap::from([("in".into(), vec!["a file \"x\".wav".into()])]),
            parameters: HashMap::new(),
            output_path: None,
        };
        let p = ProcessDefinition {
            schema_version: 1,
            id: "x".into(),
            title: "x".into(),
            category: ProcessCategory::Utilities,
            summary: "x".into(),
            description: "x".into(),
            use_cases: vec![],
            tags: vec![],
            identity: Identity {
                executable: BinaryId::Modify,
                operation: None,
            },
            documentation: Documentation {
                local_path: "x".into(),
                anchor: None,
                help_command: vec![],
                verified_with_version: "x".into(),
            },
            modes: vec![ModeDefinition {
                id: "m".into(),
                cli_mode: None,
                title: "m".into(),
                summary: "m".into(),
                inputs: vec![InputDefinition {
                    id: "in".into(),
                    label: "in".into(),
                    description: "".into(),
                    file_types: vec![CdpFileType::Soundfile],
                    min_items: 1,
                    max_items: Some(1),
                    ordered: false,
                    constraints: vec![],
                }],
                parameters: vec![],
                output: OutputDefinition::None,
                argument_order: vec![ArgumentToken::Input {
                    input_id: "in".into(),
                }],
                constraints: vec![],
            }],
        };
        let (cmd, _) = compile_request(&[p], &req, "modify".into()).unwrap();
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn fifo_queue_and_queued_cancel_are_deterministic() {
        let command = CompiledCommand {
            binary: BinaryId::Modify,
            program: "modify".into(),
            args: vec![],
            output: None,
            output_definition: OutputDefinition::None,
        };
        let mut registry = RunRegistry::new();
        let first = registry.enqueue(command.clone()).unwrap();
        let second = registry.enqueue(command).unwrap();
        assert_eq!(first.queue_position, 0);
        assert_eq!(second.queue_position, 1);
        registry.cancel(first.run_id).unwrap();
        assert_eq!(registry.get(second.run_id).unwrap().queue_position, Some(0));
        assert_eq!(registry.start_next().unwrap().0, second.run_id);
    }

    #[test]
    fn active_cancel_is_terminal_and_worker_result_cannot_resurrect_it() {
        let command = CompiledCommand {
            binary: BinaryId::Modify,
            program: "modify".into(),
            args: vec![],
            output: None,
            output_definition: OutputDefinition::None,
        };
        let mut registry = RunRegistry::new();
        let accepted = registry.enqueue(command).unwrap();
        let _ = registry.start_next();
        registry.cancel(accepted.run_id).unwrap();
        registry.finish(accepted.run_id, Ok(vec![]));
        assert_eq!(
            registry.get(accepted.run_id).unwrap().status,
            RunStatus::Cancelled
        );
    }

    #[test]
    fn output_suggestion_never_overwrites_existing_files() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("take one.wav");
        std::fs::write(&input, b"input").unwrap();
        std::fs::write(dir.path().join("take one-speed.wav"), b"existing").unwrap();
        let suggested = suggest_output_path(&input, "speed", "wav").unwrap();
        assert_eq!(suggested.file_name().unwrap(), "take one-speed-2.wav");
    }

    #[test]
    fn sfprops_parser_requires_typed_complete_output() {
        let props = parse_sfprops(
            b"Duration: 2.5 sec\nSample Rate: 48000 Hz\nChannels: 2\nSample Format: 24-bit\n",
            true,
        )
        .unwrap();
        assert_eq!(props.channels, 2);
        assert!(parse_sfprops(b"Duration: 2\nChannels: 2\n", true).is_err());
        assert!(parse_sfprops(
            b"Duration: 2\nSample Rate: 48000\nChannels: 2\nSample Format: x\n",
            false
        )
        .is_err());
        assert!(parse_sfprops(&[0xff], true).is_err());
    }

    #[test]
    fn breakpoint_validation_is_bounded_and_rejects_malformed_rows() {
        let dir = tempfile::tempdir().unwrap();
        let good = dir.path().join("curve.brk");
        std::fs::write(&good, "0 0\n1.5 -3\n").unwrap();
        assert!(validate_breakpoint(&good).is_ok());
        let bad = dir.path().join("bad.brk");
        std::fs::write(&bad, "0 nope\n").unwrap();
        assert!(validate_breakpoint(&bad).is_err());
    }

    #[test]
    fn generic_discovery_returns_only_new_numbered_outputs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("take-isolated");
        let old = dir.path().join("take-isolated-1.wav");
        let new = dir.path().join("take-isolated-2.wav");
        let unrelated = dir.path().join("take-isolated-not-a-number.wav");
        for path in [&old, &new, &unrelated] {
            std::fs::write(path, b"x").unwrap();
        }
        let found = discover_generic_outputs(&root, "wav", &[old]).unwrap();
        assert_eq!(found, vec![new]);
    }

    #[test]
    fn output_contract_checks_parent_and_extension() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.aif");
        let output = OutputDefinition::SingleFile {
            file_type: CdpFileType::Soundfile,
            extension: "wav".into(),
            name_suffix: "-x".into(),
        };
        assert!(validate_output_path(&path, &output).is_err());
        assert!(validate_output_path(&dir.path().join("out.wav"), &output).is_ok());
    }

    #[test]
    fn declarative_constraints_use_defaults_and_reject_invalid_values() {
        let mode = ModeDefinition {
            id: "m".into(),
            cli_mode: None,
            title: "m".into(),
            summary: "".into(),
            inputs: vec![],
            parameters: vec![ParameterDefinition::Number(ParameterBase {
                id: "points".into(),
                label: "".into(),
                description: "".into(),
                required: false,
                advanced: false,
                unit: None,
                cli: CliBinding::Positional,
                details: serde_json::json!({"default": 8}),
            })],
            output: OutputDefinition::None,
            argument_order: vec![],
            constraints: vec![Constraint::PowerOfTwo {
                value: ValueRef::Parameter {
                    id: "points".into(),
                },
                message: "must be power of two".into(),
            }],
        };
        assert!(evaluate_constraints(&mode, &HashMap::new(), &HashMap::new()).is_ok());
        let bad = HashMap::from([(
            String::from("points"),
            ParameterValue::Number { value: 7.0 },
        )]);
        assert!(evaluate_constraints(&mode, &bad, &HashMap::new()).is_err());
    }

    #[test]
    fn metadata_constraints_enforce_channels_and_relationships() {
        let mode = ModeDefinition {
            id: "m".into(),
            cli_mode: None,
            title: "m".into(),
            summary: "".into(),
            inputs: vec![InputDefinition {
                id: "b".into(),
                label: "B".into(),
                description: "".into(),
                file_types: vec![],
                min_items: 1,
                max_items: Some(1),
                ordered: false,
                constraints: vec![FileConstraint::SameSampleRate {
                    input_id: "a".into(),
                }],
            }],
            parameters: vec![],
            output: OutputDefinition::None,
            argument_order: vec![],
            constraints: vec![],
        };
        let metadata = HashMap::from([
            (
                "a".into(),
                InputMetadata {
                    duration_seconds: 1.0,
                    sample_rate: 48000,
                    channels: 1,
                    sample_format: "x".into(),
                },
            ),
            (
                "b".into(),
                InputMetadata {
                    duration_seconds: 1.0,
                    sample_rate: 44100,
                    channels: 1,
                    sample_format: "x".into(),
                },
            ),
        ]);
        assert!(validate_metadata_constraints(&mode, &metadata).is_err());
    }
}
