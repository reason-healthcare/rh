use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;
use tempfile::TempDir;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

#[test]
fn test_snapshot_help() {
    rh_cmd()
        .args(["snapshot", "--help"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("snapshot")
                .or(predicate::str::contains("StructureDefinition")),
        );
}

#[test]
fn test_snapshot_generate_help() {
    rh_cmd()
        .args(["snapshot", "generate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate snapshot"));
}

#[test]
fn test_snapshot_info_help() {
    rh_cmd()
        .args(["snapshot", "info", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Display information"));
}

#[test]
fn test_snapshot_generate_missing_package_fails() {
    let dir = TempDir::new().unwrap();

    rh_cmd()
        .args([
            "snapshot",
            "generate",
            "http://example.org/StructureDefinition/missing",
            "--package",
            "hl7.fhir.r4.core@4.0.1",
            "--packages-dir",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn test_snapshot_diff_fails() {
    rh_cmd()
        .args(["snapshot", "diff", "url1", "url2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: not yet implemented"));
}

#[test]
fn test_snapshot_diff_json_error_envelope() {
    let output = rh_cmd()
        .args(["--format", "json", "snapshot", "diff", "url1", "url2"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok": false"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(false));
}
