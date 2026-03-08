//! CQL operator implementations, split across domain submodules.
//!
//! All public items are re-exported from this module so that callers can
//! continue to write `use crate::eval::operators::add;` etc.

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod string_ops;
pub mod temporal;

pub use arithmetic::*;
pub use comparison::*;
pub use conversion::*;
pub use string_ops::*;
pub use temporal::*;

use super::super::elm;
use super::context::EvalError;
use super::value::Value;

// ---------------------------------------------------------------------------
// ELM namespace utilities (shared with the engine)
// ---------------------------------------------------------------------------

/// Strip the XML/ELM Clark-notation namespace prefix from a qualified type name.
///
/// e.g. `"{urn:hl7-org:elm-types:r1}Integer"` → `"Integer"`.
pub(crate) fn strip_elm_namespace(raw: &str) -> &str {
    if let Some(pos) = raw.rfind('}') {
        &raw[pos + 1..]
    } else {
        raw
    }
}

// ---------------------------------------------------------------------------
// Literal evaluation (shared with the engine)
// ---------------------------------------------------------------------------

/// Parse an ELM [`Literal`] node into a runtime [`Value`].
pub(crate) fn eval_literal(lit: &elm::Literal) -> Result<Value, EvalError> {
    let value_str = lit.value.as_deref().unwrap_or("");
    let raw_type = lit.value_type.as_deref().unwrap_or("");
    let value_type = strip_elm_namespace(raw_type);

    match value_type {
        "Boolean" => Ok(Value::Boolean(value_str == "true")),
        "Integer" => value_str
            .parse::<i64>()
            .map(Value::Integer)
            .map_err(|_| EvalError::General(format!("Invalid Integer literal: '{value_str}'"))),
        "Long" => value_str
            .trim_end_matches('L')
            .parse::<i128>()
            .map(Value::Long)
            .map_err(|_| EvalError::General(format!("Invalid Long literal: '{value_str}'"))),
        "Decimal" => value_str
            .parse::<f64>()
            .map(Value::Decimal)
            .map_err(|_| EvalError::General(format!("Invalid Decimal literal: '{value_str}'"))),
        "String" => Ok(Value::String(value_str.to_string())),
        "" if value_str.is_empty() => Ok(Value::Null),
        _ => Ok(Value::String(value_str.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Built-in function dispatch (shared with the engine)
// ---------------------------------------------------------------------------

/// Evaluate a built-in CQL system function referenced by name.
///
/// These are functions that may be emitted as `FunctionRef` by the CQL → ELM
/// compiler (e.g. `Count`, `Sum`, `ToString`, `ToInteger`).
pub(crate) fn eval_builtin_function(name: &str, args: Vec<Value>) -> Result<Value, EvalError> {
    match (name, args.as_slice()) {
        // Aggregate
        ("Count", [list]) => super::lists::count(list),
        ("Sum", [list]) => super::lists::sum(list),
        ("Min", [list]) => super::lists::min(list),
        ("Max", [list]) => super::lists::max(list),
        ("Avg", [list]) => super::lists::avg(list),
        // Type conversions
        ("ToString", [v]) => to_string(v),
        ("ToInteger", [v]) => to_integer(v),
        ("ToLong", [v]) => to_long(v),
        ("ToDecimal", [v]) => to_decimal(v),
        ("ToBoolean", [v]) => to_boolean(v),
        ("ToDate", [v]) => to_date(v),
        ("ToDateTime", [v]) => to_datetime(v),
        ("ToTime", [v]) => to_time(v),
        ("ToQuantity", [v]) => to_quantity(v),
        ("ToConcept", [v]) => to_concept(v),
        _ => Err(EvalError::General(format!(
            "evaluate_elm: unknown FunctionRef '{name}' with {} arg(s)",
            args.len()
        ))),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::super::value::{CqlDate, CqlDateTime, CqlQuantity, CqlTime};
    use super::*;

    // ---- Arithmetic --------------------------------------------------------

    #[test]
    fn add_integers() {
        assert_eq!(
            add(&Value::Integer(3), &Value::Integer(4)).unwrap(),
            Value::Integer(7)
        );
    }

    #[test]
    fn add_null_propagates() {
        assert_eq!(add(&Value::Null, &Value::Integer(1)).unwrap(), Value::Null);
        assert_eq!(add(&Value::Integer(1), &Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn subtract_decimals() {
        assert_eq!(
            subtract(&Value::Decimal(5.5), &Value::Decimal(2.5)).unwrap(),
            Value::Decimal(3.0)
        );
    }

    #[test]
    fn multiply_long() {
        assert_eq!(
            multiply(&Value::Long(100), &Value::Long(200)).unwrap(),
            Value::Long(20000)
        );
    }

    #[test]
    fn divide_integers_yields_decimal() {
        assert_eq!(
            divide(&Value::Integer(7), &Value::Integer(2)).unwrap(),
            Value::Decimal(3.5)
        );
    }

    #[test]
    fn divide_by_zero_yields_null() {
        assert_eq!(
            divide(&Value::Integer(5), &Value::Integer(0)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn modulo_integer() {
        assert_eq!(
            modulo(&Value::Integer(10), &Value::Integer(3)).unwrap(),
            Value::Integer(1)
        );
    }

    #[test]
    fn modulo_by_zero_yields_null() {
        assert_eq!(
            modulo(&Value::Integer(5), &Value::Integer(0)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn negate_decimal() {
        assert_eq!(
            negate(&Value::Decimal(1.25)).unwrap(),
            Value::Decimal(-1.25)
        );
    }

    #[test]
    fn abs_negative_integer() {
        assert_eq!(abs(&Value::Integer(-7)).unwrap(), Value::Integer(7));
    }

    #[test]
    fn ceiling_decimal() {
        assert_eq!(ceiling(&Value::Decimal(1.1)).unwrap(), Value::Integer(2));
        assert_eq!(ceiling(&Value::Decimal(-1.1)).unwrap(), Value::Integer(-1));
    }

    #[test]
    fn floor_decimal() {
        assert_eq!(floor(&Value::Decimal(1.9)).unwrap(), Value::Integer(1));
        assert_eq!(floor(&Value::Decimal(-1.1)).unwrap(), Value::Integer(-2));
    }

    #[test]
    fn truncate_decimal() {
        assert_eq!(truncate(&Value::Decimal(3.9)).unwrap(), Value::Integer(3));
        assert_eq!(truncate(&Value::Decimal(-3.9)).unwrap(), Value::Integer(-3));
    }

    #[test]
    fn round_default_precision() {
        assert_eq!(
            round(&Value::Decimal(3.567), None).unwrap(),
            Value::Decimal(4.0)
        );
    }

    #[test]
    fn round_with_precision() {
        assert_eq!(
            round(&Value::Decimal(3.567), Some(&Value::Integer(2))).unwrap(),
            Value::Decimal(3.57)
        );
    }

    #[test]
    fn power_integer_base() {
        assert_eq!(
            power(&Value::Integer(2), &Value::Integer(10)).unwrap(),
            Value::Decimal(1024.0)
        );
    }

    #[test]
    fn ln_positive() {
        let result = ln(&Value::Decimal(std::f64::consts::E)).unwrap();
        if let Value::Decimal(x) = result {
            assert!((x - 1.0).abs() < 1e-10);
        } else {
            panic!("expected Decimal");
        }
    }

    #[test]
    fn ln_non_positive_yields_null() {
        assert_eq!(ln(&Value::Decimal(0.0)).unwrap(), Value::Null);
        assert_eq!(ln(&Value::Decimal(-1.0)).unwrap(), Value::Null);
    }

    #[test]
    fn exp_zero_is_one() {
        assert_eq!(exp(&Value::Decimal(0.0)).unwrap(), Value::Decimal(1.0));
    }

    #[test]
    fn log_base10() {
        let result = log(&Value::Decimal(1000.0), &Value::Decimal(10.0)).unwrap();
        if let Value::Decimal(x) = result {
            assert!((x - 3.0).abs() < 1e-10);
        } else {
            panic!("expected Decimal");
        }
    }

    #[test]
    fn predecessor_integer() {
        assert_eq!(predecessor(&Value::Integer(5)).unwrap(), Value::Integer(4));
    }

    #[test]
    fn predecessor_integer_min_yields_null() {
        assert_eq!(
            predecessor(&Value::Integer(i32::MIN as i64)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn successor_integer() {
        assert_eq!(successor(&Value::Integer(5)).unwrap(), Value::Integer(6));
    }

    #[test]
    fn successor_integer_max_yields_null() {
        assert_eq!(
            successor(&Value::Integer(i32::MAX as i64)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn min_max_value_integer() {
        assert_eq!(
            min_value("Integer").unwrap(),
            Value::Integer(i32::MIN as i64)
        );
        assert_eq!(
            max_value("Integer").unwrap(),
            Value::Integer(i32::MAX as i64)
        );
    }

    #[test]
    fn min_max_value_long() {
        assert_eq!(min_value("Long").unwrap(), Value::Long(i64::MIN as i128));
        assert_eq!(max_value("Long").unwrap(), Value::Long(i64::MAX as i128));
    }

    #[test]
    fn min_max_value_unknown_type_errors() {
        assert!(min_value("Foo").is_err());
        assert!(max_value("Bar").is_err());
    }

    // ---- Comparison --------------------------------------------------------

    #[test]
    fn less_integers() {
        assert_eq!(
            less(&Value::Integer(1), &Value::Integer(2)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            less(&Value::Integer(2), &Value::Integer(2)).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            less(&Value::Integer(3), &Value::Integer(2)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn greater_decimals() {
        assert_eq!(
            greater(&Value::Decimal(3.5), &Value::Decimal(2.0)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            greater(&Value::Decimal(1.0), &Value::Decimal(2.0)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn less_or_equal_strings() {
        assert_eq!(
            less_or_equal(&Value::String("abc".into()), &Value::String("abd".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            less_or_equal(&Value::String("abc".into()), &Value::String("abc".into())).unwrap(),
            Value::Boolean(true)
        );
    }

    #[test]
    fn greater_or_equal_null_propagates() {
        assert_eq!(
            greater_or_equal(&Value::Null, &Value::Integer(1)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn equal_delegates_to_cql_equal() {
        assert_eq!(
            equal(&Value::Integer(5), &Value::Integer(5)),
            Value::Boolean(true)
        );
        assert_eq!(equal(&Value::Null, &Value::Integer(5)), Value::Null);
    }

    #[test]
    fn equivalent_null_safe() {
        assert_eq!(equivalent(&Value::Null, &Value::Null), Value::Boolean(true));
        assert_eq!(
            equivalent(&Value::Null, &Value::Integer(1)),
            Value::Boolean(false)
        );
    }

    #[test]
    fn compare_different_types_yields_null() {
        // Integer vs Decimal are different variant arms — not comparable, returns null
        assert_eq!(
            less(&Value::Integer(1), &Value::Decimal(2.0)).unwrap(),
            Value::Null
        );
    }

    // ---- Temporal comparison (S1) ------------------------------------------

    #[test]
    fn less_date_values() {
        let earlier = Value::Date(CqlDate {
            year: 2020,
            month: Some(1),
            day: Some(1),
        });
        let later = Value::Date(CqlDate {
            year: 2020,
            month: Some(6),
            day: Some(15),
        });
        assert_eq!(less(&earlier, &later).unwrap(), Value::Boolean(true));
        assert_eq!(less(&later, &earlier).unwrap(), Value::Boolean(false));
        assert_eq!(less(&earlier, &earlier).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn greater_datetime_values() {
        let dt1 = Value::DateTime(CqlDateTime {
            year: 2023,
            month: Some(3),
            day: Some(10),
            hour: Some(12),
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: None,
        });
        let dt2 = Value::DateTime(CqlDateTime {
            year: 2023,
            month: Some(3),
            day: Some(10),
            hour: Some(8),
            minute: Some(30),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: None,
        });
        assert_eq!(greater(&dt1, &dt2).unwrap(), Value::Boolean(true));
        assert_eq!(greater(&dt2, &dt1).unwrap(), Value::Boolean(false));
    }

    // ---- Temporal predecessor / successor (W2) ------------------------------

    #[test]
    fn predecessor_date_day_precision() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(3),
            day: Some(1),
        });
        let expected = Value::Date(CqlDate {
            year: 2023,
            month: Some(2),
            day: Some(28),
        });
        assert_eq!(predecessor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_date_crosses_year() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        let expected = Value::Date(CqlDate {
            year: 2022,
            month: Some(12),
            day: Some(31),
        });
        assert_eq!(predecessor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_date_min_yields_null() {
        let d = Value::Date(CqlDate {
            year: 1,
            month: Some(1),
            day: Some(1),
        });
        assert_eq!(predecessor(&d).unwrap(), Value::Null);
    }

    #[test]
    fn successor_date_month_end() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(31),
        });
        let expected = Value::Date(CqlDate {
            year: 2023,
            month: Some(2),
            day: Some(1),
        });
        assert_eq!(successor(&d).unwrap(), expected);
    }

    #[test]
    fn successor_date_leap_year() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: Some(2),
            day: Some(28),
        });
        let expected = Value::Date(CqlDate {
            year: 2024,
            month: Some(2),
            day: Some(29),
        });
        assert_eq!(successor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_time_millisecond() {
        let t = Value::Time(CqlTime {
            hour: 10,
            minute: Some(30),
            second: Some(0),
            millisecond: Some(0),
        });
        let expected = Value::Time(CqlTime {
            hour: 10,
            minute: Some(29),
            second: Some(59),
            millisecond: Some(999),
        });
        assert_eq!(predecessor(&t).unwrap(), expected);
    }

    #[test]
    fn successor_time_millisecond() {
        let t = Value::Time(CqlTime {
            hour: 10,
            minute: Some(30),
            second: Some(0),
            millisecond: Some(999),
        });
        let expected = Value::Time(CqlTime {
            hour: 10,
            minute: Some(30),
            second: Some(1),
            millisecond: Some(0),
        });
        assert_eq!(successor(&t).unwrap(), expected);
    }

    #[test]
    fn predecessor_datetime_millisecond() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2023,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: None,
        });
        let expected = Value::DateTime(CqlDateTime {
            year: 2022,
            month: Some(12),
            day: Some(31),
            hour: Some(23),
            minute: Some(59),
            second: Some(59),
            millisecond: Some(999),
            offset_seconds: None,
        });
        assert_eq!(predecessor(&dt).unwrap(), expected);
    }

    #[test]
    fn successor_datetime_millisecond() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2023,
            month: Some(12),
            day: Some(31),
            hour: Some(23),
            minute: Some(59),
            second: Some(59),
            millisecond: Some(999),
            offset_seconds: None,
        });
        let expected = Value::DateTime(CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: None,
        });
        assert_eq!(successor(&dt).unwrap(), expected);
    }

    // ---- MinValue / MaxValue for temporal types (W1) -----------------------

    #[test]
    fn min_max_value_date() {
        assert_eq!(
            min_value("Date").unwrap(),
            Value::Date(CqlDate {
                year: 1,
                month: Some(1),
                day: Some(1)
            })
        );
        assert_eq!(
            max_value("Date").unwrap(),
            Value::Date(CqlDate {
                year: 9999,
                month: Some(12),
                day: Some(31)
            })
        );
    }

    #[test]
    fn min_max_value_time() {
        assert_eq!(
            min_value("Time").unwrap(),
            Value::Time(CqlTime {
                hour: 0,
                minute: Some(0),
                second: Some(0),
                millisecond: Some(0)
            })
        );
        assert_eq!(
            max_value("Time").unwrap(),
            Value::Time(CqlTime {
                hour: 23,
                minute: Some(59),
                second: Some(59),
                millisecond: Some(999)
            })
        );
    }

    #[test]
    fn min_max_value_datetime() {
        let min_dt = min_value("DateTime").unwrap();
        let max_dt = max_value("DateTime").unwrap();
        assert!(
            matches!(min_dt, Value::DateTime(ref d) if d.year == 1 && d.millisecond == Some(0))
        );
        assert!(
            matches!(max_dt, Value::DateTime(ref d) if d.year == 9999 && d.millisecond == Some(999))
        );
    }

    // ---- String operators (9.12) --------------------------------------------

    #[test]
    fn concatenate_two_strings() {
        let a = Value::String("Hello, ".into());
        let b = Value::String("World!".into());
        assert_eq!(
            concatenate(&a, &b).unwrap(),
            Value::String("Hello, World!".into())
        );
    }

    #[test]
    fn concatenate_null_propagates() {
        assert_eq!(
            concatenate(&Value::Null, &Value::String("x".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn combine_list_with_separator() {
        let list = Value::List(vec![
            Value::String("a".into()),
            Value::String("b".into()),
            Value::String("c".into()),
        ]);
        assert_eq!(
            combine(&list, Some(&Value::String(", ".into()))).unwrap(),
            Value::String("a, b, c".into())
        );
    }

    #[test]
    fn combine_skips_nulls() {
        let list = Value::List(vec![
            Value::String("a".into()),
            Value::Null,
            Value::String("c".into()),
        ]);
        assert_eq!(
            combine(&list, Some(&Value::String("-".into()))).unwrap(),
            Value::String("a-c".into())
        );
    }

    #[test]
    fn combine_null_separator_means_empty_string() {
        let list = Value::List(vec![Value::String("x".into()), Value::String("y".into())]);
        assert_eq!(combine(&list, None).unwrap(), Value::String("xy".into()));
    }

    #[test]
    fn split_basic() {
        let result = split(&Value::String("a,b,c".into()), &Value::String(",".into())).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".into()),
                Value::String("b".into()),
                Value::String("c".into()),
            ])
        );
    }

    #[test]
    fn length_str_unicode() {
        // "café" has 4 Unicode characters
        assert_eq!(
            length_str(&Value::String("café".into())).unwrap(),
            Value::Integer(4)
        );
    }

    #[test]
    fn upper_lower_round_trip() {
        let s = Value::String("Hello World".into());
        assert_eq!(upper(&s).unwrap(), Value::String("HELLO WORLD".into()));
        assert_eq!(lower(&s).unwrap(), Value::String("hello world".into()));
    }

    #[test]
    fn starts_ends_with() {
        let s = Value::String("foobar".into());
        assert_eq!(
            starts_with(&s, &Value::String("foo".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            starts_with(&s, &Value::String("bar".into())).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            ends_with(&s, &Value::String("bar".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            ends_with(&s, &Value::String("foo".into())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn matches_regex_full_match() {
        let s = Value::String("hello123".into());
        // Must match full string
        assert_eq!(
            matches_regex(&s, &Value::String("[a-z]+\\d+".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            matches_regex(&s, &Value::String("[a-z]+".into())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn replace_matches_substitution() {
        let result = replace_matches(
            &Value::String("foo123bar".into()),
            &Value::String("\\d+".into()),
            &Value::String("NUM".into()),
        )
        .unwrap();
        assert_eq!(result, Value::String("fooNUMbar".into()));
    }

    #[test]
    fn indexer_str_zero_based() {
        let s = Value::String("abc".into());
        assert_eq!(
            indexer_str(&s, &Value::Integer(0)).unwrap(),
            Value::String("a".into())
        );
        assert_eq!(
            indexer_str(&s, &Value::Integer(2)).unwrap(),
            Value::String("c".into())
        );
        assert_eq!(indexer_str(&s, &Value::Integer(5)).unwrap(), Value::Null);
        assert_eq!(indexer_str(&s, &Value::Integer(-1)).unwrap(), Value::Null);
    }

    #[test]
    fn position_of_found_and_not_found() {
        let s = Value::String("abcabc".into());
        let pat = Value::String("bc".into());
        // first occurrence at index 1 (0-based)
        assert_eq!(position_of(&pat, &s).unwrap(), Value::Integer(1));
        // not found: -1
        assert_eq!(
            position_of(&Value::String("xyz".into()), &s).unwrap(),
            Value::Integer(-1)
        );
    }

    #[test]
    fn last_position_of_found() {
        let s = Value::String("abcabc".into());
        let pat = Value::String("bc".into());
        assert_eq!(last_position_of(&pat, &s).unwrap(), Value::Integer(4));
    }

    #[test]
    fn substring_from_start() {
        let s = Value::String("Hello, World!".into());
        // 0-based
        assert_eq!(
            substring(&s, &Value::Integer(7), None).unwrap(),
            Value::String("World!".into())
        );
    }

    #[test]
    fn substring_with_length() {
        let s = Value::String("Hello, World!".into());
        assert_eq!(
            substring(&s, &Value::Integer(7), Some(&Value::Integer(5))).unwrap(),
            Value::String("World".into())
        );
    }

    #[test]
    fn substring_out_of_bounds_null() {
        let s = Value::String("Hi".into());
        assert_eq!(
            substring(&s, &Value::Integer(10), None).unwrap(),
            Value::Null
        );
    }

    // ---- Date/Time construction (9.13) -------------------------------------

    #[test]
    fn date_construct_year_only() {
        assert_eq!(
            date_construct(&Value::Integer(2024), None, None).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: None,
                day: None
            })
        );
    }

    #[test]
    fn date_construct_full() {
        assert_eq!(
            date_construct(
                &Value::Integer(2024),
                Some(&Value::Integer(3)),
                Some(&Value::Integer(15))
            )
            .unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(3),
                day: Some(15)
            })
        );
    }

    #[test]
    fn time_construct_partial() {
        assert_eq!(
            time_construct(&Value::Integer(10), Some(&Value::Integer(30)), None, None).unwrap(),
            Value::Time(CqlTime {
                hour: 10,
                minute: Some(30),
                second: None,
                millisecond: None
            })
        );
    }

    #[test]
    fn datetime_construct_basic() {
        let dt = datetime_construct(
            &Value::Integer(2024),
            Some(&Value::Integer(6)),
            Some(&Value::Integer(15)),
            Some(&Value::Integer(12)),
            Some(&Value::Integer(0)),
            Some(&Value::Integer(0)),
            Some(&Value::Integer(0)),
            None,
        )
        .unwrap();
        assert!(
            matches!(dt, Value::DateTime(ref d) if d.year == 2024 && d.month == Some(6) && d.hour == Some(12))
        );
    }

    // ---- Component extraction (9.13) ----------------------------------------

    #[test]
    fn date_component_extraction() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: Some(3),
            day: Some(15),
        });
        assert_eq!(
            date_time_component(&d, "year").unwrap(),
            Value::Integer(2024)
        );
        assert_eq!(date_time_component(&d, "month").unwrap(), Value::Integer(3));
        assert_eq!(date_time_component(&d, "day").unwrap(), Value::Integer(15));
    }

    #[test]
    fn date_component_absent_returns_null() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: None,
            day: None,
        });
        assert_eq!(date_time_component(&d, "month").unwrap(), Value::Null);
        assert_eq!(date_time_component(&d, "day").unwrap(), Value::Null);
    }

    #[test]
    fn time_component_extraction() {
        let t = Value::Time(CqlTime {
            hour: 14,
            minute: Some(30),
            second: Some(45),
            millisecond: Some(500),
        });
        assert_eq!(date_time_component(&t, "hour").unwrap(), Value::Integer(14));
        assert_eq!(
            date_time_component(&t, "minute").unwrap(),
            Value::Integer(30)
        );
        assert_eq!(
            date_time_component(&t, "millisecond").unwrap(),
            Value::Integer(500)
        );
    }

    // ---- Date arithmetic (9.13) -------------------------------------------

    #[test]
    fn date_add_one_year() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(15),
        });
        let qty = Value::Quantity(CqlQuantity {
            value: 1.0,
            unit: "a".into(),
        });
        assert_eq!(
            date_time_add(&d, &qty).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(6),
                day: Some(15)
            })
        );
    }

    #[test]
    fn date_add_months_crosses_year() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(11),
            day: Some(1),
        });
        let qty = Value::Quantity(CqlQuantity {
            value: 3.0,
            unit: "month".into(),
        });
        assert_eq!(
            date_time_add(&d, &qty).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(2),
                day: Some(1)
            })
        );
    }

    #[test]
    fn date_add_days() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(30),
        });
        let qty = Value::Quantity(CqlQuantity {
            value: 5.0,
            unit: "d".into(),
        });
        assert_eq!(
            date_time_add(&d, &qty).unwrap(),
            Value::Date(CqlDate {
                year: 2023,
                month: Some(2),
                day: Some(4)
            })
        );
    }

    #[test]
    fn date_subtract_days() {
        let d = Value::Date(CqlDate {
            year: 2023,
            month: Some(3),
            day: Some(1),
        });
        let qty = Value::Quantity(CqlQuantity {
            value: 1.0,
            unit: "day".into(),
        });
        assert_eq!(
            date_time_subtract(&d, &qty).unwrap(),
            Value::Date(CqlDate {
                year: 2023,
                month: Some(2),
                day: Some(28)
            })
        );
    }

    #[test]
    fn datetime_add_hours_overflow() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2023,
            month: Some(1),
            day: Some(31),
            hour: Some(22),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: None,
        });
        let qty = Value::Quantity(CqlQuantity {
            value: 3.0,
            unit: "h".into(),
        });
        let result = date_time_add(&dt, &qty).unwrap();
        assert!(
            matches!(result, Value::DateTime(ref d) if d.year == 2023 && d.month == Some(2) && d.day == Some(1) && d.hour == Some(1))
        );
    }

    // ---- SameAs / SameOrBefore / SameOrAfter (9.13) ------------------------

    #[test]
    fn same_as_dates() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(15),
        });
        let d2 = d1.clone();
        let d3 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(16),
        });
        assert_eq!(same_as(&d1, &d2, None).unwrap(), Value::Boolean(true));
        assert_eq!(same_as(&d1, &d3, None).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn same_as_at_year_precision() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(12),
            day: Some(31),
        });
        // Same year → true at year precision
        assert_eq!(
            same_as(&d1, &d2, Some("year")).unwrap(),
            Value::Boolean(true)
        );
    }

    #[test]
    fn same_or_before_dates() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(15),
        });
        assert_eq!(
            same_or_before(&d1, &d2, None).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            same_or_before(&d2, &d1, None).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            same_or_before(&d1, &d1, None).unwrap(),
            Value::Boolean(true)
        );
    }

    #[test]
    fn same_or_after_dates() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(15),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(1),
        });
        assert_eq!(same_or_after(&d1, &d2, None).unwrap(), Value::Boolean(true));
        assert_eq!(
            same_or_after(&d2, &d1, None).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn same_as_mixed_precision_returns_null() {
        // year-only compared to full date → uncertain
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: None,
            day: None,
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(1),
        });
        assert_eq!(same_as(&d1, &d2, Some("month")).unwrap(), Value::Null);
    }

    // ---- Duration / Difference between (9.13) ------------------------------

    #[test]
    fn duration_between_years() {
        let d1 = Value::Date(CqlDate {
            year: 2020,
            month: Some(1),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        assert_eq!(
            duration_between(&d1, &d2, "year").unwrap(),
            Value::Integer(3)
        );
    }

    #[test]
    fn duration_between_months() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(4),
            day: Some(1),
        });
        assert_eq!(duration_between(&d1, &d2, "mo").unwrap(), Value::Integer(3));
    }

    #[test]
    fn duration_between_days() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(8),
        });
        assert_eq!(duration_between(&d1, &d2, "d").unwrap(), Value::Integer(7));
    }

    #[test]
    fn duration_between_negative() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(15),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(6),
            day: Some(1),
        });
        assert_eq!(
            duration_between(&d1, &d2, "day").unwrap(),
            Value::Integer(-14)
        );
    }

    // ---- SplitOnMatches (9.12) ---------------------------------------------

    #[test]
    fn split_on_matches_basic() {
        let result = split_on_matches(
            &Value::String("a1b2c3".into()),
            &Value::String("\\d".into()),
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".into()),
                Value::String("b".into()),
                Value::String("c".into()),
                Value::String("".into()),
            ])
        );
    }

    #[test]
    fn split_on_matches_no_match_returns_single_element() {
        let result = split_on_matches(
            &Value::String("hello".into()),
            &Value::String("\\d+".into()),
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![Value::String("hello".into())]));
    }

    #[test]
    fn split_on_matches_null_propagates() {
        assert_eq!(
            split_on_matches(&Value::Null, &Value::String("x".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            split_on_matches(&Value::String("x".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    // ---- DifferenceBetween (9.13) ------------------------------------------

    #[test]
    fn difference_between_years() {
        let d1 = Value::Date(CqlDate {
            year: 2020,
            month: Some(6),
            day: Some(15),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        // difference truncates to whole year components (2023 - 2020 = 3)
        assert_eq!(
            difference_between(&d1, &d2, "year").unwrap(),
            Value::Integer(3)
        );
    }

    #[test]
    fn difference_between_days() {
        let d1 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(1),
        });
        let d2 = Value::Date(CqlDate {
            year: 2023,
            month: Some(1),
            day: Some(8),
        });
        assert_eq!(
            difference_between(&d1, &d2, "day").unwrap(),
            Value::Integer(7)
        );
    }

    #[test]
    fn difference_between_null_propagates() {
        assert_eq!(
            difference_between(
                &Value::Null,
                &Value::Date(CqlDate {
                    year: 2023,
                    month: Some(1),
                    day: Some(1)
                }),
                "day"
            )
            .unwrap(),
            Value::Null
        );
    }

    // ---- Null propagation for string operators (suggestion) ----------------

    #[test]
    fn starts_with_null_propagates() {
        assert_eq!(
            starts_with(&Value::Null, &Value::String("foo".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            starts_with(&Value::String("foo".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn ends_with_null_propagates() {
        assert_eq!(
            ends_with(&Value::Null, &Value::String("bar".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            ends_with(&Value::String("foobar".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn upper_null_propagates() {
        assert_eq!(upper(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn lower_null_propagates() {
        assert_eq!(lower(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn length_str_null_propagates() {
        assert_eq!(length_str(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn split_null_propagates() {
        assert_eq!(
            split(&Value::Null, &Value::String(",".into())).unwrap(),
            Value::Null
        );
        assert_eq!(
            split(&Value::String("a,b".into()), &Value::Null).unwrap(),
            Value::Null
        );
    }

    // ---- Type conversion operators -----------------------------------------

    #[test]
    fn to_boolean_from_string_true() {
        assert_eq!(
            to_boolean(&Value::String("true".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            to_boolean(&Value::String("T".into())).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            to_boolean(&Value::String("yes".into())).unwrap(),
            Value::Boolean(true)
        );
    }

    #[test]
    fn to_boolean_from_string_false() {
        assert_eq!(
            to_boolean(&Value::String("false".into())).unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            to_boolean(&Value::String("no".into())).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn to_boolean_null_propagates() {
        assert_eq!(to_boolean(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn to_boolean_invalid_string_yields_null() {
        assert_eq!(
            to_boolean(&Value::String("maybe".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_integer_from_string() {
        assert_eq!(
            to_integer(&Value::String("42".into())).unwrap(),
            Value::Integer(42)
        );
    }

    #[test]
    fn to_integer_overflow_yields_null() {
        // Long value outside i32 range → null
        assert_eq!(
            to_integer(&Value::Long(i32::MAX as i128 + 1)).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_integer_null_propagates() {
        assert_eq!(to_integer(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn to_decimal_from_integer() {
        assert_eq!(to_decimal(&Value::Integer(5)).unwrap(), Value::Decimal(5.0));
    }

    #[test]
    fn to_decimal_null_propagates() {
        assert_eq!(to_decimal(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn to_string_from_boolean() {
        assert_eq!(
            to_string(&Value::Boolean(true)).unwrap(),
            Value::String("true".into())
        );
        assert_eq!(
            to_string(&Value::Boolean(false)).unwrap(),
            Value::String("false".into())
        );
    }

    #[test]
    fn to_string_from_integer() {
        assert_eq!(
            to_string(&Value::Integer(7)).unwrap(),
            Value::String("7".into())
        );
    }

    #[test]
    fn to_string_null_propagates() {
        assert_eq!(to_string(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn is_type_matching() {
        assert_eq!(is_type(&Value::Integer(1), "Integer"), Value::Boolean(true));
        assert_eq!(
            is_type(&Value::Boolean(true), "Boolean"),
            Value::Boolean(true)
        );
        assert_eq!(
            is_type(&Value::Decimal(1.5), "Decimal"),
            Value::Boolean(true)
        );
    }

    #[test]
    fn is_type_non_matching() {
        assert_eq!(is_type(&Value::Integer(1), "String"), Value::Boolean(false));
        assert_eq!(is_type(&Value::Null, "Integer"), Value::Boolean(false));
    }

    #[test]
    fn as_type_matching_returns_value() {
        assert_eq!(as_type(&Value::Integer(42), "Integer"), Value::Integer(42));
    }

    #[test]
    fn as_type_non_matching_returns_null() {
        assert_eq!(as_type(&Value::Integer(42), "String"), Value::Null);
    }

    #[test]
    fn convert_round_trips_decimal() {
        assert_eq!(
            convert(&Value::Integer(3), "Decimal").unwrap(),
            Value::Decimal(3.0)
        );
    }

    #[test]
    fn convert_round_trips_string() {
        assert_eq!(
            convert(&Value::Integer(10), "String").unwrap(),
            Value::String("10".into())
        );
    }

    #[test]
    fn convert_invalid_type_errors() {
        assert!(convert(&Value::Integer(1), "Foo").is_err());
    }

    #[test]
    fn convert_to_list_wraps_value() {
        assert_eq!(
            convert(&Value::Integer(5), "List").unwrap(),
            Value::List(vec![Value::Integer(5)])
        );
    }

    #[test]
    fn convert_to_interval_errors() {
        assert!(convert(&Value::Integer(1), "Interval").is_err());
    }

    #[test]
    fn to_list_null_yields_empty_list() {
        assert_eq!(to_list(&Value::Null).unwrap(), Value::List(vec![]));
    }

    #[test]
    fn to_list_value_wraps_in_singleton() {
        assert_eq!(
            to_list(&Value::Boolean(true)).unwrap(),
            Value::List(vec![Value::Boolean(true)])
        );
    }

    #[test]
    fn to_list_list_is_identity() {
        let v = Value::List(vec![Value::Integer(1)]);
        assert_eq!(to_list(&v).unwrap(), v);
    }

    // ---- ToLong (9.14) -----------------------------------------------------

    #[test]
    fn to_long_from_integer() {
        assert_eq!(to_long(&Value::Integer(42)).unwrap(), Value::Long(42));
    }

    #[test]
    fn to_long_from_long_identity() {
        assert_eq!(to_long(&Value::Long(999)).unwrap(), Value::Long(999));
    }

    #[test]
    fn to_long_from_decimal_whole_number() {
        assert_eq!(to_long(&Value::Decimal(5.0)).unwrap(), Value::Long(5));
    }

    #[test]
    fn to_long_from_decimal_fractional_yields_null() {
        assert_eq!(to_long(&Value::Decimal(5.5)).unwrap(), Value::Null);
    }

    #[test]
    fn to_long_from_string() {
        assert_eq!(
            to_long(&Value::String("12345678901".into())).unwrap(),
            Value::Long(12345678901)
        );
    }

    #[test]
    fn to_long_from_string_invalid_yields_null() {
        assert_eq!(
            to_long(&Value::String("not-a-number".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_long_from_boolean() {
        assert_eq!(to_long(&Value::Boolean(true)).unwrap(), Value::Long(1));
        assert_eq!(to_long(&Value::Boolean(false)).unwrap(), Value::Long(0));
    }

    #[test]
    fn to_long_null_propagates() {
        assert_eq!(to_long(&Value::Null).unwrap(), Value::Null);
    }

    // ---- ToDate (9.14) -----------------------------------------------------

    #[test]
    fn to_date_from_string_full() {
        assert_eq!(
            to_date(&Value::String("2024-03-15".into())).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(3),
                day: Some(15)
            })
        );
    }

    #[test]
    fn to_date_from_string_year_month() {
        assert_eq!(
            to_date(&Value::String("2024-06".into())).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(6),
                day: None
            })
        );
    }

    #[test]
    fn to_date_from_string_year_only() {
        assert_eq!(
            to_date(&Value::String("2024".into())).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: None,
                day: None
            })
        );
    }

    #[test]
    fn to_date_from_string_invalid_yields_null() {
        assert_eq!(
            to_date(&Value::String("not-a-date".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_date_from_datetime() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2024,
            month: Some(3),
            day: Some(15),
            hour: Some(10),
            minute: None,
            second: None,
            millisecond: None,
            offset_seconds: None,
        });
        assert_eq!(
            to_date(&dt).unwrap(),
            Value::Date(CqlDate {
                year: 2024,
                month: Some(3),
                day: Some(15)
            })
        );
    }

    #[test]
    fn to_date_from_date_identity() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: Some(1),
            day: Some(1),
        });
        assert_eq!(to_date(&d).unwrap(), d);
    }

    #[test]
    fn to_date_null_propagates() {
        assert_eq!(to_date(&Value::Null).unwrap(), Value::Null);
    }

    // ---- ToDateTime (9.14) -------------------------------------------------

    #[test]
    fn to_datetime_from_string_full() {
        let result = to_datetime(&Value::String("2024-03-15T10:30:00".into())).unwrap();
        assert!(
            matches!(result, Value::DateTime(ref dt) if dt.year == 2024 && dt.month == Some(3) && dt.day == Some(15) && dt.hour == Some(10))
        );
    }

    #[test]
    fn to_datetime_from_string_date_only() {
        let result = to_datetime(&Value::String("2024-06-01".into())).unwrap();
        assert!(
            matches!(result, Value::DateTime(ref dt) if dt.year == 2024 && dt.month == Some(6) && dt.day == Some(1) && dt.hour.is_none())
        );
    }

    #[test]
    fn to_datetime_from_string_invalid_yields_null() {
        assert_eq!(
            to_datetime(&Value::String("garbage".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_datetime_from_date() {
        let d = Value::Date(CqlDate {
            year: 2024,
            month: Some(3),
            day: Some(15),
        });
        let result = to_datetime(&d).unwrap();
        assert!(
            matches!(result, Value::DateTime(ref dt) if dt.year == 2024 && dt.month == Some(3) && dt.day == Some(15) && dt.hour.is_none())
        );
    }

    #[test]
    fn to_datetime_from_datetime_identity() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: None,
        });
        assert_eq!(to_datetime(&dt).unwrap(), dt);
    }

    #[test]
    fn to_datetime_null_propagates() {
        assert_eq!(to_datetime(&Value::Null).unwrap(), Value::Null);
    }

    // ---- ToTime (9.14) -----------------------------------------------------

    #[test]
    fn to_time_from_string_full() {
        assert_eq!(
            to_time(&Value::String("14:30:00".into())).unwrap(),
            Value::Time(CqlTime {
                hour: 14,
                minute: Some(30),
                second: Some(0),
                millisecond: None
            })
        );
    }

    #[test]
    fn to_time_from_string_hour_only() {
        assert_eq!(
            to_time(&Value::String("10".into())).unwrap(),
            Value::Time(CqlTime {
                hour: 10,
                minute: None,
                second: None,
                millisecond: None
            })
        );
    }

    #[test]
    fn to_time_from_string_with_milliseconds() {
        assert_eq!(
            to_time(&Value::String("08:15:30.500".into())).unwrap(),
            Value::Time(CqlTime {
                hour: 8,
                minute: Some(15),
                second: Some(30),
                millisecond: Some(500)
            })
        );
    }

    #[test]
    fn to_time_from_t_prefixed_string() {
        // CQL time literals start with T
        assert_eq!(
            to_time(&Value::String("T12:00".into())).unwrap(),
            Value::Time(CqlTime {
                hour: 12,
                minute: Some(0),
                second: None,
                millisecond: None
            })
        );
    }

    #[test]
    fn to_time_from_time_identity() {
        let t = Value::Time(CqlTime {
            hour: 9,
            minute: Some(30),
            second: None,
            millisecond: None,
        });
        assert_eq!(to_time(&t).unwrap(), t);
    }

    #[test]
    fn to_time_null_propagates() {
        assert_eq!(to_time(&Value::Null).unwrap(), Value::Null);
    }

    // ---- ToQuantity (9.14) -------------------------------------------------

    #[test]
    fn to_quantity_from_integer() {
        let result = to_quantity(&Value::Integer(5)).unwrap();
        assert_eq!(
            result,
            Value::Quantity(CqlQuantity {
                value: 5.0,
                unit: "1".to_string()
            })
        );
    }

    #[test]
    fn to_quantity_from_long() {
        let result = to_quantity(&Value::Long(100)).unwrap();
        assert_eq!(
            result,
            Value::Quantity(CqlQuantity {
                value: 100.0,
                unit: "1".to_string()
            })
        );
    }

    #[test]
    fn to_quantity_from_decimal() {
        let result = to_quantity(&Value::Decimal(1.25)).unwrap();
        assert_eq!(
            result,
            Value::Quantity(CqlQuantity {
                value: 1.25,
                unit: "1".to_string()
            })
        );
    }

    #[test]
    fn to_quantity_from_string_with_unit() {
        let result = to_quantity(&Value::String("10.5 'mg'".into())).unwrap();
        assert_eq!(
            result,
            Value::Quantity(CqlQuantity {
                value: 10.5,
                unit: "mg".to_string()
            })
        );
    }

    #[test]
    fn to_quantity_from_string_number_only() {
        let result = to_quantity(&Value::String("42".into())).unwrap();
        assert_eq!(
            result,
            Value::Quantity(CqlQuantity {
                value: 42.0,
                unit: "1".to_string()
            })
        );
    }

    #[test]
    fn to_quantity_from_string_invalid_yields_null() {
        assert_eq!(
            to_quantity(&Value::String("not-a-quantity".into())).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn to_quantity_from_quantity_identity() {
        let q = Value::Quantity(CqlQuantity {
            value: 7.0,
            unit: "kg".to_string(),
        });
        assert_eq!(to_quantity(&q).unwrap(), q);
    }

    #[test]
    fn to_quantity_null_propagates() {
        assert_eq!(to_quantity(&Value::Null).unwrap(), Value::Null);
    }

    // ---- ToConcept (9.14) --------------------------------------------------

    #[test]
    fn to_concept_from_code() {
        let code = super::super::value::CqlCode {
            code: "ABC".to_string(),
            system: "http://example.com".to_string(),
            version: None,
            display: Some("Some display".to_string()),
        };
        let result = to_concept(&Value::Code(code.clone())).unwrap();
        if let Value::Concept(c) = result {
            assert_eq!(c.codes.len(), 1);
            assert_eq!(c.codes[0], code);
            assert_eq!(c.display, Some("Some display".to_string()));
        } else {
            panic!("expected Concept");
        }
    }

    #[test]
    fn to_concept_from_list_of_codes() {
        let code1 = super::super::value::CqlCode {
            code: "A1".to_string(),
            system: "http://example.com".to_string(),
            version: None,
            display: None,
        };
        let code2 = super::super::value::CqlCode {
            code: "A2".to_string(),
            system: "http://example.com".to_string(),
            version: None,
            display: None,
        };
        let list = Value::List(vec![Value::Code(code1.clone()), Value::Code(code2.clone())]);
        let result = to_concept(&list).unwrap();
        if let Value::Concept(c) = result {
            assert_eq!(c.codes.len(), 2);
            assert_eq!(c.codes[0], code1);
            assert_eq!(c.codes[1], code2);
        } else {
            panic!("expected Concept");
        }
    }

    #[test]
    fn to_concept_from_list_with_nulls_skips_nulls() {
        let code = super::super::value::CqlCode {
            code: "X".to_string(),
            system: "http://example.com".to_string(),
            version: None,
            display: None,
        };
        let list = Value::List(vec![Value::Code(code.clone()), Value::Null]);
        let result = to_concept(&list).unwrap();
        if let Value::Concept(c) = result {
            assert_eq!(c.codes.len(), 1);
        } else {
            panic!("expected Concept");
        }
    }

    #[test]
    fn to_concept_from_concept_identity() {
        let concept = super::super::value::CqlConcept {
            codes: vec![],
            display: Some("Test".to_string()),
        };
        let v = Value::Concept(concept.clone());
        let result = to_concept(&v).unwrap();
        assert_eq!(result, Value::Concept(concept));
    }

    #[test]
    fn to_concept_null_propagates() {
        assert_eq!(to_concept(&Value::Null).unwrap(), Value::Null);
    }
}
