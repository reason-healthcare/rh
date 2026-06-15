//! Golden-file tests for codegen output.
//!
//! These tests generate Rust code from representative StructureDefinitions
//! and compare against golden snapshots. Run with `UPDATE_GOLDEN=1` to
//! update the golden files after intentional codegen changes.
//!
//! Tests are serialized because the global TypeRegistry is shared state.

use std::env;
use std::fs;
use std::path::PathBuf;

use rh_codegen::{
    generate_organized_directories_with_traits, CodeGenerator, CodegenConfig, StructureDefinition,
};
use serial_test::serial;
use tempfile::TempDir;

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
}

fn update_mode() -> bool {
    env::var("UPDATE_GOLDEN").map(|v| v == "1").unwrap_or(false)
}

fn make_sd(name: &str, kind: &str, base_type: &str, base_def: &str) -> StructureDefinition {
    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: name.to_string(),
        url: format!("http://hl7.org/fhir/StructureDefinition/{name}"),
        version: Some("4.0.1".to_string()),
        name: name.to_string(),
        title: Some(name.to_string()),
        status: "active".to_string(),
        description: Some(format!("A {kind} for golden testing")),
        purpose: None,
        kind: kind.to_string(),
        is_abstract: false,
        base_type: base_type.to_string(),
        base_definition: Some(base_def.to_string()),
        differential: None,
        snapshot: None,
    }
}

fn generate_and_read(sd: &StructureDefinition, relative_path: &str) -> String {
    let mut generator = CodeGenerator::new(CodegenConfig::default());
    let temp_dir = TempDir::new().unwrap();
    generate_organized_directories_with_traits(&mut generator, sd, temp_dir.path())
        .expect("generation should succeed");
    let file_path = temp_dir.path().join(relative_path);
    fs::read_to_string(&file_path).unwrap_or_else(|e| {
        let src = temp_dir.path().join("src");
        let mut files = vec![];
        if src.exists() {
            if let Ok(entries) = fs::read_dir(&src) {
                for entry in entries.flatten() {
                    files.push(entry.path().display().to_string());
                }
            }
        }
        panic!(
            "Failed to read {}: {}. Files in src: {:?}",
            relative_path, e, files
        )
    })
}

/// Normalize generated code for deterministic comparison.
/// Sorts import lines and normalizes trailing whitespace.
fn normalize(content: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    // Find contiguous import blocks and sort them
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("use ") {
            let start = i;
            while i < lines.len() && lines[i].starts_with("use ") {
                i += 1;
            }
            lines[start..i].sort();
        } else {
            i += 1;
        }
    }

    let result = lines.join("\n");
    // Ensure trailing newline
    if !result.ends_with('\n') {
        result + "\n"
    } else {
        result
    }
}

fn assert_golden(name: &str, content: &str) {
    let golden_path = golden_dir().join(format!("{name}.rs"));
    let normalized = normalize(content);
    if update_mode() {
        fs::create_dir_all(golden_dir()).unwrap();
        fs::write(&golden_path, &normalized).unwrap();
        eprintln!("Updated golden file: {}", golden_path.display());
    } else {
        if !golden_path.exists() {
            panic!(
                "Golden file {} does not exist. Run with UPDATE_GOLDEN=1 to create it.",
                golden_path.display()
            );
        }
        let expected = fs::read_to_string(&golden_path).unwrap_or_else(|e| {
            panic!(
                "Failed to read golden file {}: {}",
                golden_path.display(),
                e
            )
        });
        if expected != normalized {
            let diff_temp = PathBuf::from(
                env::var("CARGO_TARGET_TMPDIR").unwrap_or_else(|_| "/tmp".to_string()),
            );
            let actual_path = diff_temp.join(format!("{name}_actual.rs"));
            fs::write(&actual_path, &normalized).unwrap();
            panic!(
                "Golden mismatch for {}. Actual written to {}\nExpected length: {}, Actual length: {}",
                name,
                actual_path.display(),
                expected.len(),
                normalized.len()
            );
        }
    }
}

#[test]
#[serial]
fn golden_resource_patient() {
    let sd = make_sd(
        "Patient",
        "resource",
        "Patient",
        "http://hl7.org/fhir/StructureDefinition/DomainResource",
    );
    let content = generate_and_read(&sd, "src/resources/patient.rs");
    assert_golden("resource_patient", &content);
}

#[test]
#[serial]
fn golden_datatype_human_name() {
    let sd = make_sd(
        "HumanName",
        "complex-type",
        "HumanName",
        "http://hl7.org/fhir/StructureDefinition/Element",
    );
    let content = generate_and_read(&sd, "src/datatypes/human_name.rs");
    assert_golden("datatype_human_name", &content);
}

#[test]
#[serial]
fn golden_primitive_string() {
    let sd = make_sd(
        "string",
        "primitive-type",
        "string",
        "http://hl7.org/fhir/StructureDefinition/Element",
    );
    let mut generator = CodeGenerator::new(CodegenConfig::default());
    let temp_dir = TempDir::new().unwrap();
    generate_organized_directories_with_traits(&mut generator, &sd, temp_dir.path())
        .expect("generation should succeed");
    let content = fs::read_to_string(temp_dir.path().join("src/primitives/string.rs")).unwrap();
    assert_golden("primitive_string", &content);
}

#[test]
#[serial]
fn golden_profile_vitalsigns() {
    let mut sd = make_sd(
        "VitalSigns",
        "resource",
        "observation-VitalSigns",
        "http://hl7.org/fhir/StructureDefinition/Observation",
    );
    sd.kind = "resource".to_string();
    let content = generate_and_read(&sd, "src/profiles/vital_signs.rs");
    assert_golden("profile_vitalsigns", &content);
}

#[test]
#[serial]
fn golden_trait_patient() {
    let sd = make_sd(
        "Patient",
        "resource",
        "Patient",
        "http://hl7.org/fhir/StructureDefinition/DomainResource",
    );
    let content = generate_and_read(&sd, "src/traits/patient.rs");
    assert_golden("trait_patient", &content);
}
