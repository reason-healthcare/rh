//! CQL list operators (CQL §20).
//!
//! Implements: Exists, Count, Sum, Min, Max, Avg, First, Last, IndexOf,
//! Contains (list), In (list), Flatten, Distinct, Sort (list), Filter,
//! ForEach, SingletonFrom, Union, Intersect, Except.

use super::context::EvalError;
use super::operators::{cql_compare, equal as cql_equal};
use super::value::Value;
use std::cmp::Ordering;

fn err(op: &str, msg: &str) -> EvalError {
    EvalError::General(format!("{op}: {msg}"))
}

fn require_list<'a>(op: &str, v: &'a Value) -> Result<&'a [Value], EvalError> {
    match v {
        Value::List(items) => Ok(items),
        _ => Err(err(op, "expected List")),
    }
}

// ---------------------------------------------------------------------------
// Existence
// ---------------------------------------------------------------------------

/// `Exists` — true if the list is non-empty (ignoring nulls per CQL semantics).
pub fn exists(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Boolean(false));
    }
    let items = require_list("Exists", list)?;
    Ok(Value::Boolean(
        items.iter().any(|v| !matches!(v, Value::Null)),
    ))
}

/// `Count` — number of non-null elements in a list.
pub fn count(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Integer(0));
    }
    let items = require_list("Count", list)?;
    let n = items.iter().filter(|v| !matches!(v, Value::Null)).count();
    Ok(Value::Integer(n as i64))
}

// ---------------------------------------------------------------------------
// Aggregates
// ---------------------------------------------------------------------------

/// `Sum` — arithmetic sum of numeric list elements (nulls excluded).
pub fn sum(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Sum", list)?;
    let nums: Vec<&Value> = items.iter().filter(|v| !matches!(v, Value::Null)).collect();
    if nums.is_empty() {
        return Ok(Value::Null);
    }
    let mut acc = nums[0].clone();
    for v in &nums[1..] {
        acc = super::operators::add(&acc, v)?;
    }
    Ok(acc)
}

/// `Min` — minimum value in a list (nulls excluded).
pub fn min(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Min", list)?;
    let mut result: Option<&Value> = None;
    for v in items {
        if matches!(v, Value::Null) {
            continue;
        }
        result = Some(match result {
            None => v,
            Some(cur) => match cql_compare(v, cur) {
                Some(Ordering::Less) => v,
                _ => cur,
            },
        });
    }
    Ok(result.cloned().unwrap_or(Value::Null))
}

/// `Max` — maximum value in a list (nulls excluded).
pub fn max(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Max", list)?;
    let mut result: Option<&Value> = None;
    for v in items {
        if matches!(v, Value::Null) {
            continue;
        }
        result = Some(match result {
            None => v,
            Some(cur) => match cql_compare(v, cur) {
                Some(Ordering::Greater) => v,
                _ => cur,
            },
        });
    }
    Ok(result.cloned().unwrap_or(Value::Null))
}

/// `Avg` — arithmetic mean of numeric list elements (nulls excluded).
pub fn avg(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Avg", list)?;
    let nums: Vec<&Value> = items.iter().filter(|v| !matches!(v, Value::Null)).collect();
    if nums.is_empty() {
        return Ok(Value::Null);
    }
    let n = nums.len() as f64;
    let total = {
        let mut acc = nums[0].clone();
        for v in &nums[1..] {
            acc = super::operators::add(&acc, v)?;
        }
        acc
    };
    // Divide by count — promote to Decimal.
    match total {
        Value::Integer(x) => Ok(Value::Decimal(x as f64 / n)),
        Value::Long(x) => Ok(Value::Decimal(x as f64 / n)),
        Value::Decimal(x) => Ok(Value::Decimal(x / n)),
        other => Err(err(
            "Avg",
            &format!("unsupported element type for average: {other:?}"),
        )),
    }
}

// ---------------------------------------------------------------------------
// Access
// ---------------------------------------------------------------------------

