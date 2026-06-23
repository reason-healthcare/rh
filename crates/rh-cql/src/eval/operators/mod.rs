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

const CQL_DECIMAL_MAX_MAGNITUDE: f64 = 1e28;
const CQL_DECIMAL_MIN_STEP: f64 = 1e-8;
const CQL_DECIMAL_MAX_SCALE: usize = 8;
const CQL_TIME_MAX_HOUR: u8 = 23;
const CQL_TIME_MAX_MINUTE_OR_SECOND: u8 = 59;
const CQL_TIME_MAX_MILLISECOND: u32 = 999;

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
    let hour = parse_time_component(parts[0], CQL_TIME_MAX_HOUR, "hour", s)?;
    let minute = parse_time_component(parts[1], CQL_TIME_MAX_MINUTE_OR_SECOND, "minute", s)?;
    if parts.len() == 2 {
        return Ok(Value::Time(CqlTime {
            hour,
            minute: Some(minute),
            second: None,
            millisecond: None,
        }));
    }
    // parts[2] is "SS" or "SS.mmm"
    let sec_ms: Vec<&str> = parts[2].splitn(2, '.').collect();
    let second = parse_time_component(sec_ms[0], CQL_TIME_MAX_MINUTE_OR_SECOND, "second", s)?;
    let millisecond = match sec_ms.get(1) {
        Some(ms_str) => Some(
            parse_millisecond(ms_str)
                .ok_or_else(|| EvalError::General(format!("Invalid Time millisecond in '{s}'")))?,
        ),
        None => None,
    };
    Ok(Value::Time(CqlTime {
        hour,
        minute: Some(minute),
        second: Some(second),
        millisecond,
    }))
}

fn parse_time_component(
    raw: &str,
    max: u8,
    component_name: &str,
    source: &str,
) -> Result<u8, EvalError> {
    let value = raw
        .parse::<u8>()
        .map_err(|_| EvalError::General(format!("Invalid Time {component_name} in '{source}'")))?;
    if value > max {
        return Err(EvalError::General(format!(
            "Invalid Time {component_name} in '{source}'"
        )));
    }
    Ok(value)
}

pub(super) fn parse_time_component_opt(raw: &str, max: u8) -> Option<u8> {
    let value = raw.parse::<u8>().ok()?;
    (value <= max).then_some(value)
}

pub(super) fn parse_hour(raw: &str) -> Option<u8> {
    parse_time_component_opt(raw, CQL_TIME_MAX_HOUR)
}

pub(super) fn parse_minute_or_second(raw: &str) -> Option<u8> {
    parse_time_component_opt(raw, CQL_TIME_MAX_MINUTE_OR_SECOND)
}

/// Parse a CQL time millisecond component, right-padding to three digits.
pub(super) fn parse_millisecond(raw: &str) -> Option<u32> {
    let padded = format!("{:0<3}", raw);
    let value = padded.get(..3)?.parse::<u32>().ok()?;
    (value <= CQL_TIME_MAX_MILLISECOND).then_some(value)
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
        "Decimal" => parse_decimal_value(value_str),
        "String" => Ok(Value::String(value_str.to_string())),
        "Date" => parse_date_value(value_str),
        "DateTime" => parse_datetime_value(value_str),
        "Time" => parse_time_value(value_str),
        "" if value_str.is_empty() => Ok(Value::Null),
        _ => Ok(Value::String(value_str.to_string())),
    }
}

fn parse_decimal_value(value_str: &str) -> Result<Value, EvalError> {
    let value = value_str
        .parse::<f64>()
        .map_err(|_| EvalError::General(format!("Invalid Decimal literal: '{value_str}'")))?;

    if value.abs() >= CQL_DECIMAL_MAX_MAGNITUDE {
        return Err(EvalError::General(format!(
            "Invalid Decimal literal outside CQL Decimal range: '{value_str}'"
        )));
    }

    if decimal_scale(value_str) > CQL_DECIMAL_MAX_SCALE
        || (value != 0.0 && value.abs() < CQL_DECIMAL_MIN_STEP)
    {
        return Err(EvalError::General(format!(
            "Invalid Decimal literal exceeds CQL Decimal scale: '{value_str}'"
        )));
    }

    Ok(Value::Decimal(value))
}

