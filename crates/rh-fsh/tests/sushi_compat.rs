//! Sushi Compatibility Tests
//!
//! These tests compare rh-fsh output against sushi-generated golden files.
//!
//! # Generating golden files
//!
//! Requires Node.js and npx:
//! ```sh
//! ./scripts/generate-fsh-goldens.sh
//! ```
//!
//! # Running the tests
//! ```sh
//! # All compatibility tests:
//! cargo test -p rh-fsh --test sushi_compat -- --include-ignored
//!
//! # A single category:
//! cargo test -p rh-fsh --test sushi_compat test_value_sets -- --include-ignored
//! ```

use rh_fsh::{CompilerOptions, FshCompiler, FshConfig};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ── Paths ────────────────────────────────────────────────────────────────────

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn goldens_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

// ── Test case discovery ──────────────────────────────────────────────────────

struct CompatTestCase {
    /// Absolute path to the .fsh fixture
    fixture_path: PathBuf,
    /// Directory containing sushi-generated JSON golden files for this fixture
    golden_dir: PathBuf,
    /// Human-readable name (relative fixture path without extension)
    name: String,
}

fn discover_test_cases() -> Vec<CompatTestCase> {
    let fixtures = fixtures_dir();
    let goldens = goldens_dir();
    let mut cases = Vec::new();

    for entry in WalkDir::new(&fixtures)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|x| x == "fsh"))
    {
        let fixture_path = entry.path().to_path_buf();
        let rel = fixture_path.strip_prefix(&fixtures).unwrap();
        // Strip .fsh extension to get the golden subdir name
        let stem = rel.with_extension("");
        let golden_dir = goldens.join(&stem);
        let name = stem.to_string_lossy().into_owned();
        cases.push(CompatTestCase {
            fixture_path,
            golden_dir,
            name,
        });
    }

    cases
}

// ── Compilation ──────────────────────────────────────────────────────────────

fn compile_fixture(fsh_path: &Path) -> Result<Vec<Value>, String> {
    let source =
        fs::read_to_string(fsh_path).map_err(|e| format!("Failed to read fixture: {e}"))?;
    let name = fsh_path.file_name().unwrap().to_string_lossy();
    let compiler = FshCompiler::new(CompilerOptions {
        config: FshConfig {
            canonical: Some("http://rh-fsh-test.example.org".to_string()),
            status: Some("draft".to_string()),
            version: Some("0.1.0".to_string()),
            fhir_version: Some("4.0.1".to_string()),
            ..Default::default()
        },
        pretty_print: true,
    });
    let pkg = compiler
        .compile(&source, &name)
        .map_err(|e| format!("Compile error: {e}"))?;
    Ok(pkg.resources)
}

// ── Golden loading ───────────────────────────────────────────────────────────

