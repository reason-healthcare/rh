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
fn test_batch_validation_with_errors() {
    let tmp_dir = TempDir::new().unwrap();
    let batch_file = tmp_dir.path().join("batch.ndjson");

    let content = r#"{"resourceType": "Patient", "id": "p1"}
{"resourceType": "Patient", "id": "p2"}
{"resourceType": "Observation", "id": "o1"}
"#;

    std::fs::write(&batch_file, content).unwrap();

    rh_cmd()
        .arg("validate")
        .arg("batch")
        .arg(&batch_file)
        .assert()
        .failure();
}

#[test]
fn test_resource_validation_operationoutcome_format() {
    let patient = r#"{"resourceType": "Patient", "id": "test123", "contact": [{}]}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("operationoutcome")
        .write_stdin(patient)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(stdout.contains("\"issue\""));
}

#[test]
fn test_operationoutcome_has_severity() {
    let patient = r#"{"resourceType": "Patient", "id": "test", "contact": [{}]}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("operationoutcome")
        .write_stdin(patient)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"severity\""));
    assert!(
        stdout.contains("\"error\"")
            || stdout.contains("\"warning\"")
            || stdout.contains("\"information\"")
    );
}

#[test]
fn test_operationoutcome_has_code() {
    let patient = r#"{"resourceType": "Patient", "id": "test", "contact": [{}]}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("operationoutcome")
        .write_stdin(patient)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"code\""));
}

#[test]
fn test_operationoutcome_has_diagnostics() {
    let patient = r#"{"resourceType": "Patient", "id": "test", "contact": [{}]}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("operationoutcome")
        .write_stdin(patient)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"diagnostics\""));
}

#[test]
fn test_operationoutcome_missing_field() {
    let missing_resource_type = r#"{"id": "test123"}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("operationoutcome")
        .write_stdin(missing_resource_type)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(stdout.contains("\"severity\": \"error\""));
    assert!(stdout.contains("\"code\": \"required\""));
}

#[test]
fn test_batch_validation_operationoutcome_format() {
    let tmp_dir = TempDir::new().unwrap();
    let batch_file = tmp_dir.path().join("batch.ndjson");

    let content = r#"{"resourceType": "Patient", "id": "p1"}
{"resourceType": "Patient", "id": "p2"}
"#;

    std::fs::write(&batch_file, content).unwrap();

    let output = rh_cmd()
        .arg("validate")
        .arg("batch")
        .arg(&batch_file)
        .arg("--format")
        .arg("operationoutcome")
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    for line in lines {
        if !line.trim().is_empty() {
            assert!(line.contains("\"resourceType\": \"OperationOutcome\""));
        }
    }
}

#[test]
fn test_operationoutcome_uppercase_format() {
    let patient = r#"{"resourceType": "Patient", "id": "test", "contact": [{}]}"#;

    let output = rh_cmd()
        .arg("validate")
        .arg("resource")
        .arg("--format")
        .arg("OPERATIONOUTCOME")
        .write_stdin(patient)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("\"resourceType\": \"OperationOutcome\""));
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
