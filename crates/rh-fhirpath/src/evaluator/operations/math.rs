//! Mathematical functions for FHIRPath expressions

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct MathEvaluator;

impl MathEvaluator {
    pub fn abs(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(n.abs())),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(n.abs())),
            FhirPathValue::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: value.abs(),
                unit: unit.clone(),
            }),
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.len() == 1 {
                    Self::abs(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Empty)
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

    pub fn ceiling(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(
                n.ceil().to_i64().unwrap_or(i64::MAX),
            )),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    pub fn exp(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Number(
                Decimal::from_f64_retain((*n as f64).exp()).unwrap_or(Decimal::ZERO),
            )),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(
                Decimal::from_f64_retain(n.to_f64().unwrap().exp()).unwrap_or(Decimal::ZERO),
            )),
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.len() == 1 {
                    Self::exp(&items[0])
                } else if items.is_empty() {
                    Ok(FhirPathValue::Empty)
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

    pub fn floor(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(
                n.floor().to_i64().unwrap_or(i64::MIN),
            )),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    pub fn ln(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => {
                if *n <= 0 {
                    Err(FhirPathError::EvaluationError {
                        message: "ln() requires a positive number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(
                        Decimal::from_f64_retain((*n as f64).ln()).unwrap_or(Decimal::ZERO),
                    ))
                }
            }
            FhirPathValue::Number(n) => {
                if *n <= Decimal::ZERO {
                    Err(FhirPathError::EvaluationError {
                        message: "ln() requires a positive number".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(
                        Decimal::from_f64_retain(n.to_f64().unwrap().ln()).unwrap_or(Decimal::ZERO),
                    ))
                }
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    pub fn log(target: &FhirPathValue, base: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let target_num = Self::extract_number_f64(target)?;
        let base_num = Self::extract_number_f64(base)?;

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

        Ok(FhirPathValue::Number(
            Decimal::from_f64_retain(target_num.log(base_num)).unwrap_or(Decimal::ZERO),
        ))
    }

    pub fn power(
        target: &FhirPathValue,
        exponent: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let base_num = Self::extract_number_f64(target)?;
        let exp_num = Self::extract_number_f64(exponent)?;

        let result = base_num.powf(exp_num);

        if result.is_nan() {
            return Ok(FhirPathValue::Empty);
        }

        if result.fract() == 0.0
            && Self::is_integer_value(target)
            && Self::is_integer_value(exponent)
        {
            Ok(FhirPathValue::Integer(result as i64))
        } else {
            Ok(FhirPathValue::Number(
                Decimal::from_f64_retain(result).unwrap_or(Decimal::ZERO),
            ))
        }
    }

    pub fn round(
        target: &FhirPathValue,
        precision: Option<&FhirPathValue>,
    ) -> FhirPathResult<FhirPathValue> {
        let num = Self::extract_number(target)?;

        let precision_val: u32 = if let Some(p) = precision {
            match p {
                FhirPathValue::Integer(i) => {
                    if *i < 0 {
                        return Err(FhirPathError::EvaluationError {
                            message: "round() precision must be non-negative".to_string(),
                        });
                    }
                    *i as u32
                }
                FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                    if items.len() == 1 =>
                {
                    match &items[0] {
                        FhirPathValue::Integer(i) => {
                            if *i < 0 {
                                return Err(FhirPathError::EvaluationError {
                                    message: "round() precision must be non-negative".to_string(),
                                });
                            }
                            *i as u32
                        }
                        _ => {
                            return Err(FhirPathError::TypeError {
                                message: "round() precision must be an integer".to_string(),
                            })
                        }
                    }
                }
                _ => {
                    return Err(FhirPathError::TypeError {
                        message: "round() precision must be an integer".to_string(),
                    })
                }
            }
        } else {
            0
        };

        let result = num.round_dp(precision_val);

        if precision_val == 0 && Self::is_integer_value(target) {
            Ok(FhirPathValue::Integer(
                result.trunc().to_i64().unwrap_or(i64::MAX),
            ))
        } else {
            Ok(FhirPathValue::Number(result))
        }
    }

    pub fn sqrt(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => {
                if *n < 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(
                        Decimal::from_f64_retain((*n as f64).sqrt()).unwrap_or(Decimal::ZERO),
                    ))
                }
            }
            FhirPathValue::Number(n) => {
                if *n < Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(
                        Decimal::from_f64_retain(n.to_f64().unwrap().sqrt())
                            .unwrap_or(Decimal::ZERO),
                    ))
                }
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    pub fn truncate(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Integer(n) => Ok(FhirPathValue::Integer(*n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Integer(n.trunc().to_i64().unwrap_or(0))),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    fn extract_number(value: &FhirPathValue) -> FhirPathResult<Decimal> {
        match value {
            FhirPathValue::Integer(n) => Ok(Decimal::from(*n)),
            FhirPathValue::Number(n) => Ok(*n),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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

    fn extract_number_f64(value: &FhirPathValue) -> FhirPathResult<f64> {
        match value {
            FhirPathValue::Integer(n) => Ok(*n as f64),
            FhirPathValue::Number(n) => Ok(n.to_f64().unwrap_or(0.0)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.len() == 1 {
                    Self::extract_number_f64(&items[0])
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

    fn is_integer_value(value: &FhirPathValue) -> bool {
        match value {
            FhirPathValue::Integer(_) => true,
            FhirPathValue::Number(n) => n.fract() == Decimal::ZERO,
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.len() == 1 && Self::is_integer_value(&items[0])
            }
            _ => false,
        }
    }

    fn type_name(value: &FhirPathValue) -> &'static str {
        match value {
            FhirPathValue::Boolean(_) => "Boolean",
            FhirPathValue::TypedBoolean { .. } => "Boolean",
            FhirPathValue::Integer(_) => "Integer",
            FhirPathValue::Long(_) => "Long",
            FhirPathValue::Number(_) => "Number",
            FhirPathValue::String(_) | FhirPathValue::TypedString { .. } => "String",
            FhirPathValue::Date(_) => "Date",
            FhirPathValue::DateTime(_) => "DateTime",
            FhirPathValue::TypedDateTime { .. } => "DateTime",
            FhirPathValue::Time(_) => "Time",
            FhirPathValue::Quantity { .. } => "Quantity",
            FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_) => "Collection",
            FhirPathValue::Object(_) => "Object",
            FhirPathValue::TypedObject { .. } => "Object",
            FhirPathValue::FhirPrimitive { inner, .. } => Self::type_name(inner),
            FhirPathValue::DateTimePrecision(_) => "DateTimePrecision",
            FhirPathValue::Empty => "Empty",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> Decimal {
        Decimal::from_str_exact(s).unwrap()
    }

    #[test]
    fn test_abs() {
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Integer(-5)).unwrap(),
            FhirPathValue::Integer(5)
        );
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Number(d("-3.15"))).unwrap(),
            FhirPathValue::Number(d("3.15"))
        );
        assert_eq!(
            MathEvaluator::abs(&FhirPathValue::Integer(10)).unwrap(),
            FhirPathValue::Integer(10)
        );
    }

    #[test]
    fn test_ceiling() {
        assert_eq!(
            MathEvaluator::ceiling(&FhirPathValue::Number(d("3.15"))).unwrap(),
            FhirPathValue::Integer(4)
        );
        assert_eq!(
            MathEvaluator::ceiling(&FhirPathValue::Number(d("-2.7"))).unwrap(),
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
            MathEvaluator::floor(&FhirPathValue::Number(d("3.15"))).unwrap(),
            FhirPathValue::Integer(3)
        );
        assert_eq!(
            MathEvaluator::floor(&FhirPathValue::Number(d("-2.7"))).unwrap(),
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
            FhirPathValue::Number(d("4"))
        );
        assert_eq!(
            MathEvaluator::sqrt(&FhirPathValue::Number(d("2.25"))).unwrap(),
            FhirPathValue::Number(d("1.5"))
        );

        assert_eq!(
            MathEvaluator::sqrt(&FhirPathValue::Integer(-1)).unwrap(),
            FhirPathValue::Empty
        );
    }

    #[test]
    fn test_truncate() {
        assert_eq!(
            MathEvaluator::truncate(&FhirPathValue::Number(d("3.15"))).unwrap(),
            FhirPathValue::Integer(3)
        );
        assert_eq!(
            MathEvaluator::truncate(&FhirPathValue::Number(d("-2.7"))).unwrap(),
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
            MathEvaluator::power(
                &FhirPathValue::Number(d("2")),
                &FhirPathValue::Number(d("0.5"))
            )
            .unwrap(),
            FhirPathValue::Number(Decimal::from_f64_retain(2.0_f64.sqrt()).unwrap())
        );
    }

    #[test]
    fn test_round() {
        assert_eq!(
            MathEvaluator::round(
                &FhirPathValue::Number(d("3.15159")),
                Some(&FhirPathValue::Integer(2))
            )
            .unwrap(),
            FhirPathValue::Number(d("3.15"))
        );
        assert_eq!(
            MathEvaluator::round(&FhirPathValue::Number(d("3.7")), None).unwrap(),
            FhirPathValue::Number(d("4"))
        );
        assert_eq!(
            MathEvaluator::round(&FhirPathValue::Integer(5), None).unwrap(),
            FhirPathValue::Integer(5)
        );
    }
}
