use super::compiler::validate_output_path;
use super::*;
use crate::catalog::types::*;
use std::collections::HashMap;
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
fn sfprops_parser_accepts_release_8_sample_type_label() {
    let props = parse_sfprops(
            b"Format : Standard WAVE format\nSample Rate : 48000\nChannels : 1\nsample type: : 16-bit\nduration : 3.7500 secs\n",
            true,
        )
        .unwrap();
    assert_eq!(props.sample_format, "16-bit");
    assert_eq!(props.sample_rate, 48_000);
}

#[test]
fn sfprops_parser_accepts_pvoc_analysis_metadata() {
    let props = parse_sfprops(
            b"File type: CDP pvoc analysis file.\nChannel Format: Amplitude,Frequency\nOrig rate: 48000\nAnalysis Window Size: 1024\nAnalysis channels: 513\nAnalysis rate: 375.0000\nFrame count: 1415\nDuration (secs): 3.773\n",
            true,
        )
        .unwrap();
    assert_eq!(props.duration_seconds, 3.773);
    assert_eq!(props.sample_rate, 48_000);
    assert_eq!(props.channels, 1);
    assert_eq!(props.sample_format, "Amplitude,Frequency");
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
fn generic_discovery_accepts_padded_root_number_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("source");
    let old = dir.path().join("source001.wav");
    let new = dir.path().join("source002.wav");
    std::fs::write(&old, b"x").unwrap();
    std::fs::write(&new, b"x").unwrap();
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
