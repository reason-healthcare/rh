//! Evaluation context management for FHIRPath expressions

use crate::evaluator::types::FhirPathValue;
use serde_json::Value;
use std::collections::HashMap;

/// Context for evaluating FHIRPath expressions
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    /// The root resource being evaluated
    pub root: Value,
    /// Current context (this)
    pub current: Value,
    /// Current FhirPathValue for $this context variable
    pub this_value: Option<FhirPathValue>,
    /// External constants
    pub constants: HashMap<String, FhirPathValue>,
}

impl EvaluationContext {
    /// Create a new evaluation context
    pub fn new(resource: Value) -> Self {
        Self {
            current: resource.clone(),
            root: resource,
            this_value: None,
            constants: HashMap::new(),
        }
    }

    /// Add an external constant
    pub fn add_constant(&mut self, name: String, value: FhirPathValue) {
        self.constants.insert(name, value);
    }

    /// Create a new context with a different current value
    pub fn with_current(&self, current: Value) -> Self {
        Self {
            root: self.root.clone(),
            current,
            this_value: self.this_value.clone(),
            constants: self.constants.clone(),
        }
    }

    /// Create a new context with a specific $this value
    pub fn with_this_value(&self, this_value: FhirPathValue) -> Self {
        Self {
            root: self.root.clone(),
            current: this_value.to_json(),
            this_value: Some(this_value),
            constants: self.constants.clone(),
        }
    }
}
