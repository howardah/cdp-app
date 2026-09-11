pub mod catalog;
pub mod runtime;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

#[tauri::command]
fn get_process_catalog(
    window: tauri::WebviewWindow,
) -> Result<Vec<catalog::types::ProcessDefinition>, String> {
    if window.label() != "main" && !window.label().starts_with("process-") {
        return Err("catalog is unavailable to this window".into());
    }
    catalog::bundled_catalog()
}

#[tauri::command]
fn preview_process(
    window: tauri::WebviewWindow,
    request: runtime::RunProcessRequest,
) -> Result<runtime::CommandPreview, String> {
    if !window.label().starts_with("process-") {
        return Err("process preview requires a process window".into());
    }
    let catalog = catalog::bundled_catalog()?;
    runtime::validate_request_files(&catalog, &request)?;
    let (_, preview) = runtime::compile_request(
        &catalog,
        &request,
        std::path::PathBuf::from("<packaged-cdp>"),
    )?;
    Ok(preview)
}

#[tauri::command]
fn open_process_window(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    process_id: String,
    mode_id: Option<String>,
) -> Result<String, String> {
    if window.label() != "main" {
        return Err("only the navigator can open process windows".into());
    }
    let catalog = catalog::bundled_catalog()?;
    let instance = Uuid::new_v4();
    let label = format!("process-{instance}");
    let url = process_window_url(&catalog, &process_id, mode_id.as_deref(), instance)?;
    tauri::WebviewWindowBuilder::new(&app, &label, tauri::WebviewUrl::App(url.into()))
        .title("CDP Process")
        .inner_size(760.0, 820.0)
        .min_inner_size(640.0, 560.0)
        .build()
        .map_err(|error| error.to_string())?;
    Ok(label)
}

fn process_window_url(
    catalog: &[catalog::types::ProcessDefinition],
    process_id: &str,
    mode_id: Option<&str>,
    instance: Uuid,
) -> Result<String, String> {
    let safe_id = |id: &str| {
        !id.is_empty()
            && id
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '-')
    };
    if !safe_id(process_id) {
        return Err("invalid process id".into());
    }
    let process = catalog
        .iter()
        .find(|process| process.id == process_id)
        .ok_or("unknown process")?;
    if let Some(mode_id) = mode_id {
        if !safe_id(mode_id) {
            return Err("invalid mode id".into());
        }
        if !process.modes.iter().any(|mode| mode.id == mode_id) {
            return Err("unknown mode for process".into());
        }
    }
    let mode_query = mode_id
        .map(|mode| format!("&mode={mode}"))
        .unwrap_or_default();
    Ok(format!(
        "/#/process/{process_id}?instance={instance}{mode_query}"
    ))
}

