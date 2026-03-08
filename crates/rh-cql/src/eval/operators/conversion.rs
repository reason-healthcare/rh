//! CQL type-conversion operators (CQL §22).
//!
//! Exports: [`to_boolean`], [`to_integer`], [`to_long`], [`to_decimal`],
//! [`to_string`], [`to_date`], [`to_datetime`], [`to_time`], [`to_quantity`],
//! [`to_concept`], [`to_list`], [`is_type`], [`as_type`], [`convert`].

use super::super::context::EvalError;
use super::super::value::{CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

macro_rules! null1 {
    ($a:expr) => {
        if matches!($a, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

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
