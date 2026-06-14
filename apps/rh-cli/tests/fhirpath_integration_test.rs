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
fn test_fhirpath_help() {
    rh_cmd()
        .args(["fhirpath", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FHIRPath"));
}

#[test]
fn test_fhirpath_eval_human_output() {
    rh_cmd()
        .args(["fhirpath", "eval", "5 + 3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8"));
}

#[test]
fn test_fhirpath_eval_json_output() {
    let output = rh_cmd()
        .args(["--format", "json", "fhirpath", "eval", "5 + 3"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert_eq!(json["result"], json!(8));
}

#[test]
fn test_fhirpath_eval_json_is_valid() {
    let output = rh_cmd()
        .args(["--format", "json", "fhirpath", "eval", "1 + 1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert_eq!(json["result"], json!(2));
}

#[test]
fn test_fhirpath_parse_human_output() {
    rh_cmd()
        .args(["fhirpath", "parse", "name.given"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Successfully parsed")
                .or(predicate::str::contains("expression"))
                .or(predicate::str::contains("AST")),
        );
}

#[test]
fn test_fhirpath_parse_json_output() {
    rh_cmd()
        .args(["--format", "json", "fhirpath", "parse", "name.given"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#));
}

#[test]
fn test_fhirpath_eval_with_data_file() {
    let dir = TempDir::new().unwrap();
    let data_path = dir.path().join("patient.json");
    fs::write(&data_path, r#"{"resourceType":"Patient","id":"p1"}"#).unwrap();

    rh_cmd()
        .args([
            "fhirpath",
            "eval",
            "id",
            "--data",
            data_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("p1"));
}

#[test]
fn test_fhirpath_eval_ignores_piped_stdin_without_hanging() {
    let output = rh_cmd()
        .args(["--format", "json", "fhirpath", "eval", "true"])
        .write_stdin("\n")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert_eq!(json["result"], json!(true));
}

#[test]
fn test_fhirpath_eval_invalid_expression_fails() {
    rh_cmd()
        .args(["fhirpath", "eval", "!!!invalid!!!"])
        .assert()
        .failure();
}
