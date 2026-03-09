//! CQL type-conversion operators (CQL §22).
//!
//! Exports: [`to_boolean`], [`to_integer`], [`to_long`], [`to_decimal`],
//! [`to_string`], [`to_date`], [`to_datetime`], [`to_time`], [`to_quantity`],
//! [`to_concept`], [`to_list`], [`is_type`], [`as_type`], [`convert`].

use super::super::context::EvalError;
use super::super::value::{CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};
use super::utils::{err, null1};

// ---------------------------------------------------------------------------
// Type conversion operators (CQL §22)
// ---------------------------------------------------------------------------

/// `ToBoolean` — converts String, Integer, Long, or Decimal to Boolean.
///
/// - `"true"` / `"T"` / `"yes"` / `"1"` → `true`
/// - `"false"` / `"F"` / `"no"` / `"0"` → `false`
/// - Integer/Long `1` → `true`, `0` → `false`
/// - `null` propagates.
pub fn to_boolean(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Boolean(b) => Ok(Value::Boolean(*b)),
        Value::String(s) => match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "1" => Ok(Value::Boolean(true)),
            "false" | "f" | "no" | "0" => Ok(Value::Boolean(false)),
            _ => Ok(Value::Null),
        },
        Value::Integer(n) => match n {
            1 => Ok(Value::Boolean(true)),
            0 => Ok(Value::Boolean(false)),
            _ => Ok(Value::Null),
        },
        Value::Long(n) => match n {
            1 => Ok(Value::Boolean(true)),
            0 => Ok(Value::Boolean(false)),
            _ => Ok(Value::Null),
        },
        Value::Decimal(d) => {
            if (*d - 1.0).abs() < f64::EPSILON {
                Ok(Value::Boolean(true))
            } else if d.abs() < f64::EPSILON {
                Ok(Value::Boolean(false))
            } else {
                Ok(Value::Null)
            }
        }
        _ => Err(err("ToBoolean", "cannot convert to Boolean")),
    }
}

/// `ToInteger` — converts String, Long, Decimal, or Boolean to Integer.
///
/// Returns `null` when the string is not a valid integer literal.
pub fn to_integer(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Integer(n) => Ok(Value::Integer(*n)),
        Value::Long(n) => {
            if *n >= i32::MIN as i128 && *n <= i32::MAX as i128 {
                Ok(Value::Integer(*n as i64))
            } else {
                Ok(Value::Null)
            }
        }
        Value::Decimal(d) => {
            if d.fract() == 0.0 && *d >= i32::MIN as f64 && *d <= i32::MAX as f64 {
                Ok(Value::Integer(*d as i64))
            } else {
                Ok(Value::Null)
            }
        }
        Value::String(s) => s
            .trim()
            .parse::<i64>()
            .map(Value::Integer)
            .map_err(|_| err("ToInteger", "invalid integer string"))
            .or(Ok(Value::Null)),
        Value::Boolean(b) => Ok(Value::Integer(if *b { 1 } else { 0 })),
        _ => Err(err("ToInteger", "cannot convert to Integer")),
    }
}

/// `ToLong` — converts String, Integer, Decimal, or Boolean to Long.
pub fn to_long(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Long(n) => Ok(Value::Long(*n)),
        Value::Integer(n) => Ok(Value::Long(*n as i128)),
        Value::Decimal(d) => {
            if d.fract() == 0.0 {
                Ok(Value::Long(*d as i128))
            } else {
                Ok(Value::Null)
            }
        }
        Value::String(s) => s
            .trim()
            .parse::<i128>()
            .map(Value::Long)
            .map_err(|_| err("ToLong", "invalid long string"))
            .or(Ok(Value::Null)),
        Value::Boolean(b) => Ok(Value::Long(if *b { 1 } else { 0 })),
        _ => Err(err("ToLong", "cannot convert to Long")),
    }
}

/// `ToDecimal` — converts String, Integer, Long, or Boolean to Decimal.
pub fn to_decimal(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Decimal(d) => Ok(Value::Decimal(*d)),
        Value::Integer(n) => Ok(Value::Decimal(*n as f64)),
        Value::Long(n) => Ok(Value::Decimal(*n as f64)),
        Value::String(s) => s
            .trim()
            .parse::<f64>()
            .map(Value::Decimal)
            .map_err(|_| err("ToDecimal", "invalid decimal string"))
            .or(Ok(Value::Null)),
        Value::Boolean(b) => Ok(Value::Decimal(if *b { 1.0 } else { 0.0 })),
        _ => Err(err("ToDecimal", "cannot convert to Decimal")),
    }
}

