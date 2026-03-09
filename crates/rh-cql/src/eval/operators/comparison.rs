//! CQL comparison operators (CQL §5).
//!
//! Exports: [`cql_compare`], [`equal`], [`equivalent`], [`less`], [`greater`],
//! [`less_or_equal`], [`greater_or_equal`].

use std::cmp::Ordering;

use super::super::context::EvalError;
use super::super::value::{CqlDate, CqlDateTime, CqlTime, Value};
use super::utils::null2;

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
        // Integer/Decimal cross-type: implicit promotion to Decimal.
        (Value::Integer(x), Value::Decimal(y)) => (*x as f64).partial_cmp(y),
        (Value::Decimal(x), Value::Integer(y)) => x.partial_cmp(&(*y as f64)),
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
/// For lists, performs element-by-element comparison with null propagation:
/// extra null elements in a longer list cause the result to be Null.
pub fn equivalent(a: &Value, b: &Value) -> Value {
    if let (Value::List(av), Value::List(bv)) = (a, b) {
        let shorter = av.len().min(bv.len());
        // Compare shared elements
        for i in 0..shorter {
            if !super::super::value::cql_equivalent(&av[i], &bv[i]) {
                return Value::Boolean(false);
            }
        }
        // Check extra elements in longer list
        if av.len() != bv.len() {
            let extra: &[Value] = if av.len() > bv.len() {
                &av[shorter..]
            } else {
                &bv[shorter..]
            };
            // Non-null extra elements → not equivalent
            if extra.iter().any(|v| !matches!(v, Value::Null)) {
                return Value::Boolean(false);
            }
            // All extra elements are null → result is null (null propagation)
            return Value::Null;
        }
        return Value::Boolean(true);
    }
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::value::{CqlDate, CqlDateTime, Value};

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
    fn compare_integer_and_decimal() {
        // Integer and Decimal are comparable via implicit promotion (CQL spec).
        assert_eq!(
            less(&Value::Integer(1), &Value::Decimal(2.0)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            less(&Value::Integer(3), &Value::Decimal(2.0)).unwrap(),
            Value::Boolean(false)
        );
    }

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
}
