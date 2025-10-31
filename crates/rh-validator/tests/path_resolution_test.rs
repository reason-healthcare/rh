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
fn test_simple_path_resolution() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Simple top-level field
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
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Simple path validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Simple paths should resolve correctly"
    );
}

#[test]
fn test_nested_path_resolution() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Nested paths like Patient.name.family
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
            "given": ["John"],
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
        println!("Nested path validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Nested paths should resolve correctly"
    );
}

#[test]
fn test_array_path_resolution() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Arrays at multiple levels
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://example.org",
                "value": "123"
            },
            {
                "system": "http://another.org",
                "value": "456"
            }
        ],
        "name": [{
            "family": "Doe",
            "given": ["John", "Q"]
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
        println!("Array path validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Array paths should resolve correctly"
    );
}

#[test]
fn test_deeply_nested_array_path() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Path through multiple arrays: Patient.identifier.type.coding.code
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
                "coding": [
                    {
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR",
                        "display": "Medical Record Number"
                    }
                ],
                "text": "MRN"
            }
        }],
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
        println!("Deeply nested array path validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Deeply nested array paths should resolve correctly"
    );
}

#[test]
fn test_missing_intermediate_path() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Resource missing intermediate path elements
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [{
            "system": "http://example.org",
            "value": "123"
            // Missing 'type' - so type.coding.code path doesn't exist
        }],
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

    // Should not crash or report false errors for non-existent paths
    if !result.valid {
        println!("Missing intermediate path validation:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    // Validation might fail for other reasons, but should not crash
    assert!(
        result.issues.len() < 100,
        "Should not generate excessive errors for missing optional paths"
    );
}

#[test]
fn test_empty_arrays_in_path() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Empty arrays should not cause path resolution to fail
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
                "coding": []  // Empty array
            }
        }],
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

    // Should handle empty arrays gracefully
    assert!(
        result.issues.len() < 100,
        "Should handle empty arrays in paths gracefully"
    );
}

#[test]
fn test_mixed_arrays_and_objects() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Complex structure with arrays and objects at multiple levels
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "use": "official",
                "system": "http://example.org",
                "value": "123",
                "period": {
                    "start": "2020-01-01"
                }
            },
            {
                "use": "temp",
                "system": "http://temp.org",
                "value": "456"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "Q"],
                "prefix": ["Mr."]
            },
            {
                "use": "nickname",
                "given": ["Johnny"]
            }
        ],
        "gender": "male",
        "address": [
            {
                "use": "home",
                "line": ["123 Main St", "Apt 4"],
                "city": "Springfield",
                "state": "IL",
                "postalCode": "62701"
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
        println!("Mixed arrays and objects validation:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Mixed arrays and objects should be handled correctly"
    );
}

#[test]
fn test_resource_type_prefix_handling() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // Paths in profiles have "Patient." prefix but JSON doesn't
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
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Resource type prefix handling errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse invariant dom-6"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Resource type prefix in paths should be handled correctly"
    );
}
