//! SDC Questionnaire assembly, population, and lightweight response checks.

use serde_json::{json, Map, Value};
use std::collections::HashSet;
use uuid::Uuid;

use crate::context::ApplyContext;
use crate::error::{CpgError, CpgResult};
use crate::expression::evaluate_expression;
const ASSEMBLE_EXPECTATION: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-assemble-expectation";
const ASSEMBLED_FROM: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-assembledFrom";
const CQF_LIBRARY: &str = "http://hl7.org/fhir/StructureDefinition/cqf-library";
const INITIAL_EXPRESSION: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-initialExpression";
const SUB_QUESTIONNAIRE: &str =
    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-subQuestionnaire";

/// Assemble a modular Questionnaire by recursively substituting
/// `sdc-questionnaire-subQuestionnaire` extensions with the referenced
/// Questionnaire's items.
///
/// Contained sub-questionnaires are resolved before `contained` is removed.
pub fn assemble_questionnaire(questionnaire: &Value, ctx: &ApplyContext) -> CpgResult<Value> {
    validate_questionnaire(questionnaire)?;
    let mut visited = HashSet::new();
    if let Some(url) = canonical_url(questionnaire) {
        visited.insert(url);
    }
    assemble_with_visited(questionnaire, &mut visited, ctx)
}

/// Produce a QuestionnaireResponse for the subject by evaluating the
/// questionnaire's initial values and initial expressions.
pub fn populate_questionnaire(questionnaire: &Value, ctx: &ApplyContext) -> CpgResult<Value> {
    validate_questionnaire(questionnaire)?;
    // `authored` is intentionally omitted: population happens at preview
    // time in a fixed evaluation context with no clock; a hard-coded
    // date would become stale metadata.
    let mut response = json!({
        "resourceType": "QuestionnaireResponse",
        "status": "in-progress",
        "item": [],
    });
    if let Some(canonical) = authored_questionnaire_canonical(questionnaire) {
        response["questionnaire"] = Value::String(canonical);
    }
    response["subject"] = json!({ "reference": ctx.subject });

    let library_canonicals = library_canonicals(questionnaire);
    if let Some(items) = questionnaire_items(questionnaire) {
        response["item"] = Value::Array(
            items
                .iter()
                .map(|item| populate_item(item, questionnaire, &library_canonicals, ctx))
                .collect(),
        );
    }
    Ok(response)
}

/// Perform basic structural checks of a QuestionnaireResponse against its
/// Questionnaire. Full SDC validation in `rh-validator` is intentionally left
/// as a follow-up.
pub fn validate_questionnaire_response(questionnaire: &Value, response: &Value) -> Vec<String> {
    let mut issues = Vec::new();
    let questionnaire_ids = questionnaire_link_ids(questionnaire);
    validate_response_items(
        response_items(response).unwrap_or_default(),
        &questionnaire_ids,
        &mut issues,
    );
    if let Some(items) = questionnaire_items(questionnaire) {
        required_item_issues(items, response, &mut issues);
    }
    issues
}

fn assemble_with_visited(
    questionnaire: &Value,
    visited: &mut HashSet<String>,
    ctx: &ApplyContext,
) -> CpgResult<Value> {
    // An assembled Questionnaire is a transient rendering representation.
    // QuestionnaireResponse.questionnaire must keep the authored root's
    // versioned canonical identity so downstream CQL retrieves match it.
    let authored_canonical = canonical_identity(questionnaire);
    let mut assembled = questionnaire.clone();
    let contained_resources = contained_questionnaires(&assembled);
    if let Some(version) = assembled.get("version").and_then(Value::as_str) {
        assembled["version"] = Value::String(format!("{version}-assembled"));
    } else {
        assembled["version"] = Value::String(Uuid::new_v4().to_string());
    }
    assembled["extension"] = Value::Array(
        assembled
            .get("extension")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(adjust_assemble_expectation)
            .collect(),
    );
    if let Some(canonical) = authored_canonical {
        assembled["extension"]
            .as_array_mut()
            .expect("extensions are always an array here")
            .push(json!({ "url": ASSEMBLED_FROM, "valueCanonical": canonical }));
    }
    if assembled["extension"]
        .as_array()
        .is_some_and(|extensions| extensions.is_empty())
    {
        if let Some(object) = assembled.as_object_mut() {
            object.remove("extension");
        }
    }

    if let Some(items) = questionnaire_items(&assembled) {
        let mut replaced = Vec::new();
        for item in items {
            match assemble_item(item, &contained_resources, visited, ctx) {
                Ok(mut result) => replaced.append(&mut result),
                Err(error) => return Err(error),
            }
        }
        assembled["item"] = Value::Array(replaced);
    }
    if let Some(object) = assembled.as_object_mut() {
        object.remove("contained");
    }
    Ok(assembled)
}

