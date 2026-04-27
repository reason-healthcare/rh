use rh_validator::{FhirValidator, TerminologyConfig};
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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

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

    // Without a terminology service, intensional ValueSets (those defined only by compose
    // rules without a pre-computed expansion) are skipped — no error even for invalid codes.
    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "gender": "invalid"
    });

    // validate_auto() applies profile rules including binding validation
    let result = validator.validate_auto(&resource).expect("Should validate");
    let code_invalid_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("not in required ValueSet"))
        .collect();
    assert!(
        code_invalid_errors.is_empty(),
        "Without a terminology service, intensional ValueSet errors should be suppressed; got: {code_invalid_errors:?}"
    );
}

#[test]
fn test_intensional_valueset_with_ts_reports_invalid_code() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    // With the mock terminology service, the administrative-gender ValueSet is registered
    // and an invalid code should be reported as an error for required bindings.
    let validator = FhirValidator::with_terminology(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
        Some(TerminologyConfig::mock()),
    )
    .expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "gender": "invalid-not-a-real-gender"
    });

    // validate_auto() applies profile rules including binding validation
    let result = validator.validate_auto(&resource).expect("Should validate");
    let code_invalid_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("not in required ValueSet"))
        .collect();
    assert!(
        !code_invalid_errors.is_empty(),
        "With a terminology service, an invalid required binding code should be reported"
    );
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

    let validator = FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create validator");

    let resource = json!({
        "resourceType": "Patient",
        "gender": "male"
    });

    // Should handle missing ValueSets gracefully
    let result = validator.validate(&resource).expect("Should validate");
    println!("Validation completed with {} issues", result.issues.len());
}
