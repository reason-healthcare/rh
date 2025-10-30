use rh_validator::{FhirValidator, Severity};
use serde_json::json;

fn setup_validator_with_us_core() -> Option<FhirValidator> {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        return None;
    }

    FhirValidator::new(Some(packages_dir.to_str().unwrap())).ok()
}

#[test]
fn test_us_core_patient_identifier_slicing() {
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
                "system": "http://hospital.example.org/mrn",
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
        "Expected no validation errors with identifier, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_patient_without_identifier() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
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

    let identifier_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| i.message.contains("identifier"))
        .collect();

    assert!(
        !identifier_errors.is_empty(),
        "Expected error for missing required identifier"
    );
}

#[test]
fn test_multiple_identifiers_same_system() {
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
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            },
            {
                "system": "http://hospital.example.org/mrn",
                "value": "67890"
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
        "Expected no errors for multiple identifiers, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_identifier_with_different_systems() {
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
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            },
            {
                "system": "http://hl7.org/fhir/sid/us-ssn",
                "value": "123-45-6789"
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
        "Expected no errors for different identifier systems, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_telecom_slicing() {
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
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john@example.com"
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
        "Expected no errors for telecom, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_address_slicing() {
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
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            }
        ],
        "address": [
            {
                "use": "home",
                "line": ["123 Main St"],
                "city": "Boston",
                "state": "MA",
                "postalCode": "02101"
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
        "Expected no errors for address, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_name_slicing() {
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
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "William"]
            },
            {
                "use": "nickname",
                "given": ["Johnny"]
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
        "Expected no errors for multiple names, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_comprehensive_patient_with_all_slices() {
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
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                "valueCode": "M"
            }
        ],
        "identifier": [
            {
                "system": "http://hospital.example.org/mrn",
                "value": "12345"
            },
            {
                "system": "http://hl7.org/fhir/sid/us-ssn",
                "value": "123-45-6789"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "William"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john.doe@example.com"
            }
        ],
        "gender": "male",
        "birthDate": "1970-01-01",
        "address": [
            {
                "use": "home",
                "line": ["123 Main St", "Apt 4B"],
                "city": "Boston",
                "state": "MA",
                "postalCode": "02101",
                "country": "US"
            }
        ]
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
        "Expected no errors for comprehensive patient, but got: {actual_errors:#?}"
    );
}

#[test]
fn test_slicing_validation_basic() {
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
                "use": "official",
                "system": "http://example.org/identifiers",
                "value": "ABC123"
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
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Expected no slicing errors, but got: {actual_errors:#?}"
    );
}