fn assemble_item(
    item: &Value,
    contained: &[Value],
    visited: &mut HashSet<String>,
    ctx: &ApplyContext,
) -> CpgResult<Vec<Value>> {
    let Some(extension) = find_extension(item, SUB_QUESTIONNAIRE) else {
        let mut processed = item.clone();
        process_nested_assembly(&mut processed, contained, visited, ctx)?;
        return Ok(vec![processed]);
    };
    let Some(canonical) = extension_value_canonical(extension).map(str::to_string) else {
        let mut processed = item.clone();
        remove_extension(&mut processed, SUB_QUESTIONNAIRE);
        process_nested_assembly(&mut processed, contained, visited, ctx)?;
        return Ok(vec![processed]);
    };

    if visited.contains(&canonical) {
        return Err(CpgError::InvalidResource(format!(
            "sub-questionnaire cycle detected at {canonical}"
        )));
    }

    let resolved = contained
        .iter()
        .find(|resource| {
            resource.get("resourceType").and_then(Value::as_str) == Some("Questionnaire")
                && canonical_identity(resource).is_some_and(|identity| {
                    // A versioned reference must match the resource identity
                    // exactly; an unversioned reference matches the url alone.
                    identity == canonical
                        || canonical_url(resource).is_some_and(|url| url == canonical)
                })
        })
        .cloned();
    let resolved = match resolved {
        Some(resource) => Some(resource),
        None => ctx.content_resolver.resolve_canonical(&canonical)?,
    };

    let Some(sub_questionnaire) = resolved else {
        let mut processed = item.clone();
        if let Some(object) = processed.as_object_mut() {
            object.insert(
                "text".to_string(),
                Value::String(format!(
                    "Error: unable to resolve sub-questionnaire {canonical}"
                )),
            );
        }
        remove_extension(&mut processed, SUB_QUESTIONNAIRE);
        process_nested_assembly(&mut processed, contained, visited, ctx)?;
        return Ok(vec![processed]);
    };

    if sub_questionnaire
        .get("resourceType")
        .and_then(Value::as_str)
        != Some("Questionnaire")
    {
        let link_id = item
            .get("linkId")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        return Ok(vec![json!({
            "linkId": format!("{link_id}-error"),
            "text": format!(
                "Error: {canonical} does not contain items for substitution"
            ),
        })]);
    }

    visited.insert(canonical.clone());
    // Also track the resolved resource's identity so a reference omitting
    // the version cannot bypass cycle detection against the versioned
    // resource.
    if let Some(identity) = canonical_identity(&sub_questionnaire) {
        visited.insert(identity);
    }
    let assembled_sub = assemble_with_visited(&sub_questionnaire, visited, ctx);
    visited.remove(&canonical);
    if let Some(identity) = canonical_identity(&sub_questionnaire) {
        visited.remove(&identity);
    }
    let assembled_sub = assembled_sub?;
    let Some(items) = questionnaire_items(&assembled_sub) else {
        let link_id = item
            .get("linkId")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        return Ok(vec![json!({
            "linkId": format!("{link_id}-error"),
            "text": format!(
                "Error: {canonical} does not contain items for substitution"
            ),
        })]);
    };
    Ok(items.to_vec())
}

