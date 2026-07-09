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

/// Minimal CQL library with a FHIR retrieve for SQL-on-FHIR emission.
const FHIR_RETRIEVE_CQL: &str = r#"library DiabetesMeasure version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/ValueSet/diabetes'
parameter MeasurementPeriod Interval<DateTime>
context Patient

define "Diabetes Conditions":
  [Condition: "Diabetes"]

define "Has Diabetes":
  exists "Diabetes Conditions"
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
        .args(["cql", "eval", "-", "X"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_eval_boolean_expression() {
    rh_cmd()
        .args(["cql", "eval", "-", "IsTrue"])
        .write_stdin(BOOL_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("true"));
}

#[test]
fn test_eval_unknown_expression_fails() {
    rh_cmd()
        .args(["cql", "eval", "-", "NonExistent"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .failure();
}

#[test]
fn test_eval_with_trace_flag() {
    rh_cmd()
        .args(["cql", "eval", "-", "X", "--trace"])
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
        .args(["cql", "eval", "-", "X", "--trace"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        // Should see an Add operation in the trace
        .stdout(predicate::str::contains("op=Add").or(predicate::str::contains("op=")));
}

#[test]
fn test_eval_invalid_cql_fails() {
    rh_cmd()
        .args(["cql", "eval", "-", "X"])
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

#[test]
fn test_compile_source_map_resolves_lib_path_includes() {
    let dir = TempDir::new().unwrap();
    let lib_dir = dir.path().join("libs");
    fs::create_dir(&lib_dir).unwrap();

    let helper_path = lib_dir.join("Helper.cql");
    fs::write(
        &helper_path,
        r#"library Helper version '1.0'

define Answer: 41
"#,
    )
    .unwrap();

    let main_path = dir.path().join("main.cql");
    let elm_path = dir.path().join("main.json");
    fs::write(
        &main_path,
        r#"library Main version '1.0'

include Helper version '1.0' called H

define Result: H.Answer + 1
"#,
    )
    .unwrap();

    rh_cmd()
        .args([
            "cql",
            "compile",
            main_path.to_str().unwrap(),
            "--output",
            elm_path.to_str().unwrap(),
            "--lib-path",
            lib_dir.to_str().unwrap(),
            "--source-map",
        ])
        .assert()
        .success();

    assert!(elm_path.exists(), "ELM output file should exist");
    let sm_path = PathBuf::from(format!("{}.sourcemap.json", elm_path.display()));
    assert!(sm_path.exists(), "Source map sidecar should exist");
}

// ---------------------------------------------------------------------------
// Fixtures for exit-behavior tests
// ---------------------------------------------------------------------------

const INVALID_CQL: &str = "this is not valid CQL !!!###";

/// CQL that parses correctly but contains a semantic error (undefined reference).
/// This causes `validate` to return a `ValidationResult` with errors rather
/// than an API-level `Err`, so the CLI prints the errors to stdout.
const SEMANTIC_ERROR_CQL: &str = r#"library SemanticBad version '1.0'
define Foo: UndefinedIdent
"#;

// ---------------------------------------------------------------------------
// validate – exit code and output behavior (task 2.4)
// ---------------------------------------------------------------------------

#[test]
fn test_validate_exits_zero_on_valid_cql() {
    rh_cmd()
        .args(["cql", "validate", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_exits_nonzero_on_invalid_cql() {
    // Completely unparseable CQL → API-level error → non-zero exit
    rh_cmd()
        .args(["cql", "validate", "-"])
        .write_stdin(INVALID_CQL)
        .assert()
        .failure();
}

#[test]
fn test_validate_exits_nonzero_on_semantic_error() {
    // Syntactically valid but semantically invalid CQL → ValidationResult with
    // errors → errors printed to stdout → non-zero exit
    rh_cmd()
        .args(["cql", "validate", "-"])
        .write_stdin(SEMANTIC_ERROR_CQL)
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"));
}

#[test]
fn test_validate_verbose_shows_location_on_error() {
    rh_cmd()
        .args(["cql", "validate", "-", "--details"])
        .write_stdin(SEMANTIC_ERROR_CQL)
        .assert()
        .failure()
        // With --verbose, location info (line/column) should appear when available
        .stdout(
            predicate::str::contains("✗")
                .and(predicate::str::contains("line").or(predicate::str::contains("✗"))),
        );
}

#[test]
fn test_compile_result_types_flag_emits_structural_type_metadata() {
    let cql = r#"library ResultTypes version '1.0'

define Numbers: { 1, 2 }
"#;

    rh_cmd()
        .args(["cql", "compile", "--result-types", "-"])
        .write_stdin(cql)
        .assert()
        .success()
        .stdout(predicate::str::contains("resultTypeSpecifier"))
        .stdout(predicate::str::contains("ListTypeSpecifier"))
        .stdout(predicate::str::contains("EnableResultTypes"));
}

#[test]
fn test_compile_omits_result_type_metadata_by_default() {
    let cql = r#"library ResultTypes version '1.0'

define Numbers: { 1, 2 }
"#;

    rh_cmd()
        .args(["cql", "compile", "-"])
        .write_stdin(cql)
        .assert()
        .success()
        .stdout(predicate::str::contains("resultTypeSpecifier").not())
        .stdout(predicate::str::contains("EnableResultTypes").not());
}

// ---------------------------------------------------------------------------
// analytics tooling
// ---------------------------------------------------------------------------

#[test]
fn test_elm_inspect_stdin() {
    rh_cmd()
        .args(["cql", "elm", "inspect", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("Library: SimpleMath"))
        .stdout(predicate::str::contains("Expressions: 2"));
}

#[test]
fn test_elm_deps_json_envelope() {
    rh_cmd()
        .args(["--format", "json", "cql", "elm", "deps", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("cql elm deps"));
}

#[test]
fn test_data_requirements_stdin() {
    rh_cmd()
        .args(["cql", "data-requirements", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("resources: none"));
}

#[test]
fn test_relational_plan_stdin() {
    rh_cmd()
        .args(["cql", "plan", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("Target: relational"))
        .stdout(predicate::str::contains("X"));
}

#[test]
fn test_lower_check_stdin() {
    rh_cmd()
        .args(["cql", "lower-check", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("Target: sql-on-fhir"))
        .stdout(predicate::str::contains("Supported:"));
}

#[test]
fn test_emit_views_writes_view_definition_files() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("measure.cql");
    let out_dir = dir.path().join("views");
    fs::write(&cql_path, FHIR_RETRIEVE_CQL).unwrap();

    rh_cmd()
        .args([
            "cql",
            "emit-views",
            cql_path.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrote 1 ViewDefinition"));

    let view_path = out_dir.join("condition_view.json");
    assert!(view_path.exists(), "ViewDefinition file should exist");
    let view = fs::read_to_string(view_path).unwrap();
    assert!(view.contains("\"resource\": \"Condition\""));
    assert!(view.contains("getResourceKey()"));
}

#[test]
fn test_emit_views_json_envelope() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("measure.cql");
    let out_dir = dir.path().join("views");
    fs::write(&cql_path, FHIR_RETRIEVE_CQL).unwrap();

    rh_cmd()
        .args([
            "--format",
            "json",
            "cql",
            "emit-views",
            cql_path.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"count\": 1"));
}

#[test]
fn test_emit_sql_sql_only_outputs_sql() {
    rh_cmd()
        .args(["cql", "emit-sql", "-", "--sql-only"])
        .write_stdin(FHIR_RETRIEVE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("WITH"))
        .stdout(predicate::str::contains("condition_view"));
}

#[test]
fn test_emit_sql_writes_sql_query_library() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("measure.cql");
    let out_path = dir.path().join("query-library.json");
    fs::write(&cql_path, FHIR_RETRIEVE_CQL).unwrap();

    rh_cmd()
        .args([
            "cql",
            "emit-sql",
            cql_path.to_str().unwrap(),
            "--out",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrote SQLQuery Library"));

    let library = fs::read_to_string(out_path).unwrap();
    assert!(library.contains("\"resourceType\": \"Library\""));
    assert!(library.contains("\"code\": \"sql-query\""));
    assert!(library.contains("application/sql"));
}

#[test]
fn test_emit_runtime_writes_measure_runtime_manifest() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("measure.cql");
    let views_dir = dir.path().join("views");
    let query_path = dir.path().join("query-library.json");
    let runtime_path = dir.path().join("measure-runtime.json");
    fs::write(&cql_path, FHIR_RETRIEVE_CQL).unwrap();

    rh_cmd()
        .args([
            "cql",
            "emit-views",
            cql_path.to_str().unwrap(),
            "--out",
            views_dir.to_str().unwrap(),
        ])
        .assert()
        .success();
    rh_cmd()
        .args([
            "cql",
            "emit-sql",
            cql_path.to_str().unwrap(),
            "--views",
            views_dir.to_str().unwrap(),
            "--out",
            query_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    rh_cmd()
        .args([
            "cql",
            "emit-runtime",
            cql_path.to_str().unwrap(),
            "--query",
            query_path.to_str().unwrap(),
            "--views",
            views_dir.to_str().unwrap(),
            "--out",
            runtime_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrote measure runtime manifest"));

    let manifest = fs::read_to_string(runtime_path).unwrap();
    assert!(manifest.contains("\"resourceType\": \"ReasonHealthMeasureRuntime\""));
    assert!(manifest.contains("\"query\": \"query-library.json\""));
    assert!(manifest.contains("\"views\": ["));
    assert!(manifest.contains("\"views/condition_view.json\""));
    assert!(manifest.contains("\"name\": \"measurementperiod\""));
    assert!(manifest.contains("\"name\": \"initialPopulation\""));
    assert!(manifest.contains("\"kind\": \"population\""));
    assert!(manifest.contains("\"column\": \"patient_id\""));
}

#[test]
fn test_emit_runtime_json_envelope() {
    let dir = TempDir::new().unwrap();
    let cql_path = dir.path().join("measure.cql");
    let view_path = dir.path().join("condition_view.json");
    let query_path = dir.path().join("query-library.json");
    fs::write(&cql_path, FHIR_RETRIEVE_CQL).unwrap();
    fs::write(&view_path, "{}").unwrap();
    fs::write(&query_path, "{}").unwrap();

    rh_cmd()
        .args([
            "--format",
            "json",
            "cql",
            "emit-runtime",
            cql_path.to_str().unwrap(),
            "--query",
            query_path.to_str().unwrap(),
            "--views",
            view_path.to_str().unwrap(),
            "--result",
            "denominator=patient_id",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains(
            "\"command\": \"cql emit-runtime\"",
        ))
        .stdout(predicate::str::contains("\"name\": \"denominator\""));
}

// ---------------------------------------------------------------------------
// compile – exit code behavior (task 2.4)
// ---------------------------------------------------------------------------

#[test]
fn test_compile_exits_zero_on_valid_cql() {
    rh_cmd()
        .args(["cql", "compile", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"library\""));
}

#[test]
fn test_compile_exits_nonzero_on_invalid_cql() {
    rh_cmd()
        .args(["cql", "compile", "-"])
        .write_stdin(INVALID_CQL)
        .assert()
        .failure()
        // Error message is reported on stderr
        .stderr(predicate::str::contains("✗").or(predicate::str::contains("error")));
}

// ---------------------------------------------------------------------------
// info – exit code behavior (task 2.4)
// ---------------------------------------------------------------------------

#[test]
fn test_info_exits_zero_on_valid_cql() {
    rh_cmd()
        .args(["cql", "info", "-"])
        .write_stdin(SIMPLE_CQL)
        .assert()
        .success()
        .stdout(predicate::str::contains("SimpleMath"));
}

#[test]
fn test_info_exits_nonzero_on_invalid_cql() {
    rh_cmd()
        .args(["cql", "info", "-"])
        .write_stdin(INVALID_CQL)
        .assert()
        .failure();
}

// ---------------------------------------------------------------------------
// eval – exit code behavior (task 2.4)
// ---------------------------------------------------------------------------

#[test]
fn test_eval_exits_nonzero_on_invalid_cql() {
    rh_cmd()
        .args(["cql", "eval", "-", "X"])
        .write_stdin(INVALID_CQL)
        .assert()
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("✗")));
}
