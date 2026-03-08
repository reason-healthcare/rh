//! CQL interval operators (CQL §19).
//!
//! All functions return `Result<Value, EvalError>` and propagate `null` unless
//! otherwise noted.
//!
//! An interval value is represented as [`Value::Interval`] with optional
//! `low`/`high` bounds (absence means ±∞) and boolean `low_closed`/
//! `high_closed` flags.

use super::context::EvalError;
use super::operators::cql_compare;
use super::value::Value;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

/// Extract the bounds of an Interval value or return an error.
fn unwrap_interval(v: &Value) -> Result<(Option<&Value>, Option<&Value>, bool, bool), EvalError> {
    match v {
        Value::Interval { low, high, low_closed, high_closed } => {
            Ok((low.as_deref(), high.as_deref(), *low_closed, *high_closed))
        }
        _ => Err(err("interval", "expected Interval value")),
    }
}

/// Compare point to interval low-bound: `None` = −∞ so point is always ≥.
fn point_ge_low(point: &Value, low: Option<&Value>, low_closed: bool) -> Option<bool> {
    match low {
        None => Some(true),
        Some(l) => {
            let ord = cql_compare(point, l)?;
            if low_closed {
                Some(ord != Ordering::Less)
            } else {
                Some(ord == Ordering::Greater)
            }
        }
    }
}

/// Compare point to interval high-bound: `None` = +∞ so point is always ≤.
fn point_le_high(point: &Value, high: Option<&Value>, high_closed: bool) -> Option<bool> {
    match high {
        None => Some(true),
        Some(h) => {
            let ord = cql_compare(point, h)?;
            if high_closed {
                Some(ord != Ordering::Greater)
            } else {
                Some(ord == Ordering::Less)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Start / End / Width / PointFrom
// ---------------------------------------------------------------------------

/// `Start` — returns the low bound of an interval (inclusive).
///
/// Returns `null` when the interval is null or the low bound is open/unbounded.
pub fn start(interval: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) {
        return Ok(Value::Null);
    }
    let (low, _, low_closed, _) = unwrap_interval(interval)?;
    match (low, low_closed) {
        (None, _) => Ok(Value::Null),
        (Some(l), true) => Ok(l.clone()),
        (Some(l), false) => {
            // For open start the "start" is predecessor(l); for simplicity
            // we delegate to predecessor if available, otherwise return null.
            super::operators::predecessor(l)
        }
    }
}

/// `End` — returns the high bound of an interval (inclusive).
pub fn end(interval: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) {
        return Ok(Value::Null);
    }
    let (_, high, _, high_closed) = unwrap_interval(interval)?;
    match (high, high_closed) {
        (None, _) => Ok(Value::Null),
        (Some(h), true) => Ok(h.clone()),
        (Some(h), false) => super::operators::predecessor(h),
    }
}

/// `Width` — returns `|end - start|` for closed-bound numeric/temporal intervals.
pub fn width(interval: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) {
        return Ok(Value::Null);
    }
    let s = start(interval)?;
    let e = end(interval)?;
    if matches!(s, Value::Null) || matches!(e, Value::Null) {
        return Ok(Value::Null);
    }
    super::operators::subtract(&e, &s)
}

/// `PointFrom` — returns the point value of a unit-width interval.
///
/// Returns `null` if the interval spans more than one point.
pub fn point_from(interval: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) {
        return Ok(Value::Null);
    }
    let (low, high, low_closed, high_closed) = unwrap_interval(interval)?;
    match (low, high) {
        (Some(l), Some(h)) if low_closed && high_closed => {
            if cql_compare(l, h) == Some(Ordering::Equal) {
                Ok(l.clone())
            } else {
                Ok(Value::Null)
            }
        }
        _ => Ok(Value::Null),
    }
}

// ---------------------------------------------------------------------------
// Membership / Containment
// ---------------------------------------------------------------------------

