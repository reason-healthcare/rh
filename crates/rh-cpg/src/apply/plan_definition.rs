use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::apply::action::apply_plan_definition_action;
use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};
pub(crate) const BASE_URL: &str = "http://apply-processor";
const GOAL_EXTENSION_URL: &str = "http://hl7.org/fhir/StructureDefinition/resource-pertainsToGoal";

/// Apply a PlanDefinition ($apply) to a subject, producing a FHIR collection
/// Bundle whose first entry is the primary RequestGroup.
pub fn apply_plan_definition(plan_definition: &Value, ctx: &ApplyContext) -> CpgResult<Value> {
    if plan_definition.get("resourceType").and_then(Value::as_str) != Some("PlanDefinition") {
        return Err(CpgError::InvalidResource(
            "expected a FHIR PlanDefinition object".to_string(),
        ));
    }

    let mut entries = Vec::new();
    let mut request_group = json!({
        "resourceType": "RequestGroup",
        "id": Uuid::new_v4().to_string(),
        "intent": "proposal",
        "status": "draft",
        "subject": {"reference": ctx.subject},
    });

    if let Some(canonical) = canonicalize(plan_definition) {
        set_field(
            &mut request_group,
            "instantiatesCanonical",
            json!([canonical]),
        );
    }

    if let Some(practitioner) = &ctx.practitioner {
        if let Some(organization) = &ctx.organization {
            let practitioner_role = json!({
                "resourceType": "PractitionerRole",
                "id": Uuid::new_v4().to_string(),
                "practitioner": {"reference": practitioner},
                "organization": {"reference": organization},
            });
            push_bundle_entry(&mut entries, &practitioner_role);
            set_field(
                &mut request_group,
                "author",
                json!({"reference": format!("PractitionerRole/{}", practitioner_role["id"]) }),
            );
        } else {
            set_field(
                &mut request_group,
                "author",
                json!({"reference": practitioner}),
            );
        }
    }

    if let Some(encounter) = &ctx.encounter {
        set_field(
            &mut request_group,
            "encounter",
            json!({"reference": encounter}),
        );
    }

    if let Some(goals) = plan_definition.get("goal").and_then(Value::as_array) {
        if !goals.is_empty() {
            let mut extensions = Vec::new();
            for goal_properties in goals {
                let mut goal = Map::new();
                goal.insert("resourceType".to_string(), json!("Goal"));
                goal.insert("id".to_string(), json!(Uuid::new_v4().to_string()));
                goal.insert("subject".to_string(), json!({"reference": ctx.subject}));
                goal.insert("lifecycleStatus".to_string(), json!("proposed"));
                for (field, value) in goal_properties.as_object().into_iter().flatten() {
                    if field == "category" {
                        goal.insert(field.clone(), json!([value.clone()]));
                    } else {
                        goal.insert(field.clone(), value.clone());
                    }
                }

                let goal = Value::Object(goal);
                let goal_id = goal
                    .get("id")
                    .and_then(Value::as_str)
                    .expect("goal id was just generated");
                extensions.push(json!({
                    "url": GOAL_EXTENSION_URL,
                    "valueReference": {"reference": format!("Goal/{goal_id}")}
                }));
                push_bundle_entry(&mut entries, &goal);
            }

            set_field(&mut request_group, "extension", Value::Array(extensions));
        }
    }

    let libraries = resolve_libraries(plan_definition, ctx)?;
    if let Some(actions) = plan_definition.get("action").and_then(Value::as_array) {
        let mut request_group_actions = Vec::new();
        for action in actions {
            if let Some(request_group_action) = apply_plan_definition_action(
                action,
                plan_definition,
                ctx,
                &mut entries,
                &libraries,
            )? {
                request_group_actions.push(request_group_action);
            }
        }

        if !request_group_actions.is_empty() {
            set_field(
                &mut request_group,
                "action",
                Value::Array(request_group_actions),
            );
        }
    }

    let mut bundle = Map::new();
    bundle.insert("resourceType".to_string(), json!("Bundle"));
    bundle.insert("id".to_string(), json!(Uuid::new_v4().to_string()));
    bundle.insert("type".to_string(), json!("collection"));
    bundle.insert(
        "entry".to_string(),
        json!([bundle_entry(&request_group)]
            .into_iter()
            .collect::<Vec<_>>()),
    );

    let mut bundle = Value::Object(bundle);
    if let Some(bundle_entries) = bundle.get_mut("entry").and_then(Value::as_array_mut) {
        bundle_entries.extend(entries);
    }

    Ok(bundle)
}

