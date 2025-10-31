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
fn test_primitive_type_string_valid() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // gender should be a string
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
        println!("String type validation errors:");
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

    assert!(actual_errors.is_empty(), "Valid string should pass");
}

#[test]
fn test_primitive_type_string_invalid() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // gender should be a string, not a number
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
        "gender": 123
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Invalid type validation:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }

    // Should detect type mismatch
    assert!(!result.valid, "Invalid type should be detected");
    let has_type_error = result
        .issues
        .iter()
        .any(|i| i.message.contains("type") || i.message.contains("gender"));
    assert!(has_type_error, "Should have type error for gender");
}

#[test]
fn test_primitive_type_boolean_valid() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

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
        "active": true
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Boolean type validation errors:");
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

    assert!(actual_errors.is_empty(), "Valid boolean should pass");
}

#[test]
fn test_complex_type_humanname() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // name should be HumanName (object with family, given, etc.)
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
            "given": ["John", "Q"],
            "prefix": ["Mr."],
            "suffix": ["Jr."]
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
        println!("Complex type validation errors:");
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

    assert!(actual_errors.is_empty(), "Valid complex type should pass");
}

#[test]
fn test_complex_type_invalid_structure() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // name should be an object, not a string
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
        "name": ["John Doe"],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Invalid complex type validation:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }

    // Should detect that name array contains string instead of HumanName objects
    assert!(
        !result.valid,
        "Invalid complex type structure should be detected"
    );
}

#[test]
fn test_array_of_strings() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // given is an array of strings
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
            "given": ["John", "Q", "Public"]
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
        println!("Array of strings validation errors:");
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
        "Valid array of strings should pass"
    );
}

#[test]
fn test_array_of_strings_invalid_element() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // given should be array of strings, not numbers
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
            "given": ["John", 123, "Public"]
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Invalid array element type validation:");
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }

    // Should detect that one array element has wrong type
    assert!(
        !result.valid,
        "Invalid element type in array should be detected"
    );
}

#[test]
fn test_reference_type() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // managingOrganization is a Reference
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
        "managingOrganization": {
            "reference": "Organization/example"
        }
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Reference type validation errors:");
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

    assert!(actual_errors.is_empty(), "Valid Reference should pass");
}

#[test]
fn test_multiple_types_accepted() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    // deceased[x] can be boolean or dateTime
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
        "deceasedBoolean": false
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    if !result.valid {
        println!("Choice type validation errors:");
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

    assert!(actual_errors.is_empty(), "Valid choice type should pass");
}
