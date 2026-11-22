//! Evaluation context management for FHIRPath expressions

use crate::evaluator::{metadata::FhirModel, types::FhirPathValue};
use crate::extensions::fhir::host::FhirHost;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

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
    /// The starting focus, retained when scoped functions replace $this.
    pub(crate) initial_focus: Option<Rc<FhirPathValue>>,
    // An unqualified invocation can have an input distinct from the enclosing
    // scope's $this when evaluating a defineVariable projection.
    pub(crate) invocation_focus: Option<FhirPathValue>,
    /// Running accumulator for `aggregate()` — the `$total` variable.
    pub total_value: Option<FhirPathValue>,
    /// Current iteration index for `$index` variable (set by select(), where(), etc.)
    pub index_value: Option<i64>,
    /// External constants
    pub constants: HashMap<String, FhirPathValue>,
    pub(crate) variables: HashMap<String, FhirPathValue>,
    /// FHIR release used for logical-model evaluation.
    pub model: FhirModel,
    /// Optional services used by FHIR-specific functions.
    pub host: Option<Arc<dyn FhirHost>>,
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
            initial_focus: None,
            invocation_focus: None,
            total_value: None,
            index_value: Some(0),
            constants: HashMap::new(),
            variables: HashMap::new(),
            model: FhirModel::default(),
            host: None,
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

    /// Set the initial focus to a different JSON value, keeping the root resource.
    pub fn with_current(&self, current: Value) -> Self {
        let value =
            crate::evaluator::metadata::FhirNode::root(&current, self.model, current == self.root);
        let mut context = self.with_initial_focus(value);
        context.current = current;
        context
    }

    /// Set the initial focus to a typed value, keeping the root resource.
    pub fn with_initial_focus(&self, value: FhirPathValue) -> Self {
        let mut context = self.with_this_and_index(value.clone(), 0);
        context.initial_focus = Some(Rc::new(value));
        context.total_value = None;
        context
    }

    /// Create a new context with a specific $this value
    pub fn with_this_value(&self, this_value: FhirPathValue) -> Self {
        Self {
            current: this_value.to_json(),
            this_value: Some(this_value),
            invocation_focus: None,
            ..self.clone()
        }
    }

    pub(crate) fn with_invocation_focus(&self, focus: FhirPathValue) -> Self {
        Self {
            invocation_focus: Some(focus),
            ..self.clone()
        }
    }

    pub(crate) fn current_focus(&self) -> FhirPathValue {
        self.invocation_focus
            .as_ref()
            .or(self.this_value.as_ref())
            .cloned()
            .unwrap_or_else(|| crate::evaluator::metadata::root_or_current_node(self))
    }

    /// Create a new context with `$this` and `$index` set (for select(), where()).
    pub fn with_this_and_index(&self, this_value: FhirPathValue, index: i64) -> Self {
        Self {
            index_value: Some(index),
            ..self.with_this_value(this_value)
        }
    }

    /// Change the focus where the iteration index is undefined.
    pub(crate) fn with_unindexed_focus(&self, this_value: FhirPathValue) -> Self {
        Self {
            index_value: None,
            ..self.with_this_value(this_value)
        }
    }

    /// Create a new context with `$this` set but `current` left unchanged.
    pub fn with_this_only(&self, this_value: FhirPathValue) -> Self {
        Self {
            this_value: Some(this_value),
            invocation_focus: Some(self.current_focus()),
            ..self.clone()
        }
    }

    /// Create a new context with both $this and $total set (for aggregate()).
    pub fn with_aggregate_vars(
        &self,
        this_value: FhirPathValue,
        total_value: FhirPathValue,
    ) -> Self {
        Self {
            total_value: Some(total_value),
            index_value: None,
            ..self.with_this_value(this_value)
        }
    }

    pub fn with_aggregate_vars_and_index(
        &self,
        this_value: FhirPathValue,
        total_value: FhirPathValue,
        index: i64,
    ) -> Self {
        Self {
            total_value: Some(total_value),
            index_value: Some(index),
            ..self.with_this_value(this_value)
        }
    }
}
