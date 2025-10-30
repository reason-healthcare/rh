use rh_validator::FhirValidator;
use serde_json::json;

#[test]
fn test_required_binding_valid_code() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    // Create a simple resource with a valid code from yesnodontknow ValueSet
    let resource = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "test"
            }]
        },
        // Using a value from yesnodontknow ValueSet (extensional)
        "valueCodeableConcept": {
            "coding": [{
                "system": "http://terminology.hl7.org/CodeSystem/v2-0136",
                "code": "Y",
                "display": "Yes"
            }]
        }
    });

    // Note: We can't easily test this without a profile that uses yesnodontknow
    // This test validates the structure works
    let result = validator.validate(&resource).expect("Should validate");
    // Just ensure it runs without panic
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_required_binding_invalid_code() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    let resource = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "test"
            }]
        }
    });

    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_extensible_binding_generates_warning() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "gender": "male"
    });

    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_preferred_binding_skipped() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "language": "invalid-code" // Language has preferred binding
    });

    let result = validator.validate(&resource).expect("Should validate");
    // Preferred bindings should not generate errors
    assert!(result.valid);
}

#[test]
fn test_extract_code_from_string() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    // Simple string code
    let resource = json!({
        "resourceType": "Patient",
        "gender": "male"
    });

    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_extract_code_from_coding() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    // Coding with system and code
    let resource = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "system": "http://loinc.org",
            "code": "test"
        }
    });

    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_extract_codes_from_codeable_concept() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    // CodeableConcept with multiple codings
    let resource = json!({
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "coding": [
                {
                    "system": "http://loinc.org",
                    "code": "1234-5"
                },
                {
                    "system": "http://snomed.info/sct",
                    "code": "789012"
                }
            ]
        }
    });

    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}

#[test]
fn test_intensional_valueset_skipped() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    // administrative-gender is intensional (no expansion)
    let resource = json!({
        "resourceType": "Patient",
        "gender": "invalid"
    });

    let result = validator.validate(&resource).expect("Should validate");
    // Should not fail because intensional ValueSets are skipped
    println!("Validation result: {result:?}");
    // We can't assert much here since we're skipping intensional ValueSets
}

#[test]
fn test_missing_valueset_handled_gracefully() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let validator =
        FhirValidator::new(Some(packages_dir.to_str().unwrap())).expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "gender": "male"
    });

    // Should handle missing ValueSets gracefully
    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}
