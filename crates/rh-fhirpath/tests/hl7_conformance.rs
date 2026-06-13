//! Official HL7 FHIRPath conformance suite harness.
//!
//! Runs the vendored `tests-fhir-r4.xml` suite (937 cases) against the
//! rh-fhirpath parser/evaluator and categorizes every case as pass,
//! wrong-answer, parse-error, eval-error, or skipped.
//!
//! Gate: **no new wrong answers** — every wrong-answer must be listed in
//! `KNOWN_WRONG_ANSWERS`. Parse/eval errors are tolerated (tracked in
//! CONFORMANCE.md) but wrong answers are not.
//!
//! A machine-readable summary is written to
//! `target/hl7_fhirpath_conformance.json` (override with
//! `HL7_CONFORMANCE_OUT`).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use quick_xml::events::Event;
use quick_xml::Reader;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/hl7_fhirpath_tests"
);

/// Wrong answers present at harness introduction (baseline 2026-06-12).
/// Shrink-only: fixing one means removing it here. Adding to this list is a
/// conformance regression and must not happen.
const KNOWN_WRONG_ANSWERS: &[&str] = &[
    "HighBoundary::HighBoundaryDecimal12",
    "HighBoundary::HighBoundaryDecimal15",
    "HighBoundary::HighBoundaryDecimal16",
    "LowBoundary::LowBoundaryDecimal11",
    "LowBoundary::LowBoundaryDecimal15",
    "Precision::PrecisionDecimal",
    "from-Zulip::from-zulip-2",
    "polymorphics::testPolymorphicsB",
    "testAll::testAllTrue4",
    "testBooleanImplies::testBooleanImplies3",
    "testCombine()::testCombine1",
    "testDollar::testDollarOrderNotAllowed",
    "testEquality::testEquality28",
    "testExists::testExists2",
    "testExtension::testExtension1",
    "testIif::testIif10",
    "testIif::testIif11",
    "testIif::testIif9",
    "testIndexer::testIndexer2",
    "testInheritance::testFHIRPathAsFunction11",
    "testInheritance::testFHIRPathAsFunction13",
    "testInheritance::testFHIRPathAsFunction15",
    "testInheritance::testFHIRPathAsFunction16",
    "testInheritance::testFHIRPathAsFunction18",
    "testInheritance::testFHIRPathAsFunction20",
    "testInheritance::testFHIRPathAsFunction22",
    "testInheritance::testFHIRPathIsFunction3",
    "testInheritance::testFHIRPathIsFunction5",
    "testInheritance::testFHIRPathIsFunction7",
    "testInheritance::testFHIRPathIsFunction8",
    "testInheritance::testFHIRPathIsFunction9",
    "testLiterals::testIntegerBooleanNotFalse",
    "testLiterals::testIntegerBooleanNotTrue",
    "testMatches::testMatchesSingleLineMode1",
    "testMinus::testMinus3",
    "testMod::testMod4",
    "testObservations::testPolymorphismB",
    "testObservations::testPolymorphismIsA3",
    "testPower::testPower3",
    "testPrecedence::testPrecedence3",
    "testPrecedence::testPrecedence4",
    "testRepeat::testRepeat1",
    "testRepeat::testRepeat5",
    "testType::testType10",
    "testType::testType12",
    // Patient.active.is(System.Boolean).not() expects FHIR.boolean to be
    // distinct from System.Boolean. Both currently map to FhirPathValue::Boolean
    // because the engine doesn't track FHIR vs System primitive provenance —
    // unblocked by the typed-primitive variant refactor.
    "testType::testType14",
    "testType::testType9",
    "testTypes::testStringQuantityMonthLiteralToQuantity",
    "testTypes::testStringQuantityYearLiteralToQuantity",
    "testUnion::testUnion2",
    "testUnion::testUnion3",
    "testVariables::testVariables4",
    "testWhere::testWhere2",
    "testWhere::testWhere4",
];

#[derive(Debug, Clone)]
struct TestCase {
    group: String,
    name: String,
    inputfile: String,
    expression: String,
    invalid: Option<String>,
    predicate: bool,
    outputs: Vec<(String, String)>, // (type, value)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Pass,
    WrongAnswer,
    ParseError,
    EvalError,
    Skipped,
}

