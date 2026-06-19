//! Arithmetic operations for FHIRPath values

use crate::ast::*;
use crate::error::*;
use crate::evaluator::operations::units::UnitConverter;
use crate::evaluator::types::FhirPathValue;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

/// Round the result of decimal arithmetic to the number of fractional digits
/// of the less-precise input. This compensates for rounding artefacts when
/// both inputs have a small, well-defined precision (e.g. `1.8 - 1.2 → 0.6`).
fn round_to_decimal_precision(result: Decimal, a: Decimal, b: Decimal) -> Decimal {
    let prec = decimal_places(a).min(decimal_places(b));
    if prec == 0 {
        return result;
    }
    let scale = Decimal::from_str_exact("10").unwrap().powi(prec as i64);
    (result * scale).round() / scale
}

fn decimal_places(n: Decimal) -> u8 {
    n.scale() as u8
}

fn is_empty_value(value: &FhirPathValue) -> bool {
    match value {
        FhirPathValue::Empty => true,
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
            items.is_empty()
        }
        _ => false,
    }
}

fn integer_result(value: i64) -> FhirPathValue {
    if value < i32::MIN as i64 || value > i32::MAX as i64 {
        FhirPathValue::Empty
    } else {
        FhirPathValue::Integer(value)
    }
}

fn duration_millis(precision: &DateTimePrecision, count: Decimal) -> FhirPathResult<i64> {
    let multiplier = match precision {
        DateTimePrecision::Week => Decimal::from(604_800_000_i64),
        DateTimePrecision::Day => Decimal::from(86_400_000_i64),
        DateTimePrecision::Hour => Decimal::from(3_600_000_i64),
        DateTimePrecision::Minute => Decimal::from(60_000_i64),
        DateTimePrecision::Second => Decimal::from(1_000_i64),
        DateTimePrecision::Millisecond => Decimal::ONE,
        DateTimePrecision::Year | DateTimePrecision::Month => {
            return Err(FhirPathError::EvaluationError {
                message: format!("Cannot use {precision} precision in time arithmetic"),
            })
        }
    };
    (count * multiplier)
        .round()
        .to_i64()
        .ok_or_else(|| FhirPathError::EvaluationError {
            message: format!("Duration is out of range: {count} {precision}"),
        })
}