/// `ToString` — converts any scalar CQL value to its string representation.
pub fn to_string(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Boolean(b) => Ok(Value::String(b.to_string())),
        Value::Integer(n) => Ok(Value::String(n.to_string())),
        Value::Long(n) => Ok(Value::String(n.to_string())),
        Value::Decimal(d) => Ok(Value::String(
            format!("{d:.8}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        )),
        Value::Date(d) => Ok(Value::String(d.to_string())),
        Value::DateTime(dt) => Ok(Value::String(dt.to_string())),
        Value::Time(t) => Ok(Value::String(t.to_string())),
        Value::Quantity(q) => Ok(Value::String(q.to_string())),
        _ => Err(err("ToString", "cannot convert to String")),
    }
}

/// `ToDate` — parses a String in `YYYY`, `YYYY-MM`, or `YYYY-MM-DD` format.
///
/// Returns `null` on parse failure per CQL semantics.
pub fn to_date(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Date(d) => Ok(Value::Date(d.clone())),
        Value::DateTime(dt) => Ok(Value::Date(CqlDate {
            year: dt.year,
            month: dt.month,
            day: dt.day,
        })),
        Value::String(s) => {
            let s = s.trim();
            if let Some(d) = parse_date_str(s) {
                Ok(Value::Date(d))
            } else {
                Ok(Value::Null)
            }
        }
        _ => Err(err("ToDate", "cannot convert to Date")),
    }
}

fn parse_date_str(s: &str) -> Option<CqlDate> {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        1 => {
            let year = parts[0].parse::<i32>().ok()?;
            Some(CqlDate {
                year,
                month: None,
                day: None,
            })
        }
        2 => {
            let year = parts[0].parse::<i32>().ok()?;
            let month = parts[1].parse::<u8>().ok()?;
            if !(1..=12).contains(&month) {
                return None;
            }
            Some(CqlDate {
                year,
                month: Some(month),
                day: None,
            })
        }
        3 => {
            let year = parts[0].parse::<i32>().ok()?;
            let month = parts[1].parse::<u8>().ok()?;
            let day = parts[2].parse::<u8>().ok()?;
            if !(1..=12).contains(&month) {
                return None;
            }
            if !(1..=31).contains(&day) {
                return None;
            }
            Some(CqlDate {
                year,
                month: Some(month),
                day: Some(day),
            })
        }
        _ => None,
    }
}

/// `ToDateTime` — parses a String to a CQL DateTime.
///
/// Accepts ISO-8601 style date-time strings. Returns `null` on failure.
pub fn to_datetime(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::DateTime(dt) => Ok(Value::DateTime(dt.clone())),
        Value::Date(d) => Ok(Value::DateTime(CqlDateTime {
            year: d.year,
            month: d.month,
            day: d.day,
            hour: None,
            minute: None,
            second: None,
            millisecond: None,
            offset_seconds: None,
        })),
        Value::String(s) => {
            let s = s.trim();
            if let Some(dt) = parse_datetime_str(s) {
                Ok(Value::DateTime(dt))
            } else {
                Ok(Value::Null)
            }
        }
        _ => Err(err("ToDateTime", "cannot convert to DateTime")),
    }
}

fn parse_datetime_str(s: &str) -> Option<CqlDateTime> {
    // Split on 'T' to separate date and time parts.
    let (date_part, time_part) = if let Some(pos) = s.find('T') {
        (&s[..pos], Some(&s[pos + 1..]))
    } else {
        (s, None)
    };

    let date = parse_date_str(date_part)?;

    let mut hour = None;
    let mut minute = None;
    let mut second = None;
    let mut millisecond = None;
    let mut offset_seconds = None;

    if let Some(tp) = time_part {
        // Strip timezone suffix.
        let (tp, tz) = strip_timezone(tp);
        offset_seconds = tz;

        let time_components: Vec<&str> = tp.split(':').collect();
        if !time_components.is_empty() {
            hour = time_components[0].parse::<u8>().ok();
        }
        if time_components.len() >= 2 {
            minute = time_components[1].parse::<u8>().ok();
        }
        if time_components.len() >= 3 {
            let sec_str = time_components[2];
            if let Some(dot) = sec_str.find('.') {
                second = sec_str[..dot].parse::<u8>().ok();
                millisecond = sec_str[dot + 1..].parse::<u32>().ok();
            } else {
                second = sec_str.parse::<u8>().ok();
            }
        }
    }

    Some(CqlDateTime {
        year: date.year,
        month: date.month,
        day: date.day,
        hour,
        minute,
        second,
        millisecond,
        offset_seconds,
    })
}

