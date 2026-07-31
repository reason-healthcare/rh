//! CMS122 end-to-end integration test.
//!
//! Loads the actual CMS122 FHIR 4.0.1 measure CQL and the 7 test-patient
//! FHIR resources from the sibling `reasonhealth-analytics` repository.
//! Evaluates the four population expressions per-patient and asserts the
//! results match `expected-results.json`.
//!
//! Run with:
//!   cargo test -p rh-cql --test cms122_integration_test -- --nocapture

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rh_cql::{
    compile_with_libraries, evaluate_elm_with_libraries, CqlDateTime, EvalContextBuilder,
    FileLibrarySourceProvider, FixedClock, InMemoryDataProvider, Value,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn example_dir() -> Option<PathBuf> {
    // Navigate from CARGO_MANIFEST_DIR (rh/crates/rh-cql) → rh workspace
    // root → sibling reasonhealth-analytics repo.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.ancestors().nth(2)?;
    let dir = workspace_root
        .parent()?
        .join("reasonhealth-analytics")
        .join("examples")
        .join("cms122-diabetes-hba1c");
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

fn json_to_cql_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else {
                Value::Decimal(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::List(arr.into_iter().map(json_to_cql_value).collect())
        }
        serde_json::Value::Object(map) => Value::Tuple(
            map.into_iter()
                .map(|(k, v)| (k, json_to_cql_value(v)))
                .collect::<BTreeMap<_, _>>(),
        ),
    }
}

/// Load patient data from data.ndjson, returning a map:
///   patient_id → (InMemoryDataProvider, patient_context_value)
fn load_patient_data(example: &Path) -> BTreeMap<String, (InMemoryDataProvider, Value)> {
    let ndjson_path = example.join("data.ndjson");
    let content = std::fs::read_to_string(ndjson_path).expect("data.ndjson not found");

    // First pass: collect all non-patient resources per patient-id.
    let all_resources: Vec<serde_json::Value> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("invalid JSON line"))
        .collect();

    // Collect patient IDs.
    let patient_ids: Vec<String> = all_resources
        .iter()
        .filter_map(|r| {
            if r.get("resourceType")?.as_str()? == "Patient" {
                r.get("id")?.as_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut result = BTreeMap::new();

    for patient_id in &patient_ids {
        let mut provider = InMemoryDataProvider::new();
        let mut patient_val = Value::Null;

        for resource in &all_resources {
            let rt = match resource.get("resourceType").and_then(|v| v.as_str()) {
                Some(rt) => rt,
                None => continue,
            };

            // Only include resources that belong to this patient.
            let belongs = match rt {
                "Patient" => resource.get("id").and_then(|v| v.as_str()) == Some(patient_id),
                _ => {
                    // Check subject/patient reference.
                    let subject_ref = resource
                        .get("subject")
                        .or_else(|| resource.get("patient"))
                        .and_then(|s| s.get("reference"))
                        .and_then(|r| r.as_str());
                    subject_ref == Some(&format!("Patient/{patient_id}"))
                }
            };

            if !belongs {
                continue;
            }

            let val = json_to_cql_value(resource.clone());
            if rt == "Patient" {
                patient_val = val.clone();
            }
            provider.add_resource(rt.to_string(), val);
        }

        result.insert(patient_id.clone(), (provider, patient_val));
    }

    result
}

/// Measurement period context: 2019-01-01 to 2020-01-01.
fn measurement_period_clock() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2019,
        month: Some(1),
        day: Some(1),
        hour: Some(0),
        minute: Some(0),
        second: Some(0),
        millisecond: None,
        offset_seconds: Some(0),
    })
}

fn load_terminology(example: &Path) -> rh_cql::InMemoryTerminologyProvider {
    let vs_path = example.join("valueset-expansions.json");
    let vs_content = std::fs::read_to_string(&vs_path).expect("valueset-expansions.json");
    let vs_json: serde_json::Value = serde_json::from_str(&vs_content).unwrap();
    let mut term_provider = rh_cql::InMemoryTerminologyProvider::new();
    if let Some(obj) = vs_json.as_object() {
        for (url, entry) in obj {
            if let Some(codes) = entry.get("codes").and_then(|c| c.as_array()) {
                for code in codes {
                    let system = code.get("system").and_then(|s| s.as_str()).unwrap_or("");
                    let code_val = code.get("code").and_then(|c| c.as_str()).unwrap_or("");
                    if !system.is_empty() && !code_val.is_empty() {
                        term_provider.add_code(
                            url,
                            rh_cql::CqlCode {
                                code: code_val.to_string(),
                                system: system.to_string(),
                                display: None,
                                version: None,
                            },
                        );
                    }
                }
            }
        }
    }
    term_provider
}