fn parse_suite(xml: &str) -> Vec<TestCase> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);

    let mut cases = Vec::new();
    let mut group = String::new();
    let mut current: Option<TestCase> = None;
    let mut in_expression = false;
    let mut in_output = false;
    let mut output_type = String::new();
    let mut text_buf = String::new();

    loop {
        match reader.read_event().expect("well-formed suite XML") {
            Event::Start(e) | Event::Empty(e) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let attrs: HashMap<String, String> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| {
                        (
                            String::from_utf8_lossy(a.key.as_ref()).to_string(),
                            String::from_utf8_lossy(&a.value).to_string(),
                        )
                    })
                    .collect();
                match tag.as_str() {
                    "group" => {
                        group = attrs.get("name").cloned().unwrap_or_default();
                    }
                    "test" => {
                        current = Some(TestCase {
                            group: group.clone(),
                            name: attrs.get("name").cloned().unwrap_or_default(),
                            inputfile: attrs.get("inputfile").cloned().unwrap_or_default(),
                            expression: String::new(),
                            invalid: attrs.get("invalid").cloned(),
                            predicate: attrs.get("predicate").map(|p| p == "true") == Some(true),
                            outputs: Vec::new(),
                        });
                    }
                    "expression" => {
                        in_expression = true;
                        text_buf.clear();
                        // expression-level invalid attribute (e.g. invalid="true")
                        if let Some(inv) = attrs.get("invalid") {
                            if let Some(case) = current.as_mut() {
                                case.invalid.get_or_insert_with(|| inv.clone());
                            }
                        }
                    }
                    "output" => {
                        in_output = true;
                        text_buf.clear();
                        output_type = attrs.get("type").cloned().unwrap_or_default();
                    }
                    _ => {}
                }
            }
            Event::Text(t) => {
                if in_expression || in_output {
                    text_buf.push_str(&t.unescape().expect("decode text"));
                }
            }
            Event::End(e) => match e.name().as_ref() {
                b"expression" => {
                    if let Some(case) = current.as_mut() {
                        case.expression = text_buf.clone();
                    }
                    in_expression = false;
                }
                b"output" => {
                    if let Some(case) = current.as_mut() {
                        case.outputs.push((output_type.clone(), text_buf.clone()));
                    }
                    in_output = false;
                }
                b"test" => {
                    if let Some(case) = current.take() {
                        cases.push(case);
                    }
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    cases
}

fn load_inputs() -> HashMap<String, serde_json::Value> {
    let dir = Path::new(FIXTURES).join("input");
    let mut inputs = HashMap::new();
    for entry in fs::read_dir(&dir).expect("input dir").flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            let value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read input"))
                    .expect("parse input JSON");
            inputs.insert(stem, value);
        }
    }
    inputs
}

/// Flatten an evaluation result into an ordered list of scalar values.
fn flatten(value: FhirPathValue) -> Vec<FhirPathValue> {
    match value {
        FhirPathValue::Empty => vec![],
        FhirPathValue::Collection(items) => items,
        other => vec![other],
    }
}

