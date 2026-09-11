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
    include_str!("../../src/processes/definitions/sfedit-cut.json"),
    include_str!("../../src/processes/definitions/sfedit-cutend.json"),
    include_str!("../../src/processes/definitions/sfedit-excise.json"),
    include_str!("../../src/processes/definitions/sfedit-insil.json"),
    include_str!("../../src/processes/definitions/sfedit-insert.json"),
    include_str!("../../src/processes/definitions/sfedit-replace.json"),
    include_str!("../../src/processes/definitions/sfedit-cutmany.json"),
    include_str!("../../src/processes/definitions/sfedit-masks.json"),
    include_str!("../../src/processes/definitions/modify-space.json"),
    include_str!("../../src/processes/definitions/modify-revecho.json"),
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

#[test]
fn new_processes_compile_exact_release_7_1_argument_vectors() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");
    let output = |name: &str| directory.path().join(name).to_string_lossy().into_owned();
    let compile = |request: RunProcessRequest, binary: &str| {
        compile_request(&catalog, &request, binary.into())
            .expect("new process request should compile")
            .0
            .args
    };

    let cut = compile(
        RunProcessRequest {
            process_id: "sfedit-cut".into(),
            mode_id: "seconds".into(),
            inputs: HashMap::from([("source".into(), vec!["source.wav".into()])]),
            parameters: HashMap::from([
                ("start".into(), ParameterValue::Number { value: 0.25 }),
                ("end".into(), ParameterValue::Number { value: 1.5 }),
                ("spliceMs".into(), ParameterValue::Number { value: 15.0 }),
            ]),
            output_path: Some(output("cut.wav")),
        },
        "sfedit",
    );
    assert_eq!(
        cut,
        vec![
            OsString::from("cut"),
            OsString::from("1"),
            OsString::from("source.wav"),
            OsString::from(output("cut.wav")),
            OsString::from("0.25"),
            OsString::from("1.5"),
            OsString::from("-w15"),
        ]
    );

    let insert = compile(
        RunProcessRequest {
            process_id: "sfedit-insert".into(),
            mode_id: "seconds".into(),
            inputs: HashMap::from([
                ("source".into(), vec!["base.wav".into()]),
                ("insert".into(), vec!["piece.wav".into()]),
            ]),
            parameters: HashMap::from([
                ("time".into(), ParameterValue::Number { value: 2.0 }),
                ("spliceMs".into(), ParameterValue::Number { value: 15.0 }),
                ("level".into(), ParameterValue::Number { value: 0.5 }),
                ("overwrite".into(), ParameterValue::Flag { value: true }),
            ]),
            output_path: Some(output("insert.wav")),
        },
        "sfedit",
    );
    assert_eq!(
        insert,
        vec![
            OsString::from("insert"),
            OsString::from("1"),
            OsString::from("base.wav"),
            OsString::from("piece.wav"),
            OsString::from(output("insert.wav")),
            OsString::from("2"),
            OsString::from("-w15"),
            OsString::from("-l0.5"),
            OsString::from("-o"),
        ]
    );

    let cutmany = compile(
        RunProcessRequest {
            process_id: "sfedit-cutmany".into(),
            mode_id: "seconds".into(),
            inputs: HashMap::from([
                ("source".into(), vec!["source.wav".into()]),
                ("cuts".into(), vec!["cuts.txt".into()]),
            ]),
            parameters: HashMap::from([(
                "spliceMs".into(),
                ParameterValue::Number { value: 10.0 },
            )]),
            output_path: Some(output("segments.wav")),
        },
        "sfedit",
    );
    assert_eq!(
        cutmany,
        vec![
            OsString::from("cutmany"),
            OsString::from("1"),
            OsString::from("source.wav"),
            OsString::from(output("segments.wav")),
            OsString::from("cuts.txt"),
            OsString::from("10"),
        ]
    );

    let space = compile(
        RunProcessRequest {
            process_id: "modify-space".into(),
            mode_id: "pan".into(),
            inputs: HashMap::from([("source".into(), vec!["mono.wav".into()])]),
            parameters: HashMap::from([
                ("pan".into(), ParameterValue::Number { value: -0.25 }),
                ("prescale".into(), ParameterValue::Number { value: 0.7 }),
            ]),
            output_path: Some(output("space.wav")),
        },
        "modify",
    );
    assert_eq!(
        space,
        vec![
            OsString::from("space"),
            OsString::from("1"),
            OsString::from("mono.wav"),
            OsString::from(output("space.wav")),
            OsString::from("-0.25"),
            OsString::from("-p0.7"),
        ]
    );

    let revecho = compile(
        RunProcessRequest {
            process_id: "modify-revecho".into(),
            mode_id: "standard".into(),
            inputs: HashMap::from([("source".into(), vec!["source.wav".into()])]),
            parameters: HashMap::from([
                ("delayMs".into(), ParameterValue::Number { value: 250.0 }),
                ("mix".into(), ParameterValue::Number { value: 0.5 }),
                ("feedback".into(), ParameterValue::Number { value: 0.3 }),
                ("tail".into(), ParameterValue::Number { value: 1.0 }),
                ("prescale".into(), ParameterValue::Number { value: 0.7 }),
                ("inverted".into(), ParameterValue::Flag { value: true }),
            ]),
            output_path: Some(output("echo.wav")),
        },
        "modify",
    );
    assert_eq!(
        revecho,
        vec![
            OsString::from("revecho"),
            OsString::from("1"),
            OsString::from("source.wav"),
            OsString::from(output("echo.wav")),
            OsString::from("250"),
            OsString::from("0.5"),
            OsString::from("0.3"),
            OsString::from("1"),
            OsString::from("-p0.7"),
            OsString::from("-i"),
        ]
    );

    let masks = compile(
        RunProcessRequest {
            process_id: "sfedit-masks".into(),
            mode_id: "seconds".into(),
            inputs: HashMap::from([
                ("source".into(), vec!["source.wav".into()]),
                ("excisefile".into(), vec!["mask.txt".into()]),
            ]),
            parameters: HashMap::from([(
                "spliceMs".into(),
                ParameterValue::Number { value: 15.0 },
            )]),
            output_path: Some(output("masked.wav")),
        },
        "sfedit",
    );
    assert_eq!(
        masks,
        vec![
            OsString::from("masks"),
            OsString::from("1"),
            OsString::from("source.wav"),
            OsString::from(output("masked.wav")),
            OsString::from("mask.txt"),
            OsString::from("-w15"),
        ]
    );
}
