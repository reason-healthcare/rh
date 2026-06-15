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
    /// Running accumulator for `aggregate()` — the `$total` variable.
    pub total_value: Option<FhirPathValue>,
    /// Current iteration index for `$index` variable (set by select(), where(), etc.)
    pub index_value: Option<i64>,
    /// External constants
    pub constants: HashMap<String, FhirPathValue>,
    /// Trace logs collected during evaluation
    pub trace_logs: Rc<RefCell<Vec<TraceLog>>>,
    /// When `true`, choice-type suffixed keys (e.g. `valueQuantity`) are
    /// rejected — the canonical base name (`value`) must be used instead.
    /// Conforms to HL7 FHIRPath `mode="strict"` semantics.
    pub strict_mode: bool,
    /// When `true`, order-sensitive functions (`skip`, `tail`, `first`,
    /// `last`, `orderBy`, `reverse`) applied to unordered collections
    /// (e.g. `children()`) produce a semantic error rather than a result.
    /// Conforms to HL7 FHIRPath `checkOrderedFunctions="true"` semantics.
    pub check_ordered_functions: bool,
}

impl EvaluationContext {
    /// Create a new evaluation context
    pub fn new(resource: Value) -> Self {
        Self {
            current: resource.clone(),
            root: resource,
            this_value: None,
            total_value: None,
            index_value: None,
            constants: HashMap::new(),
            trace_logs: Rc::new(RefCell::new(Vec::new())),
            strict_mode: false,
            check_ordered_functions: false,
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
            total_value: self.total_value.clone(),
            index_value: self.index_value,
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
            strict_mode: self.strict_mode,
            check_ordered_functions: self.check_ordered_functions,
        }
    }

    /// Create a new context with a specific $this value
    pub fn with_this_value(&self, this_value: FhirPathValue) -> Self {
        Self {
            root: self.root.clone(),
            current: this_value.to_json(),
            this_value: Some(this_value),
            total_value: self.total_value.clone(),
            index_value: self.index_value,
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
            strict_mode: self.strict_mode,
            check_ordered_functions: self.check_ordered_functions,
        }
    }

    /// Create a new context with `$this` and `$index` set (for select(), where()).
    pub fn with_this_and_index(&self, this_value: FhirPathValue, index: i64) -> Self {
        Self {
            root: self.root.clone(),
            current: this_value.to_json(),
            this_value: Some(this_value),
            total_value: self.total_value.clone(),
            index_value: Some(index),
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
            strict_mode: self.strict_mode,
            check_ordered_functions: self.check_ordered_functions,
        }
    }

    /// Create a new context with `$this` set but `current` left unchanged.
    /// Use this for iif(), where parameters must resolve root paths (e.g.
    /// `testData.foo`) while $this should still reflect the iif receiver.
    pub fn with_this_only(&self, this_value: FhirPathValue) -> Self {
        Self {
            root: self.root.clone(),
            current: self.current.clone(),
            this_value: Some(this_value),
            total_value: self.total_value.clone(),
            index_value: self.index_value,
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
            strict_mode: self.strict_mode,
            check_ordered_functions: self.check_ordered_functions,
        }
    }

    /// Create a new context with both $this and $total set (for aggregate()).
    pub fn with_aggregate_vars(
        &self,
        this_value: FhirPathValue,
        total_value: FhirPathValue,
    ) -> Self {
        Self {
            root: self.root.clone(),
            current: this_value.to_json(),
            this_value: Some(this_value),
            total_value: Some(total_value),
            index_value: None,
            constants: self.constants.clone(),
            trace_logs: self.trace_logs.clone(),
            strict_mode: self.strict_mode,
            check_ordered_functions: self.check_ordered_functions,
        }
    }
}
