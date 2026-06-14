use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

fn parse_envelope(stdout: &[u8]) -> serde_json::Value {
    serde_json::from_slice(stdout).expect("stdout should be valid JSON envelope")
}

const VALID_PATIENT: &str = r#"{"resourceType":"Patient","id":"ws3-test","name":[{"family":"Test","given":["Unit"]}],"gender":"male"}"#;

const INVALID_JSON: &str = r#"{"resourceType":"Patient","id":"ws3-test""#;

#[test]
fn test_version() {
    rh_cmd()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicates::str::contains("rh"));
}

#[test]
fn test_help() {
    rh_cmd()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--output-format"))
        .stdout(predicates::str::contains("--pretty"))
        .stdout(predicates::str::contains("--quiet"))
        .stdout(predicates::str::contains("--color"));
}

#[test]
fn test_validate_human_success() {
    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0);
}

#[test]
fn test_validate_human_validation_error() {
    let invalid_resource = r#"{"resourceType":"Patient","id":"ws3-test","name":"not-an-array"}"#;
    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin(invalid_resource)
        .assert()
        .code(3);
}

#[test]
fn test_validate_json_envelope_valid() {
    let output = rh_cmd()
        .args(["--output-format", "json", "validate", "resource"])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"].is_object());
    assert!(envelope["meta"]["version"].is_string());
    assert_eq!(envelope["meta"]["command"], "validate");
}

#[test]
fn test_validate_json_envelope_parse_error() {
    let output = rh_cmd()
        .args(["--output-format", "json", "--quiet", "validate", "resource"])
        .env("NO_COLOR", "1")
        .write_stdin(INVALID_JSON)
        .assert()
        .code(4)
        .get_output()
        .clone();

    let stderr = String::from_utf8(output.stderr).unwrap();
    let json_line = stderr
        .lines()
        .filter(|l| l.starts_with('{'))
        .last()
        .expect("should find JSON envelope in stderr");
    let envelope: serde_json::Value = serde_json::from_str(json_line).unwrap();
    assert_eq!(envelope["ok"], false);
    assert!(envelope["errors"].is_array());
}

#[test]
fn test_validate_ndjson_envelope() {
    let output = rh_cmd()
        .args(["--output-format", "ndjson", "validate", "resource"])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"].is_object());
}

#[test]
fn test_validate_pretty_json() {
    let output = rh_cmd()
        .args([
            "--output-format",
            "json",
            "--pretty",
            "validate",
            "resource",
        ])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0)
        .get_output()
        .clone();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains('\n'), "pretty JSON should be multiline");
}

#[test]
fn test_fsh_parse_human() {
    let fsh_input = r#"Profile: WS3Test
Parent: Patient
* name 1..1"#;
    rh_cmd()
        .args(["fsh", "parse", "-"])
        .write_stdin(fsh_input)
        .assert()
        .code(0);
}

#[test]
fn test_fsh_parse_json_envelope() {
    let fsh_input = r#"Profile: WS3Test
Parent: Patient
* name 1..1"#;
    let output = rh_cmd()
        .args(["--output-format", "json", "fsh", "parse", "-"])
        .write_stdin(fsh_input)
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"].is_object());
    assert_eq!(envelope["meta"]["command"], "fsh");
}

#[test]
fn test_fsh_tank_json_envelope() {
    let fsh_input = r#"Profile: WS3Test
Parent: Patient
* name 1..1"#;
    let output = rh_cmd()
        .args(["--output-format", "json", "fsh", "tank", "-"])
        .write_stdin(fsh_input)
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"]["profiles"].is_number());
    assert_eq!(envelope["meta"]["command"], "fsh");
}

#[test]
fn test_codegen_missing_package_json_envelope() {
    let output = rh_cmd()
        .args([
            "--output-format",
            "json",
            "--quiet",
            "codegen",
            "nonexistent.package@1.0",
        ])
        .env("NO_COLOR", "1")
        .assert()
        .code(1)
        .get_output()
        .clone();

    let stderr = String::from_utf8(output.stderr).unwrap();
    let json_line = stderr
        .lines()
        .filter(|l| l.starts_with('{'))
        .last()
        .expect("should find JSON envelope in stderr");
    let envelope: serde_json::Value = serde_json::from_str(json_line).unwrap();
    assert_eq!(envelope["ok"], false);
    assert!(envelope["errors"].is_array());
}

#[test]
fn test_fhirpath_parse_json_envelope() {
    let output = rh_cmd()
        .args([
            "--output-format",
            "json",
            "fhirpath",
            "parse",
            "name.family",
        ])
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"].is_object());
    assert_eq!(envelope["meta"]["command"], "fhirpath");
}

#[test]
fn test_fhirpath_eval_json_envelope() {
    let output = rh_cmd()
        .args(["--output-format", "json", "fhirpath", "eval", "1 + 2"])
        .assert()
        .code(0)
        .get_output()
        .clone();

    let envelope = parse_envelope(&output.stdout);
    assert_eq!(envelope["ok"], true);
    assert!(envelope["result"].is_number());
}

#[test]
fn test_completions_bash() {
    rh_cmd()
        .args(["completions", "bash"])
        .assert()
        .code(0)
        .stdout(predicates::str::contains("_rh"));
}

#[test]
fn test_completions_zsh() {
    rh_cmd().args(["completions", "zsh"]).assert().code(0);
}

#[test]
fn test_cql_compile_human() {
    let cql = r#"library Ws3Test version '1.0'
define X: 1 + 2
"#;
    rh_cmd()
        .args(["cql", "compile", "-"])
        .write_stdin(cql)
        .assert()
        .code(0);
}

#[test]
fn test_cql_validate_human() {
    let cql = r#"library Ws3Validate version '1.0'
define X: 1 + 2
"#;
    rh_cmd()
        .args(["cql", "validate", "-"])
        .write_stdin(cql)
        .assert()
        .code(0);
}

#[test]
fn test_vcl_parse_help() {
    rh_cmd().args(["vcl", "parse", "--help"]).assert().code(0);
}

#[test]
fn test_snapshot_diff_not_implemented() {
    rh_cmd()
        .args([
            "snapshot",
            "diff",
            "http://example.com/1",
            "http://example.com/2",
        ])
        .assert()
        .code(1);
}

#[test]
fn test_snapshot_validate_not_implemented() {
    rh_cmd()
        .args(["snapshot", "validate", "/tmp/nonexistent.json"])
        .assert()
        .code(1);
}

#[test]
fn test_package_help() {
    rh_cmd().args(["package", "--help"]).assert().code(0);
}

#[test]
fn test_download_help() {
    rh_cmd().args(["download", "--help"]).assert().code(0);
}

#[test]
fn test_quiet_flag() {
    rh_cmd()
        .args(["--quiet", "validate", "resource"])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0);
}

#[test]
fn test_color_never() {
    rh_cmd()
        .args(["--color", "never", "validate", "resource"])
        .write_stdin(VALID_PATIENT)
        .assert()
        .code(0);
}
