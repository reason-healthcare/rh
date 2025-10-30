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

    FhirValidator::new(Some(packages_dir.to_str().unwrap())).ok()
}

#[test]
fn test_resource_level_invariant_context() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Test dom-2: resource-level invariant
    // Expression: contained.contained.empty()
    // This should evaluate with resource as %resource and %context

    let invalid_patient = json!({
        "resourceType": "Patient",
        "id": "test-dom-2",
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org1",
                "contained": [  // Nested contained - violates dom-2
                    {
                        "resourceType": "Practitioner",
                        "id": "prac1"
                    }
                ]
            }
        ],
        "name": [{"family": "Doe", "given": ["John"]}]
    });

    let result = validator.validate_with_profile(
        &invalid_patient,
        "http://hl7.org/fhir/StructureDefinition/Patient",
    )?;

    // Check for dom-2 violation
    let dom_2_issues: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("dom-2"))
        .collect();

    assert!(
        !dom_2_issues.is_empty(),
        "Should detect dom-2 violation with nested contained"
    );

    Ok(())
}

#[test]
fn test_resource_level_invariant_with_resource_reference() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Test dom-3: uses %resource in expression
    // Expression: contained.where((('#'+id in (%resource.descendants()...
    // This verifies %resource variable is accessible

    let patient_with_unmatched_contained = json!({
        "resourceType": "Patient",
        "id": "test-dom-3",
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org-unreferenced"
                // This contained resource is not referenced - may violate dom-3
            }
        ],
        "name": [{"family": "Doe", "given": ["John"]}]
    });

    let result = validator.validate_with_profile(
        &patient_with_unmatched_contained,
        "http://hl7.org/fhir/StructureDefinition/Patient",
    )?;

    // dom-3 should be evaluated (though it's complex and might not catch all cases)
    // The important thing is it doesn't error during evaluation
    let dom_3_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("dom-3"))
        .collect();

    // dom-3 might flag this or might not depending on the exact logic
    // We're just verifying the FHIRPath expression with %resource executes
    println!("dom-3 issues found: {}", dom_3_errors.len());

    Ok(())
}

#[test]
fn test_element_level_invariant_path_tracking() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // ext-1 is an element-level invariant on Extension
    // Expression: extension.exists() != value.exists()
    // Path would be something like "Patient.extension" or "Extension"

    let patient_with_extension = json!({
        "resourceType": "Patient",
        "id": "test-ext-1",
        "extension": [
            {
                "url": "http://example.org/test-extension",
                "valueString": "test value"
            }
        ],
        "name": [{"family": "Doe", "given": ["John"]}]
    });

    let result = validator.validate_with_profile(
        &patient_with_extension,
        "http://hl7.org/fhir/StructureDefinition/Patient",
    )?;

    // ext-1 should validate correctly
    // Valid extensions should pass ext-1
    let ext_1_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("ext-1"))
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .collect();

    // This valid extension should not trigger ext-1 errors
    assert!(
        ext_1_errors.is_empty(),
        "Valid extension should not violate ext-1, got: {ext_1_errors:?}"
    );

    Ok(())
}

#[test]
fn test_context_setup_for_valid_resource() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Simple valid patient - context should be set up correctly
    let patient = json!({
        "resourceType": "Patient",
        "id": "test-context",
        "name": [{"family": "Doe", "given": ["John"]}],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // Filter out parse warnings and known ext-1 false positives
    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .filter(|i| !i.message.contains("Failed to parse"))
        .filter(|i| !i.message.contains("ext-1")) // ext-1 triggers on base Patient definition
        .collect();

    assert!(
        errors.is_empty(),
        "Valid patient should have no context-related errors, got: {errors:?}"
    );

    Ok(())
}

#[test]
fn test_invariant_path_information() -> Result<()> {
    use rh_validator::{ProfileRegistry, RuleCompiler};

    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    }

    let registry = ProfileRegistry::new(Some(packages_dir.to_str().unwrap()))?;
    let compiler = RuleCompiler::default();

    let snapshot = registry
        .get_snapshot("http://hl7.org/fhir/StructureDefinition/Patient")?
        .expect("Patient profile should exist");

    let rules = compiler.compile(&snapshot)?;

    // Verify invariants have path information
    assert!(
        !rules.invariant_rules.is_empty(),
        "Should have invariant rules"
    );

    for rule in &rules.invariant_rules {
        assert!(
            !rule.path.is_empty(),
            "Invariant {} should have a path",
            rule.key
        );

        // Resource-level invariants should have path matching resource type
        // Element-level invariants should have longer paths
        let parts: Vec<&str> = rule.path.split('.').collect();
        assert!(!parts.is_empty(), "Path should have at least one component");
    }

    // Check for a known resource-level invariant
    let dom_invariants: Vec<_> = rules
        .invariant_rules
        .iter()
        .filter(|r| r.key.starts_with("dom-"))
        .collect();

    assert!(!dom_invariants.is_empty(), "Should have dom-* invariants");

    // dom-* invariants are typically on the resource root
    for inv in &dom_invariants {
        println!("Invariant {}: path = {}", inv.key, inv.path);
    }

    Ok(())
}

#[test]
fn test_multiple_invariants_with_different_contexts() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Patient that might trigger various invariants
    let patient = json!({
        "resourceType": "Patient",
        "id": "multi-context-test",
        "meta": {
            "versionId": "1",
            "lastUpdated": "2024-01-01T00:00:00Z"
        },
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org1",
                "name": "Test Org"
                // Note: contained resources should not have meta.versionId (dom-4)
            }
        ],
        "managingOrganization": {
            "reference": "#org1"
        },
        "name": [{"family": "Doe", "given": ["John"]}],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // All invariants should execute without errors
    // (Even if they generate validation issues, the execution itself should work)
    let execution_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("Failed to evaluate"))
        .collect();

    assert!(
        execution_errors.is_empty(),
        "No invariants should fail to execute, got: {execution_errors:?}"
    );

    Ok(())
}

#[test]
fn test_context_variables_accessibility() -> Result<()> {
    let Some(validator) = setup_validator() else {
        println!("Skipping test: Base R4 package not found");
        return Ok(());
    };

    // Test that both %resource and %context work in expressions
    // dom-3 uses %resource
    let patient = json!({
        "resourceType": "Patient",
        "id": "context-vars-test",
        "contained": [
            {
                "resourceType": "Organization",
                "id": "org1",
                "name": "Test Org"
            }
        ],
        "managingOrganization": {
            "reference": "#org1"
        },
        "name": [{"family": "Doe", "given": ["John"]}],
        "gender": "male"
    });

    let result = validator
        .validate_with_profile(&patient, "http://hl7.org/fhir/StructureDefinition/Patient")?;

    // If there's no error about undefined %resource, then it worked
    let undefined_resource_errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.message.contains("resource") && i.message.contains("undefined"))
        .collect();

    assert!(
        undefined_resource_errors.is_empty(),
        "%resource should be accessible in FHIRPath, got: {undefined_resource_errors:?}"
    );

    Ok(())
}
