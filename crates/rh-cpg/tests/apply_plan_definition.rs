use std::sync::Arc;

use rh_cpg::apply::plan_definition::apply_plan_definition;
use rh_cpg::context::ApplyContext;
use rh_cpg::resolver::BundleResolver;
use serde_json::{json, Value};

fn fixture(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    let contents = std::fs::read_to_string(path).expect("fixture should exist");
    serde_json::from_str(&contents).expect("fixture should be valid JSON")
}

fn content_bundle(entries: Vec<Value>) -> Value {
    json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": entries.into_iter().map(|resource| json!({"resource": resource})).collect::<Vec<_>>(),
    })
}

fn context(bundle: &Value) -> ApplyContext {
    ApplyContext::new(
        Arc::new(BundleResolver::new(bundle).expect("bundle should be valid")),
        "Patient/123",
    )
}

fn resource_of(result: &Value, index: usize) -> &Value {
    result
        .get("entry")
        .and_then(Value::as_array)
        .and_then(|entries| entries.get(index))
        .and_then(|entry| entry.get("resource"))
        .expect("expected bundle entry resource")
}

fn resources(result: &Value) -> Vec<&Value> {
    result
        .get("entry")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("resource"))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn simple_plan_definition() {
    let plan = fixture("PlanDefinition-SimplePlanDefinition.json");
    let ctx = context(&content_bundle(Vec::new()));
    let result = apply_plan_definition(&plan, &ctx).expect("apply should succeed");

    let request_group = resource_of(&result, 0);
    assert_eq!(
        request_group.get("resourceType").and_then(Value::as_str),
        Some("RequestGroup")
    );
    assert_eq!(
        request_group
            .get("subject")
            .and_then(|subject| subject.get("reference"))
            .and_then(Value::as_str),
        Some("Patient/123")
    );
    assert_eq!(
        request_group.get("status").and_then(Value::as_str),
        Some("draft")
    );
    assert_eq!(
        request_group.get("intent").and_then(Value::as_str),
        Some("proposal")
    );
    assert_eq!(
        request_group
            .get("instantiatesCanonical")
            .and_then(Value::as_array),
        Some(&vec![json!(
            "http://example.org/PlanDefinition/SimplePlanDefinition|0.1.0"
        )])
    );
}

#[test]
fn plan_definition_with_goal() {
    let plan = fixture("PlanDefinition-SimplePlanDefinitionWithGoal.json");
    let ctx = context(&content_bundle(Vec::new()));
    let result = apply_plan_definition(&plan, &ctx).expect("apply should succeed");

    let request_group = resource_of(&result, 0);
    assert_eq!(
        request_group.get("resourceType").and_then(Value::as_str),
        Some("RequestGroup")
    );

    let goal = resources(&result)
        .into_iter()
        .find(|resource| resource.get("resourceType").and_then(Value::as_str) == Some("Goal"))
        .expect("expected Goal resource");
    assert_eq!(
        goal.pointer("/description/text").and_then(Value::as_str),
        Some("to improve")
    );

    let goal_id = goal.get("id").and_then(Value::as_str).expect("goal id");
    let extension = request_group
        .get("extension")
        .and_then(Value::as_array)
        .expect("goal extension")
        .iter()
        .find(|extension| {
            extension.get("url").and_then(Value::as_str)
                == Some("http://hl7.org/fhir/StructureDefinition/resource-pertainsToGoal")
        })
        .expect("pertains-to-goal extension");
    assert_eq!(
        extension
            .pointer("/valueReference/reference")
            .and_then(Value::as_str),
        Some(format!("Goal/{goal_id}").as_str())
    );
}

#[test]
fn plan_definition_with_action() {
    let plan = fixture("PlanDefinition-PlanDefinitionWithAction.json");
    let ctx = context(&content_bundle(Vec::new()));
    let result = apply_plan_definition(&plan, &ctx).expect("apply should succeed");

    let request_group = resource_of(&result, 0);
    let action = request_group
        .pointer("/action/0")
        .expect("first request group action");
    assert_eq!(action.get("id").and_then(Value::as_str), Some("action-1"));
    assert_eq!(
        action.get("prefix").and_then(Value::as_str),
        Some("action-1 prefix")
    );
    assert_eq!(
        action.get("title").and_then(Value::as_str),
        Some("action-1 title")
    );
    assert_eq!(
        action.get("description").and_then(Value::as_str),
        Some("action-1 description")
    );
    assert_eq!(
        action.get("textEquivalent").and_then(Value::as_str),
        Some("action-1 textEquivalent")
    );
    assert_eq!(
        action.get("priority").and_then(Value::as_str),
        Some("routine")
    );
    assert_eq!(
        action.pointer("/code/0/text").and_then(Value::as_str),
        Some("action-1 code")
    );
    assert_eq!(
        action
            .pointer("/relatedAction/0/actionId")
            .and_then(Value::as_str),
        Some("action-1 relatedActionId")
    );
    assert_eq!(
        action
            .pointer("/relatedAction/0/relationship")
            .and_then(Value::as_str),
        Some("before")
    );
}

#[test]
fn activity_definition_application() {
    let mut plan = fixture("PlanDefinition-PlanDefinitionWithAction.json");
    plan.pointer_mut("/action/0")
        .expect("action")
        .as_object_mut()
        .expect("action object")
        .insert(
            "definitionCanonical".to_string(),
            json!("http://example.org/ActivityDefinition/SimpleActivityDefinition"),
        );
    let bundle = content_bundle(vec![
        plan.clone(),
        fixture("ActivityDefinition-SimpleActivityDefinition.json"),
    ]);
    let ctx = context(&bundle);
    let result = apply_plan_definition(&plan, &ctx).expect("apply should succeed");

    let request_group = resource_of(&result, 0);
    let action = request_group
        .pointer("/action/0")
        .expect("first request group action");

    let medication_request = resources(&result)
        .into_iter()
        .find(|resource| {
            resource.get("resourceType").and_then(Value::as_str) == Some("MedicationRequest")
        })
        .expect("expected MedicationRequest");
    assert_eq!(
        medication_request.get("status").and_then(Value::as_str),
        Some("draft")
    );
    assert_eq!(
        medication_request
            .pointer("/medicationCodeableConcept/coding/0/code")
            .and_then(Value::as_str),
        Some("1946772")
    );

    let medication_id = medication_request
        .get("id")
        .and_then(Value::as_str)
        .expect("medication id");
    assert_eq!(
        action
            .pointer("/resource/reference")
            .and_then(Value::as_str),
        Some(format!("MedicationRequest/{medication_id}").as_str())
    );
    assert_eq!(
        action
            .pointer("/type/coding/0/code")
            .and_then(Value::as_str),
        Some("create")
    );
}