/// Split a datetime literal into its `(base, tz_suffix)` components.
///
/// Recognises `Z`, `+HH:MM`, and `-HH:MM` suffixes; everything else is treated
/// as the base. Used to preserve the original timezone marker through datetime
/// arithmetic without losing sub-second precision.
fn split_datetime_tz(s: &str) -> (&str, &str) {
    if let Some(base) = s.strip_suffix('Z') {
        return (base, "Z");
    }
    if s.len() >= 6 {
        let tail = &s[s.len() - 6..];
        let head = tail.as_bytes()[0];
        if (head == b'+' || head == b'-') && tail.as_bytes()[3] == b':' {
            return (&s[..s.len() - 6], tail);
        }
    }
    (s, "")
}

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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => Ok(a
                .checked_add(*b)
                .map(integer_result)
                .unwrap_or(FhirPathValue::Empty)),
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a + b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(Decimal::from(*a) + b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a + Decimal::from(*b)))
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
                    Ok(FhirPathValue::Quantity {
                        value: value + Decimal::from(*i),
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
            (FhirPathValue::Time(time_str), FhirPathValue::DateTimePrecision(precision)) => {
                Self::add_precision_to_time(time_str, precision, Decimal::ONE)
            }
            (
                FhirPathValue::Time(time_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                let precision = Self::parse_precision_unit(unit_str)?;
                Self::add_precision_to_time(time_str, &precision, *value)
            }
            // Date/DateTime arithmetic with compound duration quantities (6 months, etc.)
            (
                FhirPathValue::Date(date_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                // Per FHIRPath spec, UCUM `mo` and `a` (mean month/year — 30.44 and
                // 365.25 days) are ambiguous for calendar Date arithmetic and must
                // produce an execution error.
                if matches!(unit_str.as_str(), "mo" | "a") {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "UCUM duration '{unit_str}' is ambiguous for Date arithmetic"
                        ),
                    })
                } else if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(date_str, &precision, value.to_i64().unwrap_or(0))
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
                // Per FHIRPath spec, UCUM `mo` and `a` (mean month/year — 30.44 and
                // 365.25 days) are ambiguous for calendar Date arithmetic and must
                // produce an execution error.
                if matches!(unit_str.as_str(), "mo" | "a") {
                    Err(FhirPathError::EvaluationError {
                        message: format!(
                            "UCUM duration '{unit_str}' is ambiguous for Date arithmetic"
                        ),
                    })
                } else if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(date_str, &precision, value.to_i64().unwrap_or(0))
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
                    Self::add_decimal_precision_to_datetime(datetime_str, &precision, *value)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add quantity with unit '{unit_str}' to datetime"),
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
                    Self::add_decimal_precision_to_datetime(datetime_str, &precision, *value)
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: format!("Cannot add quantity with unit '{unit_str}' to datetime"),
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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => Ok(a
                .checked_sub(*b)
                .map(integer_result)
                .unwrap_or(FhirPathValue::Empty)),
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => Ok(FhirPathValue::Number(
                round_to_decimal_precision(a - b, *a, *b),
            )),
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(Decimal::from(*a) - b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a - Decimal::from(*b)))
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
                        value: value - Decimal::from(*i),
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
                        value: Decimal::from(*i) - value,
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
            (FhirPathValue::Time(time_str), FhirPathValue::DateTimePrecision(precision)) => {
                Self::add_precision_to_time(time_str, precision, Decimal::NEGATIVE_ONE)
            }
            (
                FhirPathValue::Time(time_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                let precision = Self::parse_precision_unit(unit_str)?;
                Self::add_precision_to_time(time_str, &precision, -*value)
            }
            // Date/DateTime arithmetic with compound duration quantities (subtraction)
            (
                FhirPathValue::Date(date_str),
                FhirPathValue::Quantity {
                    value,
                    unit: Some(unit_str),
                },
            ) => {
                if let Ok(precision) = Self::parse_precision_unit(unit_str) {
                    Self::add_precision_to_date(
                        date_str,
                        &precision,
                        -(value.to_i64().unwrap_or(0)),
                    )
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
                    Self::add_decimal_precision_to_datetime(datetime_str, &precision, -*value)
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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => Ok(a
                .checked_mul(*b)
                .map(integer_result)
                .unwrap_or(FhirPathValue::Empty)),
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a * b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(Decimal::from(*a) * b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a * Decimal::from(*b)))
            }
            // Quantity multiplication by scalars
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n))
            | (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                let converter = UnitConverter::new();
                Ok(converter.multiply_by_scalar(*value, unit, *n))
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i))
            | (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                let converter = UnitConverter::new();
                Ok(converter.multiply_by_scalar(*value, unit, Decimal::from(*i)))
            }
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
                converter.multiply_quantities(*a, unit_a, *b, unit_b)
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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    // Division always returns a Number (decimal) in FHIRPath
                    Ok(FhirPathValue::Number(Decimal::from(*a) / Decimal::from(*b)))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(a / b))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(Decimal::from(*a) / b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(a / Decimal::from(*b)))
                }
            }
            // Quantity division by scalars
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Number(n)) => {
                let converter = UnitConverter::new();
                converter.divide_by_scalar(*value, unit, *n)
            }
            (FhirPathValue::Quantity { value, unit }, FhirPathValue::Integer(i)) => {
                let converter = UnitConverter::new();
                converter.divide_by_scalar(*value, unit, Decimal::from(*i))
            }
            // Number/Integer divided by Quantity (result is different unit)
            (FhirPathValue::Number(n), FhirPathValue::Quantity { value, unit }) => {
                if *value == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else if unit.is_none() {
                    Ok(FhirPathValue::Number(n / value))
                } else {
                    Err(FhirPathError::EvaluationError {
                        message: "Division of number by quantity with units is not supported"
                            .to_string(),
                    })
                }
            }
            (FhirPathValue::Integer(i), FhirPathValue::Quantity { value, unit }) => {
                if *value == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else if unit.is_none() {
                    Ok(FhirPathValue::Number(Decimal::from(*i) / value))
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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Integer(a / b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Integer(
                        (a / b).trunc().to_i64().unwrap_or(0),
                    ))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Integer(
                        (Decimal::from(*a) / b).trunc().to_i64().unwrap_or(0),
                    ))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Integer(
                        (a / Decimal::from(*b)).trunc().to_i64().unwrap_or(0),
                    ))
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
            _ if is_empty_value(left) || is_empty_value(right) => Ok(FhirPathValue::Empty),
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Integer(a % b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(round_to_decimal_precision(
                        a % b,
                        *a,
                        *b,
                    )))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == Decimal::ZERO {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(Decimal::from(*a) % b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Number(a % Decimal::from(*b)))
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
            FhirPathValue::Integer(n) => Ok(n
                .checked_neg()
                .map(integer_result)
                .unwrap_or(FhirPathValue::Empty)),
            FhirPathValue::Long(n) => Ok(FhirPathValue::Long(-n)),
            FhirPathValue::Number(n) => Ok(FhirPathValue::Number(-n)),
            FhirPathValue::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: -value,
                unit: unit.clone(),
            }),
            _ if is_empty_value(value) => Ok(FhirPathValue::Empty),
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
        Self::add_decimal_precision_to_datetime(datetime_str, precision, Decimal::from(count))
    }

    fn add_decimal_precision_to_datetime(
        datetime_str: &str,
        precision: &DateTimePrecision,
        count: Decimal,
    ) -> FhirPathResult<FhirPathValue> {
        // Split off the timezone marker (Z, +HH:MM, or -HH:MM) so the base
        // string starts with `YYYY-MM-DDTHH:MM:SS[.fff]` and nothing more.
        let (base_str, tz_str) = split_datetime_tz(datetime_str);
        let has_fraction = base_str
            .find('T')
            .is_some_and(|t| base_str[t..].contains('.'));

        // %.f accepts any number of fractional digits; if absent, we fall back
        // to second-precision parse. Either way the input is fully consumed.
        let datetime = NaiveDateTime::parse_from_str(base_str, "%Y-%m-%dT%H:%M:%S%.f")
            .or_else(|_| NaiveDateTime::parse_from_str(base_str, "%Y-%m-%dT%H:%M:%S"))
            .map_err(|_| FhirPathError::EvaluationError {
                message: format!("Invalid datetime format: {datetime_str}"),
            })?;

        let new_datetime = match precision {
            DateTimePrecision::Year => {
                let count = count.to_i64().unwrap_or(0);
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
                let count = count.to_i64().unwrap_or(0);
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
            DateTimePrecision::Week
            | DateTimePrecision::Day
            | DateTimePrecision::Hour
            | DateTimePrecision::Minute
            | DateTimePrecision::Second
            | DateTimePrecision::Millisecond => {
                let millis = duration_millis(precision, count)?;
                datetime
                    .checked_add_signed(Duration::milliseconds(millis))
                    .ok_or_else(|| FhirPathError::EvaluationError {
                        message: format!("Invalid datetime after adding {count} {precision}"),
                    })?
            }
        };

        // Preserve original sub-second precision (3-digit millisecond format
        // when input had any fractional component) and the timezone marker.
        let body = if has_fraction {
            new_datetime.format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
        } else {
            new_datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
        };
        let result_str = format!("{body}{tz_str}");
        Ok(FhirPathValue::DateTime(result_str))
    }

    fn add_precision_to_time(
        time_str: &str,
        precision: &DateTimePrecision,
        count: Decimal,
    ) -> FhirPathResult<FhirPathValue> {
        let raw = time_str.strip_prefix('T').unwrap_or(time_str);
        let has_fraction = raw.contains('.');
        let time = NaiveTime::parse_from_str(raw, "%H:%M:%S%.f")
            .or_else(|_| NaiveTime::parse_from_str(raw, "%H:%M:%S"))
            .map_err(|_| FhirPathError::EvaluationError {
                message: format!("Invalid time format: {time_str}"),
            })?;

        let millis = duration_millis(precision, count)?;
        let day_millis = 86_400_000_i64;
        let current = i64::from(time.num_seconds_from_midnight()) * 1000
            + i64::from(time.nanosecond() / 1_000_000);
        let wrapped = (current + millis).rem_euclid(day_millis);
        let secs = (wrapped / 1000) as u32;
        let nanos = ((wrapped % 1000) as u32) * 1_000_000;
        let result =
            NaiveTime::from_num_seconds_from_midnight_opt(secs, nanos).ok_or_else(|| {
                FhirPathError::EvaluationError {
                    message: format!("Invalid time after adding {count} {precision}"),
                }
            })?;

        let result_str = if has_fraction || nanos != 0 {
            result.format("%H:%M:%S%.3f").to_string()
        } else {
            result.format("%H:%M:%S").to_string()
        };
        Ok(FhirPathValue::Time(result_str))
    }

    /// Parse a precision unit from a string. Accepts both FHIRPath calendar-
    /// duration names (`year`, `months`, etc.) and the UCUM abbreviations
    /// used in quoted-unit quantity literals (`a`, `mo`, `wk`, `d`, `h`,
    /// `min`, `s`, `ms`).
    fn parse_precision_unit(unit_str: &str) -> Result<DateTimePrecision, FhirPathError> {
        match unit_str {
            "year" | "years" | "a" => Ok(DateTimePrecision::Year),
            "month" | "months" | "mo" => Ok(DateTimePrecision::Month),
            "week" | "weeks" | "wk" => Ok(DateTimePrecision::Week),
            "day" | "days" | "d" => Ok(DateTimePrecision::Day),
            "hour" | "hours" | "h" => Ok(DateTimePrecision::Hour),
            "minute" | "minutes" | "min" => Ok(DateTimePrecision::Minute),
            "second" | "seconds" | "s" => Ok(DateTimePrecision::Second),
            "millisecond" | "milliseconds" | "ms" => Ok(DateTimePrecision::Millisecond),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Unknown precision unit: {unit_str}"),
            }),
        }
    }
}
