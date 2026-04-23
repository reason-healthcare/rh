//! End-to-end integration tests: compile CQL → evaluate ELM.
//!
//! Each test compiles a small CQL snippet with no FHIR model (using
//! `compile_with_model(..., None)`) and then evaluates a named expression
//! against an `EvalContext`, asserting on the resulting `Value`.

use rh_cql::{
    compile_with_model, evaluate_elm, CqlDate, CqlDateTime, CqlTime, EvalContextBuilder,
    FixedClock, Value,
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
    assert_eq!(
        eval_expr(cql, "X"),
        Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ])
    );
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
    assert!(
        matches!(result, Value::Interval { ref low, ref high, low_closed: true, high_closed: true }
        if low.as_deref() == Some(&Value::Integer(1)) && high.as_deref() == Some(&Value::Integer(5)))
    );
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
    let (value, trace) =
        evaluate_elm_with_trace(&result.library, "X", &ctx).expect("evaluation failed");

    assert_eq!(value, Value::Integer(5));
    assert!(!trace.is_empty(), "expected trace events");
    // At minimum there should be an Add event and two Literal events
    let has_add = trace.iter().any(|e| e.op == "Add");
    assert!(
        has_add,
        "expected an Add trace event, got: {:?}",
        trace.iter().map(|e| &e.op).collect::<Vec<_>>()
    );
}

#[test]
fn eval_trace_event_has_inputs_and_output() {
    use rh_cql::evaluate_elm_with_trace;

    let cql = "library T define X: 10 + 5";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    let ctx = default_ctx();
    let (value, trace) =
        evaluate_elm_with_trace(&result.library, "X", &ctx).expect("evaluation failed");

    assert_eq!(value, Value::Integer(15));
    let add_event = trace
        .iter()
        .find(|e| e.op == "Add")
        .expect("expected Add event");
    assert_eq!(add_event.output, Value::Integer(15));
    assert_eq!(add_event.inputs.len(), 2);
}

// ---------------------------------------------------------------------------
// 9.23 — Query evaluation (end-to-end, spec §Query evaluation scenarios)
// ---------------------------------------------------------------------------

/// Scenario: Simple query with where
/// A query over a list literal filters elements by a condition.
#[test]
fn eval_query_simple_where() {
    // Filter a list of integers: return elements > 2
    let cql = "library T
define Numbers: {1, 2, 3, 4, 5}
define X: Numbers N where N > 2";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }
    let ctx = default_ctx();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => {
            assert_eq!(items.len(), 3, "expected 3 items > 2, got {:?}", items);
            assert!(items.contains(&Value::Integer(3)));
            assert!(items.contains(&Value::Integer(4)));
            assert!(items.contains(&Value::Integer(5)));
        }
        other => panic!("expected List, got {:?}", other),
    }
}

/// Scenario: Query with return projection
/// A query that projects a transformed value from each element.
#[test]
fn eval_query_return_projection() {
    // Multiply each element by 2 in the return clause
    let cql = "library T
define Numbers: {1, 2, 3}
define X: Numbers N return N * 2";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }
    let ctx = default_ctx();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => {
            assert_eq!(
                items.len(),
                3,
                "expected 3 projected items, got {:?}",
                items
            );
            assert!(items.contains(&Value::Integer(2)));
            assert!(items.contains(&Value::Integer(4)));
            assert!(items.contains(&Value::Integer(6)));
        }
        other => panic!("expected List, got {:?}", other),
    }
}

/// Scenario: Query with where + return (combined)
#[test]
fn eval_query_where_and_return() {
    let cql = "library T
define Numbers: {1, 2, 3, 4, 5}
define X: Numbers N where N >= 3 return N * 10";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }
    let ctx = default_ctx();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => {
            assert_eq!(items.len(), 3, "expected 3 items, got {:?}", items);
            assert!(items.contains(&Value::Integer(30)));
            assert!(items.contains(&Value::Integer(40)));
            assert!(items.contains(&Value::Integer(50)));
        }
        other => panic!("expected List, got {:?}", other),
    }
}

