use rh_validator::{FhirValidator, FhirVersion, Severity, TerminologyConfig, ValidationOptions};
use serde_json::json;

#[test]
fn security_checks_disabled_reports_information() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "text": "<script>alert('x')</script>"
    });

    let result = validator.validate(&patient).unwrap();

    assert!(result
        .issues
        .iter()
        .any(|i| i.severity == Severity::Information));
}

#[test]
fn security_checks_enabled_reports_error() {
    let validator = FhirValidator::with_options(
        FhirVersion::R4,
        None,
        None,
        ValidationOptions {
            security_checks: true,
        },
    )
    .unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "text": "<script>alert('x')</script>"
    });

    let result = validator.validate(&patient).unwrap();

    assert!(result.issues.iter().any(|i| i.severity == Severity::Error));
}

#[test]
fn document_bundle_duplicate_fullurl_is_reported() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "document",
        "entry": [
            {
                "fullUrl": "http://example.org/Patient/p1",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p1"
                }
            },
            {
                "fullUrl": "http://example.org/Patient/p1",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p2"
                }
            }
        ]
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.path.as_deref() == Some("Bundle.entry[0].fullUrl")
            || i.path.as_deref() == Some("Bundle.entry[1].fullUrl")
    }));
}

#[test]
fn bundle_duplicate_resource_identity_is_reported() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": [
            {
                "fullUrl": "http://example.org/Patient/p1-a",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p1"
                }
            },
            {
                "fullUrl": "http://example.org/Patient/p1-b",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p1"
                }
            }
        ]
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.path.as_deref() == Some("Bundle.entry[0].resource.id")
            || i.path.as_deref() == Some("Bundle.entry[1].resource.id")
    }));
}

#[test]
fn ucum_skip_note_present_without_terminology_service() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "text": "Weight"
        },
        "valueQuantity": {
            "value": 70,
            "system": "http://unitsofmeasure.org",
            "code": "kg",
            "unit": "kg"
        }
    });

    let result = validator.validate(&observation).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.message
            .contains("UCUM/unit quick-win validation was skipped because no terminology service")
    }));
}

#[test]
fn ucum_skip_note_not_present_with_terminology_service() {
    let validator =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))
            .unwrap();

    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "text": "Weight"
        },
        "valueQuantity": {
            "value": 70,
            "system": "http://unitsofmeasure.org",
            "code": "kg",
            "unit": "kg"
        }
    });

    let result = validator.validate(&observation).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.message
            .contains("UCUM/unit quick-win validation was skipped because no terminology service")
    }));
}
