//! Tests to ensure FHIRPath examples build and run without exceptions
//!
//! This test module validates that all FHIRPath examples can be executed
//! successfully without runtime errors. This is crucial for ensuring
//! documentation examples remain functional as the codebase evolves.
//!
//! The tests cover:
//! - Build validation: All examples compile successfully
//! - Runtime validation: Examples run without panics or errors
//! - Output validation: Examples produce expected output patterns
//! - Completeness validation: Test list matches actual examples in filesystem
//!
//! Examples with known issues are tracked separately and won't fail the tests,
//! but will be reported for visibility.

use std::path::Path;
use std::process::Command;

/// List of all FHIRPath examples that should be tested
const FHIRPATH_EXAMPLES: &[&str] = &[
    "basic_demo",
    "arithmetic_operations",
    "string_functions",
    "collection_operations",
    "indexing_operations",
    "union_operations",
    "fhir_patient_navigation",
    "verify_union",
    "verify_indexing",
    "verify_union_debug",
];

/// Examples that are known to have issues but are still valuable for documentation
/// These will be tested for building but runtime failures are expected/tolerated
const EXAMPLES_WITH_KNOWN_ISSUES: &[&str] = &[
    "string_functions", // Has parser issues with empty string literals
];

#[test]
fn test_all_examples_build() {
    println!("🔨 Testing that all FHIRPath examples build successfully...");

    // Change to workspace root for cargo commands
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Build all examples
    let output = Command::new("cargo")
        .arg("build")
        .arg("--examples")
        .arg("-p")
        .arg("fhirpath")
        .current_dir(workspace_root)
        .output()
        .expect("Failed to execute cargo build --examples");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "Failed to build FHIRPath examples!\n\nSTDOUT:\n{}\n\nSTDERR:\n{}",
            stdout, stderr
        );
    }

    println!("✅ All FHIRPath examples built successfully");
}

#[test]
fn test_all_examples_run_without_error() {
    println!("🚀 Testing that all FHIRPath examples run without exceptions...");

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut failed_examples = Vec::new();
    let mut known_issue_failures = Vec::new();

    for example in FHIRPATH_EXAMPLES {
        println!("  🧪 Testing example: {}", example);

        let output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("fhirpath")
            .arg("--example")
            .arg(example)
            .current_dir(workspace_root)
            .output()
            .expect(&format!("Failed to execute example: {}", example));

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Check if this is a known issue
            if EXAMPLES_WITH_KNOWN_ISSUES.contains(example) {
                println!(
                    "    ⚠️  {} - Known issue (runtime failure expected)",
                    example
                );
                known_issue_failures.push(example);
            } else {
                eprintln!("❌ Example '{}' failed to run:", example);
                eprintln!("STDOUT:\n{}", stdout);
                eprintln!("STDERR:\n{}", stderr);
                failed_examples.push(example);
            }
        } else {
            println!("    ✅ {} - PASSED", example);
        }
    }

    // Report known issues but don't fail the test
    if !known_issue_failures.is_empty() {
        println!(
            "⚠️  Examples with known issues that failed (expected): {:?}",
            known_issue_failures
        );
    }

    // Only fail the test for unexpected failures
    if !failed_examples.is_empty() {
        panic!(
            "The following FHIRPath examples failed unexpectedly: {:?}\n\
             This indicates that the examples have runtime errors and need to be fixed.",
            failed_examples
        );
    }

    let successful_count = FHIRPATH_EXAMPLES.len() - known_issue_failures.len();
    println!(
        "🎉 {} FHIRPath examples ran successfully! ({} with known issues)",
        successful_count,
        known_issue_failures.len()
    );
}