fn decimal_scale(value_str: &str) -> usize {
    value_str
        .trim()
        .trim_start_matches('-')
        .split_once('.')
        .map(|(_, fractional)| fractional.trim_end_matches('0').len())
        .unwrap_or(0)
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
        ("Length", [Value::List(_) | Value::Null]) => super::lists::length(&args[0]),
        ("Length", [Value::String(_)]) => length_str(&args[0]),
        ("Sum", [list]) => super::lists::sum(list),
        ("Min", [list]) => super::lists::min(list),
        ("Max", [list]) => super::lists::max(list),
        ("Avg", [list]) => super::lists::avg(list),
        ("First", [list]) => super::lists::first(list),
        ("Last", [list]) => super::lists::last(list),
        ("Except", [a, b]) => super::lists::except_list(a, b),
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
        ("Concatenate", [a, b]) => concatenate(a, b),
        ("Upper", [s]) => upper(s),
        ("Lower", [s]) => lower(s),
        ("StartsWith", [s, prefix]) => starts_with(s, prefix),
        ("EndsWith", [s, suffix]) => ends_with(s, suffix),
        ("Matches", [s, pattern]) => matches_regex(s, pattern),
        ("SplitOnMatches", [s, pattern]) => split_on_matches(s, pattern),
        ("PositionOf", [pattern, s]) => position_of(pattern, s),
        ("LastPositionOf", [pattern, s]) => last_position_of(pattern, s),
        ("Substring", [s, start]) => substring(s, start, None),
        ("Substring", [s, start, len]) => substring(s, start, Some(len)),
        ("ReplaceMatches", [s, pattern, substitution]) => replace_matches(s, pattern, substitution),
        ("Indexer", [source, index]) => match source {
            Value::List(_) => super::lists::indexer(source, index),
            Value::String(_) => indexer_str(source, index),
            Value::Null => Ok(Value::Null),
            _ => Err(EvalError::General(
                "Indexer: unsupported operand types".to_string(),
            )),
        },
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
        // Aggregate extensions
        ("Product", [list]) => super::lists::product(list),
        ("GeometricMean", [list]) => super::lists::geometric_mean(list),
        // Uncertainty / precision arithmetic
        ("Precision", [v]) => eval_precision(v),
        ("LowBoundary", [v, prec]) => eval_low_boundary(v, prec),
        ("HighBoundary", [v, prec]) => eval_high_boundary(v, prec),
        // String combine without separator (the 2-arg form is emitted as an
        // ELM Combine node; the 1-arg overload arrives as a FunctionRef)
        ("Combine", [list]) => combine(list, None),
        ("ToRatio", [v]) => to_ratio(v),
        // Tree navigation
        ("Children", [v]) => children(v),
        ("Descendants", [v]) => descendants(v),
        ("Descendents", [v]) | ("descendents", [v]) => descendants(v),
        ("Descendents", []) | ("descendents", []) => Ok(Value::Null),
        // Errors & messaging
        ("Message", [source, condition, code, severity, message]) => {
            eval_message(source, condition, code, severity, message)
        }
        _ => Err(EvalError::General(format!(
            "evaluate_elm: unknown FunctionRef '{name}' with {} arg(s)",
            args.len()
        ))),
    }
}

/// `Children(x)` — the immediate child values of a structured value: tuple
/// field values, or the children of each element for a list. Non-structured
/// values have no children.
fn children(v: &Value) -> Result<Value, EvalError> {
    match v {
        Value::Null => Ok(Value::Null),
        Value::Tuple(fields) => Ok(Value::List(fields.values().cloned().collect())),
        Value::List(items) => {
            let mut out = Vec::new();
            for item in items {
                match children(item)? {
                    Value::List(cs) => out.extend(cs),
                    Value::Null => {}
                    other => out.push(other),
                }
            }
            Ok(Value::List(out))
        }
        _ => Ok(Value::List(Vec::new())),
    }
}

/// `Descendants(x)` — all descendant values of a structured value
/// (children, recursively).
fn descendants(v: &Value) -> Result<Value, EvalError> {
    fn collect(v: &Value, out: &mut Vec<Value>) {
        match v {
            Value::Tuple(fields) => {
                for child in fields.values() {
                    out.push(child.clone());
                    collect(child, out);
                }
            }
            Value::List(items) => {
                for item in items {
                    collect(item, out);
                }
            }
            _ => {}
        }
    }
    match v {
        Value::Null => Ok(Value::Null),
        other => {
            let mut out = Vec::new();
            collect(other, &mut out);
            Ok(Value::List(out))
        }
    }
}

