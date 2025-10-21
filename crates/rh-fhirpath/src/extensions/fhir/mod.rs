//! FHIR-specific Extensions for FHIRPath
//!
//! This module provides FHIR-specific extensions to the core FHIRPath specification,
//! including functions and variables commonly used in FHIR contexts.
//!
//! # Available Extensions
//!
//! ## Functions
//! - `extension(url)` - Get extensions by URL
//! - `resolve()` - Resolve resource references
//! - `conformsTo(url)` - Check if resource conforms to profile
//!
//! ## Variables
//! - `%resource` - Current resource being evaluated
//! - `%context` - Current evaluation context
//! - `%rootResource` - Root resource in the evaluation
//!
//! # Feature Flag
//!
//! This module is only available when the `fhir-extensions` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! fhirpath = { version = "0.1", features = ["fhir-extensions"] }
//! ```
//!
//! # Usage Example
//!
//! ```rust
//! use rh_fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
//! use rh_fhirpath::serde_json::json;
//!
//! let parser = FhirPathParser::new();
//! let evaluator = FhirPathEvaluator::new();
//! let patient = json!({
//!     "resourceType": "Patient",
//!     "extension": [{
//!         "url": "http://example.org/fhir/StructureDefinition/patient-nickname",
//!         "valueString": "Johnny"
//!     }]
//! });
//! let context = EvaluationContext::new(patient);
//!
//! // Use FHIR extension function
//! let parsed = parser.parse("extension('http://example.org/fhir/StructureDefinition/patient-nickname').valueString").unwrap();
//! let result = evaluator.evaluate(&parsed, &context).unwrap();
//!
//! // Use FHIR variable
//! let parsed2 = parser.parse("%resource.resourceType").unwrap();
//! let result2 = evaluator.evaluate(&parsed2, &context).unwrap();
//! ```
//! ```

pub mod functions;
pub mod variables;

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

/// Function signature for extension functions
type ExtensionFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue> + Send + Sync>;

/// Variable resolver signature
type VariableResolver =
    Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>> + Send + Sync>;

/// FHIR extension registry
pub struct FhirExtensions {
    pub functions: HashMap<String, ExtensionFunction>,
    pub variables: HashMap<String, VariableResolver>,
}

impl FhirExtensions {
    /// Create a new FHIR extension registry with all functions and variables registered
    pub fn new() -> Self {
        let mut extensions = Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        };

        extensions.register_all();
        extensions
    }

    /// Register all FHIR extension functions and variables
    fn register_all(&mut self) {
        functions::register_functions(&mut self.functions);
        variables::register_variables(&mut self.variables);
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&ExtensionFunction> {
        self.functions.get(name)
    }

    /// Resolve a variable by name
    pub fn resolve_variable(
        &self,
        name: &str,
        context: &EvaluationContext,
    ) -> FhirPathResult<Option<FhirPathValue>> {
        for resolver in self.variables.values() {
            if let Some(value) = resolver(name, context)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    /// Get all registered function names
    pub fn function_names(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }

    /// Get all registered variable names  
    pub fn variable_names(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }
}

impl Default for FhirExtensions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_fhir_extensions_creation() {
        let extensions = FhirExtensions::new();

        // Should have some functions and variables registered
        assert!(!extensions.functions.is_empty());
        assert!(!extensions.variables.is_empty());
    }

    #[test]
    fn test_function_registration() {
        let extensions = FhirExtensions::new();
        let function_names = extensions.function_names();

        // Should have the FHIR functions
        println!("FHIR functions: {function_names:?}");
        assert!(function_names.iter().any(|name| *name == "extension"));
    }

    #[test]
    fn test_variable_registration() {
        let extensions = FhirExtensions::new();
        let variable_names = extensions.variable_names();

        // Should have the FHIR variables
        println!("FHIR variables: {variable_names:?}");
        assert!(variable_names.iter().any(|name| *name == "resource"));
    }

    #[test]
    fn test_variable_resolution() {
        let extensions = FhirExtensions::new();
        let patient = json!({
            "resourceType": "Patient",
            "id": "123"
        });
        let context = EvaluationContext::new(patient);

        let result = extensions.resolve_variable("resource", &context).unwrap();
        assert!(result.is_some());
    }
}