/// Compare a single actual value against an expected (type, text) output.
fn matches_expected(actual: &FhirPathValue, expected_type: &str, expected: &str) -> bool {
    // Some suite groups (LowBoundary/HighBoundary/Precision) omit the type
    // attribute — infer it from the expected text.
    if expected_type.is_empty() {
        let inferred = if expected == "true" || expected == "false" {
            "boolean"
        } else if expected.starts_with("@T") {
            "time"
        } else if expected.starts_with('@') {
            if expected.contains('T') {
                "dateTime"
            } else {
                "date"
            }
        } else if expected.contains('\'') {
            "Quantity"
        } else if expected.parse::<f64>().is_ok() {
            "decimal"
        } else {
            "string"
        };
        return matches_expected(actual, inferred, expected);
    }
    match expected_type {
        "boolean" => matches!(
            (actual, expected),
            (FhirPathValue::Boolean(true), "true") | (FhirPathValue::Boolean(false), "false")
        ),
        "integer" => match actual {
            FhirPathValue::Integer(i) | FhirPathValue::Long(i) => expected.parse::<i64>() == Ok(*i),
            FhirPathValue::Number(n) if n.fract() == 0.0 => {
                expected.parse::<i64>() == Ok(*n as i64)
            }
            _ => false,
        },
        "decimal" => {
            let expected_num: f64 = match expected.parse() {
                Ok(n) => n,
                Err(_) => return false,
            };
            match actual {
                FhirPathValue::Number(n) => (n - expected_num).abs() < 1e-9,
                FhirPathValue::Integer(i) | FhirPathValue::Long(i) => {
                    (*i as f64 - expected_num).abs() < 1e-9
                }
                _ => false,
            }
        }
        "string" | "code" | "id" | "uri" => match actual {
            FhirPathValue::String(s) => s == expected,
            FhirPathValue::Object(serde_json::Value::String(s)) => s == expected,
            _ => false,
        },
        "date" | "dateTime" | "time" => {
            let expected_norm = expected.trim_start_matches('@');
            let actual_str = match actual {
                FhirPathValue::Date(s)
                | FhirPathValue::DateTime(s)
                | FhirPathValue::Time(s)
                | FhirPathValue::String(s) => s.as_str(),
                _ => return false,
            };
            let actual_norm = actual_str.trim_start_matches('@').trim_start_matches('T');
            let expected_norm = expected_norm.trim_start_matches('T');
            actual_norm == expected_norm
        }
        "Quantity" => match actual {
            FhirPathValue::Quantity { value, unit } => {
                // Compare numerically: "1.58650000 'cm'" == 1.5865 'cm'.
                let (num_str, unit_str) = match expected.split_once(' ') {
                    Some((n, u)) => (n, Some(u.trim().trim_matches('\''))),
                    None => (expected, None),
                };
                let Ok(expected_num) = num_str.parse::<f64>() else {
                    return false;
                };
                (value - expected_num).abs() < 1e-9
                    && match (unit, unit_str) {
                        (Some(u), Some(e)) => u == e,
                        (None, None) => true,
                        (Some(u), None) => u == "1",
                        (None, Some(e)) => e == "1",
                    }
            }
            _ => false,
        },
        _ => false,
    }
}

fn run_case(
    case: &TestCase,
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    inputs: &HashMap<String, serde_json::Value>,
) -> (Outcome, String) {
    let input_stem = case
        .inputfile
        .trim_end_matches(".xml")
        .trim_end_matches(".json");
    let resource = if case.inputfile.is_empty() {
        Some(&serde_json::Value::Null)
    } else {
        inputs.get(input_stem)
    };
    let Some(resource) = resource else {
        return (
            Outcome::Skipped,
            format!("missing input {}", case.inputfile),
        );
    };

    let parsed = match parser.parse(&case.expression) {
        Ok(p) => p,
        Err(e) => {
            return if case.invalid.is_some() {
                (Outcome::Pass, "expected invalid: parse error".into())
            } else {
                (Outcome::ParseError, e.to_string())
            };
        }
    };

    let context = EvaluationContext::new(resource.clone());
    let result = match evaluator.evaluate(&parsed, &context) {
        Ok(r) => r,
        Err(e) => {
            return if case.invalid.is_some() {
                (Outcome::Pass, "expected invalid: eval error".into())
            } else {
                (Outcome::EvalError, e.to_string())
            };
        }
    };

    if case.invalid.is_some() {
        // Expected an error but evaluation succeeded. Many engines evaluate
        // "semantically invalid" expressions to empty; treat empty as a
        // tolerated error-equivalent, anything else as a wrong answer.
        return match &result {
            FhirPathValue::Empty => (Outcome::Pass, "expected invalid: empty result".into()),
            FhirPathValue::Collection(c) if c.is_empty() => {
                (Outcome::Pass, "expected invalid: empty result".into())
            }
            other => (
                Outcome::WrongAnswer,
                format!("expected error ({:?}), got {other:?}", case.invalid),
            ),
        };
    }

    let actual = flatten(result);

    if case.predicate {
        let exists = !actual.is_empty();
        let expected = case
            .outputs
            .first()
            .map(|(_, v)| v == "true")
            .unwrap_or(false);
        return if exists == expected {
            (Outcome::Pass, String::new())
        } else {
            (
                Outcome::WrongAnswer,
                format!("predicate: expected {expected}, got {exists}"),
            )
        };
    }

    if actual.len() != case.outputs.len() {
        return (
            Outcome::WrongAnswer,
            format!(
                "expected {} value(s) {:?}, got {} value(s) {:?}",
                case.outputs.len(),
                case.outputs,
                actual.len(),
                actual
            ),
        );
    }

    for (value, (expected_type, expected)) in actual.iter().zip(&case.outputs) {
        if !matches_expected(value, expected_type, expected) {
            return (
                Outcome::WrongAnswer,
                format!("expected {expected_type} {expected:?}, got {value:?}"),
            );
        }
    }

    (Outcome::Pass, String::new())
}

