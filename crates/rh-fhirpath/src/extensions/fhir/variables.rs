//! FHIR-specific extension variables
//!
//! This module provides FHIR-specific variables that are commonly used
//! in FHIR FHIRPath expressions.

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

/// Variable resolver function signature
type VariableResolver =
    Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>> + Send + Sync>;

/// Register all FHIR extension variables
pub fn register_variables(variables: &mut HashMap<String, VariableResolver>) {
    // %resource variable - the current resource being evaluated
    variables.insert(
        "resource".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| {
            resolve_resource_variable(name, context)
        }),
    );

    // %context variable - the current evaluation context
    variables.insert(
        "context".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| resolve_context_variable(name, context)),
    );

    // %rootResource variable - the root resource in evaluation
    variables.insert(
        "rootResource".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| {
            resolve_root_resource_variable(name, context)
        }),
    );

    // %ucum variable - UCUM system URI
    variables.insert(
        "ucum".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| resolve_ucum_variable(name, context)),
    );

    // %sct variable - SNOMED CT system URI
    variables.insert(
        "sct".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| resolve_sct_variable(name, context)),
    );

    // %loinc variable - LOINC system URI
    variables.insert(
        "loinc".to_string(),
        Box::new(|name: &str, context: &EvaluationContext| resolve_loinc_variable(name, context)),
    );
}

/// Resolve %resource variable - the current resource being evaluated
///
/// Returns the current resource from the evaluation context.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context containing the resource
///
/// # Returns
/// * `Ok(Some(FhirPathValue))` - The current resource if available
/// * `Ok(None)` - Variable not handled by this resolver
/// * `Err(FhirPathError)` - Error accessing the resource
fn resolve_resource_variable(
    name: &str,
    context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "resource" => {
            // Get the current resource from context
            // Convert JSON Value to FhirPathValue
            let value = crate::evaluator::types::FhirPathValue::from_json(&context.root);
            Ok(Some(value))
        }
        _ => Ok(None),
    }
}

/// Resolve %context variable - the current evaluation context
///
/// Returns information about the current evaluation context.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context
///
/// # Returns
/// * `Ok(Some(FhirPathValue))` - Context information if available
/// * `Ok(None)` - Variable not handled by this resolver
/// * `Err(FhirPathError)` - Error accessing the context
fn resolve_context_variable(
    name: &str,
    context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "context" => {
            // Return the current context - convert from JSON to FhirPathValue
            let value = crate::evaluator::types::FhirPathValue::from_json(&context.current);
            Ok(Some(value))
        }
        _ => Ok(None),
    }
}

/// Resolve %rootResource variable - the root resource in evaluation
///
/// Returns the root resource that started the evaluation.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context
///
/// # Returns
/// * `Ok(Some(FhirPathValue))` - The root resource if available
/// * `Ok(None)` - Variable not handled by this resolver
/// * `Err(FhirPathError)` - Error accessing the root resource
fn resolve_root_resource_variable(
    name: &str,
    context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "rootResource" => {
            // Return the root resource from context
            let value = crate::evaluator::types::FhirPathValue::from_json(&context.root);
            Ok(Some(value))
        }
        _ => Ok(None),
    }
}

/// Resolve %ucum variable - UCUM system URI
///
/// Returns the standard UCUM system URI for units of measure.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context (unused)
///
/// # Returns
/// * `Ok(Some(FhirPathValue::String))` - The UCUM system URI
/// * `Ok(None)` - Variable not handled by this resolver
fn resolve_ucum_variable(
    name: &str,
    _context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "ucum" => Ok(Some(FhirPathValue::String(
            "http://unitsofmeasure.org".to_string(),
        ))),
        _ => Ok(None),
    }
}

/// Resolve %sct variable - SNOMED CT system URI
///
/// Returns the standard SNOMED CT system URI.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context (unused)
///
/// # Returns
/// * `Ok(Some(FhirPathValue::String))` - The SNOMED CT system URI
/// * `Ok(None)` - Variable not handled by this resolver
fn resolve_sct_variable(
    name: &str,
    _context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "sct" => Ok(Some(FhirPathValue::String(
            "http://snomed.info/sct".to_string(),
        ))),
        _ => Ok(None),
    }
}