fn process_nested_assembly(
    item: &mut Value,
    contained: &[Value],
    visited: &mut HashSet<String>,
    ctx: &ApplyContext,
) -> CpgResult<()> {
    if let Some(items) = item.get("item").and_then(Value::as_array).cloned() {
        let mut processed = Vec::new();
        for nested in items {
            processed.extend(assemble_item(&nested, contained, visited, ctx)?);
        }
        item["item"] = Value::Array(processed);
    }
    if let Some(actions) = item.get("action").and_then(Value::as_array).cloned() {
        let mut processed = Vec::new();
        for action in actions {
            let mut action = action;
            process_nested_assembly(&mut action, contained, visited, ctx)?;
            processed.push(action);
        }
        item["action"] = Value::Array(processed);
    }
    Ok(())
}

fn adjust_assemble_expectation(extension: Value) -> Option<Value> {
    if extension.get("url").and_then(Value::as_str) != Some(ASSEMBLE_EXPECTATION) {
        return Some(extension);
    }
    let Some(value_code) = extension.get("valueCode").and_then(Value::as_str) else {
        return Some(extension);
    };
    if value_code == "assemble-root" {
        return None;
    }
    if let Some(replaced) = replace_once(value_code, "assemble", "independent") {
        let mut adjusted = extension;
        adjusted["valueCode"] = Value::String(replaced);
        return Some(adjusted);
    }
    if let Some(replaced) = replace_once(value_code, "assembly", "independent") {
        let mut adjusted = extension;
        adjusted["valueCode"] = Value::String(replaced);
        return Some(adjusted);
    }
    Some(extension)
}

fn replace_once(source: &str, from: &str, to: &str) -> Option<String> {
    source
        .split_once(from)
        .map(|(before, after)| format!("{before}{to}{after}"))
}

