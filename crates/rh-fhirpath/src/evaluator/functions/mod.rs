//! Built-in function registration and evaluation for FHIRPath
//!
//! This module contains the function registry and implementations for all
//! built-in FHIRPath functions organized by category:
//! - Collection functions (empty, exists, count, distinct, etc.)
//! - String functions (length, upper, lower, substring, etc.)
//! - Math functions (abs, ceiling, floor, sqrt, etc.)
//! - DateTime functions (now, today, yearOf, etc.)
//! - Conversion functions (toBoolean, toInteger, toString, etc.)
//! - Boolean functions (not, etc.)
//! - Type functions (is, as - backward compatibility)

pub mod boolean_functions;
pub mod collection_functions;
pub mod conversion_functions;
pub mod datetime_functions;
pub mod math_functions;
pub mod string_functions;

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

/// Type alias for FHIRPath function implementation
pub type FhirPathFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>;

/// Built-in function registry
pub struct FunctionRegistry {
    functions: HashMap<String, FhirPathFunction>,
}

impl FunctionRegistry {
    /// Create a new function registry with built-in functions
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        registry.register_builtin_functions();
        registry.register_extension_functions();
        registry
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&FhirPathFunction> {
        self.functions.get(name)
    }

    /// Register all built-in functions
    fn register_builtin_functions(&mut self) {
        // Collection functions (empty, exists, count, distinct, etc.)
        collection_functions::register_collection_functions(&mut self.functions);

        // Boolean collection functions (all, allTrue, anyTrue, etc.)
        collection_functions::register_boolean_collection_functions(&mut self.functions);

        // Subsetting functions (single, first, last, skip, take, etc.)
        collection_functions::register_subsetting_functions(&mut self.functions);

        // Control flow functions (iif)
        collection_functions::register_control_flow_functions(&mut self.functions);

        // String functions (length, upper, lower, substring, etc.)
        string_functions::register_string_functions(&mut self.functions);

        // Math functions (abs, ceiling, floor, sqrt, etc.)
        math_functions::register_math_functions(&mut self.functions);

        // Date/time functions (now, today, yearOf, etc.)
        datetime_functions::register_datetime_functions(&mut self.functions);

        // Conversion functions (toBoolean, toInteger, toString, etc.)
        conversion_functions::register_conversion_functions(&mut self.functions);

        // Boolean functions (not, etc.)
        boolean_functions::register_boolean_functions(&mut self.functions);
    }

    /// Register extension functions from the extension system
    fn register_extension_functions(&mut self) {
        let extension_registry = crate::extensions::ExtensionRegistry::new();

        // Get all extension functions and register them
        for (name, func) in extension_registry.get_all_functions() {
            // Convert the extension function signature to the built-in function signature
            let extension_func: FhirPathFunction =
                Box::new(move |target, params| func(target, params));
            self.functions.insert(name, extension_func);
        }
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
