//! FHIRPath value types and basic value operations

use crate::ast::DateTimePrecision;
use crate::error::*;
use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum FhirPathValue {
    Boolean(bool),
    String(String),
    /// A string value with FHIR primitive type provenance (code, id, uri, etc.)
    TypedString {
        value: String,
        fhir_type: FhirPrimitiveType,
    },
    /// A boolean value with FHIR primitive type provenance.
    TypedBoolean {
        value: bool,
        fhir_type: FhirPrimitiveType,
    },
    /// Decimal value preserving trailing-zero precision (FHIRPath spec requires
    /// `1.58700.precision() = 5`, which `f64` cannot represent).
    Number(Decimal),
    Integer(i64),
    Long(i64),
    Date(String),
    DateTime(String),
    /// A datetime-like value with FHIR primitive type provenance (dateTime, instant).
    TypedDateTime {
        value: String,
        fhir_type: FhirPrimitiveType,
    },
    Time(String),
    Quantity {
        value: Decimal,
        unit: Option<String>,
    },
    DateTimePrecision(DateTimePrecision),
    Collection(Vec<FhirPathValue>),
    /// A collection whose element order is not defined (e.g. from `children()`).
    /// Order-sensitive functions (`skip`, `tail`, `first`, `last`, `orderBy`,
    /// `reverse`) must not be applied in strict mode with
    /// `checkOrderedFunctions=true`.  In lenient mode, it behaves identically
    /// to `Collection`.
    UnorderedCollection(Vec<FhirPathValue>),
    Object(Value),
    /// A FHIR complex object carrying its declared FHIR type name (e.g. "HumanName").
    TypedObject {
        value: Value,
        fhir_type: String,
    },
    /// A FHIR primitive value that also carries the `_fieldName` extension data
    /// (extensions array from the companion `_fieldName` sibling in FHIR JSON).
    FhirPrimitive {
        inner: Box<FhirPathValue>,
        extensions: serde_json::Map<String, Value>,
    },
    Empty,
}

impl FhirPathValue {
    fn canonical_time(value: &str) -> &str {
        value.strip_prefix('T').unwrap_or(value)
    }

    /// Returns `true` if this is an `UnorderedCollection`.
    pub fn is_unordered(&self) -> bool {
        matches!(self, FhirPathValue::UnorderedCollection(_))
    }

    /// Normalize `UnorderedCollection` → `Collection`; all other variants
    /// pass through unchanged.
    pub fn normalize(self) -> Self {
        match self {
            FhirPathValue::UnorderedCollection(items) => FhirPathValue::Collection(items),
            other => other,
        }
    }

    /// Get the string content if this is a String or TypedString value
    pub fn as_str(&self) -> Option<&str> {
        match self {
            FhirPathValue::String(s) => Some(s),
            FhirPathValue::TypedString { value, .. } => Some(value),
            _ => None,
        }
    }

    /// Get the FHIR primitive type provenance, if any
    pub fn fhir_primitive_type(&self) -> Option<FhirPrimitiveType> {
        match self {
            FhirPathValue::TypedString { fhir_type, .. } => Some(*fhir_type),
            FhirPathValue::TypedBoolean { fhir_type, .. } => Some(*fhir_type),
            FhirPathValue::Integer(_) => Some(FhirPrimitiveType::Integer),
            FhirPathValue::Number(_) => Some(FhirPrimitiveType::Decimal),
            FhirPathValue::Date(_) => Some(FhirPrimitiveType::Date),
            FhirPathValue::TypedDateTime { fhir_type, .. } => Some(*fhir_type),
            FhirPathValue::Time(_) => Some(FhirPrimitiveType::Time),
            FhirPathValue::Quantity { .. } => Some(FhirPrimitiveType::Decimal),
            FhirPathValue::Object(_) => None,
            _ => None,
        }
    }

    /// Check if this is a string-like value (String or TypedString)
    pub fn is_string_like(&self) -> bool {
        matches!(
            self,
            FhirPathValue::String(_) | FhirPathValue::TypedString { .. }
        )
    }

