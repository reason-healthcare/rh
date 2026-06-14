//! Conversion function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::temporal::parse_temporal;
use crate::evaluator::types::FhirPathValue;
use regex::Regex;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all conversion functions
pub fn register_conversion_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // toBoolean() function
    functions.insert(
        "toBoolean".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_boolean(target)),
    );

    // convertsToBoolean() function
    functions.insert(
        "convertsToBoolean".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_boolean(target)),
    );

    // toInteger() function
    functions.insert(
        "toInteger".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_integer(target)),
    );

    // convertsToInteger() function
    functions.insert(
        "convertsToInteger".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_integer(target)),
    );

    // toLong() function
    functions.insert(
        "toLong".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_long(target)),
    );

    // convertsToLong() function
    functions.insert(
        "convertsToLong".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_long(target)),
    );

    // toDate() function
    functions.insert(
        "toDate".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_date(target)),
    );

    // convertsToDate() function
    functions.insert(
        "convertsToDate".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_date(target)),
    );

    // toDateTime() function
    functions.insert(
        "toDateTime".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_datetime(target)),
    );

    // convertsToDateTime() function
    functions.insert(
        "convertsToDateTime".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_datetime(target)),
    );

    // toString() function
    functions.insert(
        "toString".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_string(target)),
    );

    // convertsToString() function
    functions.insert(
        "convertsToString".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_string(target)),
    );

    // toTime() function
    functions.insert(
        "toTime".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_time(target)),
    );

    // convertsToTime() function
    functions.insert(
        "convertsToTime".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_time(target)),
    );

    // toQuantity() function
    functions.insert(
        "toQuantity".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            to_quantity(target, params.first())
        }),
    );

    // convertsToQuantity() function
    functions.insert(
        "convertsToQuantity".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            converts_to_quantity(target, params.first())
        }),
    );

    // toDecimal() function
    functions.insert(
        "toDecimal".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| to_decimal(target)),
    );

    // convertsToDecimal() function
    functions.insert(
        "convertsToDecimal".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| converts_to_decimal(target)),
    );
}

/// Convert a value to Decimal according to the FHIRPath specification.
fn to_decimal(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_decimal(&items[0])
            } else {
                Ok(FhirPathValue::Empty)
            }
        }
        FhirPathValue::Number(n) => Ok(FhirPathValue::Number(*n)),
        FhirPathValue::Integer(i) | FhirPathValue::Long(i) => Ok(FhirPathValue::Number(*i as f64)),
        FhirPathValue::Boolean(b) => Ok(FhirPathValue::Number(if *b { 1.0 } else { 0.0 })),
        FhirPathValue::String(s) => {
            let decimal_re = Regex::new(r"^[+-]?\d+(\.\d+)?$").expect("static regex");
            if decimal_re.is_match(s.trim()) {
                match s.trim().parse::<f64>() {
                    Ok(n) => Ok(FhirPathValue::Number(n)),
                    Err(_) => Ok(FhirPathValue::Empty),
                }
            } else {
                Ok(FhirPathValue::Empty)
            }
        }
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check whether a value is convertible to Decimal.
fn converts_to_decimal(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),
        FhirPathValue::Collection(items) if items.len() != 1 => Ok(FhirPathValue::Empty),
        other => match to_decimal(other)? {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
            _ => Ok(FhirPathValue::Boolean(true)),
        },
    }
}

