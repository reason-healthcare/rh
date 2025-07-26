//! Arithmetic operations for FHIRPath values

use crate::ast::*;
use crate::error::*;
use crate::evaluator::units::UnitConverter;
use crate::evaluator::values::FhirPathValue;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};

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
            // Quantity arithmetic - with automatic unit conversion
            (
                FhirPathValue::Quantity {
                    value: a,
                    unit: unit_a,
                },
                FhirPathValue::Quantity {
                    value: b,
                    unit: unit_b,
                },
            ) => {
                let converter = UnitConverter::new();
                converter.add_quantities(*a, unit_a, *b, unit_b)
            }
            // Mixed quantity and numeric operations (treating numeric as dimensionless)
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n))
            | (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
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
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i))
            | (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
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
            // Date/DateTime arithmetic with precision units
            (FhirPathValue::Date(date_str), FhirPathValue::DateTimePrecision(precision)) => {
                Self::add_precision_to_date(date_str, precision, 1)
            }
            (FhirPathValue::DateTimePrecision(precision), FhirPathValue::Date(date_str)) => {
                Self::add_precision_to_date(date_str, precision, 1)
            }
            (
                FhirPathValue::DateTime(datetime_str),
                FhirPathValue::DateTimePrecision(precision),
            ) => Self::add_precision_to_datetime(datetime_str, precision, 1),
            (
                FhirPathValue::DateTimePrecision(precision),
                FhirPathValue::DateTime(datetime_str),
            ) => Self::add_precision_to_datetime(datetime_str, precision, 1),
            // Date/DateTime arithmetic with compound duration quantities (6 months, etc.)
            (
                FhirPathValue::Date(date_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(date_str, &precision, *value as i64)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add quantity with unit '{unit_str}' to date"),
                    })
                }
            }
            (
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
                FhirPathValue::Date(date_str),
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(date_str, &precision, *value as i64)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add quantity with unit '{unit_str}' to date"),
                    })
                }
            }
            (
                FhirPathValue::DateTime(datetime_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_datetime(datetime_str, &precision, *value as i64)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot add quantity with unit '{unit_str}' to datetime"
                        ),
                    })
                }
            }
            (
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
                FhirPathValue::DateTime(datetime_str),
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_datetime(datetime_str, &precision, *value as i64)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot add quantity with unit '{unit_str}' to datetime"
                        ),
                    })
                }
            }
            // Support for "6 months" style expressions (number + precision)
            (FhirPathValue::Integer(count), FhirPathValue::DateTimePrecision(precision)) => {
                // Return a compound value that can be used with dates
                Ok(FhirPathValue::Collection(vec![
                    FhirPathValue::Integer(*count),
                    FhirPathValue::DateTimePrecision(precision.clone()),
                ]))
            }
            (FhirPathValue::DateTimePrecision(precision), FhirPathValue::Integer(count)) => {
                Ok(FhirPathValue::Collection(vec![
                    FhirPathValue::Integer(*count),
                    FhirPathValue::DateTimePrecision(precision.clone()),
                ]))
            }
            // Date + compound duration (6 months)
            (FhirPathValue::Date(date_str), FhirPathValue::Collection(items)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, *count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, *count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot add {left:?} and {right:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add {left:?} and {right:?}"),
                    })
                }
            }
            (FhirPathValue::Collection(items), FhirPathValue::Date(date_str)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, *count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, *count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot add {left:?} and {right:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add {left:?} and {right:?}"),
                    })
                }
            }
            // DateTime + compound duration (6 months)
            (FhirPathValue::DateTime(datetime_str), FhirPathValue::Collection(items)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, *count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, *count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot add {left:?} and {right:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add {left:?} and {right:?}"),
                    })
                }
            }
            (FhirPathValue::Collection(items), FhirPathValue::DateTime(datetime_str)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, *count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, *count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot add {left:?} and {right:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add {left:?} and {right:?}"),
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
            // Quantity arithmetic - with automatic unit conversion
            (
                FhirPathValue::Quantity {
                    value: a,
                    unit: unit_a,
                },
                FhirPathValue::Quantity {
                    value: b,
                    unit: unit_b,
                },
            ) => {
                let converter = UnitConverter::new();
                converter.subtract_quantities(*a, unit_a, *b, unit_b)
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
            // Date/DateTime arithmetic with precision units (subtraction)
            (FhirPathValue::Date(date_str), FhirPathValue::DateTimePrecision(precision)) => {
                Self::add_precision_to_date(date_str, precision, -1)
            }
            (
                FhirPathValue::DateTime(datetime_str),
                FhirPathValue::DateTimePrecision(precision),
            ) => Self::add_precision_to_datetime(datetime_str, precision, -1),
            // Date/DateTime arithmetic with compound duration quantities (subtraction)
            (
                FhirPathValue::Date(date_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(date_str, &precision, -(*value as i64))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot subtract quantity with unit '{unit_str}' from date"
                        ),
                    })
                }
            }
            (
                FhirPathValue::DateTime(datetime_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_datetime(datetime_str, &precision, -(*value as i64))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "Cannot subtract quantity with unit '{unit_str}' from datetime"
                        ),
                    })
                }
            }
            // Date - compound duration (6 months)
            (FhirPathValue::Date(date_str), FhirPathValue::Collection(items)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, -count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_date(date_str, precision, -count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot subtract {right:?} from {left:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot subtract {right:?} from {left:?}"),
                    })
                }
            }
            // DateTime - compound duration (6 months)
            (FhirPathValue::DateTime(datetime_str), FhirPathValue::Collection(items)) => {
                if items.len() == 2 {
                    if let (
                        FhirPathValue::Integer(count),
                        FhirPathValue::DateTimePrecision(precision),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, -count)
                    } else if let (
                        FhirPathValue::DateTimePrecision(precision),
                        FhirPathValue::Integer(count),
                    ) = (&items[0], &items[1])
                    {
                        Self::add_precision_to_datetime(datetime_str, precision, -count)
                    } else {
                        Err(FhirPathError::EvaluationError {
                            message: format!("Cannot subtract {right:?} from {left:?}"),
                        })
                    }
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot subtract {right:?} from {left:?}"),
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
                let converter = UnitConverter::new();
                Ok(converter.multiply_by_scalar(*value, unit, *n))
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) |
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                let converter = UnitConverter::new();
                Ok(converter.multiply_by_scalar(*value, unit, *i as f64))
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
                let converter = UnitConverter::new();
                converter.divide_by_scalar(*value, unit, *n)
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) => {
                let converter = UnitConverter::new();
                converter.divide_by_scalar(*value, unit, *i as f64)
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
                        message: "Division of number by quantity with units is not supported"
                            .to_string(),
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
                        message: "Division of integer by quantity with units is not supported"
                            .to_string(),
                    })
                }
            }
            // Quantity / Quantity with compatible units -> dimensionless number
            (
                FhirPathValue::Quantity {
                    value: a,
                    unit: unit_a,
                },
                FhirPathValue::Quantity {
                    value: b,
                    unit: unit_b,
                },
            ) => {
                let converter = UnitConverter::new();
                converter.divide_quantities(*a, unit_a, *b, unit_b)
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

    /// Add a precision unit to a date
    fn add_precision_to_date(
        date_str: &str,
        precision: &DateTimePrecision,
        count: i64,
    ) -> FhirPathResult<FhirPathValue> {
        // Parse different date formats
        let date = if date_str.len() == 4 {
            // Year only format like "2025"
            let year: i32 = date_str
                .parse()
                .map_err(|_| FhirPathError::EvaluationError {
                    message: format!("Invalid year format: {date_str}"),
                })?;
            NaiveDate::from_ymd_opt(year, 1, 1).ok_or_else(|| FhirPathError::EvaluationError {
                message: format!("Invalid date: {date_str}"),
            })?
        } else if date_str.len() == 7 {
            // Year-month format like "2025-03"
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() != 2 {
                return Err(FhirPathError::EvaluationError {
                    message: format!("Invalid date format: {date_str}"),
                });
            }
            let year: i32 = parts[0]
                .parse()
                .map_err(|_| FhirPathError::EvaluationError {
                    message: format!("Invalid year in date: {date_str}"),
                })?;
            let month: u32 = parts[1]
                .parse()
                .map_err(|_| FhirPathError::EvaluationError {
                    message: format!("Invalid month in date: {date_str}"),
                })?;
            NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid date: {date_str}"),
                }
            })?
        } else {
            // Full date format like "2025-03-15"
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid date format: {date_str}"),
                }
            })?
        };

        let new_date = match precision {
            DateTimePrecision::Year => {
                let new_year = (date.year() as i64 + count) as i32;
                let mut new_day = date.day();
                let new_month = date.month();

                // Handle leap year edge case (Feb 29 -> Feb 28 in non-leap year)
                loop {
                    if let Some(new_date) = NaiveDate::from_ymd_opt(new_year, new_month, new_day) {
                        break new_date;
                    }
                    new_day -= 1;
                    if new_day == 0 {
                        return Err(FhirPathError::EvaluationError {
                            message: format!("Invalid date after adding {count} years"),
                        });
                    }
                }
            }
            DateTimePrecision::Month => {
                let total_months = date.year() as i64 * 12 + date.month() as i64 - 1 + count;
                let new_year = (total_months / 12) as i32;
                let new_month = (total_months % 12 + 1) as u32;

                // Handle day overflow for months with different numbers of days
                let mut new_day = date.day();
                loop {
                    if let Some(new_date) = NaiveDate::from_ymd_opt(new_year, new_month, new_day) {
                        break new_date;
                    }
                    new_day -= 1;
                    if new_day == 0 {
                        return Err(FhirPathError::EvaluationError {
                            message: format!("Invalid date after adding {count} months"),
                        });
                    }
                }
            }
            DateTimePrecision::Week => {
                date.checked_add_signed(Duration::weeks(count))
                    .ok_or_else(|| FhirPathError::EvaluationError {
                        message: format!("Invalid date after adding {count} weeks"),
                    })?
            }
            DateTimePrecision::Day => {
                date.checked_add_signed(Duration::days(count))
                    .ok_or_else(|| FhirPathError::EvaluationError {
                        message: format!("Invalid date after adding {count} days"),
                    })?
            }
            _ => {
                return Err(FhirPathError::EvaluationError {
                    message: format!(
                        "Cannot add {precision} to date (time precision not supported for dates)"
                    ),
                });
            }
        };

        // Format result based on original precision
        let result_str = if date_str.len() == 4 {
            new_date.format("%Y").to_string()
        } else if date_str.len() == 7 {
            new_date.format("%Y-%m").to_string()
        } else {
            new_date.format("%Y-%m-%d").to_string()
        };

        Ok(FhirPathValue::Date(result_str))
    }

    /// Add a precision unit to a datetime
    fn add_precision_to_datetime(
        datetime_str: &str,
        precision: &DateTimePrecision,
        count: i64,
    ) -> FhirPathResult<FhirPathValue> {
        // Parse datetime - handle timezone
        let datetime = if let Some(base_str) = datetime_str.strip_suffix('Z') {
            NaiveDateTime::parse_from_str(base_str, "%Y-%m-%dT%H:%M:%S").map_err(|_| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid datetime format: {datetime_str}"),
                }
            })?
        } else if datetime_str.contains('+')
            || datetime_str.len() > 19 && datetime_str.chars().nth(19) == Some('-')
        {
            // Has timezone offset
            let base_str = &datetime_str[..19];
            NaiveDateTime::parse_from_str(base_str, "%Y-%m-%dT%H:%M:%S").map_err(|_| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid datetime format: {datetime_str}"),
                }
            })?
        } else {
            NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S").map_err(|_| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid datetime format: {datetime_str}"),
                }
            })?
        };

        let new_datetime = match precision {
            DateTimePrecision::Year => {
                if let Some(new_date) = datetime
                    .date()
                    .with_year((datetime.year() as i64 + count) as i32)
                {
                    new_date.and_time(datetime.time())
                } else {
                    return Err(FhirPathError::EvaluationError {
                        message: format!("Invalid datetime after adding {count} years"),
                    });
                }
            }
            DateTimePrecision::Month => {
                let total_months =
                    datetime.year() as i64 * 12 + datetime.month() as i64 - 1 + count;
                let new_year = (total_months / 12) as i32;
                let new_month = (total_months % 12 + 1) as u32;

                // Handle day overflow
                let mut new_day = datetime.day();
                loop {
                    if let Some(new_date) = NaiveDate::from_ymd_opt(new_year, new_month, new_day) {
                        break new_date.and_time(datetime.time());
                    }
                    new_day -= 1;
                    if new_day == 0 {
                        return Err(FhirPathError::EvaluationError {
                            message: format!("Invalid datetime after adding {count} months"),
                        });
                    }
                }
            }
            DateTimePrecision::Week => datetime
                .checked_add_signed(Duration::weeks(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} weeks"),
                })?,
            DateTimePrecision::Day => datetime
                .checked_add_signed(Duration::days(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} days"),
                })?,
            DateTimePrecision::Hour => datetime
                .checked_add_signed(Duration::hours(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} hours"),
                })?,
            DateTimePrecision::Minute => datetime
                .checked_add_signed(Duration::minutes(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} minutes"),
                })?,
            DateTimePrecision::Second => datetime
                .checked_add_signed(Duration::seconds(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} seconds"),
                })?,
            DateTimePrecision::Millisecond => datetime
                .checked_add_signed(Duration::milliseconds(count))
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("Invalid datetime after adding {count} milliseconds"),
                })?,
        };

        // Preserve timezone if present
        let result_str = if datetime_str.ends_with('Z') {
            format!("{}Z", new_datetime.format("%Y-%m-%dT%H:%M:%S"))
        } else if datetime_str.contains('+') {
            let tz_part = &datetime_str[datetime_str.find('+').unwrap()..];
            format!("{}{}", new_datetime.format("%Y-%m-%dT%H:%M:%S"), tz_part)
        } else if datetime_str.len() > 19 && datetime_str.chars().nth(19) == Some('-') {
            let tz_part = &datetime_str[19..];
            format!("{}{}", new_datetime.format("%Y-%m-%dT%H:%M:%S"), tz_part)
        } else {
            new_datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
        };

        Ok(FhirPathValue::DateTime(result_str))
    }

    /// Parse a precision unit from a string
    fn parse_precision_unit(unit_str: &str) -> Result<DateTimePrecision, FhirPathError> {
        match unit_str {
            "year" | "years" => Ok(DateTimePrecision::Year),
            "month" | "months" => Ok(DateTimePrecision::Month),
            "week" | "weeks" => Ok(DateTimePrecision::Week),
            "day" | "days" => Ok(DateTimePrecision::Day),
            "hour" | "hours" => Ok(DateTimePrecision::Hour),
            "minute" | "minutes" => Ok(DateTimePrecision::Minute),
            "second" | "seconds" => Ok(DateTimePrecision::Second),
            "millisecond" | "milliseconds" => Ok(DateTimePrecision::Millisecond),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Unknown precision unit: {unit_str}"),
            }),
        }
    }
}
