//! Mathematical functions for FHIRPath expressions

use crate::error::*;
use crate::evaluator::values::FhirPathValue;

/// Evaluator for mathematical functions in FHIRPath expressions
pub struct MathEvaluator;

impl MathEvaluator {
    /// Absolute value function - abs()
    /// Returns the absolute value of a number
    pub fn abs(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(n.abs())),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(n.abs())),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::abs(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "abs() can only be applied to a single numeric value".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("abs() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Ceiling function - ceiling()
    /// Returns the smallest integer greater than or equal to the input
    pub fn ceiling(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(n.ceil() as i64)),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::ceiling(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "ceiling() can only be applied to a single numeric value"
                            .to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("ceiling() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Exponential function - exp()
    /// Returns e raised to the power of the input
    pub fn exp(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Number((*n as f64).exp())),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(n.exp())),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::exp(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "exp() can only be applied to a single numeric value".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("exp() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Floor function - floor()
    /// Returns the largest integer less than or equal to the input
    pub fn floor(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(n.floor() as i64)),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::floor(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "floor() can only be applied to a single numeric value"
                            .to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("floor() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Natural logarithm function - ln()
    /// Returns the natural logarithm of the input
    pub fn ln(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => {
                if *n <= 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "ln() requires a positive number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number((*n as f64).ln()))
                }
            }
            FhirPathValue::Number(n) => {
                if *n <= 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "ln() requires a positive number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(n.ln()))
                }
            }
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::ln(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "ln() can only be applied to a single numeric value".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("ln() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Logarithm function - log(base)
    /// Returns the logarithm of the target with the specified base
    pub fn log(target: &FhirPathValue, base: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let target_num = Self::extract_number(target)?;
        let base_num = Self::extract_number(base)?;

        if target_num <= 0.0 {
            return Err(FhirPathError::EvaluationError {
                message: "log() requires a positive number".to_string(),
            });
        }

        if base_num <= 0.0 || base_num == 1.0 {
            return Err(FhirPathError::EvaluationError {
                message: "log() requires a positive base that is not equal to 1".to_string(),
            });
        }

        Ok(FhirPathValue::Number(target_num.log(base_num)))
    }

    /// Power function - power(exponent)
    /// Returns the target raised to the power of the exponent
    pub fn power(
        target: &FhirPathValue,
        exponent: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let base_num = Self::extract_number(target)?;
        let exp_num = Self::extract_number(exponent)?;

        let result = base_num.powf(exp_num);

        // If the result is a whole number and both inputs were integers, return an integer
        if result.fract() == 0.0
            && Self::is_integer_value(target)
            && Self::is_integer_value(exponent)
        {
            Ok(FhirPathValue::Integer(result as i64))
        } else {
            Ok(FhirPathValue::Number(result))
        }
    }

    /// Round function - round([precision])
    /// Returns the input rounded to the specified number of decimal places (default 0)
    pub fn round(
        target: &FhirPathValue,
        precision: Option<&FhirPathValue>,
    ) -> FhirPathResult<FhirPathValue> {
        let num = Self::extract_number(target)?;

        let precision_val = if let Some(p) = precision {
            match p {
                FhirPathValue::Integer(i) => *i as i32,
                FhirPathValue::Collection(items) if items.len() == 1 => match &items[0] {
                    FhirPathValue::Integer(i) => *i as i32,
                    _ => {
                        return Err(FhirPathError::TypeError {
                            message: "round() precision must be an integer".to_string(),
                        })
                    }
                },
                _ => {
                    return Err(FhirPathError::TypeError {
                        message: "round() precision must be an integer".to_string(),
                    })
                }
            }
        } else {
            0
        };

        if precision_val < 0 {
            return Err(FhirPathError::EvaluationError {
                message: "round() precision must be non-negative".to_string(),
            });
        }

        let multiplier = 10_f64.powi(precision_val);
        let result = (num * multiplier).round() / multiplier;

        // If precision is 0 and the original was an integer, return integer
        if precision_val == 0 && Self::is_integer_value(target) {
            Ok(FhirPathValue::Integer(result as i64))
        } else {
            Ok(FhirPathValue::Number(result))
        }
    }

    /// Square root function - sqrt()
    /// Returns the square root of the input
    pub fn sqrt(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => {
                if *n < 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "sqrt() requires a non-negative number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number((*n as f64).sqrt()))
                }
            }
            FhirPathValue::Number(n) => {
                if *n < 0.0 {
                    Err(FhirPathError::EvaluationError {
                        message: "sqrt() requires a non-negative number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(n.sqrt()))
                }
            }
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::sqrt(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "sqrt() can only be applied to a single numeric value".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("sqrt() cannot be applied to {}", Self::type_name(target)),
            }),
        }
    }

    /// Truncate function - truncate()
    /// Returns the integer part of the input (truncates toward zero)
    pub fn truncate(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(n.trunc() as i64)),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::truncate(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Collection(vec![]))
                } else {
                    Err(FhirPathError::TypeError {
                        message: "truncate() can only be applied to a single numeric value"
                            .to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!(
                    "truncate() cannot be applied to {}",
                    Self::type_name(target)
                ),
            }),
        }
    }

    // Helper functions

    /// Extract a numeric value from a FhirPathValue
    fn extract_number(value: &FhirPathValue) -> FhirPathResult<f64> {
        match value {
            FhirPathValue::Integer(n) => Ok(*n as f64),
            FhirPathValue::Number(n) => Ok(*n),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Self::extract_number(&items[0])
                } else if items.is_empty() {
                    Err(FhirPathError::TypeError {
                        message: "Cannot extract number from empty collection".to_string(),
                    })
                } else {
                    Err(FhirPathError::TypeError {
                        message: "Cannot extract number from multi-item collection".to_string(),
                    })
                }
            }
            _ => Err(FhirPathError::TypeError {
                message: format!("Expected number, got {}", Self::type_name(value)),
            }),
        }
    }

    /// Check if a value represents an integer (even if stored as Number)
    fn is_integer_value(value: &FhirPathValue) -> bool {
        match value {
            FhirPathValue::Integer(_) => true,
            FhirPathValue::Number(n) => n.fract() == 0.0,
            FhirPathValue::Collection(items) => {
                items.len() == 1 && Self::is_integer_value(&items[0])
            }
            _ => false,
        }
    }

    /// Get type name for error messages
    fn type_name(value: &FhirPathValue) -> &'static str {
        match value {
            FhirPathValue::Boolean(_) => "Boolean",
            FhirPathValue::Integer(_) => "Integer",
            FhirPathValue::Number(_) => "Number",
            FhirPathValue::String(_) => "String",
            FhirPathValue::Date(_) => "Date",
            FhirPathValue::DateTime(_) => "DateTime",
            FhirPathValue::Time(_) => "Time",
            FhirPathValue::Quantity { .. } => "Quantity",
            FhirPathValue::Collection(_) => "Collection",
            FhirPathValue::Object(_) => "Object",
            FhirPathValue::DateTimePrecision(_) => "DateTimePrecision",
            FhirPathValue::Empty => "Empty",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Integer(-5)).unwrap(),
            FhirPathValue::Integer(5)
        );
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Number(-3.15)).unwrap(),
            FhirPathValue::Number(3.15)
        );
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Integer(10)).unwrap(),
            FhirPathValue::Integer(10)
        );
    }

    #[test]
    fn test_ceiling() {
        assert_eq!(
            MathEvaluator::ceiling(&FhirPathValue::Number(3.15)).unwrap(),
            FhirPathValue::Integer(4)
        );
        assert_eq!(
            MathEvaluator::ceiling(&FhirPathValue::Number(-2.7)).unwrap(),
            FhirPathValue::Integer(-2)
        );
        assert_eq!(
            MathEvaluator::ceiling(&FhirPathValue::Integer(5)).unwrap(),
            FhirPathValue::Integer(5)
        );
    }

    #[test]
    fn test_floor() {
        assert_eq!(
            MathEvaluator::floor(&FhirPathValue::Number(3.15)).unwrap(),
            FhirPathValue::Integer(3)
        );
        assert_eq!(
            MathEvaluator::floor(&FhirPathValue::Number(-2.7)).unwrap(),
            FhirPathValue::Integer(-3)
        );
        assert_eq!(
            MathEvaluator::floor(&FhirPathValue::Integer(5)).unwrap(),
            FhirPathValue::Integer(5)
        );
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(
            MathEvaluator::sqrt(&FhirPathValue::Integer(16)).unwrap(),
            FhirPathValue::Number(4.0)
        );
        assert_eq!(
            MathEvaluator::sqrt(&FhirPathValue::Number(2.25)).unwrap(),
            FhirPathValue::Number(1.5)
        );

        // Test error for negative number
        assert!(MathEvaluator::sqrt(&FhirPathValue::Integer(-1)).is_err());
    }

    #[test]
    fn test_truncate() {
        assert_eq!(
            MathEvaluator::truncate(&FhirPathValue::Number(3.15)).unwrap(),
            FhirPathValue::Integer(3)
        );
        assert_eq!(
            MathEvaluator::truncate(&FhirPathValue::Number(-2.7)).unwrap(),
            FhirPathValue::Integer(-2)
        );
        assert_eq!(
            MathEvaluator::truncate(&FhirPathValue::Integer(5)).unwrap(),
            FhirPathValue::Integer(5)
        );
    }

    #[test]
    fn test_power() {
        assert_eq!(
            MathEvaluator::power(&FhirPathValue::Integer(2), &FhirPathValue::Integer(3)).unwrap(),
            FhirPathValue::Integer(8)
        );
        assert_eq!(
            MathEvaluator::power(&FhirPathValue::Number(2.0), &FhirPathValue::Number(0.5)).unwrap(),
            FhirPathValue::Number(2.0_f64.sqrt()) // Use computed sqrt(2)
        );
    }

    #[test]
    fn test_round() {
        assert_eq!(
            MathEvaluator::round(
                &FhirPathValue::Number(3.15159),
                Some(&FhirPathValue::Integer(2))
            )
            .unwrap(),
            FhirPathValue::Number(3.15)
        );
        assert_eq!(
            MathEvaluator::round(&FhirPathValue::Number(3.7), None).unwrap(),
            FhirPathValue::Number(4.0)
        );
        // Test with integer input - should return integer
        assert_eq!(
            MathEvaluator::round(&FhirPathValue::Integer(5), None).unwrap(),
            FhirPathValue::Integer(5)
        );
    }
}
