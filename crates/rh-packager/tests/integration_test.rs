//! Integration tests for the rh-packager pipeline.

use rh_packager::pipeline::{build, check, lock, pack_dir};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Copy the fixture directory into a temp dir so tests can mutate it.
fn copy_fixture(tmp: &TempDir) {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/my-package");
    for entry in fs::read_dir(&src).unwrap().flatten() {
        let dest = tmp.path().join(entry.file_name());
        fs::copy(entry.path(), dest).unwrap();
    }
}

#[test]
fn build_produces_output_directory_and_tarball() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    let tgz = build(tmp.path(), &output_dir).unwrap();

    // Tarball should exist.
    assert!(tgz.exists(), "tarball not found at {}", tgz.display());
    assert!(
        tgz.to_string_lossy().ends_with(".tgz"),
        "expected .tgz extension, got {}",
        tgz.display()
    );

    // Expanded output should have a package/ directory.
    let pkg_dir = output_dir.join("package");
    assert!(pkg_dir.is_dir());

    // package.json and .index.json must be present.
    assert!(pkg_dir.join("package.json").exists());
    assert!(pkg_dir.join(".index.json").exists());

    // ValueSet resource must be in the package.
    assert!(pkg_dir.join("ValueSet-condition-codes.json").exists());

    // Standalone overview.md should be in package/other/.
    assert!(pkg_dir.join("other").join("overview.md").exists());
}

#[test]
fn build_embeds_narrative_into_resource() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    build(tmp.path(), &output_dir).unwrap();

    let valueset_path = output_dir
        .join("package")
        .join("ValueSet-condition-codes.json");
    let content = fs::read_to_string(valueset_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // The markdown narrative should have been embedded as .text.div.
    let div = json["text"]["div"]
        .as_str()
        .expect(".text.div should be a string");
    assert!(
        div.contains("<h1>Condition Codes</h1>"),
        "Expected H1 heading in narrative div"
    );
    assert_eq!(json["text"]["status"], "generated");
}

#[test]
fn build_with_cql_creates_library_resource() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    build(tmp.path(), &output_dir).unwrap();

    let lib_path = output_dir.join("package").join("Library-SimpleLib.json");
    assert!(
        lib_path.exists(),
        "Library-SimpleLib.json not found in output"
    );

    let content = fs::read_to_string(lib_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["resourceType"], "Library");

    let content_arr = json["content"]
        .as_array()
        .expect("Library.content must be array");
    let types: Vec<&str> = content_arr
        .iter()
        .filter_map(|c| c["contentType"].as_str())
        .collect();
    assert!(types.contains(&"text/cql"), "Missing text/cql content type");
    assert!(
        types.contains(&"application/elm+json"),
        "Missing ELM content type"
    );
}

#[test]
fn build_index_json_lists_all_resources() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    build(tmp.path(), &output_dir).unwrap();

    let index_path = output_dir.join("package").join(".index.json");
    let content = fs::read_to_string(index_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(json["index-version"], 2);
    let files = json["files"].as_array().expect(".files must be array");
    assert!(!files.is_empty());
}

#[test]
fn pack_produces_tarball_from_output_dir() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    build(tmp.path(), &output_dir).unwrap();

    // Run pack on the already-built output directory.
    let tgz = pack_dir(&output_dir).unwrap();
    assert!(tgz.exists());
    assert!(tgz.to_string_lossy().ends_with(".tgz"));
}

#[test]
fn lock_writes_fhir_lock_json() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    let output_dir = tmp.path().join("output");
    lock(tmp.path(), &output_dir).unwrap();

    let lock_path = tmp.path().join("fhir-lock.json");
    assert!(lock_path.exists(), "fhir-lock.json was not written");

    let content = fs::read_to_string(lock_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["pinMode"], "pin-all");
    assert!(json["generated"].as_str().is_some());
}

#[test]
fn check_passes_valid_source_directory() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);
    // check should succeed without writing any output.
    check(tmp.path()).unwrap();
    assert!(
        !tmp.path().join("output").exists(),
        "check must not write output"
    );
}

#[test]
fn build_with_snapshot_and_validate_hooks() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    // Override packager.toml to configure snapshot + validate before_build hooks.
    fs::write(
        tmp.path().join("packager.toml"),
        "id = \"example.fhir.test\"\nversion = \"1.0.0\"\nfhir_version = \"4.0.1\"\n\n[hooks]\nbefore_build = [\"snapshot\", \"validate\"]\n",
    )
    .unwrap();

    let output_dir = tmp.path().join("output");
    // snapshot processor will skip non-StructureDefinition resources.
    // validate processor runs with no declared dependencies, so it should succeed.
    // In either case, the pipeline must not fail with UnknownProcessor.
    let result = build(tmp.path(), &output_dir);
    match &result {
        Ok(tgz) => {
            // Full success: output and tarball should exist.
            assert!(tgz.exists(), "tarball not found at {}", tgz.display());
            assert!(output_dir.join("package").is_dir());
        }
        Err(e) => {
            // Acceptable failures: validation error if FHIR core packages not installed.
            // Must NOT be an unknown processor error or sync error.
            assert!(
                !matches!(e, rh_packager::PublisherError::UnknownProcessor(_)),
                "Pipeline must not fail with UnknownProcessor: {e:?}"
            );
            assert!(
                !matches!(e, rh_packager::PublisherError::IgSync(_)),
                "Pipeline must not fail with IgSync: {e:?}"
            );
        }
    }
}

#[test]
fn check_fails_on_ig_version_mismatch() {
    let tmp = TempDir::new().unwrap();
    copy_fixture(&tmp);

    // Tamper the IG version.
    let ig_path = tmp.path().join("ImplementationGuide.json");
    let mut ig: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&ig_path).unwrap()).unwrap();
    ig["version"] = serde_json::Value::String("9.9.9".to_string());
    fs::write(&ig_path, serde_json::to_string_pretty(&ig).unwrap()).unwrap();

    let err = check(tmp.path()).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("version") || msg.contains("sync"),
        "Expected sync error mentioning version, got: {msg}"
    );
}
