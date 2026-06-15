//! FHIR metadata integration for FHIRPath
//!
//! This module provides access to FHIR type metadata to ensure that FHIRPath
//! expressions return correctly typed values according to the FHIR specification.
//!
//! See: https://www.hl7.org/fhir/fhirpath.html

use crate::evaluator::types::FhirPathValue;
use rh_hl7_fhir_r4_core::metadata::{FhirFieldType, FhirPrimitiveType};
use rust_decimal::Decimal;

/// Get the FHIR type for a field in a resource
///
/// # Arguments
/// * `resource_type` - The FHIR resource type (e.g., "Patient")
/// * `field_name` - The field name (e.g., "birthDate")
///
/// # Returns
/// The FHIR field type if found, None otherwise
pub fn get_field_type(resource_type: &str, field_name: &str) -> Option<&'static FhirFieldType> {
    rh_hl7_fhir_r4_core::metadata::get_field_info(resource_type, field_name)
        .map(|info| &info.field_type)
}

/// Convert a JSON string value to a typed FHIRPath value based on FHIR metadata
///
/// # Arguments
/// * `value` - The string value from JSON
/// * `field_type` - The FHIR field type metadata
///
/// # Returns
/// A properly typed FhirPathValue
pub fn typed_value_from_string(value: String, field_type: &FhirFieldType) -> FhirPathValue {
    match field_type {
        FhirFieldType::Primitive(primitive_type) => match primitive_type {
            FhirPrimitiveType::Boolean => {
                // Try to parse as boolean
                match value.to_lowercase().as_str() {
                    "true" => FhirPathValue::TypedBoolean {
                        value: true,
                        fhir_type: FhirPrimitiveType::Boolean,
                    },
                    "false" => FhirPathValue::TypedBoolean {
                        value: false,
                        fhir_type: FhirPrimitiveType::Boolean,
                    },
                    _ => FhirPathValue::String(value),
                }
            }
            FhirPrimitiveType::Integer
            | FhirPrimitiveType::UnsignedInt
            | FhirPrimitiveType::PositiveInt => {
                // Try to parse as integer
                value
                    .parse::<i64>()
                    .map(FhirPathValue::Integer)
                    .unwrap_or_else(|_| FhirPathValue::String(value))
            }
            FhirPrimitiveType::Decimal => {
                // Try to parse as decimal (preserves trailing zeros)
                Decimal::from_str_exact(&value)
                    .map(FhirPathValue::Number)
                    .unwrap_or_else(|_| FhirPathValue::String(value))
            }
            FhirPrimitiveType::Date => FhirPathValue::Date(value),
            FhirPrimitiveType::DateTime | FhirPrimitiveType::Instant => {
                FhirPathValue::TypedDateTime {
                    value,
                    fhir_type: *primitive_type,
                }
            }
            FhirPrimitiveType::Time => FhirPathValue::Time(value),
            // String-based primitives: carry type provenance for is/as/ofType/type()
            FhirPrimitiveType::String => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::String,
            },
            FhirPrimitiveType::Code => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Code,
            },
            FhirPrimitiveType::Id => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Id,
            },
            FhirPrimitiveType::Uri => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Uri,
            },
            FhirPrimitiveType::Url => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Url,
            },
            FhirPrimitiveType::Canonical => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Canonical,
            },
            FhirPrimitiveType::Oid => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Oid,
            },
            FhirPrimitiveType::Markdown => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Markdown,
            },
            FhirPrimitiveType::Base64Binary => FhirPathValue::TypedString {
                value,
                fhir_type: FhirPrimitiveType::Base64Binary,
            },
        },
        // Complex types and references remain as-is (already handled as Objects)
        FhirFieldType::Complex(_)
        | FhirFieldType::Reference
        | FhirFieldType::BackboneElement(_) => FhirPathValue::String(value),
    }
}

/// Apply FHIR metadata typing to a value extracted from JSON
///
/// This is the main entry point for applying type information when accessing
/// fields on FHIR resources.
///
/// # Arguments
/// * `value` - The serde_json::Value extracted from the resource
/// * `resource_type` - The FHIR resource type (e.g., "Patient")
/// * `field_name` - The field name (e.g., "birthDate")
///
/// # Returns
/// A properly typed FhirPathValue
pub fn apply_fhir_typing(
    value: &serde_json::Value,
    resource_type: &str,
    field_name: &str,
) -> FhirPathValue {
    // Get the field type from metadata
    if let Some(field_type) = get_field_type(resource_type, field_name) {
        match (value, field_type) {
            (serde_json::Value::String(s), _) => {
                return typed_value_from_string(s.clone(), field_type);
            }
            (serde_json::Value::Bool(b), FhirFieldType::Primitive(FhirPrimitiveType::Boolean)) => {
                return FhirPathValue::TypedBoolean {
                    value: *b,
                    fhir_type: FhirPrimitiveType::Boolean,
                };
            }
            (serde_json::Value::Array(arr), FhirFieldType::Complex(type_name)) => {
                // Return each array element as a TypedObject, always as a Collection
                // (even for single items, to match the behavior of from_json on arrays)
                let items: Vec<FhirPathValue> = arr
                    .iter()
                    .map(|item| {
                        if item.is_object() {
                            FhirPathValue::TypedObject {
                                value: item.clone(),
                                fhir_type: type_name.to_string(),
                            }
                        } else {
                            FhirPathValue::from_json(item)
                        }
                    })
                    .collect();
                return if items.is_empty() {
                    FhirPathValue::Empty
                } else {
                    FhirPathValue::Collection(items)
                };
            }
            (serde_json::Value::Object(_), FhirFieldType::Complex(type_name)) => {
                return FhirPathValue::TypedObject {
                    value: value.clone(),
                    fhir_type: type_name.to_string(),
                };
            }
            _ => {}
        }
    } else {
        // Fallback: Try to infer type from field name patterns (for choice types and missing metadata)
        if let serde_json::Value::String(s) = value {
            return infer_type_from_field_name(s.clone(), field_name);
        }
    }

    // Fall back to default JSON conversion
    FhirPathValue::from_json(value)
}

