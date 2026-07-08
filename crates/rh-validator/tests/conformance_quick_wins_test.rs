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
            ..ValidationOptions::default()
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
fn r4_rejects_fhir_comments_property() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "fhir_comments": ["legacy comment"]
    });

    let result = validator.validate(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message.contains("Unrecognized property 'fhir_comments'")
            && i.path.as_deref() == Some("Patient")
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
fn questionnaire_response_questionnaire_satisfies_contained_dom3() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let response = json!({
        "resourceType": "QuestionnaireResponse",
        "status": "completed",
        "questionnaire": "#q1",
        "contained": [
            {
                "resourceType": "Questionnaire",
                "id": "q1",
                "status": "draft"
            }
        ]
    });

    let result = validator.validate(&response).unwrap();

    assert!(!result.issues.iter().any(|i| i.message.contains("dom-3")));
}

#[test]
fn questionnaire_answer_valueset_satisfies_contained_dom3() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "draft",
        "item": [
            {
                "linkId": "q1",
                "type": "choice",
                "answerValueSet": "#options"
            }
        ],
        "contained": [
            {
                "resourceType": "ValueSet",
                "id": "options",
                "status": "draft"
            }
        ]
    });

    let result = validator.validate(&questionnaire).unwrap();

    assert!(!result.issues.iter().any(|i| i.message.contains("dom-3")));
}

#[test]
fn contained_resource_id_rejects_invalid_characters() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let location = json!({
        "resourceType": "Location",
        "id": "loc1",
        "managingOrganization": {
            "reference": "#org_1"
        },
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org_1"
            }
        ]
    });

    let result = validator.validate(&location).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invalid
            && i.message.contains("Invalid Resource id")
            && i.path.as_deref() == Some("Location.contained[0]/*Organization*/.id")
    }));
}

#[test]
fn contained_resource_id_length_remains_lenient() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    let long_id = "a".repeat(65);

    let condition = json!({
        "resourceType": "Condition",
        "id": "c1",
        "contained": [
            {
                "resourceType": "Observation",
                "id": long_id
            }
        ]
    });

    let result = validator.validate(&condition).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("value exceeds 64 characters")
            && i.path
                .as_deref()
                .is_some_and(|path| path.contains(".contained[0]"))
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
fn period_invariant_rejects_mixed_precision_start_end() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let encounter = json!({
        "resourceType": "Encounter",
        "status": "unknown",
        "class": {
            "system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "code": "AMB"
        },
        "period": {
            "start": "2023-06-21",
            "end": "2023-06-21T06:20:00Z"
        }
    });

    let result = validator.validate_auto(&encounter).unwrap();

    assert!(
        result.issues.iter().any(|i| {
            i.severity == Severity::Error
                && i.code == IssueCode::Invariant
                && i.message.contains("per-1")
                && i.message.contains("period")
        }),
        "issues: {:#?}",
        result.issues
    );
}

#[test]
fn period_invariant_accepts_ordered_same_precision_start_end() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let encounter = json!({
        "resourceType": "Encounter",
        "status": "unknown",
        "class": {
            "system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "code": "AMB"
        },
        "period": {
            "start": "2023-06-21T05:20:00Z",
            "end": "2023-06-21T06:20:00Z"
        }
    });

    let result = validator.validate_auto(&encounter).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invariant
            && i.message.contains("per-1")
    }));
}

#[test]
fn humanname_mothers_family_extension_rejects_name_level_usage() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "name": [
            {
                "family": "Family",
                "given": ["Names"],
                "extension": [
                    {
                        "url": "http://hl7.org/fhir/StructureDefinition/humanname-mothers-family",
                        "valueString": "Mothers family"
                    }
                ]
            }
        ]
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message.contains("humanname-mothers-family")
            && i.path.as_deref() == Some("Patient.name[0].extension[0]")
    }));
}

