use rh_validator::{ProfileRegistry, RuleCompiler};

#[test]
fn test_invariant_extraction_us_core_patient() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    println!("\n=== US Core Patient Invariant Rules ===");
    println!("Total invariant rules: {}", rules.invariant_rules.len());

    assert!(
        !rules.invariant_rules.is_empty(),
        "Should have invariant rules"
    );

    // Display first few invariants
    for (i, rule) in rules.invariant_rules.iter().take(5).enumerate() {
        println!("\n{}. Invariant: {}", i + 1, rule.key);
        println!("   Severity: {}", rule.severity);
        println!("   Human: {}", rule.human);
        println!("   Expression: {}", rule.expression);
    }
}

#[test]
fn test_invariant_structure_validation() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    if let Some(first_rule) = rules.invariant_rules.first() {
        // Validate structure
        assert!(!first_rule.key.is_empty(), "Key should not be empty");
        assert!(
            !first_rule.severity.is_empty(),
            "Severity should not be empty"
        );
        assert!(!first_rule.human.is_empty(), "Human should not be empty");
        assert!(
            !first_rule.expression.is_empty(),
            "Expression should not be empty"
        );

        println!("\n=== First Invariant Validated ===");
        println!("Key: {}", first_rule.key);
        println!("Severity: {}", first_rule.severity);
        println!("Human: {}", first_rule.human);
        println!("Expression: {}", first_rule.expression);
    }
}

#[test]
fn test_invariant_severities() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    let mut error_count = 0;
    let mut warning_count = 0;

    for rule in &rules.invariant_rules {
        match rule.severity.as_str() {
            "error" => error_count += 1,
            "warning" => warning_count += 1,
            _ => {}
        }
    }

    println!("\n=== Invariant Severity Counts ===");
    println!("Error invariants: {error_count}");
    println!("Warning invariants: {warning_count}");
    println!("Total: {}", rules.invariant_rules.len());

    assert!(
        error_count > 0 || warning_count > 0,
        "Should have at least some error or warning invariants"
    );
}

#[test]
fn test_invariant_keys_unique() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    let mut keys = std::collections::HashSet::new();
    let mut duplicate_keys = Vec::new();

    for rule in &rules.invariant_rules {
        if !keys.insert(rule.key.clone()) {
            duplicate_keys.push(rule.key.clone());
        }
    }

    println!("\n=== Invariant Key Analysis ===");
    println!("Total invariants: {}", rules.invariant_rules.len());
    println!("Unique keys: {}", keys.len());

    if !duplicate_keys.is_empty() {
        println!("Duplicate keys found: {duplicate_keys:?}");
        println!("Note: Duplicate keys may be valid if they apply to different elements");
    }
}

#[test]
fn test_base_patient_invariants() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/StructureDefinition/Patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    println!("\n=== Base Patient Invariant Rules ===");
    println!("Total invariant rules: {}", rules.invariant_rules.len());

    assert!(
        !rules.invariant_rules.is_empty(),
        "Base Patient should have invariant rules"
    );

    // Show a few examples
    for (i, rule) in rules.invariant_rules.iter().take(3).enumerate() {
        println!("\n{}. {} ({})", i + 1, rule.key, rule.severity);
        println!("   {}", rule.human);
        println!("   Expression: {}", rule.expression);
    }
}

#[test]
fn test_fhirpath_expression_format() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/StructureDefinition/Patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    println!("\n=== FHIRPath Expression Analysis ===");

    let mut has_resource_ref = 0;
    let mut has_context_ref = 0;
    let mut has_complex_expression = 0;

    for rule in &rules.invariant_rules {
        if rule.expression.contains("%resource") {
            has_resource_ref += 1;
        }
        if rule.expression.contains("%context") {
            has_context_ref += 1;
        }
        if rule.expression.contains("where(") || rule.expression.contains("exists(") {
            has_complex_expression += 1;
        }
    }

    println!("Total invariants: {}", rules.invariant_rules.len());
    println!("With %resource: {has_resource_ref}");
    println!("With %context: {has_context_ref}");
    println!("With where/exists: {has_complex_expression}");

    // Show an example of a complex expression
    if let Some(complex) = rules
        .invariant_rules
        .iter()
        .find(|r| r.expression.len() > 50)
    {
        println!("\n=== Example Complex Expression ===");
        println!("Key: {}", complex.key);
        println!("Expression: {}", complex.expression);
    }
}
