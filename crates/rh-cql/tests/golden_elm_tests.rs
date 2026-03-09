//! Golden file tests for the ELM compilation pipeline.
//!
//! These tests verify that the multi-stage CQL compiler (parse → semantic
//! analysis → ELM emit) produces stable, deterministic output by comparing
//! compiled ELM JSON against committed baseline ("golden") files.
//!
//! # Tasks
//!
//! - **11.1** — Golden integration tests comparing ELM output to known-good
//!   baselines (CQL fixture files in `tests/golden/fixtures/`).
//! - **11.2** — Verify the pipeline produces bit-identical output during
//!   refactoring (regression protection).
//!
//! # How it works
//!
//! For each `*.cql` file in `tests/golden/fixtures/`:
//! 1. The CQL is compiled with default options.
//! 2. The resulting ELM library is serialized to pretty JSON.
//! 3. The JSON is compared against the expected baseline in
//!    `tests/golden/expected/<name>.elm.json`.
//!
//! # Updating baselines
//!
//! When an intentional pipeline change alters ELM output, regenerate the
//! baselines with:
//!
//! ```text
//! UPDATE_GOLDEN=1 cargo test -p rh-cql --test golden_elm_tests
//! ```
//!
//! This writes the new expected files; commit them alongside your code change.

use rh_cql::compile;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/fixtures")
}

fn expected_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/expected")
}

fn update_mode() -> bool {
    std::env::var("UPDATE_GOLDEN").is_ok()
}

/// Compile `source` and return the pretty-printed ELM JSON.
///
/// Panics if compilation yields any error-level diagnostics.
fn compile_to_golden_json(name: &str, source: &str) -> String {
    let result =
        compile(source, None).unwrap_or_else(|e| panic!("hard compile failure for '{name}': {e}"));

    assert!(
        result.errors.is_empty(),
        "'{name}' compiled with errors: {:?}",
        result.errors
    );

    result
        .to_json()
        .unwrap_or_else(|e| panic!("JSON serialization failed for '{name}': {e}"))
}

/// Load the expected golden JSON for `name`.
///
/// Returns `None` if the file does not exist (first-run seeding case).
fn load_expected(name: &str) -> Option<String> {
    let path = expected_dir().join(format!("{name}.elm.json"));
    fs::read_to_string(&path).ok()
}

/// Write `json` as the golden baseline for `name`.
fn save_expected(name: &str, json: &str) {
    let dir = expected_dir();
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("cannot create golden/expected dir: {e}"));
    let path = dir.join(format!("{name}.elm.json"));
    fs::write(&path, json).unwrap_or_else(|e| panic!("cannot write golden file '{name}': {e}"));
    println!("  [golden] wrote {}", path.display());
}

/// Run the golden test for a single `name` / `source` pair.
///
/// In update mode (`UPDATE_GOLDEN=1`): writes (or overwrites) the expected
/// file and always passes.
///
/// In normal mode: compares compiled output against the expected file and
/// panics with a diff-friendly message on mismatch.  If no expected file
/// exists yet (first run), it also seeds the file and passes.
fn run_golden(name: &str, source: &str) {
    let actual = compile_to_golden_json(name, source);

    if update_mode() {
        save_expected(name, &actual);
        return;
    }

    match load_expected(name) {
        None => {
            // First-run seeding: write the file so subsequent runs compare.
            eprintln!("[golden] No baseline for '{name}' — seeding and passing this run.");
            save_expected(name, &actual);
        }
        Some(expected) => {
            assert_eq!(
                actual, expected,
                "\nGolden mismatch for '{name}'.\n\
                 Run `UPDATE_GOLDEN=1 cargo test -p rh-cql --test golden_elm_tests` \
                 to update baselines if the change is intentional.\n"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Compiled library identifier normalisation check
//
// In addition to golden comparison, assert that every fixture produces an ELM
// library with a non-empty identifier — a basic structural sanity check.
// ---------------------------------------------------------------------------

fn assert_has_identifier(name: &str, source: &str) {
    let result =
        compile(source, None).unwrap_or_else(|e| panic!("compile failure for '{name}': {e}"));
    let id = result
        .library
        .identifier
        .as_ref()
        .and_then(|i| i.id.as_deref());
    assert!(
        id.map(|s| !s.is_empty()).unwrap_or(false),
        "'{name}' ELM output is missing a library identifier"
    );
}

// ---------------------------------------------------------------------------
// Determinism check
// ---------------------------------------------------------------------------

fn assert_deterministic(name: &str, source: &str) {
    let j1 = compile_to_golden_json(name, source);
    let j2 = compile_to_golden_json(name, source);
    assert_eq!(j1, j2, "'{name}' pipeline output is not deterministic");
}

// ---------------------------------------------------------------------------
// Discover and run all fixture files automatically
// ---------------------------------------------------------------------------

/// Collect all `*.cql` files from the fixtures directory.
fn collect_fixtures() -> Vec<(String, String)> {
    let dir = fixtures_dir();
    let mut fixtures = Vec::new();

    let entries =
        fs::read_dir(&dir).unwrap_or_else(|e| panic!("cannot read fixtures dir {:?}: {e}", dir));

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("cql") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read fixture {path:?}: {e}"));
            fixtures.push((name, source));
        }
    }

    fixtures.sort_by(|a, b| a.0.cmp(&b.0));
    fixtures
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Golden comparison test: for each CQL fixture, compile and compare (or
/// seed) the ELM JSON baseline.
#[test]
fn golden_elm_output() {
    for (name, source) in collect_fixtures() {
        println!("  [golden] checking '{name}'");
        run_golden(&name, &source);
    }
}

/// Determinism test: compiling the same fixture twice always produces
/// identical output.
#[test]
fn golden_elm_output_is_deterministic() {
    for (name, source) in collect_fixtures() {
        assert_deterministic(&name, &source);
    }
}

/// Structural sanity: every fixture must produce an ELM library with an
/// identifier (library name).
#[test]
fn golden_elm_has_library_identifier() {
    for (name, source) in collect_fixtures() {
        assert_has_identifier(&name, &source);
    }
}
