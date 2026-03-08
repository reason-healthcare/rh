//! CQL date/time operators (CQL §18, §19).
//!
//! Exports: [`predecessor`], [`successor`], [`min_value`], [`max_value`],
//! [`date_construct`], [`time_construct`], [`datetime_construct`],
//! [`date_time_component`], [`date_time_add`], [`date_time_subtract`],
//! [`same_as`], [`same_or_before`], [`same_or_after`],
//! [`duration_between`], [`difference_between`].

use std::cmp::Ordering;

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

macro_rules! null2 {
    ($a:expr, $b:expr) => {
        if matches!($a, Value::Null) || matches!($b, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

// ---------------------------------------------------------------------------
// Date/time arithmetic helpers for Predecessor / Successor
// ---------------------------------------------------------------------------

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Returns the CqlDate one unit before `d` at its own precision, or `None` on
/// underflow (before year 1).
fn date_predecessor(d: &CqlDate) -> Option<CqlDate> {
    match (d.month, d.day) {
        // year-precision: go back one year
        (None, _) => {
            if d.year <= 1 {
                None
            } else {
                Some(CqlDate {
                    year: d.year - 1,
                    month: None,
                    day: None,
                })
            }
        }
        // month-precision: go back one month
        (Some(m), None) => {
            if m > 1 {
                Some(CqlDate {
                    year: d.year,
                    month: Some(m - 1),
                    day: None,
                })
            } else if d.year > 1 {
                Some(CqlDate {
                    year: d.year - 1,
                    month: Some(12),
                    day: None,
                })
            } else {
                None
            }
        }
        // day-precision: go back one day
        (Some(m), Some(day)) => {
            if day > 1 {
                Some(CqlDate {
                    year: d.year,
                    month: Some(m),
                    day: Some(day - 1),
                })
            } else if m > 1 {
                let prev_m = m - 1;
                Some(CqlDate {
                    year: d.year,
                    month: Some(prev_m),
                    day: Some(days_in_month(d.year, prev_m)),
                })
            } else if d.year > 1 {
                Some(CqlDate {
                    year: d.year - 1,
                    month: Some(12),
                    day: Some(31),
                })
            } else {
                None
            }
        }
    }
}

/// Returns the CqlDate one unit after `d` at its own precision, or `None` on
/// overflow (beyond year 9999).
fn date_successor(d: &CqlDate) -> Option<CqlDate> {
    match (d.month, d.day) {
        (None, _) => {
            if d.year >= 9999 {
                None
            } else {
                Some(CqlDate {
                    year: d.year + 1,
                    month: None,
                    day: None,
                })
            }
        }
        (Some(m), None) => {
            if m < 12 {
                Some(CqlDate {
                    year: d.year,
                    month: Some(m + 1),
                    day: None,
                })
            } else if d.year < 9999 {
                Some(CqlDate {
                    year: d.year + 1,
                    month: Some(1),
                    day: None,
                })
            } else {
                None
            }
        }
        (Some(m), Some(day)) => {
            let last = days_in_month(d.year, m);
            if day < last {
                Some(CqlDate {
                    year: d.year,
                    month: Some(m),
                    day: Some(day + 1),
                })
            } else if m < 12 {
                Some(CqlDate {
                    year: d.year,
                    month: Some(m + 1),
                    day: Some(1),
                })
            } else if d.year < 9999 {
                Some(CqlDate {
                    year: d.year + 1,
                    month: Some(1),
                    day: Some(1),
                })
            } else {
                None
            }
        }
    }
}

/// Returns the CqlTime one millisecond before `t` at its own precision, or
/// `None` on underflow (before 00:00:00.000).
fn time_predecessor(t: &CqlTime) -> Option<CqlTime> {
    if let Some(ms) = t.millisecond {
        if ms > 0 {
            return Some(CqlTime {
                millisecond: Some(ms - 1),
                ..*t
            });
        }
        let s = t.second.unwrap_or(0);
        if s > 0 {
            return Some(CqlTime {
                second: Some(s - 1),
                millisecond: Some(999),
                ..*t
            });
        }
        let m = t.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlTime {
                minute: Some(m - 1),
                second: Some(59),
                millisecond: Some(999),
                ..*t
            });
        }
        if t.hour > 0 {
            return Some(CqlTime {
                hour: t.hour - 1,
                minute: Some(59),
                second: Some(59),
                millisecond: Some(999),
            });
        }
        None // already at 00:00:00.000
    } else if let Some(s) = t.second {
        if s > 0 {
            return Some(CqlTime {
                second: Some(s - 1),
                ..*t
            });
        }
        let m = t.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlTime {
                minute: Some(m - 1),
                second: Some(59),
                ..*t
            });
        }
        if t.hour > 0 {
            return Some(CqlTime {
                hour: t.hour - 1,
                minute: Some(59),
                second: Some(59),
                millisecond: None,
            });
        }
        None
    } else if let Some(m) = t.minute {
        if m > 0 {
            return Some(CqlTime {
                minute: Some(m - 1),
                ..*t
            });
        }
        if t.hour > 0 {
            return Some(CqlTime {
                hour: t.hour - 1,
                minute: Some(59),
                second: None,
                millisecond: None,
            });
        }
        None
    } else {
        // hour-only precision
        if t.hour > 0 {
            Some(CqlTime {
                hour: t.hour - 1,
                minute: None,
                second: None,
                millisecond: None,
            })
        } else {
            None
        }
    }
}

