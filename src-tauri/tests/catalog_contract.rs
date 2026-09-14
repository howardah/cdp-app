//! Contract tests for the checked-in process manifests.
//!
//! These tests deliberately load the same JSON documents shipped to the UI;
//! they catch schema/serde drift before a manifest can reach the navigator.

use composers_desktop_application::catalog::load_catalog;
use composers_desktop_application::runtime::file_type_for_path;
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
    include_str!("../../src/processes/definitions/sfedit-zcut.json"),
    include_str!("../../src/processes/definitions/sfedit-zcuts.json"),
    include_str!("../../src/processes/definitions/sfedit-excises.json"),
    include_str!("../../src/processes/definitions/sfedit-noisecut.json"),
    include_str!("../../src/processes/definitions/modify-radical.json"),
    include_str!("../../src/processes/definitions/modify-convolve.json"),
    include_str!("../../src/processes/definitions/modify-stack.json"),
    include_str!("../../src/processes/definitions/modify-shudder.json"),
    include_str!("../../src/processes/definitions/modify-scaledpan.json"),
    include_str!("../../src/processes/definitions/modify-brassage.json"),
    include_str!("../../src/processes/definitions/modify-sausage.json"),
    include_str!("../../src/processes/definitions/modify-spaceform.json"),
    include_str!("../../src/processes/definitions/modify-findpan.json"),
    include_str!("../../src/processes/definitions/reverb.json"),
    include_str!("../../src/processes/definitions/filter-fixed.json"),
    include_str!("../../src/processes/definitions/filter-lohi.json"),
    include_str!("../../src/processes/definitions/filter-variable.json"),
    include_str!("../../src/processes/definitions/filter-sweeping.json"),
    include_str!("../../src/processes/definitions/filter-phasing.json"),
    include_str!("../../src/processes/definitions/filter-iterated.json"),
    include_str!("../../src/processes/definitions/filter-bank.json"),
    include_str!("../../src/processes/definitions/filter-userbank.json"),
    include_str!("../../src/processes/definitions/filter-varibank.json"),
    include_str!("../../src/processes/definitions/phasor.json"),
    include_str!("../../src/processes/definitions/blur-avrg.json"),
    include_str!("../../src/processes/definitions/blur-blur.json"),
    include_str!("../../src/processes/definitions/blur-chorus.json"),
    include_str!("../../src/processes/definitions/blur-drunk.json"),
    include_str!("../../src/processes/definitions/blur-noise.json"),
    include_str!("../../src/processes/definitions/blur-scatter.json"),
    include_str!("../../src/processes/definitions/blur-spread.json"),
    include_str!("../../src/processes/definitions/extend-drunk.json"),
    include_str!("../../src/processes/definitions/extend-repetitions.json"),
    include_str!("../../src/processes/definitions/extend-scramble.json"),
    include_str!("../../src/processes/definitions/extend-sequence.json"),
    include_str!("../../src/processes/definitions/extend-zigzag.json"),
    include_str!("../../src/processes/definitions/hover.json"),
    include_str!("../../src/processes/definitions/hover2.json"),
    include_str!("../../src/processes/definitions/sfecho-echo.json"),
    include_str!("../../src/processes/definitions/spectstr.json"),
    include_str!("../../src/processes/definitions/stretch-spectrum.json"),
    include_str!("../../src/processes/definitions/stretch-time.json"),
    include_str!("../../src/processes/definitions/submix-balance.json"),
    include_str!("../../src/processes/definitions/submix-merge.json"),
    include_str!("../../src/processes/definitions/submix-mergemany.json"),
    include_str!("../../src/processes/definitions/submix-crossfade.json"),
    include_str!("../../src/processes/definitions/submix-mix.json"),
    include_str!("../../src/processes/definitions/submix-interleave.json"),
    include_str!("../../src/processes/definitions/submix-pan.json"),
    include_str!("../../src/processes/definitions/submix-spacewarp.json"),
    include_str!("../../src/processes/definitions/blur-suppress.json"),
    include_str!("../../src/processes/definitions/blur-weave.json"),
    include_str!("../../src/processes/definitions/combine-cross.json"),
    include_str!("../../src/processes/definitions/combine-diff.json"),
    include_str!("../../src/processes/definitions/combine-interleave.json"),
    include_str!("../../src/processes/definitions/combine-max.json"),
    include_str!("../../src/processes/definitions/combine-mean.json"),
    include_str!("../../src/processes/definitions/combine-sum.json"),
    include_str!("../../src/processes/definitions/focus-accu.json"),
    include_str!("../../src/processes/definitions/focus-exag.json"),
    include_str!("../../src/processes/definitions/focus-focus.json"),
    include_str!("../../src/processes/definitions/focus-fold.json"),
    include_str!("../../src/processes/definitions/focus-freeze.json"),
    include_str!("../../src/processes/definitions/focus-hold.json"),
    include_str!("../../src/processes/definitions/blur-shuffle.json"),
    include_str!("../../src/processes/definitions/iterline.json"),
    include_str!("../../src/processes/definitions/iterlinef.json"),
    include_str!("../../src/processes/definitions/submix-inbetween.json"),
    include_str!("../../src/processes/definitions/submix-inbetween2.json"),
    include_str!("../../src/processes/definitions/submix-sync.json"),
    include_str!("../../src/processes/definitions/submix-syncattack.json"),
    include_str!("../../src/processes/definitions/submix-timewarp.json"),
    include_str!("../../src/processes/definitions/submix-faders.json"),
    include_str!("../../src/processes/definitions/submix-addtomix.json"),
    include_str!("../../src/processes/definitions/envel-attack.json"),
    include_str!("../../src/processes/definitions/envel-curtail.json"),
    include_str!("../../src/processes/definitions/envel-dovetail.json"),
    include_str!("../../src/processes/definitions/envel-tremolo.json"),
    include_str!("../../src/processes/definitions/envel-cyclic.json"),
    include_str!("../../src/processes/definitions/envel-swell.json"),
    include_str!("../../src/processes/definitions/envel-pluck.json"),
    include_str!("../../src/processes/definitions/envel-warp.json"),
    include_str!("../../src/processes/definitions/envel-impose.json"),
    include_str!("../../src/processes/definitions/envel-replace.json"),
    include_str!("../../src/processes/definitions/envel-extract.json"),
    include_str!("../../src/processes/definitions/envel-create.json"),
    include_str!("../../src/processes/definitions/envnu-expdecay.json"),
    include_str!("../../src/processes/definitions/envnu-peakchop.json"),
    include_str!("../../src/processes/definitions/flatten.json"),
    include_str!("../../src/processes/definitions/sfedit-joinseq.json"),
    include_str!("../../src/processes/definitions/sfedit-joindyn.json"),
    include_str!("../../src/processes/definitions/sfedit-randchunks.json"),
    include_str!("../../src/processes/definitions/sfedit-twixt.json"),
    include_str!("../../src/processes/definitions/sfedit-sphinx.json"),
    include_str!("../../src/processes/definitions/sfedit-syllables.json"),
    include_str!("../../src/processes/definitions/extend-baktobak.json"),
    include_str!("../../src/processes/definitions/bounce.json"),
    include_str!("../../src/processes/definitions/extend-freeze.json"),
    include_str!("../../src/processes/definitions/extend-iterate.json"),
    include_str!("../../src/processes/definitions/extend-loop.json"),
    include_str!("../../src/processes/definitions/envel-brktoenv.json"),
    include_str!("../../src/processes/definitions/envel-dbtoenv.json"),
    include_str!("../../src/processes/definitions/extend-doublets.json"),
    include_str!("../../src/processes/definitions/focus-step.json"),
];

