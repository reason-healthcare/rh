//! Integration tests for FHIR extension functionality
//!
//! These tests validate the FHIR extension functions and variables work correctly
//! with various FHIR resources and extension patterns.

#[cfg(test)]
mod tests {
    use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
    use serde_json::json;

    /// Sample Patient with US Core race and ethnicity extensions
    fn sample_patient_with_extensions() -> serde_json::Value {
        json!({
            "resourceType": "Patient",
            "id": "patient-with-extensions",
            "meta": {
                "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
            },
            "extension": [
                {
                    "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                    "extension": [
                        {
                            "url": "ombCategory",
                            "valueCoding": {
                                "system": "urn:oid:2.16.840.1.113883.6.238",
                                "code": "2106-3",
                                "display": "White"
                            }
                        },
                        {
                            "url": "text",
                            "valueString": "White"
                        }
                    ]
                },
                {
                    "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                    "extension": [
                        {
                            "url": "ombCategory",
                            "valueCoding": {
                                "system": "urn:oid:2.16.840.1.113883.6.238",
                                "code": "2186-5",
                                "display": "Not Hispanic or Latino"
                            }
                        }
                    ]
                },
                {
                    "url": "http://example.org/custom-extension",
                    "valueString": "custom-value"
                }
            ],
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
                "given": ["John", "Michael"]
            }],
            "gender": "male",
            "birthDate": "1980-01-01"
        })
    }

    /// Sample Observation with various extensions
    fn sample_observation_with_extensions() -> serde_json::Value {
        json!({
            "resourceType": "Observation",
            "id": "obs-with-extensions",
            "status": "final",
            "extension": [
                {
                    "url": "http://hl7.org/fhir/StructureDefinition/observation-bodyPosition",
                    "valueCodeableConcept": {
                        "coding": [{
                            "system": "http://snomed.info/sct",
                            "code": "33586001",
                            "display": "Sitting position"
                        }]
                    }
                }
            ],
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
                "reference": "Patient/example"
            },
            "valueQuantity": {
                "value": 72,
                "unit": "beats/min",
                "system": "http://unitsofmeasure.org",
                "code": "/min"
            }
        })
    }

    /// Sample resource without extensions for negative testing
    fn sample_patient_no_extensions() -> serde_json::Value {
        json!({
            "resourceType": "Patient",
            "id": "patient-no-extensions",
            "name": [{
                "use": "official",
                "family": "Doe",
                "given": ["Jane"]
            }],
            "gender": "female",
            "birthDate": "1990-01-01"
        })
    }

    #[test]
    fn test_extension_function_basic() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test finding US Core race extension
        let parsed = parser
            .parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 1, "Should find exactly one race extension");
                // Verify it's the correct extension
                if let FhirPathValue::Object(ext) = &items[0] {
                    assert_eq!(
                        ext.get("url").unwrap(),
                        &json!("http://hl7.org/fhir/us/core/StructureDefinition/us-core-race")
                    );
                } else {
                    panic!("Extension should be an object");
                }
            }
            _ => panic!("Should return a collection of extensions"),
        }
    }

    #[test]
    fn test_extension_function_multiple_matches() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test finding extensions with partial URL match (should find US Core extensions)
        let parsed = parser
            .parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/')")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(
                    items.len(),
                    2,
                    "Should find both US Core extensions (race and ethnicity)"
                );
            }
            _ => panic!("Should return a collection of extensions"),
        }
    }

    #[test]
    fn test_extension_function_no_match() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test finding non-existent extension
        let parsed = parser
            .parse("extension('http://nonexistent.org/extension')")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 0, "Should find no extensions");
            }
            _ => panic!("Should return an empty collection"),
        }
    }

    #[test]
    fn test_extension_function_nested_navigation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test navigating into nested extensions
        let parsed = parser.parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'ombCategory').valueCoding.code").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 1, "Should find one race code");
                assert_eq!(items[0], FhirPathValue::String("2106-3".to_string()));
            }
            FhirPathValue::String(code) => {
                assert_eq!(code, "2106-3");
            }
            _ => panic!("Should return race code, got: {result:?}"),
        }
    }

    #[test]
    fn test_has_value_function_with_value() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test hasValue() on extension with nested extensions (should return true)
        let parsed = parser.parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')[0].hasValue()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Race extension should have value (nested extensions)"
        );
    }

    #[test]
    fn test_has_value_function_with_direct_value() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test hasValue() on extension with direct value
        let parsed = parser
            .parse("extension('http://example.org/custom-extension')[0].hasValue()")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Custom extension should have direct value"
        );
    }

    #[test]
    fn test_has_value_function_empty_extension() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        // Create a patient with an empty extension
        let patient_empty_ext = json!({
            "resourceType": "Patient",
            "id": "patient-empty-ext",
            "extension": [{
                "url": "http://example.org/empty-extension"
                // No value* properties or nested extensions
            }],
            "name": [{"family": "Test"}]
        });
        let context = EvaluationContext::new(patient_empty_ext);

        let parsed = parser
            .parse("extension('http://example.org/empty-extension')[0].hasValue()")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(false),
            "Empty extension should not have value"
        );
    }

    #[test]
    fn test_context_id_has_value() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test %context.id.hasValue() - should return true for primitive with value
        let parsed = parser.parse("%context.id.hasValue()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Context id should have value (primitive string with value)"
        );
    }

    #[test]
    fn test_has_value_with_multiple_items() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test hasValue() on collection with multiple items - should return empty
        let parsed = parser.parse("name.hasValue()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(false),
            "Collection with multiple items should return false for hasValue()"
        );
    }

    #[test]
    fn test_has_value_with_single_item_collection() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test hasValue() on collection with single item - should return true for primitive
        let parsed = parser.parse("gender.hasValue()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Single item collection with primitive should have value"
        );
    }

    #[test]
    fn test_has_value_with_empty_collection() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        // Create a patient with no telecom
        let patient_no_telecom = json!({
            "resourceType": "Patient",
            "id": "patient-no-telecom",
            "name": [{"family": "Test"}]
        });
        let context = EvaluationContext::new(patient_no_telecom);

        // Test hasValue() on non-existent field - should return empty
        let parsed = parser.parse("telecom.hasValue()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(false),
            "Empty/non-existent field should return false for hasValue()"
        );
    }

    #[test]
    fn test_resource_variable() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test %resource variable access
        let parsed = parser.parse("%resource.resourceType").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::String("Patient".to_string()),
            "%resource should provide access to root resource"
        );
    }

    #[test]
    fn test_resource_variable_complex_navigation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test complex navigation using %resource
        let parsed = parser
            .parse("%resource.name.where(use = 'official').family")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], FhirPathValue::String("Smith".to_string()));
            }
            FhirPathValue::String(family) => {
                assert_eq!(family, "Smith");
            }
            _ => panic!("Should return family name, got: {result:?}"),
        }
    }

    #[test]
    fn test_ucum_variable() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation_with_extensions());

        // Test %ucum variable
        let parsed = parser.parse("%ucum").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::String("http://unitsofmeasure.org".to_string()),
            "%ucum should return UCUM system URL"
        );
    }

    #[test]
    fn test_loinc_variable() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation_with_extensions());

        // Test %loinc variable in comparison
        let parsed = parser.parse("code.coding.system = %loinc").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Observation should use LOINC coding system"
        );
    }

    #[test]
    fn test_sct_variable() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation_with_extensions());

        // Test %sct variable
        let parsed = parser.parse("%sct").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::String("http://snomed.info/sct".to_string()),
            "%sct should return SNOMED CT system URL"
        );
    }

    #[test]
    fn test_extension_variables_in_complex_expression() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_observation_with_extensions());

        // Test using multiple extension variables in one expression
        let parsed = parser
            .parse("code.coding.where(system = %loinc).exists() and valueQuantity.system = %ucum")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Should validate both LOINC and UCUM usage"
        );
    }

    #[test]
    fn test_extension_function_with_no_extensions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_no_extensions());

        // Test extension() function on resource without extensions
        let parsed = parser
            .parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(
                    items.len(),
                    0,
                    "Should return empty collection when no extensions exist"
                );
            }
            _ => panic!("Should return empty collection"),
        }
    }

    #[test]
    fn test_extension_exists_validation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test common validation pattern: extension exists
        let parsed = parser.parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').exists()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Race extension should exist"
        );

        // Test non-existent extension
        let parsed2 = parser
            .parse("extension('http://nonexistent.org/extension').exists()")
            .unwrap();
        let result2 = evaluator.evaluate(&parsed2, &context).unwrap();

        assert_eq!(
            result2,
            FhirPathValue::Boolean(false),
            "Non-existent extension should not exist"
        );
    }

    #[test]
    fn test_extension_count_validation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test counting extensions
        let parsed = parser
            .parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/').count()")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        assert_eq!(
            result,
            FhirPathValue::Integer(2),
            "Should find 2 US Core extensions"
        );
    }

    #[test]
    fn test_extension_value_extraction() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test extracting extension value
        let parsed = parser
            .parse("extension('http://example.org/custom-extension').valueString")
            .unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], FhirPathValue::String("custom-value".to_string()));
            }
            FhirPathValue::String(value) => {
                assert_eq!(value, "custom-value");
            }
            _ => panic!("Should return extension value, got: {result:?}"),
        }
    }

    #[test]
    fn test_extension_chaining_with_standard_functions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test chaining extension() with standard FHIRPath functions
        let parsed = parser.parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'ombCategory').valueCoding.display.upper()").unwrap();
        let result = evaluator.evaluate(&parsed, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], FhirPathValue::String("WHITE".to_string()));
            }
            FhirPathValue::String(display) => {
                assert_eq!(display, "WHITE");
            }
            _ => panic!("Should return uppercase race display, got: {result:?}"),
        }
    }

    #[test]
    fn test_error_handling_invalid_extension_url() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Test extension() with non-string argument (should handle gracefully)
        let parsed = parser.parse("extension(123)");
        match parsed {
            Ok(ast) => {
                // If it parses, evaluation should handle the type error gracefully
                let result = evaluator.evaluate(&ast, &context);
                match result {
                    Ok(FhirPathValue::Collection(items)) => {
                        assert_eq!(
                            items.len(),
                            0,
                            "Should return empty collection for invalid URL type"
                        );
                    }
                    Err(_) => {
                        // Error is also acceptable for invalid argument type
                    }
                    _ => panic!("Should return empty collection or error for invalid URL type"),
                }
            }
            Err(_) => {
                // Parse error is also acceptable for invalid syntax
            }
        }
    }

    #[test]
    fn test_extension_function_case_sensitivity() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient_with_extensions());

        // Extension URLs should be case-sensitive
        let parsed1 = parser
            .parse("extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')")
            .unwrap();
        let result1 = evaluator.evaluate(&parsed1, &context).unwrap();

        let parsed2 = parser
            .parse("extension('HTTP://HL7.ORG/FHIR/US/CORE/STRUCTUREDEFINITION/US-CORE-RACE')")
            .unwrap();
        let result2 = evaluator.evaluate(&parsed2, &context).unwrap();

        match (result1, result2) {
            (FhirPathValue::Collection(items1), FhirPathValue::Collection(items2)) => {
                assert_eq!(items1.len(), 1, "Correct case should find extension");
                assert_eq!(items2.len(), 0, "Wrong case should not find extension");
            }
            _ => panic!("Both should return collections"),
        }
    }
}
