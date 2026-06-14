use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use std::path::PathBuf;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

const SIMPLE_CQL: &str = r#"library Test version '1.0'

define X: 1
"#;

#[test]
fn test_global_help_lists_flags() {
    rh_cmd()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--quiet"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("--color"));
}

#[test]
fn test_global_version() {
    rh_cmd()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_global_format_human() {
    rh_cmd()
        .args(["--format", "human", "fhirpath", "eval", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_global_format_json_contains_ok() {
    rh_cmd()
        .args(["--format", "json", "fhirpath", "eval", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#));
}

#[test]
fn test_global_format_json_parses() {
    let output = rh_cmd()
        .args(["--format", "json", "fhirpath", "eval", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert_eq!(json["result"], json!(1));
}

#[test]
fn test_global_quiet_flag() {
    rh_cmd()
        .args(["--quiet", "fhirpath", "eval", "1"])
        .assert()
        .success();
}

#[test]
fn test_global_verbose_flag() {
    rh_cmd()
        .args(["-v", "fhirpath", "eval", "1"])
        .assert()
        .success();
}

#[test]
fn test_global_color_flag() {
    rh_cmd()
        .args(["--color", "never", "fhirpath", "eval", "1"])
        .assert()
        .success();
}

#[test]
fn test_cql_stdin_pipe() {
    rh_cmd()
        .args(["cql", "compile", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"library\""));
}