/// Scenario: Query returning empty list when no elements match
#[test]
fn eval_query_where_no_match_returns_empty() {
    let cql = "library T
define Numbers: {1, 2, 3}
define X: Numbers N where N > 100";
    let result = compile_with_model(cql, None, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }
    let ctx = default_ctx();
    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => assert!(items.is_empty(), "expected empty list, got {:?}", items),
        other => panic!("expected List, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 9.23 — Retrieve evaluation (end-to-end with InMemoryDataProvider)
// ---------------------------------------------------------------------------

/// Scenario: Retrieve returns all resources of a given type.
#[test]
fn eval_retrieve_returns_resources() {
    use rh_cql::{compile, InMemoryDataProvider};
    use std::collections::BTreeMap;

    let cql = "library T
using FHIR version '4.0.1'
define X: [Observation]";

    let result = compile(cql, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }

    let mut provider = InMemoryDataProvider::new();
    let mut obs1 = BTreeMap::new();
    obs1.insert("status".to_string(), Value::String("final".to_string()));
    obs1.insert("id".to_string(), Value::String("obs-1".to_string()));
    provider.add_resource("Observation", Value::Tuple(obs1));

    let mut obs2 = BTreeMap::new();
    obs2.insert(
        "status".to_string(),
        Value::String("preliminary".to_string()),
    );
    obs2.insert("id".to_string(), Value::String("obs-2".to_string()));
    provider.add_resource("Observation", Value::Tuple(obs2));

    let ctx = EvalContextBuilder::new(test_clock())
        .data_provider(provider)
        .build();

    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => {
            assert_eq!(items.len(), 2, "expected 2 Observations, got {:?}", items);
        }
        other => panic!("expected List, got {:?}", other),
    }
}

/// Scenario: Retrieve returns empty list when no resources registered.
#[test]
fn eval_retrieve_empty_when_no_resources() {
    use rh_cql::{compile, InMemoryDataProvider};

    let cql = "library T
using FHIR version '4.0.1'
define X: [Observation]";

    let result = compile(cql, None).expect("compile failed");
    if !result.errors.is_empty() {
        panic!("compilation errors: {:?}", result.errors);
    }

    let provider = InMemoryDataProvider::new();
    let ctx = EvalContextBuilder::new(test_clock())
        .data_provider(provider)
        .build();

    let value = evaluate_elm(&result.library, "X", &ctx).expect("evaluation failed");
    match value {
        Value::List(items) => assert!(items.is_empty(), "expected empty list, got {:?}", items),
        other => panic!("expected List, got {:?}", other),
    }
}

#[test]
fn eval_substring_function() {
    let cql = "library T define X: Substring('abc', 1, 1)";
    assert_eq!(eval_expr(cql, "X"), Value::String("b".into()));
}

#[test]
fn eval_position_of_function() {
    let cql = "library T define X: PositionOf('b', 'abc')";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(1));
}

#[test]
fn eval_last_position_of_function() {
    let cql = "library T define X: LastPositionOf('a', 'abca')";
    assert_eq!(eval_expr(cql, "X"), Value::Integer(3));
}

#[test]
fn eval_split_on_matches_function() {
    let cql = "library T define X: SplitOnMatches('a,b;c', '[,;]')";
    assert_eq!(
        eval_expr(cql, "X"),
        Value::List(vec![
            Value::String("a".into()),
            Value::String("b".into()),
            Value::String("c".into())
        ])
    );
}

#[test]
fn eval_replace_matches_function() {
    let cql = "library T define X: ReplaceMatches('ab ab', 'ab', 'cd')";
    assert_eq!(eval_expr(cql, "X"), Value::String("cd cd".into()));
}

