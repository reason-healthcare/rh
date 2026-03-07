//! CQL arithmetic and comparison operators.
//!
//! Implements the arithmetic operators defined in CQL §12 and comparison
//! operators defined in CQL §5.
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
                    if mo != Ordering::Equal { Some(mo) } else { ad.partial_cmp(&bd) }
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
    let da = CqlDate { year: a.year, month: a.month, day: a.day };
    let db = CqlDate { year: b.year, month: b.month, day: b.day };
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
            Ok(Value::Quantity(CqlQuantity { value: x.value + y.value, unit: x.unit.clone() }))
        }
        _ => Err(err("Add", &format!("unsupported operand types: {a:?} + {b:?}"))),
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
            Ok(Value::Quantity(CqlQuantity { value: x.value - y.value, unit: x.unit.clone() }))
        }
        _ => Err(err("Subtract", &format!("unsupported operand types: {a:?} - {b:?}"))),
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
        _ => Err(err("Multiply", &format!("unsupported operand types: {a:?} * {b:?}"))),
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
        _ => return Err(err("Divide", &format!("unsupported operand types: {a:?} / {b:?}"))),
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
            if *y == 0 { Ok(Value::Null) } else { Ok(Value::Integer(x % y)) }
        }
        (Value::Long(x), Value::Long(y)) => {
            if *y == 0 { Ok(Value::Null) } else { Ok(Value::Long(x % y)) }
        }
        (Value::Decimal(x), Value::Decimal(y)) => {
            if *y == 0.0 { Ok(Value::Null) } else { Ok(Value::Decimal(x % y)) }
        }
        _ => Err(err("Modulo", &format!("unsupported operand types: {a:?} mod {b:?}"))),
    }
}

