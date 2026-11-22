//! Arithmetic operations for FHIRPath values.

use crate::ast::{AdditiveOperator, MultiplicativeOperator, PolarityOperator};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::{temporal::shift_temporal, units::UnitConverter};
use crate::evaluator::types::FhirPathValue;
use rust_decimal::Decimal;

use FhirPathValue::{Date, DateTime, Empty, Integer, Long, Number, Quantity, String, Time};

pub(super) fn integer_result(value: i64) -> FhirPathValue {
    i32::try_from(value).map_or(Empty, |value| Integer(i64::from(value)))
}

pub(super) fn numeric_value(value: &FhirPathValue) -> Option<Decimal> {
    match value {
        Integer(value) | Long(value) => Some(Decimal::from(*value)),
        Number(value) => Some(*value),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Div,
    Mod,
}

impl ArithmeticOp {
    fn integers(self, left: i64, right: i64) -> Option<i64> {
        match self {
            Self::Add => left.checked_add(right),
            Self::Subtract => left.checked_sub(right),
            Self::Multiply => left.checked_mul(right),
            Self::Div => left.checked_div(right),
            Self::Mod => left.checked_rem(right),
            Self::Divide => unreachable!("division always produces a decimal"),
        }
    }

    fn decimals(self, left: Decimal, right: Decimal) -> Option<Decimal> {
        match self {
            Self::Add => left.checked_add(right),
            Self::Subtract => left.checked_sub(right),
            Self::Multiply => left
                .checked_mul(right)
                .filter(|result| !result.is_zero() || left.is_zero() || right.is_zero()),
            Self::Divide => left
                .checked_div(right)
                .filter(|result| !result.is_zero() || left.is_zero()),
            Self::Div => left.checked_div(right).map(|value| value.trunc()),
            Self::Mod => left.checked_rem(right),
        }
    }
}

/// Arithmetic operations on singleton operands.
pub struct ArithmeticEvaluator;

impl ArithmeticEvaluator {
    pub fn evaluate_additive(
        left: &FhirPathValue,
        operator: &AdditiveOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            AdditiveOperator::Add => Self::add_values(left, right),
            AdditiveOperator::Subtract => Self::subtract_values(left, right),
            AdditiveOperator::Concatenate => Self::concatenate_values(left, right),
        }
    }

    pub fn evaluate_multiplicative(
        left: &FhirPathValue,
        operator: &MultiplicativeOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            MultiplicativeOperator::Multiply => Self::multiply_values(left, right),
            MultiplicativeOperator::Divide => Self::divide_values(left, right),
            MultiplicativeOperator::Div => Self::div_values(left, right),
            MultiplicativeOperator::Mod => Self::mod_values(left, right),
        }
    }

    pub fn evaluate_polarity(
        operator: &PolarityOperator,
        operand: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let operand = operand
            .singleton("Polarity")?
            .map_or(&Empty, FhirPathValue::system_value);
        match operator {
            PolarityOperator::Plus
                if numeric_value(operand).is_some()
                    || matches!(operand, Quantity { .. } | Empty) =>
            {
                Ok(operand.clone())
            }
            PolarityOperator::Plus => Err(FhirPathError::TypeError {
                message: format!("Cannot apply unary + to {operand:?}"),
            }),
            PolarityOperator::Minus => Self::negate_value(operand),
        }
    }

    pub fn add_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Add)
    }

    pub fn subtract_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Subtract)
    }

    pub fn multiply_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Multiply)
    }

    pub fn divide_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Divide)
    }

    pub fn div_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Div)
    }

    pub fn mod_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Self::apply(left, right, ArithmeticOp::Mod)
    }

    fn apply(
        left: &FhirPathValue,
        right: &FhirPathValue,
        op: ArithmeticOp,
    ) -> FhirPathResult<FhirPathValue> {
        let left = left
            .singleton("Arithmetic")?
            .map_or(&Empty, FhirPathValue::system_value);
        let right = right
            .singleton("Arithmetic")?
            .map_or(&Empty, FhirPathValue::system_value);
        if matches!(left, Empty) || matches!(right, Empty) {
            return Ok(Empty);
        }

        if !matches!(op, ArithmeticOp::Divide) {
            match (left, right) {
                (Integer(a), Integer(b)) => {
                    return Ok(op.integers(*a, *b).map_or(Empty, integer_result))
                }
                (Long(a) | Integer(a), Long(b)) | (Long(a), Integer(b)) => {
                    return Ok(op.integers(*a, *b).map_or(Empty, Long));
                }
                _ => {}
            }
        }
        if let (Some(a), Some(b)) = (numeric_value(left), numeric_value(right)) {
            return Ok(op.decimals(a, b).map_or(Empty, Number));
        }

        if let ArithmeticOp::Add = op {
            if let (Some(a), Some(b)) = (left.as_str(), right.as_str()) {
                return Ok(String(format!("{a}{b}")));
            }
        }
        if let (
            Date(_) | DateTime(_) | Time(_) | FhirPathValue::TypedDateTime { .. },
            Quantity {
                value,
                unit: Some(unit),
            },
        ) = (left, right)
        {
            match op {
                ArithmeticOp::Add => return shift_temporal(left, *value, unit),
                ArithmeticOp::Subtract => return shift_temporal(left, -*value, unit),
                _ => {}
            }
        }
        if let (Some((a, unit_a)), Some((b, unit_b))) = (as_quantity(left), as_quantity(right)) {
            let converter = UnitConverter::new();
            match op {
                ArithmeticOp::Add => return converter.add_quantities(a, unit_a, b, unit_b),
                ArithmeticOp::Subtract => {
                    return converter.subtract_quantities(a, unit_a, b, unit_b)
                }
                ArithmeticOp::Multiply => {
                    return converter.multiply_quantities(a, unit_a, b, unit_b)
                }
                ArithmeticOp::Divide => return converter.divide_quantities(a, unit_a, b, unit_b),
                ArithmeticOp::Div | ArithmeticOp::Mod => {}
            }
        }
        Err(FhirPathError::TypeError {
            message: format!("Incompatible arithmetic operands: {left:?} and {right:?}"),
        })
    }

    pub fn concatenate_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_str = left
            .singleton("Concatenation")?
            .map_or(&Empty, FhirPathValue::system_value)
            .to_string_value()?;
        let right_str = right
            .singleton("Concatenation")?
            .map_or(&Empty, FhirPathValue::system_value)
            .to_string_value()?;
        Ok(String(format!("{left_str}{right_str}")))
    }

    pub fn negate_value(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value
            .singleton("Negation")?
            .map_or(&Empty, FhirPathValue::system_value)
        {
            Integer(n) => Ok(n.checked_neg().map_or(Empty, integer_result)),
            Long(n) => Ok(n.checked_neg().map_or(Empty, Long)),
            Number(n) => Ok(Number(-n)),
            Quantity { value, unit } => Ok(Quantity {
                value: -value,
                unit: unit.clone(),
            }),
            Empty => Ok(Empty),
            _ => Err(FhirPathError::TypeError {
                message: format!("Cannot negate {value:?}"),
            }),
        }
    }
}

fn as_quantity(value: &FhirPathValue) -> Option<(Decimal, &Option<std::string::String>)> {
    match value {
        Quantity { value, unit } => Some((*value, unit)),
        _ => numeric_value(value).map(|value| (value, &None)),
    }
}