#[tauri::command]
fn list_runs(
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<Vec<runtime::RunSnapshot>, String> {
    state
        .lock()
        .map_err(|_| "run registry unavailable".into())
        .map(|runs| runs.list())
}
#[tauri::command]
fn get_run(
    run_id: Uuid,
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<Option<runtime::RunSnapshot>, String> {
    state
        .lock()
        .map_err(|_| "run registry unavailable".into())
        .map(|runs| runs.get(run_id))
}
#[tauri::command]
fn cancel_process(
    run_id: Uuid,
    window: tauri::WebviewWindow,
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<(), String> {
    if !window.label().starts_with("process-") && window.label() != "main" {
        return Err("run cancellation unavailable to this window".into());
    }
    state
        .lock()
        .map_err(|_| "run registry unavailable".to_string())?
        .cancel(run_id)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct InspectedFile {
    path: String,
    file_type: catalog::types::CdpFileType,
    size_bytes: u64,
    duration_seconds: Option<f64>,
    sample_rate: Option<u32>,
    channels: Option<u32>,
    sample_format: Option<String>,
}

#[tauri::command]
fn inspect_file(
    app: tauri::AppHandle,
    path: std::path::PathBuf,
    expected_types: Vec<catalog::types::CdpFileType>,
) -> Result<InspectedFile, String> {
    let canonical =
        std::fs::canonicalize(&path).map_err(|_| "input file does not exist".to_string())?;
    let metadata =
        std::fs::metadata(&canonical).map_err(|_| "input is not a readable file".to_string())?;
    if !metadata.is_file() {
        return Err("input path is not a regular file".into());
    }
    let extension = canonical
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let file_type = match extension.as_str() {
        "wav" | "aif" | "aiff" => catalog::types::CdpFileType::Soundfile,
        "ana" => catalog::types::CdpFileType::AnalysisAna,
        "pvx" => catalog::types::CdpFileType::AnalysisPvx,
        "brk" | "bpf" => catalog::types::CdpFileType::Breakpoint,
        "txt" => catalog::types::CdpFileType::TextData,
        _ => return Err("file type is not recognized; choose a supported CDP file".into()),
    };
    if !expected_types.is_empty() && !expected_types.contains(&file_type) {
        return Err("file type is incompatible with this input".into());
    }
    if matches!(file_type, catalog::types::CdpFileType::Breakpoint) {
        runtime::validate_breakpoint(&canonical)?;
    } else if matches!(file_type, catalog::types::CdpFileType::TextData) {
        runtime::validate_text_data(&canonical)?;
    }
    let props = if matches!(
        file_type,
        catalog::types::CdpFileType::Soundfile
            | catalog::types::CdpFileType::AnalysisAna
            | catalog::types::CdpFileType::AnalysisPvx
    ) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");
        let binary = resolve_binary(&app, catalog::types::BinaryId::Sfprops, &root)?;
        let output = std::process::Command::new(binary)
            .arg(&canonical)
            .output()
            .map_err(|e| format!("could not inspect file: {e}"))?;
        Some(runtime::parse_sfprops(
            &output.stdout,
            output.status.success(),
        )?)
    } else {
        None
    };
    Ok(InspectedFile {
        path: canonical.to_string_lossy().into_owned(),
        file_type,
        size_bytes: metadata.len(),
        duration_seconds: props.as_ref().map(|p| p.duration_seconds),
        sample_rate: props.as_ref().map(|p| p.sample_rate),
        channels: props.as_ref().map(|p| p.channels),
        sample_format: props.map(|p| p.sample_format),
    })
}

#[tauri::command]
fn suggest_output_path(
    process_id: String,
    mode_id: String,
    primary_input: std::path::PathBuf,
) -> Result<String, String> {
    let catalog = catalog::bundled_catalog()?;
    let process = catalog
        .iter()
        .find(|p| p.id == process_id)
        .ok_or("unknown process")?;
    let mode = process
        .modes
        .iter()
        .find(|m| m.id == mode_id)
        .ok_or("unknown mode")?;
    let (extension, suffix) = match &mode.output {
        catalog::types::OutputDefinition::SingleFile {
            extension,
            name_suffix,
            ..
        }
        | catalog::types::OutputDefinition::GenericRoot {
            extension,
            name_suffix,
            ..
        } => (extension.as_str(), name_suffix.as_str()),
        _ => return Err("this mode does not produce a file".into()),
    };
    runtime::suggest_output_path(&primary_input, suffix.trim_start_matches('-'), extension)
        .map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
fn enqueue_process(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    request: runtime::RunProcessRequest,
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<runtime::RunAccepted, String> {
    if !window.label().starts_with("process-") {
        return Err("processing requires a process window".into());
    }
    let catalog = catalog::bundled_catalog()?;
    runtime::validate_request_files(&catalog, &request)?;
    let process = catalog
        .iter()
        .find(|p| p.id == request.process_id)
        .ok_or("unknown process")?;
    let binary_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");
    let binary = resolve_binary(&app, process.identity.executable, &binary_root)?;
    let sfprops = resolve_binary(&app, catalog::types::BinaryId::Sfprops, &binary_root)?;
    let mode = process
        .modes
        .iter()
        .find(|m| m.id == request.mode_id)
        .ok_or("unknown mode")?;
    let mut metadata = std::collections::HashMap::new();
    for input in &mode.inputs {
        if let Some(path) = request
            .inputs
            .get(&input.id)
            .and_then(|paths| paths.first())
        {
            if matches!(
                runtime::file_type_for_path(std::path::Path::new(path)),
                Some(
                    catalog::types::CdpFileType::Soundfile
                        | catalog::types::CdpFileType::AnalysisAna
                        | catalog::types::CdpFileType::AnalysisPvx
                )
            ) {
                let canonical = std::fs::canonicalize(path)
                    .map_err(|_| format!("input does not exist: {path}"))?;
                metadata.insert(
                    input.id.clone(),
                    runtime::inspect_audio_metadata(&canonical, &sfprops)?,
                );
            }
        }
    }
    runtime::validate_metadata_constraints(mode, &metadata)?;
    runtime::evaluate_constraints(mode, &request.parameters, &metadata)?;
    let (command, _) = runtime::compile_request(&catalog, &request, binary)?;
    state
        .lock()
        .map_err(|_| "run registry unavailable".to_string())?
        .enqueue(command)
}

#[tauri::command]
fn reveal_artifact(
    app: tauri::AppHandle,
    run_id: Uuid,
    artifact_index: usize,
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<(), String> {
    let run = state
        .lock()
        .map_err(|_| "run registry unavailable".to_string())?
        .get(run_id)
        .ok_or("run not found")?;
    let artifact = run
        .artifacts
        .get(artifact_index)
        .ok_or("artifact not found")?;
    if artifact.path != "<stdout>" && !std::path::Path::new(&artifact.path).is_file() {
        return Err("artifact is missing; it may have been moved or deleted".into());
    }
    app.opener()
        .open_path(&artifact.path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn allow_artifact_playback(
    app: tauri::AppHandle,
    run_id: Uuid,
    artifact_index: usize,
    state: tauri::State<'_, runtime::SharedRunRegistry>,
) -> Result<String, String> {
    let run = state
        .lock()
        .map_err(|_| "run registry unavailable".to_string())?
        .get(run_id)
        .ok_or("run not found")?;
    let artifact = run
        .artifacts
        .get(artifact_index)
        .ok_or("artifact not found")?;
    if !artifact.playable {
        return Err("artifact is not a playable soundfile".into());
    }
    if !std::path::Path::new(&artifact.path).is_file() {
        return Err("artifact is missing; it may have been moved or deleted".into());
    }
    app.asset_protocol_scope()
        .allow_file(&artifact.path)
        .map_err(|e| format!("could not scope artifact playback: {e}"))?;
    Ok(format!("asset://localhost/{}", asset_path(&artifact.path)))
}

fn resolve_binary(
    app: &tauri::AppHandle,
    id: catalog::types::BinaryId,
    fallback: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    if let Ok(resources) = app.path().resource_dir() {
        if let Ok(path) = runtime::resolve_packaged(&resources.join("binaries"), id) {
            return Ok(path);
        }
    }
    runtime::resolve_packaged(fallback, id)
}

fn asset_path(path: &str) -> String {
    path.as_bytes()
        .iter()
        .map(|b| match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' => {
                (*b as char).to_string()
            }
            b'\\' => "/".into(),
            b => format!("%{b:02X}"),
        })
        .collect()
}

/// Tauri application entry point kept separate so integration tests can use the
/// catalog and compiler without starting a native webview.
pub fn run() {
    let registry = std::sync::Arc::new(Mutex::new(runtime::RunRegistry::new()));
    let _worker = runtime::spawn_worker(
        registry.clone(),
        std::sync::Arc::new(runtime::PackagedRunner),
    );
    let shutdown_registry = registry.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(registry)
        .invoke_handler(tauri::generate_handler![
            get_process_catalog,
            preview_process,
            open_process_window,
            list_runs,
            get_run,
            cancel_process,
            inspect_file,
            suggest_output_path,
            enqueue_process,
            reveal_artifact,
            allow_artifact_playback
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_app, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                if let Ok(mut runs) = shutdown_registry.lock() {
                    runs.shutdown();
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_window_url_accepts_a_mode_from_the_process() {
        let catalog = catalog::bundled_catalog().expect("catalog should load");
        let instance = Uuid::nil();
        let url = process_window_url(&catalog, "modify-speed", Some("semitones"), instance)
            .expect("recipe mode should be valid");
        assert_eq!(
            url,
            format!("/#/process/modify-speed?instance={instance}&mode=semitones")
        );
    }

    #[test]
    fn process_window_url_rejects_a_mode_from_another_process() {
        let catalog = catalog::bundled_catalog().expect("catalog should load");
        let error = process_window_url(&catalog, "modify-speed", Some("normalise"), Uuid::nil())
            .expect_err("a mismatched mode must be rejected");
        assert_eq!(error, "unknown mode for process");
    }
}