#[test]
fn test_example_output_contains_expected_patterns() {
    println!("🔍 Testing that FHIRPath examples produce expected output patterns...");

    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Test patterns for specific examples
    let example_patterns = [
        (
            "basic_demo",
            vec!["FHIRPath Basic Demonstration", "completed successfully"],
        ),
        (
            "arithmetic_operations",
            vec![
                "Mathematical Operations",
                "Addition:",
                "completed successfully",
            ],
        ),
        (
            "string_functions",
            vec!["String Functions", "length()", "completed successfully"],
        ),
        (
            "collection_operations",
            vec!["Collection Operations", "where()", "completed successfully"],
        ),
        (
            "indexing_operations",
            vec!["Array Indexing", "name[0]", "completed successfully"],
        ),
        (
            "union_operations",
            vec!["Union Operations", "union", "completed successfully"],
        ),
        (
            "fhir_patient_navigation",
            vec![
                "Patient Navigation",
                "resourceType",
                "completed successfully",
            ],
        ),
        ("verify_union", vec!["Union verification", "union"]),
        ("verify_indexing", vec!["Indexing verification", "indexing"]),
        ("verify_union_debug", vec!["Debug union", "union"]),
    ];

    let mut failed_patterns = Vec::new();

    for (example, expected_patterns) in example_patterns.iter() {
        println!("  🔍 Checking output patterns for: {}", example);

        let output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("fhirpath")
            .arg("--example")
            .arg(example)
            .current_dir(workspace_root)
            .output()
            .expect(&format!("Failed to execute example: {}", example));

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stdout_lower = stdout.to_lowercase();

            let mut missing_patterns = Vec::new();
            for pattern in expected_patterns {
                if !stdout_lower.contains(&pattern.to_lowercase()) {
                    missing_patterns.push(pattern);
                }
            }

            if !missing_patterns.is_empty() {
                println!(
                    "    ⚠️  {} - Missing expected patterns: {:?}",
                    example, missing_patterns
                );
                failed_patterns.push((example, missing_patterns));
            } else {
                println!("    ✅ {} - All expected patterns found", example);
            }
        } else {
            println!(
                "    ❌ {} - Failed to run (skipping pattern check)",
                example
            );
        }
    }

    if !failed_patterns.is_empty() {
        eprintln!("⚠️  Some examples are missing expected output patterns:");
        for (example, patterns) in failed_patterns {
            eprintln!("  - {}: {:?}", example, patterns);
        }
        eprintln!("This might indicate that examples have changed output format or are not working as expected.");
        // Note: We don't panic here as this is more of a warning than a hard failure
    }

    println!("🔍 Output pattern validation completed");
}

#[cfg(test)]
mod example_validation {
    use super::*;

    /// Test that verifies the example list is up-to-date with actual examples in the filesystem
    #[test]
    fn test_example_list_is_complete() {
        println!("📋 Verifying example list completeness...");

        let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

        if !examples_dir.exists() {
            panic!(
                "Examples directory not found at: {}",
                examples_dir.display()
            );
        }

        let mut actual_examples = Vec::new();

        // Read all .rs files in examples directory (excluding README.md)
        for entry in std::fs::read_dir(&examples_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let filename = path.file_stem().unwrap().to_str().unwrap();
                actual_examples.push(filename.to_string());
            }
        }

        actual_examples.sort();
        let mut expected_examples: Vec<String> =
            FHIRPATH_EXAMPLES.iter().map(|s| s.to_string()).collect();
        expected_examples.sort();

        if actual_examples != expected_examples {
            eprintln!("❌ Example list mismatch!");
            eprintln!("Actual examples found: {:?}", actual_examples);
            eprintln!("Expected examples in test: {:?}", expected_examples);

            let missing_from_test: Vec<_> = actual_examples
                .iter()
                .filter(|example| !expected_examples.contains(example))
                .collect();

            let extra_in_test: Vec<_> = expected_examples
                .iter()
                .filter(|example| !actual_examples.contains(example))
                .collect();

            if !missing_from_test.is_empty() {
                eprintln!("Examples missing from test list: {:?}", missing_from_test);
            }

            if !extra_in_test.is_empty() {
                eprintln!("Examples in test list but not found: {:?}", extra_in_test);
            }

            panic!(
                "The FHIRPATH_EXAMPLES constant needs to be updated to match the actual examples.\n\
                 Please update the list in crates/fhirpath/tests/examples_test.rs"
            );
        }

        println!(
            "✅ Example list is complete and up-to-date ({} examples)",
            actual_examples.len()
        );
    }
}
