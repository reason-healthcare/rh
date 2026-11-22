//! WASM bindings for FHIRPath parsing and evaluation
//!
//! This module provides WebAssembly bindings for FHIRPath functionality,
//! exposing parse and evaluate functions for use in web browsers and Node.js.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{EvaluationContext, FhirPathEvaluator, FhirPathParser};

// Re-export WasmResult from foundation
pub use rh_foundation::wasm::WasmResult;

// Initialize panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn init() {
    rh_foundation::wasm::init_panic_hook();
}

/// Parse options for FHIRPath parsing
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ParseOptions {
    format: String,
}

#[wasm_bindgen]
impl ParseOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            format: "json".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }
}

/// Evaluation options for FHIRPath evaluation
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct EvaluateOptions {
    format: String,
    #[serde(default)]
    fhir_version: Option<String>,
}

#[wasm_bindgen]
impl EvaluateOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            format: "json".to_string(),
            fhir_version: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }

    #[wasm_bindgen(getter)]
    pub fn fhir_version(&self) -> String {
        self.fhir_version.as_deref().unwrap_or("r4").to_owned()
    }

    #[wasm_bindgen(setter)]
    pub fn set_fhir_version(&mut self, version: String) {
        self.fhir_version = Some(version);
    }
}

/// Parse a FHIRPath expression and return the AST
#[wasm_bindgen]
pub fn parse_fhirpath_expression(expression: &str, options: &ParseOptions) -> WasmResult {
    let parser = FhirPathParser::new();

    match parser.parse(expression) {
        Ok(ast) => match options.format.as_str() {
            "pretty" => {
                let pretty_output =
                    format!("✅ FHIRPath Expression parsed successfully:\n{ast:#?}");
                WasmResult::ok(pretty_output)
            }
            "json" => match serde_json::to_string_pretty(&ast) {
                Ok(json) => WasmResult::ok(json),
                Err(e) => WasmResult::err(format!("Failed to serialize AST to JSON: {e}")),
            },
            "debug" => {
                let debug_output = format!("{ast:#?}");
                WasmResult::ok(debug_output)
            }
            _ => WasmResult::err(format!(
                "Invalid format: '{}'. Use 'pretty', 'json', or 'debug'",
                options.format
            )),
        },
        Err(e) => WasmResult::err(format!("Failed to parse FHIRPath expression: {e}")),
    }
}

