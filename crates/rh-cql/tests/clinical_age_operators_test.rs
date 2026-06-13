//! Clinical age operators: `AgeIn<unit>[At]` (implicit patient context) and
//! `CalculateAgeIn<unit>[At]` (explicit birthDate argument).

use std::collections::BTreeMap;

use rh_cql::{
    compile_with_model, evaluate_elm, CqlDateTime, EvalContext, EvalContextBuilder, FixedClock,
    Value,
};

fn clock_2024_06_15() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2024,
        month: Some(6),
        day: Some(15),
        hour: Some(10),
        minute: Some(30),
        second: Some(0),
        millisecond: None,
        offset_seconds: None,
    })
}

fn patient_ctx(birth_date: &str) -> EvalContext {
    let mut fields = BTreeMap::new();
    fields.insert(
        "birthDate".to_string(),
        Value::String(birth_date.to_string()),
    );
    EvalContextBuilder::new(clock_2024_06_15())
        .context_value(Value::Tuple(fields))
        .build()
}

fn eval(cql: &str, name: &str, ctx: &EvalContext) -> Value {
    let result = compile_with_model(cql, None, None).expect("compile failed");
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    evaluate_elm(&result.library, name, ctx).expect("evaluation failed")
}

#[test]
fn age_in_years_uses_patient_context_birth_date() {
    let ctx = patient_ctx("1990-01-01");
    assert_eq!(
        eval("library T define X: AgeInYears()", "X", &ctx),
        Value::Integer(34)
    );
}

#[test]
fn age_in_months_uses_patient_context_birth_date() {
    let ctx = patient_ctx("1990-01-01");
    assert_eq!(
        eval("library T define X: AgeInMonths()", "X", &ctx),
        Value::Integer(413)
    );
}

#[test]
fn age_in_years_at_supplied_date() {
    let ctx = patient_ctx("1990-01-01");
    assert_eq!(
        eval("library T define X: AgeInYearsAt(@2020-06-15)", "X", &ctx),
        Value::Integer(30)
    );
}

#[test]
fn age_is_null_without_patient_context() {
    let ctx = EvalContextBuilder::new(clock_2024_06_15()).build();
    assert_eq!(
        eval("library T define X: AgeInYears()", "X", &ctx),
        Value::Null
    );
}

#[test]
fn calculate_age_in_years_with_explicit_birth_date() {
    let ctx = EvalContextBuilder::new(clock_2024_06_15()).build();
    assert_eq!(
        eval(
            "library T define X: CalculateAgeInYears(@1990-01-01)",
            "X",
            &ctx
        ),
        Value::Integer(34)
    );
}

#[test]
fn calculate_age_in_years_at_explicit_dates() {
    let ctx = EvalContextBuilder::new(clock_2024_06_15()).build();
    assert_eq!(
        eval(
            "library T define X: CalculateAgeInYearsAt(@1990-01-01, @2020-01-01)",
            "X",
            &ctx
        ),
        Value::Integer(30)
    );
}

#[test]
fn calculate_age_in_days_at_explicit_dates() {
    let ctx = EvalContextBuilder::new(clock_2024_06_15()).build();
    assert_eq!(
        eval(
            "library T define X: CalculateAgeInDaysAt(@2024-01-01, @2024-06-15)",
            "X",
            &ctx
        ),
        Value::Integer(166)
    );
}

#[test]
fn calculate_age_with_null_birth_date_is_null() {
    let ctx = EvalContextBuilder::new(clock_2024_06_15()).build();
    assert_eq!(
        eval(
            "library T define X: CalculateAgeInYears(null as Date)",
            "X",
            &ctx
        ),
        Value::Null
    );
}
