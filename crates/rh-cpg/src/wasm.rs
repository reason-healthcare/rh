use std::sync::Arc;

use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

use crate::apply::activity_definition::apply_activity_definition as apply_activity_definition_native;
use crate::apply::plan_definition::apply_plan_definition as apply_plan_definition_native;
use crate::context::ApplyContext;
use crate::measure::evaluate_measure as evaluate_measure_native;
use crate::questionnaire::{
    assemble_questionnaire as assemble_questionnaire_native,
    populate_questionnaire as populate_questionnaire_native,
    validate_questionnaire_response as validate_questionnaire_response_native,
};
use crate::resolver::{as_content_bundle, BundleResolver};

pub use rh_foundation::wasm::WasmResult;

#[wasm_bindgen(start)]
pub fn init() {
    rh_foundation::wasm::init_panic_hook();
}

fn parse_content_bundle(json: &str) -> Result<Value, WasmResult> {
    let value = parse_json(json, "content")?;
    as_content_bundle(value).map_err(|error| WasmResult::err(error.to_string()))
}

fn parse_json(json: &str, description: &str) -> Result<Value, WasmResult> {
    serde_json::from_str(json)
        .map_err(|error| WasmResult::err(format!("Failed to parse {description} JSON: {error}")))
}

/// Evaluate a Measure. Returns WasmResult whose data is a MeasureReport JSON.
#[wasm_bindgen]
pub fn evaluate_measure(
    measure: &str,
    subject: &str,
    content_bundle: &str,
    data_bundle: Option<String>,
    encounter: Option<String>,
    practitioner: Option<String>,
    organization: Option<String>,
) -> WasmResult {
    let measure = match parse_json(measure, "Measure") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let content_bundle = match parse_content_bundle(content_bundle) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let context = match apply_context(
        content_bundle,
        subject,
        data_bundle,
        encounter,
        practitioner,
        organization,
    ) {
        Ok(value) => value,
        Err(error) => return error,
    };

    match evaluate_measure_native(&measure, &context) {
        Ok(result) => match serialize_json(&result) {
            Ok(json) => WasmResult::ok(json),
            Err(error) => error,
        },
        Err(error) => WasmResult::err(error.to_string()),
    }
}

/// Assemble sub-questionnaires. Returns WasmResult whose data is the assembled
/// Questionnaire JSON.
#[wasm_bindgen]
pub fn assemble_questionnaire(questionnaire: &str, content_bundle: &str) -> WasmResult {
    let questionnaire = match parse_json(questionnaire, "Questionnaire") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let content_bundle = match parse_content_bundle(content_bundle) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let resolver = match BundleResolver::new(&content_bundle)
        .map_err(|error| WasmResult::err(error.to_string()))
    {
        Ok(value) => value,
        Err(error) => return error,
    };
    let context = ApplyContext::new(Arc::new(resolver), "Patient/example");

    match assemble_questionnaire_native(&questionnaire, &context) {
        Ok(result) => match serialize_json(&result) {
            Ok(json) => WasmResult::ok(json),
            Err(error) => error,
        },
        Err(error) => WasmResult::err(error.to_string()),
    }
}

/// Populate a Questionnaire for a subject. Returns WasmResult whose data is a
/// QuestionnaireResponse JSON.
#[wasm_bindgen]
pub fn populate_questionnaire(
    questionnaire: &str,
    subject: &str,
    content_bundle: &str,
    data_bundle: Option<String>,
    encounter: Option<String>,
    practitioner: Option<String>,
    organization: Option<String>,
) -> WasmResult {
    let questionnaire = match parse_json(questionnaire, "Questionnaire") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let content_bundle = match parse_content_bundle(content_bundle) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let context = match apply_context(
        content_bundle,
        subject,
        data_bundle,
        encounter,
        practitioner,
        organization,
    ) {
        Ok(value) => value,
        Err(error) => return error,
    };

    match populate_questionnaire_native(&questionnaire, &context) {
        Ok(result) => match serialize_json(&result) {
            Ok(json) => WasmResult::ok(json),
            Err(error) => error,
        },
        Err(error) => WasmResult::err(error.to_string()),
    }
}

