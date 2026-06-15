//! Comparison and equality operations for FHIRPath values

use std::cmp::Ordering;

use crate::ast::*;
use crate::error::*;
use crate::evaluator::operations::temporal::{
    compare_parts, equals_parts, parse_temporal, TemporalParts,
};
use crate::evaluator::types::FhirPathValue;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

/// Comparison operations handler
pub struct ComparisonEvaluator;

/// Extract a temporal-parts view from any temporal-valued FhirPathValue.
/// Also handles String values that parse as dates/datetimes (e.g. FHIR period
/// start/end fields accessed without typed metadata).
fn try_temporal(v: &FhirPathValue) -> Option<TemporalParts> {
    match v {
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) | FhirPathValue::Time(s) => {
            parse_temporal(s)
        }
        FhirPathValue::String(s) => parse_temporal(s),
        _ => None,
    }
}

/// Are two temporal values structurally comparable?
/// Date vs DateTime is compatible (Date is a coarser-precision DateTime).
/// Date vs Time and Time vs DateTime are NOT — they carry disjoint components.
fn temporal_types_compatible(a: &FhirPathValue, b: &FhirPathValue) -> bool {
    use FhirPathValue::*;
    matches!(
        (a, b),
        (Date(_), Date(_))
            | (DateTime(_), DateTime(_))
            | (Time(_), Time(_))
            | (Date(_), DateTime(_))
            | (DateTime(_), Date(_))
    )
}

fn is_temporal(v: &FhirPathValue) -> bool {
    matches!(
        v,
        FhirPathValue::Date(_) | FhirPathValue::DateTime(_) | FhirPathValue::Time(_)
    )
}