/// `First` — returns the first non-null element, or null if empty.
pub fn first(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("First", list)?;
    Ok(items.first().cloned().unwrap_or(Value::Null))
}

/// `Last` — returns the last non-null element, or null if empty.
pub fn last(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Last", list)?;
    Ok(items.last().cloned().unwrap_or(Value::Null))
}

pub fn tail(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Tail", list)?;
    if items.is_empty() {
        return Ok(Value::List(vec![]));
    }
    Ok(Value::List(items[1..].to_vec()))
}

pub fn skip(list: &Value, count: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    if matches!(count, Value::Null) {
        return Ok(Value::List(vec![]));
    }
    let items = require_list("Skip", list)?;
    let n = match count {
        Value::Integer(v) => *v,
        _ => return Err(err("Skip", "expected Integer count")),
    };
    if n <= 0 {
        return Ok(Value::List(items.to_vec()));
    }
    let start = (n as usize).min(items.len());
    Ok(Value::List(items[start..].to_vec()))
}

pub fn take(list: &Value, count: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    if matches!(count, Value::Null) {
        return Ok(Value::List(vec![]));
    }
    let items = require_list("Take", list)?;
    let n = match count {
        Value::Integer(v) => *v,
        _ => return Err(err("Take", "expected Integer count")),
    };
    if n <= 0 {
        return Ok(Value::List(vec![]));
    }
    let end = (n as usize).min(items.len());
    Ok(Value::List(items[..end].to_vec()))
}

pub fn slice(
    list: &Value,
    start_index: &Value,
    end_index: Option<&Value>,
) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    if matches!(start_index, Value::Null) {
        return Ok(Value::Null);
    }

    let items = require_list("Slice", list)?;
    let start = match start_index {
        Value::Integer(v) => *v,
        _ => return Err(err("Slice", "expected Integer start index")),
    };
    if start < 0 {
        return Ok(Value::Null);
    }
    let start = start as usize;
    if start >= items.len() {
        return Ok(Value::List(vec![]));
    }

    let end = match end_index {
        None | Some(Value::Null) => items.len(),
        Some(Value::Integer(v)) => {
            if *v < 0 {
                return Ok(Value::Null);
            }
            (*v as usize).min(items.len())
        }
        Some(_) => return Err(err("Slice", "expected Integer end index")),
    };

    if end <= start {
        return Ok(Value::List(vec![]));
    }
    Ok(Value::List(items[start..end].to_vec()))
}

/// `IndexOf` — returns the 0-based index of the first occurrence of `element`
/// in `list`, or -1 if not found.
pub fn index_of(list: &Value, element: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    if matches!(element, Value::Null) {
        // Special case: find null in the list (null element can be in a list)
        let items = require_list("IndexOf", list)?;
        for (i, v) in items.iter().enumerate() {
            if matches!(v, Value::Null) {
                return Ok(Value::Integer(i as i64)); // 0-based
            }
        }
        return Ok(Value::Integer(-1));
    }
    let items = require_list("IndexOf", list)?;
    for (i, v) in items.iter().enumerate() {
        if cql_equal(v, element) == Value::Boolean(true) {
            return Ok(Value::Integer(i as i64)); // 0-based
        }
    }
    Ok(Value::Integer(-1))
}

