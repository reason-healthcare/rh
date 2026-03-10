//! CQL operator implementations, split across domain submodules.
//!
//! All public items are re-exported from this module so that callers can
//! continue to write `use crate::eval::operators::add;` etc.

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod string_ops;
pub mod temporal;
pub(super) mod utils;

pub use arithmetic::*;
pub use comparison::*;
pub use conversion::*;
pub use string_ops::*;
pub use temporal::*;

use super::super::elm;
use super::context::EvalError;
use super::value::{CqlDate, CqlDateTime, CqlTime, Value};

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

/// Parse a time string `"HH:MM"`, `"HH:MM:SS"`, or `"HH:MM:SS.mmm"` → `Value::Time`.
fn parse_time_value(s: &str) -> Result<Value, EvalError> {
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    if parts.len() < 2 {
        return Err(EvalError::General(format!("Invalid Time literal: '{s}'")));
    }
    let hour = parts[0]
        .parse::<u8>()
        .map_err(|_| EvalError::General(format!("Invalid Time hour in '{s}'")))?;
    let minute = parts[1].parse::<u8>().ok();
    if parts.len() == 2 {
        return Ok(Value::Time(CqlTime {
            hour,
            minute,
            second: None,
            millisecond: None,
        }));
    }
    // parts[2] is "SS" or "SS.mmm"
    let sec_ms: Vec<&str> = parts[2].splitn(2, '.').collect();
    let second = sec_ms[0].parse::<u8>().ok();
    // Milliseconds: "9"→900ms, "99"→990ms, "999"→999ms (right-pad with zeros to 3 digits)
    let millisecond = sec_ms.get(1).and_then(|ms_str| {
        let padded = format!("{:0<3}", ms_str);
        padded[..3].parse::<u32>().ok()
    });
    Ok(Value::Time(CqlTime {
        hour,
        minute,
        second,
        millisecond,
    }))
}

/// Parse a date string `"YYYY"`, `"YYYY-MM"`, or `"YYYY-MM-DD"` → `Value::Date`.
fn parse_date_value(s: &str) -> Result<Value, EvalError> {
    let parts: Vec<&str> = s.splitn(3, '-').collect();
    if parts.is_empty() {
        return Err(EvalError::General(format!("Invalid Date literal: '{s}'")));
    }
    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| EvalError::General(format!("Invalid Date year in '{s}'")))?;
    let month = parts.get(1).and_then(|m| m.parse::<u8>().ok());
    let day = parts.get(2).and_then(|d| d.parse::<u8>().ok());
    Ok(Value::Date(CqlDate { year, month, day }))
}

/// Parse a timezone offset string like `"+05:00"` or `"-07:00"` → seconds east of UTC.
fn parse_tz_offset(tz: &str) -> Option<i32> {
    if tz.len() < 6 {
        return None;
    }
    let sign: i32 = if tz.starts_with('+') { 1 } else { -1 };
    let h: i32 = tz[1..3].parse().ok()?;
    let m: i32 = tz[4..6].parse().ok()?;
    Some(sign * (h * 3600 + m * 60))
}

/// Parse a datetime string `"YYYY-MM-DDTHH:MM:SS.mmm[±HH:MM]"` → `Value::DateTime`.
fn parse_datetime_value(s: &str) -> Result<Value, EvalError> {
    let t_pos = s.find('T').ok_or_else(|| {
        EvalError::General(format!("Invalid DateTime literal (missing T): '{s}'"))
    })?;
    let date_str = &s[..t_pos];
    let time_and_tz = &s[t_pos + 1..];

    // Strip timezone suffix (+HH:MM, -HH:MM, Z).
    let (time_str, offset_seconds) = if let Some(stripped) = time_and_tz.strip_suffix('Z') {
        (stripped, Some(0i32))
    } else if let Some(pos) = time_and_tz.rfind(['+', '-']).filter(|&p| p > 0) {
        let offset = parse_tz_offset(&time_and_tz[pos..]);
        (&time_and_tz[..pos], offset)
    } else {
        (time_and_tz, None)
    };

    let date_parts: Vec<&str> = date_str.splitn(3, '-').collect();
    let year = date_parts
        .first()
        .and_then(|y| y.parse::<i32>().ok())
        .ok_or_else(|| EvalError::General(format!("Invalid DateTime year in '{s}'")))?;
    let month = date_parts.get(1).and_then(|m| m.parse::<u8>().ok());
    let day = date_parts.get(2).and_then(|d| d.parse::<u8>().ok());

    let time_parts: Vec<&str> = time_str.splitn(3, ':').collect();
    let hour = time_parts.first().and_then(|h| h.parse::<u8>().ok());
    let minute = time_parts.get(1).and_then(|m| m.parse::<u8>().ok());
    let (second, millisecond) = if let Some(sec_ms) = time_parts.get(2) {
        let sec_ms_parts: Vec<&str> = sec_ms.splitn(2, '.').collect();
        let sec = sec_ms_parts.first().and_then(|s| s.parse::<u8>().ok());
        let ms = sec_ms_parts.get(1).and_then(|ms_str| {
            let padded = format!("{:0<3}", ms_str);
            padded[..3].parse::<u32>().ok()
        });
        (sec, ms)
    } else {
        (None, None)
    };

    Ok(Value::DateTime(CqlDateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
        offset_seconds,
    }))
}

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
        "Date" => parse_date_value(value_str),
        "DateTime" => parse_datetime_value(value_str),
        "Time" => parse_time_value(value_str),
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
        // String operators
        ("SplitOnMatches", [s, pattern]) => split_on_matches(s, pattern),
        ("PositionOf", [pattern, s]) => position_of(pattern, s),
        ("LastPositionOf", [pattern, s]) => last_position_of(pattern, s),
        ("Substring", [s, start]) => substring(s, start, None),
        ("Substring", [s, start, len]) => substring(s, start, Some(len)),
        ("ReplaceMatches", [s, pattern, substitution]) => replace_matches(s, pattern, substitution),
        ("IndexOf", [left, right]) => match left {
            Value::List(_) => super::lists::index_of(left, right),
            Value::String(_) => position_of(right, left),
            Value::Null => Ok(Value::Null),
            _ => Err(EvalError::General(
                "IndexOf: unsupported operand types".to_string(),
            )),
        },
        // List slice family
        ("Tail", [list]) => super::lists::tail(list),
        ("Skip", [list, n]) => super::lists::skip(list, n),
        ("Take", [list, n]) => super::lists::take(list, n),
        ("Slice", [list, start]) => super::lists::slice(list, start, None),
        ("Slice", [list, start, end]) => super::lists::slice(list, start, Some(end)),
        _ => Err(EvalError::General(format!(
            "evaluate_elm: unknown FunctionRef '{name}' with {} arg(s)",
            args.len()
        ))),
    }
}