#[test]
fn every_shipped_manifest_deserializes_and_has_modes() {
    let catalog =
        load_catalog(MANIFESTS).expect("all shipped manifests must be valid Rust catalog data");
    assert_eq!(catalog.len(), MANIFESTS.len());
    assert!(catalog.iter().all(|process| !process.modes.is_empty()));
}

#[test]
fn envelope_extension_and_playability_classification_are_typed() {
    assert_eq!(
        file_type_for_path(std::path::Path::new("result.evl")),
        Some(composers_desktop_application::catalog::types::CdpFileType::BinaryEnvelope)
    );
    assert_eq!(
        file_type_for_path(std::path::Path::new("result.env")),
        Some(composers_desktop_application::catalog::types::CdpFileType::BinaryEnvelope)
    );
    assert_eq!(
        file_type_for_path(std::path::Path::new("result.mix")),
        Some(composers_desktop_application::catalog::types::CdpFileType::Mixfile)
    );
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

#[test]
fn submix_wave_three_processes_compile_exact_argument_vectors() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");
    let output = |name: &str| directory.path().join(name).to_string_lossy().into_owned();

    let merge = RunProcessRequest {
        process_id: "submix-merge".into(),
        mode_id: "merge".into(),
        inputs: HashMap::from([("sources".into(), vec!["one.wav".into(), "two.wav".into()])]),
        parameters: HashMap::from([
            ("stagger".into(), ParameterValue::Number { value: 0.5 }),
            ("skip".into(), ParameterValue::Number { value: 0.25 }),
            ("skew".into(), ParameterValue::Number { value: 1.2 }),
            ("start".into(), ParameterValue::Number { value: 0.0 }),
            ("end".into(), ParameterValue::Number { value: 0.0 }),
        ]),
        output_path: Some(output("merge.wav")),
    };
    let (command, _) = compile_request(&catalog, &merge, "submix".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("merge"),
            OsString::from("one.wav"),
            OsString::from("two.wav"),
            OsString::from(output("merge.wav")),
            OsString::from("-s0.5"),
            OsString::from("-j0.25"),
            OsString::from("-k1.2"),
            OsString::from("-b0"),
            OsString::from("-e0"),
        ]
    );

    let balance = RunProcessRequest {
        process_id: "submix-balance".into(),
        mode_id: "balance".into(),
        inputs: HashMap::from([("sources".into(), vec!["one.wav".into(), "two.wav".into()])]),
        parameters: HashMap::from([("balance".into(), ParameterValue::Number { value: 0.75 })]),
        output_path: Some(output("balance.wav")),
    };
    let (command, _) = compile_request(&catalog, &balance, "submix".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("balance"),
            OsString::from("one.wav"),
            OsString::from("two.wav"),
            OsString::from(output("balance.wav")),
            OsString::from("-k0.75"),
        ]
    );
}

