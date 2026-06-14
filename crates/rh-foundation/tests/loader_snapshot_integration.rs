//! Integration tests covering the package loader, StructureDefinition loader,
//! and snapshot generator working together against an on-disk fixture FHIR
//! package. No network access is required.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rh_foundation::snapshot::{SnapshotError, SnapshotGenerator, StructureDefinitionLoader};

const PACKAGE_NAME: &str = "test.fixture.pkg";
const PACKAGE_VERSION: &str = "1.0.0";
const BASE_URL: &str = "http://example.org/StructureDefinition/BasePatient";
const PROFILE_URL: &str = "http://example.org/StructureDefinition/MyPatient";

fn base_structure_definition() -> &'static str {
    r#"{
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/BasePatient",
        "name": "BasePatient",
        "type": "Patient",
        "snapshot": {
            "element": [
                { "path": "Patient", "min": 0, "max": "*" },
                { "path": "Patient.identifier", "min": 0, "max": "*", "short": "Business identifier" },
                { "path": "Patient.name", "min": 0, "max": "*" }
            ]
        }
    }"#
}

fn profile_structure_definition() -> &'static str {
    r#"{
        "resourceType": "StructureDefinition",
        "url": "http://example.org/StructureDefinition/MyPatient",
        "name": "MyPatient",
        "type": "Patient",
        "baseDefinition": "http://example.org/StructureDefinition/BasePatient",
        "differential": {
            "element": [
                { "path": "Patient.identifier", "min": 1, "max": "1", "mustSupport": true }
            ]
        }
    }"#
}

/// Lay out a fixture package as `<packages_dir>/<name>#<version>/package/`.
fn write_fixture_package(packages_dir: &Path) -> PathBuf {
    let package_dir = packages_dir
        .join(format!("{PACKAGE_NAME}#{PACKAGE_VERSION}"))
        .join("package");
    fs::create_dir_all(&package_dir).expect("create fixture package dir");

    fs::write(
        package_dir.join("package.json"),
        format!(r#"{{ "name": "{PACKAGE_NAME}", "version": "{PACKAGE_VERSION}" }}"#),
    )
    .expect("write package.json");
    fs::write(
        package_dir.join("StructureDefinition-BasePatient.json"),
        base_structure_definition(),
    )
    .expect("write base SD");
    fs::write(
        package_dir.join("StructureDefinition-MyPatient.json"),
        profile_structure_definition(),
    )
    .expect("write profile SD");
    // A non-StructureDefinition resource that the loader must skip.
    fs::write(
        package_dir.join("Patient-example.json"),
        r#"{ "resourceType": "Patient", "id": "example" }"#,
    )
    .expect("write example resource");

    package_dir
}

fn loaded_generator(packages_dir: &Path) -> SnapshotGenerator {
    let sds =
        StructureDefinitionLoader::load_from_package(PACKAGE_NAME, PACKAGE_VERSION, packages_dir)
            .expect("load fixture package");
    let mut generator = SnapshotGenerator::new();
    generator.load_structure_definitions(sds);
    generator
}

#[test]
fn load_from_package_finds_structure_definitions_and_skips_other_resources() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_fixture_package(tmp.path());

    let sds =
        StructureDefinitionLoader::load_from_package(PACKAGE_NAME, PACKAGE_VERSION, tmp.path())
            .expect("load fixture package");

    assert_eq!(
        sds.len(),
        2,
        "expected exactly the two StructureDefinitions"
    );
    let mut urls: Vec<&str> = sds.iter().map(|sd| sd.url.as_str()).collect();
    urls.sort_unstable();
    assert_eq!(urls, vec![BASE_URL, PROFILE_URL]);
}

#[test]
fn load_from_package_errors_for_missing_package() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let result = StructureDefinitionLoader::load_from_package("no.such.pkg", "9.9.9", tmp.path());
    assert!(result.is_err());
}

