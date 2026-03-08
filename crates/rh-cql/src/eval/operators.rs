//! CQL arithmetic, comparison, string, and date/time operators.
//!
//! Implements:
//! - Arithmetic operators (CQL §12)
//! - Comparison operators (CQL §5)
//! - String operators (CQL §15): Concatenate, Combine, Split, SplitOnMatches,
//!   Length, Upper, Lower, StartsWith, EndsWith, Matches, ReplaceMatches,
//!   Indexer, PositionOf, LastPositionOf, Substring
//! - Date/time operators (CQL §18, §19): construction, component extraction,
//!   date arithmetic, SameAs, SameOrBefore, SameOrAfter, duration calculations
//!
//! ## Null propagation
//!
//! All operators propagate `null`: when any argument is [`Value::Null`] the
//! result is `Value::Null`, unless explicitly noted (e.g. [`min_value`] and
//! [`max_value`] take a type-name string, not a `Value`).
//!
//! ## Return type
//!
//! Functions return `Result<Value, EvalError>` so that type mismatches and
//! domain errors (division by zero, logarithm of negative number, etc.) can
//! be reported without panicking.

use std::cmp::Ordering;

use regex::Regex;

use super::context::EvalError;
use super::value::{CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};

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
// Ordering helper (used by comparison operators)
// ---------------------------------------------------------------------------

/// Compare two non-null CQL values and return their ordering, or `None` when
/// the values are incomparable (different types, incompatible precisions, or
/// units that cannot be converted).
pub fn cql_compare(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => x.partial_cmp(y),
        (Value::Long(x), Value::Long(y)) => x.partial_cmp(y),
        (Value::Decimal(x), Value::Decimal(y)) => x.partial_cmp(y),
        (Value::String(x), Value::String(y)) => Some(x.cmp(y)),
        (Value::Quantity(x), Value::Quantity(y)) if x.unit == y.unit => {
            x.value.partial_cmp(&y.value)
        }
        (Value::Date(x), Value::Date(y)) => compare_dates(x, y),
        (Value::DateTime(x), Value::DateTime(y)) => compare_datetimes(x, y),
        (Value::Time(x), Value::Time(y)) => compare_times(x, y),
        _ => None,
    }
}

fn compare_dates(a: &CqlDate, b: &CqlDate) -> Option<Ordering> {
    // Precisions must match for a well-defined comparison (per CQL spec).
    match (a.month, b.month) {
        (None, None) => a.year.partial_cmp(&b.year),
        (Some(am), Some(bm)) => {
            let yr = a.year.cmp(&b.year);
            if yr != Ordering::Equal {
                return Some(yr);
            }
            match (a.day, b.day) {
                (None, None) => am.partial_cmp(&bm),
                (Some(ad), Some(bd)) => {
                    let mo = am.cmp(&bm);
                    if mo != Ordering::Equal {
                        Some(mo)
                    } else {
                        ad.partial_cmp(&bd)
                    }
                }
                _ => None, // mismatched day precision → null
            }
        }
        _ => None, // mismatched month precision → null
    }
}

