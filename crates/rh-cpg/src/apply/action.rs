use serde_json::{json, Map, Value};

use crate::apply::activity_definition::apply_activity_definition;
use crate::apply::dynamic_value::process_dynamic_value;
use crate::apply::plan_definition::{apply_plan_definition, BASE_URL};
use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};

const ACTION_TYPE_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/action-type";
const CREATE_ACTION_CODE: &str = "create";

/// Apply a single PlanDefinition action to a subject, returning the resulting
/// RequestGroupAction and appending created resources to the caller's bundle
/// entries. Returns `None` when the action is not applicable.
pub fn apply_plan_definition_action(
    action: &Value,
    plan_definition: &Value,
    ctx: &ApplyContext,
    resource_bundle_entries: &mut Vec<Value>,
    libraries: &[Value],
    request_group_notes: &mut Vec<Value>,
) -> CpgResult<Option<Value>> {
    if !is_applicable(action, plan_definition, ctx, libraries)? {
        return Ok(None);
    }

    let mut request_group_action = Map::new();
    for field in [
        "id",
        "prefix",
        "title",
        "description",
        "priority",
        "textEquivalent",
        "type",
        "groupingBehavior",
        "selectionBehavior",
        "requiredBehavior",
        "precheckBehavior",
        "cardinalityBehavior",
        "code",
        "documentation",
        "condition",
        "relatedAction",
    ] {
        copy_non_null_field(&mut request_group_action, action, field);
    }

    for timing_field in [
        "timingAge",
        "timingDuration",
        "timingDateTime",
        "timingPeriod",
        "timingRange",
        "timingTiming",
    ] {
        if copy_non_null_field(&mut request_group_action, action, timing_field) {
            break;
        }
    }

    if let Some(children) = action.get("action").and_then(Value::as_array) {
        if !children.is_empty() {
            if let Some(definition_canonical) =
                non_null_field(action, "definitionCanonical").and_then(Value::as_str)
            {
                return Err(CpgError::InvalidResource(format!(
                    "PlanDefinition action cannot apply definitionCanonical '{definition_canonical}' and nested actions together"
                )));
            }
            let mut child_actions = Vec::new();
            for child in children {
                if let Some(child_action) = apply_plan_definition_action(
                    child,
                    plan_definition,
                    ctx,
                    resource_bundle_entries,
                    libraries,
                    request_group_notes,
                )? {
                    child_actions.push(child_action);
                }
            }

            // RequestGroup.action is required to contain either a resource or
            // nested actions. Structural PlanDefinition actions may carry
            // explanatory text but have no executable definition; omit those
            // leaves instead of emitting an invalid empty RequestGroup.action.
            if child_actions.is_empty() {
                return Ok(None);
            }
            request_group_action.insert("action".to_string(), Value::Array(child_actions));
            return Ok(Some(Value::Object(request_group_action)));
        }
    }

    let library_canonicals = library_canonical_strings(libraries);
    apply_definition(
        action,
        plan_definition,
        ctx,
        resource_bundle_entries,
        &library_canonicals,
        &mut request_group_action,
    )?;

    // A PlanDefinition action with neither an executable definition nor an
    // applicable child action is descriptive source content, not a valid
    // RequestGroup action. Preserve applicable guidance in an R4
    // RequestGroup.note, rather than emitting an invalid empty action or
    // fabricating an order.
    if !request_group_action.contains_key("resource") {
        if let Some(note) = informational_note(action) {
            request_group_notes.push(note);
        }
        return Ok(None);
    }

    Ok(Some(Value::Object(request_group_action)))
}

fn informational_note(action: &Value) -> Option<Value> {
    let mut paragraphs = Vec::new();
    if let Some(id) = action.get("id").and_then(Value::as_str) {
        paragraphs.push(format!("Source action: {id}"));
    }
    if let Some(prefix) = action.get("prefix").and_then(Value::as_str) {
        paragraphs.push(format!("Prefix: {prefix}"));
    }
    if let Some(title) = action.get("title").and_then(Value::as_str) {
        paragraphs.push(title.to_string());
    }
    if let Some(text_equivalent) = action.get("textEquivalent").and_then(Value::as_str) {
        paragraphs.push(format!("Text equivalent: {text_equivalent}"));
    }
    if let Some(description) = action.get("description").and_then(Value::as_str) {
        paragraphs.push(description.to_string());
    }
    for documentation in action
        .get("documentation")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let citation = documentation.get("citation").and_then(Value::as_str);
        let label = documentation.get("label").and_then(Value::as_str);
        let url = documentation.get("url").and_then(Value::as_str);
        let resource = documentation.get("resource").and_then(Value::as_str);
        let source = match (label, citation) {
            (Some(label), Some(citation)) => Some(format!("Source ({label}): {citation}")),
            (Some(label), None) => Some(format!("Source: {label}")),
            (None, Some(citation)) => Some(format!("Source: {citation}")),
            (None, None) => None,
        };
        if let Some(source) = source {
            paragraphs.push(source);
        }
        if let Some(url) = url {
            paragraphs.push(format!("Source URL: {url}"));
        }
        if let Some(resource) = resource {
            paragraphs.push(format!("Source resource: {resource}"));
        }
    }

    (!paragraphs.is_empty()).then(|| json!({"text": paragraphs.join("\n\n")}))
}