/// `Contains` (list) — true if `list` contains `element`.
///
/// If the list is null (treated as empty), returns false.
/// If the element is null, propagates null (per CQL spec §20.3).
/// If comparison with a list element returns null (e.g. precision mismatch),
/// and no definitive true match is found, the result is null.
pub fn list_contains(list: &Value, element: &Value) -> Result<Value, EvalError> {
    // Null list = empty list, doesn't contain anything
    if matches!(list, Value::Null) {
        return Ok(Value::Boolean(false));
    }
    // Null element: standard null propagation (per CQL spec §20.3)
    if matches!(element, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Contains", list)?;
    let mut had_null = false;
    for v in items {
        // Null items in the list are never equal to any element per CQL null semantics.
        if matches!(v, Value::Null) {
            continue;
        }
        match cql_equal(v, element) {
            Value::Boolean(true) => return Ok(Value::Boolean(true)),
            // Null result from comparison (e.g. precision mismatch) propagates.
            Value::Null => had_null = true,
            _ => {}
        }
    }
    if had_null {
        Ok(Value::Null)
    } else {
        Ok(Value::Boolean(false))
    }
}

/// `In` (list) — alias for `Contains` with reversed arguments.
pub fn in_list(element: &Value, list: &Value) -> Result<Value, EvalError> {
    list_contains(list, element)
}

// ---------------------------------------------------------------------------
// Transformation
// ---------------------------------------------------------------------------

/// `Flatten` — flattens one level of list nesting.
pub fn flatten(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Flatten", list)?;
    let mut result = Vec::new();
    for item in items {
        match item {
            Value::List(inner) => result.extend(inner.iter().cloned()),
            other => result.push(other.clone()),
        }
    }
    Ok(Value::List(result))
}

/// `Distinct` — returns a list with duplicate elements removed (preserving order).
pub fn distinct(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Distinct", list)?;
    let mut seen: Vec<&Value> = Vec::new();
    let mut result = Vec::new();
    for item in items {
        if !seen
            .iter()
            .any(|s| cql_equal(s, item) == Value::Boolean(true))
        {
            seen.push(item);
            result.push(item.clone());
        }
    }
    Ok(Value::List(result))
}

/// `Sort` — returns a sorted copy of the list. Ascending unless `descending`
/// is `true`.
pub fn sort_list(list: &Value, descending: bool) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Sort", list)?;
    let mut sorted: Vec<Value> = items.to_vec();
    sorted.sort_by(|a, b| {
        let ord = cql_compare(a, b).unwrap_or(Ordering::Equal);
        if descending {
            ord.reverse()
        } else {
            ord
        }
    });
    Ok(Value::List(sorted))
}

/// `Filter` — returns elements for which `predicate(element)` returns `true`.
///
/// `predicate` receives each element and should return `Value::Boolean(true)` to keep it.
pub fn filter<F>(list: &Value, predicate: F) -> Result<Value, EvalError>
where
    F: Fn(&Value) -> Result<Value, EvalError>,
{
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Filter", list)?;
    let mut result = Vec::new();
    for item in items {
        if predicate(item)? == Value::Boolean(true) {
            result.push(item.clone());
        }
    }
    Ok(Value::List(result))
}

/// `ForEach` — applies `transform` to each element and returns a new list.
pub fn for_each<F>(list: &Value, transform: F) -> Result<Value, EvalError>
where
    F: Fn(&Value) -> Result<Value, EvalError>,
{
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("ForEach", list)?;
    let result: Result<Vec<Value>, EvalError> = items.iter().map(transform).collect();
    Ok(Value::List(result?))
}

/// `SingletonFrom` — returns the single element of a list, or null if empty.
/// Returns an error if the list has more than one element.
pub fn singleton_from(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("SingletonFrom", list)?;
    let non_null: Vec<&Value> = items.iter().filter(|v| !matches!(v, Value::Null)).collect();
    match non_null.len() {
        0 => Ok(Value::Null),
        1 => Ok(non_null[0].clone()),
        _ => Err(err("SingletonFrom", "list contains more than one element")),
    }
}

// ---------------------------------------------------------------------------
// Set operators (list version)
// ---------------------------------------------------------------------------

/// `Union` (list) — concatenates two lists and removes duplicates.
pub fn union_list(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) && matches!(b, Value::Null) {
        return Ok(Value::List(vec![]));
    }
    let mut combined = match a {
        Value::Null => vec![],
        _ => require_list("Union", a)?.to_vec(),
    };
    if let Value::List(bv) = b {
        combined.extend(bv.iter().cloned());
    }
    distinct(&Value::List(combined))
}

