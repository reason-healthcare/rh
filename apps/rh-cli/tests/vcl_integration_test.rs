use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

const VALID_VCL: &str = "(http://snomed.info/sct)123456";

#[test]
fn test_vcl_help() {
    rh_cmd()
        .args(["vcl", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("VCL"));
}

#[test]
fn test_vcl_parse_human_output() {
    rh_cmd()
        .args(["vcl", "parse", VALID_VCL])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("AST").or(predicate::str::contains("parsed successfully")),
        );
}

#[test]
fn test_vcl_parse_json_output() {
    let output = rh_cmd()
        .args(["--format", "json", "vcl", "parse", VALID_VCL])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
}

#[test]
fn test_vcl_translate_json_output() {
    let output = rh_cmd()
        .args(["--format", "json", "vcl", "translate", VALID_VCL])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
}

#[test]
fn test_vcl_explain_json_output() {
    let output = rh_cmd()
        .args(["--format", "json", "vcl", "explain", VALID_VCL])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
}

#[test]
fn test_vcl_parse_invalid_expression_fails() {
    rh_cmd().args(["vcl", "parse", "((("]).assert().failure();
}
