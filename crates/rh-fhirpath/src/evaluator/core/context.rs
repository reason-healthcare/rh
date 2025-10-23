//! Evaluation context management for FHIRPath expressions

use crate::evaluator::types::FhirPathValue;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A trace log entry from the trace() function
#[derive(Debug, Clone)]
pub struct TraceLog {
    pub name: String,
    pub value: String,
}

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
    /// Trace logs collected during evaluation
    pub trace_logs: Rc<RefCell<Vec<TraceLog>>>,
}

impl EvaluationContext {
    /// Create a new evaluation context
    pub fn new(resource: Value) -> Self {
        Self {
            current: resource.clone(),
            root: resource,
            this_value: None,
            constants: HashMap::new(),
            trace_logs: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Add an external constant
    pub fn add_constant(&mut self, name: String, value: FhirPathValue) {
        self.constants.insert(name, value);
    }

    /// Add a trace log entry
    pub fn add_trace_log(&self, name: String, value: String) {
        self.trace_logs.borrow_mut().push(TraceLog { name, value });
    }

    /// Get all trace logs
    pub fn get_trace_logs(&self) -> Vec<TraceLog> {
        self.trace_logs.borrow().clone()
    }

    /// Clear all trace logs
    pub fn clear_trace_logs(&self) {
        self.trace_logs.borrow_mut().clear();
    }

    /// Create a new context with a different current value
    pub fn with_current(&self, current: Value) -> Self {
        Self {
            root: self.root.clone(),
            current,
            this_value: self.this_value.clone(),
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
        }
    }

    /// Create a new context with a specific $this value
    pub fn with_this_value(&self, this_value: FhirPathValue) -> Self {
        Self {
            root: self.root.clone(),
            current: this_value.to_json(),
            this_value: Some(this_value),
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
        }
    }
}
