//! FHIR-specific extension functions
//!
//! This module provides FHIR-specific functions that extend the core FHIRPath
//! specification with functionality commonly needed when working with FHIR resources.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

/// Function signature for FHIRPath extension functions
type ExtensionFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue> + Send + Sync>;

/// Register all FHIR extension functions
pub fn register_functions(functions: &mut HashMap<String, ExtensionFunction>) {
    // extension(url) function
    functions.insert(
        "extension".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            extension_function(target, params)
        }),
    );

    // hasValue() function to check if extension has a value
    functions.insert(
        "hasValue".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            has_value_function(target, params)
        }),
    );
}

/// Extension function that retrieves extensions by URL
///
/// Searches for extensions with the specified URL in the target resource or element.
///
/// # Arguments
/// * `target` - The FHIR resource or element to search for extensions
/// * `params` - Array containing exactly one string parameter (the extension URL)
///
/// # Returns
/// * `Ok(FhirPathValue::Collection)` - Collection of matching extensions
/// * `Err(FhirPathError)` - Error if parameters are invalid
///
/// # Example
/// ```ignore
/// Patient.extension('http://example.org/fhir/StructureDefinition/patient-nickname')
/// ```
fn extension_function(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    // Validate parameters
    if params.len() != 1 {
        return Err(FhirPathError::InvalidOperation {
            message: "extension() requires exactly one parameter (URL)".to_string(),
        });
    }

    let url = match &params[0] {
        FhirPathValue::String(s) => s,
        _ => {
            return Err(FhirPathError::TypeError {
                message: "extension() parameter must be a string URL".to_string(),
            })
        }
    };

    // Search for extensions in the target
    match target {
        FhirPathValue::Object(obj) => {
            // Look for extension array
            if let Some(extensions_value) = obj.get("extension") {
                let extensions =
                    crate::evaluator::types::FhirPathValue::from_json(extensions_value);
                if let FhirPathValue::Collection(extensions_vec) = extensions {
                    let matching_extensions: Vec<FhirPathValue> = extensions_vec
                        .into_iter()
                        .filter(|ext| {
                            if let FhirPathValue::Object(ext_obj) = ext {
                                if let Some(ext_url) = ext_obj.get("url") {
                                    if let Some(ext_url_str) = ext_url.as_str() {
                                        // Support both exact match and partial match (starts_with)
                                        return ext_url_str == url || ext_url_str.starts_with(url);
                                    }
                                }
                            }
                            false
                        })
                        .collect();

                    return Ok(FhirPathValue::Collection(matching_extensions));
                }
            }

            // No extensions found
            Ok(FhirPathValue::Collection(vec![]))
        }
        _ => {
            // Target is not an object that can have extensions
            Ok(FhirPathValue::Collection(vec![]))
        }
    }
}

/// HasValue function that checks if an extension has a value
///
/// Checks if the target extension has any value field (valueString, valueInteger, etc.)
/// or nested extensions that provide a value
///
/// # Arguments
/// * `target` - The extension to check
/// * `params` - Array of parameters (should be empty)
///
/// # Returns
/// * `Ok(FhirPathValue::Boolean)` - true if extension has a value, false otherwise
/// * `Err(FhirPathError)` - Error if parameters are provided
fn has_value_function(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    // Validate parameters
    if !params.is_empty() {
        return Err(FhirPathError::InvalidOperation {
            message: "hasValue() takes no parameters".to_string(),
        });
    }

    match target {
        // Handle collections - must contain exactly one value
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                has_value_function(&items[0], params)
            } else {
                Ok(FhirPathValue::Boolean(false))
            }
        }
        // FHIR primitives that have values
        FhirPathValue::String(_)
        | FhirPathValue::Integer(_)
        | FhirPathValue::Number(_)
        | FhirPathValue::Boolean(_)
        | FhirPathValue::Date(_)
        | FhirPathValue::DateTime(_)
        | FhirPathValue::Time(_)
        | FhirPathValue::Long(_) => Ok(FhirPathValue::Boolean(true)),
        // FHIR complex types (like extensions) - check for value[x] fields
        FhirPathValue::Object(obj) => {
            // Check for any value[x] field
            let has_direct_value = if let Some(obj_map) = obj.as_object() {
                obj_map.keys().any(|key| key.starts_with("value"))
            } else {
                false
            };

            // If no direct value, check for nested extensions
            if !has_direct_value {
                if let Some(extensions_value) = obj.get("extension") {
                    if extensions_value.is_array()
                        && !extensions_value.as_array().unwrap().is_empty()
                    {
                        return Ok(FhirPathValue::Boolean(true));
                    }
                }
            }

            if has_direct_value {
                Ok(FhirPathValue::Boolean(true))
            } else {
                Ok(FhirPathValue::Boolean(false))
            }
        }
        // Empty values and other types don't have values
        FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
        _ => Ok(FhirPathValue::Boolean(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extension_function_with_matching_extension() {
        let target_json = json!({
            "resourceType": "Patient",
            "extension": [
                {
                    "url": "http://example.org/nickname",
                    "valueString": "Johnny"
                },
                {
                    "url": "http://example.org/age",
                    "valueInteger": 25
                }
            ]
        });

        let target = crate::evaluator::types::FhirPathValue::from_json(&target_json);
        let params = vec![FhirPathValue::String(
            "http://example.org/nickname".to_string(),
        )];

        let result = extension_function(&target, &params).unwrap();

        match result {
            FhirPathValue::Collection(extensions) => {
                assert_eq!(extensions.len(), 1);
                // Check that we got the right extension
                if let FhirPathValue::Object(ext) = &extensions[0] {
                    assert_eq!(
                        ext.get("url").unwrap().as_str().unwrap(),
                        "http://example.org/nickname"
                    );
                } else {
                    panic!("Expected extension object");
                }
            }
            _ => panic!("Expected collection of extensions"),
        }
    }

    #[test]
    fn test_extension_function_no_matching_extension() {
        let target_json = json!({
            "resourceType": "Patient",
            "extension": [
                {
                    "url": "http://example.org/nickname",
                    "valueString": "Johnny"
                }
            ]
        });

        let target = crate::evaluator::types::FhirPathValue::from_json(&target_json);
        let params = vec![FhirPathValue::String(
            "http://example.org/nonexistent".to_string(),
        )];

        let result = extension_function(&target, &params).unwrap();

        match result {
            FhirPathValue::Collection(extensions) => {
                assert_eq!(extensions.len(), 0);
            }
            _ => panic!("Expected empty collection"),
        }
    }

    #[test]
    fn test_has_value_function_with_value() {
        let extension_json = json!({
            "url": "http://example.org/test",
            "valueString": "test value"
        });

        let target = crate::evaluator::types::FhirPathValue::from_json(&extension_json);
        let params = vec![];

        let result = has_value_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_has_value_function_without_value() {
        // Extension with nested extensions should have value (nested extensions are the value)
        let extension_with_nested = json!({
            "url": "http://example.org/test",
            "extension": [
                {
                    "url": "nested",
                    "valueString": "nested value"
                }
            ]
        });

        let target = crate::evaluator::types::FhirPathValue::from_json(&extension_with_nested);
        let params = vec![];

        let result = has_value_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Extension with no value* fields and no nested extensions should not have value
        let extension_empty = json!({
            "url": "http://example.org/test"
        });

        let target_empty = crate::evaluator::types::FhirPathValue::from_json(&extension_empty);
        let result_empty = has_value_function(&target_empty, &params).unwrap();
        assert_eq!(result_empty, FhirPathValue::Boolean(false));
    }
}