/// Evaluate a FHIRPath expression against a FHIR resource
#[wasm_bindgen]
pub fn evaluate_fhirpath_expression(
    expression: &str,
    resource_json: &str,
    options: &EvaluateOptions,
) -> WasmResult {
    // Parse the resource JSON
    let resource = match serde_json::from_str(resource_json) {
        Ok(resource) => resource,
        Err(e) => return WasmResult::err(format!("Failed to parse resource JSON: {e}")),
    };

    // Parse the FHIRPath expression
    let parser = FhirPathParser::new();
    let ast = match parser.parse(expression) {
        Ok(ast) => ast,
        Err(e) => return WasmResult::err(format!("Failed to parse FHIRPath expression: {e}")),
    };

    // Evaluate the expression
    let evaluator = FhirPathEvaluator::new();
    let mut context = EvaluationContext::new(resource);
    if let Some(version) = &options.fhir_version {
        match version.parse() {
            Ok(model) => context.model = model,
            Err(error) => return WasmResult::err(error.to_string()),
        }
    }

    let result = match evaluator.evaluate(&ast, &context) {
        Ok(result) => result,
        Err(e) => return WasmResult::err(format!("Failed to evaluate FHIRPath expression: {e}")),
    };

    // Get trace logs if any
    let trace_logs = context.get_trace_logs();
    let trace_output: Vec<serde_json::Value> = trace_logs
        .iter()
        .map(|log| {
            serde_json::json!({
                "name": log.name,
                "value": log.value
            })
        })
        .collect();

    // Convert result to a more user-friendly format
    let result_json = match result {
        crate::FhirPathValue::Collection(ref items)
        | crate::FhirPathValue::UnorderedCollection(ref items)
            if items.is_empty() =>
        {
            serde_json::json!({
                "result": [],
                "count": 0,
                "type": "empty_collection",
                "trace": trace_output
            })
        }
        crate::FhirPathValue::Collection(ref items)
        | crate::FhirPathValue::UnorderedCollection(ref items) => {
            let json_items: Vec<serde_json::Value> =
                items.iter().map(|item| item.to_json()).collect();
            serde_json::json!({
                "result": json_items,
                "count": items.len(),
                "type": "collection",
                "trace": trace_output
            })
        }
        crate::FhirPathValue::Empty => {
            serde_json::json!({
                "result": null,
                "count": 0,
                "type": "empty",
                "trace": trace_output
            })
        }
        ref single_value => {
            let json_value = single_value.to_json();
            serde_json::json!({
                "result": json_value,
                "count": 1,
                "type": "single_value",
                "trace": trace_output
            })
        }
    };

    // Format output
    match options.format.as_str() {
        "json" => match serde_json::to_string_pretty(&result_json) {
            Ok(json) => WasmResult::ok(json),
            Err(e) => WasmResult::err(format!("Failed to serialize result to JSON: {e}")),
        },
        "pretty" => {
            let json = match serde_json::to_string_pretty(&result_json) {
                Ok(json) => json,
                Err(e) => {
                    return WasmResult::err(format!("Failed to serialize result to JSON: {e}"))
                }
            };
            let pretty_output = format!(
                "✅ FHIRPath Evaluation successful:\n\nExpression:\n  {expression}\n\nResult:\n{json}"
            );
            WasmResult::ok(pretty_output)
        }
        _ => WasmResult::err(format!(
            "Invalid format: '{}'. Use 'json' or 'pretty'",
            options.format
        )),
    }
}

/// Convenience function to parse FHIRPath with default options
#[wasm_bindgen]
pub fn parse_fhirpath(expression: &str) -> WasmResult {
    let options = ParseOptions::new();
    parse_fhirpath_expression(expression, &options)
}

/// Convenience function to evaluate FHIRPath with default options
#[wasm_bindgen]
pub fn evaluate_fhirpath(expression: &str, resource_json: &str) -> WasmResult {
    let options = EvaluateOptions::new();
    evaluate_fhirpath_expression(expression, resource_json, &options)
}

/// Get the version of the FHIRPath library
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Validate a FHIRPath expression (parse-only, no evaluation)
#[wasm_bindgen]
pub fn validate_fhirpath_expression(expression: &str) -> WasmResult {
    let parser = FhirPathParser::new();

    match parser.parse(expression) {
        Ok(_) => WasmResult::ok(
            serde_json::json!({"valid": true, "message": "Valid FHIRPath expression"}).to_string(),
        ),
        Err(e) => WasmResult::err(format!("Invalid FHIRPath expression: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn evaluates_typed_r5_values_and_preserves_decimal_output() {
        let mut options = EvaluateOptions::new();
        options.set_fhir_version("r5".into());
        let resource = r#"{"resourceType":"Parameters","parameter":[{"name":"count","valueInteger64":"9223372036854775807"}]}"#;
        let result = evaluate_fhirpath_expression(
            "parameter.value.getValue() is System.Long",
            resource,
            &options,
        );
        assert!(result.success, "{:?}", result.error());
        let output: serde_json::Value = serde_json::from_str(&result.data().unwrap()).unwrap();
        assert_eq!(output["result"], serde_json::json!(true));
        let result = evaluate_fhirpath_expression("1.234567890123456789", "null", &options);
        assert!(result.success, "{:?}", result.error());
        let payload = result.data().unwrap();
        assert!(payload.contains("1.234567890123456789"), "{payload}");
    }
}
