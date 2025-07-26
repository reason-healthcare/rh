//! Built-in function registration and evaluation for FHIRPath

use crate::error::*;
use crate::evaluator::collection::CollectionEvaluator;
use crate::evaluator::values::FhirPathValue;
use std::collections::HashMap;

/// Type alias for FHIRPath function implementation
type FhirPathFunction =
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
        registry
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&FhirPathFunction> {
        self.functions.get(name)
    }

    /// Register all built-in functions
    fn register_builtin_functions(&mut self) {
        self.register_empty_function();
        self.register_exists_function();
        self.register_count_function();
        self.register_distinct_function();
        self.register_is_distinct_function();
    }

    /// Register the empty() function
    fn register_empty_function(&mut self) {
        self.functions.insert(
            "empty".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::is_empty(target)
            }),
        );
    }

    /// Register the exists() function
    fn register_exists_function(&mut self) {
        self.functions.insert(
            "exists".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::exists(target)
            }),
        );
    }

    /// Register the count() function
    fn register_count_function(&mut self) {
        self.functions.insert(
            "count".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::count(target)
            }),
        );
    }

    /// Register the distinct() function
    fn register_distinct_function(&mut self) {
        self.functions.insert(
            "distinct".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::distinct(target)
            }),
        );
    }

    /// Register the isDistinct() function
    fn register_is_distinct_function(&mut self) {
        self.functions.insert(
            "isDistinct".to_string(),
            Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
                CollectionEvaluator::is_distinct(target)
            }),
        );
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