#[test]
fn humanname_mothers_family_extension_allows_family_usage() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "name": [
            {
                "family": "Family",
                "_family": {
                    "extension": [
                        {
                            "url": "http://hl7.org/fhir/StructureDefinition/humanname-mothers-family",
                            "valueString": "Mothers family"
                        }
                    ]
                },
                "given": ["Names"]
            }
        ]
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message.contains("humanname-mothers-family")
    }));
}

#[test]
fn known_missing_core_extension_definition_is_allowed() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let endpoint = json!({
        "resourceType": "Endpoint",
        "status": "active",
        "connectionType": {
            "system": "http://terminology.hl7.org/CodeSystem/endpoint-connection-type",
            "code": "hl7-fhir-rest"
        },
        "payloadType": [
            {
                "text": "FHIR"
            }
        ],
        "address": "https://example.org/fhir",
        "extension": [
            {
                "url": "http://hl7.org/fhir/StructureDefinition/endpoint-fhir-version",
                "valueCode": "4.0.1"
            }
        ]
    });

    let result = validator.validate(&endpoint).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains(
                "Extension definition 'http://hl7.org/fhir/StructureDefinition/endpoint-fhir-version'",
            )
    }));
}

#[test]
fn questionnaire_multiple_enable_when_requires_enable_behavior() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "draft",
        "item": [
            {
                "linkId": "q1",
                "type": "integer"
            },
            {
                "linkId": "q2",
                "type": "integer"
            },
            {
                "linkId": "q3",
                "type": "integer",
                "enableWhen": [
                    {
                        "question": "q1",
                        "operator": "=",
                        "answerInteger": 1
                    },
                    {
                        "question": "q2",
                        "operator": "=",
                        "answerInteger": 1
                    }
                ]
            }
        ]
    });

    let result = validator.validate(&questionnaire).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invariant
            && i.message.contains("qst-2")
            && i.path.as_deref() == Some("Questionnaire.item[2]")
    }));
}

#[test]
fn questionnaire_multiple_enable_when_accepts_enable_behavior() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "status": "draft",
        "item": [
            {
                "linkId": "q1",
                "type": "integer"
            },
            {
                "linkId": "q2",
                "type": "integer"
            },
            {
                "linkId": "q3",
                "type": "integer",
                "enableBehavior": "all",
                "enableWhen": [
                    {
                        "question": "q1",
                        "operator": "=",
                        "answerInteger": 1
                    },
                    {
                        "question": "q2",
                        "operator": "=",
                        "answerInteger": 1
                    }
                ]
            }
        ]
    });

    let result = validator.validate(&questionnaire).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invariant
            && i.message.contains("qst-2")
    }));
}

#[test]
fn questionnaire_response_validates_quantity_constraints_from_registered_questionnaire() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "url": "http://example.org/Questionnaire/quantity-min-max",
        "status": "active",
        "item": [
            {
                "extension": [
                    {
                        "url": "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-maxQuantity",
                        "valueQuantity": {
                            "value": 5,
                            "unit": "Kg"
                        }
                    },
                    {
                        "url": "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-minQuantity",
                        "valueQuantity": {
                            "value": 50,
                            "unit": "Kg"
                        }
                    }
                ],
                "linkId": "q1",
                "type": "quantity"
            }
        ]
    });
    validator.register_questionnaire(&questionnaire);

    let response = json!({
        "resourceType": "QuestionnaireResponse",
        "status": "in-progress",
        "questionnaire": "http://example.org/Questionnaire/quantity-min-max",
        "item": [
            {
                "linkId": "q1",
                "answer": [
                    {
                        "valueQuantity": {
                            "value": 10,
                            "unit": "Kg"
                        }
                    }
                ]
            }
        ]
    });

    let result = validator.validate_auto(&response).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invariant
            && i.message.contains("cannot be compared")
    }));
}