/// Resolve %loinc variable - LOINC system URI
///
/// Returns the standard LOINC system URI.
///
/// # Arguments
/// * `name` - The variable name being requested
/// * `context` - The evaluation context (unused)
///
/// # Returns
/// * `Ok(Some(FhirPathValue::String))` - The LOINC system URI
/// * `Ok(None)` - Variable not handled by this resolver
fn resolve_loinc_variable(
    name: &str,
    _context: &EvaluationContext,
) -> FhirPathResult<Option<FhirPathValue>> {
    match name {
        "loinc" => Ok(Some(FhirPathValue::String("http://loinc.org".to_string()))),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_resource_variable_resolution() {
        let patient = json!({
            "resourceType": "Patient",
            "id": "123",
            "name": [{"family": "Doe"}]
        });
        let context = EvaluationContext::new(patient.clone());

        let result = resolve_resource_variable("resource", &context).unwrap();
        assert!(result.is_some());

        // Should return the patient resource
        match result.unwrap() {
            FhirPathValue::Object(obj) => {
                assert_eq!(
                    obj.get("resourceType").unwrap().as_str().unwrap(),
                    "Patient"
                );
                assert_eq!(obj.get("id").unwrap().as_str().unwrap(), "123");
            }
            _ => panic!("Expected patient object"),
        }
    }

    #[test]
    fn test_context_variable_resolution() {
        let patient = json!({
            "resourceType": "Patient",
            "id": "123"
        });
        let context = EvaluationContext::new(patient);

        let result = resolve_context_variable("context", &context).unwrap();
        assert!(result.is_some());

        // Should return the context (same as resource for now)
        match result.unwrap() {
            FhirPathValue::Object(obj) => {
                assert_eq!(
                    obj.get("resourceType").unwrap().as_str().unwrap(),
                    "Patient"
                );
            }
            _ => panic!("Expected context object"),
        }
    }

    #[test]
    fn test_root_resource_variable_resolution() {
        let patient = json!({
            "resourceType": "Patient",
            "id": "123"
        });
        let context = EvaluationContext::new(patient);

        let result = resolve_root_resource_variable("rootResource", &context).unwrap();
        assert!(result.is_some());

        // Should return the root resource
        match result.unwrap() {
            FhirPathValue::Object(obj) => {
                assert_eq!(
                    obj.get("resourceType").unwrap().as_str().unwrap(),
                    "Patient"
                );
            }
            _ => panic!("Expected root resource object"),
        }
    }

    #[test]
    fn test_ucum_variable_resolution() {
        let context = EvaluationContext::new(json!({}));
        let result = resolve_ucum_variable("ucum", &context).unwrap();

        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            FhirPathValue::String("http://unitsofmeasure.org".to_string())
        );
    }

    #[test]
    fn test_sct_variable_resolution() {
        let context = EvaluationContext::new(json!({}));
        let result = resolve_sct_variable("sct", &context).unwrap();

        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            FhirPathValue::String("http://snomed.info/sct".to_string())
        );
    }

    #[test]
    fn test_loinc_variable_resolution() {
        let context = EvaluationContext::new(json!({}));
        let result = resolve_loinc_variable("loinc", &context).unwrap();

        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            FhirPathValue::String("http://loinc.org".to_string())
        );
    }

    #[test]
    fn test_unknown_variable() {
        let context = EvaluationContext::new(json!({}));

        // Test that unknown variables return None from all resolvers
        assert!(resolve_resource_variable("unknown", &context)
            .unwrap()
            .is_none());
        assert!(resolve_context_variable("unknown", &context)
            .unwrap()
            .is_none());
        assert!(resolve_root_resource_variable("unknown", &context)
            .unwrap()
            .is_none());
        assert!(resolve_ucum_variable("unknown", &context)
            .unwrap()
            .is_none());
        assert!(resolve_sct_variable("unknown", &context).unwrap().is_none());
        assert!(resolve_loinc_variable("unknown", &context)
            .unwrap()
            .is_none());
    }
}