/// `Contains` — true if the interval contains the point.
///
/// `interval Contains point` — point is within [low, high].
pub fn contains(interval: &Value, point: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) || matches!(point, Value::Null) {
        return Ok(Value::Null);
    }
    let (low, high, low_closed, high_closed) = unwrap_interval(interval)?;
    let ge = point_ge_low(point, low, low_closed);
    let le = point_le_high(point, high, high_closed);
    match (ge, le) {
        (Some(a), Some(b)) => Ok(Value::Boolean(a && b)),
        _ => Ok(Value::Null),
    }
}

/// `In` — true if the point is in the interval (alias for `Contains` reversed).
pub fn in_interval(point: &Value, interval: &Value) -> Result<Value, EvalError> {
    contains(interval, point)
}

/// `Includes` — true if the first interval includes the second entirely.
pub fn includes(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (a_low, a_high, a_lc, a_hc) = unwrap_interval(a)?;
    let (b_low, b_high, b_lc, b_hc) = unwrap_interval(b)?;

    // a.low <= b.low
    let low_ok = match (a_low, b_low) {
        (None, _) => Some(true),                 // a is left-unbounded
        (_, None) => Some(false),                // b is left-unbounded but a is not
        (Some(al), Some(bl)) => {
            let ord = cql_compare(al, bl)
                .ok_or_else(|| EvalError::General("includes: incomparable interval bounds".to_string()))?;;
            Some(match ord {
                Ordering::Less => true,
                Ordering::Equal => !a_lc || b_lc, // a open ⊆ a closed requires b closed
                Ordering::Greater => false,
            })
        }
    };

    // a.high >= b.high
    let high_ok = match (a_high, b_high) {
        (None, _) => Some(true),
        (_, None) => Some(false),
        (Some(ah), Some(bh)) => {
            let ord = cql_compare(ah, bh)
                .ok_or_else(|| EvalError::General("includes: incomparable interval bounds".to_string()))?;;
            Some(match ord {
                Ordering::Greater => true,
                Ordering::Equal => !a_hc || b_hc,
                Ordering::Less => false,
            })
        }
    };

    match (low_ok, high_ok) {
        (Some(a), Some(b)) => Ok(Value::Boolean(a && b)),
        _ => Ok(Value::Null),
    }
}

/// `IncludedIn` — true if `a` is entirely included within `b`.
pub fn included_in(a: &Value, b: &Value) -> Result<Value, EvalError> {
    includes(b, a)
}

// ---------------------------------------------------------------------------
// Overlap / Meet
// ---------------------------------------------------------------------------

/// `Overlaps` — true if two intervals share at least one point.
pub fn overlaps(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (a_low, a_high, a_lc, a_hc) = unwrap_interval(a)?;
    let (b_low, b_high, b_lc, b_hc) = unwrap_interval(b)?;

    // a starts before b ends AND b starts before a ends
    let a_starts_before_b_ends = match (a_low, b_high) {
        (None, _) | (_, None) => Some(true),
        (Some(al), Some(bh)) => {
            let ord = cql_compare(al, bh)
                .ok_or_else(|| EvalError::General("overlaps_before: incomparable bounds".to_string()))?;;
            Some(match ord {
                Ordering::Less => true,
                Ordering::Equal => a_lc && b_hc,
                Ordering::Greater => false,
            })
        }
    };

    let b_starts_before_a_ends = match (b_low, a_high) {
        (None, _) | (_, None) => Some(true),
        (Some(bl), Some(ah)) => {
            let ord = cql_compare(bl, ah)
                .ok_or_else(|| EvalError::General("overlaps_after: incomparable bounds".to_string()))?;;
            Some(match ord {
                Ordering::Less => true,
                Ordering::Equal => b_lc && a_hc,
                Ordering::Greater => false,
            })
        }
    };

    match (a_starts_before_b_ends, b_starts_before_a_ends) {
        (Some(a), Some(b)) => Ok(Value::Boolean(a && b)),
        _ => Ok(Value::Null),
    }
}

