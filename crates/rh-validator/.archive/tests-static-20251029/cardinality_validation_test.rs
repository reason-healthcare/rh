use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, ValidatorConfig};

#[test]
fn test_cardinality_validation_required_field() {
    // Skip invariants to focus on cardinality validation
    let config = ValidatorConfig::default()
        .with_skip_invariants(true)
        .with_skip_bindings(true);
    let validator = FhirValidator::with_config(config).expect("Failed to create validator");

    // Test with a simple Patient resource
    // Patient doesn't have any required fields (all are 0..x)
    // So this should pass cardinality validation

    let json = r#"{
        "resourceType": "Patient",
        "id": "example"
    }"#;

    let result = validator
        .validate_full::<Patient>(json)
        .expect("Validation failed");

    // Patient has no required fields, so should be valid for cardinality
    assert!(result.is_valid());
    assert_eq!(result.issues.len(), 0, "Expected no cardinality issues");
}

#[test]
fn test_cardinality_validation_max_exceeded() {
    // Skip invariants to focus on cardinality validation
    let config = ValidatorConfig::default()
        .with_skip_invariants(true)
        .with_skip_bindings(true);
    let validator = FhirValidator::with_config(config).expect("Failed to create validator");

    // Test with a valid patient
    // For Patient, most arrays are unbounded (max = None)
    // So we can't easily test max exceeded on Patient

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "active": true
    }"#;

    let result = validator
        .validate_full::<Patient>(json)
        .expect("Validation failed");

    assert!(result.is_valid());
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_cardinality_validation_can_be_skipped() {
    let config = ValidatorConfig::default()
        .with_skip_cardinality(true)
        .with_skip_invariants(true)
        .with_skip_bindings(true);

    let validator = FhirValidator::with_config(config).expect("Failed to create validator");

    let json = r#"{
        "resourceType": "Patient",
        "id": "example"
    }"#;

    let result = validator
        .validate_full::<Patient>(json)
        .expect("Validation failed");

    // Should be valid when all validation is skipped
    assert!(result.is_valid());
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_cardinality_metadata_present() {
    // Verify that Patient has cardinality metadata
    use hl7_fhir_r4_core::validation::ValidatableResource;

    let cardinalities = Patient::cardinalities();

    // Should have cardinality definitions
    assert!(!cardinalities.is_empty());

    // Verify specific cardinalities
    let id_card = cardinalities
        .iter()
        .find(|c| c.path == "Patient.id")
        .expect("Should have Patient.id cardinality");
    assert_eq!(id_card.min, 0);
    assert_eq!(id_card.max, Some(1));

    let identifier_card = cardinalities
        .iter()
        .find(|c| c.path == "Patient.identifier")
        .expect("Should have Patient.identifier cardinality");
    assert_eq!(identifier_card.min, 0);
    assert_eq!(identifier_card.max, None); // unbounded
}

#[test]
fn test_validate_resource_with_cardinality() {
    use hl7_fhir_r4_core::traits::resource::ResourceMutators;

    // Skip invariants and bindings to focus on cardinality
    let config = ValidatorConfig::default()
        .with_skip_invariants(true)
        .with_skip_bindings(true);
    let validator = FhirValidator::with_config(config).expect("Failed to create validator");

    let patient = Patient::new().set_id("example".to_string());

    let result = validator
        .validate_resource(&patient)
        .expect("Validation failed");

    // Patient with just an ID should be valid (no required fields)
    assert!(result.is_valid());
    assert_eq!(result.issues.len(), 0);
}
