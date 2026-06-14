use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

const FSH_CONTENT: &str = r#"Alias: $SCT = http://snomed.info/sct

Profile: MyPatient
Parent: Patient
Title: "My Patient Profile"
Description: "Test profile"
* name 1..* MS
"#;

fn write_fsh_fixture(dir: &Path) -> PathBuf {
    let path = dir.join("test.fsh");
    fs::write(&path, FSH_CONTENT).unwrap();
    path
}

#[test]
fn test_fsh_help() {
    rh_cmd()
        .args(["fsh", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FSH").or(predicate::str::contains("fsh")));
}

#[test]
fn test_fsh_parse_outputs_valid_json() {
    let dir = TempDir::new().unwrap();
    let fsh_path = write_fsh_fixture(dir.path());

    let output = rh_cmd()
        .args(["fsh", "parse", fsh_path.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert!(json.get("entities").is_some());
}

#[test]
fn test_fsh_parse_json_envelope() {
    let dir = TempDir::new().unwrap();
    let fsh_path = write_fsh_fixture(dir.path());

    let output = rh_cmd()
        .args([
            "--format",
            "json",
            "fsh",
            "parse",
            fsh_path.to_str().unwrap(),
        ])
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
fn test_fsh_tank_json_envelope() {
    let dir = TempDir::new().unwrap();
    let fsh_path = write_fsh_fixture(dir.path());

    let output = rh_cmd()
        .args([
            "--format",
            "json",
            "fsh",
            "tank",
            fsh_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert_eq!(json["result"]["profiles"], json!(1));
}

#[test]
fn test_fsh_compile_outputs_resource_json() {
    let dir = TempDir::new().unwrap();
    let fsh_path = write_fsh_fixture(dir.path());

    rh_cmd()
        .args(["fsh", "compile", fsh_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("StructureDefinition"))
        .stdout(predicate::str::contains("MyPatient"));
}

#[test]
fn test_fsh_tank_human_output() {
    let dir = TempDir::new().unwrap();
    let fsh_path = write_fsh_fixture(dir.path());

    rh_cmd()
        .args(["fsh", "tank", fsh_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Profiles:"));
}