#[test]
fn questionnaire_response_validates_unit_value_set_extension() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "url": "http://example.org/Questionnaire/unit-valueset",
        "status": "active",
        "item": [
            {
                "extension": [
                    {
                        "url": "http://hl7.org/fhir/StructureDefinition/questionnaire-unitValueSet",
                        "valueCanonical": "http://hl7.org/fhir/ValueSet/jurisdiction"
                    }
                ],
                "linkId": "q1",
                "type": "quantity"
            }
        ]
    });
    validator.register_questionnaire(&questionnaire);

    let response = json!({
        "resourceType": "QuestionnaireResponse",
        "status": "completed",
        "questionnaire": "http://example.org/Questionnaire/unit-valueset",
        "item": [
            {
                "linkId": "q1",
                "answer": [
                    {
                        "valueQuantity": {
                            "value": 10,
                            "unit": "kilometer",
                            "system": "http://unitsofmeasure.org",
                            "code": "km"
                        }
                    }
                ]
            }
        ]
    });

    let result = validator.validate_auto(&response).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invariant
            && i.message.contains("unit that is not in the unit value set")
    }));
}

#[test]
fn questionnaire_response_validates_answer_valueset_display_with_terminology() {
    let validator =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))
            .unwrap();

    let questionnaire = json!({
        "resourceType": "Questionnaire",
        "url": "http://example.org/Questionnaire/item-type",
        "status": "active",
        "item": [
            {
                "linkId": "q1",
                "type": "choice",
                "answerValueSet": "http://hl7.org/fhir/ValueSet/item-type"
            }
        ]
    });
    validator.register_questionnaire(&questionnaire);

    let response = json!({
        "resourceType": "QuestionnaireResponse",
        "status": "completed",
        "questionnaire": "http://example.org/Questionnaire/item-type",
        "item": [
            {
                "linkId": "q1",
                "answer": [
                    {
                        "valueCoding": {
                            "system": "http://hl7.org/fhir/item-type",
                            "code": "string",
                            "display": "Australia"
                        }
                    }
                ]
            }
        ]
    });

    let result = validator.validate_auto(&response).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::CodeInvalid
            && i.message.contains("Wrong Display Name")
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
fn valueset_rejects_invalid_known_filter_values() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/bad-filter-values",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://snomed.info/sct",
                    "filter": [
                        { "property": "concept", "op": "is-a", "value": "something" },
                        { "property": "1142143009", "op": "not-in", "value": "734137005" },
                        { "property": "constraint", "op": "=", "value": "234234 << 234234" }
                    ]
                },
                {
                    "system": "http://terminology.hl7.org/CodeSystem/ex-tooth",
                    "filter": [
                        { "property": "notSelectable", "op": "=", "value": "1" }
                    ]
                }
            ]
        }
    });

    let result = validator.validate(&valueset).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("property 'concept' must be a valid code")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[0]")
    }));
    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("operation 'not-in' is not allowed for property '1142143009'")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[1]")
    }));
    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("value '234234 << 234234' for the filter constraint is invalid")
            && i.path.as_deref() == Some("ValueSet.compose.include[0].filter[2]")
    }));
    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("property 'notSelectable' must be either 'true' or 'false'")
            && i.path.as_deref() == Some("ValueSet.compose.include[1].filter[0]")
    }));
}

#[test]
fn conceptmap_source_code_must_be_in_source_valueset_when_resolved() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let valueset = json!({
        "resourceType": "ValueSet",
        "url": "http://example.org/ValueSet/source",
        "status": "active",
        "compose": {
            "include": [
                {
                    "system": "http://example.org/CodeSystem/source",
                    "concept": [
                        { "code": "pharm" },
                        { "code": "vac" }
                    ]
                }
            ]
        }
    });
    validator.register_valueset(&valueset);

    let conceptmap = json!({
        "resourceType": "ConceptMap",
        "url": "http://example.org/ConceptMap/example",
        "status": "active",
        "sourceCanonical": "http://example.org/ValueSet/source",
        "targetCanonical": "http://example.org/ValueSet/target",
        "group": [
            {
                "source": "http://example.org/CodeSystem/source",
                "target": "http://snomed.info/sct",
                "element": [
                    {
                        "code": "med",
                        "target": [
                            {
                                "code": "264358009",
                                "equivalence": "equivalent"
                            }
                        ]
                    }
                ]
            }
        ]
    });

    let result = validator.validate(&conceptmap).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Required
            && i.message
                .contains("The source code 'med' is not valid in the value set")
            && i.path.as_deref() == Some("ConceptMap.group[0].element[0].code")
    }));
}

