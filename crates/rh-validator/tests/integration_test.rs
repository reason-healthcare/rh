use rh_validator::{FhirValidator, ProfileRegistry, Severity};
use serde_json::json;

#[test]
fn test_profile_registry_creation() {
    let registry = ProfileRegistry::new(None);
    assert!(registry.is_ok());
}

#[test]
fn test_validator_creation() {
    let validator = FhirValidator::new(None);
    assert!(validator.is_ok());
}

#[test]
fn test_basic_validation_missing_resource_type() {
    let validator = FhirValidator::new(None).unwrap();
    let resource = json!({
        "id": "test"
    });

    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
    assert_eq!(result.error_count(), 1);
    assert!(result.issues[0].message.contains("resourceType"));
}

#[test]
fn test_basic_validation_valid_structure() {
    let validator = FhirValidator::new(None).unwrap();
    let resource = json!({
        "resourceType": "Patient",
        "id": "example"
    });

    let result = validator.validate(&resource).unwrap();
    assert!(result.valid);
    assert_eq!(result.error_count(), 0);
}

#[test]
fn test_basic_validation_not_json_object() {
    let validator = FhirValidator::new(None).unwrap();
    let resource = json!("not an object");

    let result = validator.validate(&resource).unwrap();
    assert!(!result.valid);
    assert_eq!(result.error_count(), 1);
    assert!(result.issues[0].message.contains("JSON object"));
}

#[test]
fn test_profile_url_extraction() {
    let resource = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": [
                "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
            ]
        }
    });

    let urls = ProfileRegistry::extract_profile_urls(&resource);
    assert_eq!(urls.len(), 1);
    assert_eq!(
        urls[0],
        "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
    );
}

#[test]
fn test_profile_url_extraction_multiple() {
    let resource = json!({
        "resourceType": "Observation",
        "meta": {
            "profile": [
                "http://hl7.org/fhir/StructureDefinition/vitalsigns",
                "http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation"
            ]
        }
    });

    let urls = ProfileRegistry::extract_profile_urls(&resource);
    assert_eq!(urls.len(), 2);
}

#[test]
fn test_profile_url_extraction_no_meta() {
    let resource = json!({
        "resourceType": "Patient",
        "id": "example"
    });

    let urls = ProfileRegistry::extract_profile_urls(&resource);
    assert_eq!(urls.len(), 0);
}

#[test]
fn test_validation_result_helpers() {
    use rh_validator::{IssueCode, ValidationIssue, ValidationResult};

    let mut result = ValidationResult::valid();
    assert!(result.valid);
    assert_eq!(result.error_count(), 0);

    result = result.with_issue(ValidationIssue::error(IssueCode::Required, "Missing field"));
    assert!(!result.valid);
    assert_eq!(result.error_count(), 1);

    result = result.with_issue(ValidationIssue::warning(
        IssueCode::Value,
        "Deprecated field",
    ));
    assert_eq!(result.warning_count(), 1);

    result = result.with_issue(ValidationIssue::info(IssueCode::Informational, "FYI"));
    assert_eq!(result.info_count(), 1);
}

#[test]
fn test_validation_issue_builder() {
    use rh_validator::{IssueCode, ValidationIssue};

    let issue = ValidationIssue::error(IssueCode::Required, "Missing required field")
        .with_path("Patient.name");

    assert_eq!(issue.severity, Severity::Error);
    assert_eq!(issue.path, Some("Patient.name".to_string()));
    assert!(issue.message.contains("Missing required field"));
}

#[test]
fn test_cache_stats() {
    let validator = FhirValidator::new(None).unwrap();
    let (profile_cache, rule_cache) = validator.cache_stats();

    assert_eq!(profile_cache.0, 0);
    assert_eq!(rule_cache.0, 0);
}
