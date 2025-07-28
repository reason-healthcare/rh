//! SQL-on-FHIR extension functions
//!
//! This module provides SQL-on-FHIR specific functions for resource and reference key handling.
//! These functions are designed to support efficient joins in SQL-based FHIR implementations.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::extensions::ExtensionFunction;
use std::collections::HashMap;

/// Register all SQL-on-FHIR extension functions
pub fn register_functions(functions: &mut HashMap<String, ExtensionFunction>) {
    // getResourceKey() function
    functions.insert(
        "getResourceKey".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            get_resource_key_function(target, params)
        }),
    );

    // getReferenceKey([resource: type specifier]) function
    functions.insert(
        "getReferenceKey".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            get_reference_key_function(target, params)
        }),
    );
}

/// getResourceKey() function that returns an opaque value to be used as the primary key
/// 
/// This function is invoked at the root of a FHIR Resource and returns a key value
/// that can be used for efficient joins in the system's underlying data storage.
/// 
/// # Arguments
/// * `target` - The FHIR resource (must be at root level)
/// * `params` - Array of parameters (should be empty)
/// 
/// # Returns
/// * `Ok(FhirPathValue::String)` - The resource key (typically the resource ID)
/// * `Ok(FhirPathValue::Collection(vec![]))` - Empty collection if no ID available
/// * `Err(FhirPathError)` - Error if not invoked on a resource or parameters provided
/// 
/// # Example
/// ```ignore
/// Patient.getResourceKey() // Returns the Patient's key
/// ```
fn get_resource_key_function(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    // Validate parameters - getResourceKey takes no parameters
    if !params.is_empty() {
        return Err(FhirPathError::InvalidOperation {
            message: "getResourceKey() takes no parameters".to_string(),
        });
    }

    match target {
        FhirPathValue::Object(obj) => {
            // Verify this is a FHIR resource (has resourceType)
            if let Some(resource_type) = obj.get("resourceType") {
                if let Some(_resource_type_str) = resource_type.as_str() {
                    // Extract the resource ID - this is the primary implementation
                    // In real implementations, this could be more sophisticated
                    if let Some(id_value) = obj.get("id") {
                        if let Some(id_str) = id_value.as_str() {
                            return Ok(FhirPathValue::String(id_str.to_string()));
                        }
                    }
                    
                    // If no ID, return empty collection
                    return Ok(FhirPathValue::Collection(vec![]));
                } else {
                    return Err(FhirPathError::TypeError {
                        message: "resourceType must be a string".to_string(),
                    });
                }
            } else {
                return Err(FhirPathError::InvalidOperation {
                    message: "getResourceKey() can only be invoked on FHIR resources (objects with resourceType)".to_string(),
                });
            }
        }
        _ => {
            return Err(FhirPathError::InvalidOperation {
                message: "getResourceKey() can only be invoked on FHIR resources".to_string(),
            });
        }
    }
}

/// getReferenceKey([resource: type specifier]) function that returns the database key of the referenced resource
/// 
/// This function is invoked on Reference elements and returns a key value that matches
/// the getResourceKey value of the referenced resource.
/// 
/// # Arguments
/// * `target` - The Reference element to extract key from
/// * `params` - Optional array containing expected resource type (e.g., "Patient")
/// 
/// # Returns
/// * `Ok(FhirPathValue::String)` - The reference key if resolvable and type matches
/// * `Ok(FhirPathValue::Collection(vec![]))` - Empty collection if reference cannot be resolved or type doesn't match
/// * `Err(FhirPathError)` - Error if invalid parameters
/// 
/// # Example
/// ```ignore
/// Observation.subject.getReferenceKey() // Returns key for any referenced resource
/// Observation.subject.getReferenceKey(Patient) // Returns key only if reference is to Patient
/// ```
fn get_reference_key_function(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    // Validate parameters - getReferenceKey takes 0 or 1 parameter (optional resource type)
    if params.len() > 1 {
        return Err(FhirPathError::InvalidOperation {
            message: "getReferenceKey() takes at most one parameter (expected resource type)".to_string(),
        });
    }

    // Extract expected resource type if provided
    let expected_type = if params.len() == 1 {
        match &params[0] {
            FhirPathValue::String(type_str) => Some(type_str.as_str()),
            _ => return Err(FhirPathError::TypeError {
                message: "getReferenceKey() parameter must be a string resource type".to_string(),
            }),
        }
    } else {
        None
    };

    match target {
        FhirPathValue::Object(obj) => {
            // Look for reference field - this should be a Reference element
            if let Some(reference_value) = obj.get("reference") {
                if let Some(reference_str) = reference_value.as_str() {
                    return extract_reference_key(reference_str, expected_type);
                } else {
                    // Reference is not a string, return empty collection
                    return Ok(FhirPathValue::Collection(vec![]));
                }
            } else {
                // Not a reference object, return empty collection
                return Ok(FhirPathValue::Collection(vec![]));
            }
        }
        FhirPathValue::String(reference_str) => {
            // Direct string reference
            return extract_reference_key(reference_str, expected_type);
        }
        _ => {
            // Target is not a reference, return empty collection
            return Ok(FhirPathValue::Collection(vec![]));
        }
    }
}