/// Convert a value to Boolean according to FHIRPath specification
fn to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_boolean(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Boolean(b) => Ok(FhirPathValue::Boolean(*b)),

        FhirPathValue::Integer(i) => match *i {
            1 => Ok(FhirPathValue::Boolean(true)),
            0 => Ok(FhirPathValue::Boolean(false)),
            _ => Ok(FhirPathValue::Empty), // Not convertible
        },

        FhirPathValue::Number(n) => {
            if (*n - 1.0).abs() < f64::EPSILON {
                Ok(FhirPathValue::Boolean(true))
            } else if (*n - 0.0).abs() < f64::EPSILON {
                Ok(FhirPathValue::Boolean(false))
            } else {
                Ok(FhirPathValue::Empty) // Not convertible
            }
        }

        FhirPathValue::String(s) => match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "1" | "1.0" => Ok(FhirPathValue::Boolean(true)),
            "false" | "f" | "no" | "n" | "0" | "0.0" => Ok(FhirPathValue::Boolean(false)),
            _ => Ok(FhirPathValue::Empty), // Not convertible
        },

        // All other types cannot be converted to Boolean
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Boolean according to FHIRPath specification
fn converts_to_boolean(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_boolean(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Integer(1 | 0) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible

        FhirPathValue::Number(n) => {
            if (*n - 1.0).abs() < f64::EPSILON || (*n - 0.0).abs() < f64::EPSILON {
                Ok(FhirPathValue::Boolean(true))
            } else {
                Ok(FhirPathValue::Boolean(false)) // Not convertible
            }
        }

        FhirPathValue::String(s) => match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "1" | "1.0" | "false" | "f" | "no" | "n" | "0" | "0.0" => {
                Ok(FhirPathValue::Boolean(true))
            }
            _ => Ok(FhirPathValue::Boolean(false)), // Not convertible
        },

        // All other types cannot be converted to Boolean
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to Integer according to FHIRPath specification
fn to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_integer(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Integer(i) => Ok(FhirPathValue::Integer(*i)),

        FhirPathValue::Boolean(b) => {
            if *b {
                Ok(FhirPathValue::Integer(1))
            } else {
                Ok(FhirPathValue::Integer(0))
            }
        }

        FhirPathValue::Number(n) => {
            // Check if it's a whole number
            if n.fract() == 0.0 && n.is_finite() {
                // Check if within integer range
                if *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                    Ok(FhirPathValue::Integer(*n as i64))
                } else {
                    Ok(FhirPathValue::Empty) // Out of range
                }
            } else {
                Ok(FhirPathValue::Empty) // Not a whole number
            }
        }

        FhirPathValue::String(s) => match s.trim().parse::<i64>() {
            Ok(i) => Ok(FhirPathValue::Integer(i)),
            Err(_) => Ok(FhirPathValue::Empty), // Not convertible
        },

        // All other types cannot be converted to Integer
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Integer according to FHIRPath specification
fn converts_to_integer(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_integer(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Number(n) => {
            // Check if it's a whole number within integer range
            if n.fract() == 0.0 && n.is_finite() && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                Ok(FhirPathValue::Boolean(true))
            } else {
                Ok(FhirPathValue::Boolean(false)) // Not convertible
            }
        }

        FhirPathValue::String(s) => match s.trim().parse::<i64>() {
            Ok(_) => Ok(FhirPathValue::Boolean(true)),
            Err(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible
        },

        // All other types cannot be converted to Integer
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to Long according to FHIRPath specification
fn to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_long(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Long(l) => Ok(FhirPathValue::Long(*l)),

        FhirPathValue::Integer(i) => Ok(FhirPathValue::Long(*i)),

        FhirPathValue::Boolean(b) => {
            if *b {
                Ok(FhirPathValue::Long(1))
            } else {
                Ok(FhirPathValue::Long(0))
            }
        }

        FhirPathValue::Number(n) => {
            // Check if it's a whole number
            if n.fract() == 0.0 && n.is_finite() {
                // Check if within i64 range
                if *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                    Ok(FhirPathValue::Long(*n as i64))
                } else {
                    Ok(FhirPathValue::Empty) // Out of range
                }
            } else {
                Ok(FhirPathValue::Empty) // Not a whole number
            }
        }

        FhirPathValue::String(s) => match s.trim().parse::<i64>() {
            Ok(l) => Ok(FhirPathValue::Long(l)),
            Err(_) => Ok(FhirPathValue::Empty), // Not convertible
        },

        // All other types cannot be converted to Long
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Long according to FHIRPath specification
fn converts_to_long(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_long(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Long(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Number(n) => {
            // Check if it's a whole number within i64 range
            if n.fract() == 0.0 && n.is_finite() && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                Ok(FhirPathValue::Boolean(true))
            } else {
                Ok(FhirPathValue::Boolean(false)) // Not convertible
            }
        }

        FhirPathValue::String(s) => match s.trim().parse::<i64>() {
            Ok(_) => Ok(FhirPathValue::Boolean(true)),
            Err(_) => Ok(FhirPathValue::Boolean(false)), // Not convertible
        },

        // All other types cannot be converted to Long
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to Date according to FHIRPath specification
fn to_date(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_date(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Date(d) => Ok(FhirPathValue::Date(d.clone())),

        FhirPathValue::DateTime(dt) => {
            // Extract date part from datetime
            if let Some(date_part) = dt.split('T').next() {
                Ok(FhirPathValue::Date(date_part.to_string()))
            } else {
                Ok(FhirPathValue::Empty) // Invalid datetime format
            }
        }

        FhirPathValue::String(s) => {
            // Try to parse as date (YYYY-MM-DD format)
            if is_valid_date_string(s) {
                Ok(FhirPathValue::Date(s.clone()))
            } else {
                // Try to extract date from datetime string
                if let Some(date_part) = s.split('T').next() {
                    if is_valid_date_string(date_part) {
                        Ok(FhirPathValue::Date(date_part.to_string()))
                    } else {
                        Ok(FhirPathValue::Empty) // Not convertible
                    }
                } else {
                    Ok(FhirPathValue::Empty) // Not convertible
                }
            }
        }

        // All other types cannot be converted to Date
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Date according to FHIRPath specification
fn converts_to_date(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_date(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::DateTime(dt) => {
            // Check if we can extract a valid date part
            if let Some(date_part) = dt.split('T').next() {
                Ok(FhirPathValue::Boolean(is_valid_date_string(date_part)))
            } else {
                Ok(FhirPathValue::Boolean(false)) // Invalid datetime format
            }
        }

        FhirPathValue::String(s) => {
            // Check if it's a valid date or can extract date from datetime
            if is_valid_date_string(s) {
                Ok(FhirPathValue::Boolean(true))
            } else if let Some(date_part) = s.split('T').next() {
                Ok(FhirPathValue::Boolean(is_valid_date_string(date_part)))
            } else {
                Ok(FhirPathValue::Boolean(false)) // Not convertible
            }
        }

        // All other types cannot be converted to Date
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to DateTime according to FHIRPath specification
fn to_datetime(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_datetime(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::DateTime(dt) => Ok(FhirPathValue::DateTime(dt.clone())),

        FhirPathValue::Date(d) => {
            // Convert date to datetime by adding midnight time
            Ok(FhirPathValue::DateTime(format!("{d}T00:00:00")))
        }

        FhirPathValue::String(s) => {
            // Try to parse as datetime
            if is_valid_datetime_string(s) {
                Ok(FhirPathValue::DateTime(s.clone()))
            } else if is_valid_date_string(s) {
                // Convert date string to datetime
                Ok(FhirPathValue::DateTime(format!("{s}T00:00:00")))
            } else {
                Ok(FhirPathValue::Empty) // Not convertible
            }
        }

        // All other types cannot be converted to DateTime
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to DateTime according to FHIRPath specification
fn converts_to_datetime(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_datetime(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::DateTime(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::String(s) => {
            // Check if it's a valid datetime or date that can be converted
            Ok(FhirPathValue::Boolean(
                is_valid_datetime_string(s) || is_valid_date_string(s),
            ))
        }

        // All other types cannot be converted to DateTime
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to String according to FHIRPath specification
fn to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_string(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::String(s) => Ok(FhirPathValue::String(s.clone())),

        FhirPathValue::Boolean(b) => Ok(FhirPathValue::String(if *b {
            "true".to_string()
        } else {
            "false".to_string()
        })),

        FhirPathValue::Integer(i) => Ok(FhirPathValue::String(i.to_string())),

        FhirPathValue::Long(l) => Ok(FhirPathValue::String(l.to_string())),

        FhirPathValue::Number(n) => {
            // FHIRPath Decimal.toString() preserves the decimal point so that
            // 1.0 → "1.0" — required to distinguish Decimal from Integer in
            // string form. Trailing zeros past the first are still trimmed.
            let mut s = format!("{n}");
            if !s.contains('.') {
                s.push_str(".0");
            } else {
                s = s.trim_end_matches('0').to_string();
                if s.ends_with('.') {
                    s.push('0');
                }
            }
            Ok(FhirPathValue::String(s))
        }

        FhirPathValue::Date(d) => Ok(FhirPathValue::String(d.clone())),

        FhirPathValue::DateTime(dt) => Ok(FhirPathValue::String(dt.clone())),

        FhirPathValue::Time(t) => {
            // Remove the leading 'T' from time format for string conversion
            let time_str = if let Some(stripped) = t.strip_prefix('T') {
                stripped.to_string()
            } else {
                t.clone()
            };
            Ok(FhirPathValue::String(time_str))
        }

        FhirPathValue::Quantity { value, unit } => match unit {
            // Calendar-duration words are emitted unquoted (`1 week`),
            // while UCUM codes are quoted (`1 'kg'`). The set tracks the
            // FHIRPath calendar-duration vocabulary plus their plurals.
            Some(u) if is_calendar_duration_unit(u) => {
                Ok(FhirPathValue::String(format!("{value} {u}")))
            }
            Some(u) => Ok(FhirPathValue::String(format!("{value} '{u}'"))),
            None => Ok(FhirPathValue::String(value.to_string())),
        },

        // All other types cannot be converted to String (return empty)
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to String according to FHIRPath specification
fn converts_to_string(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_string(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks - most types can be converted to string
        FhirPathValue::String(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Boolean(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Integer(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Long(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Number(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Date(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::DateTime(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Time(_) => Ok(FhirPathValue::Boolean(true)),
        FhirPathValue::Quantity { .. } => Ok(FhirPathValue::Boolean(true)),

        // Complex types (Object, DateTimePrecision) cannot be converted to string
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to Time according to FHIRPath specification
fn to_time(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_time(&items[0])
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Time(t) => Ok(FhirPathValue::Time(t.clone())),

        FhirPathValue::String(s) => {
            // Validate time format: HH:MM:SS or HH:MM:SS.sss
            if is_valid_time_string(s) {
                // Add leading 'T' for internal representation
                let time_with_t = if s.starts_with('T') {
                    s.clone()
                } else {
                    format!("T{s}")
                };
                Ok(FhirPathValue::Time(time_with_t))
            } else {
                Ok(FhirPathValue::Empty)
            }
        }

        FhirPathValue::DateTime(dt) => {
            // Extract time part from datetime
            if let Some(time_part) = extract_time_from_datetime(dt) {
                Ok(FhirPathValue::Time(format!("T{time_part}")))
            } else {
                Ok(FhirPathValue::Empty)
            }
        }

        // All other types cannot be converted to Time (return empty)
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Time according to FHIRPath specification
fn converts_to_time(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_time(&items[0])
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Time(_) => Ok(FhirPathValue::Boolean(true)),

        FhirPathValue::String(s) => Ok(FhirPathValue::Boolean(is_valid_time_string(s))),

        FhirPathValue::DateTime(dt) => Ok(FhirPathValue::Boolean(
            extract_time_from_datetime(dt).is_some(),
        )),

        // All other types cannot be converted to Time
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Convert a value to Quantity according to FHIRPath specification
fn to_quantity(
    value: &FhirPathValue,
    unit_param: Option<&FhirPathValue>,
) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                to_quantity(&items[0], unit_param)
            } else {
                // Multiple items - return empty per spec
                Ok(FhirPathValue::Empty)
            }
        }

        // Single item conversions
        FhirPathValue::Quantity { value, unit } => {
            // If unit parameter is provided, use it; otherwise keep existing unit
            let target_unit = if let Some(unit_param) = unit_param {
                match unit_param {
                    FhirPathValue::String(u) => Some(u.clone()),
                    _ => unit.clone(), // Invalid unit parameter, keep existing
                }
            } else {
                unit.clone()
            };

            Ok(FhirPathValue::Quantity {
                value: *value,
                unit: target_unit,
            })
        }

        // Numeric and Boolean conversions get the default UCUM unit "1"
        // when no explicit unit is supplied — per FHIRPath §6.3 toQuantity().
        FhirPathValue::Integer(i) => Ok(FhirPathValue::Quantity {
            value: *i as f64,
            unit: Some(quantity_unit_or_default(unit_param)),
        }),

        FhirPathValue::Number(n) => Ok(FhirPathValue::Quantity {
            value: *n,
            unit: Some(quantity_unit_or_default(unit_param)),
        }),

        FhirPathValue::Long(l) => Ok(FhirPathValue::Quantity {
            value: *l as f64,
            unit: Some(quantity_unit_or_default(unit_param)),
        }),

        FhirPathValue::Boolean(b) => Ok(FhirPathValue::Quantity {
            value: if *b { 1.0 } else { 0.0 },
            unit: Some(quantity_unit_or_default(unit_param)),
        }),

        FhirPathValue::String(s) => {
            // Try to parse as a quantity string or as a number
            if let Some((value, unit)) = parse_quantity_string(s) {
                // If unit parameter is provided, use it; otherwise use parsed unit
                let target_unit = if let Some(unit_param) = unit_param {
                    match unit_param {
                        FhirPathValue::String(u) => Some(u.clone()),
                        _ => unit, // Invalid unit parameter, use parsed unit
                    }
                } else {
                    unit
                };

                Ok(FhirPathValue::Quantity {
                    value,
                    unit: target_unit,
                })
            } else if let Ok(value) = s.parse::<f64>() {
                // Plain numeric string — defaults to UCUM unit "1" like the
                // direct numeric-input path does.
                Ok(FhirPathValue::Quantity {
                    value,
                    unit: Some(quantity_unit_or_default(unit_param)),
                })
            } else {
                // Not convertible
                Ok(FhirPathValue::Empty)
            }
        }

        // All other types cannot be converted to Quantity
        _ => Ok(FhirPathValue::Empty),
    }
}

/// Check if a value can be converted to Quantity according to FHIRPath specification
fn converts_to_quantity(
    value: &FhirPathValue,
    _unit_param: Option<&FhirPathValue>,
) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return false
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),

        // If the collection contains more than one item, return false
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                converts_to_quantity(&items[0], _unit_param)
            } else {
                // Multiple items - cannot convert
                Ok(FhirPathValue::Boolean(false))
            }
        }

        // Single item checks
        FhirPathValue::Quantity { .. } => {
            // Always convertible (identity or unit change)
            Ok(FhirPathValue::Boolean(true))
        }

        FhirPathValue::Integer(_)
        | FhirPathValue::Number(_)
        | FhirPathValue::Long(_)
        | FhirPathValue::Boolean(_) => {
            // Numeric and Boolean inputs convert via the default unit "1".
            Ok(FhirPathValue::Boolean(true))
        }

        FhirPathValue::String(s) => {
            // Check if string can be parsed as quantity or number
            let is_quantity = parse_quantity_string(s).is_some();
            let is_number = s.parse::<f64>().is_ok();
            Ok(FhirPathValue::Boolean(is_quantity || is_number))
        }

        // All other types cannot be converted to Quantity
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

/// Helper function to validate date string format (YYYY-MM-DD)
fn is_valid_date_string(s: &str) -> bool {
    // Per FHIRPath spec, Date accepts YYYY, YYYY-MM, or YYYY-MM-DD. Delegates
    // to the temporal parser to cover all three precisions plus value-range
    // validation (month 1-12, day 1-31).
    let Some(parts) = parse_temporal(s) else {
        return false;
    };
    // Must not be a Time-only or DateTime-with-time string.
    if s.contains('T') || parts.hour.is_some() || parts.year.is_none() {
        return false;
    }
    if let Some(m) = parts.month {
        if !(1..=12).contains(&m) {
            return false;
        }
    }
    if let Some(d) = parts.day {
        if !(1..=31).contains(&d) {
            return false;
        }
    }
    true
}

/// Per FHIRPath spec, DateTime accepts partial-precision shapes (year alone,
/// year-month, full date, and the date plus any prefix of `T[hh[:mm[:ss[.fff]]][TZ]]`).
fn is_valid_datetime_string(s: &str) -> bool {
    let Some(parts) = parse_temporal(s) else {
        return false;
    };
    if parts.year.is_none() {
        return false;
    }
    if let Some(m) = parts.month {
        if !(1..=12).contains(&m) {
            return false;
        }
    }
    if let Some(d) = parts.day {
        if !(1..=31).contains(&d) {
            return false;
        }
    }
    if let Some(h) = parts.hour {
        if h > 23 {
            return false;
        }
    }
    if let Some(m) = parts.minute {
        if m > 59 {
            return false;
        }
    }
    if let Some(sec) = parts.second {
        if sec > 59 {
            return false;
        }
    }
    true
}

/// Per FHIRPath spec, Time accepts hh, hh:mm, hh:mm:ss, or hh:mm:ss.fff.
/// Storage form has a leading T; user-supplied strings often do not, so we
/// normalise before delegating to the temporal parser.
fn is_valid_time_string(s: &str) -> bool {
    let with_t = if s.starts_with('T') {
        s.to_string()
    } else {
        format!("T{s}")
    };
    let Some(parts) = parse_temporal(&with_t) else {
        return false;
    };
    if parts.hour.is_none() {
        return false;
    }
    if let Some(h) = parts.hour {
        if h > 23 {
            return false;
        }
    }
    if let Some(m) = parts.minute {
        if m > 59 {
            return false;
        }
    }
    if let Some(sec) = parts.second {
        if sec > 59 {
            return false;
        }
    }
    true
}

/// Extract time part from datetime string
fn extract_time_from_datetime(dt: &str) -> Option<String> {
    // Look for 'T' separator in datetime
    if let Some(t_pos) = dt.find('T') {
        let time_part = &dt[t_pos + 1..];

        // Remove timezone information if present
        let time_clean = if let Some(tz_pos) = time_part.find('+').or_else(|| time_part.find('-')) {
            &time_part[..tz_pos]
        } else if let Some(z_pos) = time_part.find('Z') {
            &time_part[..z_pos]
        } else {
            time_part
        };

        // Validate the extracted time
        if is_valid_time_string(time_clean) {
            Some(time_clean.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Parse a string that might represent a quantity (e.g., "5 'mg'", "10.5'kg'")
/// Returns (value, unit) if successful
fn parse_quantity_string(s: &str) -> Option<(f64, Option<String>)> {
    let s = s.trim();

    // Quoted UCUM unit form: "value 'unit'" or "value'unit'"
    if let Some(quote_start) = s.find('\'') {
        if let Some(quote_end) = s.rfind('\'') {
            if quote_start < quote_end {
                let value_str = s[..quote_start].trim();
                let unit_str = &s[quote_start + 1..quote_end];
                if let Ok(value) = value_str.parse::<f64>() {
                    let unit = if unit_str.is_empty() {
                        None
                    } else {
                        Some(unit_str.to_string())
                    };
                    return Some((value, unit));
                }
            }
        }
    }

    // Unquoted calendar-duration form: "value day" / "value weeks" etc.
    if let Some(sep) = s.find(char::is_whitespace) {
        let value_str = s[..sep].trim();
        let unit_str = s[sep..].trim();
        if !unit_str.is_empty() && is_calendar_duration_unit(unit_str) {
            if let Ok(value) = value_str.parse::<f64>() {
                return Some((value, Some(unit_str.to_string())));
            }
        }
    }

    None
}

/// Read the unit-argument for `toQuantity(unit)`, defaulting to the UCUM
/// "no unit" code `'1'` when no argument is supplied — per FHIRPath §6.3.
fn quantity_unit_or_default(unit_param: Option<&FhirPathValue>) -> String {
    match unit_param {
        Some(FhirPathValue::String(u)) => u.clone(),
        _ => "1".to_string(),
    }
}

/// FHIRPath calendar-duration vocabulary (used by Quantity.toString to decide
/// whether the unit gets quoted). Both singular and plural forms accepted.
fn is_calendar_duration_unit(u: &str) -> bool {
    matches!(
        u,
        "year"
            | "years"
            | "month"
            | "months"
            | "week"
            | "weeks"
            | "day"
            | "days"
            | "hour"
            | "hours"
            | "minute"
            | "minutes"
            | "second"
            | "seconds"
            | "millisecond"
            | "milliseconds"
    )
}
