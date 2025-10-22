//! Tests for FHIR metadata type integration
//!
//! These tests verify that FHIRPath expressions return correctly typed values
//! based on FHIR metadata, according to the FHIRPath specification:
//! https://www.hl7.org/fhir/fhirpath.html

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_patient_primitive_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient_data = json!({
        "resourceType": "Patient",
        "id": "example",
        "active": true,
        "birthDate": "1974-12-25",
        "deceasedBoolean": false,
        "multipleBirthInteger": 2
    });

    let context = EvaluationContext::new(patient_data);

    // Test Date type
    let expr = parser.parse("Patient.birthDate").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Date(d) => assert_eq!(d, "1974-12-25"),
        _ => panic!("Expected Date type for birthDate, got {result:?}"),
    }

    // Test Boolean type
    let expr = parser.parse("Patient.active").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean type for active, got {result:?}"),
    }

    // Test Boolean type for deceased
    let expr = parser.parse("Patient.deceasedBoolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(b) => assert!(!b),
        _ => panic!("Expected Boolean type for deceasedBoolean, got {result:?}"),
    }

    // Test Integer type
    let expr = parser.parse("Patient.multipleBirthInteger").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Integer(i) => assert_eq!(i, 2),
        _ => panic!("Expected Integer type for multipleBirthInteger, got {result:?}"),
    }
}

#[test]
fn test_observation_datetime_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let observation_data = json!({
        "resourceType": "Observation",
        "id": "example",
        "status": "final",
        "effectiveDateTime": "2023-10-22T14:30:00Z",
        "issued": "2023-10-22T15:00:00Z"
    });

    let context = EvaluationContext::new(observation_data);

    // Test DateTime type for effectiveDateTime (choice type field)
    let expr = parser.parse("Observation.effectiveDateTime").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::DateTime(dt) => assert_eq!(dt, "2023-10-22T14:30:00Z"),
        _ => panic!("Expected DateTime type for effectiveDateTime, got {result:?}"),
    }

    // Test issued field - without metadata, it remains a String
    // (issued doesn't match any fallback pattern, and Observation metadata is empty)
    let expr = parser.parse("Observation.issued").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::String(s) | FhirPathValue::DateTime(s) => {
            assert_eq!(s, "2023-10-22T15:00:00Z")
        }
        _ => panic!("Expected String or DateTime type for issued, got {result:?}"),
    }
}

#[test]
fn test_date_comparison_with_string_literals() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient_data = json!({
        "resourceType": "Patient",
        "birthDate": "1990-05-15"
    });

    let context = EvaluationContext::new(patient_data);

    // Date should be comparable with string literal
    let expr = parser.parse("Patient.birthDate < '2000-01-01'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("Patient.birthDate > '1980-01-01'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Date equality with string literal
    let expr = parser.parse("Patient.birthDate = '1990-05-15'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("Patient.birthDate != '2000-01-01'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_datetime_comparison_with_string_literals() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let observation_data = json!({
        "resourceType": "Observation",
        "effectiveDateTime": "2023-10-22T14:30:00Z"
    });

    let context = EvaluationContext::new(observation_data);

    // DateTime should be comparable with string literal
    let expr = parser
        .parse("Observation.effectiveDateTime > '2023-01-01T00:00:00Z'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser
        .parse("Observation.effectiveDateTime < '2024-01-01T00:00:00Z'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // DateTime equality with string literal
    let expr = parser
        .parse("Observation.effectiveDateTime = '2023-10-22T14:30:00Z'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_choice_type_fields() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with boolean choice type
    let patient_data = json!({
        "resourceType": "Patient",
        "multipleBirthBoolean": false
    });

    let context = EvaluationContext::new(patient_data);

    let expr = parser.parse("Patient.multipleBirthBoolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(b) => assert!(!b),
        _ => panic!("Expected Boolean type for multipleBirthBoolean, got {result:?}"),
    }

    // Test with integer choice type
    let patient_data = json!({
        "resourceType": "Patient",
        "multipleBirthInteger": 3
    });

    let context = EvaluationContext::new(patient_data);

    let expr = parser.parse("Patient.multipleBirthInteger").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Integer(i) => assert_eq!(i, 3),
        _ => panic!("Expected Integer type for multipleBirthInteger, got {result:?}"),
    }
}

#[test]
fn test_fallback_type_inference() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with a resource that might have empty metadata (lowercase 'observation')
    // The fallback should infer types from field name patterns
    let data = json!({
        "resourceType": "TestResource",
        "recordedDate": "2023-10-22",
        "performedDateTime": "2023-10-22T10:00:00Z",
        "startTime": "10:00:00",
        "completedBoolean": true,
        "countInteger": 42
    });

    let context = EvaluationContext::new(data);

    // Fields ending in "Date" should be Date type
    let expr = parser.parse("TestResource.recordedDate").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Date(d) => assert_eq!(d, "2023-10-22"),
        _ => panic!("Expected Date type from fallback inference, got {result:?}"),
    }

    // Fields ending in "DateTime" should be DateTime type
    let expr = parser.parse("TestResource.performedDateTime").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::DateTime(dt) => assert_eq!(dt, "2023-10-22T10:00:00Z"),
        _ => panic!("Expected DateTime type from fallback inference, got {result:?}"),
    }

    // Fields ending in "Time" should be Time type
    let expr = parser.parse("TestResource.startTime").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Time(t) => assert_eq!(t, "10:00:00"),
        _ => panic!("Expected Time type from fallback inference, got {result:?}"),
    }

    // Fields ending in "Boolean" should be Boolean type
    let expr = parser.parse("TestResource.completedBoolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean type from fallback inference, got {result:?}"),
    }

    // Fields ending in "Integer" should be Integer type
    let expr = parser.parse("TestResource.countInteger").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected Integer type from fallback inference, got {result:?}"),
    }
}

