//! `precision()`, `lowBoundary()`, and `highBoundary()` functions.
//!
//! Semantics follow the HL7 FHIRPath test suite (LowBoundary / HighBoundary /
//! Precision groups). Decimal values use `rust_decimal::Decimal` to preserve
//! trailing zeros in literals (e.g. `1.58700` has precision 5).

use std::collections::HashMap;

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

use super::FhirPathFunction;

pub fn register_boundary_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    functions.insert(
        "precision".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| precision(target)),
    );
    functions.insert(
        "lowBoundary".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            boundary(target, params, false)
        }),
    );
    functions.insert(
        "highBoundary".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| boundary(target, params, true)),
    );
}

// ---------------------------------------------------------------------------
// precision()
// ---------------------------------------------------------------------------

fn precision(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match unwrap_singleton(target) {
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),
        FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(decimal_precision(*n) as i64)),
        FhirPathValue::Integer(_) | FhirPathValue::Long(_) => Ok(FhirPathValue::Integer(0)),
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) | FhirPathValue::Time(s) => {
            let without_tz = strip_timezone(s);
            let digits = without_tz.chars().filter(|c| c.is_ascii_digit()).count();
            Ok(FhirPathValue::Integer(digits as i64))
        }
        other => Err(FhirPathError::FunctionError {
            message: format!("precision() not supported for {other:?}"),
        }),
    }
}

// ---------------------------------------------------------------------------
// lowBoundary() / highBoundary()
// ---------------------------------------------------------------------------

fn boundary(
    target: &FhirPathValue,
    params: &[FhirPathValue],
    high: bool,
) -> FhirPathResult<FhirPathValue> {
    let out_prec = match params.first() {
        None => None,
        Some(FhirPathValue::Integer(p)) | Some(FhirPathValue::Long(p)) => Some(*p),
        Some(other) => {
            return Err(FhirPathError::FunctionError {
                message: format!("boundary precision must be an integer, got {other:?}"),
            })
        }
    };

    // Out-of-range precision yields empty (suite: lowBoundary(-1) / (32) / (39)).
    if let Some(p) = out_prec {
        if !(0..=28).contains(&p) {
            return Ok(FhirPathValue::Empty);
        }
    }

    match unwrap_singleton(target) {
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),
        FhirPathValue::Number(n) => Ok(FhirPathValue::Number(boundary_decimal(*n, out_prec, high))),
        FhirPathValue::Integer(i) => Ok(FhirPathValue::Number(boundary_decimal(
            Decimal::from(*i),
            out_prec,
            high,
        ))),
        FhirPathValue::Long(i) => Ok(FhirPathValue::Number(boundary_decimal(
            Decimal::from(*i),
            out_prec,
            high,
        ))),
        FhirPathValue::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
            value: boundary_decimal(*value, out_prec, high),
            unit: unit.clone(),
        }),
        FhirPathValue::Date(s) => boundary_date(s, out_prec.unwrap_or(8), high),
        FhirPathValue::DateTime(s) => boundary_datetime(s, out_prec.unwrap_or(17), high),
        FhirPathValue::Time(s) => Ok(FhirPathValue::Time(boundary_time(
            s,
            out_prec.unwrap_or(9),
            high,
        ))),
        // String values that look like temporal types are dispatched to the
        // corresponding boundary function. This handles deeply nested FHIR
        // fields (e.g. `identifier.period.start`) where FHIR metadata typing
        // may not be applied because the field lives inside a BackboneElement
        // not directly in the resource's top-level metadata registry.
        FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
            if let Some(result) = try_boundary_temporal_string(s, out_prec, high) {
                result
            } else {
                Err(FhirPathError::FunctionError {
                    message: format!("boundary functions not supported for String(\"{s}\")"),
                })
            }
        }
        other => Err(FhirPathError::FunctionError {
            message: format!("boundary functions not supported for {other:?}"),
        }),
    }
}

