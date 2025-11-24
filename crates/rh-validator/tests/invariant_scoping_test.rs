use anyhow::Result;
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::json;

#[test]
fn test_ext1_only_applies_to_extensions() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_without_extensions = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }]
    });

    let result = validator.validate_auto(&patient_without_extensions)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        0,
        "ext-1 should not apply to Patient without extensions, but got: {ext1_errors:?}"
    );

    Ok(())
}

// TODO: This test requires per-element invariant evaluation
// Currently invariants are evaluated at resource level, not per-element
#[test]
#[ignore]
fn test_ext1_applies_to_extension_elements() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_with_invalid_extension = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "extension": [{
            "valueString": "test"
        }]
    });

    let result = validator.validate_auto(&patient_with_invalid_extension)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert!(
        !ext1_errors.is_empty(),
        "ext-1 should apply to extension without url"
    );

    Ok(())
}

#[test]
fn test_ext1_applies_to_valid_extension() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_with_valid_extension = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "extension": [{
            "url": "http://example.org/test",
            "valueString": "test"
        }]
    });

    let result = validator.validate_auto(&patient_with_valid_extension)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        0,
        "ext-1 should pass for valid extension with url, but got: {ext1_errors:?}"
    );

    Ok(())
}

// TODO: This test requires per-element invariant evaluation
#[test]
#[ignore]
fn test_multiple_extension_instances() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_with_multiple_extensions = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "extension": [
            {
                "url": "http://example.org/test1",
                "valueString": "valid1"
            },
            {
                "valueString": "invalid - no url"
            },
            {
                "url": "http://example.org/test2",
                "valueString": "valid2"
            }
        ]
    });

    let result = validator.validate_auto(&patient_with_multiple_extensions)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        1,
        "ext-1 should only fail for the one invalid extension, but got: {ext1_errors:?}"
    );

    if let Some(error) = ext1_errors.first() {
        assert!(
            error.message.contains("[at extension[1]]"),
            "Error should indicate extension[1] as location, but got: {}",
            error.message
        );
    }

    Ok(())
}

#[test]
fn test_resource_level_invariant() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let invalid_questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "draft",
        "item": [
            {
                "linkId": "1",
                "type": "string",
                "text": "First question"
            },
            {
                "linkId": "1",
                "type": "string",
                "text": "Duplicate linkId"
            }
        ]
    });

    let result = validator.validate_auto(&invalid_questionnaire)?;

    let que2_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("que-2"))
        .collect();

    assert!(
        !que2_errors.is_empty(),
        "que-2 (linkId uniqueness) should fail for duplicate linkIds"
    );

    Ok(())
}

// TODO: This test requires per-element invariant evaluation for nested paths
#[test]
#[ignore]
fn test_element_level_invariant_on_nested_element() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_with_nested_extension = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "name": [{
            "family": "Doe",
            "given": ["John"],
            "extension": [{
                "valueString": "invalid nested extension"
            }]
        }]
    });

    let result = validator.validate_auto(&patient_with_nested_extension)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        1,
        "ext-1 should apply to nested extension in name, but got: {ext1_errors:?}"
    );

    if let Some(error) = ext1_errors.first() {
        assert!(
            error.message.contains("name") || error.message.contains("extension"),
            "Error should reference the nested extension, but got: {}",
            error.message
        );
    }

    Ok(())
}

#[test]
fn test_contained_resource_validation() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let questionnaire_with_valid_contained = json!({
        "resourceType": "Questionnaire",
        "status": "draft",
        "contained": [{
            "resourceType": "Patient",
            "id": "contained-patient",
            "active": true
        }],
        "item": [{
            "linkId": "1",
            "type": "string",
            "text": "Question"
        }]
    });

    let result = validator.validate_auto(&questionnaire_with_valid_contained)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        0,
        "ext-1 should not apply to contained resources without extensions, but got: {ext1_errors:?}"
    );

    Ok(())
}

#[test]
fn test_no_false_positives_on_simple_patient() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let simple_patient = json!({
        "resourceType": "Patient",
        "id": "simple-patient",
        "active": true,
        "gender": "male",
        "birthDate": "1974-12-25",
        "name": [{
            "use": "official",
            "family": "Chalmers",
            "given": ["Peter", "James"]
        }],
        "telecom": [{
            "system": "phone",
            "value": "(03) 5555 6473",
            "use": "work"
        }]
    });

    let result = validator.validate_auto(&simple_patient)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    assert_eq!(
        ext1_errors.len(),
        0,
        "Simple patient without extensions should not trigger ext-1, but got: {ext1_errors:?}"
    );

    let total_errors = result
        .issues
        .iter()
        .filter(|i| i.severity.to_string() == "error")
        .count();
    assert_eq!(
        total_errors, 0,
        "Simple valid patient should have no errors, but got: {:?}",
        result.issues
    );

    Ok(())
}
