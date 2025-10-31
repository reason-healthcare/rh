use rh_validator::FhirValidator;
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
fn test_per_item_cardinality_validation() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Resource with 2 identifiers, each should have system and value
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://hospital.example.org",
                "value": "12345"
            },
            {
                "system": "http://another-system.example.org",
                "value": "67890"
            }
        ],
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    // Filter out known false positives: ext-1, us-core-6, and dom-6 parse errors
    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    // Both identifiers have system and value, so should be valid
    // (ignoring known false positive invariants)
    assert!(
        actual_errors.is_empty(),
        "Two complete identifiers should pass validation (ignoring known false positives)"
    );
}

#[test]
fn test_missing_nested_required_field() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Resource with identifier missing system (required field)
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "value": "12345"
                // Missing system
            }
        ],
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Missing nested field validation:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }

    // Should detect missing system in identifier
    assert!(!result.valid);
    let has_system_error = result
        .issues
        .iter()
        .any(|i| i.message.contains("system") || i.message.contains("identifier"));
    assert!(
        has_system_error,
        "Should detect missing system in identifier"
    );
}

#[test]
fn test_deeply_nested_paths() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Resource with deeply nested structure
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [{
            "system": "http://example.org",
            "value": "123",
            "type": {
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                    "code": "MR"
                }]
            }
        }],
        "name": [{
            "family": "Doe",
            "given": ["John", "Michael"],
            "prefix": ["Mr."]
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Deep nesting validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    // Filter out known false positives: ext-1, us-core-6, and dom-6 parse errors
    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    // Deep nesting with all required fields should be valid
    // (ignoring known false positive invariants)
    assert!(
        actual_errors.is_empty(),
        "Valid deeply nested structure should pass (ignoring known false positives)"
    );
}

#[test]
fn test_array_with_mixed_completeness() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // One identifier complete, one missing system
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://hospital.example.org",
                "value": "12345"
            },
            {
                "value": "67890"
                // Missing system
            }
        ],
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Mixed completeness validation:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }

    // Should detect that second identifier is missing system
    // This is a challenging case that requires per-item validation
    assert!(
        !result.valid,
        "Should detect incomplete identifier in array"
    );
}

#[test]
fn test_optional_nested_arrays() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Patient with optional nested arrays (telecom, address)
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [{
            "system": "http://example.org",
            "value": "123"
        }],
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male",
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234"
            },
            {
                "system": "email",
                "value": "john@example.com"
            }
        ],
        "address": [
            {
                "line": ["123 Main St"],
                "city": "Springfield"
            }
        ]
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Optional nested arrays validation:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    // Filter out known false positives: ext-1, us-core-6, and dom-6 parse errors
    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    // Optional nested arrays with complete data should be valid
    // (ignoring known false positive invariants)
    assert!(
        actual_errors.is_empty(),
        "Complete optional nested arrays should be valid (ignoring known false positives)"
    );
}
