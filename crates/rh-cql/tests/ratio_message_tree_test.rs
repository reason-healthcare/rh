//! Tests for refactor-plan task 2.5 operators: Ratio literals, `ToRatio`,
//! 1-arg `Combine`, `Message`, and `Children`/`Descendants`.

use rh_cql::{
    compile_with_model, evaluate_elm, CqlDateTime, CqlQuantity, EvalContextBuilder, FixedClock,
    Value,
};

fn eval(cql: &str, name: &str) -> Result<Value, String> {
    let result = compile_with_model(cql, None, None).map_err(|e| format!("{e:?}"))?;
    if !result.errors.is_empty() {
        return Err(format!("{:?}", result.errors));
    }
    let clock = FixedClock::new(CqlDateTime {
        year: 2024,
        month: Some(6),
        day: Some(15),
        hour: Some(10),
        minute: Some(30),
        second: Some(0),
        millisecond: None,
        offset_seconds: None,
    });
    let ctx = EvalContextBuilder::new(clock).build();
    evaluate_elm(&result.library, name, &ctx).map_err(|e| format!("{e:?}"))
}

fn quantity(value: f64, unit: &str) -> CqlQuantity {
    CqlQuantity {
        value,
        unit: unit.to_string(),
    }
}

#[test]
fn ratio_literal_with_quantities() {
    assert_eq!(
        eval("library T define X: 1 'mg' : 2 'mL'", "X").unwrap(),
        Value::Ratio {
            numerator: quantity(1.0, "mg"),
            denominator: quantity(2.0, "mL"),
        }
    );
}

#[test]
fn ratio_literal_with_plain_numbers() {
    assert_eq!(
        eval("library T define X: 1:128", "X").unwrap(),
        Value::Ratio {
            numerator: quantity(1.0, "1"),
            denominator: quantity(128.0, "1"),
        }
    );
}

#[test]
fn to_ratio_parses_quantity_pair_string() {
    assert_eq!(
        eval(
            r#"library T define X: ToRatio('1.0 \'mg\':2.0 \'mL\'')"#,
            "X"
        )
        .unwrap(),
        Value::Ratio {
            numerator: quantity(1.0, "mg"),
            denominator: quantity(2.0, "mL"),
        }
    );
}

#[test]
fn to_ratio_malformed_string_is_null() {
    assert_eq!(
        eval("library T define X: ToRatio('not a ratio')", "X").unwrap(),
        Value::Null
    );
}

#[test]
fn combine_without_separator() {
    assert_eq!(
        eval("library T define X: Combine({'a','b','c'})", "X").unwrap(),
        Value::String("abc".to_string())
    );
}

#[test]
fn message_returns_source_when_condition_false() {
    assert_eq!(
        eval(
            "library T define X: Message(42, false, 'c', 'Error', 'nope')",
            "X"
        )
        .unwrap(),
        Value::Integer(42)
    );
}

#[test]
fn message_returns_source_for_trace_severity() {
    assert_eq!(
        eval(
            "library T define X: Message(42, true, 'c', 'Trace', 'traced')",
            "X"
        )
        .unwrap(),
        Value::Integer(42)
    );
}

#[test]
fn message_error_severity_raises() {
    let err = eval(
        "library T define X: Message(42, true, 'E100', 'Error', 'boom')",
        "X",
    )
    .unwrap_err();
    assert!(err.contains("E100") && err.contains("boom"), "{err}");
}

#[test]
fn children_of_tuple_returns_field_values() {
    let result = eval(
        "library T define X: Children(Tuple { a: 1, b: 'two' })",
        "X",
    )
    .unwrap();
    match result {
        Value::List(items) => {
            assert_eq!(items.len(), 2);
            assert!(items.contains(&Value::Integer(1)));
            assert!(items.contains(&Value::String("two".to_string())));
        }
        other => panic!("expected list, got {other:?}"),
    }
}

#[test]
fn descendants_recurses_into_nested_tuples() {
    let result = eval(
        "library T define X: Descendants(Tuple { a: Tuple { b: 1 } })",
        "X",
    )
    .unwrap();
    match result {
        Value::List(items) => {
            assert!(
                items.contains(&Value::Integer(1)),
                "nested leaf missing: {items:?}"
            );
            assert_eq!(items.len(), 2, "inner tuple + leaf: {items:?}");
        }
        other => panic!("expected list, got {other:?}"),
    }
}

#[test]
fn quantity_to_string_matches_hl7_suite_format() {
    assert_eq!(
        eval("library T define X: ToString(125 'cm')", "X").unwrap(),
        Value::String("125cm".to_string())
    );
}
