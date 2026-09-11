//! Contract tests for the checked-in process manifests.
//!
//! These tests deliberately load the same JSON documents shipped to the UI;
//! they catch schema/serde drift before a manifest can reach the navigator.

use composers_desktop_application::catalog::load_catalog;
use composers_desktop_application::runtime::{compile_request, ParameterValue, RunProcessRequest};
use std::collections::HashMap;
use std::ffi::OsString;

const MANIFESTS: &[&str] = &[
    include_str!("../../src/processes/definitions/modify-speed.json"),
    include_str!("../../src/processes/definitions/modify-loudness.json"),
    include_str!("../../src/processes/definitions/sfedit-join.json"),
    include_str!("../../src/processes/definitions/pvoc-analyze.json"),
    include_str!("../../src/processes/definitions/pvoc-synthesize.json"),
    include_str!("../../src/processes/definitions/isolate.json"),
];

#[test]
fn every_shipped_manifest_deserializes_and_has_modes() {
    let catalog =
        load_catalog(MANIFESTS).expect("all shipped manifests must be valid Rust catalog data");
    assert_eq!(catalog.len(), MANIFESTS.len());
    assert!(catalog.iter().all(|process| !process.modes.is_empty()));
}

#[test]
fn shipped_process_ids_are_unique_and_nonempty() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let mut ids = catalog
        .iter()
        .map(|process| process.id.as_str())
        .collect::<Vec<_>>();
    ids.sort_unstable();
    assert!(ids.iter().all(|id| !id.is_empty()));
    assert!(ids.windows(2).all(|pair| pair[0] != pair[1]));
}

#[test]
fn catalog_rejects_unsupported_schema_versions() {
    let mut value: serde_json::Value = serde_json::from_str(MANIFESTS[0]).unwrap();
    value["schemaVersion"] = serde_json::json!(2);
    let manifest = serde_json::to_string(&value).unwrap();
    let error = load_catalog(&[&manifest]).expect_err("schema version drift must be rejected");
    assert!(error.contains("unsupported catalog schema"));
}

#[test]
fn pvoc_recipe_steps_compile_the_release_8_command_tokens() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");
    let analysis_path = directory.path().join("source-analysis.ana");
    let analyze = RunProcessRequest {
        process_id: "pvoc-analyze".into(),
        mode_id: "analyze".into(),
        inputs: HashMap::from([("source".into(), vec!["source.wav".into()])]),
        parameters: HashMap::from([
            ("points".into(), ParameterValue::Number { value: 1024.0 }),
            ("overlap".into(), ParameterValue::Number { value: 3.0 }),
        ]),
        output_path: Some(analysis_path.to_string_lossy().into_owned()),
    };
    let (command, _) = compile_request(&catalog, &analyze, "pvoc".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("anal"),
            OsString::from("1"),
            OsString::from("source.wav"),
            analysis_path.clone().into_os_string(),
            OsString::from("-c1024"),
            OsString::from("-o3"),
        ]
    );

    let synthesize = RunProcessRequest {
        process_id: "pvoc-synthesize".into(),
        mode_id: "synthesize".into(),
        inputs: HashMap::from([(
            "analysis".into(),
            vec![analysis_path.to_string_lossy().into_owned()],
        )]),
        parameters: HashMap::new(),
        output_path: Some(
            directory
                .path()
                .join("rendered.wav")
                .to_string_lossy()
                .into_owned(),
        ),
    };
    let (command, _) = compile_request(&catalog, &synthesize, "pvoc".into()).unwrap();
    assert_eq!(command.args[0], OsString::from("synth"));
}