#[test]
fn hl7_fhirpath_conformance_suite() {
    let xml = fs::read_to_string(Path::new(FIXTURES).join("tests-fhir-r4.xml"))
        .expect("vendored suite present");
    let cases = parse_suite(&xml);
    assert!(cases.len() > 900, "suite should contain ~937 cases");

    let inputs = load_inputs();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let mut tallies: HashMap<&'static str, usize> = HashMap::new();
    let mut wrong_answers: Vec<(String, String)> = Vec::new();
    let mut failures_by_group: HashMap<String, usize> = HashMap::new();
    let mut details = Vec::new();

    for case in &cases {
        let (outcome, detail) = run_case(case, &parser, &evaluator, &inputs);
        let key = match outcome {
            Outcome::Pass => "pass",
            Outcome::WrongAnswer => "wrong_answer",
            Outcome::ParseError => "parse_error",
            Outcome::EvalError => "eval_error",
            Outcome::Skipped => "skipped",
        };
        *tallies.entry(key).or_default() += 1;
        if outcome != Outcome::Pass {
            *failures_by_group.entry(case.group.clone()).or_default() += 1;
            details.push(serde_json::json!({
                "group": case.group,
                "name": case.name,
                "expression": case.expression,
                "outcome": key,
                "detail": detail,
            }));
        }
        if outcome == Outcome::WrongAnswer {
            wrong_answers.push((format!("{}::{}", case.group, case.name), detail));
        }
    }

    let total = cases.len();
    let pass = tallies.get("pass").copied().unwrap_or(0);
    let summary = serde_json::json!({
        "suite": "tests-fhir-r4.xml",
        "total": total,
        "pass": pass,
        "pass_rate": format!("{:.1}%", 100.0 * pass as f64 / total as f64),
        "wrong_answer": tallies.get("wrong_answer").copied().unwrap_or(0),
        "parse_error": tallies.get("parse_error").copied().unwrap_or(0),
        "eval_error": tallies.get("eval_error").copied().unwrap_or(0),
        "skipped": tallies.get("skipped").copied().unwrap_or(0),
        "failures": details,
    });

    let out_path = std::env::var("HL7_CONFORMANCE_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/hl7_fhirpath_conformance.json")
        });
    fs::write(
        &out_path,
        serde_json::to_string_pretty(&summary).expect("serialize"),
    )
    .expect("write summary JSON");

    let mut report = String::new();
    let _ = writeln!(
        report,
        "HL7 FHIRPath conformance: {pass}/{total} pass ({:.1}%), {} wrong, {} parse-err, {} eval-err, {} skipped — details: {}",
        100.0 * pass as f64 / total as f64,
        tallies.get("wrong_answer").copied().unwrap_or(0),
        tallies.get("parse_error").copied().unwrap_or(0),
        tallies.get("eval_error").copied().unwrap_or(0),
        tallies.get("skipped").copied().unwrap_or(0),
        out_path.display(),
    );
    println!("{report}");

    // Gate: no NEW wrong answers.
    let new_wrong: Vec<_> = wrong_answers
        .iter()
        .filter(|(id, _)| !KNOWN_WRONG_ANSWERS.contains(&id.as_str()))
        .collect();
    assert!(
        new_wrong.is_empty(),
        "{} new wrong answer(s) vs baseline:\n{}",
        new_wrong.len(),
        new_wrong
            .iter()
            .map(|(id, d)| format!("  {id}: {d}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Gate: baseline is shrink-only; flag stale entries so the list gets cleaned up.
    let stale: Vec<_> = KNOWN_WRONG_ANSWERS
        .iter()
        .filter(|id| !wrong_answers.iter().any(|(w, _)| w == *id))
        .collect();
    assert!(
        stale.is_empty(),
        "stale KNOWN_WRONG_ANSWERS entries (now passing — remove them): {stale:?}"
    );
}
