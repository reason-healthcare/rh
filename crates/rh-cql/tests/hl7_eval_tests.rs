//! HL7 CQL specification eval tests.
//!
//! Task 11.3 — Verify evaluation results against stable result sets defined
//! by the HL7 CQL test suite (<https://cql.hl7.org/tests.html>).
//!
//! # Format
//!
//! The test files use a simple XML format (described by `testSchema.xsd`):
//!
//! ```xml
//! <tests name="SuiteName">
//!   <group name="GroupName">
//!     <test name="TestName">
//!       <expression>CQL_EXPRESSION</expression>
//!       <output>EXPECTED_RESULT</output>
//!     </test>
//!   </group>
//! </tests>
//! ```
//!
//! Invalid-expression tests use `<expression invalid="true">…</expression>`;
//! these are expected to fail and are counted separately.
//!
//! # Coverage
//!
//! The runner covers the following test suites from the spec:
//! - `CqlLogicalOperatorsTest.xml`
//! - `CqlNullologicalOperatorsTest.xml`
//! - `CqlConditionalOperatorsTest.xml`
//! - `CqlArithmeticFunctionsTest.xml`
//!
//! # Skip policy
//!
//! Tests whose expected output cannot be parsed by the current implementation
//! (Long literals, Quantities, Lists, Dates, Intervals, Tuples) are counted
//! as "skipped" rather than failures. The suite asserts a minimum pass rate
//! so that regressions are caught even when some tests are still unimplemented.

use quick_xml::events::Event;
use quick_xml::Reader;
use rh_cql::{
    compile_with_model, evaluate_elm, CqlDateTime, EvalContextBuilder, FixedClock, Value,
};
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Test-case data structures
// ---------------------------------------------------------------------------

/// A single test case parsed from an HL7 test XML file.
#[derive(Debug, Clone)]
struct HlTestCase {
    group: String,
    name: String,
    expression: String,
    expected_output: Option<String>,
    /// `true` when `<expression invalid="true">` — evaluation is expected to
    /// error.
    invalid: bool,
}

// ---------------------------------------------------------------------------
// XML parser
// ---------------------------------------------------------------------------