#[test]
fn profile_base_definition_is_parsed_from_fhir_json() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_fixture_package(tmp.path());

    let sds =
        StructureDefinitionLoader::load_from_package(PACKAGE_NAME, PACKAGE_VERSION, tmp.path())
            .expect("load fixture package");
    let profile = sds
        .iter()
        .find(|sd| sd.url == PROFILE_URL)
        .expect("profile");

    // Regression guard: baseDefinition uses FHIR camelCase in JSON.
    assert_eq!(profile.base_definition.as_deref(), Some(BASE_URL));
}

#[test]
fn snapshot_round_trip_merges_differential_onto_base() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_fixture_package(tmp.path());
    let generator = loaded_generator(tmp.path());

    let snapshot = generator.generate_snapshot(PROFILE_URL).expect("snapshot");

    assert_eq!(snapshot.element.len(), 3, "all base elements present");

    let identifier = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.identifier")
        .expect("identifier element");
    assert_eq!(identifier.min, Some(1), "differential narrowed min");
    assert_eq!(
        identifier.max.as_deref(),
        Some("1"),
        "differential narrowed max"
    );
    assert_eq!(identifier.must_support, Some(true));
    assert_eq!(
        identifier.short.as_deref(),
        Some("Business identifier"),
        "descriptive fields inherited from base"
    );

    let name = snapshot
        .element
        .iter()
        .find(|e| e.path == "Patient.name")
        .expect("name element");
    assert_eq!(
        name.max.as_deref(),
        Some("*"),
        "untouched element inherited"
    );
}

#[test]
fn snapshot_cache_returns_shared_arc_without_cloning() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_fixture_package(tmp.path());
    let generator = loaded_generator(tmp.path());

    let first = generator.generate_snapshot(PROFILE_URL).expect("snapshot");
    let second = generator.generate_snapshot(PROFILE_URL).expect("snapshot");

    assert!(
        Arc::ptr_eq(&first, &second),
        "cache hit must return the same allocation"
    );
    assert!(generator.cache_size() >= 1);

    generator.clear_cache();
    assert_eq!(generator.cache_size(), 0);
}

#[test]
fn generate_snapshot_for_unknown_url_is_base_not_found() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_fixture_package(tmp.path());
    let generator = loaded_generator(tmp.path());

    let err = generator
        .generate_snapshot("http://example.org/StructureDefinition/Nope")
        .expect_err("must fail");
    assert!(matches!(err, SnapshotError::BaseNotFound(_)));
}

#[test]
fn circular_base_definitions_are_detected() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let package_dir = tmp
        .path()
        .join(format!("{PACKAGE_NAME}#{PACKAGE_VERSION}"))
        .join("package");
    fs::create_dir_all(&package_dir).expect("create fixture package dir");

    for (name, base) in [("A", "B"), ("B", "A")] {
        fs::write(
            package_dir.join(format!("StructureDefinition-{name}.json")),
            format!(
                r#"{{
                    "resourceType": "StructureDefinition",
                    "url": "http://example.org/StructureDefinition/{name}",
                    "name": "{name}",
                    "type": "Patient",
                    "baseDefinition": "http://example.org/StructureDefinition/{base}",
                    "differential": {{ "element": [ {{ "path": "Patient" }} ] }}
                }}"#
            ),
        )
        .expect("write SD");
    }

    let generator = loaded_generator(tmp.path());
    let err = generator
        .generate_snapshot("http://example.org/StructureDefinition/A")
        .expect_err("must fail");
    assert!(matches!(err, SnapshotError::CircularDependency(_)));
}

#[cfg(feature = "http")]
mod with_loader {
    use super::*;
    use rh_foundation::loader::{LoaderConfig, PackageLoader};

    #[test]
    fn is_package_downloaded_detects_fixture_package_without_network() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_fixture_package(tmp.path());

        let loader = PackageLoader::new(LoaderConfig::default()).expect("loader");
        assert!(loader
            .is_package_downloaded(PACKAGE_NAME, PACKAGE_VERSION, tmp.path())
            .expect("check downloaded"));
        assert!(!loader
            .is_package_downloaded("absent.pkg", "0.0.0", tmp.path())
            .expect("check downloaded"));
    }
}