#[test]
fn sfecho_echo_compiles_exact_argument_vector() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");
    let output = directory.path().join("echo.wav");
    let request = RunProcessRequest {
        process_id: "sfecho-echo".into(),
        mode_id: "echo".into(),
        inputs: HashMap::from([("source".into(), vec!["source.wav".into()])]),
        parameters: HashMap::from([
            ("delay".into(), ParameterValue::Number { value: 6.0 }),
            ("attenuation".into(), ParameterValue::Number { value: 0.6 }),
            ("duration".into(), ParameterValue::Number { value: 12.0 }),
            ("randomize".into(), ParameterValue::Number { value: 0.0 }),
            ("cutoff".into(), ParameterValue::Number { value: -96.0 }),
        ]),
        output_path: Some(output.to_string_lossy().into_owned()),
    };
    let (command, _) = compile_request(&catalog, &request, "sfecho".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("echo"),
            OsString::from("source.wav"),
            output.into_os_string(),
            OsString::from("6"),
            OsString::from("0.6"),
            OsString::from("12"),
            OsString::from("-r0"),
            OsString::from("-c-96"),
        ]
    );
}

#[test]
fn extend_wave_three_processes_compile_exact_argument_vectors() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");
    let repetitions_output = directory.path().join("repetitions.wav");
    let repetitions = RunProcessRequest {
        process_id: "extend-repetitions".into(),
        mode_id: "repeat".into(),
        inputs: HashMap::from([
            ("source".into(), vec!["source.wav".into()]),
            ("times".into(), vec!["times.txt".into()]),
        ]),
        parameters: HashMap::from([("level".into(), ParameterValue::Number { value: 0.75 })]),
        output_path: Some(repetitions_output.to_string_lossy().into_owned()),
    };
    let (command, _) = compile_request(&catalog, &repetitions, "extend".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("repetitions"),
            OsString::from("source.wav"),
            repetitions_output.into_os_string(),
            OsString::from("times.txt"),
            OsString::from("0.75"),
        ]
    );

    let zigzag_output = directory.path().join("zigzag.wav");
    let zigzag = RunProcessRequest {
        process_id: "extend-zigzag".into(),
        mode_id: "random".into(),
        inputs: HashMap::from([("source".into(), vec!["source.wav".into()])]),
        parameters: HashMap::from([
            ("start".into(), ParameterValue::Number { value: 0.0 }),
            ("end".into(), ParameterValue::Number { value: 2.0 }),
            ("duration".into(), ParameterValue::Number { value: 10.0 }),
            ("minimumZig".into(), ParameterValue::Number { value: 0.1 }),
            ("spliceMs".into(), ParameterValue::Number { value: 25.0 }),
            ("maximumZig".into(), ParameterValue::Number { value: 1.0 }),
            ("seed".into(), ParameterValue::Number { value: 7.0 }),
        ]),
        output_path: Some(zigzag_output.to_string_lossy().into_owned()),
    };
    let (command, _) = compile_request(&catalog, &zigzag, "extend".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("zigzag"),
            OsString::from("1"),
            OsString::from("source.wav"),
            zigzag_output.into_os_string(),
            OsString::from("0"),
            OsString::from("2"),
            OsString::from("10"),
            OsString::from("0.1"),
            OsString::from("-s25"),
            OsString::from("-m1"),
            OsString::from("-r7"),
        ]
    );
}