/// `Intersect` (list) — elements present in both lists.
pub fn intersect_list(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) || matches!(b, Value::Null) {
        return Ok(Value::List(vec![]));
    }
    let av = require_list("Intersect", a)?;
    let bv = require_list("Intersect", b)?;
    let result: Vec<Value> = av
        .iter()
        .filter(|item| {
            bv.iter()
                .any(|bitem| cql_equal(item, bitem) == Value::Boolean(true))
        })
        .cloned()
        .collect();
    distinct(&Value::List(result))
}

/// `Except` (list) — elements in `a` that are not in `b`.
pub fn except_list(a: &Value, b: &Value) -> Result<Value, EvalError> {
    if matches!(a, Value::Null) {
        return Ok(Value::List(vec![]));
    }
    if matches!(b, Value::Null) {
        return Ok(a.clone());
    }
    let av = require_list("Except", a)?;
    let bv = require_list("Except", b)?;
    let result: Vec<Value> = av
        .iter()
        .filter(|item| {
            !bv.iter()
                .any(|bitem| cql_equal(item, bitem) == Value::Boolean(true))
        })
        .cloned()
        .collect();
    Ok(Value::List(result))
}

// ---------------------------------------------------------------------------
// Inclusion
// ---------------------------------------------------------------------------

/// `Includes` (list) — true if list `a` contains every element of list `b`.
///
/// Null container (a) = empty list → returns false (unless b is also null/empty).
/// Null item list (b) = empty list → returns true (any list includes empty).
/// Null elements in b are matched via identity (null in a = match).
pub fn list_includes(a: &Value, b: &Value) -> Result<Value, EvalError> {
    // Null container = empty list → only includes empty list
    if matches!(a, Value::Null) {
        // empty includes empty (true), empty doesn't include non-empty (false)
        return match b {
            Value::Null => Ok(Value::Boolean(true)),
            Value::List(bv) => Ok(Value::Boolean(bv.is_empty())),
            _ => Ok(Value::Boolean(false)),
        };
    }
    // Null item list = empty list → any list includes empty
    if matches!(b, Value::Null) {
        return Ok(Value::Boolean(true));
    }
    let av = require_list("Includes", a)?;
    let bv = require_list("Includes", b)?;
    Ok(Value::Boolean(bv.iter().all(|bitem| {
        if matches!(bitem, Value::Null) {
            // Check if null is literally in av
            av.iter().any(|aitem| matches!(aitem, Value::Null))
        } else {
            av.iter()
                .any(|aitem| cql_equal(aitem, bitem) == Value::Boolean(true))
        }
    })))
}

/// `IncludedIn` (list) — true if every element of `a` is in `b`.
pub fn list_included_in(a: &Value, b: &Value) -> Result<Value, EvalError> {
    list_includes(b, a)
}

// ---------------------------------------------------------------------------
// Statistical aggregates
// ---------------------------------------------------------------------------

/// `Median` — median of numeric list elements (nulls excluded).
pub fn median(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Median", list)?;
    let mut nums: Vec<f64> = items
        .iter()
        .filter_map(|v| match v {
            Value::Integer(x) => Some(*x as f64),
            Value::Long(x) => Some(*x as f64),
            Value::Decimal(x) => Some(*x),
            _ => None,
        })
        .collect();
    if nums.is_empty() {
        return Ok(Value::Null);
    }
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let n = nums.len();
    let m = if n.is_multiple_of(2) {
        (nums[n / 2 - 1] + nums[n / 2]) / 2.0
    } else {
        nums[n / 2]
    };
    Ok(Value::Decimal(m))
}

