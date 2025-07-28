//! Integration tests for SQL-on-FHIR extension functionality
//!
//! These tests validate that the SQL-on-FHIR extension functions work correctly
//! with the FHIRPath parser and evaluator.

#[cfg(test)]
mod tests {
    use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
    use serde_json::json;

    /// Sample Patient resource for testing
    fn sample_patient() -> serde_json::Value {
        json!({
            "resourceType": "Patient",
            "id": "patient-123",
            "identifier": [{
                "use": "usual",
                "type": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }]
                },
                "system": "http://hospital.example.org",
                "value": "12345"
            }],
            "name": [{
                "use": "official",
                "family": "Smith",
                "given": ["John"]
            }],
            "gender": "male",
            "birthDate": "1980-01-01"
        })
    }

    /// Sample Observation resource with reference for testing
    fn sample_observation() -> serde_json::Value {
        json!({
            "resourceType": "Observation",
            "id": "observation-456",
            "status": "final",
            "category": [{
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                    "code": "vital-signs"
                }]
            }],
            "code": {
                "coding": [{
                    "system": "http://loinc.org",
                    "code": "8867-4",
                    "display": "Heart rate"
                }]
            },
            "subject": {
                "reference": "Patient/patient-123"
            },
            "valueQuantity": {
                "value": 72,
                "unit": "beats/min",
                "system": "http://unitsofmeasure.org",
                "code": "/min"
            }
        })
    }

    #[test]
    fn test_get_resource_key_integration() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient());

        // Test getResourceKey() function
        let parsed = parser.parse("getResourceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_resource_key_no_id() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        
        let patient_no_id = json!({
            "resourceType": "Patient",
            "name": [{
                "family": "Smith"
            }]
        });
        let context = EvaluationContext::new(patient_no_id);

        let parsed = parser.parse("getResourceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0, "Should return empty collection when no ID");
            },
            _ => panic!("Expected empty collection for resource without ID")
        }
    }

    #[test]
    fn test_get_reference_key_integration() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation());

        // Test getReferenceKey() without type filter
        let parsed = parser.parse("subject.getReferenceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_reference_key_with_type_filter() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation());

        // Test getReferenceKey(Patient) - should match
        let parsed = parser.parse("subject.getReferenceKey('Patient')").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_get_reference_key_with_non_matching_type_filter() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation());

        // Test getReferenceKey('Organization') - should not match
        let parsed = parser.parse("subject.getReferenceKey('Organization')").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0, "Should return empty collection for non-matching type");
            },
            _ => panic!("Expected empty collection for non-matching type")
        }
    }

    #[test]
    fn test_sql_on_fhir_with_complex_expressions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation());

        // Test complex expression combining SQL-on-FHIR with standard FHIRPath
        let parsed = parser.parse("subject.where(reference.exists()).getReferenceKey('Patient')").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        assert_eq!(result, FhirPathValue::String("patient-123".to_string()));
    }

    #[test]
    fn test_sql_on_fhir_key_consistency() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        
        // Test that getResourceKey and getReferenceKey return consistent values
        let patient_context = EvaluationContext::new(sample_patient());
        let observation_context = EvaluationContext::new(sample_observation());

        // Get resource key from Patient
        let parsed_resource = parser.parse("getResourceKey()").unwrap();
        let resource_key = evaluator.evaluate(&parsed_resource, &patient_context).unwrap();

        // Get reference key from Observation pointing to the same Patient
        let parsed_reference = parser.parse("subject.getReferenceKey()").unwrap();
        let reference_key = evaluator.evaluate(&parsed_reference, &observation_context).unwrap();

        // Keys should be equal
        assert_eq!(resource_key, reference_key, "Resource key and reference key should be equal");
    }

    #[test]
    fn test_get_resource_key_on_non_resource() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        
        let non_resource = json!({
            "value": "not a resource"
        });
        let context = EvaluationContext::new(non_resource);

        let parsed = parser.parse("getResourceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context);
        
        // Should return an error when called on non-resource
        assert!(result.is_err());
    }

    #[test]
    fn test_get_reference_key_on_non_reference() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient());

        // Test getReferenceKey on a non-reference field (should return empty collection)
        let parsed = parser.parse("name.getReferenceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0, "Should return empty collection for non-reference");
            },
            _ => panic!("Expected empty collection for non-reference")
        }
    }

    #[test]
    fn test_get_reference_key_invalid_reference_format() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        
        let invalid_reference = json!({
            "resourceType": "Observation",
            "id": "obs-1",
            "subject": {
                "reference": "invalid-reference-format"
            }
        });
        let context = EvaluationContext::new(invalid_reference);

        let parsed = parser.parse("subject.getReferenceKey()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();
        
        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0, "Should return empty collection for invalid reference format");
            },
            _ => panic!("Expected empty collection for invalid reference format")
        }
    }

    #[test]
    fn test_sql_on_fhir_error_handling() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient());

        // Test getResourceKey with parameters (should error)
        let parsed = parser.parse("getResourceKey('invalid')").unwrap();
        let result = evaluator.evaluate(&parsed, &context);
        assert!(result.is_err(), "getResourceKey should error when given parameters");

        // Test getReferenceKey with too many parameters (should error)
        let observation_context = EvaluationContext::new(sample_observation());
        let parsed2 = parser.parse("subject.getReferenceKey('Patient', 'extra')").unwrap();
        let result2 = evaluator.evaluate(&parsed2, &observation_context);
        assert!(result2.is_err(), "getReferenceKey should error with too many parameters");
    }
}