#[test]
fn corrected_generic_processes_compile_and_resolve_output_roots() {
    let catalog = load_catalog(MANIFESTS).expect("catalog should load");
    let directory = tempfile::tempdir().expect("temporary output directory");

    let randchunks = RunProcessRequest {
        process_id: "sfedit-randchunks".into(),
        mode_id: "chunks".into(),
        inputs: HashMap::from([(
            "source".into(),
            vec![directory
                .path()
                .join("source.wav")
                .to_string_lossy()
                .into_owned()],
        )]),
        parameters: HashMap::from([
            ("count".into(), ParameterValue::Number { value: 2.0 }),
            (
                "minimumLength".into(),
                ParameterValue::Number { value: 0.05 },
            ),
            (
                "maximumLength".into(),
                ParameterValue::Number { value: 1.0 },
            ),
            (
                "evenlyDistributed".into(),
                ParameterValue::Flag { value: false },
            ),
            (
                "startAtBeginning".into(),
                ParameterValue::Flag { value: false },
            ),
        ]),
        output_path: None,
    };
    let (command, _) = compile_request(&catalog, &randchunks, "sfedit".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("randchunks"),
            directory.path().join("source.wav").into_os_string(),
            OsString::from("2"),
            OsString::from("0.05"),
            OsString::from("-m1"),
        ]
    );
    assert_eq!(command.output, Some(directory.path().join("sourc.wav")));

    let output = directory.path().join("blend.wav");
    let inbetween = RunProcessRequest {
        process_id: "submix-inbetween".into(),
        mode_id: "even-steps".into(),
        inputs: HashMap::from([
            ("sourceA".into(), vec!["first.wav".into()]),
            ("sourceB".into(), vec!["second.wav".into()]),
        ]),
        parameters: HashMap::from([("count".into(), ParameterValue::Number { value: 3.0 })]),
        output_path: Some(output.to_string_lossy().into_owned()),
    };
    let (command, _) = compile_request(&catalog, &inbetween, "submix".into()).unwrap();
    assert_eq!(
        command.args,
        vec![
            OsString::from("inbetween"),
            OsString::from("1"),
            OsString::from("first.wav"),
            OsString::from("second.wav"),
            output.into_os_string(),
            OsString::from("3"),
        ]
    );
}
