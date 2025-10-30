use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

#[test]
fn test_validate_resource_basic_structure() {
    let patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin(patient)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Patient"));
}

#[test]
fn test_validate_resource_invalid_json() {
    let invalid_json = "{ not valid json }";

    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin(invalid_json)
        .assert()
        .failure();
}

#[test]
fn test_validate_resource_missing_resource_type() {
    let resource = r#"{
        "id": "example",
        "name": "test"
    }"#;

    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin(resource)
        .assert()
        .failure()
        .stdout(predicate::str::contains(
            "Missing required field 'resourceType'",
        ));
}

#[test]
fn test_validate_resource_json_output() {
    let patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    rh_cmd()
        .args(["validate", "resource", "--format", "json"])
        .write_stdin(patient)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("\"valid\""))
        .stdout(predicate::str::contains("\"resourceType\""));
}

#[test]
fn test_validate_resource_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("patient.json");

    let patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    fs::write(&file_path, patient).unwrap();

    rh_cmd()
        .args([
            "validate",
            "resource",
            "--input",
            file_path.to_str().unwrap(),
        ])
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Patient"));
}

#[test]
fn test_validate_resource_with_cli_flags() {
    let patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    rh_cmd()
        .args(["validate", "resource", "--strict"])
        .write_stdin(patient)
        .assert()
        .code(predicate::in_iter([0, 1]));
}

#[test]
fn test_validate_batch_valid_resources() {
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}
{"resourceType": "Observation", "id": "2", "status": "final", "code": {"text": "Test"}}
{"resourceType": "Organization", "id": "3", "name": "Test Org"}"#;

    rh_cmd()
        .args(["validate", "batch"])
        .write_stdin(ndjson)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Batch Validation Summary"))
        .stdout(predicate::str::contains("Total resources: 3"));
}

#[test]
fn test_validate_batch_with_invalid() {
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}
{"id": "2"}
{"resourceType": "Organization", "id": "3", "name": "Test Org"}"#;

    rh_cmd()
        .args(["validate", "batch"])
        .write_stdin(ndjson)
        .assert()
        .failure()
        .stdout(predicate::str::contains("Batch Validation Summary"));
}

#[test]
fn test_validate_batch_summary_only() {
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}
{"resourceType": "Observation", "id": "2", "status": "final", "code": {"text": "Test"}}"#;

    rh_cmd()
        .args(["validate", "batch", "--summary-only"])
        .write_stdin(ndjson)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Total resources: 2"));
}

#[test]
fn test_validate_batch_json_output() {
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}
{"resourceType": "Organization", "id": "2", "name": "Test Org"}"#;

    rh_cmd()
        .args(["validate", "batch", "--format", "json"])
        .write_stdin(ndjson)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("\"summary\""))
        .stdout(predicate::str::contains("\"results\""));
}

#[test]
fn test_validate_batch_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("resources.ndjson");

    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}
{"resourceType": "Organization", "id": "2", "name": "Test Org"}"#;

    fs::write(&file_path, ndjson).unwrap();

    rh_cmd()
        .args(["validate", "batch", "--input", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Total resources: 2"));
}

#[test]
fn test_validate_batch_empty_lines() {
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "name": [{"family": "Smith"}], "gender": "male"}

{"resourceType": "Organization", "id": "2", "name": "Test Org"}
"#;

    rh_cmd()
        .args(["validate", "batch"])
        .write_stdin(ndjson)
        .assert()
        .code(predicate::in_iter([0, 1]))
        .stdout(predicate::str::contains("Total resources: 2"));
}

#[test]
fn test_validate_empty_input() {
    rh_cmd()
        .args(["validate", "resource"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(predicate::str::contains("Input is empty"));
}

#[test]
fn test_validate_empty_input_strict() {
    rh_cmd()
        .args(["validate", "resource", "--strict"])
        .write_stdin("")
        .assert()
        .failure();
}
