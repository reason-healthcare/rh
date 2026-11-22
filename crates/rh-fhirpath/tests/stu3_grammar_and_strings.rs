//! Regression cases from the FHIRPath 3.0.0 audit.

use rh_fhirpath::{EvaluationContext, FhirModel, FhirPathEvaluator, FhirPathParser};
use serde_json::{json, Value};

fn evaluate(expression: &str, model: FhirModel, resource: Value) -> Value {
    let expression_ast = FhirPathParser::new()
        .parse(expression)
        .unwrap_or_else(|error| panic!("{expression}: {error}"));
    let mut context = EvaluationContext::new(resource);
    context.model = model;
    FhirPathEvaluator::new()
        .evaluate(&expression_ast, &context)
        .unwrap_or_else(|error| panic!("{expression}: {error}"))
        .to_json()
}

#[test]
fn published_precedence_and_lexical_rules() {
    for (expression, expected) in [
        ("1 * 2 is Integer", json!(true)),
        ("true in 1 = 1", json!(true)),
        ("{ /* gap */ }.empty()", json!(true)),
        ("-2147483648", json!(i32::MIN)),
        (
            "(-9223372036854775808L).toString()",
            json!(i64::MIN.to_string()),
        ),
    ] {
        assert_eq!(
            evaluate(expression, FhirModel::R4, Value::Null),
            expected,
            "{expression}"
        );
    }
    for bad in ["2147483648", "9223372036854775808L", "1.5L", r"'\uD83D'"] {
        assert!(FhirPathParser::new().parse(bad).is_err(), "{bad}");
    }
    for literal in ["@2023-02-29", "@2024-13", "@0000", "@T25:00"] {
        let expression = FhirPathParser::new().parse(literal).unwrap();
        assert!(
            FhirPathEvaluator::new()
                .evaluate(&expression, &EvaluationContext::new(Value::Null))
                .is_err(),
            "{literal}"
        );
    }
    let resource = json!({"value": {"falsehood": 3, "a b": 4}});
    for (expression, expected) in [("value.falsehood", 3), (r"value.`a\u0020b`", 4)] {
        assert_eq!(
            evaluate(expression, FhirModel::R4, resource.clone()),
            json!(expected)
        );
    }
}

#[test]
fn constructors_create_typed_values_without_source_paths() {
    for (expression, expected) in [
        ("FHIR.HumanName { family: 'Smith' }.family", json!("Smith")),
        (
            "CodeableConcept { coding: Coding { code: 'x' } }.coding.code.single()",
            json!("x"),
        ),
        ("Observation { value: 42 }.value.getValue()", json!(42)),
        (
            "Attachment { size: 9223372036854775807L }.size.getValue()",
            json!(i64::MAX),
        ),
        ("code { value: 'final', id: 'x' }.id", json!("x")),
        ("Period {:}.type().name", json!("Period")),
        (
            "FHIR.HumanName { family: 'Smith' }.family.pathname().empty()",
            json!(true),
        ),
    ] {
        assert_eq!(
            evaluate(expression, FhirModel::R5, Value::Null),
            expected,
            "{expression}"
        );
    }
}

#[test]
fn codecs_and_string_functions_preserve_value_semantics() {
    for (expression, expected) in [
        (r"'a\uD83D\uDD25b'.indexOf('b')", json!(2)),
        ("'a'.split(',').indexOf('a'.split(','))", json!(0)),
        ("'é🔥'.encode('ascii')", json!("??")),
        ("'AéB'.decode('hex').empty()", json!(true)),
        ("'test'.decode().empty()", json!(true)),
        (
            r"'\u0001\n\\n'.escape('json').unescape('json')",
            json!("\u{1}\n\\n"),
        ),
        ("'é'.escape('html').unescape('html')", json!("é")),
        (r"'\u00A0x\u00A0'.trim()", json!("\u{a0}x\u{a0}")),
    ] {
        assert_eq!(
            evaluate(expression, FhirModel::R4, Value::Null),
            expected,
            "{expression}"
        );
    }
}
