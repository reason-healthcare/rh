//! Integration tests for the `rh package link` executable bundle pipeline.

use rh_packager::{link_package, link_with_options, LinkOptions};
use serde_json::Value;
use std::path::PathBuf;
use tempfile::TempDir;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn executable_fixture() -> PathBuf {
    fixtures_dir().join("executable-package")
}

/// Read and parse a JSON file.
fn read_json(path: &std::path::Path) -> Value {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("Failed to parse JSON from {}: {e}", path.display()))
}

/// Test 1: link() produces a bundle with all source resources.
#[test]
fn link_produces_bundle_with_source_resources() {
    let output_dir = TempDir::new().unwrap();
    let output = link_package(&executable_fixture(), output_dir.path()).unwrap();

    // Output should be a JSON file.
    assert!(output.to_string_lossy().ends_with("executable-bundle.json"));
    assert!(output.exists());

    let bundle = read_json(&output);
    assert_eq!(bundle["resourceType"], "Bundle");
    assert_eq!(bundle["type"], "collection");

    // Should contain entries for PlanDefinition, Library, ValueSet, IG.
    let entries = bundle["entry"].as_array().unwrap();
    let resource_types: Vec<&str> = entries
        .iter()
        .filter_map(|e| e["resource"]["resourceType"].as_str())
        .collect();
    assert!(resource_types.contains(&"PlanDefinition"));
    assert!(resource_types.contains(&"Library"));
    assert!(resource_types.contains(&"ValueSet"));
}

/// Test 2: link() compiles CQL to ELM (existing cql processor).
#[test]
fn link_compiles_cql_to_elm() {
    let output_dir = TempDir::new().unwrap();
    let output = link_package(&executable_fixture(), output_dir.path()).unwrap();

    let bundle = read_json(&output);
    let entries = bundle["entry"].as_array().unwrap();

    let library = entries
        .iter()
        .find(|e| e["resource"]["resourceType"] == "Library")
        .map(|e| &e["resource"])
        .unwrap();

    let content = library["content"].as_array().unwrap();
    let has_elm = content
        .iter()
        .any(|c| c["contentType"].as_str() == Some("application/elm+json"));
    assert!(has_elm, "Library should have ELM attachment after link");
}

/// Test 3: link() expands ValueSets from compose with explicit concepts.
#[test]
fn link_expands_valuesets_from_compose() {
    let output_dir = TempDir::new().unwrap();
    let output = link_package(&executable_fixture(), output_dir.path()).unwrap();

    let bundle = read_json(&output);
    let entries = bundle["entry"].as_array().unwrap();

    let vs = entries
        .iter()
        .find(|e| e["resource"]["resourceType"] == "ValueSet")
        .map(|e| &e["resource"])
        .unwrap();

    let contains = vs["expansion"]["contains"].as_array().unwrap();
    assert_eq!(contains.len(), 2, "ValueSet should have 2 expanded codes");
    assert_eq!(contains[0]["code"], "A");
    assert_eq!(contains[1]["code"], "B");
}

/// Test 4: link() produces bundle metadata (tag and extension).
#[test]
fn link_produces_bundle_metadata() {
    let output_dir = TempDir::new().unwrap();
    let output = link_package(&executable_fixture(), output_dir.path()).unwrap();

    let bundle = read_json(&output);

    // Check meta.tag
    let tag = &bundle["meta"]["tag"][0];
    assert_eq!(tag["code"], "executable");
    assert_eq!(tag["display"], "Executable Bundle");

    // Check extension metadata
    let ext = &bundle["extension"][0]["extension"];
    let has_resource_count = ext
        .as_array()
        .unwrap()
        .iter()
        .any(|e| e["url"] == "resourceCount");
    assert!(has_resource_count);
}

/// Test 5: link() with --format directory produces separate files.
#[test]
fn link_directory_format_produces_separate_files() {
    let output_dir = TempDir::new().unwrap();
    let output = link_with_options(
        &executable_fixture(),
        output_dir.path(),
        LinkOptions {
            format: Some("directory".to_string()),
            ..Default::default()
        },
    )
    .unwrap();

    // Output should be the directory itself.
    assert!(output.is_dir());

    // Should have a _manifest.json.
    assert!(output.join("_manifest.json").exists());

    // Should have individual resource files.
    assert!(output.join("PlanDefinition-test.json").exists());
    assert!(output.join("Library-TestLogic.json").exists());
    assert!(output.join("ValueSet-test-codes.json").exists());
}

