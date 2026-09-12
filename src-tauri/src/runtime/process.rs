use super::paths::discover_generic_outputs;
use super::registry::SharedRunRegistry;
use super::types::{CompiledCommand, OutputArtifact, RunSnapshot, RunStatus};
use crate::catalog::types::{CdpFileType, OutputDefinition};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

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
                    OutputDefinition::GenericRoot { extension, .. }
                    | OutputDefinition::AutoNamedGeneric { extension, .. } => {
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
            if let OutputDefinition::GenericRoot { extension, .. }
            | OutputDefinition::AutoNamedGeneric { extension, .. } = &self.output_definition
            {
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
                playable: file_type_for_path(path) == Some(CdpFileType::Soundfile),
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

pub(super) fn refresh_snapshot(mut snapshot: RunSnapshot) -> RunSnapshot {
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
        "env" | "evl" => Some(CdpFileType::BinaryEnvelope),
        "mix" => Some(CdpFileType::Mixfile),
        "dimg" | "domain" => Some(CdpFileType::DomainImage),
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
            let stop = registry.lock().map(|r| r.should_stop()).unwrap_or(true);
            if stop {
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }
    })
}