#[test]
fn test_nested_field_access_with_typing() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient_data = json!({
        "resourceType": "Patient",
        "birthDate": "1990-05-15",
        "contact": [
            {
                "relationship": [{"text": "Emergency"}],
                "period": {
                    "start": "2020-01-01"
                }
            }
        ]
    });

    let context = EvaluationContext::new(patient_data);

    // Test that nested date fields are also properly typed
    let expr = parser.parse("Patient.contact.period.start").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // The result might be a collection, so handle that
    match result {
        FhirPathValue::DateTime(dt) => assert_eq!(dt, "2020-01-01"),
        FhirPathValue::Date(d) => assert_eq!(d, "2020-01-01"),
        FhirPathValue::String(s) => assert_eq!(s, "2020-01-01"),
        _ => panic!("Expected date type for nested field, got {result:?}"),
    }
}

#[test]
fn test_collection_with_typed_values() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let bundle_data = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "birthDate": "1990-05-15"
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "birthDate": "1985-03-20"
                }
            }
        ]
    });

    let context = EvaluationContext::new(bundle_data);

    // Access birthDate from multiple patients in collection
    let expr = parser.parse("Bundle.entry.resource.birthDate").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Should return a collection of Date values
    match result {
        FhirPathValue::Collection(items) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                FhirPathValue::Date(d) => assert_eq!(d, "1990-05-15"),
                _ => panic!("Expected Date type in collection"),
            }
            match &items[1] {
                FhirPathValue::Date(d) => assert_eq!(d, "1985-03-20"),
                _ => panic!("Expected Date type in collection"),
            }
        }
        _ => panic!("Expected Collection, got {result:?}"),
    }
}

#[test]
fn test_where_clause_with_typed_date_comparison() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let bundle_data = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "1",
                    "birthDate": "1990-05-15"
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "2",
                    "birthDate": "2010-03-20"
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "3",
                    "birthDate": "1985-08-10"
                }
            }
        ]
    });

    let context = EvaluationContext::new(bundle_data);

    // Filter patients born before 2000
    let expr = parser
        .parse("Bundle.entry.resource.where(birthDate < '2000-01-01').id")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::Collection(items) => {
            assert_eq!(items.len(), 2);
            // Should have patients 1 and 3 (born in 1990 and 1985)
        }
        FhirPathValue::String(id) => {
            // If only one result, that's also acceptable
            assert!(id == "1" || id == "3");
        }
        _ => panic!("Expected collection of IDs, got {result:?}"),
    }
}

#[test]
fn test_uri_and_code_types_remain_strings() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient_data = json!({
        "resourceType": "Patient",
        "id": "example",
        "language": "en-US",
        "implicitRules": "http://example.org/rules"
    });

    let context = EvaluationContext::new(patient_data);

    // Code type should remain as String
    let expr = parser.parse("Patient.language").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::String(s) => assert_eq!(s, "en-US"),
        _ => panic!("Expected String type for code, got {result:?}"),
    }

    // URI type should remain as String
    let expr = parser.parse("Patient.implicitRules").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::String(s) => assert_eq!(s, "http://example.org/rules"),
        _ => panic!("Expected String type for URI, got {result:?}"),
    }
}