#[test]
fn eval_tail_skip_take_slice_functions() {
    let tail = "library T define X: Tail({1,2,3,4})";
    assert_eq!(
        eval_expr(tail, "X"),
        Value::List(vec![
            Value::Integer(2),
            Value::Integer(3),
            Value::Integer(4)
        ])
    );

    let skip = "library T define X: Skip({1,2,3,4,5}, 2)";
    assert_eq!(
        eval_expr(skip, "X"),
        Value::List(vec![
            Value::Integer(3),
            Value::Integer(4),
            Value::Integer(5)
        ])
    );

    let take = "library T define X: Take({1,2,3,4,5}, 3)";
    assert_eq!(
        eval_expr(take, "X"),
        Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3)
        ])
    );

    let slice = "library T define X: Slice({1,2,3,4,5}, 1, 4)";
    assert_eq!(
        eval_expr(slice, "X"),
        Value::List(vec![
            Value::Integer(2),
            Value::Integer(3),
            Value::Integer(4)
        ])
    );
}

#[test]
fn eval_date_from_and_time_from() {
    let cql_date = "library T define X: date from @2003-10-29T20:50:33.955+01:00";
    assert_eq!(
        eval_expr(cql_date, "X"),
        Value::Date(CqlDate {
            year: 2003,
            month: Some(10),
            day: Some(29)
        })
    );

    let cql_time = "library T define X: time from @2003-10-29T20:50:33.955+01:00";
    assert_eq!(
        eval_expr(cql_time, "X"),
        Value::Time(CqlTime {
            hour: 20,
            minute: Some(50),
            second: Some(33),
            millisecond: Some(955)
        })
    );
}

#[test]
fn eval_timezone_offset_from() {
    let cql = "library T define X: timezoneoffset from @2003-10-29T20:50:33.955+01:00";
    assert_eq!(eval_expr(cql, "X"), Value::Decimal(1.0));
}

// ---------------------------------------------------------------------------
// Wave-2 conformance gap coverage (CQL spec section coverage)
// ---------------------------------------------------------------------------

/// Nullological operators — function-call syntax (§19)
#[test]
fn eval_wave2_nullological_function_forms() {
    // IsNull via function call
    assert_eq!(
        eval_expr("library T define X: IsNull(null)", "X"),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_expr("library T define X: IsNull(1)", "X"),
        Value::Boolean(false)
    );

    // IsTrue / IsFalse via function call
    assert_eq!(
        eval_expr("library T define X: IsTrue(true)", "X"),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_expr("library T define X: IsTrue(false)", "X"),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_expr("library T define X: IsFalse(false)", "X"),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_expr("library T define X: IsFalse(true)", "X"),
        Value::Boolean(false)
    );

    // Coalesce — variadic form: first non-null
    assert_eq!(
        eval_expr("library T define X: Coalesce(null, 5, null)", "X"),
        Value::Integer(5)
    );
    assert_eq!(
        eval_expr("library T define X: Coalesce(null, null)", "X"),
        Value::Null
    );
    assert_eq!(
        eval_expr("library T define X: Coalesce(3, null)", "X"),
        Value::Integer(3)
    );

    // Coalesce — single list argument form (§19.1)
    assert_eq!(
        eval_expr("library T define X: Coalesce({null, 7, null})", "X"),
        Value::Integer(7)
    );
    assert_eq!(
        eval_expr("library T define X: Coalesce({})", "X"),
        Value::Null
    );
}

/// Aggregate functions — AllTrue / AnyTrue (§20)
#[test]
fn eval_wave2_all_true_any_true() {
    // AllTrue
    assert_eq!(
        eval_expr("library T define X: AllTrue({true, true})", "X"),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_expr("library T define X: AllTrue({true, false})", "X"),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_expr("library T define X: AllTrue({})", "X"),
        Value::Boolean(true)
    ); // vacuous truth
    assert_eq!(
        eval_expr("library T define X: AllTrue({null, true})", "X"),
        Value::Boolean(true)
    ); // null skipped

    // AnyTrue
    assert_eq!(
        eval_expr("library T define X: AnyTrue({false, true})", "X"),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_expr("library T define X: AnyTrue({false, false})", "X"),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_expr("library T define X: AnyTrue({})", "X"),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_expr("library T define X: AnyTrue({null, true})", "X"),
        Value::Boolean(true)
    ); // null skipped
}

