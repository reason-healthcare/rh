//! Property-based tests that feed StructureDefinitions through codegen
//! and verify the generated Rust code parses successfully with `syn`.

use rh_codegen::{
    generate_organized_directories_with_traits, CodeGenerator, CodegenConfig, StructureDefinition,
};
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

fn make_sd(name: &str, kind: &str, base_type: &str, base_def: &str) -> StructureDefinition {
    StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: name.to_string(),
        url: format!("http://hl7.org/fhir/StructureDefinition/{name}"),
        version: Some("4.0.1".to_string()),
        name: name.to_string(),
        title: Some(name.to_string()),
        status: "active".to_string(),
        description: Some(format!("A {kind} for parse testing")),
        purpose: None,
        kind: kind.to_string(),
        is_abstract: false,
        base_type: base_type.to_string(),
        base_definition: Some(base_def.to_string()),
        differential: None,
        snapshot: None,
    }
}

fn generate_and_parse(sd: &StructureDefinition) -> Vec<String> {
    let mut generator = CodeGenerator::new(CodegenConfig::default());
    let temp_dir = TempDir::new().unwrap();
    generate_organized_directories_with_traits(&mut generator, sd, temp_dir.path())
        .expect("generation should succeed");

    let src_dir = temp_dir.path().join("src");
    let mut parse_errors = vec![];

    if let Ok(entries) = fs::read_dir(&src_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                let content = fs::read_to_string(&path).unwrap();
                if let Err(e) = syn::parse_file(&content) {
                    parse_errors.push(format!("{}: {}", path.display(), e));
                }
            }
        }
    }
    parse_errors
}

#[test]
#[serial]
fn prop_resource_generates_parseable_rust() {
    let sd = make_sd(
        "Observation",
        "resource",
        "Observation",
        "http://hl7.org/fhir/StructureDefinition/DomainResource",
    );
    let errors = generate_and_parse(&sd);
    assert!(errors.is_empty(), "Parse errors: {:?}", errors);
}

#[test]
#[serial]
fn prop_datatype_generates_parseable_rust() {
    let sd = make_sd(
        "Address",
        "complex-type",
        "Address",
        "http://hl7.org/fhir/StructureDefinition/Element",
    );
    let errors = generate_and_parse(&sd);
    assert!(errors.is_empty(), "Parse errors: {:?}", errors);
}

#[test]
#[serial]
fn test_retired_sd_returns_error() {
    let mut sd = make_sd(
        "RetiredThing",
        "resource",
        "RetiredThing",
        "http://hl7.org/fhir/StructureDefinition/DomainResource",
    );
    sd.status = "retired".to_string();
    let mut generator = CodeGenerator::new(CodegenConfig::default());
    let temp_dir = TempDir::new().unwrap();
    let result = generate_organized_directories_with_traits(&mut generator, &sd, temp_dir.path());
    assert!(result.is_err(), "Retired SDs should return an error");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("retired"),
        "Error should mention retired: {}",
        err_msg
    );
}

#[test]
#[serial]
fn prop_logical_model_returns_error() {
    let sd = make_sd(
        "SomeLogicalModel",
        "logical",
        "SomeLogicalModel",
        "http://hl7.org/fhir/StructureDefinition/Base",
    );
    let mut generator = CodeGenerator::new(CodegenConfig::default());
    let result = generator.generate_struct(&sd);
    assert!(result.is_err(), "LogicalModel SDs should return an error");
}
