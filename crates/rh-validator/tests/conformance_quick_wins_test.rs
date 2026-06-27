use rh_validator::{
    FhirValidator, FhirVersion, IssueCode, Severity, TerminologyConfig, ValidationOptions,
};
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
fn document_bundle_versioned_reference_allows_duplicate_identity() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "document",
        "entry": [
            {
                "fullUrl": "http://example.org/Composition/c1",
                "resource": {
                    "resourceType": "Composition",
                    "id": "c1",
                    "section": [
                        {
                            "entry": [
                                {
                                    "reference": "Observation/o1/_history/1"
                                }
                            ]
                        }
                    ]
                }
            },
            {
                "fullUrl": "http://example.org/Observation/o1",
                "resource": {
                    "resourceType": "Observation",
                    "id": "o1"
                }
            },
            {
                "fullUrl": "http://example.org/Observation/o1",
                "resource": {
                    "resourceType": "Observation",
                    "id": "o1"
                }
            }
        ]
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(!result.issues.iter().any(|i| i.code == IssueCode::Duplicate));
}

#[test]
fn bundle_signature_accepts_who_uri() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "document",
        "entry": [
            {
                "fullUrl": "http://example.org/Composition/c1",
                "resource": {
                    "resourceType": "Composition",
                    "id": "c1"
                }
            }
        ],
        "signature": {
            "type": [
                {
                    "system": "urn:iso-astm:E1762-95:2013",
                    "code": "1.2.840.10065.1.12.1.5"
                }
            ],
            "when": "2024-01-01T00:00:00Z",
            "whoUri": "urn:uuid:12345678-1234-1234-1234-123456789abc"
        }
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.message
            .contains("Signature element missing required field 'who' or 'whoUri'")
    }));
}

#[test]
fn searchset_signature_payload_is_not_fatal() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "searchset",
        "entry": [
            {
                "fullUrl": "http://example.org/Patient/p1",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p1"
                }
            }
        ],
        "signature": {
            "type": [
                {
                    "system": "urn:iso-astm:E1762-95:2013",
                    "code": "1.2.840.10065.1.12.1.5"
                }
            ],
            "when": "2024-01-01T00:00:00Z",
            "who": {
                "display": "Example Signer"
            },
            "sigFormat": "application/jose",
            "data": "payload"
        }
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.message.contains("detached signature with no payload") && i.severity == Severity::Error
    }));
}

#[test]
fn valueset_system_extension_satisfies_contained_dom3() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let valueset = json!({
        "resourceType": "ValueSet",
        "id": "vs-canonical-good",
        "url": "http://hl7.org/fhir/test/ValueSet/vs-canonical-good",
        "status": "active",
        "contained": [
            {
                "resourceType": "CodeSystem",
                "id": "c1",
                "url": "http://hl7.org/fhir/CodeSystem/c1",
                "status": "active",
                "content": "complete",
                "concept": [
                    {
                        "code": "c1"
                    }
                ]
            }
        ],
        "compose": {
            "include": [
                {
                    "system": "http://hl7.org/fhir/CodeSystem/c1",
                    "_system": {
                        "extension": [
                            {
                                "url": "http://hl7.org/fhir/StructureDefinition/valueset-system",
                                "valueCanonical": "#c1"
                            }
                        ]
                    }
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(!result.issues.iter().any(|i| i.message.contains("dom-3")));
}

#[test]
fn bundle_fullurl_accepts_uin_uuid_scheme() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let bundle = json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": [
            {
                "fullUrl": "uin:uuid:cd7f9309-2530-4adf-9b94-e44548361e8a",
                "resource": {
                    "resourceType": "Patient",
                    "id": "p1"
                }
            }
        ]
    });

    let result = validator.validate(&bundle).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.path.as_deref() == Some("Bundle.entry[0]")
            && i.message.contains("fullUrl must be an absolute URL")
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
fn valueset_compose_concepts_validate_against_supported_terminology() {
    let validator =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))
            .unwrap();

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/bad-snomed-concepts",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://snomed.info/sct",
                    "concept": [
                        {
                            "code": "1"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("Code '1' not found")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].concept[0]")
    }));
}

#[test]
fn valueset_compose_concepts_accept_known_supported_terminology_code() {
    let validator =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))
            .unwrap();

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/good-snomed-concepts",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://snomed.info/sct",
                    "concept": [
                        {
                            "code": "386661006"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.path.as_deref() == Some("ValueSet.compose.include[0].concept[0]")
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

#[test]
fn coding_filter_accepts_system_pipe_code_pattern() {
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
                            "value": "http://example.org/CodeSystem/property-target|valid-code"
                        }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[0]")
    }));
}