#[test]
fn complete_registered_codesystem_rejects_unknown_code() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    validator.register_codesystem(&json!({
        "resourceType": "CodeSystem",
        "url": "http://example.org/fhir/CodeSystem/contract-state",
        "content": "complete",
        "concept": [
            {"code": "active"},
            {"code": "cancelled"}
        ]
    }));

    let contract = json!({
        "resourceType": "Contract",
        "legalState": {
            "coding": [
                {
                    "system": "http://example.org/fhir/CodeSystem/contract-state",
                    "code": "missing-code"
                }
            ]
        }
    });

    let result = validator.validate(&contract).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::CodeInvalid
            && i.message.contains("Unknown code 'missing-code'")
            && i.path.as_deref() == Some("Contract.legalState.coding[0].code")
    }));
}

#[test]
fn structure_definition_pattern_rule_rejects_missing_required_coding() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();
    let profile_url = "http://example.org/fhir/StructureDefinition/observation-code-pattern";
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": profile_url,
        "name": "ObservationCodePattern",
        "status": "draft",
        "kind": "resource",
        "abstract": false,
        "type": "Observation",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Observation",
        "derivation": "constraint",
        "snapshot": {
            "element": [
                {
                    "id": "Observation",
                    "path": "Observation",
                    "min": 0,
                    "max": "*"
                },
                {
                    "id": "Observation.code",
                    "path": "Observation.code",
                    "min": 1,
                    "max": "1",
                    "patternCodeableConcept": {
                        "coding": [{
                            "system": "http://loinc.org",
                            "code": "85354-9"
                        }]
                    }
                }
            ]
        }
    }));

    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "76534-7"
            }]
        }
    });

    let result = validator
        .validate_with_profile(&observation, profile_url)
        .unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Value
            && i.message.contains("required pattern value")
            && i.path.as_deref() == Some("Observation.code")
    }));
}

#[test]
fn us_core_ethnicity_detailed_rejects_codes_outside_required_valueset() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "extension": [{
            "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
            "extension": [{
                "url": "detailed",
                "valueCoding": {
                    "system": "urn:oid:2.16.840.1.113883.6.238",
                    "code": "2184-0"
                }
            }]
        }]
    });

    let result = validator.validate(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::CodeInvalid
            && i.message.contains("is not in required ValueSet")
            && i.path.as_deref() == Some("Patient.extension[0].extension[0].valueCoding.code")
    }));
}

#[test]
fn coding_system_must_be_absolute_codesystem_uri() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "category": [{
            "coding": [{
                "system": "Location1",
                "code": "Location"
            }]
        }],
        "code": {
            "coding": [{
                "system": "http://hl7.org/fhir/ValueSet/measure-type",
                "code": "structure"
            }]
        }
    });

    let result = validator.validate_auto(&observation).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invalid
            && i.message.contains("absolute reference")
            && i.path.as_deref() == Some("Observation.category[0].coding[0].system")
    }));
    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invalid
            && i.message.contains("value set, not a code system")
            && i.path.as_deref() == Some("Observation.code.coding[0].system")
    }));
}

#[test]
fn hl7_published_codesystem_does_not_require_workgroup_without_publication_mode() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let codesystem = json!({
        "resourceType": "CodeSystem",
        "url": "http://hl7.org/fhir/example-status",
        "status": "active",
        "publisher": "HL7 (FHIR Project)",
        "content": "complete",
        "concept": [
            { "code": "example" }
        ]
    });

    let result = validator.validate(&codesystem).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error && i.message.contains("owning committee must be stated")
    }));
}

