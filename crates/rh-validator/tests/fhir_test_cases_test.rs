#[cfg(feature = "fhir-test-cases")]
mod fhir_test_cases;

#[cfg(feature = "fhir-test-cases")]
#[test]
fn test_fhir_test_cases_setup() {
    use fhir_test_cases::{ensure_test_cases, find_r4_test_cases};

    println!("\n=== FHIR Test Cases Setup ===\n");

    let cache_dir = ensure_test_cases().expect("Failed to ensure test cases are available");

    println!("✓ Test cases cached at: {}", cache_dir.display());

    let r4_dir = find_r4_test_cases(&cache_dir).expect("Failed to find R4 test cases");

    println!("✓ R4 test cases at: {}", r4_dir.display());

    assert!(r4_dir.exists(), "R4 directory should exist");
    assert!(r4_dir.is_dir(), "R4 path should be a directory");

    let entries: Vec<_> = std::fs::read_dir(&r4_dir)
        .expect("Failed to read R4 directory")
        .collect();

    println!("✓ Found {} entries in R4 directory", entries.len());
    assert!(!entries.is_empty(), "R4 directory should not be empty");

    println!("\n=== Setup Complete ===\n");
}

#[cfg(not(feature = "fhir-test-cases"))]
#[test]
fn test_fhir_test_cases_disabled() {
    println!("\nFHIR test cases feature is not enabled.");
    println!("Run with: cargo test --features fhir-test-cases\n");
}
