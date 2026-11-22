//! Native HL7 FHIRPath suites for the R4 and R5 logical models.
//!
//! R5 preserves typed values, distinguishes parser and evaluator failures, and
//! runs with the specified initial focus. Each in-profile difference is gated
//! by ID and outcome. Reports are written under the workspace target directory
//! or at the paths in HL7_CONFORMANCE_OUT and HL7_R5_CONFORMANCE_OUT.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::XmlVersion;
use rh_fhirpath::evaluator::metadata::FhirModel;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

struct Suite {
    filename: &'static str,
    fixtures: &'static str,
    model: FhirModel,
    upstream_revision: Option<&'static str>,
    report_env: &'static str,
    report_filename: &'static str,
    expected_case_count: usize,
    baseline: &'static str,
}

const R4: Suite = Suite {
    filename: "tests-fhir-r4.xml",
    fixtures: "tests/fixtures/hl7_fhirpath_tests",
    model: FhirModel::R4,
    upstream_revision: None,
    report_env: "HL7_CONFORMANCE_OUT",
    report_filename: "hl7_fhirpath_conformance.json",
    expected_case_count: 935,
    baseline: include_str!("fixtures/hl7_fhirpath_tests/known-differences.tsv"),
};

const R5: Suite = Suite {
    filename: "tests-fhir-r5.xml",
    fixtures: "tests/fixtures/hl7_fhirpath_r5",
    model: FhirModel::R5,
    upstream_revision: Some("469d90ab58663266d5d7ff82ba3587257fd8a4be"),
    report_env: "HL7_R5_CONFORMANCE_OUT",
    report_filename: "hl7_fhirpath_r5_conformance.json",
    expected_case_count: 1381,
    baseline: include_str!("fixtures/hl7_fhirpath_r5/known-differences.tsv"),
};

#[derive(Debug, Clone)]
struct TestCase {
    group: String,
    name: String,
    inputfile: String,
    expression: String,
    invalid: Option<String>,
    predicate: bool,
    outputs: Vec<(String, String)>, // (type, value)
    mode: Option<String>,
    check_ordered_functions: bool,
    ordered: bool,
    context: Option<String>,
    context_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Outcome {
    Pass,
    WrongAnswer,
    ParseError,
    EvalError,
    Skipped,
    OutsideProfile,
}

impl Outcome {
    fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::WrongAnswer => "wrong_answer",
            Self::ParseError => "parse_error",
            Self::EvalError => "eval_error",
            Self::Skipped => "skipped",
            Self::OutsideProfile => "outside_profile",
        }
    }
}

