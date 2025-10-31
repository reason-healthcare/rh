//! Profile-based validation example.
//!
//! This example shows how to validate against specific FHIR profiles,
//! such as US Core Patient.

use anyhow::Result;
use rh_validator::FhirValidator;
use serde_json::json;

fn main() -> Result<()> {
    println!("=== Profile-Based Validation Example ===\n");

    // Create validator with FHIR packages directory
    // This loads US Core and other Implementation Guides
    let packages_dir = std::env::var("FHIR_PACKAGES_DIR").ok();
    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, packages_dir.as_deref())?;

    // US Core Patient profile URL
    let us_core_patient = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    // Example 1: Valid US Core Patient
    println!("1. Valid US Core Patient:");
    let valid_patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": [us_core_patient]
        },
        "identifier": [
            {
                "system": "http://hospital.example.org",
                "value": "123456"
            }
        ],
        "name": [
            {
                "family": "Doe",
                "given": ["John"]
            }
        ],
        "gender": "male",
        "birthDate": "1974-12-25"
    });

    let result = validator.validate(&valid_patient)?;
    println!("   Valid: {}", result.valid);
    println!("   Issues: {}", result.issues.len());
    for issue in result.issues {
        println!("   - {}: {}", issue.severity, issue.message);
        if let Some(path) = &issue.path {
            println!("     Path: {path}");
        }
    }
    println!();

    // Example 2: Missing required US Core fields
    println!("2. US Core Patient missing required identifier:");
    let missing_identifier = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": [us_core_patient]
        },
        "name": [
            {
                "family": "Doe",
                "given": ["Jane"]
            }
        ],
        "gender": "female"
    });

    let result = validator.validate(&missing_identifier)?;
    println!("   Valid: {}", result.valid);
    println!("   Issues: {}", result.issues.len());
    for issue in result.issues {
        println!("   - {}: {}", issue.severity, issue.message);
        if let Some(path) = &issue.path {
            println!("     Path: {path}");
        }
    }
    println!();

    // Example 3: Auto-detection from meta.profile
    println!("3. Auto-detection from meta.profile:");
    let auto_detect = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": [us_core_patient]
        },
        "identifier": [{"system": "http://example.org", "value": "123"}],
        "name": [{"family": "Smith"}],
        "gender": "unknown"
    });

    let result = validator.validate(&auto_detect)?;
    println!("   Valid: {}", result.valid);
    println!("   Detected profiles from meta.profile");
    println!("   Issues: {}", result.issues.len());
    println!();

    // Example 4: Explicit profile validation
    println!("4. Explicit profile URL:");
    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{"family": "Jones"}],
        "gender": "male"
    });

    let result = validator.validate_with_profile(&patient, us_core_patient)?;
    println!("   Valid: {}", result.valid);
    println!("   Issues: {}", result.issues.len());
    for issue in result.issues {
        println!("   - {}: {}", issue.severity, issue.message);
    }

    Ok(())
}