fn populate_item(
    item: &Value,
    questionnaire: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> Value {
    let mut response_item = Map::new();
    if let Some(link_id) = item.get("linkId").and_then(Value::as_str) {
        response_item.insert("linkId".to_string(), Value::String(link_id.to_string()));
    }
    if let Some(text) = item.get("text") {
        if !text.is_null() {
            response_item.insert("text".to_string(), text.clone());
        }
    }

    let answers = initial_answers(item, questionnaire, library_canonicals, ctx);
    if !answers.is_empty() {
        response_item.insert("answer".to_string(), Value::Array(answers));
    }

    let mut response_item = Value::Object(response_item);
    if let Some(items) = questionnaire_items(item) {
        response_item["item"] = Value::Array(
            items
                .iter()
                .map(|nested| populate_item(nested, questionnaire, library_canonicals, ctx))
                .collect(),
        );
    }
    response_item
}

fn initial_answers(
    item: &Value,
    questionnaire: &Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> Vec<Value> {
    let initial_values = item
        .get("initial")
        .and_then(Value::as_array)
        .map(|initial| {
            initial
                .iter()
                .filter_map(initial_answer)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !initial_values.is_empty() {
        return initial_values;
    }

    let Some(expression) = find_extension(item, INITIAL_EXPRESSION)
        .and_then(|extension| extension.get("valueExpression"))
        .cloned()
    else {
        return Vec::new();
    };
    let Ok(result) = evaluate_expression(&expression, questionnaire, library_canonicals, ctx)
    else {
        return Vec::new();
    };
    match expression_answer(&result) {
        Some(answer) => vec![answer],
        None => Vec::new(),
    }
}

fn initial_answer(initial: &Value) -> Option<Value> {
    let object = initial.as_object()?;
    object
        .iter()
        .find(|(key, _)| key.starts_with("value"))
        .map(|(key, value)| json!({ key.clone(): value.clone() }))
}

fn expression_answer(value: &Value) -> Option<Value> {
    match value {
        Value::Bool(boolean) => Some(json!({ "valueBoolean": boolean })),
        Value::Number(number) => {
            if number.is_u64() || number.is_i64() {
                Some(json!({ "valueInteger": number.clone() }))
            } else {
                Some(json!({ "valueDecimal": number.clone() }))
            }
        }
        Value::String(string) => Some(json!({ "valueString": string.clone() })),
        Value::Object(object) => {
            if object.contains_key("code") || object.contains_key("system") {
                Some(json!({ "valueCoding": value.clone() }))
            } else {
                Some(json!({ "valueString": value.to_string() }))
            }
        }
        Value::Array(_) | Value::Null => None,
    }
}

fn library_canonicals(questionnaire: &Value) -> Vec<String> {
    let mut libraries = Vec::new();
    for library in questionnaire
        .get("library")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        libraries.push(library.to_string());
    }
    for extension in questionnaire
        .get("extension")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|extension| extension.get("url").and_then(Value::as_str) == Some(CQF_LIBRARY))
    {
        if let Some(canonical) = extension_value_canonical(extension) {
            libraries.push(canonical.to_string());
        }
    }
    libraries
}

fn validate_response_items(
    items: &[Value],
    questionnaire_ids: &HashSet<String>,
    issues: &mut Vec<String>,
) {
    let mut seen = HashSet::new();
    for item in items {
        let Some(link_id) = item.get("linkId").and_then(Value::as_str) else {
            continue;
        };
        if !questionnaire_ids.contains(link_id) && !seen.contains(link_id) {
            issues.push(format!(
                "Response item {link_id} does not match any questionnaire item"
            ));
        }
        if !seen.insert(link_id.to_string()) {
            issues.push(format!("Duplicate response item linkId {link_id}"));
        }
    }
    for item in items {
        if let Some(nested) = response_items(item) {
            validate_response_items(nested, questionnaire_ids, issues);
        }
    }
}

fn validate_questionnaire(questionnaire: &Value) -> CpgResult<()> {
    if questionnaire.get("resourceType").and_then(Value::as_str) == Some("Questionnaire") {
        Ok(())
    } else {
        Err(CpgError::InvalidResource(
            "resource must be a FHIR Questionnaire".to_string(),
        ))
    }
}

fn questionnaire_link_ids(questionnaire: &Value) -> HashSet<String> {
    let mut link_ids = HashSet::new();
    collect_link_ids(
        questionnaire
            .get("item")
            .and_then(Value::as_array)
            .map(Vec::as_slice),
        &mut link_ids,
    );
    link_ids
}

fn collect_link_ids(items: Option<&[Value]>, link_ids: &mut HashSet<String>) {
    for item in items.into_iter().flatten() {
        if let Some(link_id) = item.get("linkId").and_then(Value::as_str) {
            link_ids.insert(link_id.to_string());
        }
        collect_link_ids(
            item.get("item")
                .and_then(Value::as_array)
                .map(Vec::as_slice),
            link_ids,
        );
    }
}

fn required_item_issues(items: &[Value], response: &Value, issues: &mut Vec<String>) {
    let required_response = response_items(response).unwrap_or_default();
    for item in items {
        let is_required = item
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let link_id = item.get("linkId").and_then(Value::as_str);
        let matching_response = link_id.and_then(|link_id| {
            required_response.iter().find(|response_item| {
                response_item.get("linkId").and_then(Value::as_str) == Some(link_id)
            })
        });
        if is_required {
            if let Some(link_id) = link_id {
                let item_type = item.get("type").and_then(Value::as_str);
                // Groups (and displays) are "answered" by their nested
                // response items; questions require non-empty answers.
                let answered = if item_type == Some("group") || item_type == Some("display") {
                    matching_response
                        .and_then(|response_item| response_items(response_item))
                        .is_some_and(|nested| !nested.is_empty())
                } else {
                    matching_response
                        .and_then(|response_item| {
                            response_item.get("answer").and_then(Value::as_array)
                        })
                        .is_some_and(|answers| !answers.is_empty())
                };
                if !answered {
                    issues.push(format!("Item {link_id} is required but has no answer"));
                }
            }
        }
        if let Some(nested) = item.get("item").and_then(Value::as_array) {
            // Recurse with the response item matching this linkId so nested
            // required questions are judged against their own answers
            // rather than the root response.
            let nested_response = matching_response.cloned().unwrap_or(Value::Null);
            required_item_issues(nested, &nested_response, issues);
        }
    }
}

fn questionnaire_items(value: &Value) -> Option<&[Value]> {
    value
        .get("item")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
}

fn response_items(value: &Value) -> Option<&[Value]> {
    value
        .get("item")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
}

fn contained_questionnaires(questionnaire: &Value) -> Vec<Value> {
    questionnaire
        .get("contained")
        .and_then(Value::as_array)
        .map(|contained| contained.to_vec())
        .unwrap_or_default()
}

fn canonical_url(resource: &Value) -> Option<String> {
    resource
        .get("url")
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// The versioned canonical identity of a resource (`url|version`), used
/// for sub-questionnaire matching and cycle detection.
fn canonical_identity(resource: &Value) -> Option<String> {
    let url = canonical_url(resource)?;
    Some(match resource.get("version").and_then(Value::as_str) {
        Some(version) => format!("{url}|{version}"),
        None => url,
    })
}

/// Return the authored Questionnaire identity for a response. SDC assembly
/// records that identity in `assembledFrom`; an ordinary Questionnaire uses
/// its own versioned canonical identity.
fn authored_questionnaire_canonical(questionnaire: &Value) -> Option<String> {
    find_extension(questionnaire, ASSEMBLED_FROM)
        .and_then(extension_value_canonical)
        .map(str::to_string)
        .or_else(|| canonical_identity(questionnaire))
}

fn find_extension<'value>(value: &'value Value, url: &str) -> Option<&'value Value> {
    value
        .get("extension")
        .and_then(Value::as_array)?
        .iter()
        .find(|extension| extension.get("url").and_then(Value::as_str) == Some(url))
}

fn extension_value_canonical(extension: &Value) -> Option<&str> {
    extension.get("valueCanonical").and_then(Value::as_str)
}

fn remove_extension(value: &mut Value, url: &str) {
    let Some(object) = value.as_object_mut() else {
        return;
    };
    let Some(extensions) = object.get_mut("extension").and_then(Value::as_array_mut) else {
        return;
    };
    extensions.retain(|extension| extension.get("url").and_then(Value::as_str) != Some(url));
    if extensions.is_empty() {
        object.remove("extension");
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde_json::json;

    use super::*;
    use crate::context::ApplyContext;
    use crate::resolver::BundleResolver;

    fn context(bundle: Value) -> ApplyContext {
        ApplyContext::new(
            Arc::new(BundleResolver::new(&bundle).unwrap()),
            "Patient/123",
        )
    }

    #[test]
    fn assembles_sub_questionnaire_from_bundle() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "url": "http://example.org/root",
            "version": "1.0",
            "item": [
                { "linkId": "a", "type": "string", "text": "Normal" },
                {
                    "linkId": "b",
                    "type": "display",
                    "extension": [{
                        "url": SUB_QUESTIONNAIRE,
                        "valueCanonical": "http://example.org/sub"
                    }]
                }
            ],
            "contained": [{
                "resourceType": "Questionnaire",
                "url": "http://example.org/sub",
                "version": "1.0",
                "item": [
                    { "linkId": "s1", "type": "boolean", "text": "Sub one" },
                    { "linkId": "s2", "type": "string", "text": "Sub two" }
                ]
            }]
        });
        let ctx = context(json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": []
        }));

        let assembled =
            assemble_questionnaire(&questionnaire, &ctx).expect("assembly should succeed");
        let items = questionnaire_items(&assembled).expect("assembled items");
        assert_eq!(items.len(), 3);
        assert_eq!(items[1]["linkId"], json!("s1"));
        assert_eq!(items[2]["linkId"], json!("s2"));
        assert_eq!(assembled["version"], json!("1.0-assembled"));
        assert!(assembled.get("contained").is_none());
        assert!(assembled["extension"]
            .as_array()
            .unwrap()
            .iter()
            .any(|extension| {
                extension["url"] == json!(ASSEMBLED_FROM)
                    && extension["valueCanonical"] == json!("http://example.org/root|1.0")
            }));
    }

    #[test]
    fn populated_response_preserves_authored_canonical_after_assembly() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "url": "http://example.org/Questionnaire/root",
            "version": "0.2.0",
            "item": [{ "linkId": "screen", "type": "boolean" }]
        });
        let ctx = context(json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": []
        }));

        let assembled =
            assemble_questionnaire(&questionnaire, &ctx).expect("assembly should succeed");
        assert_eq!(assembled["version"], json!("0.2.0-assembled"));
        let response = populate_questionnaire(&assembled, &ctx).expect("population should succeed");
        assert_eq!(
            response["questionnaire"],
            json!("http://example.org/Questionnaire/root|0.2.0")
        );
    }

    #[test]
    fn assemble_reports_sub_questionnaire_cycle() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "url": "http://example.org/first",
            "item": [{
                "linkId": "first",
                "type": "display",
                "extension": [{
                    "url": SUB_QUESTIONNAIRE,
                    "valueCanonical": "http://example.org/second"
                }]
            }]
        });
        let bundle = json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": [
                { "resource": questionnaire },
                {
                    "resource": {
                        "resourceType": "Questionnaire",
                        "url": "http://example.org/second",
                        "item": [{
                            "linkId": "second",
                            "type": "display",
                            "extension": [{
                                "url": SUB_QUESTIONNAIRE,
                                "valueCanonical": "http://example.org/first"
                            }]
                        }]
                    }
                }
            ]
        });
        let ctx = context(bundle);

        let error = assemble_questionnaire(&questionnaire, &ctx)
            .expect_err("a sub-questionnaire cycle should fail");
        assert!(error.to_string().contains("cycle detected"));
    }

    #[test]
    fn assembles_unresolvable_sub_questionnaire_with_error_text() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "item": [{
                "linkId": "missing",
                "type": "display",
                "extension": [{
                    "url": SUB_QUESTIONNAIRE,
                    "valueCanonical": "http://example.org/missing"
                }]
            }]
        });
        let ctx = context(json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": []
        }));

        let assembled =
            assemble_questionnaire(&questionnaire, &ctx).expect("assembly should succeed");
        let item = &questionnaire_items(&assembled).unwrap()[0];
        assert_eq!(
            item["text"],
            json!("Error: unable to resolve sub-questionnaire http://example.org/missing")
        );
        assert!(item.get("extension").is_none());
    }

    #[test]
    fn populates_initial_values_and_fhirpath_expressions() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "url": "http://example.org/populate",
            "version": "1.0",
            "item": [
                {
                    "linkId": "q1",
                    "text": "Name",
                    "type": "string",
                    "initial": [{ "valueString": "Alice" }]
                },
                {
                    "linkId": "q2",
                    "text": "Version",
                    "type": "string",
                    "extension": [{
                        "url": INITIAL_EXPRESSION,
                        "valueExpression": {
                            "language": "text/fhirpath",
                            "expression": "version"
                        }
                    }]
                },
                { "linkId": "q3", "text": "Empty", "type": "string" }
            ]
        });
        let ctx = context(json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": []
        }));

        let response =
            populate_questionnaire(&questionnaire, &ctx).expect("population should succeed");
        let items = response_items(&response).expect("response items");
        assert_eq!(items[0]["answer"][0]["valueString"], json!("Alice"));
        assert_eq!(items[1]["answer"][0]["valueString"], json!("1.0"));
        assert!(items[2].get("answer").is_none());
        assert_eq!(items[2]["linkId"], json!("q3"));
        assert_eq!(items[2]["text"], json!("Empty"));
        assert_eq!(
            response["questionnaire"],
            json!("http://example.org/populate|1.0")
        );
        assert_eq!(response["subject"], json!({ "reference": "Patient/123" }));
    }

    #[test]
    fn validates_missing_required_unknown_and_clean_responses() {
        let questionnaire = json!({
            "resourceType": "Questionnaire",
            "item": [
                { "linkId": "required", "type": "string", "required": true },
                { "linkId": "optional", "type": "string" }
            ]
        });
        let response = json!({
            "resourceType": "QuestionnaireResponse",
            "item": [
                { "linkId": "unknown" },
                { "linkId": "unknown" }
            ]
        });
        let issues = validate_questionnaire_response(&questionnaire, &response);
        assert_eq!(
            issues,
            vec![
                "Response item unknown does not match any questionnaire item".to_string(),
                "Duplicate response item linkId unknown".to_string(),
                "Item required is required but has no answer".to_string(),
            ]
        );

        let clean = json!({
            "resourceType": "QuestionnaireResponse",
            "item": [
                { "linkId": "required", "answer": [{ "valueString": "complete" }] },
                { "linkId": "optional", "item": [] }
            ]
        });
        assert!(validate_questionnaire_response(&questionnaire, &clean).is_empty());
    }
}