/// `OverlapsBefore` — a starts before b and overlaps b.
pub fn overlaps_before(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (a_low, _, _, _) = unwrap_interval(a)?;
    let (b_low, _, _, _) = unwrap_interval(b)?;
    // a must start before b
    let starts_before = match (a_low, b_low) {
        (None, _) => Some(true),
        (_, None) => Some(false),
        (Some(al), Some(bl)) => cql_compare(al, bl).map(|o| o == Ordering::Less),
    };
    let ov = overlaps(a, b)?;
    match (starts_before, ov) {
        (Some(true), Value::Boolean(true)) => Ok(Value::Boolean(true)),
        (Some(_), _) => Ok(Value::Boolean(false)),
        _ => Ok(Value::Null),
    }
}

/// `OverlapsAfter` — a ends after b and overlaps b.
pub fn overlaps_after(a: &Value, b: &Value) -> Result<Value, EvalError> {
    overlaps_before(b, a)
}

/// `Meets` — a ends immediately before b starts (or vice versa).
pub fn meets(a: &Value, b: &Value) -> Result<Value, EvalError> {
    Ok(Value::Boolean(
        meets_before(a, b)? == Value::Boolean(true)
            || meets_after(a, b)? == Value::Boolean(true),
    ))
}

/// `MeetsBefore` — a ends immediately before b starts.
pub fn meets_before(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (_, a_high, _, a_hc) = unwrap_interval(a)?;
    let (b_low, _, b_lc, _) = unwrap_interval(b)?;
    match (a_high, b_low) {
        (Some(ah), Some(bl)) => {
            let ord = cql_compare(ah, bl)
                .ok_or_else(|| EvalError::General("meets: incomparable bounds".to_string()))?;;
            Ok(Value::Boolean(match ord {
                Ordering::Equal => a_hc != b_lc, // one closed, one open → adjacent
                _ => false,
            }))
        }
        _ => Ok(Value::Boolean(false)),
    }
}

/// `MeetsAfter` — b ends immediately before a starts.
pub fn meets_after(a: &Value, b: &Value) -> Result<Value, EvalError> {
    meets_before(b, a)
}

// ---------------------------------------------------------------------------
// Starts / Ends
// ---------------------------------------------------------------------------

/// `Starts` — a starts at the same point as b.
pub fn starts(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (a_low, _, a_lc, _) = unwrap_interval(a)?;
    let (b_low, _, b_lc, _) = unwrap_interval(b)?;
    let low_match = match (a_low, b_low) {
        (None, None) => Some(true),
        (Some(al), Some(bl)) => cql_compare(al, bl)
            .map(|o| o == Ordering::Equal && a_lc == b_lc),
        _ => Some(false),
    };
    Ok(low_match.map(Value::Boolean).unwrap_or(Value::Null))
}

/// `Ends` — a ends at the same point as b.
pub fn ends(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    let (_, a_high, _, a_hc) = unwrap_interval(a)?;
    let (_, b_high, _, b_hc) = unwrap_interval(b)?;
    let high_match = match (a_high, b_high) {
        (None, None) => Some(true),
        (Some(ah), Some(bh)) => cql_compare(ah, bh)
            .map(|o| o == Ordering::Equal && a_hc == b_hc),
        _ => Some(false),
    };
    Ok(high_match.map(Value::Boolean).unwrap_or(Value::Null))
}

// ---------------------------------------------------------------------------
// Set operators on intervals
// ---------------------------------------------------------------------------

