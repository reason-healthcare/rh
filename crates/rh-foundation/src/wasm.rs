//! WebAssembly utilities (requires `wasm` feature).
//!
//! This module provides common utilities for WebAssembly bindings,
//! including panic hook initialization, result wrapper types, and
//! JSON conversion helpers.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in WASM.
///
/// Call this function once at the start of your WASM module
/// to enable detailed panic messages in the browser console.
///
/// # Example
/// ```no_run
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen(start)]
/// pub fn init() {
///     rh_foundation::wasm::init_panic_hook();
/// }
/// ```
#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// A no-op version for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub fn init_panic_hook() {
    // No-op on non-WASM targets
}

/// Result type for WASM operations.
///
/// This type is designed to be easily used with wasm-bindgen,
/// providing JavaScript-friendly success/error handling.
///
/// # Example
/// ```no_run
/// use rh_foundation::wasm::WasmResult;
///
/// let result = WasmResult::ok("Success!".to_string());
/// assert!(result.success);
/// assert_eq!(result.data, Some("Success!".to_string()));
/// ```
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmResult {
    #[wasm_bindgen(readonly)]
    pub success: bool,
    data: Option<String>,
    error: Option<String>,
}

#[wasm_bindgen]
impl WasmResult {
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
    /// Create a successful result with data.
    pub fn ok(data: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error result with an error message.
    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }

    /// Create a result from a standard Result type.
    pub fn from_result<E: std::fmt::Display>(result: Result<String, E>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(e) => Self::err(e.to_string()),
        }
    }

    /// Convert to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Default for WasmResult {
    fn default() -> Self {
        Self::err("No result".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_result_ok() {
        let result = WasmResult::ok("test data".to_string());
        assert!(result.success);
        assert_eq!(result.data, Some("test data".to_string()));
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_wasm_result_err() {
        let result = WasmResult::err("test error".to_string());
        assert!(!result.success);
        assert_eq!(result.data, None);
        assert_eq!(result.error, Some("test error".to_string()));
    }

    #[test]
    fn test_wasm_result_from_result_ok() {
        let result: Result<String, &str> = Ok("success".to_string());
        let wasm_result = WasmResult::from_result(result);
        assert!(wasm_result.success);
        assert_eq!(wasm_result.data, Some("success".to_string()));
    }

    #[test]
    fn test_wasm_result_from_result_err() {
        let result: Result<String, &str> = Err("failure");
        let wasm_result = WasmResult::from_result(result);
        assert!(!wasm_result.success);
        assert_eq!(wasm_result.error, Some("failure".to_string()));
    }

    #[test]
    fn test_wasm_result_to_json() {
        let result = WasmResult::ok("test".to_string());
        let json = result.to_json().unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"data\":\"test\""));
    }
}
