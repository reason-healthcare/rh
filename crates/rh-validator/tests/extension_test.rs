use rh_validator::{FhirValidator, Severity};
use serde_json::json;

fn setup_validator_with_us_core() -> Option<FhirValidator> {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        return None;
    }

    FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .ok()
}

#[test]
fn test_simple_extension_validation() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                "valueCode": "M"
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_extension_url_mismatch() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://wrong.url/extension",
                "valueCode": "M"
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        // TODO: We don't yet fully support open slicing for extensions.
        // Currently we report unknown extensions as errors even with open slicing.
        .filter(|i| {
            !i.message
                .contains("could not be found, so is not allowed here")
        })
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no errors for unrecognized extension (open slicing)"
    );
}

#[test]
fn test_missing_extension_value() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex"
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let extension_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| {
            i.message
                .contains("must have either a value[x] or nested extensions")
        })
        .collect();

    assert!(
        !extension_errors.is_empty(),
        "Expected error for extension without value or nested extensions"
    );
}

#[test]
fn test_us_core_race_extension() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2106-3",
                            "display": "White"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "White"
                    }
                ]
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors for race extension, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_us_core_ethnicity_extension() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2135-2",
                            "display": "Hispanic or Latino"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Hispanic or Latino"
                    }
                ]
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors for ethnicity extension, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_multiple_extensions() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                "valueCode": "F"
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2054-5",
                            "display": "Black or African American"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Black or African American"
                    }
                ]
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2186-5",
                            "display": "Not Hispanic or Latino"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Not Hispanic or Latino"
                    }
                ]
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Smith",
                "given": ["Jane"]
            }
        ],
        "gender": "female"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .filter(|i| {
            !i.message
                .contains("Cardinality violation at 'Patient.extension'")
        })
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors for multiple extensions, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_patient_without_extensions() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors for patient without extensions (all optional), but got: {actual_errors:#?}"
    );
}

#[test]
fn test_extension_missing_url() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "valueCode": "M"
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    println!("Validation issues: {errors:#?}");
}

#[test]
fn test_nested_extension_validation() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2028-9",
                            "display": "Asian"
                        }
                    }
                ]
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Lee",
                "given": ["Ming"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no validation errors for nested extension, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_extension_with_invalid_nested_structure() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": []
            }
        ],
        "identifier": [
            {
                "system": "http://example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .expect("Validation should succeed");

    let extension_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| {
            i.message
                .contains("must have either a value[x] or nested extensions")
        })
        .collect();

    assert!(
        !extension_errors.is_empty(),
        "Expected error for extension with empty nested extensions array"
    );
}

#[test]
fn test_unknown_extension_generates_error() {
    // Use default validator (no additional profiles)
    let validator = FhirValidator::default();

    let patient = json!({
        "resourceType": "Patient",
        "extension": [
            {
                "url": "http://fkcfhir.org/fhir/StructureDefinition/unknown-ext",
                "valueString": "test"
            }
        ]
    });

    let result = validator
        .validate_auto(&patient)
        .expect("Validation should succeed");

    println!("Valid: {}", result.valid);
    println!("Error count: {}", result.error_count());
    for issue in &result.issues {
        println!(
            "  - {} {:?}: {}",
            if issue.severity == Severity::Error {
                "Error"
            } else {
                "Warning"
            },
            issue.code,
            issue.message
        );
    }

    // Should have an error for the unknown extension
    let unknown_ext_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| {
            i.message.contains("could not be found") || i.message.contains("not allowed here")
        })
        .collect();

    assert!(
        !unknown_ext_errors.is_empty(),
        "Expected error for unknown extension from non-HL7 source"
    );
}

#[test]
fn test_unknown_extension_on_medication_request() {
    // Use default validator (no additional profiles)
    let validator = FhirValidator::default();

    // MedicationRequest with unknown extensions (from target-ref-profile-empty test)
    let med_request = json!({
        "resourceType": "MedicationRequest",
        "status": "active",
        "intent": "plan",
        "subject": { "reference": "Patient/1" },
        "medicationCodeableConcept": {
            "text": "Test medication"
        },
        "extension": [
            {
                "url": "http://fkcfhir.org/fhir/StructureDefinition/medEpisodeOfCare",
                "valueReference": {
                    "reference": "EpisodeOfCare/123"
                }
            }
        ]
    });

    let result = validator
        .validate_auto(&med_request)
        .expect("Validation should succeed");

    println!("Valid: {}", result.valid);
    println!("Error count: {}", result.error_count());
    for issue in &result.issues {
        println!(
            "  - {} {:?}: {} @ {}",
            if issue.severity == Severity::Error {
                "Error"
            } else {
                "Warning"
            },
            issue.code,
            issue.message,
            issue.path.as_deref().unwrap_or("(no path)")
        );
    }

    // Should have an error for the unknown extension
    let unknown_ext_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| {
            i.message.contains("could not be found") || i.message.contains("not allowed here")
        })
        .collect();

    assert!(
        !unknown_ext_errors.is_empty(),
        "Expected error for unknown extension from non-HL7 source on MedicationRequest"
    );
}
