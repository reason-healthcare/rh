//! Comparison and equality operations for FHIRPath values

use std::cmp::Ordering;

use crate::ast::*;
use crate::error::*;
use crate::evaluator::operations::temporal::{
    compare_parts, equals_parts, parse_temporal, TemporalParts,
};
use crate::evaluator::types::FhirPathValue;

/// Comparison operations handler
pub struct ComparisonEvaluator;

/// Extract a temporal-parts view from any temporal-valued FhirPathValue.
/// Returns None for non-temporal values or unparseable strings.
fn try_temporal(v: &FhirPathValue) -> Option<TemporalParts> {
    match v {
        FhirPathValue::Date(s) | FhirPathValue::DateTime(s) | FhirPathValue::Time(s) => {
            parse_temporal(s)
        }
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

    /// Evaluate AND operation
    pub fn evaluate_and(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_bool = left.to_boolean();
        let right_bool = right.to_boolean();
        Ok(FhirPathValue::Boolean(left_bool && right_bool))
    }

    /// Evaluate implies operation
    /// If left is true, return boolean evaluation of right
    /// If left is false, return true
    /// If left is empty and right evaluates to true, return true, otherwise return empty
    pub fn evaluate_implies(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match left {
            FhirPathValue::Boolean(true) => {
                // If left is true, return the boolean evaluation of the right operand
                Ok(FhirPathValue::Boolean(right.to_boolean()))
            }
            FhirPathValue::Boolean(false) => {
                // If left is false, return true
                Ok(FhirPathValue::Boolean(true))
            }
            FhirPathValue::Empty => {
                // If left is empty, return true if right evaluates to true, otherwise return empty
                if right.to_boolean() {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            _ => {
                // For other values, convert left to boolean and apply the logic
                let left_bool = left.to_boolean();
                if left_bool {
                    Ok(FhirPathValue::Boolean(right.to_boolean()))
                } else {
                    Ok(FhirPathValue::Boolean(true))
                }
            }
        }
    }

    /// Evaluate or/xor operations
    pub fn evaluate_or(
        left: &FhirPathValue,
        operator: &OrOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_bool = left.to_boolean();
        let right_bool = right.to_boolean();

        let result = match operator {
            OrOperator::Or => left_bool || right_bool,
            OrOperator::Xor => left_bool ^ right_bool,
        };

        Ok(FhirPathValue::Boolean(result))
    }

    /// Check if two values are equivalent (more lenient than equal)
    pub fn values_equivalent(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        // For now, same as equal - can be extended for type coercion
        FhirPathValue::equals_static(left, right)
    }
}