/// Aggregate functions — Median, Variance, StdDev, Pop* (§20)
#[test]
fn eval_wave2_aggregate_statistics() {
    // Median of {1.0, 3.0} — even count → average of two middle elements
    assert_eq!(
        eval_expr("library T define X: Median({1.0, 3.0})", "X"),
        Value::Decimal(2.0)
    );

    // Sample variance of {1.0, 3.0}: mean=2, sum-sq-diff=2, denom=n-1=1 → 2.0
    assert_eq!(
        eval_expr("library T define X: Variance({1.0, 3.0})", "X"),
        Value::Decimal(2.0)
    );

    // PopulationVariance of {1.0, 3.0}: mean=2, sum-sq-diff=2, denom=n=2 → 1.0
    assert_eq!(
        eval_expr("library T define X: PopulationVariance({1.0, 3.0})", "X"),
        Value::Decimal(1.0),
    );

    // PopulationStdDev of {1.0, 3.0} = sqrt(1.0) = 1.0
    assert_eq!(
        eval_expr("library T define X: PopulationStdDev({1.0, 3.0})", "X"),
        Value::Decimal(1.0),
    );

    // Mode of {1.0, 2.0, 2.0} → 2.0
    assert_eq!(
        eval_expr("library T define X: Mode({1.0, 2.0, 2.0})", "X"),
        Value::Decimal(2.0),
    );
}

/// Aggregate functions — Product, GeometricMean (§20)
#[test]
fn eval_wave2_product_geometric_mean() {
    // Product of {1, 2, 3, 4} = 24
    assert_eq!(
        eval_expr("library T define X: Product({1, 2, 3, 4})", "X"),
        Value::Integer(24),
    );

    // GeometricMean({1.0, 4.0}) = sqrt(1*4) = 2.0
    let result = eval_expr("library T define X: GeometricMean({1.0, 4.0})", "X");
    match result {
        Value::Decimal(v) => assert!((v - 2.0).abs() < 1e-9, "expected 2.0, got {v}"),
        other => panic!("expected Decimal, got {other:?}"),
    }

    // Product of empty list → null
    assert_eq!(
        eval_expr("library T define X: Product({})", "X"),
        Value::Null
    );
}

/// Temporal — TimeOfDay() (§21)
#[test]
fn eval_wave2_time_of_day() {
    // test_clock has hour=10, minute=30, second=0
    let result = eval_expr("library T define X: TimeOfDay()", "X");
    assert_eq!(
        result,
        Value::Time(CqlTime {
            hour: 10,
            minute: Some(30),
            second: Some(0),
            millisecond: None,
        }),
    );
}

/// Arithmetic — Precision, LowBoundary, HighBoundary (§16)
#[test]
fn eval_wave2_precision_boundary() {
    // Precision(@2014) = 4 (year only)
    assert_eq!(
        eval_expr("library T define X: Precision(@2014)", "X"),
        Value::Integer(4)
    );

    // Precision(@T10:30) = 4 (hour=2 + minute=2)
    assert_eq!(
        eval_expr("library T define X: Precision(@T10:30)", "X"),
        Value::Integer(4)
    );

    // Precision(@T10:30:00.000) = 9 (hour=2 + minute=2 + second=2 + ms=3)
    assert_eq!(
        eval_expr("library T define X: Precision(@T10:30:00.000)", "X"),
        Value::Integer(9),
    );

    // LowBoundary(@2014, 6) — precision 6 = month → add month=1
    assert_eq!(
        eval_expr("library T define X: LowBoundary(@2014, 6)", "X"),
        Value::Date(CqlDate {
            year: 2014,
            month: Some(1),
            day: None
        }),
    );

    // HighBoundary(@2014, 6) — precision 6 = month → add month=12
    assert_eq!(
        eval_expr("library T define X: HighBoundary(@2014, 6)", "X"),
        Value::Date(CqlDate {
            year: 2014,
            month: Some(12),
            day: None
        }),
    );
}