/// Test 6: link-validate passes when all deps are resolved.
#[test]
fn link_validate_passes_for_complete_fixture() {
    let output_dir = TempDir::new().unwrap();
    // This should succeed because the fixture is self-contained.
    let result = link_package(&executable_fixture(), output_dir.path());
    assert!(result.is_ok(), "link should succeed for complete fixture");
}

/// Test 7: link-validate fails when a dependency is missing.
#[test]
fn link_validate_fails_for_missing_dependency() {
    let tmp = TempDir::new().unwrap();
    let fixture = tmp.path().join("incomplete");

    // Create a minimal fixture with a dangling reference.
    std::fs::create_dir_all(fixture.join("input")).unwrap();
    std::fs::write(
        fixture.join("packager.toml"),
        r#"id = "test.incomplete"
version = "1.0.0"
fhir_version = "4.0.1"
canonical = "http://test.org/fhir"

[hooks]
before_build = []

[link]
format = "bundle"
"#,
    )
    .unwrap();
    std::fs::write(
        fixture.join("input/ImplementationGuide.json"),
        r#"{"resourceType":"ImplementationGuide","id":"ig","packageId":"test.incomplete","version":"1.0.0","url":"http://test.org/fhir","fhirVersion":["4.0.1"],"status":"draft"}"#,
    )
    .unwrap();
    // PlanDefinition references a Library that doesn't exist.
    std::fs::write(
        fixture.join("input/PlanDefinition-test.json"),
        r#"{"resourceType":"PlanDefinition","id":"test","url":"http://test.org/fhir/PlanDefinition/test","name":"Test","status":"active","library":["http://missing.org/Library/DoesNotExist"]}"#,
    )
    .unwrap();

    let output_dir = TempDir::new().unwrap();
    let result = link_package(&fixture, output_dir.path());

    // link should fail because link-validate catches the unresolved reference.
    assert!(
        result.is_err(),
        "link should fail when a dependency is missing"
    );
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("Unresolved canonical reference") || msg.contains("Link validation failed"),
        "Error should mention unresolved reference, got: {msg}"
    );

    let output_dir = TempDir::new().unwrap();
    let output = link_with_options(
        &fixture,
        output_dir.path(),
        LinkOptions {
            no_validate: true,
            ..Default::default()
        },
    )
    .expect("--no-validate should skip the completeness gate");
    assert!(output.exists());
}

#[test]
fn terminology_directory_override_is_applied() {
    let tmp = TempDir::new().unwrap();
    let fixture_copy = tmp.path().join("fixture");
    copy_dir_recursive(&executable_fixture(), &fixture_copy);
    std::fs::write(
        fixture_copy.join("input/ValueSet-test-codes.json"),
        r#"{"resourceType":"ValueSet","id":"expanded-from-dir","url":"http://example.org/fhir/ValueSet/expanded-from-dir","version":"1.0.0","status":"active","compose":{"include":[{"system":"http://example.org/codes"}]}}"#,
    )
    .unwrap();
    let terminology_dir = fixtures_dir().join("terminology-dir");
    let output_dir = TempDir::new().unwrap();

    let output = link_with_options(
        &fixture_copy,
        output_dir.path(),
        LinkOptions {
            terminology_dir: Some(terminology_dir),
            ..Default::default()
        },
    )
    .unwrap();
    let bundle = read_json(&output);
    let value_set = bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|entry| {
            (entry["resource"]["id"] == "expanded-from-dir").then_some(&entry["resource"])
        })
        .unwrap();
    assert_eq!(
        value_set["expansion"]["contains"].as_array().unwrap().len(),
        2
    );
}

/// Helper: recursively copy a directory.
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) {
    std::fs::create_dir_all(dst).unwrap();
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap();
            copy_dir_recursive(&path, &dst.join(name));
        } else {
            let name = path.file_name().unwrap();
            std::fs::copy(&path, dst.join(name)).unwrap();
        }
    }
}
