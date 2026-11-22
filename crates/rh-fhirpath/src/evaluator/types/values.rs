//! FHIRPath value types and basic value operations

use crate::ast::DateTimePrecision;
use crate::error::*;
use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum FhirPathValue {
    /// Source-aware logical FHIR node returned by evaluate_nodes.
    Node(Box<crate::evaluator::metadata::FhirNode>),
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
    /// View a scalar as a one-item collection.
    pub(crate) fn items(&self) -> &[Self] {
        match self {
            Self::Empty => &[],
            Self::Collection(items) | Self::UnorderedCollection(items) => items,
            item => std::slice::from_ref(item),
        }
    }

    /// Whether this value holds an empty FHIRPath collection.
    pub fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    /// Apply singleton cardinality without losing the item's logical type.
    pub(crate) fn singleton(&self, operation: &str) -> FhirPathResult<Option<&Self>> {
        match self.items() {
            [] => Ok(None),
            [item] => Ok(Some(item)),
            items => Err(FhirPathError::EvaluationError {
                message: format!(
                    "{operation} requires at most one item, found {}",
                    items.len()
                ),
            }),
        }
    }

    /// Return the scalar carried by a FHIR node, or the value itself.
    pub(crate) fn system_value(&self) -> &Self {
        match self {
            Self::FhirPrimitive { inner, .. } => inner.system_value(),
            Self::Node(node) => node.inner.system_value(),
            value => value,
        }
    }

    /// Evaluate a collection as a Boolean using FHIRPath singleton semantics.
    pub(crate) fn singleton_boolean(&self, operation: &str) -> FhirPathResult<Option<bool>> {
        Ok(self
            .singleton(operation)?
            .map(|value| match value.system_value() {
                Self::Boolean(value) | Self::TypedBoolean { value, .. } => *value,
                _ => true,
            }))
    }

    /// Flatten the result of a projection by one collection level.
    pub(crate) fn into_items(self) -> Vec<Self> {
        match self {
            Self::Empty => Vec::new(),
            Self::Collection(items) | Self::UnorderedCollection(items) => items,
            item => vec![item],
        }
    }

    pub(crate) fn from_items(mut items: Vec<Self>) -> Self {
        match items.len() {
            0 => Self::Empty,
            1 => items.pop().unwrap(),
            _ => Self::Collection(items),
        }
    }

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
            FhirPathValue::Node(n) => n.inner.as_str(),
            FhirPathValue::String(s) => Some(s),
            FhirPathValue::TypedString { value, .. } => Some(value),
            _ => None,
        }
    }

    pub(crate) fn system_scalar_name(&self) -> Option<&'static str> {
        Some(match self {
            Self::Boolean(_) => "Boolean",
            Self::String(_) => "String",
            Self::Integer(_) => "Integer",
            Self::Long(_) => "Long",
            Self::Number(_) => "Decimal",
            Self::Date(_) => "Date",
            Self::DateTime(_) => "DateTime",
            Self::Time(_) => "Time",
            Self::Quantity { .. } => "Quantity",
            _ => return None,
        })
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

    /// Check if FHIRPath equality establishes that two values are equal.
    pub fn equals_static(left: &Self, right: &Self) -> bool {
        use crate::evaluator::operations::comparison::ComparisonEvaluator;
        ComparisonEvaluator::equal(left, right) == Some(true)
    }

    /// Check if a value is truthy for boolean operations
    pub fn is_truthy(&self) -> bool {
        match self {
            FhirPathValue::Node(n) => n.inner.is_truthy(),
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
            FhirPathValue::Node(n) => n.inner.to_boolean(),
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
            FhirPathValue::Node(n) => n.inner.to_string_value(),
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => Ok(s.clone()),
            FhirPathValue::Integer(i) | FhirPathValue::Long(i) => Ok(i.to_string()),
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
                    if i32::try_from(i).is_ok() {
                        FhirPathValue::Integer(i)
                    } else {
                        FhirPathValue::Long(i)
                    }
                } else {
                    let lexical = n.to_string();
                    Decimal::from_str_exact(&lexical)
                        .or_else(|_| Decimal::from_scientific(&lexical))
                        .map(FhirPathValue::Number)
                        .unwrap_or(FhirPathValue::Empty)
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
            FhirPathValue::Node(n) => n.raw.clone(),
            FhirPathValue::Boolean(b) => Value::Bool(*b),
            FhirPathValue::TypedBoolean { value, .. } => Value::Bool(*value),
            FhirPathValue::String(s) | FhirPathValue::TypedString { value: s, .. } => {
                Value::String(s.clone())
            }
            FhirPathValue::Number(n) => decimal_json(*n),
            FhirPathValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
            FhirPathValue::Long(l) => Value::Number(serde_json::Number::from(*l)),
            FhirPathValue::Date(s) => Value::String(s.clone()),
            FhirPathValue::DateTime(s) => Value::String(s.clone()),
            FhirPathValue::TypedDateTime { value, .. } => Value::String(value.clone()),
            FhirPathValue::Time(s) => Value::String(Self::canonical_time(s).to_string()),
            FhirPathValue::Quantity { value, unit } => {
                let mut object = serde_json::Map::new();
                object.insert("value".into(), decimal_json(*value));
                if let Some(unit) = unit {
                    object.insert("unit".into(), Value::String(unit.clone()));
                }
                Value::Object(object)
            }
            FhirPathValue::DateTimePrecision(precision) => Value::String(precision.to_string()),
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

    /// Compare two values for definite ordering (-1, 0, 1).
    pub fn compare_values(&self, other: &Self) -> FhirPathResult<i32> {
        use crate::evaluator::operations::comparison::ComparisonEvaluator;
        ComparisonEvaluator::compare_order(self, other)?
            .map(|order| order as i32)
            .ok_or_else(|| FhirPathError::EvaluationError {
                message: format!("Cannot determine order for {self:?} and {other:?}"),
            })
    }

    /// Check if a value is a member of a collection
    pub fn is_member_of(&self, collection: &FhirPathValue) -> FhirPathResult<bool> {
        Ok(collection
            .items()
            .iter()
            .any(|item| Self::equals_static(self, item)))
    }
}

fn decimal_json(value: Decimal) -> Value {
    Value::Number(
        value
            .to_string()
            .parse()
            .expect("Decimal is a valid JSON number"),
    )
}