fn compare_times(a: &CqlTime, b: &CqlTime) -> Option<Ordering> {
    let h = a.hour.cmp(&b.hour);
    if h != Ordering::Equal {
        return Some(h);
    }
    match (a.minute, b.minute) {
        (None, None) => Some(Ordering::Equal),
        (Some(am), Some(bm)) => {
            let m = am.cmp(&bm);
            if m != Ordering::Equal {
                return Some(m);
            }
            match (a.second, b.second) {
                (None, None) => Some(Ordering::Equal),
                (Some(as_), Some(bs)) => {
                    let s = as_.cmp(&bs);
                    if s != Ordering::Equal {
                        return Some(s);
                    }
                    match (a.millisecond, b.millisecond) {
                        (None, None) => Some(Ordering::Equal),
                        (Some(ams), Some(bms)) => ams.partial_cmp(&bms),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn compare_datetimes(a: &CqlDateTime, b: &CqlDateTime) -> Option<Ordering> {
    // Compare using the CqlDate portion then time components.
    let da = CqlDate {
        year: a.year,
        month: a.month,
        day: a.day,
    };
    let db = CqlDate {
        year: b.year,
        month: b.month,
        day: b.day,
    };
    let date_ord = compare_dates(&da, &db)?;
    if date_ord != Ordering::Equal {
        return Some(date_ord);
    }
    let ta = CqlTime {
        hour: a.hour.unwrap_or(0),
        minute: a.minute,
        second: a.second,
        millisecond: a.millisecond,
    };
    let tb = CqlTime {
        hour: b.hour.unwrap_or(0),
        minute: b.minute,
        second: b.second,
        millisecond: b.millisecond,
    };
    if a.hour.is_none() && b.hour.is_none() {
        return Some(Ordering::Equal);
    }
    if a.hour.is_none() != b.hour.is_none() {
        return None; // mismatched precision
    }
    compare_times(&ta, &tb)
}

// ---------------------------------------------------------------------------
// Comparison operators (9.11)
// ---------------------------------------------------------------------------

/// CQL `Equal` (`=`): null-propagating structural equality.
///
/// Delegates to [`super::value::cql_equal`].
pub fn equal(a: &Value, b: &Value) -> Value {
    super::value::cql_equal(a, b)
}

/// CQL `Equivalent` (`~`): null-safe equivalence.
///
/// Delegates to [`super::value::cql_equivalent`].
pub fn equivalent(a: &Value, b: &Value) -> Value {
    Value::Boolean(super::value::cql_equivalent(a, b))
}

/// CQL `Less` (`<`): returns `null` when either operand is `null` or the
/// types are incomparable.
///
/// ```
/// use rh_cql::eval::operators::less;
/// use rh_cql::eval::value::Value;
///
/// assert_eq!(less(&Value::Integer(1), &Value::Integer(2)).unwrap(), Value::Boolean(true));
/// assert_eq!(less(&Value::Null, &Value::Integer(2)).unwrap(), Value::Null);
/// ```
pub fn less(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match cql_compare(a, b) {
        Some(ord) => Ok(Value::Boolean(ord == Ordering::Less)),
        None => Ok(Value::Null),
    }
}

/// CQL `Greater` (`>`).
///
/// ```
/// use rh_cql::eval::operators::greater;
/// use rh_cql::eval::value::Value;
///
/// assert_eq!(greater(&Value::Decimal(3.5), &Value::Decimal(2.0)).unwrap(), Value::Boolean(true));
/// ```
pub fn greater(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match cql_compare(a, b) {
        Some(ord) => Ok(Value::Boolean(ord == Ordering::Greater)),
        None => Ok(Value::Null),
    }
}

/// CQL `LessOrEqual` (`<=`).
pub fn less_or_equal(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match cql_compare(a, b) {
        Some(ord) => Ok(Value::Boolean(ord != Ordering::Greater)),
        None => Ok(Value::Null),
    }
}

/// CQL `GreaterOrEqual` (`>=`).
pub fn greater_or_equal(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match cql_compare(a, b) {
        Some(ord) => Ok(Value::Boolean(ord != Ordering::Less)),
        None => Ok(Value::Null),
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operators (9.10)
// ---------------------------------------------------------------------------

/// CQL `Add` (`+`): Integer + Integer → Integer, Long + Long → Long,
/// Decimal + Decimal → Decimal, Quantity + Quantity (same unit) → Quantity.
pub fn add(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x + y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x + y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x + y)),
        (Value::Quantity(x), Value::Quantity(y)) if x.unit == y.unit => {
            Ok(Value::Quantity(CqlQuantity {
                value: x.value + y.value,
                unit: x.unit.clone(),
            }))
        }
        // CQL String concatenation: 'a' + 'b' = 'ab'
        (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{x}{y}"))),
        _ => Err(err(
            "Add",
            &format!("unsupported operand types: {a:?} + {b:?}"),
        )),
    }
}

/// CQL `Subtract` (`-`).
pub fn subtract(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x - y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x - y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x - y)),
        (Value::Quantity(x), Value::Quantity(y)) if x.unit == y.unit => {
            Ok(Value::Quantity(CqlQuantity {
                value: x.value - y.value,
                unit: x.unit.clone(),
            }))
        }
        _ => Err(err(
            "Subtract",
            &format!("unsupported operand types: {a:?} - {b:?}"),
        )),
    }
}

/// CQL `Multiply` (`*`).
pub fn multiply(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x * y)),
        (Value::Long(x), Value::Long(y)) => Ok(Value::Long(x * y)),
        (Value::Decimal(x), Value::Decimal(y)) => Ok(Value::Decimal(x * y)),
        (Value::Quantity(x), Value::Quantity(y)) => Ok(Value::Quantity(CqlQuantity {
            value: x.value * y.value,
            unit: format!("{}.{}", x.unit, y.unit),
        })),
        _ => Err(err(
            "Multiply",
            &format!("unsupported operand types: {a:?} * {b:?}"),
        )),
    }
}