/// Validate a QuestionnaireResponse. Returns WasmResult whose data is an
/// issues object JSON; success is true even when issues are present.
#[wasm_bindgen]
pub fn validate_questionnaire_response(questionnaire: &str, response: &str) -> WasmResult {
    let questionnaire = match parse_json(questionnaire, "Questionnaire") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let response = match parse_json(response, "QuestionnaireResponse") {
        Ok(value) => value,
        Err(error) => return error,
    };

    let issues = validate_questionnaire_response_native(&questionnaire, &response);
    match serde_json::to_string(&json!({ "issues": issues })) {
        Ok(json) => WasmResult::ok(json),
        Err(error) => WasmResult::err(format!("Failed to serialize result to JSON: {error}")),
    }
}

fn apply_context(
    content_bundle: Value,
    subject: &str,
    data_bundle: Option<String>,
    encounter: Option<String>,
    practitioner: Option<String>,
    organization: Option<String>,
) -> Result<ApplyContext, WasmResult> {
    let resolver =
        BundleResolver::new(&content_bundle).map_err(|error| WasmResult::err(error.to_string()))?;
    let mut context = ApplyContext::new(Arc::new(resolver), subject);
    context.data = data_bundle.map_or_else(
        || Ok(None),
        |json| parse_json(&json, "data bundle").map(Some),
    )?;
    context.encounter = encounter;
    context.practitioner = practitioner;
    context.organization = organization;
    Ok(context)
}

fn serialize_json(value: &Value) -> Result<String, WasmResult> {
    serde_json::to_string(value)
        .map_err(|error| WasmResult::err(format!("Failed to serialize result to JSON: {error}")))
}

/// Apply a PlanDefinition ($apply). Returns WasmResult whose data is the result Bundle JSON.
#[wasm_bindgen]
pub fn apply_plan_definition(
    plan_definition: &str,
    subject: &str,
    content_bundle: &str,
    data_bundle: Option<String>,
    encounter: Option<String>,
    practitioner: Option<String>,
    organization: Option<String>,
) -> WasmResult {
    let plan_definition = match parse_json(plan_definition, "PlanDefinition") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let content_bundle = match parse_json(content_bundle, "content") {
        Ok(value) => match as_content_bundle(value) {
            Ok(value) => value,
            Err(error) => return WasmResult::err(error.to_string()),
        },
        Err(error) => return error,
    };
    let context = match apply_context(
        content_bundle,
        subject,
        data_bundle,
        encounter,
        practitioner,
        organization,
    ) {
        Ok(value) => value,
        Err(error) => return error,
    };

    match apply_plan_definition_native(&plan_definition, &context) {
        Ok(result) => match serialize_json(&result) {
            Ok(json) => WasmResult::ok(json),
            Err(error) => error,
        },
        Err(error) => WasmResult::err(error.to_string()),
    }
}

/// Apply an ActivityDefinition. Returns WasmResult whose data is the target resource JSON (or null).
#[wasm_bindgen]
pub fn apply_activity_definition(
    activity_definition: &str,
    subject: &str,
    content_bundle: &str,
    data_bundle: Option<String>,
    encounter: Option<String>,
    practitioner: Option<String>,
    organization: Option<String>,
) -> WasmResult {
    let activity_definition = match parse_json(activity_definition, "ActivityDefinition") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let content_bundle = match parse_json(content_bundle, "content") {
        Ok(value) => match as_content_bundle(value) {
            Ok(value) => value,
            Err(error) => return WasmResult::err(error.to_string()),
        },
        Err(error) => return error,
    };
    let context = match apply_context(
        content_bundle,
        subject,
        data_bundle,
        encounter,
        practitioner,
        organization,
    ) {
        Ok(value) => value,
        Err(error) => return error,
    };

    let library_canonicals: Vec<String> = activity_definition
        .get("library")
        .and_then(Value::as_array)
        .map(|libraries| {
            libraries
                .iter()
                .filter_map(|canonical| canonical.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    match apply_activity_definition_native(&activity_definition, &library_canonicals, &context) {
        Ok(Some(result)) => match serialize_json(&result) {
            Ok(json) => WasmResult::ok(json),
            Err(error) => error,
        },
        Ok(None) => WasmResult::ok("null".to_string()),
        Err(error) => WasmResult::err(error.to_string()),
    }
}
