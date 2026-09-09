use std::sync::Arc;

use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::apply::activity_definition::apply_activity_definition as apply_activity_definition_native;
use crate::apply::plan_definition::apply_plan_definition as apply_plan_definition_native;
use crate::context::ApplyContext;
use crate::resolver::{as_content_bundle, BundleResolver};

pub use rh_foundation::wasm::WasmResult;

#[wasm_bindgen(start)]
pub fn init() {
    rh_foundation::wasm::init_panic_hook();
}

fn parse_json(json: &str, description: &str) -> Result<Value, WasmResult> {
    serde_json::from_str(json)
        .map_err(|error| WasmResult::err(format!("Failed to parse {description} JSON: {error}")))
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