/// CQL `Divide` (`/`): Integer operands are promoted to Decimal; result is
/// always Decimal.  Returns `null` on divide-by-zero per CQL spec.
pub fn divide(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    let (num, den) = match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => (*x as f64, *y as f64),
        (Value::Long(x), Value::Long(y)) => (*x as f64, *y as f64),
        (Value::Decimal(x), Value::Decimal(y)) => (*x, *y),
        _ => {
            return Err(err(
                "Divide",
                &format!("unsupported operand types: {a:?} / {b:?}"),
            ))
        }
    };
    if den == 0.0 {
        return Ok(Value::Null); // CQL spec: division by zero → null
    }
    Ok(Value::Decimal(num / den))
}

/// CQL `Modulo` (`mod`): remainder after integer division.  Returns `null` on
/// modulo-by-zero.
pub fn modulo(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => {
            if *y == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Integer(x % y))
            }
        }
        (Value::Long(x), Value::Long(y)) => {
            if *y == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Long(x % y))
            }
        }
        (Value::Decimal(x), Value::Decimal(y)) => {
            if *y == 0.0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Decimal(x % y))
            }
        }
        _ => Err(err(
            "Modulo",
            &format!("unsupported operand types: {a:?} mod {b:?}"),
        )),
    }
}