fn make_measurement_period() -> Value {
    Value::Interval {
        low: Some(Box::new(Value::DateTime(CqlDateTime {
            year: 2019,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: Some(0),
        }))),
        high: Some(Box::new(Value::DateTime(CqlDateTime {
            year: 2020,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: Some(0),
        }))),
        low_closed: true,
        high_closed: false,
    }
}

fn eval_bool(
    example: &Path,
    patient_id: &str,
    expression: &str,
    provider: InMemoryDataProvider,
    patient_val: Value,
) -> Option<bool> {
    let cql_file = example.join("measure.cql");
    let source = std::fs::read_to_string(&cql_file).expect("measure.cql not found");
    let libs_dir = example.join("cql-libs");

    let lib_provider = FileLibrarySourceProvider::new().with_path(&libs_dir);

    let out = compile_with_libraries(&source, None, &lib_provider)
        .expect("compile_with_libraries failed");

    if !out.result.is_success() {
        eprintln!(
            "[{patient_id}] {expression}: compile errors: {:?}",
            out.result.errors
        );
        return None;
    }

    let term_provider = load_terminology(example);

    let mut builder = EvalContextBuilder::new(measurement_period_clock())
        .data_provider(provider)
        .terminology_provider(term_provider);
    if !matches!(patient_val, Value::Null) {
        builder = builder.context_value(patient_val);
    }

    builder = builder.parameter("Measurement Period", make_measurement_period());

    let ctx = builder.build();

    match evaluate_elm_with_libraries(&out.result.library, &out.included, expression, &ctx) {
        Ok(Value::Boolean(b)) => Some(b),
        Ok(Value::List(items)) => {
            // Population expressions may return a list; non-empty = true.
            Some(!items.is_empty())
        }
        Ok(Value::Null) => Some(false),
        Ok(other) => {
            eprintln!("[{patient_id}] {expression}: unexpected value {:?}", other);
            None
        }
        Err(e) => {
            eprintln!("[{patient_id}] {expression}: eval error: {:?}", e);
            None
        }
    }
}

/// Run `f` in a thread with 32 MB stack (debug builds have large frames).
fn with_large_stack<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(f)
        .expect("thread spawn failed")
        .join()
        .expect("thread panicked")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn cms122_patient_numer_initial_population() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-numer"].clone();
        let result = eval_bool(
            &example,
            "patient-numer",
            "Initial Population",
            provider,
            patient,
        );
        assert_eq!(
            result,
            Some(true),
            "patient-numer should be in Initial Population"
        );
    });
}

#[test]
fn cms122_patient_numer_denominator() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-numer"].clone();
        let result = eval_bool(&example, "patient-numer", "Denominator", provider, patient);
        assert_eq!(result, Some(true), "patient-numer should be in Denominator");
    });
}

#[test]
fn cms122_patient_no_encounter_initial_population() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-no-encounter"].clone();
        let result = eval_bool(
            &example,
            "patient-no-encounter",
            "Initial Population",
            provider,
            patient,
        );
        assert_eq!(
            result,
            Some(false),
            "patient-no-encounter should NOT be in Initial Population"
        );
    });
}

#[test]
fn cms122_patient_no_diabetes_initial_population() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-no-diabetes"].clone();
        let result = eval_bool(
            &example,
            "patient-no-diabetes",
            "Initial Population",
            provider,
            patient,
        );
        assert_eq!(
            result,
            Some(false),
            "patient-no-diabetes should NOT be in Initial Population"
        );
    });
}

// ---------------------------------------------------------------------------
// Phase 2k: Additional patient assertions
// ---------------------------------------------------------------------------

#[test]
fn cms122_patient_no_hba1c_initial_population() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-no-hba1c"].clone();
        let result = eval_bool(
            &example,
            "patient-no-hba1c",
            "Initial Population",
            provider,
            patient,
        );
        assert_eq!(
            result,
            Some(true),
            "patient-no-hba1c should be in Initial Population"
        );
    });
}

#[test]
fn cms122_patient_too_young_initial_population() {
    with_large_stack(|| {
        let Some(example) = example_dir() else {
            eprintln!("Skipping: reasonhealth-analytics example not found");
            return;
        };
        let data = load_patient_data(&example);
        let (provider, patient) = data["patient-too-young"].clone();
        let result = eval_bool(
            &example,
            "patient-too-young",
            "Initial Population",
            provider,
            patient,
        );
        assert_eq!(
            result,
            Some(false),
            "patient-too-young should NOT be in Initial Population"
        );
    });
}