fn apply_definition(
    action: &Value,
    plan_definition: &Value,
    ctx: &ApplyContext,
    resource_bundle_entries: &mut Vec<Value>,
    library_canonicals: &[String],
    request_group_action: &mut Map<String, Value>,
) -> CpgResult<()> {
    let Some(definition_canonical) =
        non_null_field(action, "definitionCanonical").and_then(Value::as_str)
    else {
        return Ok(());
    };

    let definition_resource = ctx
        .content_resolver
        .resolve_canonical(definition_canonical)?
        .ok_or_else(|| CpgError::CanonicalNotFound(definition_canonical.to_string()))?;

    let definition_type = definition_resource
        .get("resourceType")
        .and_then(Value::as_str);

    match definition_type {
        Some("ActivityDefinition") => {
            let applied = apply_activity_definition(
                &definition_resource,
                &canonical_strings(
                    definition_resource
                        .get("library")
                        .and_then(Value::as_array)
                        .map(Vec::as_slice)
                        .unwrap_or_default(),
                ),
                ctx,
            )?;

            let Some(mut applied) = applied else {
                return Ok(());
            };

            if applied.get("intent").is_some() {
                set_field(&mut applied, "intent", json!("option"));
            }

            request_group_action.insert(
                "type".to_string(),
                json!({
                    "coding": [{
                        "system": ACTION_TYPE_SYSTEM,
                        "code": CREATE_ACTION_CODE
                    }]
                }),
            );

            if let Some(dynamic_values) =
                non_null_field(action, "dynamicValue").and_then(Value::as_array)
            {
                for dynamic_value in dynamic_values {
                    process_dynamic_value(
                        dynamic_value,
                        plan_definition,
                        &mut applied,
                        library_canonicals,
                        ctx,
                    )?;
                }
            }

            push_applied_resource(resource_bundle_entries, &applied);
            set_resource_reference(request_group_action, &applied);
        }
        Some("PlanDefinition") => {
            let sub_bundle = apply_plan_definition(&definition_resource, ctx)?;
            let sub_entries = sub_bundle
                .get("entry")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    CpgError::InvalidResource(
                        "sub PlanDefinition result has no bundle entries".to_string(),
                    )
                })?;

            let sub_request_group = sub_entries
                .first()
                .and_then(|entry| entry.get("resource"))
                .filter(|resource| {
                    resource.get("resourceType").and_then(Value::as_str) == Some("RequestGroup")
                })
                .ok_or_else(|| {
                    CpgError::InvalidResource(
                        "sub PlanDefinition result does not begin with a RequestGroup".to_string(),
                    )
                })?;

            let mut sub_request_group = sub_request_group.clone();
            set_field(&mut sub_request_group, "intent", json!("option"));
            push_bundle_entry(resource_bundle_entries, &sub_request_group);
            set_resource_reference(request_group_action, &sub_request_group);

            for entry in sub_entries.iter().skip(1) {
                resource_bundle_entries.push(entry.clone());
            }
        }
        Some("Questionnaire") => {
            let questionnaire_id = definition_resource
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    CpgError::InvalidResource("resolved Questionnaire has no id".to_string())
                })?;
            request_group_action.insert(
                "resource".to_string(),
                json!({"reference": format!("Questionnaire/{questionnaire_id}")}),
            );
        }
        _ => {}
    }

    Ok(())
}