/// `Union` — returns the union of two overlapping intervals, or null if they
/// do not overlap or meet.
pub fn union_interval(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    // Must overlap or meet to form a single interval.
    match overlaps(a, b)? {
        Value::Boolean(false) => {
            // Check meets
            if meets(a, b)? != Value::Boolean(true) {
                return Ok(Value::Null);
            }
        }
        Value::Null => return Ok(Value::Null),
        _ => {}
    }

    let (a_low, a_high, a_lc, a_hc) = unwrap_interval(a)?;
    let (b_low, b_high, b_lc, b_hc) = unwrap_interval(b)?;

    let (new_low, new_lc) = min_bound(a_low, a_lc, b_low, b_lc);
    let (new_high, new_hc) = max_bound(a_high, a_hc, b_high, b_hc);

    Ok(Value::Interval {
        low: new_low.map(|v| Box::new(v.clone())),
        high: new_high.map(|v| Box::new(v.clone())),
        low_closed: new_lc,
        high_closed: new_hc,
    })
}

/// `Intersect` — returns the overlapping portion of two intervals.
pub fn intersect_interval(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    match overlaps(a, b)? {
        Value::Boolean(false) | Value::Null => return Ok(Value::Null),
        _ => {}
    }

    let (a_low, a_high, a_lc, a_hc) = unwrap_interval(a)?;
    let (b_low, b_high, b_lc, b_hc) = unwrap_interval(b)?;

    let (new_low, new_lc) = max_bound(a_low, a_lc, b_low, b_lc);
    let (new_high, new_hc) = min_bound(a_high, a_hc, b_high, b_hc);

    Ok(Value::Interval {
        low: new_low.map(|v| Box::new(v.clone())),
        high: new_high.map(|v| Box::new(v.clone())),
        low_closed: new_lc,
        high_closed: new_hc,
    })
}

/// `Except` — returns the portion of `a` that does not overlap with `b`.
///
/// If `b` does not overlap `a`, returns `a` unchanged.
/// Returns `null` if `b` entirely contains `a`.
pub fn except_interval(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) {
        return Ok(Value::Null);
    }
    if matches!(b, Value::Null) {
        return Ok(a.clone());
    }
    match overlaps(a, b)? {
        Value::Boolean(false) => return Ok(a.clone()),
        Value::Null => return Ok(Value::Null),
        _ => {}
    }
    // If b includes a entirely, return null.
    match includes(b, a)? {
        Value::Boolean(true) => return Ok(Value::Null),
        _ => {}
    }
    // Return the portion of a that comes before b starts.
    let (_, b_low, _, b_lc) = unwrap_interval(b)?;
    let (a_low, a_high, a_lc, a_hc) = unwrap_interval(a)?;
    // Return left part of a (before b begins), if any.
    Ok(Value::Interval {
        low: a_low.map(|v| Box::new(v.clone())),
        high: b_low.map(|v| Box::new(v.clone())),
        low_closed: a_lc,
        high_closed: !b_lc,
    })
}

// ---------------------------------------------------------------------------
// Proper Containment / Inclusion
// ---------------------------------------------------------------------------

/// `ProperContains` (interval, point) — true if the interval contains the point
/// strictly (point is not equal to a closed boundary).
pub fn proper_contains(interval: &Value, point: &Value) -> Result<Value, EvalError> {
    if matches!(interval, Value::Null) || matches!(point, Value::Null) {
        return Ok(Value::Null);
    }
    match contains(interval, point)? {
        Value::Boolean(false) => return Ok(Value::Boolean(false)),
        Value::Null => return Ok(Value::Null),
        _ => {}
    }
    let (low, high, low_closed, high_closed) = unwrap_interval(interval)?;
    if low_closed {
        if let Some(l) = low {
            if cql_compare(point, l) == Some(Ordering::Equal) {
                return Ok(Value::Boolean(false));
            }
        }
    }
    if high_closed {
        if let Some(h) = high {
            if cql_compare(point, h) == Some(Ordering::Equal) {
                return Ok(Value::Boolean(false));
            }
        }
    }
    Ok(Value::Boolean(true))
}

/// `ProperIn` — point is properly in the interval (reverse of `proper_contains`).
pub fn proper_in(point: &Value, interval: &Value) -> Result<Value, EvalError> {
    proper_contains(interval, point)
}