    /// Check if two values are equal (strict equality)
    pub fn equals_static(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        match (left, right) {
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => a == b,
            (FhirPathValue::Boolean(a), FhirPathValue::TypedBoolean { value: b, .. })
            | (FhirPathValue::TypedBoolean { value: a, .. }, FhirPathValue::Boolean(b))
            | (
                FhirPathValue::TypedBoolean { value: a, .. },
                FhirPathValue::TypedBoolean { value: b, .. },
            ) => a == b,
            (FhirPathValue::String(a), FhirPathValue::String(b)) => a == b,
            (FhirPathValue::String(a), FhirPathValue::TypedString { value: b, .. })
            | (FhirPathValue::TypedString { value: a, .. }, FhirPathValue::String(b))
            | (
                FhirPathValue::TypedString { value: a, .. },
                FhirPathValue::TypedString { value: b, .. },
            ) => a == b,
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => a == b,
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => a == b,
            (FhirPathValue::Long(a), FhirPathValue::Long(b)) => a == b,
            (FhirPathValue::Integer(a), FhirPathValue::Long(b))
            | (FhirPathValue::Long(a), FhirPathValue::Integer(b)) => a == b,
            (FhirPathValue::Integer(a), FhirPathValue::Number(b))
            | (FhirPathValue::Long(a), FhirPathValue::Number(b)) => Decimal::from(*a) == *b,
            (FhirPathValue::Number(a), FhirPathValue::Integer(b))
            | (FhirPathValue::Number(a), FhirPathValue::Long(b)) => *a == Decimal::from(*b),
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => a == b,
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => a == b,
            (FhirPathValue::DateTime(a), FhirPathValue::TypedDateTime { value: b, .. })
            | (FhirPathValue::TypedDateTime { value: a, .. }, FhirPathValue::DateTime(b))
            | (
                FhirPathValue::TypedDateTime { value: a, .. },
                FhirPathValue::TypedDateTime { value: b, .. },
            ) => a == b,
            (FhirPathValue::Time(a), FhirPathValue::Time(b)) => {
                Self::canonical_time(a) == Self::canonical_time(b)
            }
            (FhirPathValue::DateTimePrecision(a), FhirPathValue::DateTimePrecision(b)) => a == b,
            (FhirPathValue::Date(a), FhirPathValue::String(b))
            | (FhirPathValue::Date(a), FhirPathValue::TypedString { value: b, .. }) => a == b,
            (FhirPathValue::String(a), FhirPathValue::Date(b))
            | (FhirPathValue::TypedString { value: a, .. }, FhirPathValue::Date(b)) => a == b,
            (FhirPathValue::DateTime(a), FhirPathValue::String(b))
            | (FhirPathValue::DateTime(a), FhirPathValue::TypedString { value: b, .. })
            | (FhirPathValue::TypedDateTime { value: a, .. }, FhirPathValue::String(b))
            | (
                FhirPathValue::TypedDateTime { value: a, .. },
                FhirPathValue::TypedString { value: b, .. },
            ) => a == b,
            (FhirPathValue::String(a), FhirPathValue::DateTime(b))
            | (FhirPathValue::TypedString { value: a, .. }, FhirPathValue::DateTime(b))
            | (FhirPathValue::String(a), FhirPathValue::TypedDateTime { value: b, .. })
            | (
                FhirPathValue::TypedString { value: a, .. },
                FhirPathValue::TypedDateTime { value: b, .. },
            ) => a == b,
            (FhirPathValue::Time(a), FhirPathValue::String(b))
            | (FhirPathValue::Time(a), FhirPathValue::TypedString { value: b, .. }) => {
                Self::canonical_time(a) == Self::canonical_time(b)
            }
            (FhirPathValue::String(a), FhirPathValue::Time(b))
            | (FhirPathValue::TypedString { value: a, .. }, FhirPathValue::Time(b)) => {
                Self::canonical_time(a) == Self::canonical_time(b)
            }
            (FhirPathValue::Empty, FhirPathValue::Empty) => true,
            (FhirPathValue::Collection(a), FhirPathValue::Collection(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| Self::equals_static(x, y))
            }
            (FhirPathValue::Object(a), FhirPathValue::Object(b)) => a == b,
            (
                FhirPathValue::TypedObject {
                    value: a,
                    fhir_type: ta,
                },
                FhirPathValue::TypedObject {
                    value: b,
                    fhir_type: tb,
                },
            ) => ta == tb && a == b,
            (FhirPathValue::TypedObject { value: a, .. }, FhirPathValue::Object(b))
            | (FhirPathValue::Object(a), FhirPathValue::TypedObject { value: b, .. }) => a == b,
            (FhirPathValue::FhirPrimitive { inner: a, .. }, other)
            | (other, FhirPathValue::FhirPrimitive { inner: a, .. })
                if !matches!(other, FhirPathValue::FhirPrimitive { .. }) =>
            {
                Self::equals_static(a, other)
            }
            (
                FhirPathValue::FhirPrimitive { inner: a, .. },
                FhirPathValue::FhirPrimitive { inner: b, .. },
            ) => Self::equals_static(a, b),
            (
                FhirPathValue::Quantity {
                    value: v1,
                    unit: u1,
                },
                FhirPathValue::Quantity {
                    value: v2,
                    unit: u2,
                },
            ) => {
                use crate::evaluator::operations::units::UnitConverter;
                let converter = UnitConverter::new();
                match converter.compare_quantities(*v1, u1, *v2, u2) {
                    Ok(0) => true,
                    Ok(_) => false,
                    Err(_) => false,
                }
            }
            _ => false,
        }
    }

