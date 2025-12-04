//! Example demonstrating terminology service integration for FHIR validation.
//!
//! This example shows how to configure a terminology server for validating
//! display names and code memberships in Coding elements.
//!
//! Run with: `cargo run -p rh-validator --example terminology_validation`

use rh_validator::{FhirValidator, FhirVersion, TerminologyConfig};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    println!("=== Terminology Validation Example ===\n");

    // Example 1: Validation WITHOUT terminology service (default)
    println!("1. Validation without terminology service (default behavior):");
    println!("   Display names are not validated.\n");

    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    let patient_bad_display = json!({
        "resourceType": "Patient",
        "id": "example",
        "contact": [{
            "relationship": [{
                "coding": [{
                    "system": "http://hl7.org/fhir/v3/RoleCode",
                    "code": "MTH",
                    "display": "Cother"  // Wrong display - should be "mother"
                }]
            }]
        }]
    });

    let result = validator.validate(&patient_bad_display)?;
    println!("   Valid: {} (wrong display not detected)", result.valid);
    println!();

    // Example 2: Validation WITH mock terminology service
    println!("2. Validation with mock terminology service:");
    println!("   Uses built-in common codes for testing.\n");

    let validator_with_mock =
        FhirValidator::with_terminology(FhirVersion::R4, None, Some(TerminologyConfig::mock()))?;

    let result = validator_with_mock.validate(&patient_bad_display)?;
    println!("   Valid: {}", result.valid);
    for issue in &result.issues {
        println!("   - [{:?}] {}", issue.severity, issue.message);
    }
    println!();

    // Example 3: Correct display name
    println!("3. Validation with correct display name:\n");

    let patient_good_display = json!({
        "resourceType": "Patient",
        "id": "example",
        "contact": [{
            "relationship": [{
                "coding": [{
                    "system": "http://hl7.org/fhir/v3/RoleCode",
                    "code": "MTH",
                    "display": "mother"  // Correct display
                }]
            }]
        }]
    });

    let result = validator_with_mock.validate(&patient_good_display)?;
    println!("   Valid: {}", result.valid);
    println!();

    // Example 4: Using real terminology server with persistent cache
    println!("4. Configuration for real terminology server:");
    println!("   (Not executed - requires network access)\n");

    println!("   // With tx.fhir.org and persistent cache:");
    println!("   let validator = FhirValidator::with_terminology(");
    println!("       FhirVersion::R4,");
    println!("       Some(\"~/.fhir/packages\"),");
    println!("       Some(TerminologyConfig::fhir_tx().with_default_cache())");
    println!("   )?;");
    println!();

    println!("   // With custom terminology server:");
    println!("   let validator = FhirValidator::with_terminology(");
    println!("       FhirVersion::R4,");
    println!("       None,");
    println!("       Some(TerminologyConfig::with_server(\"https://my-tx.example.com/r4\")");
    println!("           .with_cache_path(PathBuf::from(\"/tmp/terminology-cache\")))");
    println!("   )?;");
    println!();

    // Example 5: LOINC code validation
    println!("5. LOINC code validation (mock service):\n");

    let observation = json!({
        "resourceType": "Observation",
        "id": "heart-rate",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "8867-4",
                "display": "Heart rate"  // Correct display
            }]
        }
    });

    let result = validator_with_mock.validate(&observation)?;
    println!("   Heart rate observation: Valid = {}", result.valid);

    let observation_wrong = json!({
        "resourceType": "Observation",
        "id": "heart-rate",
        "status": "final",
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "8867-4",
                "display": "Cardiac rhythm"  // Wrong display
            }]
        }
    });

    let result = validator_with_mock.validate(&observation_wrong)?;
    println!("   Wrong display: Valid = {}", result.valid);
    for issue in &result.issues {
        println!("   - {}", issue.message);
    }
    println!();

    println!("=== Example Complete ===");

    Ok(())
}
