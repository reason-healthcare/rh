use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

#[test]
fn test_codegen_help() {
    rh_cmd()
        .args(["codegen", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("codegen").or(predicate::str::contains("FHIR")));
}

#[test]
fn test_codegen_help_contains_global_flags() {
    rh_cmd()
        .args(["codegen", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--quiet"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("--color"));
}

#[test]
fn test_codegen_missing_package_args_fail() {
    rh_cmd()
        .args(["codegen"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage: rh codegen"));
}

#[test]
fn test_codegen_nonempty_output_dir_fails() {
    let dir = TempDir::new().unwrap();
    let output_dir = dir.path().join("generated");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join("existing.txt"), "present").unwrap();

    rh_cmd()
        .args([
            "codegen",
            "hl7.fhir.r4.core",
            "4.0.1",
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Output directory"));
}

#[test]
fn test_codegen_json_error_envelope_for_nonempty_output_dir() {
    let dir = TempDir::new().unwrap();
    let output_dir = dir.path().join("generated");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join("existing.txt"), "present").unwrap();

    let output = rh_cmd()
        .args([
            "--format",
            "json",
            "codegen",
            "hl7.fhir.r4.core",
            "4.0.1",
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok": false"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(false));
}
