//! CQL three-valued logic (TVL).
//!
//! CQL Boolean operations propagate `null` according to the rules in the CQL
//! specification §5.1–§5.4.  These functions accept [`Value`] operands and
//! return a [`Value`] result; non-Boolean, non-Null inputs are treated as
//! `null` per CQL semantics (operations on incorrect types return `null`
//! rather than panicking).
//!
//! # Truth tables
//!
//! ## AND
//!
//! | a     | b     | a AND b |
//! |-------|-------|---------|
//! | true  | true  | true    |
//! | true  | false | false   |
//! | false | false | false   |
//! | true  | null  | null    |
//! | false | null  | **false** (short-circuit) |
//! | null  | null  | null    |
//!
//! ## OR
//!
//! | a     | b     | a OR b  |
//! |-------|-------|---------|
//! | true  | true  | true    |
//! | true  | false | true    |
//! | false | false | false   |
//! | true  | null  | **true** (short-circuit) |
//! | false | null  | null    |
//! | null  | null  | null    |
//!
//! ## NOT
//!
//! | a     | NOT a |
//! |-------|-------|
//! | true  | false |
//! | false | true  |
//! | null  | null  |

use crate::eval::value::Value;

// ---------------------------------------------------------------------------
// Helper — extract Option<bool> from a Value
// ---------------------------------------------------------------------------

/// Coerce a `Value` to an `Option<bool>` for three-valued logic use.
/// `Null` → `None`; `Boolean(b)` → `Some(b)`; anything else → `None`
/// (non-Boolean types are treated as null per CQL semantics).
#[inline]
fn to_tvl(v: &Value) -> Option<bool> {
    match v {
        Value::Boolean(b) => Some(*b),
        _ => None,
    }
}

#[inline]
fn from_tvl(opt: Option<bool>) -> Value {
    match opt {
        Some(b) => Value::Boolean(b),
        None => Value::Null,
    }
}

// ---------------------------------------------------------------------------
// Public TVL operations
// ---------------------------------------------------------------------------

/// CQL `AND` with null propagation.
///
/// ```
/// use rh_cql::eval::value::Value;
/// use rh_cql::eval::tvl::tvl_and;
///
/// assert_eq!(tvl_and(&Value::Boolean(true),  &Value::Boolean(true)),  Value::Boolean(true));
/// assert_eq!(tvl_and(&Value::Boolean(true),  &Value::Boolean(false)), Value::Boolean(false));
/// assert_eq!(tvl_and(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(false));
/// // null propagation
/// assert_eq!(tvl_and(&Value::Boolean(true),  &Value::Null),           Value::Null);
/// // short-circuit: false AND null → false
/// assert_eq!(tvl_and(&Value::Boolean(false), &Value::Null),           Value::Boolean(false));
/// assert_eq!(tvl_and(&Value::Null,           &Value::Null),           Value::Null);
/// ```
pub fn tvl_and(a: &Value, b: &Value) -> Value {
    match (to_tvl(a), to_tvl(b)) {
        // Definite false short-circuits regardless of the other operand.
        (Some(false), _) | (_, Some(false)) => Value::Boolean(false),
        (Some(true), Some(true)) => Value::Boolean(true),
        // At least one is null (neither is false).
        _ => Value::Null,
    }
}

/// CQL `OR` with null propagation.
///
/// ```
/// use rh_cql::eval::value::Value;
/// use rh_cql::eval::tvl::tvl_or;
///
/// assert_eq!(tvl_or(&Value::Boolean(true),  &Value::Boolean(false)), Value::Boolean(true));
/// assert_eq!(tvl_or(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(false));
/// // short-circuit: true OR null → true
/// assert_eq!(tvl_or(&Value::Boolean(true),  &Value::Null),           Value::Boolean(true));
/// // null propagation
/// assert_eq!(tvl_or(&Value::Boolean(false), &Value::Null),           Value::Null);
/// assert_eq!(tvl_or(&Value::Null,           &Value::Null),           Value::Null);
/// ```
pub fn tvl_or(a: &Value, b: &Value) -> Value {
    match (to_tvl(a), to_tvl(b)) {
        // Definite true short-circuits regardless of the other operand.
        (Some(true), _) | (_, Some(true)) => Value::Boolean(true),
        (Some(false), Some(false)) => Value::Boolean(false),
        // At least one is null (neither is true).
        _ => Value::Null,
    }
}

