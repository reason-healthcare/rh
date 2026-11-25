//! WASM bindings for FHIRPath parsing and evaluation
//!
//! This module provides WebAssembly bindings for FHIRPath functionality,
//! exposing parse and evaluate functions for use in web browsers and Node.js.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};

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
    let mut context = match EvaluationContext::from_json_str(resource_json) {
        Ok(context) => context,
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

    let (count, result_type) = match &result {
        FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
            if items.is_empty() {
                (0, "empty_collection")
            } else {
                (items.len(), "collection")
            }
        }
        FhirPathValue::Empty => (0, "empty"),
        _ => (1, "single_value"),
    };
    let result_json = EvaluationOutput {
        count,
        result: result.as_json(),
        trace: trace_output,
        result_type,
    };

    if !matches!(options.format.as_str(), "json" | "pretty") {
        return WasmResult::err(format!(
            "Invalid format: '{}'. Use 'json' or 'pretty'",
            options.format
        ));
    }
    let json = match serde_json::to_string_pretty(&result_json) {
        Ok(json) => json,
        Err(e) => return WasmResult::err(format!("Failed to serialize result to JSON: {e}")),
    };
    if options.format == "pretty" {
        WasmResult::ok(format!(
            "✅ FHIRPath Evaluation successful:\n\nExpression:\n  {expression}\n\nResult:\n{json}"
        ))
    } else {
        WasmResult::ok(json)
    }
}

#[derive(Serialize)]
struct EvaluationOutput<T> {
    count: usize,
    result: T,
    trace: Vec<serde_json::Value>,
    #[serde(rename = "type")]
    result_type: &'static str,
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
    use serde_json::value::RawValue;
    use std::collections::BTreeMap;
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

    #[wasm_bindgen_test]
    fn preserves_exact_numbers_in_resource_and_result_json() {
        let resource = r#"{"resourceType":"Parameters","parameter":[
            {"name":"exact","valueDecimal":1.23456789012345678900},
            {"name":"small","valueDecimal":1e-999999999},
            {"name":"large","valueQuantity":{"value":1e999999999,"unit":"kg"}}
        ]}"#;
        let result = evaluate_fhirpath("parameter.value", resource);
        assert!(result.success, "{:?}", result.error());

        let output: BTreeMap<String, Box<RawValue>> =
            serde_json::from_str(&result.data().unwrap()).unwrap();
        assert_eq!(output["count"].get(), "3");
        assert_eq!(output["type"].get(), r#""collection""#);
        assert_eq!(output["trace"].get(), "[]");
        let values: Vec<Box<RawValue>> = serde_json::from_str(output["result"].get()).unwrap();
        assert_eq!(values[0].get(), "1.23456789012345678900");
        assert_eq!(values[1].get(), "1e-999999999");
        let quantity: BTreeMap<String, Box<RawValue>> =
            serde_json::from_str(values[2].get()).unwrap();
        assert_eq!(quantity["value"].get(), "1e999999999");
    }
}