impl ComparisonEvaluator {
    /// Evaluate equality operation
    pub fn evaluate_equality(
        left: &FhirPathValue,
        operator: &EqualityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        // Temporal equality: precision-aware. Cross-type (Date vs Time,
        // DateTime vs Time) yields false — those values describe disjoint
        // kinds of clock data and are never equal. Date vs DateTime is
        // treated as a precision comparison, not a type mismatch.
        if is_temporal(left) && is_temporal(right) {
            if !temporal_types_compatible(left, right) {
                let result = match operator {
                    EqualityOperator::Equal | EqualityOperator::Equivalent => false,
                    EqualityOperator::NotEqual | EqualityOperator::NotEquivalent => true,
                };
                return Ok(FhirPathValue::Boolean(result));
            }
            if let (Some(a), Some(b)) = (try_temporal(left), try_temporal(right)) {
                match equals_parts(&a, &b) {
                    None => {
                        // Precision mismatch. `=`/`!=` follow the spec and
                        // return empty; `~`/`!~` follow the suite, treating
                        // mismatched precision as not-equivalent.
                        return Ok(match operator {
                            EqualityOperator::Equal | EqualityOperator::NotEqual => {
                                FhirPathValue::Empty
                            }
                            EqualityOperator::Equivalent => FhirPathValue::Boolean(false),
                            EqualityOperator::NotEquivalent => FhirPathValue::Boolean(true),
                        });
                    }
                    Some(eq) => {
                        let result = match operator {
                            EqualityOperator::Equal | EqualityOperator::Equivalent => eq,
                            EqualityOperator::NotEqual | EqualityOperator::NotEquivalent => !eq,
                        };
                        return Ok(FhirPathValue::Boolean(result));
                    }
                }
            }
        }

        // Per spec §6.5.1: `=` and `!=` propagate empty (unknown) when either
        // operand is the empty collection; `~` and `!~` do not.
        fn is_empty_collection(v: &FhirPathValue) -> bool {
            match v {
                FhirPathValue::Empty => true,
                FhirPathValue::Collection(c) | FhirPathValue::UnorderedCollection(c) => {
                    c.is_empty()
                }
                _ => false,
            }
        }
        if (is_empty_collection(left) || is_empty_collection(right))
            && matches!(
                operator,
                EqualityOperator::Equal | EqualityOperator::NotEqual
            )
        {
            return Ok(FhirPathValue::Empty);
        }

        // Singleton unwrapping per FHIRPath §6.1: a 1-item collection is
        // treated as its contained scalar for equality purposes.
        let mut left_unwrapped: Option<FhirPathValue> = None;
        let mut right_unwrapped: Option<FhirPathValue> = None;
        let left = unwrap_singleton(left, &mut left_unwrapped);
        let right = unwrap_singleton(right, &mut right_unwrapped);

        // Re-check empty after unwrapping (e.g. Collection([Empty]) → Empty)
        if (is_empty_collection(left) || is_empty_collection(right))
            && matches!(
                operator,
                EqualityOperator::Equal | EqualityOperator::NotEqual
            )
        {
            return Ok(FhirPathValue::Empty);
        }

        // Quantities: `=`/`!=` use exact comparison (returns Empty for incompatible
        // units); `~`/`!~` use precision-aware equivalence (returns false/true for
        // incompatible units).
        if let (
            FhirPathValue::Quantity {
                value: v1,
                unit: u1,
            },
            FhirPathValue::Quantity {
                value: v2,
                unit: u2,
            },
        ) = (left, right)
        {
            use crate::evaluator::operations::units::UnitConverter;
            let converter = UnitConverter::new();
            return match operator {
                EqualityOperator::Equal | EqualityOperator::NotEqual => {
                    match converter.compare_quantities(*v1, u1, *v2, u2) {
                        Ok(0) => Ok(FhirPathValue::Boolean(matches!(
                            operator,
                            EqualityOperator::Equal
                        ))),
                        Ok(_) => Ok(FhirPathValue::Boolean(matches!(
                            operator,
                            EqualityOperator::NotEqual
                        ))),
                        Err(_) => Ok(FhirPathValue::Empty),
                    }
                }
                EqualityOperator::Equivalent | EqualityOperator::NotEquivalent => {
                    match (
                        converter.to_base_unit(v1.to_f64().unwrap_or(0.0), u1),
                        converter.to_base_unit(v2.to_f64().unwrap_or(0.0), u2),
                    ) {
                        (Ok((base_a, _)), Ok((base_b, _))) => {
                            let equiv = quantity_equivalent(base_a, base_b);
                            Ok(FhirPathValue::Boolean(
                                if matches!(operator, EqualityOperator::Equivalent) {
                                    equiv
                                } else {
                                    !equiv
                                },
                            ))
                        }
                        _ => Ok(FhirPathValue::Boolean(matches!(
                            operator,
                            EqualityOperator::NotEquivalent
                        ))),
                    }
                }
            };
        }

        let result = match operator {
            EqualityOperator::Equal => FhirPathValue::equals_static(left, right),
            EqualityOperator::NotEqual => !FhirPathValue::equals_static(left, right),
            EqualityOperator::Equivalent => Self::values_equivalent(left, right),
            EqualityOperator::NotEquivalent => !Self::values_equivalent(left, right),
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate inequality operations (<, <=, >, >=)
    pub fn evaluate_inequality(
        left: &FhirPathValue,
        operator: &InequalityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        // Precision-aware temporal comparison: a precision mismatch yields
        // empty per the spec, even though > / < etc. normally return Boolean.
        if let (Some(a), Some(b)) = (try_temporal(left), try_temporal(right)) {
            let Some(ord) = compare_parts(&a, &b) else {
                return Ok(FhirPathValue::Empty);
            };
            let result = match operator {
                InequalityOperator::LessThan => ord == Ordering::Less,
                InequalityOperator::LessThanOrEqual => ord != Ordering::Greater,
                InequalityOperator::GreaterThan => ord == Ordering::Greater,
                InequalityOperator::GreaterThanOrEqual => ord != Ordering::Less,
            };
            return Ok(FhirPathValue::Boolean(result));
        }

        let comparison = left.compare_values(right)?;
        let result = match operator {
            InequalityOperator::LessThan => comparison < 0,
            InequalityOperator::LessThanOrEqual => comparison <= 0,
            InequalityOperator::GreaterThan => comparison > 0,
            InequalityOperator::GreaterThanOrEqual => comparison >= 0,
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate membership operations (in, contains)
    pub fn evaluate_membership(
        left: &FhirPathValue,
        operator: &MembershipOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match operator {
            MembershipOperator::In => left.is_member_of(right)?,
            MembershipOperator::Contains => right.is_member_of(left)?,
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate AND operation with three-valued logic (§6.5):
    /// false AND anything → false; true AND empty → empty; empty AND empty → empty.
    pub fn evaluate_and(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let lk = bool_three_valued(left);
        let rk = bool_three_valued(right);
        let result = match (lk, rk) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        };
        Ok(match result {
            Some(b) => FhirPathValue::Boolean(b),
            None => FhirPathValue::Empty,
        })
    }

    /// Evaluate implies per three-valued logic (§6.5.7):
    /// | left  | right | result |
    /// |-------|-------|--------|
    /// | true  | true  | true   |
    /// | true  | false | false  |
    /// | true  | {}    | {}     |
    /// | false | *     | true   |
    /// | {}    | true  | true   |
    /// | {}    | other | {}     |
    pub fn evaluate_implies(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let lk = bool_three_valued(left);
        let rk = bool_three_valued(right);
        let result = match (lk, rk) {
            (Some(false), _) => Some(true),
            (Some(true), Some(r)) => Some(r),
            (Some(true), None) => None,
            (None, Some(true)) => Some(true),
            (None, _) => None,
        };
        Ok(match result {
            Some(b) => FhirPathValue::Boolean(b),
            None => FhirPathValue::Empty,
        })
    }

    /// Evaluate or/xor operations per FHIRPath three-valued logic (§6.5):
    /// * `or`: false or empty → empty; true or anything → true.
    /// * `xor`: empty operand → empty result (always — XOR has no short-circuit
    ///   that resolves with an unknown side).
    pub fn evaluate_or(
        left: &FhirPathValue,
        operator: &OrOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_known = bool_three_valued(left);
        let right_known = bool_three_valued(right);
        let result = match operator {
            OrOperator::Or => match (left_known, right_known) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            },
            OrOperator::Xor => match (left_known, right_known) {
                (Some(a), Some(b)) => Some(a ^ b),
                _ => None,
            },
        };
        Ok(match result {
            Some(b) => FhirPathValue::Boolean(b),
            None => FhirPathValue::Empty,
        })
    }

    /// Check if two values are equivalent (more lenient than equal). Per
    /// FHIRPath §6.1.5: strings ignore case and collapse runs of whitespace;
    /// decimals match at the precision of the less-precise input; collections
    /// match as multisets (order-insensitive); otherwise it's strict equality.
    pub fn values_equivalent(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        use FhirPathValue::*;
        match (left, right) {
            (String(a), String(b)) => string_equivalent(a, b),
            (Number(a), Number(b)) => decimal_equivalent(a, b),
            (Integer(a), Number(b)) | (Long(a), Number(b)) => {
                decimal_equivalent(&Decimal::from(*a), b)
            }
            (Number(a), Integer(b)) | (Number(a), Long(b)) => {
                decimal_equivalent(a, &Decimal::from(*b))
            }
            (
                Quantity {
                    value: va,
                    unit: ua,
                },
                Quantity {
                    value: vb,
                    unit: ub,
                },
            ) => {
                // Use the unit converter so cross-unit equivalence works, then
                // apply the FHIRPath §6.5 precision rule: compare at the
                // significant-digit precision of the less-precise value.
                use crate::evaluator::operations::units::UnitConverter;
                let converter = UnitConverter::new();
                let va_f64 = va.to_f64().unwrap_or(0.0);
                let vb_f64 = vb.to_f64().unwrap_or(0.0);
                let Ok((base_a, _)) = converter.to_base_unit(va_f64, ua) else {
                    return false;
                };
                let Ok((base_b, _)) = converter.to_base_unit(vb_f64, ub) else {
                    return false;
                };
                quantity_equivalent(base_a, base_b)
            }
            (Collection(a), Collection(b))
            | (Collection(a), UnorderedCollection(b))
            | (UnorderedCollection(a), Collection(b))
            | (UnorderedCollection(a), UnorderedCollection(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                let mut matched = vec![false; b.len()];
                for x in a {
                    let mut found = false;
                    for (j, y) in b.iter().enumerate() {
                        if !matched[j] && Self::values_equivalent(x, y) {
                            matched[j] = true;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return false;
                    }
                }
                true
            }
            _ => FhirPathValue::equals_static(left, right),
        }
    }
}

/// Whitespace-collapsing case-insensitive string equivalence per FHIRPath
/// §6.1.5. Sequences of whitespace (Unicode space, tab, CR, LF) collapse to
/// a single space, leading/trailing whitespace is trimmed, and the comparison
/// is ASCII case-insensitive.
fn string_equivalent(a: &str, b: &str) -> bool {
    let na = normalize_for_equivalence(a);
    let nb = normalize_for_equivalence(b);
    na == nb
}

fn normalize_for_equivalence(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_ws = true; // leading whitespace gets trimmed via this guard
    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_ws {
                out.push(' ');
                prev_ws = true;
            }
        } else {
            out.extend(c.to_lowercase());
            prev_ws = false;
        }
    }
    if out.ends_with(' ') {
        out.pop();
    }
    out
}

/// Decimal equivalence: round both operands to the smaller fractional-digit
/// precision and compare. `1.2 / 1.8 ~ 0.67` succeeds because 0.666… rounded
/// to two digits equals 0.67.  With `Decimal`, scale() gives the number of
/// fractional digits (preserving trailing zeros).
fn decimal_equivalent(a: &Decimal, b: &Decimal) -> bool {
    if a == b {
        return true;
    }
    let prec = a.scale().min(b.scale());
    if prec == 0 {
        return a == b;
    }
    a.round_dp(prec) == b.round_dp(prec)
}

/// Quantity equivalence: compare base values at the precision of the less
/// precise input. Per FHIRPath §6.5.5, trailing zeros after the decimal point
/// are significant, but for inter-unit equivalence the common approach is to
/// round to the smaller number of significant figures.
fn quantity_equivalent(a: f64, b: f64) -> bool {
    if (a - b).abs() < f64::EPSILON {
        return true;
    }
    let prec = significant_figures(a).min(significant_figures(b));
    if prec == 0 {
        return (a - b).abs() < 1.0;
    }
    let scale = 10f64.powi(prec - 1 - a.abs().log10().floor() as i32);
    (a * scale).round() == (b * scale).round()
}

/// Count significant figures in a floating-point value.
fn significant_figures(n: f64) -> i32 {
    if n == 0.0 {
        return 1;
    }
    let s = format!("{}", n.abs());
    // Strip leading zeros and decimal point to count significant digits.
    let digits: String = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .trim_start_matches('0')
        .to_string();
    digits.len().max(1) as i32
}

/// If `v` is a single-item collection, write its content into `buf` and return
/// a reference to `buf`; otherwise return `v` unchanged. This implements the
/// FHIRPath singleton-equivalence rule for equality operators.
fn unwrap_singleton<'a>(
    v: &'a FhirPathValue,
    buf: &'a mut Option<FhirPathValue>,
) -> &'a FhirPathValue {
    if let FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) = v {
        if items.len() == 1 {
            *buf = Some(items[0].clone());
            return buf.as_ref().unwrap();
        }
    }
    v
}

/// Coerce a value to a known boolean for three-valued logic. Returns `None`
/// for the empty collection (which represents "unknown" per the FHIRPath
/// spec) and `Some(b)` for a concrete Boolean.
fn bool_three_valued(v: &FhirPathValue) -> Option<bool> {
    match v {
        FhirPathValue::Boolean(b) => Some(*b),
        FhirPathValue::Empty => None,
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
            if items.is_empty() =>
        {
            None
        }
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
            if items.len() == 1 =>
        {
            bool_three_valued(&items[0])
        }
        _ => Some(v.to_boolean()),
    }
}