pub(crate) fn canonicalize(resource: &Value) -> Option<String> {
    let url = resource.get("url").and_then(Value::as_str)?;
    Some(match resource.get("version").and_then(Value::as_str) {
        Some(version) => format!("{url}|{version}"),
        None => url.to_string(),
    })
}

fn resolve_libraries(plan_definition: &Value, ctx: &ApplyContext) -> CpgResult<Vec<Value>> {
    let mut libraries = Vec::new();
    for canonical in plan_definition
        .get("library")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(Value::as_str)
    {
        if let Some(library) = ctx.content_resolver.resolve_canonical(canonical)? {
            libraries.push(library);
        }
    }

    Ok(libraries)
}

fn push_bundle_entry(resource_bundle_entries: &mut Vec<Value>, resource: &Value) {
    resource_bundle_entries.push(bundle_entry(resource));
}

fn bundle_entry(resource: &Value) -> Value {
    let mut entry = Map::new();
    if let Some((resource_type, id)) = resource
        .get("resourceType")
        .and_then(Value::as_str)
        .zip(resource.get("id").and_then(Value::as_str))
    {
        entry.insert(
            "fullUrl".to_string(),
            json!(format!("{BASE_URL}/{resource_type}/{id}")),
        );
    }

    entry.insert("resource".to_string(), resource.clone());
    Value::Object(entry)
}

