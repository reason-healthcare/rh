use rh_validator::{FhirValidator, FhirVersion, Severity};
use serde_json::json;

#[test]
fn root_shape_validation_reports_unknown_resource_and_property() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let unknown_resource = json!({
        "resourceType": "NotARealResource",
        "id": "example"
    });
    let result = validator.validate(&unknown_resource).unwrap();
    assert!(result.issues.iter().any(|issue| {
        issue.severity == Severity::Error
            && issue.message.contains("Unknown R4 resource type")
            && issue.path.as_deref() == Some("resourceType")
    }));

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "notAField": true
    });
    let result = validator.validate(&patient).unwrap();
    assert!(result.issues.iter().any(|issue| {
        issue.severity == Severity::Error
            && issue.message.contains("Unknown property 'notAField'")
            && issue.path.as_deref() == Some("Patient.notAField")
    }));
}

#[test]
fn choice_validation_accepts_concrete_choice_and_rejects_bad_suffix() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let valid = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": { "text": "test" },
        "valueQuantity": { "value": 1 }
    });
    let result = validator.validate(&valid).unwrap();
    assert!(
        !result
            .issues
            .iter()
            .any(|issue| issue.message.contains("Choice element 'valueQuantity'")),
        "valueQuantity should be accepted as a concrete choice field: {:?}",
        result.issues
    );

    let invalid = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": { "text": "test" },
        "valueNotAType": "bad"
    });
    let result = validator.validate(&invalid).unwrap();
    assert!(result.issues.iter().any(|issue| {
        issue.severity == Severity::Error
            && issue.message.contains("Choice element 'valueNotAType'")
            && issue.path.as_deref() == Some("Observation.valueNotAType")
    }));
}

#[test]
fn profile_fixed_pattern_and_reference_targets_are_enforced() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/fhir/StructureDefinition/observation-profile",
        "name": "ObservationProfile",
        "type": "Observation",
        "snapshot": {
            "element": [
                {"id": "Observation", "path": "Observation", "min": 0, "max": "*"},
                {
                    "id": "Observation.status",
                    "path": "Observation.status",
                    "min": 1,
                    "max": "1",
                    "fixedCode": "final"
                },
                {
                    "id": "Observation.code",
                    "path": "Observation.code",
                    "min": 1,
                    "max": "1",
                    "patternCodeableConcept": {
                        "coding": [{"system": "http://loinc.org", "code": "1234-5"}]
                    }
                },
                {
                    "id": "Observation.subject",
                    "path": "Observation.subject",
                    "min": 0,
                    "max": "1",
                    "type": [{
                        "code": "Reference",
                        "targetProfile": ["http://hl7.org/fhir/StructureDefinition/Patient"]
                    }]
                }
            ]
        }
    }));

    let observation = json!({
        "resourceType": "Observation",
        "status": "amended",
        "code": { "coding": [{"system": "http://loinc.org", "code": "wrong"}] },
        "subject": { "reference": "Practitioner/example" }
    });

    let result = validator
        .validate_with_profile(
            &observation,
            "http://example.org/fhir/StructureDefinition/observation-profile",
        )
        .unwrap();

    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("required fixed value")
            && issue.path.as_deref() == Some("Observation.status")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("required pattern value")
            && issue.path.as_deref() == Some("Observation.code")));
    assert!(result
        .issues
        .iter()
        .any(|issue| issue.message.contains("not a valid Target")));
}

#[test]
fn local_valueset_bindings_validate_expansion_and_compose_codes() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/fhir/StructureDefinition/expanded-value-observation",
        "name": "ExpandedValueObservation",
        "type": "Observation",
        "snapshot": {
            "element": [
                {"id": "Observation", "path": "Observation", "min": 0, "max": "*"},
                {
                    "id": "Observation.value[x]",
                    "path": "Observation.value[x]",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}],
                    "binding": {
                        "strength": "required",
                        "valueSet": "http://example.org/fhir/ValueSet/expanded-codes"
                    }
                }
            ]
        }
    }));
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/fhir/StructureDefinition/value-observation",
        "name": "ValueObservation",
        "type": "Observation",
        "snapshot": {
            "element": [
                {"id": "Observation", "path": "Observation", "min": 0, "max": "*"},
                {
                    "id": "Observation.value[x]",
                    "path": "Observation.value[x]",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}],
                    "binding": {
                        "strength": "required",
                        "valueSet": "http://example.org/fhir/ValueSet/local-codes|1.0.0"
                    }
                }
            ]
        }
    }));
    validator.register_valueset(&json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/fhir/ValueSet/expanded-codes",
        "status": "active",
        "expansion": {
            "contains": [{
                "system": "http://example.org/codes",
                "code": "expanded"
            }]
        }
    }));
    validator.register_valueset(&json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/fhir/ValueSet/local-codes",
        "status": "active",
        "compose": {
            "include": [{
                "system": "http://example.org/codes",
                "concept": [{ "code": "allowed" }]
            }]
        }
    }));

    let expanded_valid = json!({
        "resourceType": "Observation",
        "valueCodeableConcept": {
            "coding": [{"system": "http://example.org/codes", "code": "expanded"}]
        }
    });
    let result = validator
        .validate_with_profile(
            &expanded_valid,
            "http://example.org/fhir/StructureDefinition/expanded-value-observation",
        )
        .unwrap();
    assert!(
        !result
            .issues
            .iter()
            .any(|issue| issue.message.contains("not in required ValueSet")),
        "expansion code should validate locally: {:?}",
        result.issues
    );

    let valid = json!({
        "resourceType": "Observation",
        "valueCodeableConcept": {
            "coding": [{"system": "http://example.org/codes", "code": "allowed"}]
        }
    });
    let result = validator
        .validate_with_profile(
            &valid,
            "http://example.org/fhir/StructureDefinition/value-observation",
        )
        .unwrap();
    assert!(
        !result
            .issues
            .iter()
            .any(|issue| issue.message.contains("not in required ValueSet")),
        "compose include concept should validate locally: {:?}",
        result.issues
    );

    let invalid = json!({
        "resourceType": "Observation",
        "valueCodeableConcept": {
            "coding": [{"system": "http://example.org/codes", "code": "missing"}]
        }
    });
    let result = validator
        .validate_with_profile(
            &invalid,
            "http://example.org/fhir/StructureDefinition/value-observation",
        )
        .unwrap();
    assert!(result.issues.iter().any(|issue| {
        issue.severity == Severity::Error && issue.message.contains("not in required ValueSet")
    }));
}

#[test]
fn questionnaire_response_resolves_registered_questionnaire() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_questionnaire(&json!({
        "resourceType": "Questionnaire",
        "url": "http://example.org/fhir/Questionnaire/intake",
        "status": "active",
        "item": [{
            "linkId": "required-question",
            "type": "string",
            "required": true
        }]
    }));

    let response = json!({
        "resourceType": "QuestionnaireResponse",
        "questionnaire": "http://example.org/fhir/Questionnaire/intake|1.0.0",
        "status": "completed",
        "item": [{
            "linkId": "required-question",
            "answer": [{ "valueInteger": 1 }]
        }]
    });

    let result = validator.validate(&response).unwrap();
    assert!(
        result.issues.iter().any(|issue| {
            issue.severity == Severity::Error
                && issue
                    .message
                    .contains("Answer value must be of the type string")
        }),
        "expected local Questionnaire answer type error, got: {:?}",
        result.issues
    );
    assert!(!result
        .issues
        .iter()
        .any(|issue| issue.message.contains("could not be resolved")));
}