    /// Check if a value is truthy for boolean operations
    pub fn is_truthy(&self) -> bool {
        match self {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::TypedBoolean { value, .. } => *value,
            FhirPathValue::FhirPrimitive { inner, .. } => inner.is_truthy(),
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                !items.is_empty()
            }
            _ => true,
        }
    }

    /// Convert a value to boolean
    pub fn to_boolean(&self) -> bool {
        match self {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::TypedBoolean { value, .. } => *value,
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                !items.is_empty()
            }
            _ => true,
        }
    }

    /// Convert a value to string for concatenation
    pub fn to_string_value(&self) -> FhirPathResult<String> {
        match self {
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => Ok(s.clone()),
            FhirPathValue::Integer(i) => Ok(i.to_string()),
            FhirPathValue::Number(n) => Ok(n.to_string()),
            FhirPathValue::Boolean(b) => Ok(b.to_string()),
            FhirPathValue::TypedBoolean { value, .. } => Ok(value.to_string()),
            FhirPathValue::Date(d) => Ok(d.clone()),
            FhirPathValue::DateTime(dt) => Ok(dt.clone()),
            FhirPathValue::TypedDateTime { value, .. } => Ok(value.clone()),
            FhirPathValue::Time(t) => Ok(Self::canonical_time(t).to_string()),
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
                    FhirPathValue::Number(Decimal::from_f64_retain(f).unwrap_or(Decimal::ZERO))
                } else {
                    FhirPathValue::Number(Decimal::ZERO)
                }
            }
            Value::String(s) => FhirPathValue::String(s.clone()),
            Value::Array(arr) => {
                let items: Vec<FhirPathValue> = arr.iter().map(Self::from_json).collect();
                if items.is_empty() {
                    FhirPathValue::Empty
                } else {
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
            FhirPathValue::TypedBoolean { value, .. } => Value::Bool(*value),
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
                Value::String(s.clone())
            }
            FhirPathValue::Number(n) => {
                let fval = n.to_f64().unwrap_or(0.0);
                Value::Number(
                    serde_json::Number::from_f64(fval)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                )
            }
            FhirPathValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
            FhirPathValue::Long(l) => Value::Number(serde_json::Number::from(*l)),
            FhirPathValue::Date(s) => Value::String(s.clone()),
            FhirPathValue::DateTime(s) => Value::String(s.clone()),
            FhirPathValue::TypedDateTime { value, .. } => Value::String(value.clone()),
            FhirPathValue::Time(s) => Value::String(Self::canonical_time(s).to_string()),
            FhirPathValue::Quantity { value, unit: _ } => {
                let fval = value.to_f64().unwrap_or(0.0);
                Value::Number(
                    serde_json::Number::from_f64(fval)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                )
            }
            FhirPathValue::DateTimePrecision(_) => Value::Null,
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                let json_items: Vec<Value> = items.iter().map(|item| item.to_json()).collect();
                Value::Array(json_items)
            }
            FhirPathValue::Object(obj) => obj.clone(),
            FhirPathValue::TypedObject { value, .. } => value.clone(),
            FhirPathValue::FhirPrimitive { inner, .. } => inner.to_json(),
            FhirPathValue::Empty => Value::Null,
        }
    }

    /// Compare two values for ordering (-1, 0, 1)
    pub fn compare_values(&self, other: &FhirPathValue) -> FhirPathResult<i32> {
        // Extract string content for comparison, handling both String and TypedString
        let left_str = match self {
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
                Some(s.as_str())
            }
            _ => None,
        };
        let right_str = match other {
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
                Some(s.as_str())
            }
            _ => None,
        };

        match (self, other) {
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
            (FhirPathValue::Integer(a), FhirPathValue::Number(b))
            | (FhirPathValue::Long(a), FhirPathValue::Number(b)) => {
                let a_dec = Decimal::from(*a);
                if a_dec < *b {
                    Ok(-1)
                } else if a_dec > *b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b))
            | (FhirPathValue::Number(a), FhirPathValue::Long(b)) => {
                let b_dec = Decimal::from(*b);
                if *a < b_dec {
                    Ok(-1)
                } else if *a > b_dec {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }

            // String comparisons (both String and TypedString)
            (_, _) if left_str.is_some() && right_str.is_some() => {
                Ok(left_str.unwrap().cmp(right_str.unwrap()) as i32)
            }

            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Boolean(a), FhirPathValue::TypedBoolean { value: b, .. })
            | (FhirPathValue::TypedBoolean { value: a, .. }, FhirPathValue::Boolean(b))
            | (
                FhirPathValue::TypedBoolean { value: a, .. },
                FhirPathValue::TypedBoolean { value: b, .. },
            ) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => Ok(a.cmp(b) as i32),
            (FhirPathValue::DateTime(a), FhirPathValue::TypedDateTime { value: b, .. })
            | (FhirPathValue::TypedDateTime { value: a, .. }, FhirPathValue::DateTime(b))
            | (
                FhirPathValue::TypedDateTime { value: a, .. },
                FhirPathValue::TypedDateTime { value: b, .. },
            ) => Ok(a.cmp(b) as i32),
            (FhirPathValue::Time(a), FhirPathValue::Time(b)) => {
                Ok(Self::canonical_time(a).cmp(Self::canonical_time(b)) as i32)
            }

            // Date/Time comparisons with implicit string conversion
            (FhirPathValue::Date(a), _) if right_str.is_some() => {
                Ok(a.as_str().cmp(right_str.unwrap()) as i32)
            }
            (_, FhirPathValue::Date(b)) if left_str.is_some() => {
                Ok(left_str.unwrap().cmp(b.as_str()) as i32)
            }
            (FhirPathValue::DateTime(a), _) if right_str.is_some() => {
                Ok(a.as_str().cmp(right_str.unwrap()) as i32)
            }
            (_, FhirPathValue::DateTime(b)) if left_str.is_some() => {
                Ok(left_str.unwrap().cmp(b.as_str()) as i32)
            }
            (FhirPathValue::TypedDateTime { value, .. }, _) if right_str.is_some() => {
                Ok(value.as_str().cmp(right_str.unwrap()) as i32)
            }
            (_, FhirPathValue::TypedDateTime { value, .. }) if left_str.is_some() => {
                Ok(left_str.unwrap().cmp(value.as_str()) as i32)
            }
            (FhirPathValue::Time(a), _) if right_str.is_some() => {
                Ok(Self::canonical_time(a).cmp(Self::canonical_time(right_str.unwrap())) as i32)
            }
            (_, FhirPathValue::Time(b)) if left_str.is_some() => {
                Ok(Self::canonical_time(left_str.unwrap()).cmp(Self::canonical_time(b)) as i32)
            }

            // Quantity comparisons with unit conversion
            (
                FhirPathValue::Quantity {
                    value: v1,
                    unit: u1,
                },
                FhirPathValue::Quantity {
                    value: v2,
                    unit: u2,
                },
            ) => {
                use crate::evaluator::operations::units::UnitConverter;
                let converter = UnitConverter::new();
                converter.compare_quantities(*v1, u1, *v2, u2)
            }

            _ => Err(FhirPathError::EvaluationError {
                message: format!("Cannot compare {self:?} and {other:?}"),
            }),
        }
    }

    /// Check if a value is a member of a collection
    pub fn is_member_of(&self, collection: &FhirPathValue) -> FhirPathResult<bool> {
        match collection {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Ok(items.iter().any(|item| Self::equals_static(self, item)))
            }
            FhirPathValue::Empty => Ok(false),
            other => Ok(Self::equals_static(self, other)),
        }
    }
}
