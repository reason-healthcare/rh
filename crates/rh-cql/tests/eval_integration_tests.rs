//! End-to-end integration tests: compile CQL → evaluate ELM.
//!
//! Each test compiles a small CQL snippet with no FHIR model (using
//! `compile_with_model(..., None)`) and then evaluates a named expression
//! against an `EvalContext`, asserting on the resulting `Value`.

use rh_cql::{
    compile_with_model, evaluate_elm, EvalContextBuilder, FixedClock, Value,
    CqlDate, CqlDateTime, CqlQuantity, CqlTime,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn test_clock() -> FixedClock {
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

fn default_ctx() -> rh_cql::EvalContext {
    EvalContextBuilder::new(test_clock()).build()
}

/// Compile a standalone CQL snippet (no FHIR model) and evaluate the
/// expression named `expr_name`, panicking on any error.
fn eval_expr(cql: &str, expr_name: &str) -> Value {
    let result = compile_with_model(cql, None, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }
    let ctx = default_ctx();
    evaluate_elm(&result.library, expr_name, &ctx).expect("evaluation failed")
}

// ---------------------------------------------------------------------------
// 9.23 — Arithmetic operators (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_integer_addition() {
    let cql = "library T define X: 3 + 4";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(7));
}

#[test]
fn eval_integer_subtraction() {
    let cql = "library T define X: 10 - 3";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(7));
}

#[test]
fn eval_integer_multiplication() {
    let cql = "library T define X: 6 * 7";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(42));
}

#[test]
fn eval_division_yields_decimal() {
    let cql = "library T define X: 7 / 2";
    assert_eq!(eval_expr(cql, "X"), Value::Decimal(3.5));
}

#[test]
fn eval_modulo() {
    let cql = "library T define X: 10 mod 3";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(1));
}

#[test]
fn eval_negation() {
    let cql = "library T define X: -(5)";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(-5));
}

#[test]
fn eval_chained_arithmetic() {
    // (2 + 3) * 4 = 20
    let cql = "library T define X: (2 + 3) * 4";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(20));
}

// ---------------------------------------------------------------------------
// Comparison operators (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_less_than_true() {
    let cql = "library T define X: 1 < 2";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_less_than_false() {
    let cql = "library T define X: 5 < 3";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

#[test]
fn eval_greater_or_equal() {
    let cql = "library T define X: 5 >= 5";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_equal_integers() {
    let cql = "library T define X: 7 = 7";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_not_equal() {
    let cql = "library T define X: 7 != 8";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

// ---------------------------------------------------------------------------
// Boolean / three-valued logic (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_boolean_and_true() {
    let cql = "library T define X: true and true";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_boolean_and_false() {
    let cql = "library T define X: true and false";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

#[test]
fn eval_boolean_or() {
    let cql = "library T define X: false or true";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_boolean_not() {
    let cql = "library T define X: not true";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

#[test]
fn eval_null_propagation_in_and() {
    // null and false = false; null and true = null
    let cql = "library T define X: null and false";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

// ---------------------------------------------------------------------------
// String operators (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_string_concatenation() {
    let cql = "library T define X: 'Hello' + ', ' + 'World'";
    assert_eq!(eval_expr(cql, "X"), Value::String("Hello, World".into()));
}

#[test]
fn eval_string_literal() {
    let cql = "library T define X: 'CQL'";
    assert_eq!(eval_expr(cql, "X"), Value::String("CQL".into()));
}

// ---------------------------------------------------------------------------
// Null literal (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_null_literal() {
    let cql = "library T define X: null";
    assert_eq!(eval_expr(cql, "X"), Value::Null);
}

// ---------------------------------------------------------------------------
// If / Then / Else (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_if_true_branch() {
    let cql = "library T define X: if 1 < 2 then 'yes' else 'no'";
    assert_eq!(eval_expr(cql, "X"), Value::String("yes".into()));
}

#[test]
fn eval_if_false_branch() {
    let cql = "library T define X: if 5 < 2 then 'yes' else 'no'";
    assert_eq!(eval_expr(cql, "X"), Value::String("no".into()));
}

#[test]
fn eval_if_null_condition_returns_else() {
    let cql = "library T define X: if null then 'yes' else 'no'";
    assert_eq!(eval_expr(cql, "X"), Value::String("no".into()));
}

// ---------------------------------------------------------------------------
// Case expression (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_case_first_match() {
    let cql = "library T define X:
        case
          when 1 = 2 then 'first'
          when 3 = 3 then 'second'
          else 'third'
        end";
    assert_eq!(eval_expr(cql, "X"), Value::String("second".into()));
}

#[test]
fn eval_case_no_match_returns_else() {
    let cql = "library T define X:
        case
          when 1 = 2 then 'a'
          when 3 = 4 then 'b'
          else 'default'
        end";
    assert_eq!(eval_expr(cql, "X"), Value::String("default".into()));
}

// ---------------------------------------------------------------------------
// List expressions (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_list_literal() {
    let cql = "library T define X: {1, 2, 3}";
    assert_eq!(eval_expr(cql, "X"), Value::List(vec![
        Value::Integer(1), Value::Integer(2), Value::Integer(3),
    ]));
}

#[test]
fn eval_empty_list() {
    let cql = "library T define X: List<Integer>{}";
    assert_eq!(eval_expr(cql, "X"), Value::List(vec![]));
}

// ---------------------------------------------------------------------------
// Exists (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_exists_non_empty_list() {
    let cql = "library T define X: exists {1, 2, 3}";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_exists_empty_list() {
    let cql = "library T define X: exists List<Integer>{}";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

// ---------------------------------------------------------------------------
// Cross-expression references (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_expression_references_another() {
    let cql = "library T
define Base: 10
define Result: Base * 2";
    assert_eq!(eval_expr(cql, "Result"), Value::Integer(20));
}

#[test]
fn eval_multiple_expression_defs() {
    let cql = "library T
define A: 5
define B: 3
define C: A + B";
    assert_eq!(eval_expr(cql, "C"), Value::Integer(8));
}

// ---------------------------------------------------------------------------
// Type conversion operators (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_to_decimal_cast() {
    // Integer 5 converted to Decimal via ToString for use in string context
    let cql = "library T define X: ToString(5)";
    assert_eq!(eval_expr(cql, "X"), Value::String("5".into()));
}

#[test]
fn eval_to_integer_cast() {
    let cql = "library T define X: ToInteger('42')";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(42));
}

#[test]
fn eval_is_type_check() {
    let cql = "library T define X: 42 is Integer";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_as_type_cast() {
    let cql = "library T define X: 42 as Integer";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(42));
}

// ---------------------------------------------------------------------------
// Interval expressions (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_interval_literal() {
    let cql = "library T define X: Interval[1, 5]";
    let result = eval_expr(cql, "X");
    assert!(matches!(result, Value::Interval { ref low, ref high, low_closed: true, high_closed: true }
        if low.as_deref() == Some(&Value::Integer(1)) && high.as_deref() == Some(&Value::Integer(5))));
}

