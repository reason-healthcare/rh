use std::collections::BTreeMap;

use rh_cql::{
    compile_with_libraries, evaluate_elm_with_libraries, CqlDateTime, EvalContext,
    EvalContextBuilder, FixedClock, LibraryIdentifier, MemoryLibrarySourceProvider, Value,
};

const FHIR_HELPERS: &str = include_str!("fixtures/fhirhelpers/FHIRHelpers-4.0.1.cql");

const SOURCE: &str = r#"
library FhirChoicePrimitive version '1.0.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1' called FHIRHelpers
context Patient
define "BooleanAnswer": (Patient.answer.value as FHIR.boolean).value
define "EncounterStart": FHIRHelpers.ToDateTime(Patient.start)
"#;

fn clock() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2026,
        month: Some(6),
        day: Some(15),
        hour: Some(9),
        minute: Some(20),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0),
    })
}

fn context(answer: Value) -> EvalContext {
    let mut patient = BTreeMap::new();
    patient.insert("answer".to_string(), answer);
    patient.insert(
        "start".to_string(),
        Value::String("2026-06-15T09:20:00Z".to_string()),
    );
    EvalContextBuilder::new(clock())
        .context_value(Value::Tuple(patient))
        .build()
}

fn evaluate(answer: Value) -> Value {
    let provider = MemoryLibrarySourceProvider::new();
    provider.register_source(
        LibraryIdentifier::new("FHIRHelpers", Some("4.0.1")),
        FHIR_HELPERS.to_string(),
    );
    let compiled = compile_with_libraries(SOURCE, None, &provider).expect("compile library");
    assert!(compiled.result.is_success(), "{:?}", compiled.result.errors);
    evaluate_elm_with_libraries(
        &compiled.result.library,
        &compiled.included,
        "BooleanAnswer",
        &context(answer),
    )
    .expect("evaluate BooleanAnswer")
}

fn answer(fields: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    Value::Tuple(
        fields
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect(),
    )
}

#[test]
fn standard_fhir_boolean_choice_access_handles_true_false_and_null() {
    assert_eq!(
        evaluate(answer([("valueBoolean", Value::Boolean(true))])),
        Value::Boolean(true)
    );
    assert_eq!(
        evaluate(answer([("valueBoolean", Value::Boolean(false))])),
        Value::Boolean(false)
    );
    assert_eq!(evaluate(answer([])), Value::Null);
}

#[test]
fn standard_fhir_boolean_choice_access_rejects_wrong_or_ambiguous_choice_types() {
    assert_eq!(
        evaluate(answer([("valueString", Value::String("true".to_string()))])),
        Value::Null
    );
    assert_eq!(
        evaluate(answer([("valueInteger", Value::Integer(1))])),
        Value::Null
    );
    assert_eq!(
        evaluate(answer([
            ("valueBoolean", Value::Boolean(false)),
            ("valueString", Value::String("false".to_string())),
        ])),
        Value::Null
    );
}

#[test]
fn standard_fhir_boolean_choice_access_preserves_exact_value_and_false_not_zero() {
    assert_eq!(
        evaluate(answer([
            ("value", Value::Boolean(false)),
            ("valueBoolean", Value::Boolean(true)),
        ])),
        Value::Boolean(false)
    );
    assert_eq!(
        evaluate(answer([("valueInteger", Value::Integer(0))])),
        Value::Null
    );
}
