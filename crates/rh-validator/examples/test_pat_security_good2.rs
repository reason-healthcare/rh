// Test pat-security-good2 validation
use anyhow::Result;
use rh_validator::{FhirValidator, FhirVersion, Severity};
use std::fs;

fn main() -> Result<()> {
    let validator = FhirValidator::new(FhirVersion::R4, None)?;

    // Read the pat-security-good2.json file
    let json_str = fs::read_to_string(
        std::env::home_dir()
            .unwrap()
            .join("Library/Caches/rh/fhir-test-cases/validator/pat-security-good2.json"),
    )?;

    let resource: serde_json::Value = serde_json::from_str(&json_str)?;

    // Validate
    let result = validator.validate(&resource)?;

    println!("=== pat-security-good2 Validation ===");
    println!("Valid: {}", result.valid);
    println!("Error count: {}", result.error_count());
    println!("Warning count: {}", result.warning_count());
    println!("Info count: {}", result.info_count());
    println!();

    if result.error_count() > 0 {
        println!("ERRORS:");
        for issue in result
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
        {
            println!("  - [{}] {}", issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("    @ {}", path);
            }
        }
        println!();
    }

    if result.warning_count() > 0 {
        println!("WARNINGS:");
        for issue in result
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
        {
            println!("  - [{}] {}", issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("    @ {}", path);
            }
        }
        println!();
    }

    if result.info_count() > 0 {
        println!("INFORMATION:");
        for issue in result
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Information)
        {
            println!("  - [{}] {}", issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("    @ {}", path);
            }
        }
    }

    Ok(())
}
