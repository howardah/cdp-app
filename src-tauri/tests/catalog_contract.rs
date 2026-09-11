//! Contract tests for the checked-in process manifests.
//!
//! These tests deliberately load the same JSON documents shipped to the UI;
//! they catch schema/serde drift before a manifest can reach the navigator.

use composers_desktop_application::catalog::load_catalog;

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