fn load_goldens(golden_dir: &Path) -> Vec<Value> {
    if !golden_dir.exists() {
        return Vec::new();
    }
    let mut goldens = Vec::new();
    for entry in WalkDir::new(golden_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
    {
        if let Ok(text) = fs::read_to_string(entry.path()) {
            if let Ok(val) = serde_json::from_str::<Value>(&text) {
                goldens.push(val);
            }
        }
    }
    goldens
}

// ── Comparison helpers ───────────────────────────────────────────────────────

fn resource_key(v: &Value) -> (String, String) {
    let rt = v
        .get("resourceType")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let id = v
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    (rt, id)
}

fn compare_field(actual: &Value, golden: &Value, field: &str, path: &str, diffs: &mut Vec<String>) {
    match (actual.get(field), golden.get(field)) {
        (None, None) => {}
        (Some(_), None) => {} // rh-fsh adds a field golden doesn't have — not a failure
        (None, Some(g)) => {
            diffs.push(format!(
                "{path}/{field}: missing in rh-fsh output, golden has {g}"
            ));
        }
        (Some(a), Some(g)) => {
            if a != g {
                diffs.push(format!(
                    "{path}/{field}: mismatch\n    rh-fsh: {a}\n    golden: {g}"
                ));
            }
        }
    }
}

fn extract_diff_element_paths(sd: &Value) -> Vec<String> {
    sd.get("differential")
        .and_then(|d| d.get("element"))
        .and_then(Value::as_array)
        .map(|elems| {
            elems
                .iter()
                .filter_map(|e| e.get("path").and_then(Value::as_str).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_concepts(cs: &Value) -> Vec<(String, String)> {
    cs.get("concept")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|c| {
                    let code = c.get("code").and_then(Value::as_str)?.to_string();
                    let display = c
                        .get("display")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    Some((code, display))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_compose(compose: &Value) -> Value {
    // Sort include/exclude arrays by system URL for stable comparison
    let mut c = compose.clone();
    for key in &["include", "exclude"] {
        if let Some(arr) = c.get_mut(key).and_then(Value::as_array_mut) {
            arr.sort_by(|a, b| {
                let sa = a.get("system").and_then(Value::as_str).unwrap_or("");
                let sb = b.get("system").and_then(Value::as_str).unwrap_or("");
                sa.cmp(sb)
            });
        }
    }
    c
}

fn compare_value_set(actual: &Value, golden: &Value, path: &str, diffs: &mut Vec<String>) {
    match (actual.get("compose"), golden.get("compose")) {
        (None, None) => {}
        (Some(_), None) => {}
        (None, Some(_)) => {
            diffs.push(format!("{path}/compose: missing in rh-fsh output"));
        }
        (Some(a), Some(g)) => {
            let a_norm = normalize_compose(a);
            let g_norm = normalize_compose(g);
            if a_norm != g_norm {
                diffs.push(format!(
                    "{path}/compose: mismatch\n    rh-fsh: {a_norm}\n    golden: {g_norm}"
                ));
            }
        }
    }
}

fn compare_code_system(actual: &Value, golden: &Value, path: &str, diffs: &mut Vec<String>) {
    let actual_concepts = extract_concepts(actual);
    let golden_concepts = extract_concepts(golden);
    if actual_concepts != golden_concepts {
        diffs.push(format!(
            "{path}/concept: mismatch\n    rh-fsh: {actual_concepts:?}\n    golden: {golden_concepts:?}"
        ));
    }
}

fn compare_structure_def(actual: &Value, golden: &Value, path: &str, diffs: &mut Vec<String>) {
    // Compare only differential (rh-fsh doesn't produce snapshots)
    let actual_paths = extract_diff_element_paths(actual);
    let golden_paths = extract_diff_element_paths(golden);

    for p in &actual_paths {
        if !golden_paths.contains(p) {
            diffs.push(format!(
                "{path}/differential: rh-fsh produced unexpected element path: {p}"
            ));
        }
    }
    // Also check every golden path is present in actual
    for p in &golden_paths {
        if !actual_paths.contains(p) {
            diffs.push(format!(
                "{path}/differential: rh-fsh missing element path: {p} (present in golden)"
            ));
        }
    }
}

fn compare_instance(actual: &Value, golden: &Value, path: &str, diffs: &mut Vec<String>) {
    // For instances, compare all top-level scalar and object fields
    // except meta (which may differ) and text (narrative, not set by rh-fsh)
    let skip_fields = [
        "meta",
        "text",
        "resourceType",
        "id",
        "url",
        "name",
        "title",
        "description",
    ];

    if let Some(golden_obj) = golden.as_object() {
        if let Some(actual_obj) = actual.as_object() {
            for (key, g_val) in golden_obj {
                if skip_fields.contains(&key.as_str()) {
                    continue;
                }
                match actual_obj.get(key) {
                    None => {
                        diffs.push(format!(
                            "{path}/{key}: missing in rh-fsh output, golden has {g_val}"
                        ));
                    }
                    Some(a_val) if a_val != g_val => {
                        diffs.push(format!(
                            "{path}/{key}: mismatch\n    rh-fsh: {a_val}\n    golden: {g_val}"
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn compare_resource(actual: &Value, golden: &Value, fixture_name: &str) -> Vec<String> {
    let rt = golden
        .get("resourceType")
        .and_then(Value::as_str)
        .unwrap_or("?");
    let id = golden.get("id").and_then(Value::as_str).unwrap_or("?");
    let path = format!("{fixture_name}/{rt}/{id}");
    let mut diffs = Vec::new();

    for field in &["resourceType", "id", "url", "name", "title", "description"] {
        compare_field(actual, golden, field, &path, &mut diffs);
    }

    match rt {
        "ValueSet" => compare_value_set(actual, golden, &path, &mut diffs),
        "CodeSystem" => compare_code_system(actual, golden, &path, &mut diffs),
        "StructureDefinition" => compare_structure_def(actual, golden, &path, &mut diffs),
        _ => compare_instance(actual, golden, &path, &mut diffs),
    }

    diffs
}

// ── Core comparison ──────────────────────────────────────────────────────────

fn compare_outputs(actual: &[Value], golden: &[Value], fixture_name: &str) -> Vec<String> {
    let mut diffs = Vec::new();

    for g in golden {
        let key = resource_key(g);
        if key.0.is_empty() && key.1.is_empty() {
            continue;
        }
        match actual.iter().find(|a| resource_key(a) == key) {
            None => {
                diffs.push(format!(
                    "{fixture_name}: rh-fsh missing resource ({}, {})",
                    key.0, key.1
                ));
            }
            Some(a) => {
                diffs.extend(compare_resource(a, g, fixture_name));
            }
        }
    }

    diffs
}

// ── Category runner ──────────────────────────────────────────────────────────

fn run_category(category: &str) {
    let cases = discover_test_cases();
    let category_cases: Vec<_> = cases
        .iter()
        .filter(|c| c.name.starts_with(category))
        .collect();

    assert!(
        !category_cases.is_empty(),
        "No test cases found for category '{category}' — check fixtures dir"
    );

    let mut failures = Vec::new();
    let mut pass_count = 0;
    let mut skip_count = 0;

    for case in &category_cases {
        let golden = load_goldens(&case.golden_dir);
        if golden.is_empty() {
            eprintln!(
                "SKIP {} (no goldens — run scripts/generate-fsh-goldens.sh)",
                case.name
            );
            skip_count += 1;
            continue;
        }

        match compile_fixture(&case.fixture_path) {
            Err(e) => {
                eprintln!("FAIL {} — compile error: {e}", case.name);
                failures.push((case.name.clone(), vec![format!("compile error: {e}")]));
            }
            Ok(actual) => {
                let mismatches = compare_outputs(&actual, &golden, &case.name);
                if mismatches.is_empty() {
                    eprintln!("PASS {}", case.name);
                    pass_count += 1;
                } else {
                    eprintln!("FAIL {}", case.name);
                    for m in &mismatches {
                        eprintln!("  {m}");
                    }
                    failures.push((case.name.clone(), mismatches));
                }
            }
        }
    }

    eprintln!(
        "\n{}/{} tests passed ({} skipped)",
        pass_count,
        category_cases.len(),
        skip_count
    );

    if !failures.is_empty() {
        panic!(
            "{} sushi compatibility failure(s) in category '{category}' (see above)",
            failures.len()
        );
    }
}

// ── All-cases test ───────────────────────────────────────────────────────────

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_all_sushi_compat() {
    let cases = discover_test_cases();
    assert!(
        !cases.is_empty(),
        "No test cases found — check tests/fixtures dir"
    );

    let mut failures = Vec::new();
    let mut pass_count = 0;
    let mut skip_count = 0;

    for case in &cases {
        let golden = load_goldens(&case.golden_dir);
        if golden.is_empty() {
            eprintln!("SKIP {} (no goldens)", case.name);
            skip_count += 1;
            continue;
        }

        match compile_fixture(&case.fixture_path) {
            Err(e) => {
                eprintln!("FAIL {} — compile error: {e}", case.name);
                failures.push((case.name.clone(), vec![format!("compile error: {e}")]));
            }
            Ok(actual) => {
                let mismatches = compare_outputs(&actual, &golden, &case.name);
                if mismatches.is_empty() {
                    eprintln!("PASS {}", case.name);
                    pass_count += 1;
                } else {
                    eprintln!("FAIL {}", case.name);
                    for m in &mismatches {
                        eprintln!("  {m}");
                    }
                    failures.push((case.name.clone(), mismatches));
                }
            }
        }
    }

    eprintln!(
        "\n{}/{} tests passed ({} skipped)",
        pass_count,
        cases.len(),
        skip_count
    );

    if !failures.is_empty() {
        panic!(
            "{} sushi compatibility failure(s) (see above)",
            failures.len()
        );
    }
}

// ── Per-category tests ───────────────────────────────────────────────────────

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_value_sets() {
    run_category("Value Sets");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_code_systems() {
    run_category("Code Systems");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_instances() {
    run_category("Instances");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_profiles() {
    run_category("Profiles");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_extensions() {
    run_category("Extensions");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_invariants() {
    run_category("Invariants");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_logicals() {
    run_category("Logicals");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_rule_sets() {
    run_category("Rule Sets");
}

#[test]
#[ignore = "requires sushi-generated golden files; run scripts/generate-fsh-goldens.sh first"]
fn test_rules() {
    run_category("Rules");
}