fn is_applicable(
    action: &Value,
    plan_definition: &Value,
    ctx: &ApplyContext,
    libraries: &[Value],
) -> CpgResult<bool> {
    let Some(conditions) = action.get("condition").and_then(Value::as_array) else {
        return Ok(true);
    };

    let applicability_conditions = conditions
        .iter()
        .filter(|condition| condition.get("kind").and_then(Value::as_str) == Some("applicability"));

    for condition in applicability_conditions {
        let Some(expression) = expression_for_condition(condition) else {
            continue;
        };

        let result = crate::expression::evaluate_expression(
            expression,
            plan_definition,
            &library_canonical_strings(libraries),
            ctx,
        )?;

        if result != Value::Bool(true) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn expression_for_condition(condition: &Value) -> Option<&Value> {
    let expression = condition.get("expression")?;
    let has_source = expression
        .get("expression")
        .and_then(Value::as_str)
        .is_some_and(|source| !source.trim().is_empty());
    has_source.then_some(expression)
}

/// Extract canonical URL strings from a FHIR `library` array (which holds
/// canonical strings, not Library resources).
fn canonical_strings(canonicals: &[Value]) -> Vec<String> {
    canonicals
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn library_canonical_strings(libraries: &[Value]) -> Vec<String> {
    libraries
        .iter()
        .filter_map(|library| library.get("url").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}

fn push_applied_resource(resource_bundle_entries: &mut Vec<Value>, resource: &Value) {
    push_bundle_entry(resource_bundle_entries, resource);
}

fn push_bundle_entry(resource_bundle_entries: &mut Vec<Value>, resource: &Value) {
    let mut entry = Map::new();
    if let Some(resource_type) = resource.get("resourceType").and_then(Value::as_str) {
        if let Some(id) = resource.get("id").and_then(Value::as_str) {
            entry.insert(
                "fullUrl".to_string(),
                json!(format!("{BASE_URL}/{resource_type}/{id}")),
            );
        }
    }

    entry.insert("resource".to_string(), resource.clone());
    resource_bundle_entries.push(Value::Object(entry));
}

fn set_resource_reference(request_group_action: &mut Map<String, Value>, resource: &Value) {
    let Some((resource_type, id)) = resource
        .get("resourceType")
        .and_then(Value::as_str)
        .zip(resource.get("id").and_then(Value::as_str))
    else {
        return;
    };

    request_group_action.insert(
        "resource".to_string(),
        json!({"reference": format!("{resource_type}/{id}")}),
    );
}

fn copy_non_null_field(target: &mut Map<String, Value>, source: &Value, field: &str) -> bool {
    match non_null_field(source, field) {
        Some(value) => {
            target.insert(field.to_string(), value.clone());
            true
        }
        None => false,
    }
}

fn non_null_field<'a>(resource: &'a Value, field: &str) -> Option<&'a Value> {
    resource.get(field).filter(|value| !value.is_null())
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

    #[test]
    fn non_boolean_applicability_result_makes_action_not_applicable() {
        let plan_definition = json!({
            "resourceType": "PlanDefinition",
            "title": "Skipped"
        });
        let action = json!({
            "title": "Action",
            "condition": [{
                "kind": "applicability",
                "expression": {
                    "language": "text/fhirpath",
                    "expression": "title"
                }
            }]
        });
        let ctx = ApplyContext::new(Arc::new(BundleResolver::default()), "Patient/123");
        let mut entries = Vec::new();

        let result = apply_plan_definition_action(
            &action,
            &plan_definition,
            &ctx,
            &mut entries,
            &[],
            &mut Vec::new(),
        )
        .unwrap();

        assert!(result.is_none());
        assert!(entries.is_empty());
    }

    #[test]
    fn declared_missing_definition_canonical_fails_closed() {
        let plan_definition = json!({"resourceType": "PlanDefinition"});
        let action = json!({
            "title": "Action",
            "definitionCanonical": "http://example.org/ActivityDefinition/missing"
        });
        let ctx = ApplyContext::new(Arc::new(BundleResolver::default()), "Patient/123");
        let mut entries = Vec::new();

        let error = apply_plan_definition_action(
            &action,
            &plan_definition,
            &ctx,
            &mut entries,
            &[],
            &mut Vec::new(),
        )
        .unwrap_err();

        assert!(
            matches!(error, CpgError::CanonicalNotFound(canonical) if canonical.ends_with("/missing"))
        );
        assert!(entries.is_empty());
    }

    #[test]
    fn descriptive_action_without_definition_is_omitted() {
        let plan_definition = json!({"resourceType": "PlanDefinition"});
        let action = json!({
            "id": "interpret",
            "prefix": "2",
            "title": "Interpret the screen",
            "textEquivalent": "Review the screen result.",
            "description": "Informational source content without an executable resource",
            "documentation": [{
                "label": "screening-evidence",
                "citation": "Evidence source",
                "url": "https://example.org/evidence",
                "resource": "Library/evidence"
            }]
        });
        let ctx = ApplyContext::new(Arc::new(BundleResolver::default()), "Patient/123");
        let mut entries = Vec::new();

        let mut notes = Vec::new();
        let result = apply_plan_definition_action(
            &action,
            &plan_definition,
            &ctx,
            &mut entries,
            &[],
            &mut notes,
        )
        .expect("descriptive action should not fail execution");

        assert!(result.is_none());
        assert!(entries.is_empty());
        assert_eq!(
            notes,
            vec![
                json!({"text": "Source action: interpret\n\nPrefix: 2\n\nInterpret the screen\n\nText equivalent: Review the screen result.\n\nInformational source content without an executable resource\n\nSource (screening-evidence): Evidence source\n\nSource URL: https://example.org/evidence\n\nSource resource: Library/evidence"})
            ]
        );
    }

    #[test]
    fn mixed_definition_and_nested_actions_fail_closed() {
        let plan_definition = json!({"resourceType": "PlanDefinition"});
        let action = json!({
            "definitionCanonical": "http://example.org/ActivityDefinition/order",
            "action": [{"title": "Nested"}]
        });
        let ctx = ApplyContext::new(Arc::new(BundleResolver::default()), "Patient/123");
        let mut entries = Vec::new();
        let mut notes = Vec::new();

        let error = apply_plan_definition_action(
            &action,
            &plan_definition,
            &ctx,
            &mut entries,
            &[],
            &mut notes,
        )
        .expect_err("mixed source action needs an explicit runtime representation");

        assert!(
            matches!(error, CpgError::InvalidResource(message) if message.contains("definitionCanonical"))
        );
        assert!(entries.is_empty());
        assert!(notes.is_empty());
    }
}