/// CQL `Negate` (unary `-`).
pub fn negate(a: &Value) -> Result<Value, EvalError> {
    null1!(a);
    match a {
        Value::Integer(x) => Ok(Value::Integer(-x)),
        Value::Long(x) => Ok(Value::Long(-x)),
        Value::Decimal(x) => Ok(Value::Decimal(-x)),
        Value::Quantity(q) => {
            Ok(Value::Quantity(CqlQuantity { value: -q.value, unit: q.unit.clone() }))
        }
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
        Value::Quantity(q) => {
            Ok(Value::Quantity(CqlQuantity { value: q.value.abs(), unit: q.unit.clone() }))
        }
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
        Some(other) => return Err(err("Round", &format!("precision must be Integer, got {other:?}"))),
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
    let x = numeric_as_f64(a).ok_or_else(|| err("Log", &format!("unsupported argument type: {a:?}")))?;
    let b = numeric_as_f64(base).ok_or_else(|| err("Log", &format!("unsupported base type: {base:?}")))?;
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
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

/// Returns the CqlDate one unit before `d` at its own precision, or `None` on
/// underflow (before year 1).
fn date_predecessor(d: &CqlDate) -> Option<CqlDate> {
    match (d.month, d.day) {
        // year-precision: go back one year
        (None, _) => {
            if d.year <= 1 { None } else { Some(CqlDate { year: d.year - 1, month: None, day: None }) }
        }
        // month-precision: go back one month
        (Some(m), None) => {
            if m > 1 {
                Some(CqlDate { year: d.year, month: Some(m - 1), day: None })
            } else if d.year > 1 {
                Some(CqlDate { year: d.year - 1, month: Some(12), day: None })
            } else {
                None
            }
        }
        // day-precision: go back one day
        (Some(m), Some(day)) => {
            if day > 1 {
                Some(CqlDate { year: d.year, month: Some(m), day: Some(day - 1) })
            } else if m > 1 {
                let prev_m = m - 1;
                Some(CqlDate { year: d.year, month: Some(prev_m), day: Some(days_in_month(d.year, prev_m)) })
            } else if d.year > 1 {
                Some(CqlDate { year: d.year - 1, month: Some(12), day: Some(31) })
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
            if d.year >= 9999 { None } else { Some(CqlDate { year: d.year + 1, month: None, day: None }) }
        }
        (Some(m), None) => {
            if m < 12 {
                Some(CqlDate { year: d.year, month: Some(m + 1), day: None })
            } else if d.year < 9999 {
                Some(CqlDate { year: d.year + 1, month: Some(1), day: None })
            } else {
                None
            }
        }
        (Some(m), Some(day)) => {
            let last = days_in_month(d.year, m);
            if day < last {
                Some(CqlDate { year: d.year, month: Some(m), day: Some(day + 1) })
            } else if m < 12 {
                Some(CqlDate { year: d.year, month: Some(m + 1), day: Some(1) })
            } else if d.year < 9999 {
                Some(CqlDate { year: d.year + 1, month: Some(1), day: Some(1) })
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
            return Some(CqlTime { millisecond: Some(ms - 1), ..*t });
        }
        let s = t.second.unwrap_or(0);
        if s > 0 {
            return Some(CqlTime { second: Some(s - 1), millisecond: Some(999), ..*t });
        }
        let m = t.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlTime { minute: Some(m - 1), second: Some(59), millisecond: Some(999), ..*t });
        }
        if t.hour > 0 {
            return Some(CqlTime { hour: t.hour - 1, minute: Some(59), second: Some(59), millisecond: Some(999) });
        }
        return None; // already at 00:00:00.000
    } else if let Some(s) = t.second {
        if s > 0 {
            return Some(CqlTime { second: Some(s - 1), ..*t });
        }
        let m = t.minute.unwrap_or(0);
        if m > 0 {
            return Some(CqlTime { minute: Some(m - 1), second: Some(59), ..*t });
        }
        if t.hour > 0 {
            return Some(CqlTime { hour: t.hour - 1, minute: Some(59), second: Some(59), millisecond: None });
        }
        return None;
    } else if let Some(m) = t.minute {
        if m > 0 {
            return Some(CqlTime { minute: Some(m - 1), ..*t });
        }
        if t.hour > 0 {
            return Some(CqlTime { hour: t.hour - 1, minute: Some(59), second: None, millisecond: None });
        }
        return None;
    } else {
        // hour-only precision
        if t.hour > 0 { Some(CqlTime { hour: t.hour - 1, minute: None, second: None, millisecond: None }) } else { None }
    }
}

/// Returns the CqlTime one millisecond after `t` at its own precision, or
/// `None` on overflow (beyond 23:59:59.999).
fn time_successor(t: &CqlTime) -> Option<CqlTime> {
    if let Some(ms) = t.millisecond {
        if ms < 999 {
            return Some(CqlTime { millisecond: Some(ms + 1), ..*t });
        }
        let s = t.second.unwrap_or(0);
        if s < 59 {
            return Some(CqlTime { second: Some(s + 1), millisecond: Some(0), ..*t });
        }
        let m = t.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlTime { minute: Some(m + 1), second: Some(0), millisecond: Some(0), ..*t });
        }
        if t.hour < 23 {
            return Some(CqlTime { hour: t.hour + 1, minute: Some(0), second: Some(0), millisecond: Some(0) });
        }
        return None; // already at 23:59:59.999
    } else if let Some(s) = t.second {
        if s < 59 {
            return Some(CqlTime { second: Some(s + 1), ..*t });
        }
        let m = t.minute.unwrap_or(0);
        if m < 59 {
            return Some(CqlTime { minute: Some(m + 1), second: Some(0), ..*t });
        }
        if t.hour < 23 {
            return Some(CqlTime { hour: t.hour + 1, minute: Some(59), second: Some(0), millisecond: None });
        }
        return None;
    } else if let Some(m) = t.minute {
        if m < 59 {
            return Some(CqlTime { minute: Some(m + 1), ..*t });
        }
        if t.hour < 23 {
            return Some(CqlTime { hour: t.hour + 1, minute: Some(0), second: None, millisecond: None });
        }
        return None;
    } else {
        if t.hour < 23 { Some(CqlTime { hour: t.hour + 1, minute: None, second: None, millisecond: None }) } else { None }
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

    let date_part = CqlDate { year: dt.year, month: dt.month, day: dt.day };

    if let Some(ms) = dt.millisecond {
        if ms > 0 { return Some(CqlDateTime { millisecond: Some(ms - 1), ..dt.clone() }); }
        let s = dt.second.unwrap_or(0);
        if s > 0 { return Some(CqlDateTime { second: Some(s - 1), millisecond: Some(999), ..dt.clone() }); }
        let m = dt.minute.unwrap_or(0);
        if m > 0 { return Some(CqlDateTime { minute: Some(m - 1), second: Some(59), millisecond: Some(999), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h > 0 { return Some(CqlDateTime { hour: Some(h - 1), minute: Some(59), second: Some(59), millisecond: Some(999), ..dt.clone() }); }
        let prev = date_predecessor(&date_part)?;
        return Some(CqlDateTime { year: prev.year, month: prev.month, day: prev.day,
            hour: Some(23), minute: Some(59), second: Some(59), millisecond: Some(999), offset_seconds: dt.offset_seconds });
    } else if let Some(s) = dt.second {
        if s > 0 { return Some(CqlDateTime { second: Some(s - 1), ..dt.clone() }); }
        let m = dt.minute.unwrap_or(0);
        if m > 0 { return Some(CqlDateTime { minute: Some(m - 1), second: Some(59), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h > 0 { return Some(CqlDateTime { hour: Some(h - 1), minute: Some(59), second: Some(59), ..dt.clone() }); }
        let prev = date_predecessor(&date_part)?;
        return Some(CqlDateTime { year: prev.year, month: prev.month, day: prev.day,
            hour: Some(23), minute: Some(59), second: Some(59), millisecond: None, offset_seconds: dt.offset_seconds });
    } else if let Some(m) = dt.minute {
        if m > 0 { return Some(CqlDateTime { minute: Some(m - 1), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h > 0 { return Some(CqlDateTime { hour: Some(h - 1), minute: Some(59), ..dt.clone() }); }
        let prev = date_predecessor(&date_part)?;
        return Some(CqlDateTime { year: prev.year, month: prev.month, day: prev.day,
            hour: Some(23), minute: Some(59), second: None, millisecond: None, offset_seconds: dt.offset_seconds });
    } else if let Some(h) = dt.hour {
        if h > 0 { return Some(CqlDateTime { hour: Some(h - 1), ..dt.clone() }); }
        let prev = date_predecessor(&date_part)?;
        return Some(CqlDateTime { year: prev.year, month: prev.month, day: prev.day,
            hour: Some(23), minute: None, second: None, millisecond: None, offset_seconds: dt.offset_seconds });
    } else {
        // date-only precision
        let prev = date_predecessor(&date_part)?;
        return Some(with_date!(prev));
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

    let date_part = CqlDate { year: dt.year, month: dt.month, day: dt.day };

    if let Some(ms) = dt.millisecond {
        if ms < 999 { return Some(CqlDateTime { millisecond: Some(ms + 1), ..dt.clone() }); }
        let s = dt.second.unwrap_or(0);
        if s < 59 { return Some(CqlDateTime { second: Some(s + 1), millisecond: Some(0), ..dt.clone() }); }
        let m = dt.minute.unwrap_or(0);
        if m < 59 { return Some(CqlDateTime { minute: Some(m + 1), second: Some(0), millisecond: Some(0), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h < 23 { return Some(CqlDateTime { hour: Some(h + 1), minute: Some(0), second: Some(0), millisecond: Some(0), ..dt.clone() }); }
        let next = date_successor(&date_part)?;
        return Some(CqlDateTime { year: next.year, month: next.month, day: next.day,
            hour: Some(0), minute: Some(0), second: Some(0), millisecond: Some(0), offset_seconds: dt.offset_seconds });
    } else if let Some(s) = dt.second {
        if s < 59 { return Some(CqlDateTime { second: Some(s + 1), ..dt.clone() }); }
        let m = dt.minute.unwrap_or(0);
        if m < 59 { return Some(CqlDateTime { minute: Some(m + 1), second: Some(0), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h < 23 { return Some(CqlDateTime { hour: Some(h + 1), minute: Some(0), second: Some(0), ..dt.clone() }); }
        let next = date_successor(&date_part)?;
        return Some(CqlDateTime { year: next.year, month: next.month, day: next.day,
            hour: Some(0), minute: Some(0), second: Some(0), millisecond: None, offset_seconds: dt.offset_seconds });
    } else if let Some(m) = dt.minute {
        if m < 59 { return Some(CqlDateTime { minute: Some(m + 1), ..dt.clone() }); }
        let h = dt.hour.unwrap_or(0);
        if h < 23 { return Some(CqlDateTime { hour: Some(h + 1), minute: Some(0), ..dt.clone() }); }
        let next = date_successor(&date_part)?;
        return Some(CqlDateTime { year: next.year, month: next.month, day: next.day,
            hour: Some(0), minute: Some(0), second: None, millisecond: None, offset_seconds: dt.offset_seconds });
    } else if let Some(h) = dt.hour {
        if h < 23 { return Some(CqlDateTime { hour: Some(h + 1), ..dt.clone() }); }
        let next = date_successor(&date_part)?;
        return Some(CqlDateTime { year: next.year, month: next.month, day: next.day,
            hour: Some(0), minute: None, second: None, millisecond: None, offset_seconds: dt.offset_seconds });
    } else {
        let next = date_successor(&date_part)?;
        return Some(with_date!(next));
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
            if *x <= i32::MIN as i64 { Ok(Value::Null) } else { Ok(Value::Integer(x - 1)) }
        }
        Value::Long(x) => {
            if *x <= i64::MIN as i128 { Ok(Value::Null) } else { Ok(Value::Long(x - 1)) }
        }
        Value::Decimal(x) => Ok(Value::Decimal(x - 1e-8)),
        Value::Quantity(q) => {
            Ok(Value::Quantity(CqlQuantity { value: q.value - 1e-8, unit: q.unit.clone() }))
        }
        Value::Date(d) => Ok(date_predecessor(d).map(Value::Date).unwrap_or(Value::Null)),
        Value::DateTime(dt) => Ok(datetime_predecessor(dt).map(Value::DateTime).unwrap_or(Value::Null)),
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
            if *x >= i32::MAX as i64 { Ok(Value::Null) } else { Ok(Value::Integer(x + 1)) }
        }
        Value::Long(x) => {
            if *x >= i64::MAX as i128 { Ok(Value::Null) } else { Ok(Value::Long(x + 1)) }
        }
        Value::Decimal(x) => Ok(Value::Decimal(x + 1e-8)),
        Value::Quantity(q) => {
            Ok(Value::Quantity(CqlQuantity { value: q.value + 1e-8, unit: q.unit.clone() }))
        }
        Value::Date(d) => Ok(date_successor(d).map(Value::Date).unwrap_or(Value::Null)),
        Value::DateTime(dt) => Ok(datetime_successor(dt).map(Value::DateTime).unwrap_or(Value::Null)),
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
        "Date" => Ok(Value::Date(CqlDate { year: 1, month: Some(1), day: Some(1) })),
        "DateTime" => Ok(Value::DateTime(CqlDateTime {
            year: 1, month: Some(1), day: Some(1),
            hour: Some(0), minute: Some(0), second: Some(0), millisecond: Some(0),
            offset_seconds: None,
        })),
        "Time" => Ok(Value::Time(CqlTime {
            hour: 0, minute: Some(0), second: Some(0), millisecond: Some(0),
        })),
        other => Err(EvalError::General(format!("MinValue: unsupported type '{other}'"))),
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
        "Date" => Ok(Value::Date(CqlDate { year: 9999, month: Some(12), day: Some(31) })),
        "DateTime" => Ok(Value::DateTime(CqlDateTime {
            year: 9999, month: Some(12), day: Some(31),
            hour: Some(23), minute: Some(59), second: Some(59), millisecond: Some(999),
            offset_seconds: None,
        })),
        "Time" => Ok(Value::Time(CqlTime {
            hour: 23, minute: Some(59), second: Some(59), millisecond: Some(999),
        })),
        other => Err(EvalError::General(format!("MaxValue: unsupported type '{other}'"))),
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Arithmetic --------------------------------------------------------

    #[test]
    fn add_integers() {
        assert_eq!(add(&Value::Integer(3), &Value::Integer(4)).unwrap(), Value::Integer(7));
    }

    #[test]
    fn add_null_propagates() {
        assert_eq!(add(&Value::Null, &Value::Integer(1)).unwrap(), Value::Null);
        assert_eq!(add(&Value::Integer(1), &Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn subtract_decimals() {
        assert_eq!(subtract(&Value::Decimal(5.5), &Value::Decimal(2.5)).unwrap(), Value::Decimal(3.0));
    }

    #[test]
    fn multiply_long() {
        assert_eq!(multiply(&Value::Long(100), &Value::Long(200)).unwrap(), Value::Long(20000));
    }

    #[test]
    fn divide_integers_yields_decimal() {
        assert_eq!(divide(&Value::Integer(7), &Value::Integer(2)).unwrap(), Value::Decimal(3.5));
    }

    #[test]
    fn divide_by_zero_yields_null() {
        assert_eq!(divide(&Value::Integer(5), &Value::Integer(0)).unwrap(), Value::Null);
    }

    #[test]
    fn modulo_integer() {
        assert_eq!(modulo(&Value::Integer(10), &Value::Integer(3)).unwrap(), Value::Integer(1));
    }

    #[test]
    fn modulo_by_zero_yields_null() {
        assert_eq!(modulo(&Value::Integer(5), &Value::Integer(0)).unwrap(), Value::Null);
    }

    #[test]
    fn negate_decimal() {
        assert_eq!(negate(&Value::Decimal(3.14)).unwrap(), Value::Decimal(-3.14));
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
        assert_eq!(round(&Value::Decimal(3.567), None).unwrap(), Value::Decimal(4.0));
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
        assert_eq!(power(&Value::Integer(2), &Value::Integer(10)).unwrap(), Value::Decimal(1024.0));
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
        assert_eq!(predecessor(&Value::Integer(i32::MIN as i64)).unwrap(), Value::Null);
    }

    #[test]
    fn successor_integer() {
        assert_eq!(successor(&Value::Integer(5)).unwrap(), Value::Integer(6));
    }

    #[test]
    fn successor_integer_max_yields_null() {
        assert_eq!(successor(&Value::Integer(i32::MAX as i64)).unwrap(), Value::Null);
    }

    #[test]
    fn min_max_value_integer() {
        assert_eq!(min_value("Integer").unwrap(), Value::Integer(i32::MIN as i64));
        assert_eq!(max_value("Integer").unwrap(), Value::Integer(i32::MAX as i64));
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
        assert_eq!(less(&Value::Integer(1), &Value::Integer(2)).unwrap(), Value::Boolean(true));
        assert_eq!(less(&Value::Integer(2), &Value::Integer(2)).unwrap(), Value::Boolean(false));
        assert_eq!(less(&Value::Integer(3), &Value::Integer(2)).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn greater_decimals() {
        assert_eq!(greater(&Value::Decimal(3.5), &Value::Decimal(2.0)).unwrap(), Value::Boolean(true));
        assert_eq!(greater(&Value::Decimal(1.0), &Value::Decimal(2.0)).unwrap(), Value::Boolean(false));
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
        assert_eq!(greater_or_equal(&Value::Null, &Value::Integer(1)).unwrap(), Value::Null);
    }

    #[test]
    fn equal_delegates_to_cql_equal() {
        assert_eq!(equal(&Value::Integer(5), &Value::Integer(5)), Value::Boolean(true));
        assert_eq!(equal(&Value::Null, &Value::Integer(5)), Value::Null);
    }

    #[test]
    fn equivalent_null_safe() {
        assert_eq!(equivalent(&Value::Null, &Value::Null), Value::Boolean(true));
        assert_eq!(equivalent(&Value::Null, &Value::Integer(1)), Value::Boolean(false));
    }

    #[test]
    fn compare_different_types_yields_null() {
        // Integer vs Decimal are different variant arms — not comparable, returns null
        assert_eq!(less(&Value::Integer(1), &Value::Decimal(2.0)).unwrap(), Value::Null);
    }

    // ---- Temporal comparison (S1) ------------------------------------------

    #[test]
    fn less_date_values() {
        let earlier = Value::Date(CqlDate { year: 2020, month: Some(1), day: Some(1) });
        let later   = Value::Date(CqlDate { year: 2020, month: Some(6), day: Some(15) });
        assert_eq!(less(&earlier, &later).unwrap(),   Value::Boolean(true));
        assert_eq!(less(&later, &earlier).unwrap(),   Value::Boolean(false));
        assert_eq!(less(&earlier, &earlier).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn greater_datetime_values() {
        let dt1 = Value::DateTime(CqlDateTime {
            year: 2023, month: Some(3), day: Some(10),
            hour: Some(12), minute: Some(0), second: Some(0), millisecond: Some(0),
            offset_seconds: None,
        });
        let dt2 = Value::DateTime(CqlDateTime {
            year: 2023, month: Some(3), day: Some(10),
            hour: Some(8), minute: Some(30), second: Some(0), millisecond: Some(0),
            offset_seconds: None,
        });
        assert_eq!(greater(&dt1, &dt2).unwrap(), Value::Boolean(true));
        assert_eq!(greater(&dt2, &dt1).unwrap(), Value::Boolean(false));
    }

    // ---- Temporal predecessor / successor (W2) ------------------------------

    #[test]
    fn predecessor_date_day_precision() {
        let d = Value::Date(CqlDate { year: 2023, month: Some(3), day: Some(1) });
        let expected = Value::Date(CqlDate { year: 2023, month: Some(2), day: Some(28) });
        assert_eq!(predecessor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_date_crosses_year() {
        let d = Value::Date(CqlDate { year: 2023, month: Some(1), day: Some(1) });
        let expected = Value::Date(CqlDate { year: 2022, month: Some(12), day: Some(31) });
        assert_eq!(predecessor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_date_min_yields_null() {
        let d = Value::Date(CqlDate { year: 1, month: Some(1), day: Some(1) });
        assert_eq!(predecessor(&d).unwrap(), Value::Null);
    }

    #[test]
    fn successor_date_month_end() {
        let d = Value::Date(CqlDate { year: 2023, month: Some(1), day: Some(31) });
        let expected = Value::Date(CqlDate { year: 2023, month: Some(2), day: Some(1) });
        assert_eq!(successor(&d).unwrap(), expected);
    }

    #[test]
    fn successor_date_leap_year() {
        let d = Value::Date(CqlDate { year: 2024, month: Some(2), day: Some(28) });
        let expected = Value::Date(CqlDate { year: 2024, month: Some(2), day: Some(29) });
        assert_eq!(successor(&d).unwrap(), expected);
    }

    #[test]
    fn predecessor_time_millisecond() {
        let t = Value::Time(CqlTime { hour: 10, minute: Some(30), second: Some(0), millisecond: Some(0) });
        let expected = Value::Time(CqlTime { hour: 10, minute: Some(29), second: Some(59), millisecond: Some(999) });
        assert_eq!(predecessor(&t).unwrap(), expected);
    }

    #[test]
    fn successor_time_millisecond() {
        let t = Value::Time(CqlTime { hour: 10, minute: Some(30), second: Some(0), millisecond: Some(999) });
        let expected = Value::Time(CqlTime { hour: 10, minute: Some(30), second: Some(1), millisecond: Some(0) });
        assert_eq!(successor(&t).unwrap(), expected);
    }

    #[test]
    fn predecessor_datetime_millisecond() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2023, month: Some(1), day: Some(1),
            hour: Some(0), minute: Some(0), second: Some(0), millisecond: Some(0),
            offset_seconds: None,
        });
        let expected = Value::DateTime(CqlDateTime {
            year: 2022, month: Some(12), day: Some(31),
            hour: Some(23), minute: Some(59), second: Some(59), millisecond: Some(999),
            offset_seconds: None,
        });
        assert_eq!(predecessor(&dt).unwrap(), expected);
    }

    #[test]
    fn successor_datetime_millisecond() {
        let dt = Value::DateTime(CqlDateTime {
            year: 2023, month: Some(12), day: Some(31),
            hour: Some(23), minute: Some(59), second: Some(59), millisecond: Some(999),
            offset_seconds: None,
        });
        let expected = Value::DateTime(CqlDateTime {
            year: 2024, month: Some(1), day: Some(1),
            hour: Some(0), minute: Some(0), second: Some(0), millisecond: Some(0),
            offset_seconds: None,
        });
        assert_eq!(successor(&dt).unwrap(), expected);
    }

    // ---- MinValue / MaxValue for temporal types (W1) -----------------------

    #[test]
    fn min_max_value_date() {
        assert_eq!(
            min_value("Date").unwrap(),
            Value::Date(CqlDate { year: 1, month: Some(1), day: Some(1) })
        );
        assert_eq!(
            max_value("Date").unwrap(),
            Value::Date(CqlDate { year: 9999, month: Some(12), day: Some(31) })
        );
    }

    #[test]
    fn min_max_value_time() {
        assert_eq!(
            min_value("Time").unwrap(),
            Value::Time(CqlTime { hour: 0, minute: Some(0), second: Some(0), millisecond: Some(0) })
        );
        assert_eq!(
            max_value("Time").unwrap(),
            Value::Time(CqlTime { hour: 23, minute: Some(59), second: Some(59), millisecond: Some(999) })
        );
    }

    #[test]
    fn min_max_value_datetime() {
        let min_dt = min_value("DateTime").unwrap();
        let max_dt = max_value("DateTime").unwrap();
        assert!(matches!(min_dt, Value::DateTime(ref d) if d.year == 1 && d.millisecond == Some(0)));
        assert!(matches!(max_dt, Value::DateTime(ref d) if d.year == 9999 && d.millisecond == Some(999)));
    }
}
