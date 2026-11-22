//! Mathematical functions for FHIRPath expressions.

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, MathematicalOps, RoundingStrategy};

use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::types::FhirPathValue;

use super::arithmetic::{integer_result, numeric_value};
use FhirPathValue::{Empty, Integer, Long, Number, Quantity};

pub struct MathEvaluator;

impl MathEvaluator {
    pub fn abs(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Ok(
            match target.singleton("abs()")?.map(FhirPathValue::system_value) {
                Some(Integer(value)) => value.checked_abs().map_or(Empty, integer_result),
                Some(Long(value)) => value.checked_abs().map_or(Empty, Long),
                Some(Number(value)) => Number(value.abs()),
                Some(Quantity { value, unit }) => Quantity {
                    value: value.abs(),
                    unit: unit.clone(),
                },
                None | Some(Empty) => Empty,
                _ => return Err(invalid_input("abs()")),
            },
        )
    }

    pub fn ceiling(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        integral_math(target, "ceiling()", |value| value.ceil())
    }

    pub fn exp(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let Some(value) = number(target, "exp()")? else {
            return Ok(Empty);
        };
        Ok(float_result(value.to_f64().map(f64::exp), false))
    }

    pub fn floor(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        integral_math(target, "floor()", |value| value.floor())
    }

    pub fn ln(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let Some(value) = number(target, "ln()")? else {
            return Ok(Empty);
        };
        if value <= Decimal::ZERO {
            return Err(FhirPathError::EvaluationError {
                message: "ln() requires a positive number".to_string(),
            });
        }
        Ok(float_result(
            value.to_f64().map(f64::ln),
            value == Decimal::ONE,
        ))
    }

    pub fn log(target: &FhirPathValue, base: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let (Some(target), Some(base)) = (number(target, "log()")?, number(base, "log()")?) else {
            return Ok(Empty);
        };
        if target <= Decimal::ZERO || base <= Decimal::ZERO || base == Decimal::ONE {
            return Err(FhirPathError::EvaluationError {
                message: "log() requires a positive value and a positive base different from 1"
                    .to_string(),
            });
        }
        let result = target
            .to_f64()
            .zip(base.to_f64())
            .map(|(target, base)| target.log(base));
        Ok(float_result(result, target == Decimal::ONE))
    }

    pub fn power(
        target: &FhirPathValue,
        exponent: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let (Some(value), Some(exponent)) =
            (number(target, "power()")?, number(exponent, "power()")?)
        else {
            return Ok(Empty);
        };
        if exponent.fract().is_zero() {
            if let Some(exponent) = exponent.to_i64().filter(|value| *value != i64::MIN) {
                return Ok(value
                    .checked_powi(exponent)
                    .filter(|result| !result.is_zero() || value.is_zero())
                    .map_or(Empty, Number));
            }
        }
        let result = value
            .to_f64()
            .zip(exponent.to_f64())
            .map(|(value, exponent)| value.powf(exponent));
        Ok(float_result(result, value.is_zero()))
    }

    pub fn round(
        target: &FhirPathValue,
        precision: Option<&FhirPathValue>,
    ) -> FhirPathResult<FhirPathValue> {
        let digits = match precision
            .map(|value| value.singleton("round()"))
            .transpose()?
            .flatten()
            .map(FhirPathValue::system_value)
        {
            None | Some(Empty) => 0,
            Some(Integer(value)) if *value >= 0 => u32::try_from(*value).unwrap_or(u32::MAX),
            _ => {
                return Err(FhirPathError::TypeError {
                    message: "round() precision must be a non-negative Integer".to_string(),
                })
            }
        };
        if digits > Decimal::MAX_SCALE {
            return Ok(Empty);
        }
        let round = |value: Decimal| {
            value.round_dp_with_strategy(digits, RoundingStrategy::MidpointAwayFromZero)
        };
        Ok(
            match target
                .singleton("round()")?
                .map(FhirPathValue::system_value)
            {
                Some(Quantity { value, unit }) => Quantity {
                    value: round(*value),
                    unit: unit.clone(),
                },
                Some(value) if !matches!(value, Empty) => {
                    let value = numeric_value(value).ok_or_else(|| invalid_input("round()"))?;
                    Number(round(value))
                }
                _ => Empty,
            },
        )
    }

    pub fn sqrt(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let result = number(target, "sqrt()")?.and_then(|value| value.sqrt());
        Ok(result.map_or(Empty, Number))
    }

    pub fn truncate(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        integral_math(target, "truncate()", |value| value.trunc())
    }
}

fn integral_math(
    target: &FhirPathValue,
    name: &str,
    operation: impl FnOnce(Decimal) -> Decimal,
) -> FhirPathResult<FhirPathValue> {
    Ok(
        match target.singleton(name)?.map(FhirPathValue::system_value) {
            Some(Integer(value) | Long(value)) => integer_result(*value),
            Some(Number(value)) => operation(*value).to_i64().map_or(Empty, integer_result),
            Some(Quantity { value, unit }) => Quantity {
                value: operation(*value),
                unit: unit.clone(),
            },
            None | Some(Empty) => Empty,
            _ => return Err(invalid_input(name)),
        },
    )
}

fn number(target: &FhirPathValue, name: &str) -> FhirPathResult<Option<Decimal>> {
    match target.singleton(name)?.map(FhirPathValue::system_value) {
        None | Some(Empty) => Ok(None),
        Some(value) => numeric_value(value)
            .map(Some)
            .ok_or_else(|| invalid_input(name)),
    }
}

fn float_result(value: Option<f64>, allow_zero: bool) -> FhirPathValue {
    value
        .and_then(Decimal::from_f64_retain)
        .filter(|value| allow_zero || !value.is_zero())
        .map_or(Empty, Number)
}

fn invalid_input(name: &str) -> FhirPathError {
    FhirPathError::TypeError {
        message: format!("{name} requires a numeric input"),
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
            FhirPathValue::Number(d("8"))
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
            FhirPathValue::Number(Decimal::from(5))
        );
    }
}