/// Infer FHIR type from field name patterns when metadata is unavailable
///
/// This handles common FHIR field naming patterns like:
/// - Fields ending in "Date" → Date type
/// - Fields ending in "DateTime" → DateTime type  
/// - Fields ending in "Time" → Time type
/// - Fields ending in "Boolean" → Boolean type
/// - Fields ending in "Integer" → Integer type
fn infer_type_from_field_name(value: String, field_name: &str) -> FhirPathValue {
    // Check for common suffixes in field names
    if field_name.ends_with("DateTime") || field_name.ends_with("Instant") {
        FhirPathValue::TypedDateTime {
            value,
            fhir_type: if field_name.ends_with("Instant") {
                FhirPrimitiveType::Instant
            } else {
                FhirPrimitiveType::DateTime
            },
        }
    } else if field_name.ends_with("Date") {
        FhirPathValue::Date(value)
    } else if field_name.ends_with("Time") {
        FhirPathValue::Time(value)
    } else if field_name.ends_with("Boolean") {
        match value.to_lowercase().as_str() {
            "true" => FhirPathValue::TypedBoolean {
                value: true,
                fhir_type: FhirPrimitiveType::Boolean,
            },
            "false" => FhirPathValue::TypedBoolean {
                value: false,
                fhir_type: FhirPrimitiveType::Boolean,
            },
            _ => FhirPathValue::String(value),
        }
    } else if field_name.ends_with("Integer") {
        value
            .parse::<i64>()
            .map(FhirPathValue::Integer)
            .unwrap_or_else(|_| FhirPathValue::String(value))
    } else if field_name.ends_with("String") {
        FhirPathValue::TypedString {
            value,
            fhir_type: FhirPrimitiveType::String,
        }
    } else if field_name.ends_with("Uuid") {
        FhirPathValue::TypedString {
            value,
            fhir_type: FhirPrimitiveType::Uri,
        }
    } else {
        // Default to string
        FhirPathValue::String(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_patient_birthdate_type() {
        let field_type = get_field_type("Patient", "birthDate");
        assert!(field_type.is_some());

        if let Some(FhirFieldType::Primitive(FhirPrimitiveType::Date)) = field_type {
            // Expected
        } else {
            panic!("Expected Date primitive type for Patient.birthDate");
        }
    }

    #[test]
    fn test_typed_date_value() {
        let field_type = FhirFieldType::Primitive(FhirPrimitiveType::Date);
        let value = typed_value_from_string("1974-12-25".to_string(), &field_type);

        match value {
            FhirPathValue::Date(d) => assert_eq!(d, "1974-12-25"),
            _ => panic!("Expected Date value"),
        }
    }

    #[test]
    fn test_typed_boolean_value() {
        let field_type = FhirFieldType::Primitive(FhirPrimitiveType::Boolean);

        let value = typed_value_from_string("true".to_string(), &field_type);
        assert_eq!(
            value,
            FhirPathValue::TypedBoolean {
                value: true,
                fhir_type: FhirPrimitiveType::Boolean
            }
        );

        let value = typed_value_from_string("false".to_string(), &field_type);
        assert_eq!(
            value,
            FhirPathValue::TypedBoolean {
                value: false,
                fhir_type: FhirPrimitiveType::Boolean
            }
        );
    }

    #[test]
    fn test_typed_integer_value() {
        let field_type = FhirFieldType::Primitive(FhirPrimitiveType::Integer);
        let value = typed_value_from_string("42".to_string(), &field_type);

        match value {
            FhirPathValue::Integer(i) => assert_eq!(i, 42),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_apply_fhir_typing_for_patient_birthdate() {
        let json_value = serde_json::Value::String("1974-12-25".to_string());
        let result = apply_fhir_typing(&json_value, "Patient", "birthDate");

        match result {
            FhirPathValue::Date(d) => assert_eq!(d, "1974-12-25"),
            _ => panic!("Expected Date value for Patient.birthDate"),
        }
    }

    #[test]
    fn test_infer_datetime_from_field_name() {
        let result =
            infer_type_from_field_name("2023-10-22T14:30:00Z".to_string(), "effectiveDateTime");

        match result {
            FhirPathValue::TypedDateTime { value, fhir_type } => {
                assert_eq!(value, "2023-10-22T14:30:00Z");
                assert_eq!(fhir_type, FhirPrimitiveType::DateTime);
            }
            _ => panic!("Expected DateTime value for field ending in DateTime"),
        }
    }

    #[test]
    fn test_infer_date_from_field_name() {
        let result = infer_type_from_field_name("2023-10-22".to_string(), "recordedDate");

        match result {
            FhirPathValue::Date(d) => assert_eq!(d, "2023-10-22"),
            _ => panic!("Expected Date value for field ending in Date"),
        }
    }

    #[test]
    fn test_infer_time_from_field_name() {
        let result = infer_type_from_field_name("14:30:00".to_string(), "startTime");

        match result {
            FhirPathValue::Time(t) => assert_eq!(t, "14:30:00"),
            _ => panic!("Expected Time value for field ending in Time"),
        }
    }

    #[test]
    fn test_infer_boolean_from_field_name() {
        let result = infer_type_from_field_name("true".to_string(), "completedBoolean");
        assert_eq!(
            result,
            FhirPathValue::TypedBoolean {
                value: true,
                fhir_type: FhirPrimitiveType::Boolean
            }
        );

        let result = infer_type_from_field_name("false".to_string(), "activeBoolean");
        assert_eq!(
            result,
            FhirPathValue::TypedBoolean {
                value: false,
                fhir_type: FhirPrimitiveType::Boolean
            }
        );
    }

    #[test]
    fn test_infer_integer_from_field_name() {
        let result = infer_type_from_field_name("123".to_string(), "countInteger");

        match result {
            FhirPathValue::Integer(i) => assert_eq!(i, 123),
            _ => panic!("Expected Integer value for field ending in Integer"),
        }
    }

    #[test]
    fn test_infer_instant_from_field_name() {
        let result = infer_type_from_field_name("2023-10-22T14:30:00Z".to_string(), "someInstant");

        match result {
            FhirPathValue::TypedDateTime { value, fhir_type } => {
                assert_eq!(value, "2023-10-22T14:30:00Z");
                assert_eq!(fhir_type, FhirPrimitiveType::Instant);
            }
            _ => panic!("Expected DateTime value for field ending in Instant"),
        }
    }

    #[test]
    fn test_typed_datetime_and_instant() {
        let datetime_type = FhirFieldType::Primitive(FhirPrimitiveType::DateTime);
        let value = typed_value_from_string("2023-10-22T14:30:00Z".to_string(), &datetime_type);

        match value {
            FhirPathValue::TypedDateTime { value, fhir_type } => {
                assert_eq!(value, "2023-10-22T14:30:00Z");
                assert_eq!(fhir_type, FhirPrimitiveType::DateTime);
            }
            _ => panic!("Expected DateTime value"),
        }

        let instant_type = FhirFieldType::Primitive(FhirPrimitiveType::Instant);
        let value = typed_value_from_string("2023-10-22T14:30:00Z".to_string(), &instant_type);

        match value {
            FhirPathValue::TypedDateTime { value, fhir_type } => {
                assert_eq!(value, "2023-10-22T14:30:00Z");
                assert_eq!(fhir_type, FhirPrimitiveType::Instant);
            }
            _ => panic!("Expected DateTime value for Instant"),
        }
    }

    #[test]
    fn test_typed_decimal_value() {
        let field_type = FhirFieldType::Primitive(FhirPrimitiveType::Decimal);
        let value = typed_value_from_string("4.14159".to_string(), &field_type);

        match value {
            FhirPathValue::Number(n) => assert_eq!(n, Decimal::from_str_exact("4.14159").unwrap()),
            _ => panic!("Expected Number value for Decimal"),
        }
    }

    #[test]
    fn test_typed_string_primitives() {
        let string_type = FhirFieldType::Primitive(FhirPrimitiveType::String);
        let value = typed_value_from_string("hello".to_string(), &string_type);
        assert_eq!(
            value,
            FhirPathValue::TypedString {
                value: "hello".to_string(),
                fhir_type: FhirPrimitiveType::String
            }
        );

        let uri_type = FhirFieldType::Primitive(FhirPrimitiveType::Uri);
        let value = typed_value_from_string("http://example.org".to_string(), &uri_type);
        assert_eq!(
            value,
            FhirPathValue::TypedString {
                value: "http://example.org".to_string(),
                fhir_type: FhirPrimitiveType::Uri
            }
        );

        let code_type = FhirFieldType::Primitive(FhirPrimitiveType::Code);
        let value = typed_value_from_string("active".to_string(), &code_type);
        assert_eq!(
            value,
            FhirPathValue::TypedString {
                value: "active".to_string(),
                fhir_type: FhirPrimitiveType::Code
            }
        );
    }

    #[test]
    fn test_patient_active_field() {
        let field_type = get_field_type("Patient", "active");
        assert!(field_type.is_some());

        if let Some(FhirFieldType::Primitive(FhirPrimitiveType::Boolean)) = field_type {
            // Expected
        } else {
            panic!("Expected Boolean primitive type for Patient.active");
        }
    }
}
