//! End-to-end pipeline test exercising all four built-in processors
//! (`fsh`, `cql`, `snapshot`, `validate`) in a single `build`.
//!
//! The fixture package is generated on the fly in a temp dir so the test is
//! fully hermetic: no dependency packages, no network, no `~/.fhir`.

use std::fs;
use std::path::{Path, PathBuf};

use rh_packager::pipeline::build;
use tempfile::TempDir;

const BASE_SD: &str = r#"{
  "resourceType": "StructureDefinition",
  "id": "BasePatient",
  "url": "http://example.org/fhir/StructureDefinition/BasePatient",
  "name": "BasePatient",
  "status": "draft",
  "kind": "resource",
  "abstract": false,
  "type": "Patient",
  "snapshot": {
    "element": [
      { "path": "Patient", "min": 0, "max": "*" },
      { "path": "Patient.identifier", "min": 0, "max": "*" },
      { "path": "Patient.name", "min": 0, "max": "*" }
    ]
  }
}"#;

const PROFILE_SD: &str = r#"{
  "resourceType": "StructureDefinition",
  "id": "MyPatient",
  "url": "http://example.org/fhir/StructureDefinition/MyPatient",
  "name": "MyPatient",
  "status": "draft",
  "kind": "resource",
  "abstract": false,
  "type": "Patient",
  "baseDefinition": "http://example.org/fhir/StructureDefinition/BasePatient",
  "differential": {
    "element": [
      { "path": "Patient.identifier", "min": 1, "max": "1" }
    ]
  }
}"#;

const IG: &str = r#"{
  "resourceType": "ImplementationGuide",
  "id": "example.fhir.full",
  "url": "http://example.org/fhir/ImplementationGuide/example.fhir.full",
  "version": "1.0.0",
  "name": "ExampleFhirFull",
  "status": "draft",
  "packageId": "example.fhir.full",
  "fhirVersion": ["4.0.1"]
}"#;

const CQL: &str = "library SimpleLib version '1.0'\n\ndefine \"Always True\": true\n";

const FSH: &str = r#"ValueSet: SimpleColors
Id: simple-colors
Title: "Simple Colors"
Description: "A simple value set."
* http://example.org/CodeSystem/colors#red "Red"
* http://example.org/CodeSystem/colors#blue "Blue"
"#;

fn write_fixture(root: &Path) -> PathBuf {
    let input = root.join("input");
    fs::create_dir_all(input.join("cql")).unwrap();
    fs::create_dir_all(input.join("fsh")).unwrap();

    // Empty packages dir keeps snapshot/validate hermetic (no declared deps).
    let packages_dir = root.join("packages");
    fs::create_dir_all(&packages_dir).unwrap();

    fs::write(
        root.join("packager.toml"),
        format!(
            r#"id           = "example.fhir.full"
version      = "1.0.0"
canonical    = "http://example.org/fhir"
fhir_version = "4.0.1"
description  = "Full pipeline test package"
packages_dir = "{}"

[hooks]
before_build = ["fsh", "cql", "snapshot", "validate"]
"#,
            packages_dir.display()
        ),
    )
    .unwrap();

    fs::write(input.join("ImplementationGuide.json"), IG).unwrap();
    fs::write(input.join("StructureDefinition-BasePatient.json"), BASE_SD).unwrap();
    fs::write(input.join("StructureDefinition-MyPatient.json"), PROFILE_SD).unwrap();
    fs::write(input.join("cql").join("SimpleLib.cql"), CQL).unwrap();
    fs::write(input.join("fsh").join("simple.fsh"), FSH).unwrap();

    packages_dir
}

fn read_json(path: &Path) -> serde_json::Value {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_json::from_str(&content).unwrap()
}

#[test]
fn build_runs_all_four_builtin_processors() {
    let tmp = TempDir::new().unwrap();
    write_fixture(tmp.path());

    let output_dir = tmp.path().join("output");
    let tgz = build(tmp.path(), &output_dir).expect("pipeline build should succeed");
    assert!(tgz.exists(), "tarball missing at {}", tgz.display());

    let pkg = output_dir.join("package");

    // --- snapshot processor: profile gained a generated snapshot ---
    let profile = read_json(&pkg.join("StructureDefinition-MyPatient.json"));
    let elements = profile["snapshot"]["element"]
        .as_array()
        .expect("snapshot.element generated for profile");
    assert_eq!(elements.len(), 3, "all base elements present in snapshot");
    let identifier = elements
        .iter()
        .find(|e| e["path"] == "Patient.identifier")
        .expect("identifier element in snapshot");
    assert_eq!(identifier["min"], 1, "differential merged into snapshot");
    assert_eq!(identifier["max"], "1");

    // --- cql processor: Library resource emitted with base64 CQL content ---
    let library = read_json(&pkg.join("Library-SimpleLib.json"));
    assert_eq!(library["resourceType"], "Library");
    assert_eq!(library["url"], "http://example.org/fhir/Library/SimpleLib");
    let contents = library["content"].as_array().expect("Library.content");
    assert!(
        contents.iter().any(|c| c["contentType"]
            .as_str()
            .unwrap_or_default()
            .contains("cql")),
        "Library should carry CQL content"
    );

    // --- fsh processor: compiled ValueSet present ---
    let vs = read_json(&pkg.join("ValueSet-simple-colors.json"));
    assert_eq!(vs["resourceType"], "ValueSet");
    assert_eq!(vs["id"], "simple-colors");
    assert_eq!(vs["url"], "http://example.org/fhir/ValueSet/simple-colors");
    let includes = vs["compose"]["include"]
        .as_array()
        .expect("compose.include");
    assert!(
        includes
            .iter()
            .any(|inc| inc["system"] == "http://example.org/CodeSystem/colors"),
        "FSH-compiled ValueSet should include the colors system"
    );

    // --- validate processor: ran without failing the build (asserted by
    // build() returning Ok) and the package manifest exists ---
    assert!(pkg.join("package.json").exists());
    let package_json = read_json(&pkg.join("package.json"));
    assert_eq!(package_json["url"], "http://example.org/fhir");
    assert!(package_json.get("canonical").is_none());
    assert!(pkg.join(".index.json").exists());
}

#[test]
fn build_fails_when_validate_finds_invalid_resource() {
    let tmp = TempDir::new().unwrap();
    write_fixture(tmp.path());

    // Inject a resource with an invalid id (violates FHIR id regex) — the
    // validator flags this even with no packages installed.
    fs::write(
        tmp.path().join("input").join("Patient-bogus.json"),
        r#"{ "resourceType": "Patient", "id": "white space!" }"#,
    )
    .unwrap();

    let output_dir = tmp.path().join("output");
    let result = build(tmp.path(), &output_dir);
    assert!(result.is_err(), "validation should fail the build");
}