/// CQL `Negate` (unary `-`).
pub fn negate(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(-x)),
        Value::Long(x) => Ok(Value::Long(-x)),
        Value::Decimal(x) => Ok(Value::Decimal(-x)),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: -q.value,
            unit: q.unit.clone(),
        })),
        _ => Err(err("Negate", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Abs`: absolute value.
pub fn abs(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(x.abs())),
        Value::Long(x) => Ok(Value::Long(x.abs())),
        Value::Decimal(x) => Ok(Value::Decimal(x.abs())),
        Value::Quantity(q) => Ok(Value::Quantity(CqlQuantity {
            value: q.value.abs(),
            unit: q.unit.clone(),
        })),
        _ => Err(err("Abs", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Ceiling`: least integer greater than or equal to the argument.
/// Integer/Long pass through unchanged.
pub fn ceiling(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.ceil() as i64)),
        _ => Err(err("Ceiling", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Floor`: greatest integer less than or equal to the argument.
pub fn floor(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.floor() as i64)),
        _ => Err(err("Floor", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Truncate`: integer part of the argument (truncate toward zero).
pub fn truncate(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(*x)),
        Value::Long(x) => Ok(Value::Long(*x)),
        Value::Decimal(x) => Ok(Value::Integer(x.trunc() as i64)),
        _ => Err(err("Truncate", &format!("unsupported type: {a:?}"))),
    }
}

/// CQL `Round`: round to `precision` decimal places (default 0).
/// Uses round-half-up semantics per the CQL specification.
///
/// `precision` must be an `Integer` value or `None` (treated as 0).
pub fn round(a: &Value, precision: Option<&Value>) -> Result<Value, EvalError> {
    null1!(a);
    let prec: i32 = match precision {
        None | Some(Value::Null) => 0,
        Some(Value::Integer(p)) => *p as i32,
        Some(other) => {
            return Err(err(
                "Round",
                &format!("precision must be Integer, got {other:?}"),
            ))
        }
    };
    let x = match a {
        Value::Decimal(x) => *x,
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        _ => return Err(err("Round", &format!("unsupported type: {a:?}"))),
    };
    let factor = 10f64.powi(prec);
    Ok(Value::Decimal((x * factor).round() / factor))
}

/// CQL `Power` (`^`): raises `base` to the power `exp`.  Returns Decimal.
pub fn power(base: &Value, exp: &Value) -> Result<Value, EvalError> {
    null2!(base, exp);
    let b = match base {
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        Value::Decimal(x) => *x,
        _ => return Err(err("Power", &format!("unsupported base type: {base:?}"))),
    };
    let e = match exp {
        Value::Integer(x) => *x as f64,
        Value::Long(x) => *x as f64,
        Value::Decimal(x) => *x,
        _ => return Err(err("Power", &format!("unsupported exp type: {exp:?}"))),
    };
    Ok(Value::Decimal(b.powf(e)))
}

/// CQL `Ln`: natural logarithm.  Returns `null` for non-positive arguments.
pub fn ln(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    let x = numeric_as_f64(a).ok_or_else(|| err("Ln", &format!("unsupported type: {a:?}")))?;
    if x <= 0.0 {
        return Ok(Value::Null);
    }
    Ok(Value::Decimal(x.ln()))
}

/// CQL `Exp`: e raised to the given power.
pub fn exp(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    let x = numeric_as_f64(a).ok_or_else(|| err("Exp", &format!("unsupported type: {a:?}")))?;
    Ok(Value::Decimal(x.exp()))
}

/// CQL `Log(argument, base)`: logarithm of `argument` in `base`.  Returns
/// `null` for non-positive argument or base.
pub fn log(a: &Value, base: &Value) -> Result<Value, EvalError> {
    null2!(a, base);
    let x = numeric_as_f64(a)
        .ok_or_else(|| err("Log", &format!("unsupported argument type: {a:?}")))?;
    let b = numeric_as_f64(base)
        .ok_or_else(|| err("Log", &format!("unsupported base type: {base:?}")))?;
    if x <= 0.0 || b <= 0.0 || b == 1.0 {
        return Ok(Value::Null);
    }
    Ok(Value::Decimal(x.log(b)))
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
// String operators (9.12)
// ---------------------------------------------------------------------------

/// Concatenate two strings.
///
/// Returns `Null` if either argument is `Null`.
pub fn concatenate(a: &Value, b: &Value) -> Result<Value, EvalError> {
    null2!(a, b);
    match (a, b) {
        (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{s1}{s2}"))),
        _ => Err(err("Concatenate", "expected String arguments")),
    }
}

/// Combine a list of strings with an optional separator.
///
/// Null items in the list are skipped.  If `separator` is `None` or `Null`
/// an empty string is used.  Returns `Null` if `list` is `Null`.
pub fn combine(list: &Value, separator: Option<&Value>) -> Result<Value, EvalError> {
    null1!(list);
    let sep: String = match separator {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(err("Combine", "separator must be a String")),
    };
    match list {
        Value::List(items) => {
            let mut parts: Vec<&str> = Vec::new();
            for item in items {
                match item {
                    Value::Null => {}
                    Value::String(s) => parts.push(s.as_str()),
                    _ => return Err(err("Combine", "list elements must be String")),
                }
            }
            Ok(Value::String(parts.join(&sep)))
        }
        _ => Err(err("Combine", "expected List argument")),
    }
}

/// Split `s` on every occurrence of `separator`, returning a `List<String>`.
///
/// Returns `Null` if either argument is `Null`.
pub fn split(s: &Value, separator: &Value) -> Result<Value, EvalError> {
    null2!(s, separator);
    match (s, separator) {
        (Value::String(str_val), Value::String(sep)) => {
            let parts: Vec<Value> = str_val
                .split(sep.as_str())
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        _ => Err(err("Split", "expected String arguments")),
    }
}

/// Split `s` on every match of the regex `pattern`, returning a `List<String>`.
///
/// Returns `Null` if either argument is `Null`.
pub fn split_on_matches(s: &Value, pattern: &Value) -> Result<Value, EvalError> {
    null2!(s, pattern);
    match (s, pattern) {
        (Value::String(str_val), Value::String(pat)) => {
            let re = Regex::new(pat)
                .map_err(|e| err("SplitOnMatches", &format!("invalid pattern: {e}")))?;
            let parts: Vec<Value> = re
                .split(str_val)
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        _ => Err(err("SplitOnMatches", "expected String arguments")),
    }
}

/// Return the number of characters in `s`.
///
/// Returns `Null` if `s` is `Null`.
pub fn length_str(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::Integer(str_val.chars().count() as i64)),
        _ => Err(err("Length", "expected String")),
    }
}

/// Convert `s` to upper case.
pub fn upper(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::String(str_val.to_uppercase())),
        _ => Err(err("Upper", "expected String")),
    }
}

/// Convert `s` to lower case.
pub fn lower(s: &Value) -> Result<Value, EvalError> {
    null1!(s);
    match s {
        Value::String(str_val) => Ok(Value::String(str_val.to_lowercase())),
        _ => Err(err("Lower", "expected String")),
    }
}

/// Return `true` if `s` starts with `prefix`.
pub fn starts_with(s: &Value, prefix: &Value) -> Result<Value, EvalError> {
    null2!(s, prefix);
    match (s, prefix) {
        (Value::String(str_val), Value::String(pfx)) => {
            Ok(Value::Boolean(str_val.starts_with(pfx.as_str())))
        }
        _ => Err(err("StartsWith", "expected String arguments")),
    }
}

/// Return `true` if `s` ends with `suffix`.
pub fn ends_with(s: &Value, suffix: &Value) -> Result<Value, EvalError> {
    null2!(s, suffix);
    match (s, suffix) {
        (Value::String(str_val), Value::String(sfx)) => {
            Ok(Value::Boolean(str_val.ends_with(sfx.as_str())))
        }
        _ => Err(err("EndsWith", "expected String arguments")),
    }
}

/// Return `true` if `s` matches the full regex `pattern` (CQL §15.16).
///
/// The pattern is matched against the entire string (anchored).
pub fn matches_regex(s: &Value, pattern: &Value) -> Result<Value, EvalError> {
    null2!(s, pattern);
    match (s, pattern) {
        (Value::String(str_val), Value::String(pat)) => {
            let anchored = format!("^(?:{pat})$");
            let re = Regex::new(&anchored)
                .map_err(|e| err("Matches", &format!("invalid pattern: {e}")))?;
            Ok(Value::Boolean(re.is_match(str_val)))
        }
        _ => Err(err("Matches", "expected String arguments")),
    }
}

/// Replace all regex matches of `pattern` in `s` with `substitution`.
pub fn replace_matches(
    s: &Value,
    pattern: &Value,
    substitution: &Value,
) -> Result<Value, EvalError> {
    null2!(s, pattern);
    null1!(substitution);
    match (s, pattern, substitution) {
        (Value::String(str_val), Value::String(pat), Value::String(sub)) => {
            let re = Regex::new(pat)
                .map_err(|e| err("ReplaceMatches", &format!("invalid pattern: {e}")))?;
            Ok(Value::String(
                re.replace_all(str_val, sub.as_str()).to_string(),
            ))
        }
        _ => Err(err("ReplaceMatches", "expected String arguments")),
    }
}

/// Return the character at 0-based `index` in `s`, or `Null` if out of range.
///
/// CQL `Indexer` (the `[]` operator) uses 0-based indexing per CQL §15.5.
pub fn indexer_str(s: &Value, index: &Value) -> Result<Value, EvalError> {
    null2!(s, index);
    match (s, index) {
        (Value::String(str_val), Value::Integer(idx)) => {
            if *idx < 0 {
                return Ok(Value::Null);
            }
            let ch = str_val.chars().nth(*idx as usize);
            Ok(ch
                .map(|c| Value::String(c.to_string()))
                .unwrap_or(Value::Null))
        }
        _ => Err(err("Indexer", "expected String and Integer arguments")),
    }
}

/// Return the 0-based index of the first occurrence of `pattern` in `s`,
/// or `-1` if `s` does not contain `pattern` (CQL §15.17).
pub fn position_of(pattern: &Value, s: &Value) -> Result<Value, EvalError> {
    null2!(pattern, s);
    match (pattern, s) {
        (Value::String(pat), Value::String(str_val)) => {
            let pos = str_val
                .find(pat.as_str())
                .map(|byte_idx| str_val[..byte_idx].chars().count() as i64)
                .unwrap_or(-1);
            Ok(Value::Integer(pos))
        }
        _ => Err(err("PositionOf", "expected String arguments")),
    }
}

/// Return the 0-based index of the last occurrence of `pattern` in `s`,
/// or `-1` if not found.
pub fn last_position_of(pattern: &Value, s: &Value) -> Result<Value, EvalError> {
    null2!(pattern, s);
    match (pattern, s) {
        (Value::String(pat), Value::String(str_val)) => {
            let pos = str_val
                .rfind(pat.as_str())
                .map(|byte_idx| str_val[..byte_idx].chars().count() as i64)
                .unwrap_or(-1);
            Ok(Value::Integer(pos))
        }
        _ => Err(err("LastPositionOf", "expected String arguments")),
    }
}

/// Return the substring of `s` starting at 0-based `start_index`.
///
/// If `length` is provided, at most `length` characters are returned.
/// Returns `Null` if `start_index` is out of range.  CQL §15.22.
pub fn substring(
    s: &Value,
    start_index: &Value,
    length: Option<&Value>,
) -> Result<Value, EvalError> {
    null2!(s, start_index);
    match (s, start_index) {
        (Value::String(str_val), Value::Integer(start)) => {
            if *start < 0 {
                return Ok(Value::Null);
            }
            let start = *start as usize;
            let chars: Vec<char> = str_val.chars().collect();
            if start >= chars.len() {
                return Ok(Value::Null);
            }
            let result = match length {
                None | Some(Value::Null) => chars[start..].iter().collect(),
                Some(Value::Integer(len)) if *len <= 0 => String::new(),
                Some(Value::Integer(len)) => {
                    let end = (start + *len as usize).min(chars.len());
                    chars[start..end].iter().collect()
                }
                _ => return Err(err("Substring", "length must be an Integer")),
            };
            Ok(Value::String(result))
        }
        _ => Err(err("Substring", "expected String and Integer arguments")),
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

// ---------------------------------------------------------------------------
// Private helper
// ---------------------------------------------------------------------------

fn numeric_as_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Integer(x) => Some(*x as f64),
        Value::Long(x) => Some(*x as f64),
        Value::Decimal(x) => Some(*x),
        _ => None,
    }
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
        Value::Code(c) => Ok(Value::Concept(super::value::CqlConcept {
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
            Ok(Value::Concept(super::value::CqlConcept {
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
pub(crate) fn eval_literal(lit: &crate::elm::Literal) -> Result<Value, EvalError> {
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