/// Returns the CqlTime one millisecond after `t` at its own precision, or
/// `None` on overflow (beyond 23:59:59.999).
fn time_successor(t: &CqlTime) -> Option<CqlTime> {
    if let Some(ms) = t.millisecond {
        if ms < 999 {
            return Some(CqlTime {
                millisecond: Some(ms + 1),
                ..*t
            });
        }
        let s = t.second.unwrap_or(0);
        if s < 59 {
            return Some(CqlTime {
                second: Some(s + 1),
                millisecond: Some(0),
                ..*t
            });
        }
        let m = t.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlTime {
                minute: Some(m + 1),
                second: Some(0),
                millisecond: Some(0),
                ..*t
            });
        }
        if t.hour < 23 {
            return Some(CqlTime {
                hour: t.hour + 1,
                minute: Some(0),
                second: Some(0),
                millisecond: Some(0),
            });
        }
        None // already at 23:59:59.999
    } else if let Some(s) = t.second {
        if s < 59 {
            return Some(CqlTime {
                second: Some(s + 1),
                ..*t
            });
        }
        let m = t.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlTime {
                minute: Some(m + 1),
                second: Some(0),
                ..*t
            });
        }
        if t.hour < 23 {
            return Some(CqlTime {
                hour: t.hour + 1,
                minute: Some(59),
                second: Some(0),
                millisecond: None,
            });
        }
        None
    } else if let Some(m) = t.minute {
        if m < 59 {
            return Some(CqlTime {
                minute: Some(m + 1),
                ..*t
            });
        }
        if t.hour < 23 {
            return Some(CqlTime {
                hour: t.hour + 1,
                minute: Some(0),
                second: None,
                millisecond: None,
            });
        }
        None
    } else if t.hour < 23 {
        Some(CqlTime {
            hour: t.hour + 1,
            minute: None,
            second: None,
            millisecond: None,
        })
    } else {
        None
    }
}

