use anyhow::Result;
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::{json, Value};

/// Minimal Patient StructureDefinition with ext-1 constraints on extension elements.
/// Used to make tests self-contained without requiring the FHIR R4 core package.
fn patient_profile_with_ext1() -> Value {
    json!({
        "resourceType": "StructureDefinition",
        "url": "http://hl7.org/fhir/StructureDefinition/Patient",
        "name": "Patient",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "Patient",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "snapshot": {
            "element": [
                {
                    "id": "Patient",
                    "path": "Patient",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "Patient.extension",
                    "path": "Patient.extension",
                    "min": 0,
                    "max": "*",
                    "constraint": [{
                        "key": "ext-1",
                        "severity": "error",
                        "human": "Must have either extensions or value[x], not both",
                        "expression": "extension.exists() != value.exists()"
                    }]
                },
                {
                    "id": "Patient.contact",
                    "path": "Patient.contact",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "Patient.contact.extension",
                    "path": "Patient.contact.extension",
                    "min": 0,
                    "max": "*",
                    "constraint": [{
                        "key": "ext-1",
                        "severity": "error",
                        "human": "Must have either extensions or value[x], not both",
                        "expression": "extension.exists() != value.exists()"
                    }]
                }
            ]
        }
    })
}

#[test]
fn test_ext1_only_applies_to_extensions() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;
    validator.register_profile(&patient_profile_with_ext1());

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

// Tests that per-element invariant evaluation fires ext-1 correctly
#[test]
fn test_ext1_applies_to_extension_elements() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;
    validator.register_profile(&patient_profile_with_ext1());

    // An extension with a url but neither a value[x] nor sub-extensions violates ext-1
    // (extension.exists() != value.exists() → false != false → false → violation)
    let patient_with_invalid_extension = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "extension": [{
            "url": "http://example.org/test"
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
    validator.register_profile(&patient_profile_with_ext1());

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

// Tests per-element scoping: only the invalid extension among valid ones triggers ext-1
#[test]
fn test_multiple_extension_instances() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;
    validator.register_profile(&patient_profile_with_ext1());

    // extension[1] has a url but neither value[x] nor sub-extensions → violates ext-1
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
                "url": "http://example.org/test-invalid"
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

    // Register a minimal Questionnaire profile with the invariant to ensure the test passes
    // even if the FHIR core package is not installed in the environment.
    let questionnaire_profile = json!({
        "resourceType": "StructureDefinition",
        "url": "http://hl7.org/fhir/StructureDefinition/Questionnaire",
        "name": "Questionnaire",
        "status": "active",
        "kind": "resource",
        "abstract": false,
        "type": "Questionnaire",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "derivation": "specialization",
        "snapshot": {
            "element": [
                {
                    "id": "Questionnaire",
                    "path": "Questionnaire",
                    "min": 0,
                    "max": "*",
                    "constraint": [
                        {
                            "key": "que-2",
                            "severity": "error",
                            "human": "The linkIds for groups and questions must be unique",
                            "expression": "descendants().linkId.isDistinct()"
                        }
                    ]
                }
            ]
        }
    });
    validator.register_profile(&questionnaire_profile);

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

// Tests per-element evaluation for a deeply-nested extension path (Patient.contact.extension)
#[test]
fn test_element_level_invariant_on_nested_element() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;
    validator.register_profile(&patient_profile_with_ext1());

    // contact[0].extension[0] has a url but no value[x] and no sub-extensions → violates ext-1
    let patient_with_nested_extension = json!({
        "resourceType": "Patient",
        "id": "test-patient",
        "active": true,
        "contact": [{
            "name": {"family": "Smith"},
            "extension": [{
                "url": "http://example.org/contact-ext"
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
    validator.register_profile(&patient_profile_with_ext1());

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
