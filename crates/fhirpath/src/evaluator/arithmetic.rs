//! Arithmetic operations for FHIRPath values

use crate::ast::*;
use crate::error::*;
use crate::evaluator::values::FhirPathValue;

/// Arithmetic operations handler
pub struct ArithmeticEvaluator;

impl ArithmeticEvaluator {
    /// Evaluate additive operation (+, -, &)
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

    /// Evaluate multiplicative operation (*, /, div, mod)
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

    /// Evaluate polarity operation (+ or -)
    pub fn evaluate_polarity(
        operator: &PolarityOperator,
        operand: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            PolarityOperator::Plus => Ok(operand.clone()),
            PolarityOperator::Minus => Self::negate_value(operand),
        }
    }

    /// Add two values
    pub fn add_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a + b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a + b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(*a as f64 + b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a + *b as f64))
            }
            (FhirPathValue::String(a), FhirPathValue::String(b)) => {
                Ok(FhirPathValue::String(format!("{a}{b}")))
            }
            // Quantity arithmetic - addition requires same units
            (
                FhirPathValue::Quantity { value: a, unit: unit_a },
                FhirPathValue::Quantity { value: b, unit: unit_b },
            ) => {
                if unit_a == unit_b {
                    Ok(FhirPathValue::Quantity {
                        value: a + b,
                        unit: unit_a.clone(),
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot add quantities with different units: {:?} and {:?}",
                            unit_a, unit_b
                        ),
                    })
                }
            }
            // Mixed quantity and numeric operations (treating numeric as dimensionless)
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n)) |
            (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                if unit.is_none() {
                    // Only allow if quantity has no unit (dimensionless)
                    Ok(FhirPathValue::Quantity {
                        value: value + n,
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot add number to quantity with units".to_string(),
                    })
                }
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) |
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                if unit.is_none() {
                    // Only allow if quantity has no unit (dimensionless)
                    Ok(FhirPathValue::Quantity {
                        value: value + (*i as f64),
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot add integer to quantity with units".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot add {left:?} and {right:?}"),
            }),
        }
    }

    /// Subtract two values
    pub fn subtract_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a - b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a - b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(*a as f64 - b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a - *b as f64))
            }
            // Quantity arithmetic - subtraction requires same units
            (
                FhirPathValue::Quantity { value: a, unit: unit_a },
                FhirPathValue::Quantity { value: b, unit: unit_b },
            ) => {
                if unit_a == unit_b {
                    Ok(FhirPathValue::Quantity {
                        value: a - b,
                        unit: unit_a.clone(),
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot subtract quantities with different units: {:?} and {:?}",
                            unit_a, unit_b
                        ),
                    })
                }
            }
            // Mixed quantity and numeric operations (treating numeric as dimensionless)
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n)) => {
                if unit.is_none() {
                    Ok(FhirPathValue::Quantity {
                        value: value - n,
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot subtract number from quantity with units".to_string(),
                    })
                }
            }
            (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                if unit.is_none() {
                    Ok(FhirPathValue::Quantity {
                        value: n - value,
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot subtract quantity with units from number".to_string(),
                    })
                }
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) => {
                if unit.is_none() {
                    Ok(FhirPathValue::Quantity {
                        value: value - (*i as f64),
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot subtract integer from quantity with units".to_string(),
                    })
                }
            }
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                if unit.is_none() {
                    Ok(FhirPathValue::Quantity {
                        value: (*i as f64) - value,
                        unit: None,
                    })
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Cannot subtract quantity with units from integer".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot subtract {left:?} and {right:?}"),
            }),
        }
    }

    /// Multiply two values
    pub fn multiply_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a * b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a * b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(*a as f64 * b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a * *b as f64))
            }
            // Quantity multiplication by scalars
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n)) |
            (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                Ok(FhirPathValue::Quantity {
                    value: value * n,
                    unit: unit.clone(),
                })
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) |
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                Ok(FhirPathValue::Quantity {
                    value: value * (*i as f64),
                    unit: unit.clone(),
                })
            }
            // Quantity * Quantity is more complex (unit multiplication) - for now, error
            (
                FhirPathValue::Quantity { .. },
                FhirPathValue::Quantity { .. },
            ) => {
                Err(FhirPathError::EvaluationError {
                    message: "Multiplication of two quantities is not yet supported (requires unit algebra)".to_string(),
                })
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot multiply {left:?} and {right:?}"),
            }),
        }
    }

    /// Divide two values
    pub fn divide_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    // Division always returns a Number (float) in FHIRPath
                    Ok(FhirPathValue::Number(*a as f64 / *b as f64))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a / b))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(*a as f64 / b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a / *b as f64))
                }
            }
            // Quantity division by scalars
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n)) => {
                if *n == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Quantity {
                        value: value / n,
                        unit: unit.clone(),
                    })
                }
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) => {
                if *i == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Quantity {
                        value: value / (*i as f64),
                        unit: unit.clone(),
                    })
                }
            }
            // Number/Integer divided by Quantity (result is different unit)
            (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                if *value == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else if unit.is_none() {
                    // Dividing by dimensionless quantity
                    Ok(FhirPathValue::Number(n / value))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Division of number by quantity with units is not supported".to_string(),
                    })
                }
            }
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                if *value == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else if unit.is_none() {
                    // Dividing by dimensionless quantity
                    Ok(FhirPathValue::Number((*i as f64) / value))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Division of integer by quantity with units is not supported".to_string(),
                    })
                }
            }
            // Quantity / Quantity with same units -> dimensionless number
            (
                FhirPathValue::Quantity { value: a, unit: unit_a },
                FhirPathValue::Quantity { value: b, unit: unit_b },
            ) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else if unit_a == unit_b {
                    // Same units cancel out, result is dimensionless
                    Ok(FhirPathValue::Number(a / b))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Division of quantities with different units is not supported: {:?} / {:?}",
                            unit_a, unit_b
                        ),
                    })
                }
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot divide {left:?} and {right:?}"),
            }),
        }
    }

    /// Integer division (div)
    pub fn div_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer(a / b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer((a / b).trunc() as i64))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer((*a as f64 / b).trunc() as i64))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer((a / *b as f64).trunc() as i64))
                }
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot perform integer division on {left:?} and {right:?}"),
            }),
        }
    }

    /// Modulo operation
    pub fn mod_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer(a % b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a % b))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(*a as f64 % b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a % *b as f64))
                }
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot perform modulo on {left:?} and {right:?}"),
            }),
        }
    }

    /// Concatenate values (string concatenation)
    pub fn concatenate_values(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_str = left.to_string_value()?;
        let right_str = right.to_string_value()?;
        Ok(FhirPathValue::String(format!("{left_str}{right_str}")))
    }

    /// Negate a value
    pub fn negate_value(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match value {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(-n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(-n)),
            FhirPathValue::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: -value,
                unit: unit.clone(),
            }),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot negate {value:?}"),
            }),
        }
    }
}