#[test]
fn hl7_published_codesystem_accepts_workgroup_extension() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let codesystem = json!({
        "resourceType": "CodeSystem",
        "url": "http://hl7.org/fhir/example-status",
        "status": "active",
        "publisher": "HL7 (FHIR Project)",
        "content": "complete",
        "extension": [
            {
                "url": "http://hl7.org/fhir/StructureDefinition/structuredefinition-wg",
                "valueCode": "fhir"
            }
        ],
        "concept": [
            { "code": "example" }
        ]
    });

    let result = validator.validate(&codesystem).unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error && i.message.contains("owning committee must be stated")
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
fn composite_search_parameter_requires_two_components() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let search_parameter = json!({
        "resourceType": "SearchParameter",
        "url": "http://example.org/SearchParameter/documentreference-relationship",
        "status": "active",
        "code": "relationship",
        "base": ["DocumentReference"],
        "type": "composite",
        "expression": "DocumentReference.relatesTo"
    });

    let result = validator.validate(&search_parameter).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::BusinessRule
            && i.message.contains("must define two or more components")
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

#[test]
fn dynamic_differential_profile_enforces_max_cardinality() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/patient-profile",
        "type": "Patient",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
        "derivation": "constraint",
        "kind": "resource",
        "differential": {
            "element": [{
                "id": "Patient.identifier",
                "path": "Patient.identifier",
                "max": "0"
            }]
        }
    }));

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://example.org/patient-profile"]
        },
        "identifier": [{
            "system": "http://example.org/patient-identifier",
            "value": "123456"
        }]
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("Cardinality violation at 'Patient.identifier': expected at most 0")
    }));
}

#[test]
fn dynamic_differential_profile_enforces_min_cardinality() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/other-patient-profile",
        "type": "Patient",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
        "derivation": "constraint",
        "kind": "resource",
        "differential": {
            "element": [{
                "id": "Patient.identifier",
                "path": "Patient.identifier",
                "min": 1,
                "max": "*"
            }]
        }
    }));

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://example.org/other-patient-profile"]
        }
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message
                .contains("Cardinality violation at 'Patient.identifier': expected at least 1")
    }));
}

#[test]
fn unknown_hl7_ig_extension_definition_is_rejected() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let observation = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "text": "example"
        },
        "extension": [{
            "url": "http://hl7.org/fhir/uv/example/StructureDefinition/not-loaded",
            "valueString": "unsupported"
        }]
    });

    let result = validator.validate_auto(&observation).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message.contains(
                "Extension definition 'http://hl7.org/fhir/uv/example/StructureDefinition/not-loaded' could not be found"
            )
            && i.path.as_deref() == Some("Observation.extension[0]")
    }));
}

#[test]
fn structure_definition_choice_path_must_use_base_choice_path() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let structure_definition = json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.com/Observation",
        "type": "Observation",
        "kind": "resource",
        "derivation": "constraint",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Observation",
        "differential": {
            "element": [{
                "id": "Observation.valueQuantity",
                "path": "Observation.valueBla"
            }]
        }
    });

    let result = validator.validate(&structure_definition).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Processing
            && i.message
                .contains("The path must be 'Observation.value[x]' not 'Observation.valueBla'")
            && i.path.as_deref() == Some("StructureDefinition")
    }));
}

#[test]
fn structure_definition_derived_profile_cannot_change_base_fixed_value() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://hl7.org/fhir/test/StructureDefinition/ext-base",
        "type": "Extension",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Extension",
        "differential": {
            "element": [{
                "id": "Extension.url",
                "path": "Extension.url",
                "fixedUri": "http://hl7.org/fhir/test/StructureDefinition/ext-base"
            }]
        }
    }));

    let derived = json!({
        "resourceType": "StructureDefinition",
        "url": "http://hl7.org/fhir/test/StructureDefinition/ext-derived",
        "type": "Extension",
        "kind": "complex-type",
        "derivation": "constraint",
        "baseDefinition": "http://hl7.org/fhir/test/StructureDefinition/ext-base",
        "differential": {
            "element": [{
                "id": "Extension.url",
                "path": "Extension.url",
                "fixedUri": "http://hl7.org/fhir/test/StructureDefinition/ext-derived"
            }]
        }
    });

    let result = validator.validate(&derived).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Value
            && i.message.contains("fixed to")
            && i.path.as_deref() == Some("Extension.url")
    }));
}