/// `Mode` — most frequently occurring element in a list (nulls excluded).
///
/// Returns `null` if the list is empty. If multiple elements tie for the most
/// frequent, returns the first such element (per CQL spec the result is a list,
/// but we return the single most-common for simplicity).
pub fn mode(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Mode", list)?;
    let non_null: Vec<&Value> = items.iter().filter(|v| !matches!(v, Value::Null)).collect();
    if non_null.is_empty() {
        return Ok(Value::Null);
    }
    let mut freq: Vec<(&Value, usize)> = Vec::new();
    for v in &non_null {
        match freq
            .iter_mut()
            .find(|(ev, _)| cql_equal(ev, v) == Value::Boolean(true))
        {
            Some(entry) => entry.1 += 1,
            None => freq.push((v, 1)),
        }
    }
    let max_freq = freq.iter().map(|(_, c)| *c).max().unwrap_or(0);
    Ok(freq
        .into_iter()
        .filter(|(_, c)| *c == max_freq)
        .map(|(v, _)| v.clone())
        .next()
        .unwrap_or(Value::Null))
}

fn compute_variance(list: &Value, population: bool) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Null);
    }
    let items = require_list("Variance", list)?;
    let nums: Vec<f64> = items
        .iter()
        .filter_map(|v| match v {
            Value::Integer(x) => Some(*x as f64),
            Value::Long(x) => Some(*x as f64),
            Value::Decimal(x) => Some(*x),
            _ => None,
        })
        .collect();
    let n = nums.len();
    if n == 0 {
        return Ok(Value::Null);
    }
    let denom = if population { n as f64 } else { (n - 1) as f64 };
    if denom == 0.0 {
        return Ok(Value::Null);
    }
    let mean = nums.iter().sum::<f64>() / n as f64;
    let var = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / denom;
    Ok(Value::Decimal(var))
}

/// `Variance` — sample variance (n−1 denominator, nulls excluded).
pub fn variance(list: &Value) -> Result<Value, EvalError> {
    compute_variance(list, false)
}

/// `PopulationVariance` — population variance (n denominator, nulls excluded).
pub fn population_variance(list: &Value) -> Result<Value, EvalError> {
    compute_variance(list, true)
}

/// `StdDev` — sample standard deviation.
pub fn std_dev(list: &Value) -> Result<Value, EvalError> {
    match variance(list)? {
        Value::Decimal(v) => Ok(Value::Decimal(v.sqrt())),
        other => Ok(other),
    }
}

/// `PopulationStdDev` — population standard deviation.
pub fn population_std_dev(list: &Value) -> Result<Value, EvalError> {
    match population_variance(list)? {
        Value::Decimal(v) => Ok(Value::Decimal(v.sqrt())),
        other => Ok(other),
    }
}

/// `AllTrue` — true if all non-null elements are `true`.
///
/// An empty list (or all-null list) returns `true` (vacuous truth).
pub fn all_true(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Boolean(true));
    }
    let items = require_list("AllTrue", list)?;
    for v in items {
        match v {
            Value::Null => continue,
            Value::Boolean(false) => return Ok(Value::Boolean(false)),
            Value::Boolean(true) => continue,
            _ => return Err(err("AllTrue", "expected Boolean elements")),
        }
    }
    Ok(Value::Boolean(true))
}