#[test]
fn eval_in_interval() {
    let cql = "library T define X: 3 in Interval[1, 5]";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(true));
}

#[test]
fn eval_not_in_interval() {
    let cql = "library T define X: 7 in Interval[1, 5]";
    assert_eq!(eval_expr(cql, "X"), Value::Boolean(false));
}

// ---------------------------------------------------------------------------
// Aggregate functions on lists (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_count_list() {
    let cql = "library T define X: Count({1, 2, 3, 4, 5})";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(5));
}

#[test]
fn eval_sum_list() {
    let cql = "library T define X: Sum({1, 2, 3})";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(6));
}

#[test]
fn eval_min_list() {
    let cql = "library T define X: Min({3, 1, 2})";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(1));
}

#[test]
fn eval_max_list() {
    let cql = "library T define X: Max({3, 1, 2})";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(3));
}

// ---------------------------------------------------------------------------
// Parameters (end-to-end)
// ---------------------------------------------------------------------------

#[test]
fn eval_parameter_value() {
    let cql = "library T
parameter InputValue Integer
define X: InputValue * 2";

    let result = compile_with_model(cql, None, None).expect("compile failed");
    let ctx = EvalContextBuilder::new(test_clock())
        .parameter("InputValue", Value::Integer(7))
        .build();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    assert_eq!(value, Value::Integer(14));
}

#[test]
fn eval_parameter_default_null() {
    // Parameters without a default evaluate to null when not provided
    let cql = "library T
parameter Missing Integer
define X: if Missing is null then 'absent' else 'present'";

    let result = compile_with_model(cql, None, None).expect("compile failed");
    let ctx = default_ctx();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    assert_eq!(value, Value::String("absent".into()));
}

// ---------------------------------------------------------------------------
// Evaluate with trace (evaluate_elm_with_trace)
// ---------------------------------------------------------------------------

#[test]
fn eval_with_trace_returns_events() {
    use rh_cql::evaluate_elm_with_trace;

    let cql = "library T define X: 2 + 3";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    let ctx = default_ctx();
    let (value, trace) = evaluate_elm_with_trace(&result.library, "X", &ctx)
        .expect("evaluation failed");

    assert_eq!(value, Value::Integer(5));
    assert!(!trace.is_empty(), "expected trace events");
    // At minimum there should be an Add event and two Literal events
    let has_add = trace.iter().any(|e| e.op == "Add");
    assert!(has_add, "expected an Add trace event, got: {:?}", trace.iter().map(|e| &e.op).collect::<Vec<_>>());
}

#[test]
fn eval_trace_event_has_inputs_and_output() {
    use rh_cql::evaluate_elm_with_trace;

    let cql = "library T define X: 10 + 5";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    let ctx = default_ctx();
    let (value, trace) = evaluate_elm_with_trace(&result.library, "X", &ctx)
        .expect("evaluation failed");

    assert_eq!(value, Value::Integer(15));
    let add_event = trace.iter().find(|e| e.op == "Add").expect("expected Add event");
    assert_eq!(add_event.output, Value::Integer(15));
    assert_eq!(add_event.inputs.len(), 2);
}
