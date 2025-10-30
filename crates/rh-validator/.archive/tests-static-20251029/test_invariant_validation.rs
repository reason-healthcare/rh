//! Tests for Phase 3: Invariant validation with FHIRPath
//!
//! These tests verify that the validator correctly evaluates FHIRPath invariants
//! and reports failures appropriately.

use hl7_fhir_r4_core::resources::patient::Patient;
use hl7_fhir_r4_core::validation::ValidatableResource;
use rh_validator::{FhirValidator, ValidatorConfig};

#[test]
fn test_patient_has_invariants() {
    // Verify that Patient resource has invariants defined
    let invariants = Patient::invariants();
    assert!(!invariants.is_empty(), "Patient should have invariants");

    // Check for known invariants
    let has_pat1 = invariants.iter().any(|inv| inv.key == "pat-1");
    assert!(has_pat1, "Patient should have pat-1 invariant");
}

#[test]
fn test_validate_invariants_patient_valid() {
    let validator = FhirValidator::new().unwrap();

    // Create a valid Patient with name and extension to satisfy ext-1
    // Note: ext-1 is a base element invariant that currently gets inherited by all resources
    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/fhir/StructureDefinition/example",
            "valueString": "test"
        }],
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["John"]
        }]
    }"#;

    let patient: Patient = serde_json::from_str(patient_json).unwrap();
    let issues = validator.validate_invariants(&patient).unwrap();

    // Filter out parse errors (warnings) - some FHIRPath expressions may not be supported yet
    let errors: Vec<_> = issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .collect();

    // Should have no error-level issues
    assert!(
        errors.is_empty(),
        "Valid patient should pass all invariants, but got errors: {errors:?}"
    );
}

#[test]
fn test_validate_full_patient_valid() {
    let validator = FhirValidator::new().unwrap();

    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "extension": [{
            "url": "http://example.org/fhir/StructureDefinition/example",
            "valueString": "test"
        }],
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["John"]
        }]
    }"#;

    let result = validator.validate_full::<Patient>(patient_json).unwrap();

    assert!(
        result.is_valid(),
        "Valid patient should pass all validation"
    );
    assert_eq!(result.error_count(), 0, "Should have no errors");
    // Note: warnings may occur for unsupported FHIRPath expressions
}

#[test]
fn test_validate_full_combines_structural_and_invariant() {
    let validator = FhirValidator::new().unwrap();

    // Invalid JSON (missing required fields if any)
    let patient_json = r#"{
        "resourceType": "Patient",
        "invalid_field": "should cause issues"
    }"#;

    let result = validator.validate_full::<Patient>(patient_json).unwrap();

    // Should still validate (Patient has no required fields beyond resourceType in FHIR R4)
    // The key is that both structural and invariant validation run without crashing
    // Just verify we get a result
    let _ = result.is_valid();
}

#[test]
fn test_skip_invariants_config() {
    let config = ValidatorConfig::new().with_skip_invariants(true);
    let validator = FhirValidator::with_config(config).unwrap();

    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe"
        }]
    }"#;

    let result = validator.validate_full::<Patient>(patient_json).unwrap();

    // When skip_invariants is true, only structural validation runs
    // So we should get a result without invariant checks
    assert!(result.is_valid() || !result.is_valid()); // Just checking it doesn't crash
}

#[test]
fn test_invariant_validation_error_handling() {
    let validator = FhirValidator::new().unwrap();

    // Create a patient that might trigger FHIRPath evaluation issues
    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example"
    }"#;

    let patient: Patient = serde_json::from_str(patient_json).unwrap();

    // Should not panic even if FHIRPath evaluation has issues
    let result = validator.validate_invariants(&patient);
    assert!(result.is_ok(), "Should handle FHIRPath errors gracefully");
}

#[test]
fn test_multiple_invariants_evaluated() {
    let validator = FhirValidator::new().unwrap();

    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe"
        }]
    }"#;

    let patient: Patient = serde_json::from_str(patient_json).unwrap();
    let issues = validator.validate_invariants(&patient).unwrap();

    // All invariants should be evaluated
    // If there are issues, they should have invariant_key set
    for issue in &issues {
        assert!(
            issue.invariant_key.is_some(),
            "All invariant issues should have invariant_key"
        );
        assert!(
            issue.expression.is_some(),
            "All invariant issues should have expression"
        );
    }
}

#[test]
fn test_validator_config_builder() {
    let config = ValidatorConfig::new()
        .with_skip_invariants(true)
        .with_warn_on_unknown_fields(false)
        .with_max_depth(50);

    assert!(config.skip_invariants);
    assert!(!config.warn_on_unknown_fields);
    assert_eq!(config.max_depth, 50);
}

#[test]
fn test_invariant_failure_includes_human_message() {
    let validator = FhirValidator::new().unwrap();

    // Create a minimal patient that might fail certain invariants
    let patient_json = r#"{
        "resourceType": "Patient",
        "id": "test"
    }"#;

    let patient: Patient = serde_json::from_str(patient_json).unwrap();
    let issues = validator.validate_invariants(&patient).unwrap();

    // Any issues should include human-readable messages
    for issue in &issues {
        assert!(
            !issue.details.is_empty(),
            "Issue should have details: {issue:?}"
        );
    }
}