/// `AnyTrue` — true if any non-null element is `true`.
pub fn any_true(list: &Value) -> Result<Value, EvalError> {
    if matches!(list, Value::Null) {
        return Ok(Value::Boolean(false));
    }
    let items = require_list("AnyTrue", list)?;
    for v in items {
        match v {
            Value::Null => continue,
            Value::Boolean(true) => return Ok(Value::Boolean(true)),
            Value::Boolean(false) => continue,
            _ => return Err(err("AnyTrue", "expected Boolean elements")),
        }
    }
    Ok(Value::Boolean(false))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn int_list(nums: Vec<i64>) -> Value {
        Value::List(nums.into_iter().map(Value::Integer).collect())
    }

    #[test]
    fn exists_non_empty() {
        assert_eq!(exists(&int_list(vec![1, 2])).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn exists_empty() {
        assert_eq!(exists(&Value::List(vec![])).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn count_basic() {
        assert_eq!(count(&int_list(vec![1, 2, 3])).unwrap(), Value::Integer(3));
    }

    #[test]
    fn count_with_nulls() {
        let list = Value::List(vec![Value::Integer(1), Value::Null, Value::Integer(3)]);
        assert_eq!(count(&list).unwrap(), Value::Integer(2));
    }

    #[test]
    fn sum_basic() {
        assert_eq!(sum(&int_list(vec![1, 2, 3])).unwrap(), Value::Integer(6));
    }

    #[test]
    fn min_basic() {
        assert_eq!(min(&int_list(vec![3, 1, 2])).unwrap(), Value::Integer(1));
    }

    #[test]
    fn max_basic() {
        assert_eq!(max(&int_list(vec![3, 1, 2])).unwrap(), Value::Integer(3));
    }

    #[test]
    fn avg_basic() {
        assert_eq!(avg(&int_list(vec![1, 2, 3])).unwrap(), Value::Decimal(2.0));
    }

    #[test]
    fn first_last_basic() {
        let list = int_list(vec![1, 2, 3]);
        assert_eq!(first(&list).unwrap(), Value::Integer(1));
        assert_eq!(last(&list).unwrap(), Value::Integer(3));
    }

    #[test]
    fn tail_basic() {
        let list = int_list(vec![1, 2, 3, 4]);
        assert_eq!(tail(&list).unwrap(), int_list(vec![2, 3, 4]));
        assert_eq!(tail(&Value::List(vec![])).unwrap(), Value::List(vec![]));
    }

    #[test]
    fn skip_basic() {
        let list = int_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            skip(&list, &Value::Integer(2)).unwrap(),
            int_list(vec![3, 4, 5])
        );
        assert_eq!(skip(&list, &Value::Integer(0)).unwrap(), list);
    }

    #[test]
    fn take_basic() {
        let list = int_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            take(&list, &Value::Integer(3)).unwrap(),
            int_list(vec![1, 2, 3])
        );
        assert_eq!(take(&list, &Value::Null).unwrap(), Value::List(vec![]));
    }

    #[test]
    fn slice_basic() {
        let list = int_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            slice(&list, &Value::Integer(1), Some(&Value::Integer(4))).unwrap(),
            int_list(vec![2, 3, 4])
        );
        assert_eq!(
            slice(&list, &Value::Integer(2), None).unwrap(),
            int_list(vec![3, 4, 5])
        );
    }

    #[test]
    fn index_of_found() {
        assert_eq!(
            index_of(&int_list(vec![10, 20, 30]), &Value::Integer(20)).unwrap(),
            Value::Integer(1) // 0-based: 20 is at index 1
        );
    }

    #[test]
    fn index_of_not_found() {
        assert_eq!(
            index_of(&int_list(vec![10, 20]), &Value::Integer(99)).unwrap(),
            Value::Integer(-1)
        );
    }

    #[test]
    fn list_contains_basic() {
        assert_eq!(
            list_contains(&int_list(vec![1, 2, 3]), &Value::Integer(2)).unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            list_contains(&int_list(vec![1, 2, 3]), &Value::Integer(9)).unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn flatten_nested() {
        let nested = Value::List(vec![
            Value::List(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Integer(3),
        ]);
        let flat = flatten(&nested).unwrap();
        assert_eq!(flat, int_list(vec![1, 2, 3]));
    }

    #[test]
    fn distinct_removes_duplicates() {
        let list = int_list(vec![1, 2, 1, 3, 2]);
        let result = distinct(&list).unwrap();
        assert_eq!(result, int_list(vec![1, 2, 3]));
    }

    #[test]
    fn sort_ascending() {
        let list = int_list(vec![3, 1, 2]);
        assert_eq!(sort_list(&list, false).unwrap(), int_list(vec![1, 2, 3]));
    }

    #[test]
    fn sort_descending() {
        let list = int_list(vec![3, 1, 2]);
        assert_eq!(sort_list(&list, true).unwrap(), int_list(vec![3, 2, 1]));
    }

    #[test]
    fn singleton_from_ok() {
        assert_eq!(
            singleton_from(&int_list(vec![42])).unwrap(),
            Value::Integer(42)
        );
    }

    #[test]
    fn singleton_from_empty() {
        assert_eq!(singleton_from(&Value::List(vec![])).unwrap(), Value::Null);
    }

    #[test]
    fn singleton_from_multiple_errors() {
        assert!(singleton_from(&int_list(vec![1, 2])).is_err());
    }

    #[test]
    fn union_list_basic() {
        let a = int_list(vec![1, 2, 3]);
        let b = int_list(vec![2, 3, 4]);
        let result = union_list(&a, &b).unwrap();
        assert_eq!(result, int_list(vec![1, 2, 3, 4]));
    }

    #[test]
    fn intersect_list_basic() {
        let a = int_list(vec![1, 2, 3]);
        let b = int_list(vec![2, 3, 4]);
        let result = intersect_list(&a, &b).unwrap();
        assert_eq!(result, int_list(vec![2, 3]));
    }

    #[test]
    fn except_list_basic() {
        let a = int_list(vec![1, 2, 3]);
        let b = int_list(vec![2, 3]);
        let result = except_list(&a, &b).unwrap();
        assert_eq!(result, int_list(vec![1]));
    }

    #[test]
    fn null_propagation() {
        assert_eq!(exists(&Value::Null).unwrap(), Value::Boolean(false));
        assert_eq!(count(&Value::Null).unwrap(), Value::Integer(0));
        assert_eq!(sum(&Value::Null).unwrap(), Value::Null);
    }

    #[test]
    fn median_odd() {
        assert_eq!(
            median(&int_list(vec![3, 1, 2])).unwrap(),
            Value::Decimal(2.0)
        );
    }

    #[test]
    fn median_even() {
        assert_eq!(
            median(&int_list(vec![1, 2, 3, 4])).unwrap(),
            Value::Decimal(2.5)
        );
    }

    #[test]
    fn mode_basic() {
        let list = int_list(vec![1, 2, 2, 3]);
        assert_eq!(mode(&list).unwrap(), Value::Integer(2));
    }

    #[test]
    fn variance_basic() {
        // {2,4,4,4,5,5,7,9}: population variance = 4.0
        let list = Value::List(
            vec![2, 4, 4, 4, 5, 5, 7, 9]
                .into_iter()
                .map(Value::Integer)
                .collect(),
        );
        assert_eq!(population_variance(&list).unwrap(), Value::Decimal(4.0));
    }

    #[test]
    fn std_dev_basic() {
        // population std dev of above = 2.0
        let list = Value::List(
            vec![2, 4, 4, 4, 5, 5, 7, 9]
                .into_iter()
                .map(Value::Integer)
                .collect(),
        );
        assert_eq!(population_std_dev(&list).unwrap(), Value::Decimal(2.0));
    }

    #[test]
    fn all_true_all_true() {
        let list = Value::List(vec![Value::Boolean(true), Value::Boolean(true)]);
        assert_eq!(all_true(&list).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn all_true_one_false() {
        let list = Value::List(vec![Value::Boolean(true), Value::Boolean(false)]);
        assert_eq!(all_true(&list).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn any_true_basic() {
        let list = Value::List(vec![Value::Boolean(false), Value::Boolean(true)]);
        assert_eq!(any_true(&list).unwrap(), Value::Boolean(true));
        let all_false = Value::List(vec![Value::Boolean(false), Value::Boolean(false)]);
        assert_eq!(any_true(&all_false).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn list_includes_basic() {
        let a = int_list(vec![1, 2, 3, 4]);
        let b = int_list(vec![2, 3]);
        assert_eq!(list_includes(&a, &b).unwrap(), Value::Boolean(true));
        let c = int_list(vec![2, 5]);
        assert_eq!(list_includes(&a, &c).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn list_included_in_basic() {
        let a = int_list(vec![2, 3]);
        let b = int_list(vec![1, 2, 3, 4]);
        assert_eq!(list_included_in(&a, &b).unwrap(), Value::Boolean(true));
    }
}