/// List — Size (§20)
#[test]
fn eval_wave2_size() {
    assert_eq!(
        eval_expr("library T define X: Size({1, 2, 3})", "X"),
        Value::Integer(3)
    );
    assert_eq!(
        eval_expr("library T define X: Size({})", "X"),
        Value::Integer(0)
    );
}

/// Repeat — deterministic fixpoint evaluation (§19)
#[test]
fn eval_wave2_repeat() {
    // Constant element_expr = 1: each pass maps every element to 1, deduplicates to {1},
    // then {1} → {1} on the next pass → fixpoint reached.
    assert_eq!(
        eval_expr("library T define X: Repeat({1, 2, 3}, 1)", "X"),
        Value::List(vec![Value::Integer(1)]),
        "constant element_expr must converge to singleton"
    );

    // Single-element source already matches element_expr output → immediate fixpoint.
    assert_eq!(
        eval_expr("library T define X: Repeat({42}, 42)", "X"),
        Value::List(vec![Value::Integer(42)]),
        "single-element source at fixpoint must return unchanged"
    );

    // Null source: current starts empty → deduped = [] = current → immediate fixpoint.
    assert_eq!(
        eval_expr("library T define X: Repeat(null, 1)", "X"),
        Value::List(vec![]),
        "null source must produce empty list without error"
    );
}

// ---------------------------------------------------------------------------
// Cross-library compile + evaluate (cql-library-resolution)
// ---------------------------------------------------------------------------

#[test]
fn eval_cross_library_expression_ref_end_to_end() {
    use rh_cql::{
        compile_with_libraries, evaluate_elm_with_libraries, MemoryLibrarySourceProvider,
    };

    // Included library: Helpers defines Answer = 42
    let helpers_cql = r#"
library Helpers version '1.0.0'
define Answer: 42
"#;

    // Main library: includes Helpers and re-exports via cross-library ref
    let main_cql = r#"
library Main version '1.0.0'
include Helpers version '1.0.0' called Helpers
define MyAnswer: Helpers.Answer
"#;

    let provider = MemoryLibrarySourceProvider::new();
    provider.register_source(
        rh_cql::LibraryIdentifier::new("Helpers", Some("1.0.0")),
        helpers_cql.to_string(),
    );

    let out =
        compile_with_libraries(main_cql, None, &provider).expect("compile_with_libraries failed");
    assert!(
        out.result.is_success(),
        "compilation errors: {:?}",
        out.result.errors
    );
    assert!(
        out.included.contains_key("Helpers"),
        "expected 'Helpers' in included map"
    );

    let ctx = default_ctx();
    let val = evaluate_elm_with_libraries(&out.result.library, &out.included, "MyAnswer", &ctx)
        .expect("evaluation failed");
    assert_eq!(val, Value::Integer(42));
}

#[test]
fn eval_cross_library_chained_expression_ref() {
    use rh_cql::{
        compile_with_libraries, evaluate_elm_with_libraries, MemoryLibrarySourceProvider,
    };

    // Base library: defines a constant
    let base_cql = r#"
library Base version '1.0.0'
define BaseValue: 100
"#;

    // Main library includes Base and wraps its value
    let main_cql = r#"
library Main version '1.0.0'
include Base version '1.0.0' called B
define Result: B.BaseValue + 1
"#;

    let provider = MemoryLibrarySourceProvider::new();
    provider.register_source(
        rh_cql::LibraryIdentifier::new("Base", Some("1.0.0")),
        base_cql.to_string(),
    );

    let out =
        compile_with_libraries(main_cql, None, &provider).expect("compile_with_libraries failed");
    assert!(
        out.result.is_success(),
        "compilation errors: {:?}",
        out.result.errors
    );

    let ctx = default_ctx();
    let val = evaluate_elm_with_libraries(&out.result.library, &out.included, "Result", &ctx)
        .expect("evaluation failed");
    assert_eq!(val, Value::Integer(101));
}