#[test]
fn codesystem_supplement_requires_supplement_content() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let codesystem = json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/CodeSystem/supplement",
        "status": "active",
        "content": "complete",
        "supplements": "http://loinc.org",
        "concept": [
            { "code": "test" }
        ]
    });

    let result = validator.validate(&codesystem).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("CodeSystem supplements SHALL have a content value")
            && i.path.as_deref() == Some("CodeSystem.content")
    }));
}

#[test]
fn valueset_parameter_expression_references_declared_parameters() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://snomed.info/sct",
        "status": "active",
        "content": "complete",
        "filter": [
            { "code": "inactive", "operator": ["="], "value": "boolean" }
        ],
        "concept": [
            { "code": "929360041000036105" }
        ]
    }));

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.com/ValueSet/parameterised",
        "status": "active",
        "extension": [{
            "url": "http://hl7.org/fhir/tools/StructureDefinition/valueset-parameter",
            "extension": [
                { "url": "name", "valueCode": "p-inactive" },
                { "url": "documentation", "valueMarkdown": "whether inactive" }
            ]
        }],
        "compose": {
            "include": [{
                "system": "http://snomed.info/sct",
                "filter": [{
                    "property": "inactive",
                    "op": "=",
                    "_value": {
                        "extension": [{
                            "url": "http://hl7.org/fhir/StructureDefinition/cqf-expression",
                            "valueExpression": {
                                "language": "text/fhirpath",
                                "expression": "%p-inactive3"
                            }
                        }]
                    }
                }]
            }]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("unknown parameter 'p-inactive3'")
            && i.path.as_deref()
                == Some(
                    "ValueSet.compose.include[0].filter[0].extension[0].value.ofType(Expression)",
                )
    }));
}

#[test]
fn valueset_parameter_expression_requires_fhirpath_language() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://snomed.info/sct",
        "status": "active",
        "content": "complete",
        "filter": [
            { "code": "inactive", "operator": ["="], "value": "boolean" }
        ],
        "concept": [
            { "code": "929360041000036105" }
        ]
    }));

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.com/ValueSet/parameterised",
        "status": "active",
        "compose": {
            "include": [{
                "system": "http://snomed.info/sct",
                "filter": [{
                    "property": "inactive",
                    "op": "=",
                    "_value": {
                        "extension": [{
                            "url": "http://hl7.org/fhir/StructureDefinition/cqf-expression",
                            "valueExpression": {
                                "language": "text/cql",
                                "expression": "%p-inactive"
                            }
                        }]
                    }
                }]
            }]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("ValueSet expression language must be 'text/fhirpath'")
    }));
}

#[test]
fn search_parameter_derived_from_checks_type_and_base() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let search_parameter = json!({
        "resourceType": "SearchParameter",
        "url": "http://example.org/SearchParameter/organization-name",
        "status": "active",
        "code": "name",
        "derivedFrom": "http://hl7.org/fhir/SearchParameter/Organization-name",
        "base": ["Patient"],
        "type": "token",
        "expression": "Organization.name | Organization.alias"
    });

    let result = validator.validate(&search_parameter).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("The type token is different")
            && i.path.as_deref() == Some("SearchParameter")
    }));
    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("The resource type Patient is not listed as a base")
            && i.path.as_deref() == Some("SearchParameter")
    }));
}

#[test]
fn capability_statement_search_param_definition_type_must_match() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let capability = json!({
        "resourceType": "CapabilityStatement",
        "url": "http://example.org/CapabilityStatement/example",
        "status": "draft",
        "date": "2026-01-01",
        "kind": "requirements",
        "fhirVersion": "4.0.1",
        "format": ["json"],
        "rest": [{
            "mode": "server",
            "resource": [{
                "type": "AllergyIntolerance",
                "interaction": [{ "code": "search-type" }],
                "searchParam": [{
                    "name": "clinical-status",
                    "definition": "http://hl7.org/fhir/SearchParameter/AllergyIntolerance-clinical-status",
                    "type": "date"
                }]
            }]
        }]
    });

    let result = validator.validate(&capability).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("type is token, but type here is date")
            && i.path.as_deref() == Some("CapabilityStatement.rest[0].resource[0].searchParam[0]")
    }));
}