/// CQL `NOT` with null propagation.
///
/// ```
/// use rh_cql::eval::value::Value;
/// use rh_cql::eval::tvl::tvl_not;
///
/// assert_eq!(tvl_not(&Value::Boolean(true)),  Value::Boolean(false));
/// assert_eq!(tvl_not(&Value::Boolean(false)), Value::Boolean(true));
/// assert_eq!(tvl_not(&Value::Null),           Value::Null);
/// ```
pub fn tvl_not(a: &Value) -> Value {
    from_tvl(to_tvl(a).map(|b| !b))
}

/// CQL `IMPLIES` — logically equivalent to `NOT a OR b`.
///
/// Truth table:
///
/// | a     | b     | a IMPLIES b |
/// |-------|-------|-------------|
/// | true  | true  | true        |
/// | true  | false | false       |
/// | true  | null  | null        |
/// | false | true  | true        |
/// | false | false | true        |
/// | false | null  | true        |
/// | null  | true  | true        |
/// | null  | false | null        |
/// | null  | null  | null        |
///
/// ```
/// use rh_cql::eval::value::Value;
/// use rh_cql::eval::tvl::tvl_implies;
///
/// assert_eq!(tvl_implies(&Value::Boolean(true),  &Value::Boolean(true)),  Value::Boolean(true));
/// assert_eq!(tvl_implies(&Value::Boolean(true),  &Value::Boolean(false)), Value::Boolean(false));
/// assert_eq!(tvl_implies(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(true));
/// assert_eq!(tvl_implies(&Value::Boolean(false), &Value::Null),           Value::Boolean(true));
/// assert_eq!(tvl_implies(&Value::Boolean(true),  &Value::Null),           Value::Null);
/// assert_eq!(tvl_implies(&Value::Null,            &Value::Boolean(true)), Value::Boolean(true));
/// assert_eq!(tvl_implies(&Value::Null,            &Value::Null),          Value::Null);
/// ```
pub fn tvl_implies(a: &Value, b: &Value) -> Value {
    // a IMPLIES b ≡ NOT a OR b
    let not_a = tvl_not(a);
    tvl_or(&not_a, b)
}

