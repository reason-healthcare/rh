use rh_validator::{ProfileRegistry, RuleCompiler};

#[test]
fn test_load_us_core_patient_profile() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !packages_dir.exists() {
        println!("Skipping test: US Core package not found at {packages_dir:?}");
        return;
    }

    let registry = ProfileRegistry::new(
        rh_validator::FhirVersion::R4,
        Some(packages_dir.to_str().unwrap()),
    )
    .unwrap();
    let profiles = registry.list_profiles();

    assert!(!profiles.is_empty(), "Should load profiles from US Core");

    let us_core_patient = profiles
        .iter()
        .find(|p| p.contains("us-core-patient"))
        .expect("Should find US Core Patient profile");

    println!("Found profile: {us_core_patient}");
}

#[test]
fn test_compile_us_core_patient_rules() {
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
    .unwrap();

    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";
    let snapshot = registry
        .get_snapshot(profile_url)
        .expect("Should get snapshot")
        .expect("Profile should exist");

    let compiler = RuleCompiler::new(100);
    let rules = compiler.compile(&snapshot).expect("Should compile rules");

    assert_eq!(rules.profile_url, profile_url);
    assert!(
        !rules.cardinality_rules.is_empty(),
        "Should have cardinality rules"
    );
    assert!(!rules.type_rules.is_empty(), "Should have type rules");

    println!("Compiled rules:");
    println!("  Cardinality rules: {}", rules.cardinality_rules.len());
    println!("  Type rules: {}", rules.type_rules.len());
    println!("  Binding rules: {}", rules.binding_rules.len());
    println!("  Invariant rules: {}", rules.invariant_rules.len());

    let name_rule = rules
        .cardinality_rules
        .iter()
        .find(|r| r.path == "Patient.name");
    if let Some(rule) = name_rule {
        assert!(
            rule.min.is_some(),
            "Patient.name should have min cardinality"
        );
        println!("Patient.name cardinality: {:?}..{:?}", rule.min, rule.max);
    }
}

#[test]
fn test_rule_compilation_caching() {
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
    .unwrap();
    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    let compiler = RuleCompiler::new(100);
    let snapshot = registry.get_snapshot(profile_url).unwrap().unwrap();

    let (before_len, capacity) = compiler.cache_stats();
    assert_eq!(before_len, 0, "Cache should be empty initially");

    let _rules1 = compiler.compile(&snapshot).unwrap();
    let (after_first, _) = compiler.cache_stats();
    assert_eq!(
        after_first, 1,
        "Cache should have 1 entry after first compile"
    );

    let _rules2 = compiler.compile(&snapshot).unwrap();
    let (after_second, _) = compiler.cache_stats();
    assert_eq!(
        after_second, 1,
        "Cache should still have 1 entry (cache hit)"
    );

    println!("Cache stats: {after_second} entries / {capacity} capacity");
}

#[test]
fn test_snapshot_caching() {
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
    .unwrap();
    let profile_url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient";

    let (before_len, capacity) = registry.cache_stats();
    assert_eq!(before_len, 0, "Snapshot cache should be empty initially");

    let _snapshot1 = registry.get_snapshot(profile_url).unwrap().unwrap();
    let (after_first, _) = registry.cache_stats();
    assert_eq!(
        after_first, 1,
        "Cache should have 1 snapshot after first get"
    );

    let _snapshot2 = registry.get_snapshot(profile_url).unwrap().unwrap();
    let (after_second, _) = registry.cache_stats();
    assert_eq!(
        after_second, 1,
        "Cache should still have 1 snapshot (cache hit)"
    );

    println!("Snapshot cache stats: {after_second} entries / {capacity} capacity");
}

#[test]
fn test_profile_search() {
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
    .unwrap();

    let patient_profiles = registry.search_profiles("patient");
    assert!(
        !patient_profiles.is_empty(),
        "Should find patient-related profiles"
    );
    println!("Found {} patient profiles", patient_profiles.len());

    let observation_profiles = registry.search_profiles("observation");
    println!("Found {} observation profiles", observation_profiles.len());

    let all_profiles = registry.list_profiles();
    println!("Total profiles: {}", all_profiles.len());
}

#[test]
fn test_load_profile_from_file() {
    let patient_file = dirs::home_dir().unwrap().join(
        ".fhir/packages/hl7.fhir.us.core#6.1.0/package/StructureDefinition-us-core-patient.json",
    );

    if !patient_file.exists() {
        println!("Skipping test: US Core Patient file not found");
        return;
    }

    let mut registry = ProfileRegistry::new(rh_validator::FhirVersion::R4, None).unwrap();
    let profiles_before = registry.list_profiles().len();

    let result = registry.load_from_file(patient_file.to_str().unwrap());
    assert!(result.is_ok(), "Should load profile from file");

    let profiles = registry.list_profiles();
    assert_eq!(
        profiles.len(),
        profiles_before + 1,
        "Should have exactly 1 additional profile loaded (got {} total, expected {})",
        profiles.len(),
        profiles_before + 1
    );

    let us_core_patient_found = profiles.iter().any(|p| p.contains("us-core-patient"));
    assert!(
        us_core_patient_found,
        "Should contain the US Core Patient profile"
    );
}