fn strip_timezone(s: &str) -> (&str, Option<i32>) {
    if let Some(stripped) = s.strip_suffix('Z') {
        (stripped, Some(0))
    } else if let Some(pos) = s.rfind('+').or_else(|| {
        // find a '-' that isn't at position 0 (to avoid catching '-' in seconds)
        s[1..].rfind('-').map(|p| p + 1)
    }) {
        let tz_str = &s[pos..];
        let sign = if s.as_bytes()[pos] == b'+' {
            1i32
        } else {
            -1i32
        };
        let tz_body = &tz_str[1..];
        let tz_parts: Vec<&str> = tz_body.split(':').collect();
        let offset = if tz_parts.len() == 2 {
            tz_parts[0].parse::<i32>().ok().and_then(|h| {
                tz_parts[1]
                    .parse::<i32>()
                    .ok()
                    .map(|m| sign * (h * 3600 + m * 60))
            })
        } else {
            None
        };
        (&s[..pos], offset)
    } else {
        (s, None)
    }
}

/// `ToTime` — parses a String in `Thh`, `Thh:mm`, `Thh:mm:ss[.ms]` format.
///
/// Returns `null` on parse failure.
pub fn to_time(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Time(t) => Ok(Value::Time(t.clone())),
        Value::String(s) => {
            let s = s.trim().trim_start_matches('T');
            if let Some(t) = parse_time_str(s) {
                Ok(Value::Time(t))
            } else {
                Ok(Value::Null)
            }
        }
        _ => Err(err("ToTime", "cannot convert to Time")),
    }
}

fn parse_time_str(s: &str) -> Option<CqlTime> {
    let parts: Vec<&str> = s.split(':').collect();
    let hour = parts.first()?.parse::<u8>().ok()?;
    if hour > 23 {
        return None;
    }
    let minute = if parts.len() >= 2 {
        parts[1].parse::<u8>().ok()
    } else {
        None
    };
    let (second, millisecond) = if parts.len() >= 3 {
        let sec_str = parts[2];
        if let Some(dot) = sec_str.find('.') {
            (
                sec_str[..dot].parse::<u8>().ok(),
                sec_str[dot + 1..].parse::<u32>().ok(),
            )
        } else {
            (sec_str.parse::<u8>().ok(), None)
        }
    } else {
        (None, None)
    };
    Some(CqlTime {
        hour,
        minute,
        second,
        millisecond,
    })
}

/// `ToQuantity` — parses a String in `<value> '<unit>'` format, or converts a
/// numeric value to a Quantity with the empty (`'1'`) unit.
pub fn to_quantity(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Quantity(q) => Ok(Value::Quantity(q.clone())),
        Value::Integer(n) => Ok(Value::Quantity(CqlQuantity {
            value: *n as f64,
            unit: "1".to_string(),
        })),
        Value::Long(n) => Ok(Value::Quantity(CqlQuantity {
            value: *n as f64,
            unit: "1".to_string(),
        })),
        Value::Decimal(d) => Ok(Value::Quantity(CqlQuantity {
            value: *d,
            unit: "1".to_string(),
        })),
        Value::String(s) => {
            let s = s.trim();
            // Expect: "<number> '<unit>'"
            if let Some(pos) = s.find('\'') {
                let num_str = s[..pos].trim();
                let after = &s[pos + 1..];
                if let Some(end) = after.find('\'') {
                    let unit = after[..end].to_string();
                    if let Ok(v) = num_str.parse::<f64>() {
                        return Ok(Value::Quantity(CqlQuantity { value: v, unit }));
                    }
                }
            }
            // Try parsing as plain number → unit '1'
            if let Ok(v) = s.parse::<f64>() {
                return Ok(Value::Quantity(CqlQuantity {
                    value: v,
                    unit: "1".to_string(),
                }));
            }
            Ok(Value::Null)
        }
        _ => Err(err("ToQuantity", "cannot convert to Quantity")),
    }
}

