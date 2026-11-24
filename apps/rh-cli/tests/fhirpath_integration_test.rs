use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, value::RawValue, Value};
use std::collections::BTreeMap;
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
fn test_fhirpath_eval_preserves_exact_json_numbers() {
    let dir = TempDir::new().unwrap();
    let data_path = dir.path().join("parameters.json");
    fs::write(
        &data_path,
        r#"{"resourceType":"Parameters","parameter":[
            {"name":"exact","valueDecimal":1.23456789012345678900},
            {"name":"outside-decimal","valueDecimal":1e+100},
            {"name":"large","valueQuantity":{"value":1e999999999,"unit":"kg"}}
        ]}"#,
    )
    .unwrap();

    let result = rh_cmd()
        .args([
            "--format",
            "json",
            "fhirpath",
            "eval",
            "parameter.value",
            "--data",
            data_path.to_str().unwrap(),
        ])
        .assert()
        .success();
    let output: BTreeMap<String, Box<RawValue>> =
        serde_json::from_slice(&result.get_output().stdout).unwrap();
    assert_eq!(output["ok"].get(), "true");
    let values: Vec<Box<RawValue>> = serde_json::from_str(output["result"].get()).unwrap();
    assert_eq!(values.len(), 3);
    assert_eq!(values[0].get(), "1.23456789012345678900");
    assert_eq!(values[1].get(), "1e+100");
    let quantity: BTreeMap<String, Box<RawValue>> = serde_json::from_str(values[2].get()).unwrap();
    assert_eq!(quantity["value"].get(), "1e999999999");

    rh_cmd()
        .args([
            "fhirpath",
            "eval",
            "1.23456789012345678900",
            "--display-format",
            "json",
        ])
        .assert()
        .success()
        .stdout("1.23456789012345678900\n");
}

#[test]
fn test_fhirpath_test_comparisons_preserve_exact_json_numbers() {
    let dir = TempDir::new().unwrap();
    let data_path = dir.path().join("observation.json");
    let test_path = dir.path().join("tests.json");
    fs::write(
        &data_path,
        r#"{"resourceType":"Observation","valueQuantity":{"value":1e-999999999,"unit":"kg"}}"#,
    )
    .unwrap();
    fs::write(
        &test_path,
        r#"[
            {"expression":"value", "expected":{"unit":"kg","value":1e-999999999}},
            {"expression":"1.23456789012345678900", "expected":1.23456789012345678901}
        ]"#,
    )
    .unwrap();

    rh_cmd()
        .args([
            "fhirpath",
            "test",
            "--file",
            test_path.to_str().unwrap(),
            "--data",
            data_path.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Test Results: 1 passed, 1 failed"))
        .stdout(predicate::str::contains("Expected: 1.23456789012345678901"))
        .stdout(predicate::str::contains("Got:      1.23456789012345678900"));
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

#[test]
fn test_fhirpath_r5_types_and_source_paths_in_json() {
    let dir = TempDir::new().unwrap();
    let data_path = dir.path().join("parameters.json");
    fs::write(
        &data_path,
        r#"{"resourceType":"Parameters","parameter":[{"name":"count","valueInteger64":"9223372036854775807"}]}"#,
    ).unwrap();
    let mut command = rh_cmd();
    let result = command
        .args([
            "--format",
            "json",
            "fhirpath",
            "eval",
            "--fhir-version",
            "r5",
            "--data",
            data_path.to_str().unwrap(),
            "--context",
            "parameter.value",
            "$this.getValue() is System.Long and %resource is Parameters",
        ])
        .assert()
        .success();
    let output: Value = serde_json::from_slice(&result.get_output().stdout).unwrap();
    assert_eq!(output["result"], json!(true));

    let mut command = rh_cmd();
    let result = command
        .args([
            "--format",
            "json",
            "fhirpath",
            "eval",
            "--fhir-version",
            "r5",
            "--data",
            data_path.to_str().unwrap(),
            "parameter.value.pathname(true).first()",
        ])
        .assert()
        .success();
    let output: Value = serde_json::from_slice(&result.get_output().stdout).unwrap();
    assert_eq!(output["result"], json!("Parameters.parameter[0].value"));
}
