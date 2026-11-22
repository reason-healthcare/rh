//! Precision and boundary functions for decimal and temporal values.

use std::collections::HashMap;

use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::temporal::{parse_temporal, timezone_suffix, TemporalParts};
use crate::evaluator::types::FhirPathValue;
use rust_decimal::{Decimal, MathematicalOps, RoundingStrategy};

use super::FhirPathFunction;
use FhirPathValue::{Date, DateTime, Empty, Integer, Long, Number, Quantity, Time, TypedDateTime};

pub fn register_boundary_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    functions.insert(
        "precision".to_string(),
        Box::new(|target, params| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "precision() takes no parameters".to_string(),
                });
            }
            precision(target)
        }),
    );
    for (name, high) in [("lowBoundary", false), ("highBoundary", true)] {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| boundary(target, params, high)),
        );
    }
}

fn precision(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    let Some(target) = target
        .singleton("precision()")?
        .map(FhirPathValue::system_value)
    else {
        return Ok(Empty);
    };
    let digits = match target {
        Empty => return Ok(Empty),
        Number(n) => i64::from(n.scale()),
        Integer(_) | Long(_) => 0,
        Date(s) | DateTime(s) | Time(s) | TypedDateTime { value: s, .. } => {
            let text = s.strip_suffix(timezone_suffix(s)).unwrap_or(s);
            text.bytes().filter(|byte| byte.is_ascii_digit()).count() as i64
        }
        other => {
            return Err(FhirPathError::TypeError {
                message: format!("precision() is unsupported for {other:?}"),
            })
        }
    };
    Ok(Integer(digits))
}

fn boundary(
    target: &FhirPathValue,
    params: &[FhirPathValue],
    high: bool,
) -> FhirPathResult<FhirPathValue> {
    let name = if high {
        "highBoundary()"
    } else {
        "lowBoundary()"
    };
    let invalid = || FhirPathError::TypeError {
        message: format!("{name} accepts one optional Integer precision"),
    };
    let precision = match params {
        [] => None,
        [value] => match value.singleton(name)?.map(FhirPathValue::system_value) {
            Some(Integer(value)) => Some(*value),
            None | Some(Empty) => return Ok(Empty),
            _ => return Err(invalid()),
        },
        _ => return Err(invalid()),
    };
    let Some(target) = target.singleton(name)? else {
        return Ok(Empty);
    };
    if precision.is_some_and(|value| !(0..=28).contains(&value)) {
        return Ok(Empty);
    }
    let lexical_scale = decimal_lexical_scale(target);
    Ok(match target.system_value() {
        Empty => Empty,
        Number(n) => decimal_boundary(*n, lexical_scale, precision, high).map_or(Empty, Number),
        Integer(n) | Long(n) => decimal_boundary(Decimal::from(*n), lexical_scale, precision, high)
            .map_or(Empty, Number),
        Quantity { value, unit } => decimal_boundary(*value, lexical_scale, precision, high)
            .map_or(Empty, |value| Quantity {
                value,
                unit: unit.clone(),
            }),
        Date(text) => temporal_boundary(text, precision.unwrap_or(8), high, Kind::Date),
        DateTime(text) => temporal_boundary(text, precision.unwrap_or(17), high, Kind::DateTime),
        TypedDateTime { value: text, .. } => {
            temporal_boundary(text, precision.unwrap_or(17), high, Kind::FhirDateTime)
        }
        Time(text) => temporal_boundary(text, precision.unwrap_or(9), high, Kind::Time),
        other => {
            return Err(FhirPathError::TypeError {
                message: format!("{name} is unsupported for {other:?}"),
            })
        }
    })
}

fn decimal_lexical_scale(target: &FhirPathValue) -> Option<i64> {
    let FhirPathValue::Node(node) = target else {
        return None;
    };
    let number = node
        .raw
        .as_number()
        .or_else(|| node.raw.get("value")?.as_number())?;
    let text = number.to_string();
    let (mantissa, exponent) = text.split_once(['e', 'E'])?;
    let positions = mantissa
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    (positions as i64).checked_sub(exponent.parse::<i64>().ok()?)
}

