//! WASM bindings for VCL parsing and translation
//!
//! This module provides WebAssembly bindings for VCL functionality,
//! exposing parse and translate functions with similar parameters to the CLI.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{parse_vcl, VclTranslator};

// Initialize panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Result type for WASM operations
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct WasmResult {
    success: bool,
    data: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen]
impl WasmResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl WasmResult {
    fn ok(data: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Parse options for VCL parsing
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

/// Translation options for VCL to FHIR translation
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct TranslateOptions {
    format: String,
    default_system: Option<String>,
}

#[wasm_bindgen]
impl TranslateOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            format: "json".to_string(),
            default_system: None,
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
    pub fn default_system(&self) -> Option<String> {
        self.default_system.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_system(&mut self, system: Option<String>) {
        self.default_system = system;
    }
}

/// Parse a VCL expression and return the AST
#[wasm_bindgen]
pub fn parse_vcl_expression(expression: &str, options: &ParseOptions) -> WasmResult {
    match parse_vcl(expression) {
        Ok(ast) => match options.format.as_str() {
            "pretty" => {
                // For pretty format, we'll convert to a readable string representation
                let pretty_output = format!("✅ VCL Expression parsed successfully:\n{ast:#?}");
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
        Err(e) => WasmResult::err(format!("Failed to parse VCL expression: {e}")),
    }
}

/// Translate a VCL expression to FHIR ValueSet.compose
#[wasm_bindgen]
pub fn translate_vcl_expression(expression: &str, options: &TranslateOptions) -> WasmResult {
    // First parse the expression
    let ast = match parse_vcl(expression) {
        Ok(ast) => ast,
        Err(e) => return WasmResult::err(format!("Failed to parse VCL expression: {e}")),
    };

    // Create translator with optional default system
    let translator = if let Some(ref system) = options.default_system {
        VclTranslator::with_default_system(system.clone())
    } else {
        VclTranslator::new()
    };

    // Translate to FHIR
    let fhir_compose = match translator.translate(&ast) {
        Ok(compose) => compose,
        Err(e) => return WasmResult::err(format!("Failed to translate to FHIR: {e}")),
    };

    // Format output
    match options.format.as_str() {
        "json" => match serde_json::to_string_pretty(&fhir_compose) {
            Ok(json) => WasmResult::ok(json),
            Err(e) => WasmResult::err(format!("Failed to serialize FHIR compose to JSON: {e}")),
        },
        "pretty" => {
            let json = match serde_json::to_string_pretty(&fhir_compose) {
                Ok(json) => json,
                Err(e) => return WasmResult::err(format!("Failed to serialize FHIR compose to JSON: {e}")),
            };
            let pretty_output = format!(
                "✅ VCL Translation successful:\n\nOriginal VCL:\n  {expression}\n\nFHIR ValueSet.compose:\n{json}"
            );
            WasmResult::ok(pretty_output)
        }
        _ => WasmResult::err(format!(
            "Invalid format: '{}'. Use 'json' or 'pretty'",
            options.format
        )),
    }
}

/// Convenience function to parse VCL with default options
#[wasm_bindgen]
pub fn parse_vcl_simple(expression: &str) -> WasmResult {
    let options = ParseOptions::new();
    parse_vcl_expression(expression, &options)
}

/// Convenience function to translate VCL with default options
#[wasm_bindgen]
pub fn translate_vcl_simple(expression: &str) -> WasmResult {
    let options = TranslateOptions::new();
    translate_vcl_expression(expression, &options)
}

/// Convenience function to translate VCL with a default system
#[wasm_bindgen]
pub fn translate_vcl_with_system(expression: &str, default_system: &str) -> WasmResult {
    let mut options = TranslateOptions::new();
    options.set_default_system(Some(default_system.to_string()));
    translate_vcl_expression(expression, &options)
}

/// Get the version of the VCL library
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Validate a VCL expression (parse-only, no translation)
#[wasm_bindgen]
pub fn validate_vcl_expression(expression: &str) -> WasmResult {
    match parse_vcl(expression) {
        Ok(_) => WasmResult::ok(serde_json::json!({"valid": true, "message": "Valid VCL expression"}).to_string()),
        Err(e) => WasmResult::err(format!("Invalid VCL expression: {e}")),
    }
}