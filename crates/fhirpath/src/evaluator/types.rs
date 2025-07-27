//! Type operations for FHIRPath expressions (is, as)

use crate::ast::*;
use crate::error::*;
use crate::evaluator::values::FhirPathValue;

/// Type operations handler
pub struct TypeEvaluator;

impl TypeEvaluator {
    /// Evaluate type operation (is, as)
    pub fn evaluate_type_operation(
        left: &FhirPathValue,
        operator: &TypeOperator,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            TypeOperator::Is => Self::evaluate_is(left, type_specifier),
            TypeOperator::As => Self::evaluate_as(left, type_specifier),
        }
    }

    /// Evaluate 'is' operation - type testing
    fn evaluate_is(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);
        let matches = Self::value_matches_type(value, &type_name)?;
        Ok(FhirPathValue::Boolean(matches))
    }

    /// Evaluate 'as' operation - type casting
    fn evaluate_as(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);

        // If the value matches the type, return it; otherwise return empty
        if Self::value_matches_type(value, &type_name)? {
            Ok(value.clone())
        } else {
            Ok(FhirPathValue::Empty)
        }
    }

    /// Convert TypeSpecifier to a simple type name string
    fn get_type_name(type_specifier: &TypeSpecifier) -> String {
        type_specifier.qualified_name.join(".")
    }

    /// Evaluate 'ofType' operation - filter collection by type
    /// Returns a collection containing only items that match the specified type
    pub fn evaluate_of_type(
        value: &FhirPathValue,
        type_specifier: &TypeSpecifier,
    ) -> FhirPathResult<FhirPathValue> {
        let type_name = Self::get_type_name(type_specifier);

        match value {
            FhirPathValue::Collection(items) => {
                let mut filtered_items = Vec::new();
                for item in items {
                    if Self::value_matches_type(item, &type_name)? {
                        filtered_items.push(item.clone());
                    }
                }

                if filtered_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Collection(filtered_items))
                }
            }
            // For single items, check if it matches the type
            _ => {
                if Self::value_matches_type(value, &type_name)? {
                    Ok(value.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Check if a value matches a given type
    fn value_matches_type(value: &FhirPathValue, type_name: &str) -> FhirPathResult<bool> {
        match type_name.to_lowercase().as_str() {
            // Basic types
            "boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "string" => Ok(matches!(value, FhirPathValue::String(_))),
            "integer" => Ok(matches!(value, FhirPathValue::Integer(_))),
            "number" | "decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // Collection types
            "collection" => Ok(matches!(value, FhirPathValue::Collection(_))),

            // Object/resource types - this is simplified, in a full implementation
            // you would check the actual FHIR resource type
            "object" | "resource" => Ok(matches!(value, FhirPathValue::Object(_))),

            // System types (following FHIRPath spec)
            "system.boolean" => Ok(matches!(value, FhirPathValue::Boolean(_))),
            "system.string" => Ok(matches!(value, FhirPathValue::String(_))),
            "system.integer" => Ok(matches!(value, FhirPathValue::Integer(_))),
            "system.decimal" => Ok(matches!(value, FhirPathValue::Number(_))),
            "system.date" => Ok(matches!(value, FhirPathValue::Date(_))),
            "system.datetime" => Ok(matches!(value, FhirPathValue::DateTime(_))),
            "system.time" => Ok(matches!(value, FhirPathValue::Time(_))),
            "system.quantity" => Ok(matches!(value, FhirPathValue::Quantity { .. })),

            // Handle collections recursively - check if all items match the type
            type_name if type_name.ends_with("[]") => {
                let element_type = &type_name[..type_name.len() - 2];
                match value {
                    FhirPathValue::Collection(items) => {
                        for item in items {
                            if !Self::value_matches_type(item, element_type)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }

            // Fallback for unknown types
            _ => {
                // Check for FHIR resource types by examining the resourceType field
                if let FhirPathValue::Object(obj) = value {
                    if let Some(resource_type) = obj.get("resourceType") {
                        if let Some(resource_type_str) = resource_type.as_str() {
                            return Ok(resource_type_str.eq_ignore_ascii_case(type_name));
                        }
                    }
                }

                // For unknown types, return false
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator_basic_types() {
        // Test Boolean
        let boolean_val = FhirPathValue::Boolean(true);
        let boolean_type = TypeSpecifier {
            qualified_name: vec!["Boolean".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&boolean_val, &boolean_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test String
        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&string_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test type mismatch
        let result = TypeEvaluator::evaluate_is(&boolean_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_is_operator_system_types() {
        let boolean_val = FhirPathValue::Boolean(true);
        let system_boolean_type = TypeSpecifier {
            qualified_name: vec!["System".to_string(), "Boolean".to_string()],
        };
        let result = TypeEvaluator::evaluate_is(&boolean_val, &system_boolean_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_as_operator() {
        // Test successful cast
        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let result = TypeEvaluator::evaluate_as(&string_val, &string_type).unwrap();
        assert_eq!(result, string_val);

        // Test failed cast
        let boolean_val = FhirPathValue::Boolean(true);
        let result = TypeEvaluator::evaluate_as(&boolean_val, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_numeric_types() {
        let integer_val = FhirPathValue::Integer(42);
        let number_val = FhirPathValue::Number(std::f64::consts::PI);

        let integer_type = TypeSpecifier {
            qualified_name: vec!["Integer".to_string()],
        };
        let decimal_type = TypeSpecifier {
            qualified_name: vec!["Decimal".to_string()],
        };

        // Integer is Integer
        let result = TypeEvaluator::evaluate_is(&integer_val, &integer_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Integer is not Decimal
        let result = TypeEvaluator::evaluate_is(&integer_val, &decimal_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        // Number is Decimal
        let result = TypeEvaluator::evaluate_is(&number_val, &decimal_type).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_of_type_basic_filtering() {
        // Test filtering strings from mixed collection
        let mixed_collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("hello".to_string()),
            FhirPathValue::Integer(42),
            FhirPathValue::Boolean(true),
            FhirPathValue::String("world".to_string()),
        ]);

        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&mixed_collection, &string_type).unwrap();

        if let FhirPathValue::Collection(items) = result {
            assert_eq!(items.len(), 2);
            assert!(matches!(items[0], FhirPathValue::String(_)));
            assert!(matches!(items[1], FhirPathValue::String(_)));
        } else {
            panic!("Expected collection result");
        }
    }

    #[test]
    fn test_of_type_resource_filtering() {
        use serde_json::json;

        // Test filtering FHIR resources by type
        let resources = FhirPathValue::Collection(vec![
            FhirPathValue::Object(json!({
                "resourceType": "Patient",
                "id": "patient1"
            })),
            FhirPathValue::Object(json!({
                "resourceType": "Observation",
                "id": "obs1"
            })),
            FhirPathValue::Object(json!({
                "resourceType": "Patient",
                "id": "patient2"
            })),
        ]);

        let patient_type = TypeSpecifier {
            qualified_name: vec!["Patient".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&resources, &patient_type).unwrap();

        if let FhirPathValue::Collection(items) = result {
            assert_eq!(items.len(), 2);
            for item in items {
                if let FhirPathValue::Object(obj) = item {
                    assert_eq!(
                        obj.get("resourceType").unwrap().as_str().unwrap(),
                        "Patient"
                    );
                } else {
                    panic!("Expected object result");
                }
            }
        } else {
            panic!("Expected collection result");
        }
    }

    #[test]
    fn test_of_type_single_item() {
        // Test ofType on single item
        let string_val = FhirPathValue::String("hello".to_string());
        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };
        let integer_type = TypeSpecifier {
            qualified_name: vec!["Integer".to_string()],
        };

        // String matches String type
        let result = TypeEvaluator::evaluate_of_type(&string_val, &string_type).unwrap();
        assert_eq!(result, string_val);

        // String doesn't match Integer type
        let result = TypeEvaluator::evaluate_of_type(&string_val, &integer_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_of_type_empty_result() {
        // Test ofType that returns empty
        let integer_collection = FhirPathValue::Collection(vec![
            FhirPathValue::Integer(1),
            FhirPathValue::Integer(2),
            FhirPathValue::Integer(3),
        ]);

        let string_type = TypeSpecifier {
            qualified_name: vec!["String".to_string()],
        };

        let result = TypeEvaluator::evaluate_of_type(&integer_collection, &string_type).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }
}
