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
}