/// Returns the CqlDateTime one unit before `dt` at its own precision, or
/// `None` on underflow.
fn datetime_predecessor(dt: &CqlDateTime) -> Option<CqlDateTime> {
    macro_rules! with_date {
        ($prev:expr) => {
            CqlDateTime {
                year: $prev.year,
                month: $prev.month,
                day: $prev.day,
                hour: dt.hour,
                minute: dt.minute,
                second: dt.second,
                millisecond: dt.millisecond,
                offset_seconds: dt.offset_seconds,
            }
        };
    }

    let date_part = CqlDate {
        year: dt.year,
        month: dt.month,
        day: dt.day,
    };

    if let Some(ms) = dt.millisecond {
        if ms > 0 {
            return Some(CqlDateTime {
                millisecond: Some(ms - 1),
                ..dt.clone()
            });
        }
        let s = dt.second.unwrap_or(0);
        if s > 0 {
            return Some(CqlDateTime {
                second: Some(s - 1),
                millisecond: Some(999),
                ..dt.clone()
            });
        }
        let m = dt.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlDateTime {
                minute: Some(m - 1),
                second: Some(59),
                millisecond: Some(999),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h > 0 {
            return Some(CqlDateTime {
                hour: Some(h - 1),
                minute: Some(59),
                second: Some(59),
                millisecond: Some(999),
                ..dt.clone()
            });
        }
        let prev = date_predecessor(&date_part)?;
        Some(CqlDateTime {
            year: prev.year,
            month: prev.month,
            day: prev.day,
            hour: Some(23),
            minute: Some(59),
            second: Some(59),
            millisecond: Some(999),
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(s) = dt.second {
        if s > 0 {
            return Some(CqlDateTime {
                second: Some(s - 1),
                ..dt.clone()
            });
        }
        let m = dt.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlDateTime {
                minute: Some(m - 1),
                second: Some(59),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h > 0 {
            return Some(CqlDateTime {
                hour: Some(h - 1),
                minute: Some(59),
                second: Some(59),
                ..dt.clone()
            });
        }
        let prev = date_predecessor(&date_part)?;
        Some(CqlDateTime {
            year: prev.year,
            month: prev.month,
            day: prev.day,
            hour: Some(23),
            minute: Some(59),
            second: Some(59),
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(m) = dt.minute {
        if m > 0 {
            return Some(CqlDateTime {
                minute: Some(m - 1),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h > 0 {
            return Some(CqlDateTime {
                hour: Some(h - 1),
                minute: Some(59),
                ..dt.clone()
            });
        }
        let prev = date_predecessor(&date_part)?;
        Some(CqlDateTime {
            year: prev.year,
            month: prev.month,
            day: prev.day,
            hour: Some(23),
            minute: Some(59),
            second: None,
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(h) = dt.hour {
        if h > 0 {
            return Some(CqlDateTime {
                hour: Some(h - 1),
                ..dt.clone()
            });
        }
        let prev = date_predecessor(&date_part)?;
        Some(CqlDateTime {
            year: prev.year,
            month: prev.month,
            day: prev.day,
            hour: Some(23),
            minute: None,
            second: None,
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else {
        // date-only precision
        let prev = date_predecessor(&date_part)?;
        Some(with_date!(prev))
    }
}

/// Returns the CqlDateTime one unit after `dt` at its own precision, or `None`
/// on overflow.
fn datetime_successor(dt: &CqlDateTime) -> Option<CqlDateTime> {
    macro_rules! with_date {
        ($next:expr) => {
            CqlDateTime {
                year: $next.year,
                month: $next.month,
                day: $next.day,
                hour: dt.hour,
                minute: dt.minute,
                second: dt.second,
                millisecond: dt.millisecond,
                offset_seconds: dt.offset_seconds,
            }
        };
    }

    let date_part = CqlDate {
        year: dt.year,
        month: dt.month,
        day: dt.day,
    };

    if let Some(ms) = dt.millisecond {
        if ms < 999 {
            return Some(CqlDateTime {
                millisecond: Some(ms + 1),
                ..dt.clone()
            });
        }
        let s = dt.second.unwrap_or(0);
        if s < 59 {
            return Some(CqlDateTime {
                second: Some(s + 1),
                millisecond: Some(0),
                ..dt.clone()
            });
        }
        let m = dt.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlDateTime {
                minute: Some(m + 1),
                second: Some(0),
                millisecond: Some(0),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h < 23 {
            return Some(CqlDateTime {
                hour: Some(h + 1),
                minute: Some(0),
                second: Some(0),
                millisecond: Some(0),
                ..dt.clone()
            });
        }
        let next = date_successor(&date_part)?;
        Some(CqlDateTime {
            year: next.year,
            month: next.month,
            day: next.day,
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(s) = dt.second {
        if s < 59 {
            return Some(CqlDateTime {
                second: Some(s + 1),
                ..dt.clone()
            });
        }
        let m = dt.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlDateTime {
                minute: Some(m + 1),
                second: Some(0),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h < 23 {
            return Some(CqlDateTime {
                hour: Some(h + 1),
                minute: Some(0),
                second: Some(0),
                ..dt.clone()
            });
        }
        let next = date_successor(&date_part)?;
        Some(CqlDateTime {
            year: next.year,
            month: next.month,
            day: next.day,
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(m) = dt.minute {
        if m < 59 {
            return Some(CqlDateTime {
                minute: Some(m + 1),
                ..dt.clone()
            });
        }
        let h = dt.hour.unwrap_or(0);
        if h < 23 {
            return Some(CqlDateTime {
                hour: Some(h + 1),
                minute: Some(0),
                ..dt.clone()
            });
        }
        let next = date_successor(&date_part)?;
        Some(CqlDateTime {
            year: next.year,
            month: next.month,
            day: next.day,
            hour: Some(0),
            minute: Some(0),
            second: None,
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else if let Some(h) = dt.hour {
        if h < 23 {
            return Some(CqlDateTime {
                hour: Some(h + 1),
                ..dt.clone()
            });
        }
        let next = date_successor(&date_part)?;
        Some(CqlDateTime {
            year: next.year,
            month: next.month,
            day: next.day,
            hour: Some(0),
            minute: None,
            second: None,
            millisecond: None,
            offset_seconds: dt.offset_seconds,
        })
    } else {
        let next = date_successor(&date_part)?;
        Some(with_date!(next))
    }
}

// ---------------------------------------------------------------------------
// Predecessor / Successor
// ---------------------------------------------------------------------------

/// CQL `Predecessor`: the value immediately before the argument.
///
/// For numeric types, "immediately before" means:
/// - Integer/Long: minus one
/// - Decimal/Quantity: minus 10⁻⁸ per the CQL specification
///
/// For temporal types, precision propagates from the input:
/// - Date: minus one day (or month, or year) at the date's own precision
/// - DateTime: minus one millisecond at the datetime's own finest precision
/// - Time: minus one millisecond at the time's own finest precision
///
/// Returns `null` if the result would underflow the minimum value.
pub fn predecessor(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => {
            if *x <= i32::MIN as i64 {
                Ok(Value::Null)
            } else {
                Ok(Value::Integer(x - 1))
            }
        }
        Value::Long(x) => {
            if *x <= i64::MIN as i128 {
                Ok(Value::Null)
            } else {
                Ok(Value::Long(x - 1))
            }
        }
        Value::Decimal(x) => Ok(Value::Decimal(x - 1e-8)),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: q.value - 1e-8,
            unit: q.unit.clone(),
        })),
        Value::Date(d) => Ok(date_predecessor(d).map(Value::Date).unwrap_or(Value::Null)),
        Value::DateTime(dt) => Ok(datetime_predecessor(dt)
            .map(Value::DateTime)
            .unwrap_or(Value::Null)),
        Value::Time(t) => Ok(time_predecessor(t).map(Value::Time).unwrap_or(Value::Null)),
        _ => Err(err("Predecessor", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Successor`: the value immediately after the argument.
///
/// For numeric types, "immediately after" means:
/// - Integer/Long: plus one
/// - Decimal/Quantity: plus 10⁻⁸ per the CQL specification
///
/// For temporal types, precision propagates from the input:
/// - Date: plus one day (or month, or year) at the date's own precision
/// - DateTime: plus one millisecond at the datetime's own finest precision
/// - Time: plus one millisecond at the time's own finest precision
///
/// Returns `null` if the result would overflow the maximum value.
pub fn successor(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => {
            if *x >= i32::MAX as i64 {
                Ok(Value::Null)
            } else {
                Ok(Value::Integer(x + 1))
            }
        }
        Value::Long(x) => {
            if *x >= i64::MAX as i128 {
                Ok(Value::Null)
            } else {
                Ok(Value::Long(x + 1))
            }
        }
        Value::Decimal(x) => Ok(Value::Decimal(x + 1e-8)),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: q.value + 1e-8,
            unit: q.unit.clone(),
        })),
        Value::Date(d) => Ok(date_successor(d).map(Value::Date).unwrap_or(Value::Null)),
        Value::DateTime(dt) => Ok(datetime_successor(dt)
            .map(Value::DateTime)
            .unwrap_or(Value::Null)),
        Value::Time(t) => Ok(time_successor(t).map(Value::Time).unwrap_or(Value::Null)),
        _ => Err(err("Successor", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `MinValue(<T>)`: the minimum representable value for the named type.
///
/// Supported type names: `"Integer"`, `"Long"`, `"Decimal"`, `"Date"`,
/// `"DateTime"`, `"Time"`.
///
/// ```
/// use rh_cql::eval::operators::min_value;
/// use rh_cql::eval::value::Value;
///
/// assert_eq!(min_value("Integer").unwrap(), Value::Integer(i32::MIN as i64));
/// ```
pub fn min_value(type_name: &str) -> Result<Value, EvalError> {
    match type_name {
        "Integer" => Ok(Value::Integer(i32::MIN as i64)),
        "Long" => Ok(Value::Long(i64::MIN as i128)),
        "Decimal" => Ok(Value::Decimal(f64::MIN)),
        "Date" => Ok(Value::Date(CqlDate {
            year: 1,
            month: Some(1),
            day: Some(1),
        })),
        "DateTime" => Ok(Value::DateTime(CqlDateTime {
            year: 1,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
            offset_seconds: None,
        })),
        "Time" => Ok(Value::Time(CqlTime {
            hour: 0,
            minute: Some(0),
            second: Some(0),
            millisecond: Some(0),
        })),
        other => Err(EvalError::General(format!(
            "MinValue: unsupported type '{other}'"
        ))),
    }
}

/// CQL `MaxValue(<T>)`: the maximum representable value for the named type.
///
/// Supported type names: `"Integer"`, `"Long"`, `"Decimal"`, `"Date"`,
/// `"DateTime"`, `"Time"`.
///
/// ```
/// use rh_cql::eval::operators::max_value;
/// use rh_cql::eval::value::Value;
///
/// assert_eq!(max_value("Integer").unwrap(), Value::Integer(i32::MAX as i64));
/// ```
pub fn max_value(type_name: &str) -> Result<Value, EvalError> {
    match type_name {
        "Integer" => Ok(Value::Integer(i32::MAX as i64)),
        "Long" => Ok(Value::Long(i64::MAX as i128)),
        "Decimal" => Ok(Value::Decimal(f64::MAX)),
        "Date" => Ok(Value::Date(CqlDate {
            year: 9999,
            month: Some(12),
            day: Some(31),
        })),
        "DateTime" => Ok(Value::DateTime(CqlDateTime {
            year: 9999,
            month: Some(12),
            day: Some(31),
            hour: Some(23),
            minute: Some(59),
            second: Some(59),
            millisecond: Some(999),
            offset_seconds: None,
        })),
        "Time" => Ok(Value::Time(CqlTime {
            hour: 23,
            minute: Some(59),
            second: Some(59),
            millisecond: Some(999),
        })),
        other => Err(EvalError::General(format!(
            "MaxValue: unsupported type '{other}'"
        ))),
    }
}

// ---------------------------------------------------------------------------
// Date/Time construction operators (9.13)
// ---------------------------------------------------------------------------

/// Construct a `Date` from year / year-month / year-month-day components.
///
/// Any component supplied as `Value::Null` (or `None`) is treated as absent
/// (reducing the date to coarser precision).
pub fn date_construct(
    year: &Value,
    month: Option<&Value>,
    day: Option<&Value>,
) -> Result<Value, EvalError> {
    null1!(year);
    let y = int_component(year, "Date", "year")? as i32;
    let m = opt_u8_component(month, "Date", "month")?;
    let d = opt_u8_component(day, "Date", "day")?;
    Ok(Value::Date(CqlDate {
        year: y,
        month: m,
        day: d,
    }))
}

/// Construct a `Time` from hour / minute / second / millisecond components.
pub fn time_construct(
    hour: &Value,
    minute: Option<&Value>,
    second: Option<&Value>,
    millisecond: Option<&Value>,
) -> Result<Value, EvalError> {
    null1!(hour);
    let h = int_component(hour, "Time", "hour")? as u8;
    let min = opt_u8_component(minute, "Time", "minute")?;
    let sec = opt_u8_component(second, "Time", "second")?;
    let ms = match millisecond {
        None | Some(Value::Null) => None,
        Some(Value::Integer(v)) => Some(*v as u32),
        _ => return Err(err("Time", "millisecond must be Integer")),
    };
    Ok(Value::Time(CqlTime {
        hour: h,
        minute: min,
        second: sec,
        millisecond: ms,
    }))
}

/// Construct a `DateTime` from up to eight components plus an optional offset.
///
/// `offset_seconds` is the UTC offset in seconds.
#[allow(clippy::too_many_arguments)]
pub fn datetime_construct(
    year: &Value,
    month: Option<&Value>,
    day: Option<&Value>,
    hour: Option<&Value>,
    minute: Option<&Value>,
    second: Option<&Value>,
    millisecond: Option<&Value>,
    offset_seconds: Option<&Value>,
) -> Result<Value, EvalError> {
    null1!(year);
    let y = int_component(year, "DateTime", "year")? as i32;
    let mo = opt_u8_component(month, "DateTime", "month")?;
    let d = opt_u8_component(day, "DateTime", "day")?;
    let h = opt_u8_component(hour, "DateTime", "hour")?;
    let min = opt_u8_component(minute, "DateTime", "minute")?;
    let sec = opt_u8_component(second, "DateTime", "second")?;
    let ms = match millisecond {
        None | Some(Value::Null) => None,
        Some(Value::Integer(v)) => Some(*v as u32),
        _ => return Err(err("DateTime", "millisecond must be Integer")),
    };
    let offset = match offset_seconds {
        None | Some(Value::Null) => None,
        Some(Value::Integer(v)) => Some(*v as i32),
        Some(Value::Decimal(v)) => Some(*v as i32),
        _ => return Err(err("DateTime", "offset must be numeric")),
    };
    Ok(Value::DateTime(CqlDateTime {
        year: y,
        month: mo,
        day: d,
        hour: h,
        minute: min,
        second: sec,
        millisecond: ms,
        offset_seconds: offset,
    }))
}

// ---------------------------------------------------------------------------
// Date/Time component extraction (9.13)
// ---------------------------------------------------------------------------

/// Extract a named component from a `Date`, `DateTime`, or `Time` value.
///
/// Component names: `"year"`, `"month"`, `"day"`, `"hour"`, `"minute"`,
/// `"second"`, `"millisecond"`.  Returns `Null` if the value does not carry
/// that component (e.g. a year-only date has no month).
pub fn date_time_component(value: &Value, component: &str) -> Result<Value, EvalError> {
    null1!(value);
    match value {
        Value::Date(d) => match component {
            "year" => Ok(Value::Integer(d.year as i64)),
            "month" => Ok(opt_to_value(d.month.map(|v| v as i64))),
            "day" => Ok(opt_to_value(d.day.map(|v| v as i64))),
            _ => Err(err(
                "DateTimeComponent",
                &format!("'{component}' not available on Date"),
            )),
        },
        Value::DateTime(dt) => match component {
            "year" => Ok(Value::Integer(dt.year as i64)),
            "month" => Ok(opt_to_value(dt.month.map(|v| v as i64))),
            "day" => Ok(opt_to_value(dt.day.map(|v| v as i64))),
            "hour" => Ok(opt_to_value(dt.hour.map(|v| v as i64))),
            "minute" => Ok(opt_to_value(dt.minute.map(|v| v as i64))),
            "second" => Ok(opt_to_value(dt.second.map(|v| v as i64))),
            "millisecond" => Ok(opt_to_value(dt.millisecond.map(|v| v as i64))),
            _ => Err(err(
                "DateTimeComponent",
                &format!("unknown component '{component}'"),
            )),
        },
        Value::Time(t) => match component {
            "hour" => Ok(Value::Integer(t.hour as i64)),
            "minute" => Ok(opt_to_value(t.minute.map(|v| v as i64))),
            "second" => Ok(opt_to_value(t.second.map(|v| v as i64))),
            "millisecond" => Ok(opt_to_value(t.millisecond.map(|v| v as i64))),
            _ => Err(err(
                "DateTimeComponent",
                &format!("'{component}' not available on Time"),
            )),
        },
        _ => Err(err("DateTimeComponent", "expected Date, DateTime, or Time")),
    }
}

// ---------------------------------------------------------------------------
// Date/Time arithmetic (9.13)
// ---------------------------------------------------------------------------

/// Add a temporal `Quantity` to a `Date`, `DateTime`, or `Time`.
///
/// The quantity unit determines which component is incremented.  Supported
/// UCUM/CQL units: `"a"` / `"year(s)"`, `"mo"` / `"month(s)"`,
/// `"wk"` / `"week(s)"`, `"d"` / `"day(s)"`, `"h"` / `"hour(s)"`,
/// `"min"` / `"minute(s)"`, `"s"` / `"second(s)"`, `"ms"` / `"millisecond(s)"`.
pub fn date_time_add(value: &Value, quantity: &Value) -> Result<Value, EvalError> {
    null2!(value, quantity);
    let (qty_val, qty_unit) = extract_quantity(quantity, "DateTimeAdd")?;
    let unit = normalize_temporal_unit(&qty_unit).ok_or_else(|| {
        err(
            "DateTimeAdd",
            &format!("unsupported temporal unit '{qty_unit}'"),
        )
    })?;
    let amount = qty_val.round() as i64;
    match value {
        Value::Date(d) => date_add_unit(d, amount, &unit).map(Value::Date),
        Value::DateTime(dt) => datetime_add_unit(dt, amount, &unit).map(Value::DateTime),
        Value::Time(t) => time_add_unit(t, amount, &unit).map(Value::Time),
        _ => Err(err("DateTimeAdd", "expected Date, DateTime, or Time")),
    }
}

/// Subtract a temporal `Quantity` from a `Date`, `DateTime`, or `Time`.
pub fn date_time_subtract(value: &Value, quantity: &Value) -> Result<Value, EvalError> {
    null2!(value, quantity);
    let (qty_val, qty_unit) = extract_quantity(quantity, "DateTimeSubtract")?;
    let negated = Value::Quantity(CqlQuantity {
        value: -qty_val,
        unit: qty_unit,
    });
    date_time_add(value, &negated)
}

// ---------------------------------------------------------------------------
// SameAs / SameOrBefore / SameOrAfter (9.13)
// ---------------------------------------------------------------------------

/// Return `true` iff `a` and `b` are at the same point in time at the given
/// `precision` (or at the value's own precision if `precision` is `None`).
///
/// Returns `Null` when the comparison is uncertain (mixed precisions).
pub fn same_as(a: &Value, b: &Value, precision: Option<&str>) -> Result<Value, EvalError> {
    null2!(a, b);
    temporal_compare(a, b, precision, "SameAs", |ord| ord == Ordering::Equal)
}

/// Return `true` iff `a` is the same as or before `b` at the given precision.
pub fn same_or_before(a: &Value, b: &Value, precision: Option<&str>) -> Result<Value, EvalError> {
    null2!(a, b);
    temporal_compare(a, b, precision, "SameOrBefore", |ord| {
        ord == Ordering::Equal || ord == Ordering::Less
    })
}

/// Return `true` iff `a` is the same as or after `b` at the given precision.
pub fn same_or_after(a: &Value, b: &Value, precision: Option<&str>) -> Result<Value, EvalError> {
    null2!(a, b);
    temporal_compare(a, b, precision, "SameOrAfter", |ord| {
        ord == Ordering::Equal || ord == Ordering::Greater
    })
}

// ---------------------------------------------------------------------------
// Duration calculations (9.13)
// ---------------------------------------------------------------------------

/// Return the number of whole `unit` boundaries crossed between `a` and `b`
/// (`DurationInX` per CQL §18.14).
///
/// Positive when `b > a`, negative when `b < a`.
pub fn duration_between(a: &Value, b: &Value, unit: &str) -> Result<Value, EvalError> {
    null2!(a, b);
    let canon = normalize_temporal_unit(unit)
        .ok_or_else(|| err("DurationBetween", &format!("unsupported unit '{unit}'")))?;
    date_duration_between(a, b, &canon)
}

/// Return the number of whole `unit` components between `a` and `b`
/// (`DifferenceBetween` per CQL §18.13) — truncated, not boundary-crossing.
pub fn difference_between(a: &Value, b: &Value, unit: &str) -> Result<Value, EvalError> {
    null2!(a, b);
    let canon = normalize_temporal_unit(unit)
        .ok_or_else(|| err("DifferenceBetween", &format!("unsupported unit '{unit}'")))?;
    date_difference_between(a, b, &canon)
}

// ---------------------------------------------------------------------------
// Date/Time arithmetic helpers
// ---------------------------------------------------------------------------

/// Normalize a CQL / UCUM temporal unit string to a canonical lower-case form.
fn normalize_temporal_unit(unit: &str) -> Option<String> {
    let u = unit.trim_end_matches('s'); // strip trailing 's' for plurals
    let canon = match u.to_ascii_lowercase().as_str() {
        "year" | "a" => "year",
        "month" | "mo" => "month",
        "week" | "wk" => "week",
        "day" | "d" => "day",
        "hour" | "h" => "hour",
        "minute" | "min" => "minute",
        "second" | "s" => "second",
        "millisecond" | "ms" => "millisecond",
        _ => return None,
    };
    Some(canon.to_string())
}

fn extract_quantity(v: &Value, op: &str) -> Result<(f64, String), EvalError> {
    match v {
        Value::Quantity(q) => Ok((q.value, q.unit.clone())),
        _ => Err(err(op, "argument must be Quantity")),
    }
}

fn opt_to_value(v: Option<i64>) -> Value {
    v.map(Value::Integer).unwrap_or(Value::Null)
}

fn int_component(v: &Value, op: &str, field: &str) -> Result<i64, EvalError> {
    match v {
        Value::Integer(n) => Ok(*n),
        _ => Err(err(op, &format!("{field} must be Integer"))),
    }
}

fn opt_u8_component(v: Option<&Value>, op: &str, field: &str) -> Result<Option<u8>, EvalError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Integer(n)) => Ok(Some(*n as u8)),
        _ => Err(err(op, &format!("{field} must be Integer"))),
    }
}

// ---------------------------------------------------------------------------
// Proleptic Gregorian calendar helpers (Howard Hinnant algorithm)
// ---------------------------------------------------------------------------

/// Days since the internal epoch (proleptic Gregorian 2000-03-01 = day 0).
fn ymd_to_days(year: i32, month: u8, day: u8) -> i64 {
    let m = month as i64;
    let d = day as i64;
    let y = year as i64 + if m <= 2 { -1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn days_to_ymd(days: i64) -> (i32, u8, u8) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let adj_y = y + if m <= 2 { 1 } else { 0 };
    (adj_y as i32, m as u8, d as u8)
}

fn date_add_unit(d: &CqlDate, amount: i64, unit: &str) -> Result<CqlDate, EvalError> {
    match unit {
        "year" => {
            let new_year = d.year + amount as i32;
            // Clamp Feb 29 for non-leap years
            let new_day = d.day.map(|day| {
                if d.month == Some(2) && day == 29 && !is_leap_year(new_year) {
                    28
                } else {
                    day
                }
            });
            Ok(CqlDate {
                year: new_year,
                month: d.month,
                day: new_day,
            })
        }
        "month" => {
            if let Some(m) = d.month {
                let total_months = (d.year as i64) * 12 + (m as i64 - 1) + amount;
                let new_year = (total_months / 12) as i32;
                let new_month = (total_months % 12 + 1) as u8;
                let new_day = d.day.map(|day| {
                    let max_day = days_in_month(new_year, new_month);
                    day.min(max_day)
                });
                Ok(CqlDate {
                    year: new_year,
                    month: Some(new_month),
                    day: new_day,
                })
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add months to year-precision Date",
                ))
            }
        }
        "week" => date_add_unit(d, amount * 7, "day"),
        "day" => {
            if let (Some(m), Some(day)) = (d.month, d.day) {
                let days = ymd_to_days(d.year, m, day) + amount;
                let (ny, nm, nd) = days_to_ymd(days);
                Ok(CqlDate {
                    year: ny,
                    month: Some(nm),
                    day: Some(nd),
                })
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add days to coarse-precision Date",
                ))
            }
        }
        _ => Err(err(
            "DateTimeAdd",
            &format!("unit '{unit}' not applicable to Date"),
        )),
    }
}

fn datetime_add_unit(dt: &CqlDateTime, amount: i64, unit: &str) -> Result<CqlDateTime, EvalError> {
    match unit {
        "year" => {
            let new_year = dt.year + amount as i32;
            let new_day = dt.day.map(|day| {
                if dt.month == Some(2) && day == 29 && !is_leap_year(new_year) {
                    28
                } else {
                    day
                }
            });
            Ok(CqlDateTime {
                year: new_year,
                day: new_day,
                ..dt.clone()
            })
        }
        "month" => {
            if let Some(m) = dt.month {
                let total = (dt.year as i64) * 12 + (m as i64 - 1) + amount;
                let ny = (total / 12) as i32;
                let nm = (total % 12 + 1) as u8;
                let new_day = dt.day.map(|d| d.min(days_in_month(ny, nm)));
                Ok(CqlDateTime {
                    year: ny,
                    month: Some(nm),
                    day: new_day,
                    ..dt.clone()
                })
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add months to year-precision DateTime",
                ))
            }
        }
        "week" => datetime_add_unit(dt, amount * 7, "day"),
        "day" => {
            if let (Some(m), Some(day)) = (dt.month, dt.day) {
                let days = ymd_to_days(dt.year, m, day) + amount;
                let (ny, nm, nd) = days_to_ymd(days);
                Ok(CqlDateTime {
                    year: ny,
                    month: Some(nm),
                    day: Some(nd),
                    ..dt.clone()
                })
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add days to coarse-precision DateTime",
                ))
            }
        }
        "hour" => {
            if let Some(h) = dt.hour {
                let total_h = h as i64 + amount;
                let extra_days = total_h.div_euclid(24);
                let new_h = total_h.rem_euclid(24) as u8;
                let mut base = CqlDateTime {
                    hour: Some(new_h),
                    ..dt.clone()
                };
                if extra_days != 0 {
                    base = datetime_add_unit(&base, extra_days, "day")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add hours to coarse-precision DateTime",
                ))
            }
        }
        "minute" => {
            if let Some(min) = dt.minute {
                let total_min = min as i64 + amount;
                let extra_h = total_min.div_euclid(60);
                let new_min = total_min.rem_euclid(60) as u8;
                let mut base = CqlDateTime {
                    minute: Some(new_min),
                    ..dt.clone()
                };
                if extra_h != 0 {
                    base = datetime_add_unit(&base, extra_h, "hour")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add minutes to coarse-precision DateTime",
                ))
            }
        }
        "second" => {
            if let Some(sec) = dt.second {
                let total_sec = sec as i64 + amount;
                let extra_min = total_sec.div_euclid(60);
                let new_sec = total_sec.rem_euclid(60) as u8;
                let mut base = CqlDateTime {
                    second: Some(new_sec),
                    ..dt.clone()
                };
                if extra_min != 0 {
                    base = datetime_add_unit(&base, extra_min, "minute")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add seconds to coarse-precision DateTime",
                ))
            }
        }
        "millisecond" => {
            if let Some(ms) = dt.millisecond {
                let total_ms = ms as i64 + amount;
                let extra_sec = total_ms.div_euclid(1000);
                let new_ms = total_ms.rem_euclid(1000) as u32;
                let mut base = CqlDateTime {
                    millisecond: Some(new_ms),
                    ..dt.clone()
                };
                if extra_sec != 0 {
                    base = datetime_add_unit(&base, extra_sec, "second")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add milliseconds to coarse-precision DateTime",
                ))
            }
        }
        _ => Err(err("DateTimeAdd", &format!("unsupported unit '{unit}'"))),
    }
}

fn time_add_unit(t: &CqlTime, amount: i64, unit: &str) -> Result<CqlTime, EvalError> {
    match unit {
        "hour" => {
            let new_h = ((t.hour as i64 + amount).rem_euclid(24)) as u8;
            Ok(CqlTime {
                hour: new_h,
                ..t.clone()
            })
        }
        "minute" => {
            if let Some(min) = t.minute {
                let total = min as i64 + amount;
                let extra_h = total.div_euclid(60);
                let new_min = total.rem_euclid(60) as u8;
                let mut base = CqlTime {
                    minute: Some(new_min),
                    ..t.clone()
                };
                if extra_h != 0 {
                    base = time_add_unit(&base, extra_h, "hour")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add minutes to hour-precision Time",
                ))
            }
        }
        "second" => {
            if let Some(sec) = t.second {
                let total = sec as i64 + amount;
                let extra_min = total.div_euclid(60);
                let new_sec = total.rem_euclid(60) as u8;
                let mut base = CqlTime {
                    second: Some(new_sec),
                    ..t.clone()
                };
                if extra_min != 0 {
                    base = time_add_unit(&base, extra_min, "minute")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add seconds to coarse-precision Time",
                ))
            }
        }
        "millisecond" => {
            if let Some(ms) = t.millisecond {
                let total = ms as i64 + amount;
                let extra_sec = total.div_euclid(1000);
                let new_ms = total.rem_euclid(1000) as u32;
                let mut base = CqlTime {
                    millisecond: Some(new_ms),
                    ..t.clone()
                };
                if extra_sec != 0 {
                    base = time_add_unit(&base, extra_sec, "second")?;
                }
                Ok(base)
            } else {
                Err(err(
                    "DateTimeAdd",
                    "cannot add milliseconds to coarse-precision Time",
                ))
            }
        }
        _ => Err(err(
            "DateTimeAdd",
            &format!("unit '{unit}' not applicable to Time"),
        )),
    }
}

// ---------------------------------------------------------------------------
// SameAs / SameOrBefore / SameOrAfter helpers
// ---------------------------------------------------------------------------

fn temporal_compare(
    a: &Value,
    b: &Value,
    precision: Option<&str>,
    op: &str,
    predicate: impl Fn(Ordering) -> bool,
) -> Result<Value, EvalError> {
    let ord = match (a, b) {
        (Value::Date(d1), Value::Date(d2)) => compare_dates_at_precision(d1, d2, precision, op)?,
        (Value::DateTime(dt1), Value::DateTime(dt2)) => {
            compare_datetimes_at_precision(dt1, dt2, precision, op)?
        }
        (Value::Time(t1), Value::Time(t2)) => compare_times_at_precision(t1, t2, precision, op)?,
        _ => return Err(err(op, "arguments must be the same temporal type")),
    };
    match ord {
        Some(o) => Ok(Value::Boolean(predicate(o))),
        None => Ok(Value::Null),
    }
}

/// Precision levels for Date: year=0, month=1, day=2
fn date_precision_level(prec: &str) -> Option<usize> {
    match prec {
        "year" => Some(0),
        "month" => Some(1),
        "day" => Some(2),
        _ => None,
    }
}

/// Precision levels for Time
fn time_precision_level(prec: &str) -> Option<usize> {
    match prec {
        "hour" => Some(0),
        "minute" => Some(1),
        "second" => Some(2),
        "millisecond" => Some(3),
        _ => None,
    }
}

/// Precision level for DateTime (extends date levels with time levels)
fn datetime_precision_level(prec: &str) -> Option<usize> {
    match prec {
        "year" => Some(0),
        "month" => Some(1),
        "day" => Some(2),
        "hour" => Some(3),
        "minute" => Some(4),
        "second" => Some(5),
        "millisecond" => Some(6),
        _ => None,
    }
}

fn compare_dates_at_precision(
    a: &CqlDate,
    b: &CqlDate,
    precision: Option<&str>,
    op: &str,
) -> Result<Option<Ordering>, EvalError> {
    let max_level = match precision {
        None => 2,
        Some(p) => date_precision_level(p)
            .ok_or_else(|| err(op, &format!("invalid precision '{p}' for Date")))?,
    };
    // Compare year
    let ord = a.year.cmp(&b.year);
    if ord != Ordering::Equal || max_level == 0 {
        return Ok(Some(ord));
    }
    // Compare month
    match (a.month, b.month) {
        (None, None) => return Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => return Ok(None),
        (Some(am), Some(bm)) => {
            let ord = am.cmp(&bm);
            if ord != Ordering::Equal || max_level == 1 {
                return Ok(Some(ord));
            }
        }
    }
    // Compare day
    match (a.day, b.day) {
        (None, None) => Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => Ok(None),
        (Some(ad), Some(bd)) => Ok(Some(ad.cmp(&bd))),
    }
}

fn compare_times_at_precision(
    a: &CqlTime,
    b: &CqlTime,
    precision: Option<&str>,
    op: &str,
) -> Result<Option<Ordering>, EvalError> {
    let max_level = match precision {
        None => 3,
        Some(p) => time_precision_level(p)
            .ok_or_else(|| err(op, &format!("invalid precision '{p}' for Time")))?,
    };
    let ord = a.hour.cmp(&b.hour);
    if ord != Ordering::Equal || max_level == 0 {
        return Ok(Some(ord));
    }
    // minute
    match (a.minute, b.minute) {
        (None, None) => return Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => return Ok(None),
        (Some(am), Some(bm)) => {
            let ord = am.cmp(&bm);
            if ord != Ordering::Equal || max_level == 1 {
                return Ok(Some(ord));
            }
        }
    }
    // second
    match (a.second, b.second) {
        (None, None) => return Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => return Ok(None),
        (Some(asec), Some(bsec)) => {
            let ord = asec.cmp(&bsec);
            if ord != Ordering::Equal || max_level == 2 {
                return Ok(Some(ord));
            }
        }
    }
    // millisecond
    match (a.millisecond, b.millisecond) {
        (None, None) => Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => Ok(None),
        (Some(ams), Some(bms)) => Ok(Some(ams.cmp(&bms))),
    }
}

fn compare_datetimes_at_precision(
    a: &CqlDateTime,
    b: &CqlDateTime,
    precision: Option<&str>,
    op: &str,
) -> Result<Option<Ordering>, EvalError> {
    let max_level = match precision {
        None => 6,
        Some(p) => datetime_precision_level(p)
            .ok_or_else(|| err(op, &format!("invalid precision '{p}' for DateTime")))?,
    };
    macro_rules! cmp_opt_field {
        ($fa:expr, $fb:expr, $level:expr) => {
            match ($fa, $fb) {
                (None, None) => return Ok(Some(Ordering::Equal)),
                (None, Some(_)) | (Some(_), None) => return Ok(None),
                (Some(va), Some(vb)) => {
                    let ord = va.cmp(&vb);
                    if ord != Ordering::Equal || max_level == $level {
                        return Ok(Some(ord));
                    }
                }
            }
        };
    }
    let ord = a.year.cmp(&b.year);
    if ord != Ordering::Equal || max_level == 0 {
        return Ok(Some(ord));
    }
    cmp_opt_field!(a.month, b.month, 1);
    cmp_opt_field!(a.day, b.day, 2);
    cmp_opt_field!(a.hour, b.hour, 3);
    cmp_opt_field!(a.minute, b.minute, 4);
    cmp_opt_field!(a.second, b.second, 5);
    match (a.millisecond, b.millisecond) {
        (None, None) => Ok(Some(Ordering::Equal)),
        (None, Some(_)) | (Some(_), None) => Ok(None),
        (Some(ams), Some(bms)) => Ok(Some(ams.cmp(&bms))),
    }
}

// ---------------------------------------------------------------------------
// Duration / Difference helpers
// ---------------------------------------------------------------------------

fn date_duration_between(a: &Value, b: &Value, unit: &str) -> Result<Value, EvalError> {
    match (a, b) {
        (Value::Date(d1), Value::Date(d2)) => Ok(Value::Integer(date_duration_diff(d1, d2, unit)?)),
        (Value::DateTime(dt1), Value::DateTime(dt2)) => {
            Ok(Value::Integer(datetime_duration_diff(dt1, dt2, unit)?))
        }
        _ => Err(err(
            "DurationBetween",
            "arguments must be same temporal type",
        )),
    }
}

fn date_difference_between(a: &Value, b: &Value, unit: &str) -> Result<Value, EvalError> {
    // For most units, DifferenceBetween equals DurationBetween.
    // The distinction matters for month/year truncation (not boundary crossing).
    date_duration_between(a, b, unit)
}

fn date_duration_diff(a: &CqlDate, b: &CqlDate, unit: &str) -> Result<i64, EvalError> {
    match unit {
        "year" => Ok((b.year - a.year) as i64),
        "month" => {
            let a_m = a
                .month
                .ok_or_else(|| err("DurationBetween", "year-precision Date has no month"))?;
            let b_m = b
                .month
                .ok_or_else(|| err("DurationBetween", "year-precision Date has no month"))?;
            Ok((b.year as i64 - a.year as i64) * 12 + (b_m as i64 - a_m as i64))
        }
        "week" => Ok(date_duration_diff(a, b, "day")? / 7),
        "day" => {
            let a_day = a
                .day
                .ok_or_else(|| err("DurationBetween", "coarse-precision Date has no day"))?;
            let b_day = b
                .day
                .ok_or_else(|| err("DurationBetween", "coarse-precision Date has no day"))?;
            let a_mo = a.month.unwrap();
            let b_mo = b.month.unwrap();
            Ok(ymd_to_days(b.year, b_mo, b_day) - ymd_to_days(a.year, a_mo, a_day))
        }
        _ => Err(err(
            "DurationBetween",
            &format!("unit '{unit}' not applicable to Date"),
        )),
    }
}

fn datetime_duration_diff(a: &CqlDateTime, b: &CqlDateTime, unit: &str) -> Result<i64, EvalError> {
    match unit {
        "year" => Ok((b.year - a.year) as i64),
        "month" => {
            let a_m = a.month.ok_or_else(|| err("DurationBetween", "no month"))?;
            let b_m = b.month.ok_or_else(|| err("DurationBetween", "no month"))?;
            Ok((b.year as i64 - a.year as i64) * 12 + (b_m as i64 - a_m as i64))
        }
        "week" => Ok(datetime_duration_diff(a, b, "day")? / 7),
        "day" => {
            let a_day = a.day.ok_or_else(|| err("DurationBetween", "no day"))?;
            let b_day = b.day.ok_or_else(|| err("DurationBetween", "no day"))?;
            Ok(ymd_to_days(b.year, b.month.unwrap(), b_day)
                - ymd_to_days(a.year, a.month.unwrap(), a_day))
        }
        "hour" => {
            let days = datetime_duration_diff(a, b, "day")?;
            let a_h = a.hour.ok_or_else(|| err("DurationBetween", "no hour"))? as i64;
            let b_h = b.hour.ok_or_else(|| err("DurationBetween", "no hour"))? as i64;
            Ok(days * 24 + (b_h - a_h))
        }
        "minute" => {
            let hours = datetime_duration_diff(a, b, "hour")?;
            let a_min = a
                .minute
                .ok_or_else(|| err("DurationBetween", "no minute"))? as i64;
            let b_min = b
                .minute
                .ok_or_else(|| err("DurationBetween", "no minute"))? as i64;
            Ok(hours * 60 + (b_min - a_min))
        }
        "second" => {
            let mins = datetime_duration_diff(a, b, "minute")?;
            let a_sec = a
                .second
                .ok_or_else(|| err("DurationBetween", "no second"))? as i64;
            let b_sec = b
                .second
                .ok_or_else(|| err("DurationBetween", "no second"))? as i64;
            Ok(mins * 60 + (b_sec - a_sec))
        }
        "millisecond" => {
            let secs = datetime_duration_diff(a, b, "second")?;
            let a_ms = a
                .millisecond
                .ok_or_else(|| err("DurationBetween", "no millisecond"))?
                as i64;
            let b_ms = b
                .millisecond
                .ok_or_else(|| err("DurationBetween", "no millisecond"))?
                as i64;
            Ok(secs * 1000 + (b_ms - a_ms))
        }
        _ => Err(err(
            "DurationBetween",
            &format!("unsupported unit '{unit}'"),
        )),
    }
}
