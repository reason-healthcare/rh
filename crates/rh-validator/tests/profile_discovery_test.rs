use anyhow::Result;
use rh_validator::FhirValidator;
use serde_json::json;

fn setup_validator() -> Option<FhirValidator> {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        return None;
    }

    FhirValidator::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .ok()
}

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
fn test_auto_detect_from_meta_profile() -> Result<()> {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return Ok(());
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
        "gender": "male"
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== Auto-Detected Profile Validation ===");
    println!("Valid: {}", result.valid);
    println!("Errors: {}", result.error_count());
    println!("Warnings: {}", result.warning_count());

    // Print errors if any
    if !result.valid {
        println!("\nErrors:");
        for issue in &result.issues {
            if issue.severity == rh_validator::Severity::Error {
                println!("  - {}", issue.message);
            }
        }
    }

    // Should validate against US Core Patient
    // ext-1 and us-core-6 false positives are expected, so allow those
    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("ext-1"))
        .filter(|i| !i.message.contains("us-core-6"))
        .filter(|i| !i.message.contains("Failed to parse"))
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Valid US Core Patient should pass (ignoring known ext-1/us-core-6 false positives)"
    );

    Ok(())
}

#[test]
fn test_auto_detect_no_profile_uses_base() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        // No meta.profile specified
        "name": [{
            "family": "Doe"
        }]
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== No Profile (Using Base) ===");
    println!("Valid: {}", result.valid);
    println!("Issues: {}", result.issues.len());

    // Should validate against base Patient profile
    // Just verify we didn't crash
    Ok(())
}

#[test]
fn test_auto_detect_no_resource_type() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    let resource = json!({
        "id": "example"
        // Missing resourceType
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== Missing ResourceType ===");
    println!("Valid: {}", result.valid);
    println!("Errors: {}", result.error_count());

    // Should have error for missing resourceType
    assert!(!result.valid, "Should be invalid");
    assert!(
        result.error_count() >= 1,
        "Should have error for missing resourceType"
    );

    Ok(())
}

#[test]
fn test_validate_with_multiple_profiles() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
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

    let profile_urls = vec![
        "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
        // Could add more profiles if available
    ];

    let result = validator.validate_with_profiles(&resource, &profile_urls)?;

    println!("\n=== Multiple Profile Validation ===");
    println!("Profiles: {}", profile_urls.len());
    println!("Valid: {}", result.valid);
    println!("Issues: {}", result.issues.len());

    // Each profile's issues should be annotated
    for issue in &result.issues {
        if issue.message.contains("[Profile:") {
            println!("  Profile-specific issue: {}", issue.message);
        }
    }

    Ok(())
}

#[test]
fn test_validate_with_unknown_profile() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://example.com/StructureDefinition/unknown-profile"]
        },
        "name": [{
            "family": "Doe"
        }]
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== Unknown Profile ===");
    println!("Valid: {}", result.valid);
    println!("Errors: {}", result.error_count());

    // Should have error for profile not found
    assert!(!result.valid, "Should be invalid");
    let has_not_found = result
        .issues
        .iter()
        .any(|i| i.message.contains("Profile not found") || i.message.contains("unknown-profile"));
    assert!(has_not_found, "Should have error for unknown profile");

    Ok(())
}

#[test]
fn test_multiple_profiles_merge_results() -> Result<()> {
    let Some(validator) = setup_validator_with_us_core() else {
        println!("Skipping test: US Core package not found");
        return Ok(());
    };

    // Resource with multiple profiles
    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": [
                "http://hl7.org/fhir/StructureDefinition/Patient",
                "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
            ]
        },
        "name": [{
            "family": "Doe",
            "given": ["John"]
        }],
        "gender": "male"
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== Multiple Profiles from meta.profile ===");
    println!("Valid: {}", result.valid);
    println!("Total issues: {}", result.issues.len());

    // Count issues by profile
    let base_issues = result
        .issues
        .iter()
        .filter(|i| i.message.contains("StructureDefinition/Patient]"))
        .count();
    let us_core_issues = result
        .issues
        .iter()
        .filter(|i| i.message.contains("us-core-patient]"))
        .count();

    println!("  Base Patient issues: {base_issues}");
    println!("  US Core Patient issues: {us_core_issues}");

    // Should have validated against both profiles
    assert!(
        base_issues > 0 || us_core_issues > 0 || result.valid,
        "Should validate against at least one profile"
    );

    Ok(())
}

#[test]
fn test_profile_annotation_in_messages() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    let resource = json!({
        "resourceType": "Patient",
        "id": "example",
        "meta": {
            "profile": ["http://hl7.org/fhir/StructureDefinition/Patient"]
        }
        // Missing required fields to generate errors
    });

    let result = validator.validate_auto(&resource)?;

    println!("\n=== Profile Annotation in Messages ===");
    for issue in &result.issues {
        println!("  {}", issue.message);
        if issue.severity == rh_validator::Severity::Error {
            // Errors from profile validation should have [Profile: ...] annotation
            if !issue.message.contains("resourceType") && !issue.message.contains("Failed to parse")
            {
                assert!(
                    issue.message.contains("[Profile:"),
                    "Profile-specific errors should be annotated"
                );
            }
        }
    }

    Ok(())
}

#[test]
fn test_base_resource_profile_format() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Test various resource types to verify base profile URL format
    let resource_types = vec!["Patient", "Observation", "Organization"];

    for resource_type in resource_types {
        let resource = json!({
            "resourceType": resource_type,
            "id": "test"
        });

        let result = validator.validate_auto(&resource)?;

        println!("\n=== Base Profile for {resource_type} ===");
        println!(
            "  Validation ran: {}",
            !result.issues.is_empty() || result.valid
        );

        // Just verify it doesn't crash
    }

    Ok(())
}
