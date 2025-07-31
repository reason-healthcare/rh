//! Tests to ensure FHIRPath examples build and run without exceptions
//!
//! This test module validates that all FHIRPath examples can be executed
//! successfully without runtime errors. This is crucial for ensuring
//! documentation examples remain functional as the codebase evolves.
//!
//! The tests automatically discover all .rs files in the examples directory
//! and test them for compilation and execution.

use std::path::Path;
use std::process::Command;

/// Get all example names by scanning the examples directory
fn get_all_examples() -> Vec<String> {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    if !examples_dir.exists() {
        panic!(
            "Examples directory not found at: {}",
            examples_dir.display()
        );
    }

    let mut examples = Vec::new();

    for entry in std::fs::read_dir(&examples_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            examples.push(filename.to_string());
        }
    }

    examples.sort();
    examples
}

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
        .arg("rh-fhirpath")
        .current_dir(workspace_root)
        .output()
        .expect("Failed to execute cargo build --examples");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Failed to build FHIRPath examples!\n\nSTDOUT:\n{stdout}\n\nSTDERR:\n{stderr}");
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

    let examples = get_all_examples();
    let mut failed_examples = Vec::new();

    for example in &examples {
        println!("  🧪 Testing example: {example}");

        let output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("rh-fhirpath")
            .arg("--example")
            .arg(example)
            .current_dir(workspace_root)
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute example: {example}"));

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            eprintln!("❌ Example '{example}' failed to run:");
            eprintln!("STDOUT:\n{stdout}");
            eprintln!("STDERR:\n{stderr}");
            failed_examples.push(example);
        } else {
            println!("    ✅ {example} - PASSED");
        }
    }

    // Fail the test if any examples failed
    if !failed_examples.is_empty() {
        panic!(
            "The following FHIRPath examples failed: {failed_examples:?}\n\
             This indicates that the examples have runtime errors and need to be fixed."
        );
    }

    println!(
        "🎉 All {} FHIRPath examples ran successfully!",
        examples.len()
    );
}

#[test]
fn test_example_discovery() {
    println!("� Testing example discovery...");

    let examples = get_all_examples();

    if examples.is_empty() {
        panic!("No examples found in the examples directory!");
    }

    println!("✅ Found {} examples: {examples:?}", examples.len());
}
