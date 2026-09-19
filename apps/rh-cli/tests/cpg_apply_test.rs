use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

#[test]
fn cpg_apply_outputs_request_group() {
    let temp = TempDir::new().expect("temp dir");
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/rh-cpg/tests/fixtures/PlanDefinition-SimplePlanDefinition.json");
    let plan_path = temp.path().join("plan.json");
    fs::copy(fixture, &plan_path).expect("copy plan fixture");

    let content_path = temp.path().join("content.json");
    fs::write(
        &content_path,
        r#"{"resourceType":"Bundle","type":"collection","entry":[]}"#,
    )
    .expect("write content bundle");

    let output = Command::new(env!("CARGO_BIN_EXE_rh"))
        .args([
            "cpg",
            "apply",
            "--plan-definition",
            plan_path.to_str().unwrap(),
            "--subject",
            "Patient/123",
            "--content",
            content_path.to_str().unwrap(),
        ])
        .output()
        .expect("run rh cpg apply");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let bundle: Value = serde_json::from_slice(&output.stdout).expect("parse JSON stdout");
    assert_eq!(
        bundle
            .pointer("/entry/0/resource/resourceType")
            .and_then(Value::as_str),
        Some("RequestGroup")
    );
}
