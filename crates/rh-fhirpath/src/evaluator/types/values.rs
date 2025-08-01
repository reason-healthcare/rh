//! FHIRPath value types and basic value operations

use crate::ast::DateTimePrecision;
use crate::error::*;
use serde_json::Value;

/// Result of evaluating a FHIRPath expression
#[derive(Debug, Clone, PartialEq)]
pub enum FhirPathValue {
    /// Boolean value
    Boolean(bool),
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Integer value
    Integer(i64),
    /// Long value (64-bit integer)
    Long(i64),
    /// Date value
    Date(String),
    /// DateTime value
    DateTime(String),
    /// Time value
    Time(String),
    /// Quantity value (number with unit)
    Quantity { value: f64, unit: Option<String> },
    /// DateTime precision unit value
    DateTimePrecision(DateTimePrecision),
    /// Collection of values
    Collection(Vec<FhirPathValue>),
    /// FHIR resource or object
    Object(Value),
    /// Null/empty value
    Empty,
}

impl FhirPathValue {
    /// Check if two values are equal (strict equality)
    pub fn equals_static(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        match (left, right) {
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => a == b,
            (FhirPathValue::String(a), FhirPathValue::String(b)) => a == b,
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => a == b,
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => a == b,
            (FhirPathValue::Long(a), FhirPathValue::Long(b)) => a == b,
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => a == b,
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => a == b,
            (FhirPathValue::Time(a), FhirPathValue::Time(b)) => a == b,
            (FhirPathValue::DateTimePrecision(a), FhirPathValue::DateTimePrecision(b)) => a == b,
            (FhirPathValue::Empty, FhirPathValue::Empty) => true,
            (FhirPathValue::Collection(a), FhirPathValue::Collection(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| Self::equals_static(x, y))
            }
            (FhirPathValue::Object(a), FhirPathValue::Object(b)) => a == b,
            (
                FhirPathValue::Quantity {
                    value: v1,
                    unit: u1,
                },
                FhirPathValue::Quantity {
                    value: v2,
                    unit: u2,
                },
            ) => v1 == v2 && u1 == u2,
            _ => false,
        }
    }

    /// Check if a value is truthy for boolean operations
    pub fn is_truthy(&self) -> bool {
        match self {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) => !items.is_empty(),
            _ => true, // Non-empty, non-boolean values are truthy
        }
    }

    /// Convert a value to boolean
    pub fn to_boolean(&self) -> bool {
        match self {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) => !items.is_empty(),
            _ => true,
        }
    }

    /// Convert a value to string for concatenation
    pub fn to_string_value(&self) -> FhirPathResult<String> {
        match self {
            FhirPathValue::String(s) => Ok(s.clone()),
            FhirPathValue::Integer(i) => Ok(i.to_string()),
            FhirPathValue::Number(n) => Ok(n.to_string()),
            FhirPathValue::Boolean(b) => Ok(b.to_string()),
            FhirPathValue::Date(d) => Ok(d.clone()),
            FhirPathValue::DateTime(dt) => Ok(dt.clone()),
            FhirPathValue::Time(t) => Ok(t.clone()),
            FhirPathValue::Empty => Ok("".to_string()),
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot convert {self:?} to string"),
            }),
        }
    }

    /// Convert JSON value to FHIRPath value
    pub fn from_json(value: &Value) -> FhirPathValue {
        match value {
            Value::Null => FhirPathValue::Empty,
            Value::Bool(b) => FhirPathValue::Boolean(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    FhirPathValue::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    FhirPathValue::Number(f)
                } else {
                    FhirPathValue::Number(0.0)
                }
            }
            Value::String(s) => FhirPathValue::String(s.clone()),
            Value::Array(arr) => {
                let items: Vec<FhirPathValue> = arr.iter().map(Self::from_json).collect();
                if items.is_empty() {
                    FhirPathValue::Empty
                } else {
                    // Always return a Collection for arrays, even single-element ones
                    // This preserves array semantics needed for indexing
                    FhirPathValue::Collection(items)
                }
            }
            Value::Object(_) => FhirPathValue::Object(value.clone()),
        }
    }

    /// Convert FHIRPath value to JSON value (for context)
    pub fn to_json(&self) -> Value {
        match self {
            FhirPathValue::Boolean(b) => Value::Bool(*b),
            FhirPathValue::String(s) => Value::String(s.clone()),
            FhirPathValue::Number(n) => Value::Number(
                serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
            FhirPathValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
            FhirPathValue::Long(l) => Value::Number(serde_json::Number::from(*l)),
            FhirPathValue::Date(s) => Value::String(s.clone()),
            FhirPathValue::DateTime(s) => Value::String(s.clone()),
            FhirPathValue::Time(s) => Value::String(s.clone()),
            FhirPathValue::Quantity { value, unit: _ } => Value::Number(
                serde_json::Number::from_f64(*value).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
            FhirPathValue::DateTimePrecision(_) => Value::Null,
            FhirPathValue::Collection(items) => {
                let json_items: Vec<Value> = items.iter().map(|item| item.to_json()).collect();
                Value::Array(json_items)
            }
            FhirPathValue::Object(obj) => obj.clone(),
            FhirPathValue::Empty => Value::Null,
        }
    }

    /// Compare two values for ordering (-1, 0, 1)
    pub fn compare_values(&self, other: &FhirPathValue) -> FhirPathResult<i32> {
        match (self, other) {
            // Numeric comparisons
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Long(a), FhirPathValue::Long(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Integer(a), FhirPathValue::Long(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Long(a), FhirPathValue::Integer(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if a < b {
                    Ok(-1)
                } else if a > b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                let a_f = *a as f64;
                if a_f < *b {
                    Ok(-1)
                } else if a_f > *b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (FhirPathValue::Long(a), FhirPathValue::Number(b)) => {
                let a_f = *a as f64;
                if a_f < *b {
                    Ok(-1)
                } else if a_f > *b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                let b_f = *b as f64;
                if *a < b_f {
                    Ok(-1)
                } else if *a > b_f {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Long(b)) => {
                let b_f = *b as f64;
                if *a < b_f {
                    Ok(-1)
                } else if *a > b_f {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }

            // String comparisons
            (FhirPathValue::String(a), FhirPathValue::String(b)) => Ok(a.cmp(b) as i32),

            // Boolean comparisons (false < true)
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => Ok(a.cmp(b) as i32),

            // Date/Time comparisons
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Time(a), FhirPathValue::Time(b)) => Ok(a.cmp(b) as i32),

            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot compare {self:?} and {other:?}"),
            }),
        }
    }

    /// Check if a value is a member of a collection
    pub fn is_member_of(&self, collection: &FhirPathValue) -> FhirPathResult<bool> {
        match collection {
            FhirPathValue::Collection(items) => {
                Ok(items.iter().any(|item| Self::equals_static(self, item)))
            }
            FhirPathValue::Empty => Ok(false),
            // Single values are treated as single-item collections
            other => Ok(Self::equals_static(self, other)),
        }
    }
}
