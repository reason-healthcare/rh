//! FHIRPath Extensions System
//!
//! This module provides the extension system for FHIRPath, allowing domain-specific
//! functions, variables, and operations to be added beyond the core specification.
//!
//! # Extension Types
//!
//! - **FHIR Extensions**: FHIR-specific functions and variables like `extension()`, `resolve()`, `%resource`
//! - **SQL-on-FHIR Extensions**: Functions for SQL-on-FHIR queries and transformations
//!
//! # Usage
//!
//! Extensions are automatically registered when the FHIRPath evaluator is created.
//! Individual extension modules can be enabled/disabled via feature flags.
//!
//! ```rust
//! use fhirpath::{FhirPathEvaluator, FhirPathParser, EvaluationContext};
//! use serde_json::json;
//!
//! let parser = FhirPathParser::new();
//! let evaluator = FhirPathEvaluator::new(); // Extensions auto-registered
//! let context = EvaluationContext::new(json!({"resourceType": "Patient"}));
//!
//! // Use FHIR extension functions
//! let parsed = parser.parse("%resource.resourceType").unwrap();
//! let result = evaluator.evaluate(&parsed, &context).unwrap();
//! ```

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use crate::evaluator::EvaluationContext;
use std::collections::HashMap;

// Extension modules - can be conditionally compiled with feature flags
// #[cfg(feature = "fhir-extensions")]
pub mod fhir;

// #[cfg(feature = "sql-extensions")]
pub mod sql;

/// Function signature for extension functions
pub type ExtensionFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue> + Send + Sync>;

/// Variable resolver function signature
pub type VariableResolver =
    Box<dyn Fn(&str, &EvaluationContext) -> FhirPathResult<Option<FhirPathValue>> + Send + Sync>;

/// Central registry for all FHIRPath extensions
pub struct ExtensionRegistry {
    functions: HashMap<String, ExtensionFunction>,
    variables: HashMap<String, VariableResolver>,
}

impl ExtensionRegistry {
    /// Create a new extension registry with all available extensions
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        };

        registry.register_all_extensions();
        registry
    }

    /// Register all available extension modules
    fn register_all_extensions(&mut self) {
        // Register FHIR extensions if feature is enabled
        // #[cfg(feature = "fhir-extensions")]
        // {
        self.register_fhir_extensions();
        // }

        // Register SQL-on-FHIR extensions if feature is enabled
        // #[cfg(feature = "sql-extensions")]
        // {
        self.register_sql_extensions();
        // }
    }

    /// Register FHIR-specific extensions
    // #[cfg(feature = "fhir-extensions")]
    fn register_fhir_extensions(&mut self) {
        let fhir_extensions = fhir::FhirExtensions::new();

        // Register FHIR functions
        for (name, func) in fhir_extensions.functions {
            self.functions.insert(name, func);
        }

        // Register FHIR variables
        for (name, resolver) in fhir_extensions.variables {
            self.variables.insert(name, resolver);
        }
    }

    /// Register SQL-on-FHIR extensions
    // #[cfg(feature = "sql-extensions")]
    fn register_sql_extensions(&mut self) {
        // Register SQL functions directly
        sql::functions::register_functions(&mut self.functions);

        // SQL-on-FHIR currently has no variables, but structure is ready for future additions
    }

    /// Get an extension function by name
    pub fn get_function(&self, name: &str) -> Option<&ExtensionFunction> {
        self.functions.get(name)
    }

    /// Resolve an extension variable by name
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

    /// Check if a function is registered
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Check if a variable resolver is registered
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Get all extension functions for integration with the main function registry
    pub fn get_all_functions(self) -> HashMap<String, ExtensionFunction> {
        self.functions
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extension_registry_creation() {
        let registry = ExtensionRegistry::new();

        // Should have at least custom extensions registered
        assert!(!registry.functions.is_empty() || !registry.variables.is_empty());
    }

    #[test]
    fn test_function_registration() {
        let registry = ExtensionRegistry::new();
        let function_names: Vec<&String> = registry.function_names();

        // Should have some functions registered (from custom extensions at minimum)
        // Exact count depends on enabled features
        println!("Registered functions: {function_names:?}");
    }

    #[test]
    fn test_variable_registration() {
        let registry = ExtensionRegistry::new();
        let variable_names: Vec<&String> = registry.variable_names();

        // Should have some variables registered (from custom extensions at minimum)
        println!("Registered variables: {variable_names:?}");
    }

    #[test]
    fn test_nonexistent_function() {
        let registry = ExtensionRegistry::new();
        assert!(!registry.has_function("nonexistentFunction"));
        assert!(registry.get_function("nonexistentFunction").is_none());
    }

    #[test]
    fn test_nonexistent_variable() {
        let registry = ExtensionRegistry::new();
        let context = EvaluationContext::new(json!({}));

        let result = registry
            .resolve_variable("nonexistentVar", &context)
            .unwrap();
        assert!(result.is_none());
    }
}
