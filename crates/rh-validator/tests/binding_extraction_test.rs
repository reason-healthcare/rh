use rh_validator::{ProfileRegistry, RuleCompiler};

#[test]
fn test_binding_rule_extraction_us_core_patient() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found");
        return;
    }

    let registry =
        ProfileRegistry::new(Some(packages_dir.to_str().unwrap())).expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    println!("\n=== Binding Rules Extracted ===");
    println!("Total binding rules: {}", rules.binding_rules.len());

    for binding in &rules.binding_rules {
        println!("\nPath: {}", binding.path);
        println!("  Strength: {}", binding.strength);
        println!("  ValueSet: {}", binding.value_set_url);
    }

    println!("\n=== Checking snapshot elements directly ===");
    if let Some(snap) = &snapshot.snapshot {
        let mut binding_count = 0;
        let mut binding_with_vs = 0;
        for element in &snap.element {
            if let Some(binding) = &element.binding {
                binding_count += 1;
                println!("\nElement path: {}", element.path);
                println!("  Binding strength: {}", binding.strength);
                if let Some(vs) = &binding.value_set {
                    binding_with_vs += 1;
                    println!("  ValueSet URL: {vs}");
                } else {
                    println!("  ValueSet URL: None (missing!)");
                }
            }
        }
        println!("\nTotal elements with bindings in snapshot: {binding_count}");
        println!("Total bindings with ValueSet URL: {binding_with_vs}");
    }
}

#[test]
fn test_binding_rule_extraction_from_base_patient() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let registry =
        ProfileRegistry::new(Some(packages_dir.to_str().unwrap())).expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/StructureDefinition/Patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    println!("\n=== Base Patient Binding Rules ===");
    println!("Total binding rules: {}", rules.binding_rules.len());

    for binding in &rules.binding_rules {
        println!("\nPath: {}", binding.path);
        println!("  Strength: {}", binding.strength);
        println!("  ValueSet: {}", binding.value_set_url);
    }

    assert!(
        !rules.binding_rules.is_empty(),
        "Base Patient should have binding rules"
    );
}

#[test]
fn test_binding_strengths() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let registry =
        ProfileRegistry::new(Some(packages_dir.to_str().unwrap())).expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/StructureDefinition/Patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    let strengths: Vec<String> = rules
        .binding_rules
        .iter()
        .map(|b| b.strength.clone())
        .collect();

    println!("\n=== Binding Strengths Found ===");
    let mut unique_strengths: Vec<_> = strengths.clone();
    unique_strengths.sort();
    unique_strengths.dedup();

    for strength in &unique_strengths {
        let count = strengths.iter().filter(|s| *s == strength).count();
        println!("{strength}: {count} bindings");
    }

    assert!(
        unique_strengths.contains(&"required".to_string())
            || unique_strengths.contains(&"extensible".to_string())
            || unique_strengths.contains(&"preferred".to_string())
            || unique_strengths.contains(&"example".to_string()),
        "Should find at least one standard binding strength"
    );
}

#[test]
fn test_binding_rule_structure() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let registry =
        ProfileRegistry::new(Some(packages_dir.to_str().unwrap())).expect("Should create registry");
    let compiler = RuleCompiler::new(100);

    let profile_url = "http://hl7.org/fhir/StructureDefinition/Patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    if let Some(binding) = rules.binding_rules.first() {
        assert!(!binding.path.is_empty(), "Path should not be empty");
        assert!(
            !binding.value_set_url.is_empty(),
            "ValueSet URL should not be empty"
        );
        assert!(
            !binding.strength.is_empty(),
            "Binding strength should not be empty"
        );

        println!("\nExample binding rule:");
        println!("  Path: {}", binding.path);
        println!("  Strength: {}", binding.strength);
        println!("  ValueSet: {}", binding.value_set_url);
    }
}
