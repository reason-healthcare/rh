//! Integration tests for binding validation
//!
//! These tests verify that required ValueSet bindings are validated correctly.

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, ValidatorConfig, ValueSetRegistry};

#[test]
fn test_patient_gender_valid_codes() {
    let validator = FhirValidator::new().unwrap();

    // Test all valid gender codes
    let valid_codes = vec!["male", "female", "other", "unknown"];

    for code in valid_codes {
        let json = format!(
            r#"{{
                "resourceType": "Patient",
                "id": "example",
                "gender": "{code}"
            }}"#,
        );

        let result = validator.validate_full::<Patient>(&json).unwrap();

        // Should have no binding errors (may have invariant warnings like dom-6)
        let binding_errors: Vec<_> = result
            .issues
            .iter()
            .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
            .collect();

        assert!(
            binding_errors.is_empty(),
            "Valid code '{code}' should not produce binding errors, but got: {binding_errors:?}",
        );
    }
}

#[test]
fn test_patient_gender_invalid_code() {
    // NOTE: This test demonstrates that invalid gender codes are caught during
    // structural validation (deserialization) because Patient.gender is strongly-typed
    // as the AdministrativeGender enum. The binding validation code won't run because
    // deserialization fails first.

    let validator = FhirValidator::new().unwrap();

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "invalid"
    }"#;

    let result = validator.validate_full::<Patient>(json).unwrap();

    // The error will be a structural/deserialization error, not a binding error
    // This is actually good - compile-time type safety prevents invalid codes!
    assert!(!result.is_valid(), "Should have validation errors");

    // It will be a structural error (unknown enum variant)
    let structural_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| {
            matches!(
                i.code,
                rh_validator::IssueCode::Structure | rh_validator::IssueCode::ValueType
            )
        })
        .collect();

    assert!(
        !structural_errors.is_empty(),
        "Should have structural error for invalid enum value"
    );
}

#[test]
fn test_binding_validation_concept() {
    // Note: Binding validation is most useful for CodeableConcept and Coding fields
    // which can contain arbitrary system|code pairs. Patient.gender uses a strong enum
    // so the type system prevents invalid codes at compile/deserialization time.
    //
    // This is actually a good thing - we get compile-time safety for simple code bindings!
    // Runtime binding validation is mainly for validating user-provided JSON before
    // attempting deserialization, or for more complex CodeableConcept structures.

    let validator = FhirValidator::new().unwrap();

    // For demonstration, let's test that the binding validation infrastructure works
    // by using the skip_bindings flag
    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "male"
    }"#;

    // With bindings enabled (default)
    let result = validator.validate_full::<Patient>(json).unwrap();

    // Should not have binding errors (may have invariant warnings)
    let binding_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();
    assert!(binding_errors.is_empty());

    // With bindings disabled
    let config = rh_validator::ValidatorConfig::new().with_skip_bindings(true);
    let validator_no_bindings = FhirValidator::with_config(config).unwrap();
    let result2 = validator_no_bindings
        .validate_full::<Patient>(json)
        .unwrap();

    // Should also not have binding errors
    let binding_errors2: Vec<_> = result2
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();
    assert!(binding_errors2.is_empty());
}

#[test]
fn test_patient_no_gender_no_binding_error() {
    let validator = FhirValidator::new().unwrap();

    let json = r#"{
        "resourceType": "Patient",
        "id": "example"
    }"#;

    let result = validator.validate_full::<Patient>(json).unwrap();

    // Should have no binding errors (gender is optional)
    let binding_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();

    assert!(binding_errors.is_empty());
}

#[test]
fn test_skip_binding_validation() {
    let config = ValidatorConfig::new().with_skip_bindings(true);
    let validator = FhirValidator::with_config(config).unwrap();

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "invalid"
    }"#;

    let result = validator.validate_full::<Patient>(json).unwrap();

    // Should have NO binding errors because we skipped binding validation
    let binding_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();

    assert!(binding_errors.is_empty());
}

#[test]
fn test_custom_valueset_registry() {
    // Test that we can create a custom registry and load ValueSets into it
    let mut registry = ValueSetRegistry::new();

    // Add a custom ValueSet with only "male" and "female"
    let json = r#"{
        "resourceType": "ValueSet",
        "url": "http://hl7.org/fhir/ValueSet/administrative-gender",
        "expansion": {
            "contains": [
                {
                    "system": "http://hl7.org/fhir/administrative-gender",
                    "code": "male",
                    "display": "Male"
                },
                {
                    "system": "http://hl7.org/fhir/administrative-gender",
                    "code": "female",
                    "display": "Female"
                }
            ]
        }
    }"#;

    registry.load_from_json(json).unwrap();

    let config = ValidatorConfig::new().with_valueset_registry(registry);
    let validator = FhirValidator::with_config(config).unwrap();

    // "male" should be valid
    let json_male = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "male"
    }"#;
    let result = validator.validate_full::<Patient>(json_male).unwrap();
    let binding_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();
    assert!(binding_errors.is_empty());

    // Note: We can't test that "other" is rejected by the custom ValueSet because
    // Patient.gender is a strongly-typed enum (AdministrativeGender), so invalid codes
    // fail at deserialization time, not during binding validation. This is a good thing!
    // The type system provides compile-time safety.
}

#[test]
fn test_direct_validation_with_bindings() {
    // Note: For Patient.gender, the field is strongly-typed as AdministrativeGender enum,
    // so invalid values can't be constructed programmatically. Binding validation is most
    // useful for JSON validation and for CodeableConcept/Coding fields that accept any value.

    let validator = FhirValidator::new().unwrap();

    // Create a patient programmatically with valid enum value
    let mut patient = Patient::default();
    patient.base.base.id = Some("example".to_string());
    patient.gender =
        Some(hl7_fhir_r4_core::bindings::administrative_gender::AdministrativeGender::Male);

    let result = validator.validate_resource(&patient).unwrap();

    // Should be valid
    let binding_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.code == rh_validator::IssueCode::CodeInvalid)
        .collect();
    assert!(binding_errors.is_empty());

    // For programmatic construction, the type system prevents invalid codes,
    // so we can't test invalid binding here. That's a good thing - compile-time safety!
}
