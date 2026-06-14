use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

#[test]
fn test_download_help() {
    rh_cmd()
        .args(["download", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("download").or(predicate::str::contains("FHIR")));
}

#[test]
fn test_download_package_help() {
    rh_cmd()
        .args(["download", "package", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Package version"));
}

#[test]
fn test_download_list_help() {
    rh_cmd()
        .args(["download", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List available versions"));
}

#[test]
fn test_download_package_missing_args_fails() {
    rh_cmd()
        .args(["download", "package"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage: rh download package"));
}

#[test]
fn test_download_list_missing_args_fails() {
    rh_cmd()
        .args(["download", "list"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage: rh download list"));
}