fn boundary_decimal(value: Decimal, out_prec: Option<i64>, high: bool) -> Decimal {
    let input_prec = value.scale() as i32;
    let half = Decimal::from_str_exact("0.5").unwrap()
        * Decimal::from_str_exact("10")
            .unwrap()
            .powi(-(input_prec as i64));
    let bound = if high { value + half } else { value - half };

    let p = out_prec.unwrap_or(8).clamp(0, 12);
    let scale = Decimal::from_str_exact("10").unwrap().powi(p);
    let scaled = bound * scale;

    // When the scaled value is between -1 and 1 (exclusive), the boundary
    // collapses toward zero: use truncation (round toward zero) rather than
    // floor/ceil. This handles cases like 0.0034.highBoundary(1) = 0.0 where
    // ceil(0.0345) would incorrectly give 0.1, and (-0.0034).lowBoundary(1)
    // = -0.0 where floor(-0.0345) would incorrectly give -0.1.
    let snapped = if scaled.abs() < Decimal::ONE {
        // Truncate toward zero. Preserve the sign for negative values near
        // zero so that (-0.0034).lowBoundary(1) yields -0.0.
        let truncated = scaled.trunc();
        if truncated.is_zero() {
            // Produce a zero with the correct scale and sign.
            // lowBoundary of a negative value near zero → -0.0
            // highBoundary of any value near zero → 0.0 (or -0.0 for neg)
            let mut zero = Decimal::ZERO;
            zero.set_sign_positive(!value.is_sign_negative() || high);
            zero.rescale(p as u32);
            zero
        } else {
            truncated
        }
    } else if high {
        scaled.ceil()
    } else {
        scaled.floor()
    };
    snapped / scale
}

/// Returns the number of fractional decimal digits preserved in this `Decimal`.
/// `rust_decimal` tracks scale (number of digits after the decimal point), so
/// `1.58700` has precision 5, `12.500` has precision 3, and `7` has precision 0.
fn decimal_precision(value: Decimal) -> i32 {
    value.scale() as i32
}

/// Legacy helper retained for integer boundary calculations where we don't
/// have a `Decimal` with meaningful scale.  Falls back to string-based counting.
#[allow(dead_code)]
fn frac_digits_f64(value: f64) -> i32 {
    let s = format!("{value}");
    s.split('.').nth(1).map(|f| f.len() as i32).unwrap_or(0)
}

fn boundary_date(s: &str, digits: i64, high: bool) -> FhirPathResult<FhirPathValue> {
    let parts: Vec<&str> = s.split('-').collect();
    let year = parts.first().copied().unwrap_or("0000");
    let month = parts.get(1).copied();
    let day = parts.get(2).copied();

    let result = match digits {
        d if d <= 4 => year.to_string(),
        d if d <= 6 => {
            let m = month
                .map(str::to_string)
                .unwrap_or_else(|| if high { "12" } else { "01" }.to_string());
            format!("{year}-{m}")
        }
        _ => {
            let m = month
                .map(str::to_string)
                .unwrap_or_else(|| if high { "12" } else { "01" }.to_string());
            let d = day.map(str::to_string).unwrap_or_else(|| {
                if high {
                    last_day_of_month(year, &m).to_string()
                } else {
                    "01".to_string()
                }
            });
            format!("{year}-{m}-{d}")
        }
    };
    Ok(FhirPathValue::Date(result))
}

fn boundary_datetime(s: &str, digits: i64, high: bool) -> FhirPathResult<FhirPathValue> {
    let (datetime, tz) = split_timezone(s);
    let (date_part, time_part) = match datetime.split_once('T') {
        Some((d, t)) => (d, Some(t)),
        None => (datetime, None),
    };

    // Date-only target precision: result degrades to a Date.
    if digits <= 8 {
        return boundary_date(date_part, digits, high);
    }

    let date = match boundary_date(date_part, 8, high)? {
        FhirPathValue::Date(d) => d,
        _ => unreachable!(),
    };

    let t: Vec<&str> = time_part
        .unwrap_or("")
        .split(':')
        .filter(|p| !p.is_empty())
        .collect();
    let hour = t.first().copied().unwrap_or(if high { "23" } else { "00" });
    // Suite quirk (HighBoundary group): an absent minute component is filled
    // with "00" even for the high boundary.
    let minute = t.get(1).copied().unwrap_or("00");
    let (sec, ms) = match t.get(2) {
        Some(sec_ms) => match sec_ms.split_once('.') {
            Some((sec, ms)) => (sec.to_string(), format!("{ms:0<3}")),
            None => (
                (*sec_ms).to_string(),
                if high { "999" } else { "000" }.to_string(),
            ),
        },
        None => {
            if high {
                ("59".to_string(), "999".to_string())
            } else {
                ("00".to_string(), "000".to_string())
            }
        }
    };

    let tz = tz.unwrap_or(if high { "-12:00" } else { "+14:00" });
    let time = match digits {
        d if d <= 10 => hour.to_string(),
        d if d <= 12 => format!("{hour}:{minute}"),
        d if d <= 14 => format!("{hour}:{minute}:{sec}"),
        _ => format!("{hour}:{minute}:{sec}.{ms}"),
    };
    Ok(FhirPathValue::DateTime(format!("{date}T{time}{tz}")))
}