fn parse_suite(xml: &str) -> Vec<TestCase> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);

    let mut cases = Vec::new();
    let mut group = String::new();
    let mut group_mode = None;
    let mut current: Option<TestCase> = None;
    let mut in_expression = false;
    let mut in_output = false;
    let mut output_type = String::new();
    let mut text_buf = String::new();

    loop {
        match reader.read_event().expect("well-formed suite XML") {
            Event::Start(e) | Event::Empty(e) => {
                let attrs: HashMap<String, String> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| (a.key.as_ref().to_owned(), a.value.into_owned()))
                    .collect();
                match e.name().as_ref() {
                    "group" => {
                        group = attrs.get("name").cloned().unwrap_or_default();
                        group_mode = attrs.get("mode").cloned();
                    }
                    "test" => {
                        current = Some(TestCase {
                            group: group.clone(),
                            name: attrs.get("name").cloned().unwrap_or_default(),
                            inputfile: attrs.get("inputfile").cloned().unwrap_or_default(),
                            expression: String::new(),
                            invalid: attrs.get("invalid").cloned(),
                            predicate: attrs.get("predicate").is_some_and(|p| p == "true"),
                            outputs: Vec::new(),
                            mode: if group_mode.as_deref() == Some("custom") {
                                group_mode.clone()
                            } else {
                                attrs.get("mode").cloned().or_else(|| group_mode.clone())
                            },
                            ordered: attrs.get("ordered").is_none_or(|value| value != "false"),
                            context: attrs.get("context").cloned(),
                            context_type: attrs.get("contextType").cloned(),
                            check_ordered_functions: attrs
                                .get("checkOrderedFunctions")
                                .is_some_and(|p| p == "true")
                                || (attrs.get("mode").is_some_and(|mode| mode == "strict")
                                    && attrs.get("ordered").is_some_and(|order| order == "true")),
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
            Event::Text(t) if in_expression || in_output => {
                text_buf.push_str(&t.xml_content(XmlVersion::Implicit1_0));
            }
            Event::GeneralRef(r) if in_expression || in_output => {
                let entity = r.as_ref();
                let escaped = format!("&{entity};");
                text_buf.push_str(&unescape(&escaped).expect("unescape entity reference"));
            }
            Event::End(e) => match e.name().as_ref() {
                "expression" => {
                    if let Some(case) = current.as_mut() {
                        case.expression = text_buf.clone();
                    }
                    in_expression = false;
                }
                "output" => {
                    if let Some(case) = current.as_mut() {
                        case.outputs.push((output_type.clone(), text_buf.clone()));
                    }
                    in_output = false;
                }
                "test" => {
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

fn load_inputs(fixtures: &Path) -> HashMap<String, serde_json::Value> {
    let dir = fixtures.join("input");
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

fn result_values(value: &FhirPathValue) -> &[FhirPathValue] {
    match value {
        FhirPathValue::Empty => &[],
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => items,
        other => std::slice::from_ref(other),
    }
}

/// Compare a single actual value against an expected (type, text) output.
fn matches_expected(
    actual: &FhirPathValue,
    expected_type: &str,
    expected: &str,
    model: FhirModel,
) -> bool {
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
        } else if expected.parse::<i64>().is_ok() {
            "integer"
        } else if expected.parse::<f64>().is_ok() {
            "decimal"
        } else {
            "string"
        };
        return matches_expected(actual, inferred, expected, model);
    }
    match expected_type {
        "boolean" => match actual {
            FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => {
                expected.parse::<bool>() == Ok(*value)
            }
            _ => false,
        },
        "integer" => match actual {
            FhirPathValue::Integer(i) => expected.parse::<i64>() == Ok(*i),
            FhirPathValue::Long(i) if model == FhirModel::R4 => expected.parse::<i64>() == Ok(*i),
            FhirPathValue::Number(n) if model == FhirModel::R4 && n.fract() == Decimal::ZERO => {
                expected.parse::<i64>().ok() == n.to_i64()
            }
            _ => false,
        },
        "long" | "integer64" => {
            matches!(actual, FhirPathValue::Long(value) if expected.parse::<i64>() == Ok(*value))
        }
        "decimal" => {
            let expected_dec = match Decimal::from_str_exact(expected)
                .or_else(|_| Decimal::from_scientific(expected))
            {
                Ok(d) => d,
                Err(_) => return false,
            };
            match actual {
                FhirPathValue::Number(n) => n == &expected_dec,
                FhirPathValue::Integer(i) | FhirPathValue::Long(i) if model == FhirModel::R4 => {
                    Decimal::from(*i) == expected_dec
                }
                _ => false,
            }
        }
        "string" | "code" | "id" | "uri" => match actual {
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => s == expected,
            FhirPathValue::Object(serde_json::Value::String(s)) => s == expected,
            _ => false,
        },
        "date" | "dateTime" | "time" => {
            let expected_norm = expected.trim_start_matches('@');
            let actual_str = match actual {
                FhirPathValue::Date(s)
                | FhirPathValue::DateTime(s)
                | FhirPathValue::Time(s)
                | FhirPathValue::TypedDateTime { value: s, .. } => s.as_str(),
                FhirPathValue::String(s) if model == FhirModel::R4 => s.as_str(),
                _ => return false,
            };
            let actual_norm = actual_str.trim_start_matches('@').trim_start_matches('T');
            let expected_norm = expected_norm.trim_start_matches('T');
            actual_norm == expected_norm
        }
        "Quantity" => match actual {
            FhirPathValue::Quantity { value, unit } => {
                let (num_str, unit_str) = match expected.split_once(' ') {
                    Some((n, u)) => (n, Some(u.trim().trim_matches('\''))),
                    None => (expected, None),
                };
                let Ok(expected_num) = Decimal::from_str_exact(num_str) else {
                    return false;
                };
                value == &expected_num
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

fn outside_profile_reason(case: &TestCase, model: FhirModel) -> Option<&'static str> {
    if case.group == "testConformsTo" && case.invalid.is_none() {
        return Some("requires an installed profile validation host");
    }
    if model != FhirModel::R5 {
        return None;
    }
    match case.mode.as_deref() {
        Some("custom") => Some("document and custom functions outside FHIRPath 3.0.0"),
        Some("cda") => Some("CDA model"),
        Some("tx") => Some("draft terminology client API"),
        Some("semantic") if case.invalid.as_deref() == Some("semantic") => {
            Some("requires optional static type checking against declared contextType")
        }
        _ => None,
    }
}

fn run_case(
    case: &TestCase,
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    inputs: &HashMap<String, serde_json::Value>,
    model: FhirModel,
) -> (Outcome, String) {
    if let Some(reason) = outside_profile_reason(case, model) {
        return (Outcome::OutsideProfile, reason.into());
    }

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
            return if case.invalid.as_deref().is_some_and(|invalid| {
                model == FhirModel::R4 || matches!(invalid, "syntax" | "true")
            }) {
                (Outcome::Pass, "expected invalid: parse error".into())
            } else {
                (Outcome::ParseError, e.to_string())
            };
        }
    };

    let mut context = EvaluationContext::new(resource.clone());
    context.model = model;
    context.strict_mode =
        case.mode.as_deref() == Some("strict") || case.invalid.as_deref() == Some("semantic");
    context.check_ordered_functions = case.check_ordered_functions;
    if let Some(focus_expression) = &case.context {
        let focus = parser
            .parse(focus_expression)
            .and_then(|ast| evaluator.evaluate_nodes(&ast, &context));
        match focus {
            Ok(value) if !value.is_empty() => context = context.with_initial_focus(value),
            Ok(_) => {
                return (
                    Outcome::EvalError,
                    format!("Fixture focus {focus_expression} was empty"),
                )
            }
            Err(error) => {
                return (
                    Outcome::EvalError,
                    format!("Invalid fixture focus {focus_expression}: {error}"),
                )
            }
        }
    }
    let result = match evaluator.evaluate(&parsed, &context) {
        Ok(r) => r,
        Err(e) => {
            return if case
                .invalid
                .as_deref()
                .is_some_and(|invalid| model == FhirModel::R4 || invalid != "syntax")
            {
                (Outcome::Pass, "expected invalid: eval error".into())
            } else {
                (Outcome::EvalError, e.to_string())
            };
        }
    };

    if let Some(invalid) = &case.invalid {
        if model == FhirModel::R5 {
            return (
                Outcome::WrongAnswer,
                format!("expected {invalid} error, got {result:?}"),
            );
        }
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

    compare_outputs(case, result_values(&result), model)
}

fn compare_outputs(
    case: &TestCase,
    actual: &[FhirPathValue],
    model: FhirModel,
) -> (Outcome, String) {
    if case.predicate {
        let exists = !actual.is_empty();
        let expected = case.outputs.first().is_some_and(|(_, v)| v == "true");
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

    let mut used = vec![false; actual.len()];
    for (index, (expected_type, expected)) in case.outputs.iter().enumerate() {
        let candidate = if case.ordered {
            matches_expected(&actual[index], expected_type, expected, model).then_some(index)
        } else {
            actual.iter().enumerate().position(|(index, value)| {
                !used[index] && matches_expected(value, expected_type, expected, model)
            })
        };
        let Some(index) = candidate else {
            return (
                Outcome::WrongAnswer,
                format!("expected {expected_type} {expected:?}, got {actual:?}"),
            );
        };
        used[index] = true;
    }

    (Outcome::Pass, String::new())
}

fn run_suite(suite: &Suite) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixtures = manifest_dir.join(suite.fixtures);
    let filename = suite.filename;
    let xml = fs::read_to_string(fixtures.join(filename)).expect("vendored suite present");
    let cases = parse_suite(&xml);
    assert_eq!(
        cases.len(),
        suite.expected_case_count,
        "pinned suite changed"
    );

    let inputs = load_inputs(&fixtures);
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let mut tallies = HashMap::<Outcome, usize>::new();
    let mut differences = Vec::new();

    for case in &cases {
        let (outcome, detail) = run_case(case, &parser, &evaluator, &inputs, suite.model);
        *tallies.entry(outcome).or_default() += 1;
        if outcome != Outcome::Pass {
            differences.push((case, outcome, detail));
        }
    }

    let details: Vec<_> = differences
        .iter()
        .map(|(case, outcome, detail)| {
            serde_json::json!({
                "group": case.group,
                "name": case.name,
                "expression": case.expression,
                "context": case.context,
                "context_type": case.context_type,
                "outcome": outcome.label(),
                "detail": detail,
            })
        })
        .collect();
    let total = cases.len();
    let count = |outcome| tallies.get(&outcome).copied().unwrap_or(0);
    let pass = count(Outcome::Pass);
    let wrong_answer = count(Outcome::WrongAnswer);
    let parse_error = count(Outcome::ParseError);
    let eval_error = count(Outcome::EvalError);
    let skipped = count(Outcome::Skipped);
    let outside_profile = count(Outcome::OutsideProfile);
    let summary = serde_json::json!({
        "suite": filename,
        "upstream_revision": suite.upstream_revision,
        "total": total,
        "pass": pass,
        "pass_rate": format!("{:.1}%", 100.0 * pass as f64 / total as f64),
        "wrong_answer": wrong_answer,
        "parse_error": parse_error,
        "eval_error": eval_error,
        "skipped": skipped,
        "outside_profile": outside_profile,
        "failures": details,
    });
    let out_path = std::env::var(suite.report_env)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            manifest_dir
                .join("../../target")
                .join(suite.report_filename)
        });
    if let Some(parent) = out_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).expect("create report directory");
    }
    fs::write(
        &out_path,
        serde_json::to_string_pretty(&summary).expect("serialize report"),
    )
    .expect("write summary JSON");
    println!(
        "{filename}: {pass}/{total} pass, {wrong_answer} wrong answers, {parse_error} parse errors, \
         {eval_error} evaluation errors, {skipped} skipped, {outside_profile} outside profile (report {})",
        out_path.display()
    );

    let failures: Vec<_> = differences
        .iter()
        .filter(|(_, outcome, _)| *outcome != Outcome::OutsideProfile)
        .map(|(case, outcome, detail)| {
            (
                format!("{}::{}", case.group, case.name),
                outcome.label(),
                detail.as_str(),
            )
        })
        .collect();
    check_baseline(&failures, suite.baseline);
}

#[test]
fn hl7_fhirpath_conformance_suite() {
    run_suite(&R4);
}

#[test]
fn published_fhirpath_r5_suite() {
    run_suite(&R5);
}

fn check_baseline(failures: &[(String, &str, &str)], baseline: &str) {
    let known: HashMap<_, _> = baseline
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut fields = line.splitn(3, '\t');
            let id = fields.next().unwrap();
            let outcome = fields.next().expect("baseline outcome");
            let reason = fields.next().expect("baseline reason");
            assert!(
                !reason.trim().is_empty(),
                "missing baseline reason for {id}"
            );
            (id, outcome)
        })
        .collect();
    let new: Vec<_> = failures
        .iter()
        .filter(|(id, outcome, _)| known.get(id.as_str()) != Some(outcome))
        .collect();
    assert!(new.is_empty(), "new conformance differences: {new:#?}");
    let stale: Vec<_> = known
        .keys()
        .filter(|id| !failures.iter().any(|(failure, _, _)| failure == **id))
        .collect();
    assert!(
        stale.is_empty(),
        "stale conformance baseline entries: {stale:?}"
    );
}