/// `Message(source, condition, code, severity, message)` — when `condition`
/// is true, reports a message at the given severity ('Trace', 'Message',
/// 'Warning', or 'Error'; 'Error' raises a runtime error). Returns `source`.
fn eval_message(
    source: &Value,
    condition: &Value,
    code: &Value,
    severity: &Value,
    message: &Value,
) -> Result<Value, EvalError> {
    if !matches!(condition, Value::Boolean(true)) {
        return Ok(source.clone());
    }

    let as_text = |v: &Value| -> String {
        match v {
            Value::String(s) => s.clone(),
            Value::Null => String::new(),
            other => format!("{other:?}"),
        }
    };
    let code_text = as_text(code);
    let message_text = as_text(message);
    let severity_text = match severity {
        Value::String(s) => s.as_str(),
        _ => "Message",
    };

    match severity_text {
        "Error" => Err(EvalError::General(format!(
            "Message error [{code_text}]: {message_text}"
        ))),
        "Warning" => {
            tracing::warn!("CQL Message [{code_text}]: {message_text}");
            Ok(source.clone())
        }
        _ => {
            tracing::info!("CQL {severity_text} [{code_text}]: {message_text}");
            Ok(source.clone())
        }
    }
}

// ---------------------------------------------------------------------------
// Precision / LowBoundary / HighBoundary helpers
// ---------------------------------------------------------------------------

/// Count the number of significant decimal digits needed to represent a Decimal.
fn decimal_sig_places(d: f64) -> i32 {
    // Use a minimal-representation approach: format to enough places then strip trailing zeros.
    let s = format!("{:.15}", d);
    let s = s.trim_end_matches('0');
    if let Some(dot) = s.rfind('.') {
        (s.len() - dot - 1) as i32
    } else {
        0
    }
}

/// Map a digit-count precision value to (date components, time components) indicator.
/// Digit count: year=4, month=6, day=8, datetime_hour=10, datetime_minute=12,
/// datetime_second=14, datetime_ms=17; time_hour=2, time_minute=4, time_second=6, time_ms=9.
fn datetime_precision_to_fields(precision: i64, is_time_only: bool) -> (usize, usize) {
    if is_time_only {
        let time_level = match precision {
            p if p <= 2 => 1,
            p if p <= 4 => 2,
            p if p <= 6 => 3,
            _ => 4,
        };
        (0, time_level)
    } else {
        let date_level = match precision {
            p if p <= 4 => 1,
            p if p <= 6 => 2,
            p if p <= 8 => 3,
            _ => 3,
        };
        let time_level = if precision <= 8 {
            0
        } else {
            match precision {
                p if p <= 10 => 1,
                p if p <= 12 => 2,
                p if p <= 14 => 3,
                _ => 4,
            }
        };
        (date_level, time_level)
    }
}

/// `Precision(x)` — returns the number of significant decimal digits.
fn eval_precision(v: &Value) -> Result<Value, EvalError> {
    match v {
        Value::Null => Ok(Value::Null),
        Value::Decimal(d) => {
            let places = decimal_sig_places(*d);
            Ok(Value::Integer(places as i64))
        }
        Value::Date(date) => {
            let mut precision: i64 = 4; // year always
            if date.month.is_some() {
                precision += 2;
            }
            if date.day.is_some() {
                precision += 2;
            }
            Ok(Value::Integer(precision))
        }
        Value::DateTime(dt) => {
            let mut precision: i64 = 4; // year
            if dt.month.is_some() {
                precision += 2;
            }
            if dt.day.is_some() {
                precision += 2;
            }
            if dt.hour.is_some() {
                precision += 2;
            }
            if dt.minute.is_some() {
                precision += 2;
            }
            if dt.second.is_some() {
                precision += 2;
            }
            if dt.millisecond.is_some() {
                precision += 3;
            }
            Ok(Value::Integer(precision))
        }
        Value::Time(t) => {
            let mut precision: i64 = 2; // hour
            if t.minute.is_some() {
                precision += 2;
            }
            if t.second.is_some() {
                precision += 2;
            }
            if t.millisecond.is_some() {
                precision += 3;
            }
            Ok(Value::Integer(precision))
        }
        _ => Err(EvalError::General(
            "Precision: expected Decimal, Date, DateTime, or Time".to_string(),
        )),
    }
}

