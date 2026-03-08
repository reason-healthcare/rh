//! CQL comparison operators (CQL §5).
//!
//! Exports: [`cql_compare`], [`equal`], [`equivalent`], [`less`], [`greater`],
//! [`less_or_equal`], [`greater_or_equal`].

use std::cmp::Ordering;

use super::super::context::EvalError;
use super::super::value::{CqlDate, CqlDateTime, CqlTime, Value};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

macro_rules! null2 {
    ($a:expr, $b:expr) => {
        if matches!($a, Value::Null) || matches!($b, Value::Null) {
            return Ok(Value::Null);
        }
    };
}

// ---------------------------------------------------------------------------
// Ordering helper
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

pub(super) fn compare_dates(a: &CqlDate, b: &CqlDate) -> Option<Ordering> {
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

pub(super) fn compare_times(a: &CqlTime, b: &CqlTime) -> Option<Ordering> {
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

pub(super) fn compare_datetimes(a: &CqlDateTime, b: &CqlDateTime) -> Option<Ordering> {
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
/// Delegates to [`super::super::value::cql_equal`].
pub fn equal(a: &Value, b: &Value) -> Value {
    super::super::value::cql_equal(a, b)
}

/// CQL `Equivalent` (`~`): null-safe equivalence.
///
/// Delegates to [`super::super::value::cql_equivalent`].
pub fn equivalent(a: &Value, b: &Value) -> Value {
    Value::Boolean(super::super::value::cql_equivalent(a, b))
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