fn decimal_boundary(
    value: Decimal,
    source_scale: Option<i64>,
    digits: Option<i64>,
    high: bool,
) -> Option<Decimal> {
    let digits = digits.unwrap_or(8) as u32;
    let scale = source_scale.unwrap_or_else(|| i64::from(value.scale()));
    let half = if scale < 0 {
        Decimal::from(10)
            .checked_powi(scale.checked_neg()?)?
            .checked_mul(Decimal::new(5, 1))
    } else {
        Decimal::try_new(5, u32::try_from(scale.checked_add(1)?).ok()?).ok()
    };
    let bound = match half {
        Some(half) if high => value.checked_add(half)?,
        Some(half) => value.checked_sub(half)?,
        None if scale >= i64::from(Decimal::MAX_SCALE) => value,
        None => return None,
    };
    let unit = Decimal::new(1, digits);
    let rounding = if bound.abs() < unit {
        // The suite truncates magnitudes smaller than one output unit.
        RoundingStrategy::ToZero
    } else if high {
        RoundingStrategy::ToPositiveInfinity
    } else {
        RoundingStrategy::ToNegativeInfinity
    };
    let mut rounded = bound.round_dp_with_strategy(digits, rounding);
    if !value.is_zero() && rounded == value && bound == value {
        // At the limit of Decimal precision the half-step can be lost when
        // applied to the value. Use the next representable output digit.
        rounded = if high {
            value.checked_add(unit)?
        } else {
            value.checked_sub(unit)?
        };
    }
    if rounded.is_zero() {
        rounded.set_sign_negative(!high && value.is_sign_negative());
    }
    rounded.rescale(digits);
    (rounded.scale() == digits).then_some(rounded)
}

enum Kind {
    Date,
    DateTime,
    FhirDateTime,
    Time,
}

fn temporal_boundary(text: &str, digits: i64, high: bool, kind: Kind) -> FhirPathValue {
    let boundary_digits = match kind {
        Kind::Date => [4, 6, 8].as_slice(),
        Kind::DateTime | Kind::FhirDateTime => [4, 6, 8, 10, 12, 14, 15, 16, 17].as_slice(),
        Kind::Time => [2, 4, 6, 7, 8, 9].as_slice(),
    };
    if !boundary_digits.contains(&digits) {
        return Empty;
    }
    match kind {
        Kind::Time => match parse_temporal(&format!("T{}", text.strip_prefix('T').unwrap_or(text)))
        {
            Some(parts) => Time(render_time(parts, digits, high)),
            None => Empty,
        },
        Kind::Date | Kind::DateTime | Kind::FhirDateTime => {
            let Some(parts) = parse_temporal(text) else {
                return Empty;
            };
            let Some(year) = parts.year else { return Empty };
            let mut date = format!("{year:04}");
            if digits >= 6 {
                let month = parts.month.unwrap_or(if high { 12 } else { 1 });
                date.push_str(&format!("-{month:02}"));
                if digits >= 8 {
                    let day = parts
                        .day
                        .unwrap_or(if high { last_day(year, month) } else { 1 });
                    date.push_str(&format!("-{day:02}"));
                }
            }
            if matches!(kind, Kind::Date) {
                Date(date)
            } else if digits <= 8 {
                DateTime(date)
            } else {
                let time = render_time(parts, digits - 8, high);
                let mut offset = timezone_suffix(text);
                // An unzoned FHIR dateTime contains only a date. When its
                // boundary adds time, span the range of FHIR timezone offsets.
                if offset.is_empty() && matches!(kind, Kind::FhirDateTime) {
                    offset = if high { "-12:00" } else { "+14:00" };
                }
                DateTime(format!("{date}T{time}{offset}"))
            }
        }
    }
}

fn last_day(year: i32, month: u8) -> u8 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

fn render_time(parts: TemporalParts, digits: i64, high: bool) -> String {
    let hour = parts.hour.unwrap_or(if high { 23 } else { 0 });
    let mut result = format!("{hour:02}");
    if digits >= 4 {
        let minute = parts.minute.unwrap_or(if high { 59 } else { 0 });
        result.push_str(&format!(":{minute:02}"));
    }
    if digits >= 6 {
        let second = parts.second.unwrap_or(if high { 59 } else { 0 });
        result.push_str(&format!(":{second:02}"));
    }
    if digits > 6 {
        let scale = (digits - 6) as usize;
        let fraction = parts.fraction_text().unwrap_or_default();
        let mut fraction = fraction.chars().take(scale).collect::<String>();
        let fill = if high { '9' } else { '0' };
        fraction.extend(std::iter::repeat_n(fill, scale - fraction.len()));
        result.push('.');
        result.push_str(&fraction);
    }
    result
}
