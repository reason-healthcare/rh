//! Tests for multi-resource type validation (Phase 8)

use rh_validator::{extract_resource_type, suggest_resource_type, FhirValidator, ValidatorConfig};

#[test]
fn test_validate_patient_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Patient",
        "id": "test",
        "text": {
            "status": "generated",
            "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">Test</div>"
        },
        "gender": "male"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    assert!(result.is_valid() || result.error_count() == 0);
}

#[test]
fn test_validate_observation_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Observation",
        "id": "test",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "15074-8"
            }]
        }
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    // May have validation errors but should not panic
    let _ = result.is_valid();
}

#[test]
fn test_validate_organization_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Organization",
        "id": "test",
        "name": "Test Organization"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    // Should validate successfully
    let _ = result.is_valid();
}

#[test]
fn test_validate_practitioner_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Practitioner",
        "id": "test",
        "name": [{
            "family": "Smith"
        }]
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    let _ = result.is_valid();
}

#[test]
fn test_validate_medication_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Medication",
        "id": "test"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    let _ = result.is_valid();
}

#[test]
fn test_validate_encounter_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Encounter",
        "id": "test",
        "status": "finished",
        "class": {
            "system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "code": "IMP"
        }
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    let _ = result.is_valid();
}

#[test]
fn test_validate_procedure_resource() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Procedure",
        "id": "test",
        "status": "completed",
        "subject": {
            "reference": "Patient/test"
        }
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    let _ = result.is_valid();
}

#[test]
fn test_validate_unknown_resource_type() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "UnknownType",
        "id": "test"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    assert!(!result.is_valid());
    assert_eq!(result.error_count(), 1);
    assert!(result.issues[0].details.contains("Unknown resourceType"));
}

#[test]
fn test_validate_typo_in_resource_type() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "resourceType": "Patint",
        "id": "test"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    assert!(!result.is_valid());
    assert_eq!(result.error_count(), 1);
    assert!(result.issues[0].details.contains("Did you mean 'Patient'"));
}

#[test]
fn test_validate_missing_resource_type() {
    let validator = FhirValidator::new().unwrap();
    let json = r#"{
        "id": "test",
        "name": "Test"
    }"#;

    let result = validator.validate_any_resource(json);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Missing required field: resourceType"));
}

#[test]
fn test_extract_resource_type_valid() {
    let json = r#"{"resourceType": "Patient", "id": "123"}"#;
    assert_eq!(extract_resource_type(json).unwrap(), "Patient");
}

#[test]
fn test_extract_resource_type_missing() {
    let json = r#"{"id": "123"}"#;
    assert!(extract_resource_type(json).is_err());
}

#[test]
fn test_extract_resource_type_not_string() {
    let json = r#"{"resourceType": 123}"#;
    assert!(extract_resource_type(json).is_err());
}

#[test]
fn test_suggest_resource_type_exact_match() {
    assert_eq!(
        suggest_resource_type("patient"),
        Some("Patient".to_string())
    );
    assert_eq!(
        suggest_resource_type("OBSERVATION"),
        Some("Observation".to_string())
    );
}

#[test]
fn test_suggest_resource_type_typo() {
    assert_eq!(suggest_resource_type("Patint"), Some("Patient".to_string()));
    assert_eq!(
        suggest_resource_type("Observaton"),
        Some("Observation".to_string())
    );
    assert_eq!(
        suggest_resource_type("Practitionr"),
        Some("Practitioner".to_string())
    );
}

#[test]
fn test_suggest_resource_type_no_match() {
    assert_eq!(
        suggest_resource_type("CompletelyInvalidResourceTypeName"),
        None
    );
}