/// `ProperIncludes` — `a` properly includes `b` (a includes b, and a ≠ b).
///
/// Works for interval ⊃ interval. For interval ⊃ point, `proper_contains` should
/// be used instead.
pub fn proper_includes(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::Null);
    }
    match includes(a, b)? {
        Value::Boolean(false) => return Ok(Value::Boolean(false)),
        Value::Null => return Ok(Value::Null),
        _ => {}
    }
    // a includes b; it's "proper" only if b does not also include a (i.e., they're not equal).
    match included_in(a, b)? {
        Value::Boolean(true) => Ok(Value::Boolean(false)),
        Value::Boolean(false) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Null),
    }
}

/// `ProperIncludedIn` — `a` is properly included in `b`.
pub fn proper_included_in(a: &Value, b: &Value) -> Result<Value, EvalError> {
    proper_includes(b, a)
}

// ---------------------------------------------------------------------------
// Expand / Collapse
// ---------------------------------------------------------------------------

/// `Expand` — expands a list of intervals (or a single interval) into a list
/// of unit intervals spaced by `per`.
///
/// For integer intervals with no explicit per, each integer becomes a closed
/// unit interval. Non-integer or unbounded intervals are returned unchanged.
pub fn expand(source: &Value, per: Option<&Value>) -> Result<Value, EvalError> {
    if matches!(source, Value::Null) {
        return Ok(Value::Null);
    }
    let intervals: Vec<&Value> = match source {
        Value::List(items) => items.iter().collect(),
        v @ Value::Interval { .. } => vec![v],
        _ => return Err(err("Expand", "expected Interval or List<Interval>")),
    };
    let mut result = Vec::new();
    for interval in intervals {
        if matches!(interval, Value::Null) {
            continue;
        }
        match expand_single(interval, per)? {
            Value::List(items) => result.extend(items),
            other => result.push(other),
        }
    }
    Ok(Value::List(result))
}

fn expand_single(interval: &Value, per: Option<&Value>) -> Result<Value, EvalError> {
    let (low, high, low_closed, high_closed) = unwrap_interval(interval)?;
    match (low, high) {
        (Some(Value::Integer(lo)), Some(Value::Integer(hi))) => {
            let step: i64 = match per {
                Some(Value::Integer(n)) if *n > 0 => *n,
                None => 1,
                _ => 1,
            };
            let start = if low_closed { *lo } else { lo + 1 };
            let end = if high_closed { *hi } else { hi - 1 };
            if start > end {
                return Ok(Value::List(vec![]));
            }
            let mut items = Vec::new();
            let mut pos = start;
            while pos <= end {
                let next = (pos + step - 1).min(end);
                items.push(Value::Interval {
                    low: Some(Box::new(Value::Integer(pos))),
                    high: Some(Box::new(Value::Integer(next))),
                    low_closed: true,
                    high_closed: true,
                });
                pos += step;
            }
            Ok(Value::List(items))
        }
        // Non-integer or unbounded: return the interval as-is (unsupported precision)
        _ => Ok(Value::List(vec![interval.clone()])),
    }
}

/// `Collapse` — merges overlapping/meeting intervals in a list into the
/// minimum number of non-overlapping intervals.
pub fn collapse(list: &Value, _per: Option<&Value>) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = match list {
        Value::List(v) => v,
        _ => return Err(EvalError::General("collapse: expected List".to_string())),
    };
    if items.is_empty() {
        return Ok(Value::List(vec![]));
    }

    // Sort intervals by their low bound (null low = -∞ sorts first).
    let mut intervals: Vec<Value> = items.iter().filter(|v| !matches!(v, Value::Null)).cloned().collect();
    // Stable sort by low bound.
    intervals.sort_by(|a, b| {
        let (al, _, _, _) = unwrap_interval(a).ok().unwrap_or((None, None, true, true));
        let (bl, _, _, _) = unwrap_interval(b).ok().unwrap_or((None, None, true, true));
        match (al, bl) {
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Some(al), Some(bl)) => cql_compare(al, bl).unwrap_or(Ordering::Equal),
        }
    });

    let mut merged: Vec<Value> = Vec::new();
    let mut current = intervals[0].clone();

    for next in &intervals[1..] {
        match union_interval(&current, next)? {
            Value::Null => {
                merged.push(current);
                current = next.clone();
            }
            union => {
                current = union;
            }
        }
    }
    merged.push(current);
    Ok(Value::List(merged))
}

