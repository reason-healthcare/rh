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

#[test]
fn registered_codesystem_validates_coding_property_codes() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/property-target",
        "status": "active",
        "content": "complete",
        "concept": [
            { "code": "valid-code" }
        ]
    }));

    let codesystem = json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/source",
        "status": "active",
        "content": "complete",
        "property": [
            { "code": "related", "type": "Coding" }
        ],
        "concept": [
            {
                "code": "source-code",
                "property": [
                    {
                        "code": "related",
                        "valueCoding": {
                            "system": "http://example.org/CodeSystem/property-target",
                            "code": "missing-code"
                        }
                    }
                ]
            }
        ]
    });

    let result = validator.validate(&codesystem).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("Unknown code 'missing-code'")
            && i.path.as_deref() == Some("CodeSystem.concept[0].property[0].valueCoding.code")
    }));
}

#[test]
fn registered_codesystem_validates_valueset_coding_filter_values() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/filter-source",
        "status": "active",
        "content": "complete",
        "property": [
            { "code": "related", "type": "Coding" }
        ],
        "filter": [
            { "code": "text-filter", "operator": ["="], "value": "text" }
        ],
        "concept": [
            { "code": "source-code" }
        ]
    }));

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/property-target",
        "status": "active",
        "content": "complete",
        "concept": [
            { "code": "valid-code" }
        ]
    }));

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/filter",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://example.org/CodeSystem/filter-source",
                    "filter": [
                        {
                            "property": "related",
                            "op": "=",
                            "value": "http://example.org/CodeSystem/property-target#missing-code"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("filter based on property 'related'")
            && i.message.contains("Unknown code 'missing-code'")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[0]")
    }));
}

#[test]
fn valueset_coding_filter_requires_system_hash_code_format() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/filter-source",
        "status": "active",
        "content": "complete",
        "property": [
            { "code": "related", "type": "Coding" }
        ],
        "concept": [
            { "code": "source-code" }
        ]
    }));

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/filter",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://example.org/CodeSystem/filter-source",
                    "filter": [
                        {
                            "property": "related",
                            "op": "=",
                            "value": "missing-code"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("must be in the format system(|version)#code")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[0]")
    }));
}

#[test]
fn valueset_filter_code_is_allowed_when_defined_by_codesystem_filter() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/filter-source",
        "status": "active",
        "content": "complete",
        "property": [
            { "code": "related", "type": "Coding" }
        ],
        "filter": [
            { "code": "defined-filter", "operator": ["="], "value": "text" }
        ],
        "concept": [
            { "code": "source-code" }
        ]
    }));

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/filter",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://example.org/CodeSystem/filter-source",
                    "filter": [
                        {
                            "property": "defined-filter",
                            "op": "=",
                            "value": "anything"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[0].property")
    }));
}