/// Extract the key from a reference string
/// 
/// This function handles the parsing of reference strings and extracts the key.
/// Currently supports relative literal form (e.g., "Patient/123").
/// 
/// # Arguments
/// * `reference_str` - The reference string to parse
/// * `expected_type` - Optional expected resource type filter
/// 
/// # Returns
/// * `Ok(FhirPathValue::String)` - The extracted key if valid and type matches
/// * `Ok(FhirPathValue::Collection(vec![]))` - Empty collection if invalid or type doesn't match
fn extract_reference_key(
    reference_str: &str,
    expected_type: Option<&str>,
) -> FhirPathResult<FhirPathValue> {
    // Handle relative literal form: ResourceType/id
    if let Some(slash_pos) = reference_str.find('/') {
        let resource_type = &reference_str[..slash_pos];
        let resource_id = &reference_str[slash_pos + 1..];
        
        // Check if resource type matches expected type (if provided)
        if let Some(expected) = expected_type {
            if resource_type != expected {
                // Type doesn't match, return empty collection
                return Ok(FhirPathValue::Collection(vec![]));
            }
        }
        
        // Return the resource ID as the key
        if !resource_id.is_empty() {
            return Ok(FhirPathValue::String(resource_id.to_string()));
        }
    }
    
    // Handle other reference formats (URLs, URNs, etc.)
    // For now, we only support relative literal form as per the spec requirement
    // Other formats return empty collection
    
    // Could not parse or unsupported format
    Ok(FhirPathValue::Collection(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_resource_key_with_patient() {
        let patient_json = json!({
            "resourceType": "Patient",
            "id": "patient-123",
            "name": [{
                "family": "Smith",
                "given": ["John"]
            }]
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&patient_json);
        let params = vec![];
        
        let result = get_resource_key_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_resource_key_without_id() {
        let patient_json = json!({
            "resourceType": "Patient",
            "name": [{
                "family": "Smith",
                "given": ["John"]
            }]
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&patient_json);
        let params = vec![];
        
        let result = get_resource_key_function(&target, &params).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for resource without ID"),
        }
    }

    #[test]
    fn test_get_resource_key_non_resource() {
        let non_resource = json!({
            "name": "not a resource"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&non_resource);
        let params = vec![];
        
        let result = get_resource_key_function(&target, &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_resource_key_with_params() {
        let patient_json = json!({
            "resourceType": "Patient",
            "id": "patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&patient_json);
        let params = vec![FhirPathValue::String("invalid".to_string())];
        
        let result = get_resource_key_function(&target, &params);
        assert!(result.is_err());
        if let Err(FhirPathError::InvalidOperation { message }) = result {
            assert!(message.contains("takes no parameters"));
        } else {
            panic!("Expected InvalidOperation error");
        }
    }

    #[test]
    fn test_get_reference_key_relative_literal() {
        let reference_json = json!({
            "reference": "Patient/patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_reference_key_with_type_filter_matching() {
        let reference_json = json!({
            "reference": "Patient/patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![FhirPathValue::String("Patient".to_string())];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_reference_key_with_type_filter_not_matching() {
        let reference_json = json!({
            "reference": "Patient/patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![FhirPathValue::String("Observation".to_string())];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for non-matching type"),
        }
    }

    #[test]
    fn test_get_reference_key_direct_string() {
        let target = FhirPathValue::String("Patient/patient-123".to_string());
        let params = vec![];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_reference_key_invalid_format() {
        let reference_json = json!({
            "reference": "not-a-valid-reference"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for invalid reference format"),
        }
    }

    #[test]
    fn test_get_reference_key_non_reference_object() {
        let non_reference = json!({
            "value": "not a reference"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&non_reference);
        let params = vec![];
        
        let result = get_reference_key_function(&target, &params).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for non-reference object"),
        }
    }

    #[test]
    fn test_get_reference_key_too_many_params() {
        let reference_json = json!({
            "reference": "Patient/patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![
            FhirPathValue::String("Patient".to_string()),
            FhirPathValue::String("extra".to_string())
        ];
        
        let result = get_reference_key_function(&target, &params);
        assert!(result.is_err());
        if let Err(FhirPathError::InvalidOperation { message }) = result {
            assert!(message.contains("at most one parameter"));
        } else {
            panic!("Expected InvalidOperation error");
        }
    }

    #[test]
    fn test_extract_reference_key_various_formats() {
        // Test relative literal form
        let result = extract_reference_key("Patient/123", None).unwrap();
        assert_eq!(result, FhirPathValue::String("123".to_string()));
        
        // Test with type filter matching
        let result = extract_reference_key("Patient/123", Some("Patient")).unwrap();
        assert_eq!(result, FhirPathValue::String("123".to_string()));
        
        // Test with type filter not matching
        let result = extract_reference_key("Patient/123", Some("Observation")).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for non-matching type"),
        }
        
        // Test invalid format (no slash)
        let result = extract_reference_key("Patient123", None).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for invalid format"),
        }
        
        // Test empty ID
        let result = extract_reference_key("Patient/", None).unwrap();
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty collection for empty ID"),
        }
    }

    #[test]
    fn test_get_reference_key_invalid_param_type() {
        let reference_json = json!({
            "reference": "Patient/patient-123"
        });
        
        let target = crate::evaluator::types::FhirPathValue::from_json(&reference_json);
        let params = vec![FhirPathValue::Integer(123)]; // Invalid type
        
        let result = get_reference_key_function(&target, &params);
        assert!(result.is_err());
        if let Err(FhirPathError::TypeError { message }) = result {
            assert!(message.contains("must be a string resource type"));
        } else {
            panic!("Expected TypeError");
        }
    }
}
