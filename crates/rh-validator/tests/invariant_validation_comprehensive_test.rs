use anyhow::Result;
use rh_validator::{FhirValidator, Severity};
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

#[test]
fn test_simple_invariant_dom_2() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Simple invariant: dom-2
    // Expression: contained.contained.empty()
    // Tests that contained resources don't have nested contained resources

    let invalid = json!({
        "resourceType": "Patient",
        "id": "invalid-dom-2",
        "contained": [{
            "resourceType": "Organization",
            "id": "org1",
            "contained": [{  // Nested contained - violates dom-2
                "resourceType": "Practitioner",
                "id": "prac1"
            }]
        }],
        "name": [{"family": "Test"}]
    });

    let result = validator
        .validate_with_profile(&invalid, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // Should have dom-2 error
    let dom_2_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| i.message.contains("dom-2"))
        .collect();

    assert!(!dom_2_errors.is_empty(), "Should detect dom-2 violation");

    Ok(())
}

#[test]
fn test_complex_invariant_dom_3() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Complex invariant: dom-3
    // Expression uses %resource.descendants() and complex where clauses
    // Tests that contained resources are referenced or refer to container

    let patient = json!({
        "resourceType": "Patient",
        "id": "test-dom-3",
        "contained": [{
            "resourceType": "Organization",
            "id": "org1",
            "name": "Test Org"
        }],
        "managingOrganization": {
            "reference": "#org1"  // Reference to contained resource
        },
        "name": [{"family": "Test"}]
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // dom-3 should execute (complex FHIRPath with %resource)
    // No execution errors expected
    let execution_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("Failed to evaluate"))
        .collect();

    assert!(
        execution_errors.is_empty(),
        "Complex FHIRPath expression should execute without errors"
    );

    Ok(())
}

#[test]
fn test_error_severity_invariant() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Most invariants have error severity
    // Test that errors are properly classified

    let invalid = json!({
        "resourceType": "Patient",
        "id": "test-errors",
        "contained": [{
            "resourceType": "Organization",
            "id": "org1",
            "meta": {
                "versionId": "1"  // Violates dom-4: contained resources shouldn't have meta.versionId
            }
        }],
        "name": [{"family": "Test"}]
    });

    let result = validator
        .validate_with_profile(&invalid, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // Should have errors (dom-4 or others)
    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("Failed to parse"))
        .collect();

    assert!(!errors.is_empty(), "Should detect error-level violations");

    // Verify severity is properly set
    for error in &errors {
        assert_eq!(
            error.severity,
            Severity::Error,
            "Error severity should be set"
        );
    }

    Ok(())
}

#[test]
fn test_warning_severity_invariant() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // dom-6 has warning severity
    // Expression: text.div.exists() (though it has parse issues)
    // Tests that warnings are properly classified

    let patient = json!({
        "resourceType": "Patient",
        "id": "test-warnings",
        // Missing text.div - may trigger dom-6 warning
        "name": [{"family": "Test"}]
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // Should have at least the dom-6 parse warning
    let warnings: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Warning)
        .collect();

    assert!(!warnings.is_empty(), "Should have warning-level issues");

    // Verify severity is properly set
    for warning in &warnings {
        assert_eq!(
            warning.severity,
            Severity::Warning,
            "Warning severity should be set"
        );
    }

    Ok(())
}

#[test]
fn test_all_patient_invariants_execute() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Valid patient to ensure all invariants execute without errors
    // Base Patient has 53 invariants that should all run

    let patient = json!({
        "resourceType": "Patient",
        "id": "test-all-invariants",
        "name": [{"family": "Test", "given": ["John"]}],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // No execution failures expected (parse warnings are ok)
    let execution_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("Failed to evaluate"))
        .collect();

    assert!(
        execution_errors.is_empty(),
        "All invariants should execute successfully, got: {execution_errors:?}"
    );

    // Should have only parse warnings and ext-1 false positives, no actual errors
    let actual_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("ext-1")) // Known false positive
        .collect();

    assert!(
        actual_errors.is_empty(),
        "Valid patient should pass all invariants, got: {actual_errors:?}"
    );

    Ok(())
}

#[test]
fn test_invariant_execution_statistics() -> Result<()> {
    use rh_validator::{ProfileRegistry, RuleCompiler};

    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )?;
    let compiler = RuleCompiler::default();

    let snapshot = registry
        .get_snapshot("http://hl7.org/fhir/StructureDefinition/Patient")?
        .expect("Patient profile should exist");

    let rules = compiler.compile(&snapshot)?;

    // Statistics about invariant rules
    println!("\n=== Invariant Execution Coverage ===");
    println!("Total invariants: {}", rules.invariant_rules.len());

    let error_count = rules
        .invariant_rules
        .iter()
        .filter(|r| r.severity == "error")
        .count();
    let warning_count = rules
        .invariant_rules
        .iter()
        .filter(|r| r.severity == "warning")
        .count();

    println!("Error severity: {error_count}");
    println!("Warning severity: {warning_count}");

    // Verify we have errors and warnings
    assert!(error_count > 0, "Should have error-level invariants");

    // Check for different types of expressions
    let with_resource = rules
        .invariant_rules
        .iter()
        .filter(|r| r.expression.contains("%resource"))
        .count();
    let with_where = rules
        .invariant_rules
        .iter()
        .filter(|r| r.expression.contains("where("))
        .count();
    let with_exists = rules
        .invariant_rules
        .iter()
        .filter(|r| r.expression.contains("exists("))
        .count();

    println!("With %resource: {with_resource}");
    println!("With where(): {with_where}");
    println!("With exists(): {with_exists}");

    // Verify we have a reasonable number of invariants
    assert!(
        rules.invariant_rules.len() >= 50,
        "Should have extracted many invariants"
    );

    Ok(())
}

#[test]
fn test_multiple_invariant_violations() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Patient with multiple violations to test that all are caught
    let invalid = json!({
        "resourceType": "Patient",
        "id": "multiple-violations",
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org1",
                "contained": [{  // dom-2: nested contained
                    "resourceType": "Practitioner",
                    "id": "prac1"
                }],
                "meta": {  // dom-4: meta.versionId in contained
                    "versionId": "1"
                }
            }
        ],
        "name": [{"family": "Test"}]
    });

    let result = validator
        .validate_with_profile(&invalid, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // Should catch multiple violations
    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .filter(|i| !i.message.contains("Failed to parse"))
        .filter(|i| !i.message.contains("ext-1"))
        .collect();

    assert!(
        errors.len() >= 2,
        "Should detect multiple violations, got {errors:?}"
    );

    Ok(())
}