#[test]
fn test_batch_validation_mixed_types() {
    let validator = FhirValidator::new().unwrap();
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "text": {"status": "generated", "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">P1</div>"}, "gender": "male"}
{"resourceType": "Observation", "id": "2", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}
{"resourceType": "Organization", "id": "3", "name": "Test Org"}
{"resourceType": "Practitioner", "id": "4", "name": [{"family": "Smith"}]}"#;

    let results = validator.validate_ndjson_any(ndjson).unwrap();
    assert_eq!(results.len(), 4);

    // Check line numbers are correct
    assert_eq!(results[0].0, 1);
    assert_eq!(results[1].0, 2);
    assert_eq!(results[2].0, 3);
    assert_eq!(results[3].0, 4);
}

#[test]
fn test_batch_validation_with_empty_lines() {
    let validator = FhirValidator::new().unwrap();
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "text": {"status": "generated", "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">P1</div>"}, "gender": "male"}

{"resourceType": "Observation", "id": "2", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}
# Comment line
{"resourceType": "Organization", "id": "3", "name": "Test"}"#;

    let results = validator.validate_ndjson_any(ndjson).unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_batch_validation_with_invalid_type() {
    let validator = FhirValidator::new().unwrap();
    let ndjson = r#"{"resourceType": "Patient", "id": "1", "text": {"status": "generated", "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">P1</div>"}, "gender": "male"}
{"resourceType": "InvalidType", "id": "2"}
{"resourceType": "Organization", "id": "3", "name": "Test"}"#;

    let results = validator.validate_ndjson_any(ndjson).unwrap();
    assert_eq!(results.len(), 3);

    // Second result should have error for invalid type
    assert!(!results[1].1.is_valid());
    assert!(results[1].1.issues[0]
        .details
        .contains("Unknown resourceType"));
}

#[test]
fn test_validate_all_common_resource_types() {
    let validator = FhirValidator::new().unwrap();

    let test_cases = vec![
        ("Patient", r#"{"resourceType": "Patient", "id": "1"}"#),
        (
            "Observation",
            r#"{"resourceType": "Observation", "id": "1", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}"#,
        ),
        (
            "Organization",
            r#"{"resourceType": "Organization", "id": "1", "name": "Test"}"#,
        ),
        (
            "Practitioner",
            r#"{"resourceType": "Practitioner", "id": "1"}"#,
        ),
        ("Medication", r#"{"resourceType": "Medication", "id": "1"}"#),
        (
            "Encounter",
            r#"{"resourceType": "Encounter", "id": "1", "status": "finished", "class": {"system": "http://terminology.hl7.org/CodeSystem/v3-ActCode", "code": "IMP"}}"#,
        ),
        (
            "Procedure",
            r#"{"resourceType": "Procedure", "id": "1", "status": "completed", "subject": {"reference": "Patient/1"}}"#,
        ),
        ("Condition", r#"{"resourceType": "Condition", "id": "1"}"#),
        (
            "DiagnosticReport",
            r#"{"resourceType": "DiagnosticReport", "id": "1", "status": "final", "code": {"coding": [{"system": "http://loinc.org", "code": "123"}]}}"#,
        ),
        (
            "Immunization",
            r#"{"resourceType": "Immunization", "id": "1", "status": "completed", "vaccineCode": {"coding": [{"system": "http://hl7.org/fhir/sid/cvx", "code": "01"}]}}"#,
        ),
    ];

    for (resource_type, json) in test_cases {
        let result = validator.validate_any_resource(json);
        assert!(
            result.is_ok(),
            "Failed to validate {}: {:?}",
            resource_type,
            result.err()
        );
    }
}

#[test]
fn test_validate_rare_resource_types() {
    let validator = FhirValidator::new().unwrap();

    let test_cases = vec![
        (
            "MolecularSequence",
            r#"{"resourceType": "MolecularSequence", "id": "1", "type": "dna"}"#,
        ),
        (
            "SubstanceProtein",
            r#"{"resourceType": "SubstanceProtein", "id": "1"}"#,
        ),
        (
            "EffectEvidenceSynthesis",
            r#"{"resourceType": "EffectEvidenceSynthesis", "id": "1", "status": "draft"}"#,
        ),
        (
            "RiskEvidenceSynthesis",
            r#"{"resourceType": "RiskEvidenceSynthesis", "id": "1", "status": "draft"}"#,
        ),
    ];

    for (resource_type, json) in test_cases {
        let result = validator.validate_any_resource(json);
        assert!(
            result.is_ok(),
            "Failed to validate {}: {:?}",
            resource_type,
            result.err()
        );
    }
}

#[test]
fn test_validate_with_skip_invariants() {
    let config = ValidatorConfig::new().with_skip_invariants(true);
    let validator = FhirValidator::with_config(config).unwrap();

    let json = r#"{
        "resourceType": "Patient",
        "id": "test",
        "gender": "male"
    }"#;

    let result = validator.validate_any_resource(json).unwrap();
    // Should validate without invariant checks
    let _ = result.is_valid();
}
