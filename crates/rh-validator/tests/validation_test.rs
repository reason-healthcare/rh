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
fn test_validate_with_profile_not_found() {
    let validator = FhirValidator::new(None).unwrap();
    let resource = json!({
        "resourceType": "Patient",
        "id": "example"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://example.com/StructureDefinition/fake-profile",
        )
        .unwrap();

    assert!(!result.valid);
    assert_eq!(result.error_count(), 1);
    assert!(result.issues[0].message.contains("Profile not found"));
}

#[test]
fn test_validate_with_us_core_patient_valid() {
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
            "system": "http://hospital.example.org",
            "value": "12345"
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
        println!("Validation issues:");
        for issue in &result.issues {
            println!("  [{:?}] {}: {}", issue.severity, issue.code, issue.message);
        }
    }

    assert!(result.valid || result.error_count() == 0);
}

#[test]
fn test_validate_with_us_core_patient_missing_name() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "identifier": [{
            "system": "http://hospital.example.org",
            "value": "12345"
        }],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    assert!(!result.valid);
    assert!(result.error_count() > 0);

    let name_error = result
        .issues
        .iter()
        .find(|i| i.path.as_ref().map(|p| p.contains("name")).unwrap_or(false));

    assert!(
        name_error.is_some(),
        "Should have error for missing Patient.name"
    );
    println!("Name validation error: {}", name_error.unwrap().message);
}

#[test]
fn test_validate_with_us_core_patient_missing_gender() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "identifier": [{
            "system": "http://hospital.example.org",
            "value": "12345"
        }]
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    assert!(!result.valid);
    assert!(result.error_count() > 0);

    let gender_error = result.issues.iter().find(|i| {
        i.path
            .as_ref()
            .map(|p| p.contains("gender"))
            .unwrap_or(false)
    });

    assert!(
        gender_error.is_some(),
        "Should have error for missing Patient.gender"
    );
    println!("Gender validation error: {}", gender_error.unwrap().message);
}

#[test]
fn test_validate_with_us_core_patient_missing_identifier() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
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

    assert!(!result.valid);
    assert!(result.error_count() > 0);

    let identifier_error = result.issues.iter().find(|i| {
        i.path
            .as_ref()
            .map(|p| p.contains("identifier"))
            .unwrap_or(false)
    });

    assert!(
        identifier_error.is_some(),
        "Should have error for missing Patient.identifier"
    );
    println!(
        "Identifier validation error: {}",
        identifier_error.unwrap().message
    );
}

#[test]
fn test_cardinality_validation_array() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "identifier": [
            {"system": "http://hospital.example.org", "value": "1"},
            {"system": "http://hospital.example.org", "value": "2"}
        ],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Validation with 2 identifiers:");
    println!("  Valid: {}", result.valid);
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());

    for issue in &result.issues {
        if issue.severity == Severity::Error {
            println!("  Error: {} - {}", issue.code, issue.message);
        }
    }
}

#[test]
fn test_path_traversal_nested() {
    let validator = FhirValidator::new(None).unwrap();

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{
            "family": "Doe",
            "given": ["John", "Jacob"]
        }]
    });

    let result = validator.validate(&resource).unwrap();
    assert!(result.valid);
}

#[test]
fn test_validation_result_aggregation() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    println!("Validation of minimal Patient:");
    println!("  Total issues: {}", result.issues.len());
    println!("  Errors: {}", result.error_count());
    println!("  Warnings: {}", result.warning_count());
    println!("  Info: {}", result.info_count());

    assert!(result.error_count() > 0, "Should have multiple errors");

    let mut paths = std::collections::HashSet::new();
    for issue in &result.issues {
        if let Some(path) = &issue.path {
            paths.insert(path);
        }
    }

    println!("  Unique paths with issues: {}", paths.len());
}

#[test]
fn test_validate_auto_detect_profile() {
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
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "identifier": [{
            "system": "http://hospital.example.org",
            "value": "12345"
        }],
        "gender": "male"
    });

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let result = validator
        .validate_with_profile(&resource, profile_url)
        .unwrap();

    println!("Auto-detected profile validation:");
    println!("  Valid: {}", result.valid);
    println!("  Issues: {}", result.issues.len());
}

#[test]
fn test_validate_with_empty_arrays() {
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
        "name": [],
        "identifier": [],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(
            &resource,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    assert!(!result.valid);
    assert!(result.error_count() >= 2);

    let has_name_error = result.issues.iter().any(|i| i.message.contains("name"));
    let has_identifier_error = result
        .issues
        .iter()
        .any(|i| i.message.contains("identifier"));

    assert!(has_name_error, "Should detect missing name");
    assert!(has_identifier_error, "Should detect missing identifier");
}

#[test]
fn test_validate_performance_with_caching() {
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
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "identifier": [{
            "system": "http://hospital.example.org",
            "value": "12345"
        }],
        "gender": "male"
    });

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let _ = validator.validate_with_profile(&resource, profile_url);
    }
    let duration = start.elapsed();

    println!("10 validations took: {duration:?}");
    println!("Average per validation: {:?}", duration / 10);

    let (snapshot_stats, rule_stats) = validator.cache_stats();
    println!(
        "Snapshot cache - size: {}, capacity: {}",
        snapshot_stats.0, snapshot_stats.1
    );
    println!(
        "Rule cache - size: {}, capacity: {}",
        rule_stats.0, rule_stats.1
    );

    assert!(
        rule_stats.0 >= 1,
        "Should have at least 1 cached rule after validations"
    );
    assert!(
        snapshot_stats.0 >= 1,
        "Should have at least 1 cached snapshot after validations"
    );
}

#[test]
fn test_validate_complex_nested_structure() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "complex-example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [
            {
                "system": "http://hospital.example.org",
                "value": "12345"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John", "Q"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234"
            }
        ],
        "gender": "male",
        "birthDate": "1970-01-01",
        "address": [
            {
                "use": "home",
                "line": ["123 Main St"],
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
        println!("Complex structure validation errors:");
        for issue in &result.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }

    assert!(
        result.valid || result.error_count() == 0,
        "Complex valid structure should pass validation"
    );
}

#[test]
fn test_validate_multiple_resources_different_types() {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return;
    };

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "name": [{"family": "Doe", "given": ["John"]}],
        "identifier": [{"system": "http://example.org", "value": "123"}],
        "gender": "male"
    });

    let observation = json!({
        "resourceType": "Observation",
        "id": "example",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "15074-8"
            }]
        },
        "subject": {
            "reference": "Patient/example"
        }
    });

    let patient_result = validator
        .validate_with_profile(
            &patient,
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
        )
        .unwrap();

    let observation_result = validator.validate(&observation).unwrap();

    assert!(patient_result.valid || patient_result.error_count() == 0);
    assert!(observation_result.valid);

    println!("Patient validation: {} issues", patient_result.issues.len());
    println!(
        "Observation validation: {} issues",
        observation_result.issues.len()
    );
}