/// CQL `XOR` — exclusive or with null propagation.
///
/// Truth table:
///
/// | a     | b     | a XOR b |
/// |-------|-------|---------|
/// | true  | true  | false   |
/// | true  | false | true    |
/// | false | false | false   |
/// | true  | null  | null    |
/// | false | null  | null    |
/// | null  | null  | null    |
///
/// ```
/// use rh_cql::eval::value::Value;
/// use rh_cql::eval::tvl::tvl_xor;
///
/// assert_eq!(tvl_xor(&Value::Boolean(true),  &Value::Boolean(true)),  Value::Boolean(false));
/// assert_eq!(tvl_xor(&Value::Boolean(true),  &Value::Boolean(false)), Value::Boolean(true));
/// assert_eq!(tvl_xor(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(false));
/// // null propagation
/// assert_eq!(tvl_xor(&Value::Boolean(true),  &Value::Null),           Value::Null);
/// assert_eq!(tvl_xor(&Value::Boolean(false), &Value::Null),           Value::Null);
/// assert_eq!(tvl_xor(&Value::Null,           &Value::Null),           Value::Null);
/// ```
pub fn tvl_xor(a: &Value, b: &Value) -> Value {
    match (to_tvl(a), to_tvl(b)) {
        (Some(x), Some(y)) => Value::Boolean(x ^ y),
        _ => Value::Null,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- AND ---

    #[test]
    fn and_true_true() {
        assert_eq!(tvl_and(&Value::Boolean(true), &Value::Boolean(true)), Value::Boolean(true));
    }

    #[test]
    fn and_true_false() {
        assert_eq!(tvl_and(&Value::Boolean(true), &Value::Boolean(false)), Value::Boolean(false));
    }

    #[test]
    fn and_false_false() {
        assert_eq!(tvl_and(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(false));
    }

    #[test]
    fn and_true_null_is_null() {
        // CQL spec: true and null → null
        assert_eq!(tvl_and(&Value::Boolean(true), &Value::Null), Value::Null);
    }

    #[test]
    fn and_false_null_is_false() {
        // CQL spec: false and null → false (short-circuit)
        assert_eq!(tvl_and(&Value::Boolean(false), &Value::Null), Value::Boolean(false));
    }

    #[test]
    fn and_null_true_is_null() {
        assert_eq!(tvl_and(&Value::Null, &Value::Boolean(true)), Value::Null);
    }

    #[test]
    fn and_null_false_is_false() {
        assert_eq!(tvl_and(&Value::Null, &Value::Boolean(false)), Value::Boolean(false));
    }

    #[test]
    fn and_null_null_is_null() {
        assert_eq!(tvl_and(&Value::Null, &Value::Null), Value::Null);
    }

    // --- OR ---

    #[test]
    fn or_true_false_is_true() {
        assert_eq!(tvl_or(&Value::Boolean(true), &Value::Boolean(false)), Value::Boolean(true));
    }

    #[test]
    fn or_false_false_is_false() {
        assert_eq!(tvl_or(&Value::Boolean(false), &Value::Boolean(false)), Value::Boolean(false));
    }

    #[test]
    fn or_true_null_is_true() {
        // CQL spec: true or null → true (short-circuit)
        assert_eq!(tvl_or(&Value::Boolean(true), &Value::Null), Value::Boolean(true));
    }

    #[test]
    fn or_false_null_is_null() {
        // CQL spec: false or null → null
        assert_eq!(tvl_or(&Value::Boolean(false), &Value::Null), Value::Null);
    }

    #[test]
    fn or_null_null_is_null() {
        assert_eq!(tvl_or(&Value::Null, &Value::Null), Value::Null);
    }

    // --- NOT ---

    #[test]
    fn not_true_is_false() {
        assert_eq!(tvl_not(&Value::Boolean(true)), Value::Boolean(false));
    }

    #[test]
    fn not_false_is_true() {
        assert_eq!(tvl_not(&Value::Boolean(false)), Value::Boolean(true));
    }

    #[test]
    fn not_null_is_null() {
        // CQL spec: not null → null
        assert_eq!(tvl_not(&Value::Null), Value::Null);
    }

    // --- IMPLIES ---

    #[test]
    fn implies_false_null_is_true() {
        // false implies null → true (false implies anything evaluates the
        // consequent only if antecedent is true; since it is false, result is true)
        assert_eq!(tvl_implies(&Value::Boolean(false), &Value::Null), Value::Boolean(true));
    }

    #[test]
    fn implies_true_null_is_null() {
        assert_eq!(tvl_implies(&Value::Boolean(true), &Value::Null), Value::Null);
    }

    #[test]
    fn implies_null_true_is_true() {
        assert_eq!(tvl_implies(&Value::Null, &Value::Boolean(true)), Value::Boolean(true));
    }

    #[test]
    fn implies_null_false_is_null() {
        assert_eq!(tvl_implies(&Value::Null, &Value::Boolean(false)), Value::Null);
    }

    // --- XOR ---

    #[test]
    fn xor_true_true_is_false() {
        assert_eq!(tvl_xor(&Value::Boolean(true), &Value::Boolean(true)), Value::Boolean(false));
    }

    #[test]
    fn xor_true_false_is_true() {
        assert_eq!(tvl_xor(&Value::Boolean(true), &Value::Boolean(false)), Value::Boolean(true));
    }

    #[test]
    fn xor_true_null_is_null() {
        assert_eq!(tvl_xor(&Value::Boolean(true), &Value::Null), Value::Null);
    }

    #[test]
    fn xor_false_null_is_null() {
        assert_eq!(tvl_xor(&Value::Boolean(false), &Value::Null), Value::Null);
    }

    #[test]
    fn xor_null_null_is_null() {
        assert_eq!(tvl_xor(&Value::Null, &Value::Null), Value::Null);
    }
}