#[test]
fn reference_target_profile_rejects_wrong_reference_type() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/patient-target",
        "type": "Patient",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Patient",
        "snapshot": {
            "element": [{
                "id": "Patient",
                "path": "Patient"
            }]
        }
    }));
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/documentreference-target",
        "type": "DocumentReference",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DocumentReference",
        "snapshot": {
            "element": [{
                "id": "DocumentReference.subject",
                "path": "DocumentReference.subject",
                "min": 1,
                "max": "1",
                "type": [{
                    "code": "Reference",
                    "targetProfile": [
                        "http://example.org/StructureDefinition/patient-target"
                    ]
                }]
            }]
        }
    }));

    let document_reference = json!({
        "resourceType": "DocumentReference",
        "status": "current",
        "subject": {
            "reference": "Device/foo"
        }
    });

    let result = validator
        .validate_with_profile(
            &document_reference,
            "http://example.org/StructureDefinition/documentreference-target",
        )
        .unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message
                .contains("The type 'Device' implied by the reference URL")
            && i.path.as_deref() == Some("DocumentReference.subject")
    }));
}

#[test]
fn reference_target_profile_resource_allows_any_resource_type() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/resource-target",
        "type": "Resource",
        "snapshot": {
            "element": [{
                "id": "Resource",
                "path": "Resource"
            }]
        }
    }));
    validator.register_profile(&json!({
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/communication-target",
        "type": "Communication",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/Communication",
        "snapshot": {
            "element": [{
                "id": "Communication.about",
                "path": "Communication.about",
                "min": 0,
                "max": "*",
                "type": [{
                    "code": "Reference",
                    "targetProfile": [
                        "http://example.org/StructureDefinition/resource-target"
                    ]
                }]
            }]
        }
    }));

    let communication = json!({
        "resourceType": "Communication",
        "status": "completed",
        "about": [{
            "reference": "Patient/example"
        }]
    });

    let result = validator
        .validate_with_profile(
            &communication,
            "http://example.org/StructureDefinition/communication-target",
        )
        .unwrap();

    assert!(!result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.message.contains("implied by the reference URL")
            && i.path.as_deref() == Some("Communication.about")
    }));
}

#[test]
fn base_profile_rejects_unknown_resource_property() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "unknownElement": "foo"
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Structure
            && i.message.contains("Unrecognized property 'unknownElement'")
            && i.path.as_deref() == Some("Patient")
    }));
}

#[test]
fn primitive_date_format_rejects_non_date_text() {
    let validator = FhirValidator::new(FhirVersion::R4, None).unwrap();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "birthDate": "not a date"
    });

    let result = validator.validate_auto(&patient).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::Invalid
            && i.message.contains("Not a valid date format: 'not a date'")
            && i.path.as_deref() == Some("Patient.birthDate")
    }));
}

#[test]
fn terminology_rejects_unknown_code_in_supported_codesystem() {
    let validator =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))
            .unwrap();

    let observation = json!({
        "resourceType": "Observation",
        "id": "obs-temp-code2",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://snomed.info/sct",
                "code": "276885007x"
            }]
        }
    });

    let result = validator.validate_auto(&observation).unwrap();

    assert!(result.issues.iter().any(|i| {
        i.severity == Severity::Error
            && i.code == IssueCode::CodeInvalid
            && i.message.contains("276885007x")
            && i.path.as_deref() == Some("Observation.code.coding[0].code")
    }));
}