fn boundary_time(s: &str, digits: i64, high: bool) -> String {
    let raw = s.strip_prefix('T').unwrap_or(s);
    let t: Vec<&str> = raw.split(':').collect();
    let hour = t.first().copied().unwrap_or(if high { "23" } else { "00" });
    let minute = t.get(1).copied().unwrap_or(if high { "59" } else { "00" });
    let (sec, ms) = match t.get(2) {
        Some(sec_ms) => match sec_ms.split_once('.') {
            Some((sec, ms)) => (sec.to_string(), format!("{ms:0<3}")),
            None => (
                (*sec_ms).to_string(),
                if high { "999" } else { "000" }.to_string(),
            ),
        },
        None => {
            if high {
                ("59".to_string(), "999".to_string())
            } else {
                ("00".to_string(), "000".to_string())
            }
        }
    };
    match digits {
        d if d <= 2 => hour.to_string(),
        d if d <= 4 => format!("{hour}:{minute}"),
        d if d <= 6 => format!("{hour}:{minute}:{sec}"),
        _ => format!("{hour}:{minute}:{sec}.{ms}"),
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn unwrap_singleton(v: &FhirPathValue) -> &FhirPathValue {
    match v {
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
            if items.len() == 1 =>
        {
            &items[0]
        }
        other => other,
    }
}

/// Try to dispatch a String value to the appropriate temporal boundary
/// function based on its content. Returns `Some(result)` if the string
/// looks like a date, dateTime, or time; `None` if it doesn't match any
/// temporal pattern.
fn try_boundary_temporal_string(
    s: &str,
    out_prec: Option<i64>,
    high: bool,
) -> Option<FhirPathResult<FhirPathValue>> {
    if s.contains('T') {
        Some(boundary_datetime(s, out_prec.unwrap_or(17), high))
    } else if s.contains(':') {
        Some(Ok(FhirPathValue::Time(boundary_time(
            s,
            out_prec.unwrap_or(9),
            high,
        ))))
    } else if looks_like_date(s) {
        // Return a DateTime (not a Date) so that boundary comparisons
        // between Date-sourced and DateTime-sourced values are comparable.
        // A Date string like "2001-05-06" is promoted to the DateTime
        // boundary at midnight: "2001-05-06T00:00:00[+/-14:00]".
        let prec = out_prec.unwrap_or(8);
        let date_result = match boundary_date(s, prec, high) {
            Ok(FhirPathValue::Date(d)) => d,
            other => return Some(other),
        };
        let tz = if high { "-12:00" } else { "+14:00" };
        Some(Ok(FhirPathValue::DateTime(format!(
            "{date_result}T00:00:00{tz}"
        ))))
    } else {
        None
    }
}

/// Heuristic: a string that matches `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`
/// (with optional trailing `-` parts) is treated as a date.
fn looks_like_date(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.is_empty() {
        return false;
    }
    let year_ok = parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit());
    if !year_ok {
        return false;
    }
    if parts.len() == 1 {
        return true;
    }
    let month_ok = parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit());
    if !month_ok {
        return false;
    }
    if parts.len() == 2 {
        return true;
    }
    parts[2].len() == 2 && parts[2].chars().all(|c| c.is_ascii_digit())
}

/// Strip a trailing timezone (`Z` or `±hh:mm` after the time part) from a
/// temporal string.
fn strip_timezone(s: &str) -> &str {
    split_timezone(s).0
}

fn split_timezone(s: &str) -> (&str, Option<&str>) {
    if let Some(stripped) = s.strip_suffix('Z') {
        return (stripped, Some("Z"));
    }
    // A timezone offset only appears after the 'T' time separator (dates use
    // '-' internally), or in pure time strings after a ':'.
    let search_from = s.find('T').map(|i| i + 1).unwrap_or(0);
    let tail = &s[search_from..];
    if let Some(pos) = tail.rfind(['+', '-']) {
        let candidate = &tail[pos..];
        if candidate.len() == 6 && candidate.as_bytes()[3] == b':' {
            let split_at = search_from + pos;
            return (&s[..split_at], Some(&s[split_at..]));
        }
    }
    (s, None)
}

fn last_day_of_month(year: &str, month: &str) -> u32 {
    let y: i32 = year.parse().unwrap_or(2000);
    let m: u32 = month.parse().unwrap_or(1);
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
            if leap {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}
