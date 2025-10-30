use rh_validator::ValueSetLoader;

#[test]
fn test_load_extensional_valueset() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let valueset_url = "http://hl7.org/fhir/ValueSet/yesnodontknow";
    let result = loader
        .load_valueset(valueset_url)
        .expect("Should load ValueSet");

    assert!(result.is_some(), "Should find yesnodontknow ValueSet");

    let valueset = result.unwrap();
    assert_eq!(valueset.url, valueset_url);
    assert!(valueset.expansion.is_some(), "Should have expansion");

    let expansion = valueset.expansion.unwrap();
    assert!(expansion.contains.is_some(), "Should have contains");

    let contains = expansion.contains.unwrap();
    assert!(contains.len() >= 3, "Should have at least 3 codes");

    println!("\n=== yesnodontknow ValueSet ===");
    println!("URL: {}", valueset.url);
    println!("Total codes: {}", contains.len());
    for concept in &contains {
        println!(
            "  {} | {} - {}",
            concept.system.as_ref().unwrap_or(&"".to_string()),
            concept.code,
            concept.display.as_ref().unwrap_or(&"".to_string())
        );
    }
}

#[test]
fn test_contains_code_valid() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let valueset_url = "http://hl7.org/fhir/ValueSet/yesnodontknow";

    let result = loader
        .contains_code(
            valueset_url,
            "http://terminology.hl7.org/CodeSystem/v2-0136",
            "Y",
        )
        .expect("Should check code");

    assert!(result, "Code 'Y' should be in yesnodontknow ValueSet");
}

#[test]
fn test_contains_code_invalid() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let valueset_url = "http://hl7.org/fhir/ValueSet/yesnodontknow";

    let result = loader
        .contains_code(
            valueset_url,
            "http://terminology.hl7.org/CodeSystem/v2-0136",
            "INVALID",
        )
        .expect("Should check code");

    assert!(
        !result,
        "Code 'INVALID' should not be in yesnodontknow ValueSet"
    );
}

#[test]
fn test_valueset_with_version_suffix() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let valueset_url_with_version = "http://hl7.org/fhir/ValueSet/yesnodontknow|4.0.1";

    let result = loader
        .load_valueset(valueset_url_with_version)
        .expect("Should load ValueSet");

    assert!(
        result.is_some(),
        "Should find yesnodontknow ValueSet even with version suffix"
    );
}

#[test]
fn test_caching() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let (len_before, _) = loader.cache_stats();
    assert_eq!(len_before, 0, "Cache should be empty initially");

    let valueset_url = "http://hl7.org/fhir/ValueSet/yesnodontknow";
    loader
        .load_valueset(valueset_url)
        .expect("Should load ValueSet");

    let (len_after, capacity) = loader.cache_stats();
    assert_eq!(len_after, 1, "Cache should have 1 entry");
    assert_eq!(capacity, 100, "Cache capacity should be 100");

    loader
        .load_valueset(valueset_url)
        .expect("Should load from cache");

    let (len_after2, _) = loader.cache_stats();
    assert_eq!(len_after2, 1, "Cache should still have 1 entry");
}

#[test]
fn test_is_extensional() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let extensional_url = "http://hl7.org/fhir/ValueSet/yesnodontknow";
    assert!(
        loader
            .is_extensional(extensional_url)
            .expect("Should check extensional"),
        "yesnodontknow should be extensional"
    );

    let intensional_url = "http://hl7.org/fhir/ValueSet/administrative-gender";
    let result = loader
        .is_extensional(intensional_url)
        .expect("Should check extensional");
    println!(
        "administrative-gender is extensional: {}",
        if result { "yes" } else { "no" }
    );
}

#[test]
fn test_multiple_package_directories() {
    let r4_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");
    let us_core_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.us.core#6.1.0/package");

    if !r4_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let mut dirs = vec![r4_dir];
    let has_us_core = us_core_dir.exists();
    if has_us_core {
        dirs.push(us_core_dir.clone());
    }

    let loader = ValueSetLoader::new(dirs, 100);

    let r4_valueset = "http://hl7.org/fhir/ValueSet/yesnodontknow";
    let result = loader
        .load_valueset(r4_valueset)
        .expect("Should load from R4");
    assert!(result.is_some(), "Should find R4 ValueSet");

    if has_us_core {
        let us_core_valueset = "http://hl7.org/fhir/us/core/ValueSet/us-core-usps-state";
        let result_us = loader
            .load_valueset(us_core_valueset)
            .expect("Should load from US Core");
        if result_us.is_some() {
            println!("Found US Core usps-state ValueSet");
            if let Some(vs) = result_us {
                if vs.expansion.is_some() {
                    println!("  Has expansion");
                } else {
                    println!("  No expansion (intensional)");
                }
            }
        }
    }
}

#[test]
fn test_missing_valueset() {
    let packages_dir = dirs::home_dir()
        .unwrap()
        .join(".fhir/packages/hl7.fhir.r4.core#4.0.1/package");

    if !packages_dir.exists() {
        println!("Skipping test: Base R4 package not found");
        return;
    }

    let loader = ValueSetLoader::new(vec![packages_dir], 100);

    let missing_url = "http://example.com/ValueSet/does-not-exist";
    let result = loader
        .load_valueset(missing_url)
        .expect("Should handle missing ValueSet");

    assert!(result.is_none(), "Should return None for missing ValueSet");
}
