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
fn test_package_help() {
    rh_cmd()
        .args(["package", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FHIR Package"));
}

#[test]
fn test_package_init_help() {
    rh_cmd()
        .args(["package", "init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Scaffold a new FHIR Package"));
}

#[test]
fn test_package_init_creates_packager_toml() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("pkg");

    rh_cmd()
        .args([
            "package",
            "init",
            "--canonical",
            "https://example.org/fhir",
            target.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(target.join("packager.toml").exists());
    assert!(target.join("ImplementationGuide.json").exists());
}

#[test]
fn test_package_init_json_output() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("pkg-json");

    let output = rh_cmd()
        .args([
            "--format",
            "json",
            "package",
            "init",
            "--canonical",
            "https://example.org/fhir",
            target.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["ok"], json!(true));
    assert!(target.join("packager.toml").exists());
}

#[test]
fn test_package_init_derives_name_from_canonical() {
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("pkg-derived");

    rh_cmd()
        .args([
            "package",
            "init",
            "--canonical",
            "https://example.org/fhir",
            target.to_str().unwrap(),
        ])
        .assert()
        .success();

    let contents = fs::read_to_string(target.join("packager.toml")).unwrap();
    assert!(contents.contains("id           = \"example.fhir\""));
}

#[test]
fn test_package_lock_check_missing_arg_fails() {
    rh_cmd()
        .args(["package", "lock-check"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage: rh package lock-check"));
}