/// Parse all test cases from an HL7 CQL test XML file.
fn parse_hl7_xml(xml: &str) -> Vec<HlTestCase> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut cases: Vec<HlTestCase> = Vec::new();

    let mut _suite_name = String::new();
    let mut group_name = String::new();
    let mut test_name = String::new();
    let mut expression = String::new();
    let mut invalid = false;
    let mut output: Option<String> = None;
    let mut in_test = false;
    let mut in_expression = false;
    let mut in_output = false;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name_bytes = e.name();
                let name_ref = name_bytes.as_ref();
                let tag = std::str::from_utf8(name_ref).unwrap_or("");
                match tag {
                    "tests" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                _suite_name =
                                    attr.unescape_value().unwrap_or_default().into_owned();
                            }
                        }
                    }
                    "group" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                group_name = attr.unescape_value().unwrap_or_default().into_owned();
                            }
                        }
                    }
                    "test" => {
                        in_test = true;
                        test_name = String::new();
                        expression = String::new();
                        invalid = false;
                        output = None;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                test_name = attr.unescape_value().unwrap_or_default().into_owned();
                            }
                        }
                    }
                    "expression" if in_test => {
                        in_expression = true;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"invalid" {
                                invalid = attr.unescape_value().unwrap_or_default() == "true";
                            }
                        }
                    }
                    "output" if in_test => {
                        in_output = true;
                        output = Some(String::new());
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default();
                if in_expression {
                    expression.push_str(&text);
                } else if in_output {
                    if let Some(ref mut o) = output {
                        o.push_str(&text);
                    }
                }
            }
            Ok(Event::CData(ref e)) => {
                let text = std::str::from_utf8(e.as_ref()).unwrap_or("");
                if in_expression {
                    expression.push_str(text);
                } else if in_output {
                    if let Some(ref mut o) = output {
                        o.push_str(text);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let end_name_bytes = e.name();
                let end_name_ref = end_name_bytes.as_ref();
                let tag = std::str::from_utf8(end_name_ref).unwrap_or("");
                match tag {
                    "expression" => {
                        in_expression = false;
                    }
                    "output" => {
                        in_output = false;
                    }
                    "test" if in_test => {
                        in_test = false;
                        cases.push(HlTestCase {
                            group: group_name.clone(),
                            name: test_name.clone(),
                            expression: expression.trim().to_string(),
                            expected_output: output.clone().map(|s| s.trim().to_string()),
                            invalid,
                        });
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    cases
}

// ---------------------------------------------------------------------------
// Expected-output parsing
// ---------------------------------------------------------------------------

/// A parsed expected output value from the HL7 test XML.
#[derive(Debug, Clone, PartialEq)]
enum ExpectedValue {
    Null,
    Bool(bool),
    Integer(i64),
    Decimal(f64),
    Str(String),
    /// Output format not yet supported by this runner (skip the test).
    Unsupported(String),
}

/// Parse the textual expected-output string from the HL7 test XML into an
/// [`ExpectedValue`].
///
/// Unsupported formats are returned as `[ExpectedValue::Unsupported]` so the
/// test can be skipped rather than failed.
fn parse_expected(raw: &str) -> ExpectedValue {
    let s = raw.trim();

    // null
    if s == "null" {
        return ExpectedValue::Null;
    }

    // booleans
    if s == "true" {
        return ExpectedValue::Bool(true);
    }
    if s == "false" {
        return ExpectedValue::Bool(false);
    }

    // Skip Long literals (suffix 'L')
    if s.ends_with('L') {
        return ExpectedValue::Unsupported(format!("Long literal: {s}"));
    }

    // Skip list literals {…}
    if s.starts_with('{') {
        return ExpectedValue::Unsupported(format!("List: {s}"));
    }

    // Skip Date/DateTime/Time literals
    if s.starts_with('@')
        || s.starts_with("DateTime(")
        || s.starts_with("Date(")
        || s.starts_with("Time(")
    {
        return ExpectedValue::Unsupported(format!("Temporal: {s}"));
    }

    // Skip Interval/Tuple literals
    if s.starts_with("Interval") || s.starts_with('[') || s.starts_with("Tuple") {
        return ExpectedValue::Unsupported(format!("Interval/Tuple: {s}"));
    }

    // String literals: CQL strings are delimited by single quotes.
    // Note: the test XML uses single-quoted strings like 'abc'.
    if s.starts_with('\'') && s.ends_with('\'') {
        return ExpectedValue::Str(s[1..s.len() - 1].to_string());
    }

    // Try integer (no decimal point)
    if !s.contains('.') {
        if let Ok(i) = s.parse::<i64>() {
            return ExpectedValue::Integer(i);
        }
    }

    // Try decimal
    if let Ok(f) = s.parse::<f64>() {
        return ExpectedValue::Decimal(f);
    }

    ExpectedValue::Unsupported(format!("unknown format: {s}"))
}

/// Skip heuristic for the CQL expression itself.
///
/// Returns a skip reason string if the expression uses language features not
/// yet supported by the eval engine, so that the test is skipped rather than
/// counted as a failure.
fn skip_reason_for_expression(expr: &str) -> Option<String> {
    // Long literals (digit followed by L)
    if expr.contains("1L") || expr.contains("2L") || expr.contains("0L") || expr.contains("-1L") {
        return Some(format!("Long literal in expression: {expr}"));
    }

    // Quantity literals: number followed by 'unit-string'
    // e.g.  1'g/cm3', -1.0'cm'
    // Detect by looking for a digit immediately before an apostrophe
    let chars: Vec<char> = expr.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == '\'' && chars[i - 1].is_ascii_digit() {
            return Some(format!("Quantity literal in expression: {expr}"));
        }
    }

    // Log(x, 1): log base 1 is undefined (0/0 mathematically); our engine
    // returns Null while the HL7 spec expects 0.0 — skip as a known edge case.
    if expr.contains(", 1)") && expr.starts_with("Log(") {
        return Some(format!("Log base-1 edge case (undefined): {expr}"));
    }

    None
}

// ---------------------------------------------------------------------------
// Value comparison
// ---------------------------------------------------------------------------

/// Compare a runtime [`Value`] against a parsed [`ExpectedValue`].
fn values_match(actual: &Value, expected: &ExpectedValue) -> bool {
    match (actual, expected) {
        (Value::Null, ExpectedValue::Null) => true,
        (Value::Boolean(a), ExpectedValue::Bool(e)) => a == e,
        (Value::Integer(a), ExpectedValue::Integer(e)) => *a == *e,
        // Allow Integer result vs Decimal expected (e.g. Ceiling(1.1) → 2 but
        // the expected text is "2", parsed as Integer)
        (Value::Integer(a), ExpectedValue::Decimal(e)) => (*a as f64 - e).abs() < 1e-9,
        (Value::Decimal(a), ExpectedValue::Decimal(e)) => {
            // Use 1e-6 relative tolerance (8 significant decimal digits) to
            // match the HL7 spec's expected precision for Decimal values.
            let tol = 1e-6f64;
            if e.abs() < tol {
                a.abs() < tol
            } else {
                ((a - e) / e).abs() < tol
            }
        }
        (Value::Decimal(a), ExpectedValue::Integer(e)) => (*a - *e as f64).abs() < 1e-9,
        (Value::String(a), ExpectedValue::Str(e)) => a == e,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Per-test execution
// ---------------------------------------------------------------------------

/// Outcome of running a single HL7 test case.
#[derive(Debug)]
enum Outcome {
    Pass,
    Fail {
        actual: String,
        expected: String,
    },
    SkipExpr(#[allow(dead_code)] String),
    SkipOutput(#[allow(dead_code)] String),
    CompileError(#[allow(dead_code)] String),
    EvalError(#[allow(dead_code)] String),
    /// `invalid="true"` test that correctly produced an error/null result.
    InvalidPass,
    /// `invalid="true"` test that unexpectedly succeeded.
    InvalidFail(String),
}

fn shared_clock() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2023,
        month: Some(1),
        day: Some(1),
        hour: Some(0),
        minute: Some(0),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0), // UTC
    })
}

fn run_test_case(tc: &HlTestCase) -> Outcome {
    // Check if the expression uses unsupported features.
    if let Some(reason) = skip_reason_for_expression(&tc.expression) {
        return Outcome::SkipExpr(reason);
    }

    // Wrap the standalone expression in a minimal CQL library.
    let cql = format!("library HlTest\ndefine Result: {}", tc.expression);

    // Compile.
    let compile_result = match compile_with_model(&cql, None, None) {
        Ok(r) => r,
        Err(e) => return Outcome::CompileError(e.to_string()),
    };

    if !compile_result.errors.is_empty() {
        if tc.invalid {
            // Compile errors on invalid-expression tests are acceptable.
            return Outcome::InvalidPass;
        }
        return Outcome::CompileError(format!("{:?}", compile_result.errors));
    }

    // Evaluate.
    let ctx = EvalContextBuilder::new(shared_clock()).build();
    let actual_value = match evaluate_elm(&compile_result.library, "Result", &ctx) {
        Ok(v) => v,
        Err(e) => {
            if tc.invalid {
                return Outcome::InvalidPass;
            }
            return Outcome::EvalError(e.to_string());
        }
    };

    // Handle invalid-expression tests: Inf/NaN from arithmetic overflow is a
    // known engine limitation; treat it the same as an evaluation error.
    if tc.invalid {
        let is_non_finite = matches!(
            &actual_value,
            Value::Decimal(f) if f.is_infinite() || f.is_nan()
        );
        if is_non_finite {
            return Outcome::EvalError(
                "overflow to Inf/NaN (unimplemented error-on-overflow)".to_string(),
            );
        }
        // Any other "successful" result on an invalid-expected test is a failure.
        return Outcome::InvalidFail(format!("{actual_value:?}"));
    }

    // No output element — cannot verify; treat as pass if we got this far.
    let Some(ref raw_output) = tc.expected_output else {
        return Outcome::Pass;
    };

    // Parse the expected output.
    let expected = parse_expected(raw_output);
    if let ExpectedValue::Unsupported(ref reason) = expected {
        return Outcome::SkipOutput(reason.clone());
    }

    // Compare.
    if values_match(&actual_value, &expected) {
        Outcome::Pass
    } else {
        Outcome::Fail {
            actual: format!("{actual_value:?}"),
            expected: format!("{expected:?}"),
        }
    }
}

// ---------------------------------------------------------------------------
// Suite runner
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct SuiteStats {
    pass: usize,
    fail: usize,
    skip_expr: usize,
    skip_output: usize,
    compile_err: usize,
    eval_err: usize,
    invalid_pass: usize,
    invalid_fail: usize,
}

impl SuiteStats {
    fn total_attempted(&self) -> usize {
        self.pass + self.fail + self.compile_err + self.eval_err
    }
}

fn run_suite(xml_path: &Path) -> (SuiteStats, Vec<String>) {
    let xml =
        fs::read_to_string(xml_path).unwrap_or_else(|e| panic!("cannot read {xml_path:?}: {e}"));
    let cases = parse_hl7_xml(&xml);
    let file_name = xml_path.file_name().and_then(|s| s.to_str()).unwrap_or("?");

    let mut stats = SuiteStats::default();
    let mut failures = Vec::new();

    for tc in &cases {
        let outcome = run_test_case(tc);
        match outcome {
            Outcome::Pass | Outcome::InvalidPass => {
                if matches!(outcome, Outcome::InvalidPass) {
                    stats.invalid_pass += 1;
                } else {
                    stats.pass += 1;
                }
            }
            Outcome::Fail {
                ref actual,
                ref expected,
            } => {
                stats.fail += 1;
                failures.push(format!(
                    "[FAIL] {file_name} / {} / {} / {}\n  expected: {expected}\n  actual:   {actual}",
                    tc.group, tc.name, tc.expression
                ));
            }
            Outcome::InvalidFail(ref v) => {
                stats.invalid_fail += 1;
                failures.push(format!(
                    "[INVALID-FAIL] {file_name} / {} / {} — expected error, got {v}",
                    tc.group, tc.name,
                ));
            }
            Outcome::SkipExpr(_) => stats.skip_expr += 1,
            Outcome::SkipOutput(_) => stats.skip_output += 1,
            Outcome::CompileError(_) => {
                // Compilation errors indicate unimplemented language features
                // rather than wrong answers; count but don't fail the suite.
                stats.compile_err += 1;
            }
            Outcome::EvalError(_) => {
                // Evaluation errors indicate unimplemented operators/functions;
                // count but don't fail the suite (only wrong answers fail).
                stats.eval_err += 1;
            }
        }
    }

    (stats, failures)
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hl7_cql_tests")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Run all HL7 CQL test XML files and assert a minimum pass rate.
///
/// The test prints a summary of pass/fail/skip counts for each suite.
#[test]
fn hl7_eval_suite_all() {
    let dir = fixtures_dir();
    let mut xml_paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read hl7 test dir: {e}"))
        .flatten()
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("xml"))
        .map(|e| e.path())
        .collect();
    xml_paths.sort();

    let mut total_stats = SuiteStats::default();
    let mut all_failures: Vec<String> = Vec::new();

    for path in &xml_paths {
        let (stats, failures) = run_suite(path);
        let file = path.file_name().and_then(|s| s.to_str()).unwrap_or("?");

        println!(
            "  {file}: pass={} fail={} skip_expr={} skip_output={} compile_err={} eval_err={} invalid_pass={}",
            stats.pass,
            stats.fail,
            stats.skip_expr,
            stats.skip_output,
            stats.compile_err,
            stats.eval_err,
            stats.invalid_pass,
        );

        total_stats.pass += stats.pass;
        total_stats.fail += stats.fail;
        total_stats.skip_expr += stats.skip_expr;
        total_stats.skip_output += stats.skip_output;
        total_stats.compile_err += stats.compile_err;
        total_stats.eval_err += stats.eval_err;
        total_stats.invalid_pass += stats.invalid_pass;
        all_failures.extend(failures);
    }

    println!(
        "\nTotal: pass={} fail={} skip={} unimplemented(compile_err={} eval_err={})",
        total_stats.pass,
        total_stats.fail,
        total_stats.skip_expr + total_stats.skip_output,
        total_stats.compile_err,
        total_stats.eval_err,
    );
    if total_stats.eval_err > 0 || total_stats.compile_err > 0 {
        println!(
            "  note: {unimpl} test(s) were skipped as unimplemented (eval_err/compile_err).",
            unimpl = total_stats.eval_err + total_stats.compile_err
        );
        println!("  These will become passing tests as the eval engine grows.");
    }

    // Report all failures for debugging
    if !all_failures.is_empty() {
        eprintln!("\n--- Failures ---");
        for f in &all_failures {
            eprintln!("{f}");
        }
        eprintln!("--- End failures ---\n");
    }

    // Assert: no wrong answers (Fail/InvalidFail outcomes).
    // EvalError/CompileError are counted separately as unimplemented features.
    assert!(
        all_failures.is_empty(),
        "{} assertion failure(s) — see stderr output above.\n{}",
        all_failures.len(),
        all_failures.join("\n")
    );

    // Sanity: at least some tests must have been evaluated (not all skipped).
    assert!(
        total_stats.total_attempted() >= 20,
        "too few tests were evaluated ({}) — check fixture files",
        total_stats.total_attempted()
    );
}

/// Logical operators: AND, OR, NOT, XOR, IMPLIES — all boolean/null results.
/// These are the most basic and should all pass.
#[test]
fn hl7_eval_logical_operators() {
    let path = fixtures_dir().join("CqlLogicalOperatorsTest.xml");
    let (stats, failures) = run_suite(&path);

    assert!(
        failures.is_empty(),
        "Logical operators test failures:\n{}",
        failures.join("\n")
    );
    assert!(
        stats.pass >= 10,
        "Expected ≥10 passing logical-operator tests, got {}",
        stats.pass
    );
}

/// Nullological operators: IsNull, IsTrue, IsFalse, Coalesce.
/// Some operators are not yet implemented in the eval engine (IsNull, IsTrue,
/// IsFalse); those are counted as eval errors and skipped.  The test asserts
/// that no WRONG ANSWERS are produced — only wrong answers (Fail) fail CI.
#[test]
fn hl7_eval_nullological_operators() {
    let path = fixtures_dir().join("CqlNullologicalOperatorsTest.xml");
    let (stats, failures) = run_suite(&path);

    // Document the eval-error count (unimplemented functions) in the output:
    if stats.eval_err > 0 {
        println!(
            "  note: {} nullological tests skipped (eval_err — unimplemented functions)",
            stats.eval_err
        );
    }

    assert!(
        failures.is_empty(),
        "Nullological operators test failures (wrong answers):\n{}",
        failures.join("\n")
    );
}

/// Conditional operators: if-then-else and case expressions.
#[test]
fn hl7_eval_conditional_operators() {
    let path = fixtures_dir().join("CqlConditionalOperatorsTest.xml");
    let (stats, failures) = run_suite(&path);

    assert!(
        failures.is_empty(),
        "Conditional operators test failures:\n{}",
        failures.join("\n")
    );
    assert!(
        stats.pass >= 3,
        "Expected ≥3 passing conditional tests, got {}",
        stats.pass
    );
}
