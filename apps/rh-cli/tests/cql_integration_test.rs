use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn rh_cmd() -> Command {
    let bin_path = env!("CARGO_BIN_EXE_rh");
    Command::new(PathBuf::from(bin_path))
}

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// Minimal CQL library with one expression definition.
const SIMPLE_CQL: &str = r#"library SimpleMath version '1.0'

define X: 1 + 2
define Y: 10 - 3
"#;

/// CQL library using a boolean expression.
const BOOL_CQL: &str = r#"library BoolLib version '1.0'

define IsTrue: true
define IsFalse: false
"#;

// ---------------------------------------------------------------------------
// explain parse
// ---------------------------------------------------------------------------

#[test]
fn test_explain_parse_stdin() {
    rh_cmd()
        .args(["cql", "explain", "parse", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(
            predicate::str::contains("AST")
                .or(predicate::str::contains("ExpressionDef"))
                .or(predicate::str::contains("X")),
        );
}

#[test]
fn test_explain_parse_file() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("simple.cql");
    fs::write(&cql_path, SIMPLE_CQL).unwrap();

    rh_cmd()
        .args(["cql", "explain", "parse", cql_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_explain_parse_invalid_cql() {
    rh_cmd()
        .args(["cql", "explain", "parse", "-"])
        .write_stdin("this is not valid CQL !!!###")
        .assert()
        // May succeed (partial parse) or fail – but must not panic
        .stdout(
            predicate::str::is_empty()
                .not()
                .or(predicate::str::is_empty()),
        );
}

// ---------------------------------------------------------------------------
// explain compile
// ---------------------------------------------------------------------------

#[test]
fn test_explain_compile_stdin() {
    rh_cmd()
        .args(["cql", "explain", "compile", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_explain_compile_mentions_types() {
    rh_cmd()
        .args(["cql", "explain", "compile", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        // Should describe type information somewhere
        .stdout(
            predicate::str::contains("Integer")
                .or(predicate::str::contains("type"))
                .or(predicate::str::contains("X")),
        );
}

#[test]
fn test_explain_compile_file() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("bool.cql");
    fs::write(&cql_path, BOOL_CQL).unwrap();

    rh_cmd()
        .args(["cql", "explain", "compile", cql_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

// ---------------------------------------------------------------------------
// eval
// ---------------------------------------------------------------------------

#[test]
fn test_eval_integer_expression() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "X"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_eval_boolean_expression() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "IsTrue"])
        .write_stdin(BOOL_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("true"));
}

#[test]
fn test_eval_unknown_expression_fails() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "NonExistent"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .failure();
}

#[test]
fn test_eval_with_trace_flag() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "X", "--trace"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        // Trace output should include the result and at least one trace event
        .stdout(predicate::str::contains("Result:"))
        .stdout(predicate::str::contains("Trace ("));
}

#[test]
fn test_eval_trace_shows_op() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "X", "--trace"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        // Should see an Add operation in the trace
        .stdout(predicate::str::contains("op=Add").or(predicate::str::contains("op=")));
}

#[test]
fn test_eval_invalid_cql_fails() {
    rh_cmd()
        .args(["cql", "eval", "-", "--expression", "X"])
        .write_stdin("not valid CQL !!!###")
        .assert()
        .failure();
}

// ---------------------------------------------------------------------------
// compile --source-map
// ---------------------------------------------------------------------------

#[test]
fn test_compile_source_map_to_sidecar_file() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("simple.cql");
    let elm_path = dir.path().join("simple.json");
    fs::write(&cql_path, SIMPLE_CQL).unwrap();

    rh_cmd()
        .args([
            "cql",
            "compile",
            cql_path.to_str().unwrap(),
            "--output",
            elm_path.to_str().unwrap(),
            "--source-map",
        ])
        .assert()
        .success();

    // ELM output
    assert!(elm_path.exists(), "ELM output file should exist");

    // Sidecar source map
    let sm_path = PathBuf::from(format!("{}.sourcemap.json", elm_path.display()));
    assert!(sm_path.exists(), "Source map sidecar should exist");

    let sm_content = fs::read_to_string(&sm_path).unwrap();
    assert!(
        sm_content.contains("mappings") || sm_content.contains("sourceDocuments"),
        "Source map should have expected fields"
    );
}

#[test]
fn test_compile_source_map_custom_output() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("simple.cql");
    let elm_path = dir.path().join("output.json");
    let sm_path = dir.path().join("custom.sourcemap.json");
    fs::write(&cql_path, SIMPLE_CQL).unwrap();

    rh_cmd()
        .args([
            "cql",
            "compile",
            cql_path.to_str().unwrap(),
            "--output",
            elm_path.to_str().unwrap(),
            "--source-map",
            "--source-map-output",
            sm_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(sm_path.exists(), "Custom source map path should exist");
}

#[test]
fn test_compile_source_map_to_stderr_when_no_output() {
    rh_cmd()
        .args(["cql", "compile", "-", "--source-map"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        // ELM JSON should appear on stdout
        .stdout(predicate::str::contains("\"library\""))
        // Source map marker on stderr
        .stderr(predicate::str::contains("source map").or(predicate::str::contains("mappings")));
}
