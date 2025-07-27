//! Comparison and equality operations for FHIRPath values

use crate::ast::*;
use crate::error::*;
use crate::evaluator::values::FhirPathValue;

/// Comparison operations handler
pub struct ComparisonEvaluator;

impl ComparisonEvaluator {
    /// Evaluate equality operation
    pub fn evaluate_equality(
        left: &FhirPathValue,
        operator: &EqualityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
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

    /// Check if two values are equivalent (more lenient than equal)
    pub fn values_equivalent(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        // For now, same as equal - can be extended for type coercion
        FhirPathValue::equals_static(left, right)
    }
}
