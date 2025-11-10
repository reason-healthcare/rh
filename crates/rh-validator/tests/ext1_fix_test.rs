use anyhow::Result;
use rh_validator::{FhirValidator, FhirVersion};
use serde_json::json;

#[test]
fn test_ext1_not_over_applied() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    // Simple patient with no extensions
    let patient = json!({
        "resourceType": "Patient",
        "id": "test",
        "active": true
    });

    let result = validator.validate_auto(&patient)?;

    let ext1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|issue| issue.message.contains("ext-1"))
        .collect();

    println!("Total issues: {}", result.issues.len());
    for issue in &result.issues {
        println!("  - {}: {}", issue.severity, issue.message);
    }

    assert_eq!(
        ext1_errors.len(),
        0,
        "ext-1 should not apply to Patient without extensions. Found {} ext-1 errors.",
        ext1_errors.len()
    );

    Ok(())
}