// ---------------------------------------------------------------------------
// Helpers for bound comparison
// ---------------------------------------------------------------------------

/// Returns the "minimum" (earlier) of two bounds (for union low / intersect high).
fn min_bound<'a>(
    a: Option<&'a Value>, a_closed: bool,
    b: Option<&'a Value>, b_closed: bool,
) -> (Option<&'a Value>, bool) {
    match (a, b) {
        (None, _) => (None, a_closed),
        (_, None) => (None, b_closed),
        (Some(av), Some(bv)) => match cql_compare(av, bv) {
            Some(Ordering::Less) => (Some(av), a_closed),
            Some(Ordering::Greater) => (Some(bv), b_closed),
            _ => (Some(av), a_closed || b_closed),
        },
    }
}

/// Returns the "maximum" (later) of two bounds (for union high / intersect low).
fn max_bound<'a>(
    a: Option<&'a Value>, a_closed: bool,
    b: Option<&'a Value>, b_closed: bool,
) -> (Option<&'a Value>, bool) {
    match (a, b) {
        (None, _) => (None, a_closed),
        (_, None) => (None, b_closed),
        (Some(av), Some(bv)) => match cql_compare(av, bv) {
            Some(Ordering::Greater) => (Some(av), a_closed),
            Some(Ordering::Less) => (Some(bv), b_closed),
            _ => (Some(av), a_closed || b_closed),
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn int_interval(lo: i64, hi: i64) -> Value {
        Value::Interval {
            low: Some(Box::new(Value::Integer(lo))),
            high: Some(Box::new(Value::Integer(hi))),
            low_closed: true,
            high_closed: true,
        }
    }

    #[test]
    fn contains_basic() {
        let iv = int_interval(1, 10);
        assert_eq!(contains(&iv, &Value::Integer(5)).unwrap(), Value::Boolean(true));
        assert_eq!(contains(&iv, &Value::Integer(0)).unwrap(), Value::Boolean(false));
        assert_eq!(contains(&iv, &Value::Integer(10)).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn in_interval_basic() {
        let iv = int_interval(1, 10);
        assert_eq!(in_interval(&Value::Integer(5), &iv).unwrap(), Value::Boolean(true));
        assert_eq!(in_interval(&Value::Integer(11), &iv).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn overlaps_basic() {
        let a = int_interval(1, 5);
        let b = int_interval(3, 8);
        let c = int_interval(6, 10);
        assert_eq!(overlaps(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(overlaps(&a, &c).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn union_overlapping() {
        let a = int_interval(1, 5);
        let b = int_interval(3, 8);
        let result = union_interval(&a, &b).unwrap();
        assert_eq!(result, int_interval(1, 8));
    }

    #[test]
    fn intersect_basic() {
        let a = int_interval(1, 5);
        let b = int_interval(3, 8);
        let result = intersect_interval(&a, &b).unwrap();
        assert_eq!(result, int_interval(3, 5));
    }

    #[test]
    fn collapse_basic() {
        let intervals = Value::List(vec![
            int_interval(1, 3),
            int_interval(2, 5),
            int_interval(7, 10),
        ]);
        let result = collapse(&intervals, None).unwrap();
        if let Value::List(v) = result {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], int_interval(1, 5));
            assert_eq!(v[1], int_interval(7, 10));
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn start_end_basic() {
        let iv = int_interval(1, 10);
        assert_eq!(start(&iv).unwrap(), Value::Integer(1));
        assert_eq!(end(&iv).unwrap(), Value::Integer(10));
    }

    #[test]
    fn width_basic() {
        let iv = int_interval(1, 10);
        assert_eq!(width(&iv).unwrap(), Value::Integer(9));
    }

    #[test]
    fn null_propagation() {
        assert_eq!(contains(&Value::Null, &Value::Integer(1)).unwrap(), Value::Null);
        assert_eq!(overlaps(&Value::Null, &int_interval(1, 5)).unwrap(), Value::Null);
    }

    #[test]
    fn starts_basic() {
        let a = int_interval(1, 3);
        let b = int_interval(1, 8);
        let c = int_interval(2, 8);
        assert_eq!(starts(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(starts(&c, &b).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn ends_basic() {
        let a = int_interval(5, 10);
        let b = int_interval(1, 10);
        let c = int_interval(1, 9);
        assert_eq!(ends(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(ends(&c, &b).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn meets_basic() {
        // [1,5) meets [5,10] via meets_before because a.high=5(open) and b.low=5(closed)
        let a = Value::Interval {
            low: Some(Box::new(Value::Integer(1))),
            high: Some(Box::new(Value::Integer(5))),
            low_closed: true,
            high_closed: false,
        };
        let b = Value::Interval {
            low: Some(Box::new(Value::Integer(5))),
            high: Some(Box::new(Value::Integer(10))),
            low_closed: true,
            high_closed: true,
        };
        assert_eq!(meets_before(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(meets_after(&b, &a).unwrap(), Value::Boolean(true));
        // Intervals that overlap should not meet
        assert_eq!(meets_before(&int_interval(1, 5), &int_interval(3, 8)).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn overlaps_before_basic() {
        let a = int_interval(1, 5);
        let b = int_interval(3, 8);
        assert_eq!(overlaps_before(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(overlaps_before(&b, &a).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn includes_and_included_in() {
        let a = int_interval(1, 10);
        let b = int_interval(3, 7);
        assert_eq!(includes(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(included_in(&b, &a).unwrap(), Value::Boolean(true));
        assert_eq!(includes(&b, &a).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn proper_contains_basic() {
        let iv = int_interval(1, 10);
        // Interior point: proper
        assert_eq!(proper_contains(&iv, &Value::Integer(5)).unwrap(), Value::Boolean(true));
        // Boundary point: NOT proper (closed boundary)
        assert_eq!(proper_contains(&iv, &Value::Integer(1)).unwrap(), Value::Boolean(false));
        assert_eq!(proper_contains(&iv, &Value::Integer(10)).unwrap(), Value::Boolean(false));
        // Outside: false
        assert_eq!(proper_contains(&iv, &Value::Integer(11)).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn proper_includes_basic() {
        let a = int_interval(1, 10);
        let b = int_interval(3, 7);
        let c = int_interval(1, 10); // equal to a
        assert_eq!(proper_includes(&a, &b).unwrap(), Value::Boolean(true));
        assert_eq!(proper_includes(&a, &c).unwrap(), Value::Boolean(false)); // equal, not proper
        assert_eq!(proper_included_in(&b, &a).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn expand_integer_unit() {
        let iv = int_interval(1, 5);
        let result = expand(&iv, None).unwrap();
        if let Value::List(items) = result {
            assert_eq!(items.len(), 5);
            assert_eq!(items[0], int_interval(1, 1));
            assert_eq!(items[4], int_interval(5, 5));
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn expand_with_per() {
        let iv = int_interval(1, 6);
        let per = Value::Integer(2);
        let result = expand(&iv, Some(&per)).unwrap();
        if let Value::List(items) = result {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], int_interval(1, 2));
            assert_eq!(items[1], int_interval(3, 4));
            assert_eq!(items[2], int_interval(5, 6));
        } else {
            panic!("expected List");
        }
    }
}