/// `ToConcept` — promotes a `Code` or `List<Code>` to a `Concept`.
pub fn to_concept(v: &Value) -> Result<Value, EvalError> {
    null1!(v);
    match v {
        Value::Concept(c) => Ok(Value::Concept(c.clone())),
        Value::Code(c) => Ok(Value::Concept(super::super::value::CqlConcept {
            codes: vec![c.clone()],
            display: c.display.clone(),
        })),
        Value::List(items) => {
            let mut codes = Vec::new();
            for item in items {
                match item {
                    Value::Code(c) => codes.push(c.clone()),
                    Value::Null => {}
                    _ => return Err(err("ToConcept", "list elements must be Code")),
                }
            }
            Ok(Value::Concept(super::super::value::CqlConcept {
                codes,
                display: None,
            }))
        }
        _ => Err(err("ToConcept", "cannot convert to Concept")),
    }
}

/// `ToList` — wraps a value in a singleton list.
///
/// - `null` → empty list `[]`  
/// - a `List` value is returned as-is  
/// - any other value is wrapped in `[value]`
pub fn to_list(v: &Value) -> Result<Value, EvalError> {
    match v {
        Value::Null => Ok(Value::List(vec![])),
        Value::List(_) => Ok(v.clone()),
        other => Ok(Value::List(vec![other.clone()])),
    }
}

/// `Is` — runtime type check. Returns `true` if value is of the named type.
///
/// `type_name` should be the simple CQL type name (e.g. `"Integer"`, `"List"`).
pub fn is_type(v: &Value, type_name: &str) -> Value {
    use Value::*;
    let result = match (v, type_name) {
        (Null, _) => false,
        (Boolean(_), "Boolean") => true,
        (Integer(_), "Integer") => true,
        (Long(_), "Long") => true,
        (Decimal(_), "Decimal") => true,
        (String(_), "String") => true,
        (Date(_), "Date") => true,
        (DateTime(_), "DateTime") => true,
        (Time(_), "Time") => true,
        (Quantity(_), "Quantity") => true,
        (Ratio { .. }, "Ratio") => true,
        (Code(_), "Code") => true,
        (Concept(_), "Concept") => true,
        (List(_), "List") => true,
        (Tuple(_), "Tuple") => true,
        (Interval { .. }, "Interval") => true,
        _ => false,
    };
    Value::Boolean(result)
}

/// `As` — runtime cast. Returns the value if it matches `type_name`, else `null`.
pub fn as_type(v: &Value, type_name: &str) -> Value {
    if is_type(v, type_name) == Value::Boolean(true) {
        v.clone()
    } else {
        Value::Null
    }
}

/// `Convert` — attempt conversion to the target CQL type name.
///
/// Delegates to the appropriate `To*` function based on `target_type`.
pub fn convert(v: &Value, target_type: &str) -> Result<Value, EvalError> {
    match target_type {
        "Boolean" => to_boolean(v),
        "Integer" => to_integer(v),
        "Long" => to_long(v),
        "Decimal" => to_decimal(v),
        "String" => to_string(v),
        "Date" => to_date(v),
        "DateTime" => to_datetime(v),
        "Time" => to_time(v),
        "Quantity" => to_quantity(v),
        "Concept" => to_concept(v),
        "List" => to_list(v),
        "Interval" => Err(err("Convert", "conversion to Interval requires start and end values; use interval construction syntax")),
        _ => Err(err("Convert", &format!("unsupported target type '{target_type}'"))),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::value::{
        CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value,
    };

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

    // ---- ToLong ------------------------------------------------------------

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

    // ---- ToDate ------------------------------------------------------------

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

    // ---- ToDateTime --------------------------------------------------------

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

    // ---- ToTime ------------------------------------------------------------

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

    // ---- ToQuantity --------------------------------------------------------

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

    // ---- ToConcept ---------------------------------------------------------

    #[test]
    fn to_concept_from_code() {
        let code = CqlCode {
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
        let code1 = CqlCode {
            code: "A1".to_string(),
            system: "http://example.com".to_string(),
            version: None,
            display: None,
        };
        let code2 = CqlCode {
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
        let code = CqlCode {
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
        let concept = CqlConcept {
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