fn set_field(target: &mut Value, field: &str, value: Value) {
    target
        .as_object_mut()
        .expect("target resource must be an object")
        .insert(field.to_string(), value);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::resolver::BundleResolver;
    use serde_json::json;

    fn content_bundle(include_sub_plan: bool) -> Value {
        let mut plan_definition = json!({
            "resourceType": "PlanDefinition",
            "id": "main",
            "url": "http://test/PlanDefinition/Main",
            "version": "1.0",
            "goal": [{
                "category": {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/goal-category", "code": "physiotherapy"}]},
                "description": {"text": "Improve mobility"}
            }],
            "action": [
                {
                    "title": "Order lab",
                    "definitionCanonical": "http://test/ActivityDefinition/order-lab"
                },
                {
                    "title": "Skipped action",
                    "condition": [{
                        "kind": "applicability",
                        "expression": {
                            "language": "text/fhirpath",
                            "expression": "title = 'Skip me'"
                        }
                    }]
                }
            ]
        });

        if include_sub_plan {
            plan_definition["action"]
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "title": "Use sub plan",
                    "definitionCanonical": "http://test/PlanDefinition/Sub"
                }));
        }

        let mut entries = vec![
            json!({"resource": plan_definition}),
            json!({"resource": {
                "resourceType": "ActivityDefinition",
                "id": "order-lab",
                "url": "http://test/ActivityDefinition/order-lab",
                "kind": "ServiceRequest",
                "code": {"coding": [{"system": "http://loinc.org", "code": "24357-6"}]}
            }}),
        ];

        if include_sub_plan {
            entries.insert(
                1,
                json!({"resource": json!({
                    "resourceType": "PlanDefinition",
                    "id": "sub",
                    "url": "http://test/PlanDefinition/Sub",
                    "action": [{
                        "action": [{
                            "title": "Nested order",
                            "definitionCanonical": "http://test/ActivityDefinition/order-lab"
                        }]
                    }]
                })}),
            );
        }

        json!({"resourceType": "Bundle", "entry": entries})
    }

    fn context(bundle: &Value) -> ApplyContext {
        ApplyContext::new(
            Arc::new(BundleResolver::new(bundle).unwrap()),
            "Patient/123",
        )
    }

    fn main_plan(bundle: &Value) -> Value {
        bundle.get("entry").and_then(Value::as_array).unwrap()[0]
            .get("resource")
            .unwrap()
            .clone()
    }

    #[test]
    fn applies_primary_request_group_and_activity_definition() {
        let bundle = content_bundle(false);
        let ctx = context(&bundle);
        let result = apply_plan_definition(&main_plan(&bundle), &ctx).unwrap();

        let entries = result["entry"].as_array().unwrap();
        let request_group = &entries[0]["resource"];
        assert_eq!(result["resourceType"], "Bundle");
        assert_eq!(result["type"], "collection");
        assert_eq!(request_group["resourceType"], "RequestGroup");
        assert_eq!(request_group["intent"], "proposal");
        assert_eq!(request_group["status"], "draft");
        assert_eq!(
            request_group["subject"],
            json!({"reference": "Patient/123"})
        );
        assert_eq!(
            request_group["instantiatesCanonical"],
            json!(["http://test/PlanDefinition/Main|1.0"])
        );
        assert_eq!(request_group["action"].as_array().unwrap().len(), 1);
        let first_action_reference = request_group["action"][0]["resource"]["reference"]
            .as_str()
            .unwrap();
        assert!(first_action_reference.starts_with("ServiceRequest/"));
        assert!(entries
            .iter()
            .any(|entry| entry["resource"]["resourceType"] == "Goal"));
        assert!(entries
            .iter()
            .any(|entry| entry["resource"]["resourceType"] == "ServiceRequest"));
        assert!(request_group["extension"]
            .as_array()
            .unwrap()
            .iter()
            .any(|extension| { extension["url"] == GOAL_EXTENSION_URL }));
    }

    #[test]
    fn appends_goal_resource_and_reference_extension() {
        let bundle = content_bundle(false);
        let ctx = context(&bundle);
        let result = apply_plan_definition(&main_plan(&bundle), &ctx).unwrap();
        let entries = result["entry"].as_array().unwrap();

        let goal_entry = entries
            .iter()
            .find(|entry| entry["resource"]["resourceType"] == "Goal")
            .unwrap();
        assert!(goal_entry["fullUrl"]
            .as_str()
            .unwrap()
            .starts_with(BASE_URL));
        assert_eq!(goal_entry["resource"]["lifecycleStatus"], "proposed");
        assert_eq!(
            goal_entry["resource"]["category"].as_array().unwrap().len(),
            1
        );
    }

    #[test]
    fn applies_sub_plan_and_references_sub_request_group() {
        let bundle = content_bundle(true);
        let ctx = context(&bundle);
        let result = apply_plan_definition(&main_plan(&bundle), &ctx).unwrap();
        let request_group = &result["entry"][0]["resource"];
        let parent_actions = request_group["action"].as_array().unwrap();
        let sub_reference = parent_actions
            .iter()
            .find_map(|action| {
                action["resource"]["reference"]
                    .as_str()
                    .filter(|reference| reference.starts_with("RequestGroup/"))
            })
            .unwrap();

        let sub_id = sub_reference.strip_prefix("RequestGroup/").unwrap();
        let sub_entry = result["entry"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| {
                entry["resource"]["resourceType"] == "RequestGroup"
                    && entry["resource"]["id"] == sub_id
            })
            .unwrap();

        assert_eq!(sub_entry["resource"]["intent"], "proposal");
        assert_eq!(
            sub_entry["resource"]["instantiatesCanonical"],
            json!(["http://test/PlanDefinition/Sub"])
        );
    }
}