/// `LowBoundary(value, precision)` — low boundary of value at the given precision.
fn eval_low_boundary(v: &Value, prec: &Value) -> Result<Value, EvalError> {
    let precision = match prec {
        Value::Integer(p) => *p,
        Value::Null => return Ok(Value::Null),
        _ => {
            return Err(EvalError::General(
                "LowBoundary: precision must be an Integer".to_string(),
            ))
        }
    };
    match v {
        Value::Null => Ok(Value::Null),
        Value::Decimal(d) => {
            // LowBoundary(x, n): x rounded down to n decimal places
            let factor = 10_f64.powi(precision as i32);
            Ok(Value::Decimal((d * factor).floor() / factor))
        }
        Value::Date(date) => {
            // Expand the date to the low boundary at the target precision
            let mut result = date.clone();
            let (date_level, _) = datetime_precision_to_fields(precision, false);
            if date_level >= 2 && result.month.is_none() {
                result.month = Some(1);
            }
            if date_level >= 3 && result.day.is_none() {
                result.day = Some(1);
            }
            Ok(Value::Date(result))
        }
        Value::DateTime(dt) => {
            let mut result = dt.clone();
            let (date_level, time_level) = datetime_precision_to_fields(precision, false);
            if date_level >= 2 && result.month.is_none() {
                result.month = Some(1);
            }
            if date_level >= 3 && result.day.is_none() {
                result.day = Some(1);
            }
            if time_level >= 1 && result.hour.is_none() {
                result.hour = Some(0);
            }
            if time_level >= 2 && result.minute.is_none() {
                result.minute = Some(0);
            }
            if time_level >= 3 && result.second.is_none() {
                result.second = Some(0);
            }
            if time_level >= 4 && result.millisecond.is_none() {
                result.millisecond = Some(0);
            }
            Ok(Value::DateTime(result))
        }
        Value::Time(t) => {
            let mut result = t.clone();
            let (_, time_level) = datetime_precision_to_fields(precision, true);
            if time_level >= 2 && result.minute.is_none() {
                result.minute = Some(0);
            }
            if time_level >= 3 && result.second.is_none() {
                result.second = Some(0);
            }
            if time_level >= 4 && result.millisecond.is_none() {
                result.millisecond = Some(0);
            }
            Ok(Value::Time(result))
        }
        _ => Err(EvalError::General(
            "LowBoundary: expected Decimal, Date, DateTime, or Time".to_string(),
        )),
    }
}

/// `HighBoundary(value, precision)` — high boundary of value at the given precision.
fn eval_high_boundary(v: &Value, prec: &Value) -> Result<Value, EvalError> {
    let precision = match prec {
        Value::Integer(p) => *p,
        Value::Null => return Ok(Value::Null),
        _ => {
            return Err(EvalError::General(
                "HighBoundary: precision must be an Integer".to_string(),
            ))
        }
    };
    match v {
        Value::Null => Ok(Value::Null),
        Value::Decimal(d) => {
            // HighBoundary(x, n): x + (1*10^-original_places - 1*10^-n)
            let orig_places = decimal_sig_places(*d);
            let step = 10_f64.powi(-orig_places);
            let ulp = 10_f64.powi(-(precision as i32));
            Ok(Value::Decimal(d + step - ulp))
        }
        Value::Date(date) => {
            let mut result = date.clone();
            let (date_level, _) = datetime_precision_to_fields(precision, false);
            if date_level >= 2 && result.month.is_none() {
                result.month = Some(12);
            }
            if date_level >= 3 && result.day.is_none() {
                // Use last day of the month
                let month = result.month.unwrap_or(12);
                let year = result.year;
                let last_day = days_in_month(year, month as u32) as u8;
                result.day = Some(last_day);
            }
            Ok(Value::Date(result))
        }
        Value::DateTime(dt) => {
            let mut result = dt.clone();
            let (date_level, time_level) = datetime_precision_to_fields(precision, false);
            if date_level >= 2 && result.month.is_none() {
                result.month = Some(12);
            }
            if date_level >= 3 && result.day.is_none() {
                let month = result.month.unwrap_or(12);
                let last_day = days_in_month(result.year, month as u32) as u8;
                result.day = Some(last_day);
            }
            if time_level >= 1 && result.hour.is_none() {
                result.hour = Some(23);
            }
            if time_level >= 2 && result.minute.is_none() {
                result.minute = Some(59);
            }
            if time_level >= 3 && result.second.is_none() {
                result.second = Some(59);
            }
            if time_level >= 4 && result.millisecond.is_none() {
                result.millisecond = Some(999);
            }
            Ok(Value::DateTime(result))
        }
        Value::Time(t) => {
            let mut result = t.clone();
            let (_, time_level) = datetime_precision_to_fields(precision, true);
            if time_level >= 2 && result.minute.is_none() {
                result.minute = Some(59);
            }
            if time_level >= 3 && result.second.is_none() {
                result.second = Some(59);
            }
            if time_level >= 4 && result.millisecond.is_none() {
                result.millisecond = Some(999);
            }
            Ok(Value::Time(result))
        }
        _ => Err(EvalError::General(
            "HighBoundary: expected Decimal, Date, DateTime, or Time".to_string(),
        )),
    }
}

/// Returns the number of days in a given month/year.
fn days_in_month(year: i32, month: u32) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}
