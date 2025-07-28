//! SQL-on-FHIR Extension Functions
//!
//! This module provides SQL-on-FHIR specific functions for resource and reference key handling.
//! These functions are designed to support efficient joins in SQL-based FHIR implementations.
//!
//! # Functions
//!
//! - `getResourceKey()` - Get resource primary key for SQL-on-FHIR
//! - `getReferenceKey([type])` - Get reference key for SQL-on-FHIR joins
//!
//! # Usage
//!
//! ```ignore
//! // Get resource key
//! Patient.getResourceKey()
//!
//! // Get reference key without type filter
//! Observation.subject.getReferenceKey()
//!
//! // Get reference key with type filter
//! Observation.subject.getReferenceKey(Patient)
//! ```

pub mod functions;

use crate::extensions::ExtensionFunction;
use std::collections::HashMap;

/// SQL-on-FHIR Extensions registry
pub struct SqlOnFhirExtensions {
    functions: HashMap<String, ExtensionFunction>,
}

impl SqlOnFhirExtensions {
    /// Create a new SQL-on-FHIR extensions registry
    pub fn new() -> Self {
        let mut extensions = Self {
            functions: HashMap::new(),
        };
        
        extensions.register_all();
        extensions
    }

    /// Register all SQL-on-FHIR extension functions
    fn register_all(&mut self) {
        functions::register_functions(&mut self.functions);
    }

    /// Get all registered functions
    pub fn get_all_functions(&self) -> &HashMap<String, ExtensionFunction> {
        &self.functions
    }

    /// Get a specific function by name
    pub fn get_function(&self, name: &str) -> Option<&ExtensionFunction> {
        self.functions.get(name)
    }

    /// Get all registered function names
    pub fn function_names(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }
}

impl Default for SqlOnFhirExtensions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_on_fhir_extensions_creation() {
        let extensions = SqlOnFhirExtensions::new();
        
        // Should have both functions registered
        assert!(extensions.get_function("getResourceKey").is_some());
        assert!(extensions.get_function("getReferenceKey").is_some());
        
        let function_names = extensions.function_names();
        assert_eq!(function_names.len(), 2);
        assert!(function_names.contains(&&"getResourceKey".to_string()));
        assert!(function_names.contains(&&"getReferenceKey".to_string()));
    }

    #[test]
    fn test_function_registration() {
        let extensions = SqlOnFhirExtensions::new();
        let all_functions = extensions.get_all_functions();
        
        assert_eq!(all_functions.len(), 2);
        assert!(all_functions.contains_key("getResourceKey"));
        assert!(all_functions.contains_key("getReferenceKey"));
    }
}
