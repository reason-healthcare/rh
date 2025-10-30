//! Integration tests for FHIR resource validation

use rh_validator::FhirValidator;

#[test]
fn test_valid_patient_resource() {
    let validator = FhirValidator::new().unwrap();

    let valid_patient = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(valid_patient)
        .unwrap();

    assert!(
        result.is_valid(),
        "Expected valid patient to pass validation"
    );
    assert_eq!(result.error_count(), 0);
    assert_eq!(result.resource_type, "Patient");
}

#[test]
fn test_invalid_json_syntax() {
    let validator = FhirValidator::new().unwrap();

    let invalid_json = r#"{
        "resourceType": "Patient",
        "id": "example"
        "name": "missing comma"
    }"#;

    let result = validator.validate_json::<serde_json::Value>(invalid_json);

    assert!(result.is_err(), "Expected JSON parse error");
}

#[test]
fn test_valid_observation_resource() {
    let validator = FhirValidator::new().unwrap();

    let valid_observation = r#"{
        "resourceType": "Observation",
        "id": "example",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "15074-8"
            }]
        }
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(valid_observation)
        .unwrap();

    assert!(result.is_valid(), "Expected valid observation to pass");
    assert_eq!(result.resource_type, "Observation");
}

#[test]
fn test_valid_bundle_resource() {
    let validator = FhirValidator::new().unwrap();

    let valid_bundle = r#"{
        "resourceType": "Bundle",
        "id": "example",
        "type": "searchset",
        "total": 0,
        "entry": []
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(valid_bundle)
        .unwrap();

    assert!(result.is_valid(), "Expected valid bundle to pass");
    assert_eq!(result.resource_type, "Bundle");
}

#[test]
fn test_unknown_resource_type() {
    let validator = FhirValidator::new().unwrap();

    let unknown_resource = r#"{
        "resourceType": "UnknownResource",
        "id": "example"
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(unknown_resource)
        .unwrap();

    // Should still parse as JSON, even if resource type is unknown
    assert_eq!(result.resource_type, "UnknownResource");
}

#[test]
fn test_multiple_resources_ndjson() {
    let validator = FhirValidator::new().unwrap();

    let ndjson = r#"{"resourceType": "Patient", "id": "1"}
{"resourceType": "Patient", "id": "2"}
{"resourceType": "Observation", "id": "3"}"#;

    let results = validator.validate_multiple(ndjson, None).unwrap();

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|(_, r)| r.is_valid()));
}

#[test]
fn test_validation_result_serialization() {
    let validator = FhirValidator::new().unwrap();

    let patient = r#"{
        "resourceType": "Patient",
        "id": "example"
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(patient)
        .unwrap();

    // Ensure the result can be serialized to JSON
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("Patient"));
    // Note: resource_id is not automatically extracted in Phase 1
    // Phase 3 will add this functionality
}

#[test]
fn test_empty_json_object() {
    let validator = FhirValidator::new().unwrap();

    let empty = r#"{}"#;

    let result = validator.validate_json::<serde_json::Value>(empty).unwrap();

    // Should parse successfully, resource_type will be "Unknown"
    assert_eq!(result.resource_type, "Unknown");
}

#[test]
fn test_deeply_nested_resource() {
    let validator = FhirValidator::new().unwrap();

    let nested = r#"{
        "resourceType": "Bundle",
        "entry": [{
            "resource": {
                "resourceType": "Patient",
                "name": [{
                    "given": ["John"],
                    "family": "Doe"
                }],
                "contact": [{
                    "name": {
                        "given": ["Jane"]
                    }
                }]
            }
        }]
    }"#;

    let result = validator
        .validate_json::<serde_json::Value>(nested)
        .unwrap();

    assert!(result.is_valid(), "Expected nested resource to be valid");
}
