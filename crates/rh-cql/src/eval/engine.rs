//! ELM evaluation engine entry points.
//!
//! Implements:
//! - [`evaluate_elm`]  — evaluate a named expression from an ELM library
//! - [`evaluate_elm_with_trace`] — same but return step-by-step trace events
//! - [`TraceEvent`]    — per-node evaluation record

use std::collections::{BTreeMap, HashMap, HashSet};

use super::context::{EvalContext, EvalError};
use super::operators::*;
use super::tvl::{tvl_and, tvl_implies, tvl_not, tvl_or, tvl_xor};
use super::value::Value;
use crate::elm::{
    BinaryExpression, Expression, Library, NaryExpression, StatementDef, UnaryExpression,
};

// ---------------------------------------------------------------------------
// TraceEvent (Task 9.20)
// ---------------------------------------------------------------------------

/// A single step in an evaluated expression tree.
///
/// Used by [`evaluate_elm_with_trace`] to return a step-by-step audit trail.
#[derive(Debug, Clone)]
pub struct TraceEvent {
    /// Unique sequential event id (1-based).
    pub event_id: u64,
    /// The ELM `localId` of the node that was evaluated, if present.
    pub elm_node_id: Option<String>,
    /// Operation name (e.g. `"Add"`, `"Literal"`, `"ExpressionRef"`).
    pub op: String,
    /// Evaluated input values (before the operation).
    pub inputs: Vec<Value>,
    /// The output value produced by this operation.
    pub output: Value,
    /// Child event ids (nested sub-expressions evaluated as part of this node).
    pub children: Vec<u64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Evaluate a named expression from a compiled ELM [`Library`].
///
/// # Arguments
///
/// - `library`  — the compiled ELM library
/// - `name`     — the expression definition name to evaluate
/// - `ctx`      — runtime evaluation context (clock, parameters, providers)
///
/// # Returns
///
/// `Ok(Value)` on success, `Err(EvalError)` on type mismatch or runtime error.
pub fn evaluate_elm(library: &Library, name: &str, ctx: &EvalContext) -> Result<Value, EvalError> {
    let mut engine = Engine::new(library, ctx);
    let expr = engine.find_expression(name)?;
    engine.eval_expr(expr)
}

/// Evaluate a named expression in `library`, with access to a map of
/// pre-compiled included libraries keyed by their local alias.
///
/// Cross-library `ExpressionRef` nodes with `library_name` set to one of the
/// alias keys are dispatched to the corresponding library.
pub fn evaluate_elm_with_libraries(
    library: &Library,
    included: &HashMap<String, Library>,
    name: &str,
    ctx: &EvalContext,
) -> Result<Value, EvalError> {
    let mut engine = Engine::new_with_libraries(library, Some(included), ctx);
    let expr = engine.find_expression(name)?;
    engine.eval_expr(expr)
}

/// Evaluate a named expression and return the result plus a flat trace of
/// every sub-expression evaluated.
///
/// Trace events are returned in evaluation order (parent before children is
/// not guaranteed; order reflects actual recursive evaluation order).
pub fn evaluate_elm_with_trace(
    library: &Library,
    name: &str,
    ctx: &EvalContext,
) -> Result<(Value, Vec<TraceEvent>), EvalError> {
    let mut engine = Engine::new(library, ctx);
    let expr = engine.find_expression(name)?;
    let value = engine.eval_expr(expr)?;
    let trace = std::mem::take(&mut engine.trace);
    Ok((value, trace))
}

// ---------------------------------------------------------------------------
// Engine internals
// ---------------------------------------------------------------------------

struct Engine<'lib, 'ctx> {
    library: &'lib Library,
    ctx: &'ctx EvalContext,
    trace: Vec<TraceEvent>,
    next_event_id: u64,
    /// Expression name → body, built at construction for O(1) lookup.
    expr_index: HashMap<String, &'lib Expression>,
    /// Set of parameter names declared in the library, for fast membership test.
    param_names: HashSet<String>,
    /// Binding scope stack — top frame is the innermost scope.
    ///
    /// Frame 0 is always the base frame (parameter values from `ctx`).
    /// Frames are pushed for query row scopes, let clause bindings, and
    /// function arguments, then popped when the sub-expression is done.
    /// [`lookup_binding`] walks the stack from top to bottom so inner scopes
    /// shadow outer ones correctly.
    scope_stack: Vec<BTreeMap<String, Value>>,
    /// Included libraries keyed by alias, used to resolve cross-library
    /// `ExpressionRef` and `FunctionRef` nodes.  Empty for the standard
    /// single-library evaluation path.
    included: Option<&'lib HashMap<String, Library>>,
}

impl<'lib, 'ctx> Engine<'lib, 'ctx> {
    fn new(library: &'lib Library, ctx: &'ctx EvalContext) -> Self {
        Self::new_with_libraries(library, None, ctx)
    }

    fn new_with_libraries(
        library: &'lib Library,
        included: Option<&'lib HashMap<String, Library>>,
        ctx: &'ctx EvalContext,
    ) -> Self {
        // Pre-build expression and parameter indexes for O(1) lookup.
        let mut expr_index: HashMap<String, &'lib Expression> = HashMap::new();
        let mut param_names: HashSet<String> = HashSet::new();

        if let Some(stmts) = &library.statements {
            for def in &stmts.defs {
                if let StatementDef::Expression(ed) = def {
                    if let (Some(name), Some(expr)) = (&ed.name, &ed.expression) {
                        expr_index.insert(name.clone(), expr.as_ref());
                    }
                }
            }
        }

        if let Some(params) = &library.parameters {
            for p in &params.defs {
                if let Some(name) = &p.name {
                    param_names.insert(name.clone());
                }
            }
        }

        // Initialise the base scope frame with parameter values from ctx.
        let mut base_scope: BTreeMap<String, Value> = BTreeMap::new();
        for (k, v) in &ctx.parameters {
            base_scope.insert(k.clone(), v.clone());
        }

        Self {
            library,
            ctx,
            trace: Vec::new(),
            next_event_id: 1,
            expr_index,
            param_names,
            scope_stack: vec![base_scope],
            included,
        }
    }

    fn find_expression(&self, name: &str) -> Result<&'lib Expression, EvalError> {
        self.expr_index
            .get(name)
            .copied()
            .ok_or_else(|| EvalError::General(format!("Expression '{name}' not found in library")))
    }

    /// Return true if `name` is declared as a parameter in the library.
    fn is_library_parameter(&self, name: &str) -> bool {
        self.param_names.contains(name)
    }

    // ── Scope management ────────────────────────────────────────────────────

    /// Look up a name in the scope stack, searching from the innermost (top)
    /// frame outward.  Returns `None` if the name is not bound in any frame.
    fn lookup_binding(&self, name: &str) -> Option<&Value> {
        for frame in self.scope_stack.iter().rev() {
            if let Some(v) = frame.get(name) {
                return Some(v);
            }
        }
        None
    }

    /// Push a new scope frame on top of the current environment.
    fn push_scope(&mut self, frame: BTreeMap<String, Value>) {
        self.scope_stack.push(frame);
    }

    /// Pop the topmost scope frame.  Panics (debug) if the base frame would be
    /// removed — caller must balance every `push_scope` with a `pop_scope`.
    fn pop_scope(&mut self) {
        debug_assert!(
            self.scope_stack.len() > 1,
            "pop_scope: cannot remove the base scope frame"
        );
        self.scope_stack.pop();
    }

    /// Snapshot the current merged bindings as a flat `BTreeMap` (innermost
    /// frame wins) for passing to external helpers such as `queries::eval_query`.
    fn snapshot_scope(&self) -> BTreeMap<String, Value> {
        let mut merged = BTreeMap::new();
        for frame in &self.scope_stack {
            merged.extend(frame.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        merged
    }

    fn record_trace(
        &mut self,
        op: &str,
        inputs: Vec<Value>,
        output: Value,
        children: Vec<u64>,
        elm_node_id: Option<String>,
    ) -> u64 {
        let id = self.next_event_id;
        self.next_event_id += 1;
        self.trace.push(TraceEvent {
            event_id: id,
            elm_node_id,
            op: op.to_string(),
            inputs,
            output,
            children,
        });
        id
    }

    fn eval_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            // ----- Literals -----
            Expression::Null(_) => Ok(Value::Null),
            Expression::Literal(lit) => {
                let val = eval_literal(lit)?;
                self.record_trace(
                    "Literal",
                    vec![],
                    val.clone(),
                    vec![],
                    lit.element.local_id.clone(),
                );
                Ok(val)
            }

            // ----- References -----
            Expression::ExpressionRef(r) => {
                let name = r.name.as_deref().unwrap_or("");

                // Cross-library reference: `library_name` is set when the CQL
                // source uses a qualified name like `Global."Inpatient Encounter"`.
                if let Some(alias) = r.library_name.as_deref() {
                    let included = self.included.ok_or_else(|| {
                        EvalError::LibraryNotFound { alias: alias.to_string() }
                    })?;
                    let inc_lib = included.get(alias).ok_or_else(|| {
                        EvalError::LibraryNotFound { alias: alias.to_string() }
                    })?;
                    let mut sub_engine =
                        Engine::new_with_libraries(inc_lib, Some(included), self.ctx);
                    let expr = sub_engine.find_expression(name).map_err(|_| {
                        EvalError::ExpressionNotFound(format!("{alias}.{name}"))
                    })?;
                    return sub_engine.eval_expr(expr);
                }

                // Check scope stack first (for query aliases / let clauses).
                if let Some(v) = self.lookup_binding(name) {
                    return Ok(v.clone());
                }
                // Check if it's a parameter from context.
                if let Some(v) = self.ctx.parameters.get(name) {
                    return Ok(v.clone());
                }
                // Parameters emitted as ExpressionRef evaluate to null if not provided.
                if self.is_library_parameter(name) {
                    return Ok(Value::Null);
                }
                // Otherwise evaluate from library.
                let expr = self.find_expression(name)?;
                let val = self.eval_expr(expr)?;
                self.record_trace(
                    "ExpressionRef",
                    vec![],
                    val.clone(),
                    vec![],
                    r.element.local_id.clone(),
                );
                Ok(val)
            }
            Expression::OperandRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                self.lookup_binding(name)
                    .cloned()
                    .ok_or_else(|| EvalError::General(format!("Operand '{name}' not found")))
            }
            Expression::AliasRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                self.lookup_binding(name)
                    .cloned()
                    .ok_or_else(|| EvalError::General(format!("Alias '{name}' not found")))
            }
            Expression::ParameterRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                self.ctx
                    .parameters
                    .get(name)
                    .cloned()
                    .ok_or_else(|| EvalError::General(format!("Parameter '{name}' not found")))
            }

            // ----- Logical & null -----
            Expression::And(_)
            | Expression::Or(_)
            | Expression::Not(_)
            | Expression::Xor(_)
            | Expression::Implies(_)
            | Expression::IsNull(_)
            | Expression::IsTrue(_)
            | Expression::IsFalse(_)
            | Expression::Coalesce(_) => self.eval_logical_expr(expr),

            // ----- Comparison -----
            Expression::Equal(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = equal(&a, &b);
                self.trace_binary(
                    "Equal",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Equivalent(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = equivalent(&a, &b);
                self.trace_binary(
                    "Equivalent",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::NotEqual(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let eq = equal(&a, &b);
                let result = match eq {
                    Value::Boolean(b) => Value::Boolean(!b),
                    other => other,
                };
                self.trace_binary(
                    "NotEqual",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Less(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = less(&a, &b)?;
                self.trace_binary(
                    "Less",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Greater(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = greater(&a, &b)?;
                self.trace_binary(
                    "Greater",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::LessOrEqual(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = less_or_equal(&a, &b)?;
                self.trace_binary(
                    "LessOrEqual",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::GreaterOrEqual(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = greater_or_equal(&a, &b)?;
                self.trace_binary(
                    "GreaterOrEqual",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }

            // ----- Arithmetic -----
            Expression::Add(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = add(&a, &b)?;
                self.trace_binary(
                    "Add",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Subtract(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = subtract(&a, &b)?;
                self.trace_binary(
                    "Subtract",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Multiply(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = multiply(&a, &b)?;
                self.trace_binary(
                    "Multiply",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Divide(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = divide(&a, &b)?;
                self.trace_binary(
                    "Divide",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::TruncatedDivide(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let d = divide(&a, &b)?;
                let result = truncate(&d)?;
                self.trace_binary(
                    "TruncatedDivide",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Modulo(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = modulo(&a, &b)?;
                self.trace_binary(
                    "Modulo",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Negate(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = negate(&v)?;
                self.trace_unary(
                    "Negate",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Abs(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = abs(&v)?;
                self.trace_unary(
                    "Abs",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Ceiling(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = ceiling(&v)?;
                self.trace_unary(
                    "Ceiling",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Floor(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = floor(&v)?;
                self.trace_unary(
                    "Floor",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Truncate(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = truncate(&v)?;
                self.trace_unary(
                    "Truncate",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Round(round_expr) => {
                let v = self.eval_expr_opt(round_expr.operand.as_deref())?;
                let precision = match round_expr.precision.as_deref() {
                    Some(p) => Some(self.eval_expr(p)?),
                    None => None,
                };
                round(&v, precision.as_ref())
            }
            Expression::Ln(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = ln(&v)?;
                self.trace_unary(
                    "Ln",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Exp(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = exp(&v)?;
                self.trace_unary(
                    "Exp",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Log(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = log(&a, &b)?;
                self.trace_binary(
                    "Log",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Power(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = power(&a, &b)?;
                self.trace_binary(
                    "Power",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Successor(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = successor(&v)?;
                self.trace_unary(
                    "Successor",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Predecessor(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = predecessor(&v)?;
                self.trace_unary(
                    "Predecessor",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::MinValue(typed) => min_value(typed.value_type.as_deref().unwrap_or("")),
            Expression::MaxValue(typed) => max_value(typed.value_type.as_deref().unwrap_or("")),

            // ----- String -----
            Expression::Concatenate(nary) => {
                let vals = self.eval_nary_args(nary)?;
                if vals.iter().any(|v| matches!(v, Value::Null)) {
                    return Ok(Value::Null);
                }
                let parts: Result<Vec<String>, _> = vals
                    .iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err(EvalError::General(
                            "Concatenate: non-string operand".to_string(),
                        )),
                    })
                    .collect();
                Ok(Value::String(parts?.join("")))
            }
            Expression::Combine(c) => {
                let list = self.eval_expr_opt(c.source.as_deref())?;
                let sep = match &c.separator {
                    Some(s) => Some(self.eval_expr(s)?),
                    None => None,
                };
                super::operators::combine(&list, sep.as_ref())
            }
            Expression::Split(s) => {
                let str_val = self.eval_expr_opt(s.string_to_split.as_deref())?;
                let sep = self.eval_expr_opt(s.separator.as_deref())?;
                super::operators::split(&str_val, &sep)
            }
            Expression::Length(unary) => {
                let v = self.eval_unary_arg(unary)?;
                match &v {
                    Value::String(_) => super::operators::length_str(&v),
                    Value::List(items) => Ok(Value::Integer(items.len() as i64)),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General("Length: unsupported type".to_string())),
                }
            }
            Expression::Upper(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::upper(&v)?;
                self.trace_unary(
                    "Upper",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Lower(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::lower(&v)?;
                self.trace_unary(
                    "Lower",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::StartsWith(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = super::operators::starts_with(&a, &b)?;
                self.trace_binary(
                    "StartsWith",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::EndsWith(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = super::operators::ends_with(&a, &b)?;
                self.trace_binary(
                    "EndsWith",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::Matches(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first())?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1))?;
                let result = super::operators::matches_regex(&a, &b)?;
                self.trace_binary(
                    "Matches",
                    bin.element.local_id.clone(),
                    a,
                    id_a,
                    b,
                    id_b,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::SplitOnMatches(som) => {
                let source = self.eval_expr_opt(som.string_to_split.as_deref())?;
                let pattern = self.eval_expr_opt(som.separator_pattern.as_deref())?;
                super::operators::split_on_matches(&source, &pattern)
            }
            Expression::PositionOf(pos) => {
                let pattern = self.eval_expr_opt(pos.pattern.as_deref())?;
                let string = self.eval_expr_opt(pos.string.as_deref())?;
                super::operators::position_of(&pattern, &string)
            }
            Expression::LastPositionOf(pos) => {
                let pattern = self.eval_expr_opt(pos.pattern.as_deref())?;
                let string = self.eval_expr_opt(pos.string.as_deref())?;
                super::operators::last_position_of(&pattern, &string)
            }
            Expression::Substring(sub) => {
                let string = self.eval_expr_opt(sub.string_to_sub.as_deref())?;
                let start = self.eval_expr_opt(sub.start_index.as_deref())?;
                let length = match sub.length.as_deref() {
                    Some(expr) => Some(self.eval_expr(expr)?),
                    None => None,
                };
                super::operators::substring(&string, &start, length.as_ref())
            }
            Expression::ReplaceMatches(ternary) => {
                let source = self.eval_expr_opt(ternary.operand.first())?;
                let pattern = self.eval_expr_opt(ternary.operand.get(1))?;
                let substitution = self.eval_expr_opt(ternary.operand.get(2))?;
                super::operators::replace_matches(&source, &pattern, &substitution)
            }

            // ----- Type Conversions -----
            Expression::ToBoolean(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_boolean(&v)?;
                self.trace_unary(
                    "ToBoolean",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToInteger(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_integer(&v)?;
                self.trace_unary(
                    "ToInteger",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToLong(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_long(&v)?;
                self.trace_unary(
                    "ToLong",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToDecimal(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_decimal(&v)?;
                self.trace_unary(
                    "ToDecimal",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToStringExpr(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_string(&v)?;
                self.trace_unary(
                    "ToStringExpr",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToDate(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_date(&v)?;
                self.trace_unary(
                    "ToDate",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToDateTime(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_datetime(&v)?;
                self.trace_unary(
                    "ToDateTime",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToTime(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_time(&v)?;
                self.trace_unary(
                    "ToTime",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToQuantity(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_quantity(&v)?;
                self.trace_unary(
                    "ToQuantity",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToConcept(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary)?;
                let result = super::operators::to_concept(&v)?;
                self.trace_unary(
                    "ToConcept",
                    unary.element.local_id.clone(),
                    v,
                    id_v,
                    result.clone(),
                );
                Ok(result)
            }
            Expression::ToList(unary) => {
                let v = self.eval_unary_arg(unary)?;
                match v {
                    Value::Null => Ok(Value::List(vec![])),
                    Value::List(_) => Ok(v),
                    other => Ok(Value::List(vec![other])),
                }
            }
            Expression::Is(is_expr) => {
                let v = self.eval_expr_opt(is_expr.operand.as_deref())?;
                let raw = is_expr.is_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                // "{urn:hl7-org:elm-types:r1}null" → is null check
                if type_name.eq_ignore_ascii_case("null") {
                    return Ok(Value::Boolean(v == Value::Null));
                }
                Ok(super::operators::is_type(&v, type_name))
            }
            Expression::As(as_expr) => {
                let v = self.eval_expr_opt(as_expr.operand.as_deref())?;
                let raw = as_expr.as_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                Ok(super::operators::as_type(&v, type_name))
            }
            Expression::Convert(conv_expr) => {
                let v = self.eval_expr_opt(conv_expr.operand.as_deref())?;
                let raw = conv_expr.to_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                super::operators::convert(&v, type_name)
            }
            Expression::CanConvert(can_conv) => {
                let v = self.eval_expr_opt(can_conv.operand.as_deref())?;
                if matches!(v, Value::Null) {
                    return Ok(Value::Null);
                }
                let raw = can_conv.to_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                match super::operators::convert(&v, type_name) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(_) => Ok(Value::Boolean(false)),
                }
            }
            Expression::CanConvertQuantity(binary) => {
                let (lhs, rhs) = self.eval_binary_args(binary)?;
                match (&lhs, &rhs) {
                    (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                    (Value::Quantity(q), Value::String(target_unit)) => {
                        // Conservative implementation: same-unit conversion is always valid;
                        // cross-unit conversion would require a UCUM library.
                        Ok(Value::Boolean(q.unit == *target_unit))
                    }
                    _ => Ok(Value::Boolean(false)),
                }
            }

            // ----- List -----
            Expression::List(_)
            | Expression::Exists(_)
            | Expression::Count(_)
            | Expression::Sum(_)
            | Expression::Min(_)
            | Expression::Max(_)
            | Expression::Avg(_)
            | Expression::First(_)
            | Expression::Last(_)
            | Expression::Flatten(_)
            | Expression::Distinct(_)
            | Expression::SingletonFrom(_)
            | Expression::Indexer(_)
            | Expression::Slice(_)
            | Expression::Union(_)
            | Expression::Intersect(_)
            | Expression::Except(_) => self.eval_list_expr(expr),

            // ----- Interval -----
            Expression::Interval(_)
            | Expression::Start(_)
            | Expression::End(_)
            | Expression::Width(_)
            | Expression::PointFrom(_)
            | Expression::Contains(_)
            | Expression::In(_)
            | Expression::Overlaps(_)
            | Expression::Meets(_)
            | Expression::MeetsBefore(_)
            | Expression::MeetsAfter(_)
            | Expression::Includes(_)
            | Expression::IncludedIn(_)
            | Expression::Starts(_)
            | Expression::Ends(_)
            | Expression::Collapse(_) => self.eval_interval_expr(expr),

            // ----- Control flow -----
            Expression::If(_) | Expression::Case(_) => self.eval_control_flow_expr(expr),

            // ----- Query & retrieve -----
            Expression::Query(_) | Expression::Retrieve(_) => self.eval_query_expr(expr),

            // ----- Tuple / Instance -----
            Expression::Tuple(tup) => {
                let mut fields = BTreeMap::new();
                for element in &tup.elements {
                    if let (Some(name), Some(val_expr)) = (&element.name, &element.value) {
                        fields.insert(name.clone(), self.eval_expr(val_expr)?);
                    }
                }
                Ok(Value::Tuple(fields))
            }

            // ----- Property -----
            Expression::Property(prop) => {
                let source = match &prop.source {
                    Some(e) => self.eval_expr(e)?,
                    None => {
                        let scope = prop.scope.as_deref().unwrap_or("$this");
                        self.lookup_binding(scope).cloned().unwrap_or(Value::Null)
                    }
                };
                let path = prop.path.as_deref().unwrap_or("");
                match source {
                    Value::Tuple(ref fields) => {
                        Ok(fields.get(path).cloned().unwrap_or(Value::Null))
                    }
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General(format!(
                        "Property '{path}': cannot access property on non-tuple"
                    ))),
                }
            }

            // ----- Date/Time -----
            Expression::Today(_) => Ok(Value::Date(self.ctx.today())),
            Expression::Now(_) => Ok(Value::DateTime(self.ctx.now())),
            Expression::TimeOfDay(_) => Ok(Value::Time(self.ctx.time_of_day())),
            Expression::DateFrom(unary) => {
                let value = self.eval_unary_arg(unary)?;
                match value {
                    Value::Null => Ok(Value::Null),
                    Value::Date(d) => Ok(Value::Date(d)),
                    Value::DateTime(dt) => Ok(Value::Date(crate::eval::value::CqlDate {
                        year: dt.year,
                        month: dt.month,
                        day: dt.day,
                    })),
                    _ => Err(EvalError::General(
                        "DateFrom: expected Date or DateTime".to_string(),
                    )),
                }
            }
            Expression::TimeFrom(unary) => {
                let value = self.eval_unary_arg(unary)?;
                match value {
                    Value::Null => Ok(Value::Null),
                    Value::Time(t) => Ok(Value::Time(t)),
                    Value::DateTime(dt) => match dt.hour {
                        Some(hour) => Ok(Value::Time(crate::eval::value::CqlTime {
                            hour,
                            minute: dt.minute,
                            second: dt.second,
                            millisecond: dt.millisecond,
                        })),
                        None => Ok(Value::Null),
                    },
                    _ => Err(EvalError::General(
                        "TimeFrom: expected Time or DateTime".to_string(),
                    )),
                }
            }
            Expression::TimezoneOffsetFrom(unary) => {
                let value = self.eval_unary_arg(unary)?;
                match value {
                    Value::Null => Ok(Value::Null),
                    Value::DateTime(dt) => {
                        let offset_secs = dt
                            .offset_seconds
                            .unwrap_or(self.ctx.timezone_offset_seconds);
                        Ok(Value::Decimal(offset_secs as f64 / 3600.0))
                    }
                    _ => Err(EvalError::General(
                        "TimezoneOffsetFrom: expected DateTime".to_string(),
                    )),
                }
            }
            Expression::SameAs(tb) => {
                let a = self.eval_expr_opt(tb.operand.first())?;
                let b = self.eval_expr_opt(tb.operand.get(1))?;
                super::operators::same_as(&a, &b, tb.precision.as_deref())
            }
            Expression::SameOrBefore(tb) => {
                let a = self.eval_expr_opt(tb.operand.first())?;
                let b = self.eval_expr_opt(tb.operand.get(1))?;
                super::operators::same_or_before(&a, &b, tb.precision.as_deref())
            }
            Expression::SameOrAfter(tb) => {
                let a = self.eval_expr_opt(tb.operand.first())?;
                let b = self.eval_expr_opt(tb.operand.get(1))?;
                super::operators::same_or_after(&a, &b, tb.precision.as_deref())
            }
            Expression::DurationBetween(tb) => {
                let a = self.eval_expr_opt(tb.operand.first())?;
                let b = self.eval_expr_opt(tb.operand.get(1))?;
                super::operators::duration_between(&a, &b, tb.precision.as_deref().unwrap_or("day"))
            }
            Expression::DifferenceBetween(tb) => {
                let a = self.eval_expr_opt(tb.operand.first())?;
                let b = self.eval_expr_opt(tb.operand.get(1))?;
                super::operators::difference_between(
                    &a,
                    &b,
                    tb.precision.as_deref().unwrap_or("day"),
                )
            }

            // ----- Interval: proper / expand / size -----
            Expression::OverlapsBefore(_)
            | Expression::OverlapsAfter(_)
            | Expression::Expand(_)
            | Expression::ProperContains(_)
            | Expression::ProperIn(_)
            | Expression::ProperIncludes(_)
            | Expression::ProperIncludedIn(_)
            | Expression::Size(_) => self.eval_interval_expr(expr),

            // ----- List: sort + statistical aggregates -----
            Expression::Sort(_)
            | Expression::Median(_)
            | Expression::Mode(_)
            | Expression::Variance(_)
            | Expression::StdDev(_)
            | Expression::PopulationVariance(_)
            | Expression::PopulationStdDev(_)
            | Expression::AllTrue(_)
            | Expression::AnyTrue(_)
            | Expression::Repeat(_) => self.eval_list_expr(expr),

            // ----- Terminology -----
            Expression::Code(_)
            | Expression::CodeRef(_)
            | Expression::ConceptRef(_)
            | Expression::ValueSetRef(_)
            | Expression::CodeSystemRef(_)
            | Expression::InValueSet(_)
            | Expression::InCodeSystem(_)
            | Expression::AnyInValueSet(_)
            | Expression::AnyInCodeSystem(_) => self.eval_terminology_expr(expr),

            // ----- Built-in function dispatch -----
            Expression::FunctionRef(func_ref) => {
                let name = func_ref.name.as_deref().unwrap_or("");
                let mut args = Vec::new();
                for operand in &func_ref.operand {
                    args.push(self.eval_expr(operand)?);
                }
                // Try builtin first regardless of library qualification.
                // Many FHIRHelpers functions (ToConcept, ToCode, …) mirror
                // CQL system builtins so this works transparently.
                eval_builtin_function(name, args).or_else(|e| {
                    if let Some(alias) = func_ref.library_name.as_deref() {
                        let included = self.included.ok_or_else(|| {
                            EvalError::LibraryNotFound { alias: alias.to_string() }
                        })?;
                        included.get(alias).ok_or_else(|| {
                            EvalError::LibraryNotFound { alias: alias.to_string() }
                        })?;
                        // Library found but function is not a known builtin.
                        // TODO: support evaluating user-defined functions from
                        // included libraries once the engine supports user-defined
                        // function bodies in general (same-library functions are
                        // also not yet evaluated by body).
                        Err(EvalError::ExpressionNotFound(format!("{alias}.{name}")))
                    } else {
                        Err(e)
                    }
                })
            }

            other => Err(EvalError::General(format!(
                "evaluate_elm: unsupported ELM expression type: {:?}",
                std::mem::discriminant(other)
            ))),
        }
    }

    // --- Expression-family helpers -----------------------------------------

    /// Evaluate logical and null-propagation expressions:
    /// `And`, `Or`, `Not`, `Xor`, `Implies`, `IsNull`, `IsTrue`, `IsFalse`,
    /// `Coalesce`.
    fn eval_logical_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            Expression::And(nary) => {
                let vals = self.eval_nary_args(nary)?;
                Ok(vals
                    .iter()
                    .fold(Value::Boolean(true), |acc, v| tvl_and(&acc, v)))
            }
            Expression::Or(nary) => {
                let vals = self.eval_nary_args(nary)?;
                Ok(vals
                    .iter()
                    .fold(Value::Boolean(false), |acc, v| tvl_or(&acc, v)))
            }
            Expression::Not(unary) => {
                let operand = self.eval_unary_arg(unary)?;
                Ok(tvl_not(&operand))
            }
            Expression::Xor(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                Ok(tvl_xor(&a, &b))
            }
            Expression::Implies(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                Ok(tvl_implies(&a, &b))
            }
            Expression::IsNull(unary) => {
                let v = self.eval_unary_arg(unary)?;
                Ok(Value::Boolean(matches!(v, Value::Null)))
            }
            Expression::IsTrue(unary) => {
                let v = self.eval_unary_arg(unary)?;
                Ok(Value::Boolean(v == Value::Boolean(true)))
            }
            Expression::IsFalse(unary) => {
                let v = self.eval_unary_arg(unary)?;
                Ok(Value::Boolean(v == Value::Boolean(false)))
            }
            Expression::Coalesce(nary) => {
                // CQL Coalesce has two overloads:
                //   Coalesce(a, b, ...) — variadic, first non-null arg wins
                //   Coalesce(List<T>)   — single list arg, first non-null element wins
                if nary.operand.len() == 1 {
                    let v = self.eval_expr(&nary.operand[0])?;
                    if let Value::List(items) = v {
                        for item in items {
                            if !matches!(item, Value::Null) {
                                return Ok(item);
                            }
                        }
                        return Ok(Value::Null);
                    } else {
                        return if matches!(v, Value::Null) {
                            Ok(Value::Null)
                        } else {
                            Ok(v)
                        };
                    }
                }
                for op in &nary.operand {
                    let v = self.eval_expr(op)?;
                    if !matches!(v, Value::Null) {
                        return Ok(v);
                    }
                }
                Ok(Value::Null)
            }
            _ => unreachable!("eval_logical_expr: unexpected expression"),
        }
    }

    /// Evaluate control-flow expressions: `If`, `Case`.
    fn eval_control_flow_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            Expression::If(if_expr) => {
                let cond = self.eval_expr_opt(if_expr.condition.as_deref())?;
                if cond == Value::Boolean(true) {
                    self.eval_expr_opt(if_expr.then_expr.as_deref())
                } else {
                    self.eval_expr_opt(if_expr.else_expr.as_deref())
                }
            }
            Expression::Case(case) => {
                let comparand = match &case.comparand {
                    Some(e) => Some(self.eval_expr(e)?),
                    None => None,
                };
                for item in &case.case_item {
                    let when_val = self.eval_expr_opt(item.when_expr.as_deref())?;
                    let matched = match &comparand {
                        None => when_val == Value::Boolean(true),
                        Some(comp) => equal(comp, &when_val) == Value::Boolean(true),
                    };
                    if matched {
                        return self.eval_expr_opt(item.then_expr.as_deref());
                    }
                }
                self.eval_expr_opt(case.else_expr.as_deref())
            }
            _ => unreachable!("eval_control_flow_expr: unexpected expression"),
        }
    }

    /// Evaluate list construction, aggregate, and set-operation expressions:
    /// `List`, `Exists`, `Count`, `Sum`, `Min`, `Max`, `Avg`, `First`,
    /// `Last`, `Flatten`, `Distinct`, `SingletonFrom`, `Indexer`, `Union`,
    /// `Intersect`, `Except`, `Sort`, `Median`, `Mode`, `Variance`,
    /// `StdDev`, `PopulationVariance`, `PopulationStdDev`, `AllTrue`,
    /// `AnyTrue`, `Repeat`.
    fn eval_list_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            Expression::List(list_expr) => {
                let mut items = Vec::new();
                for e in &list_expr.elements {
                    items.push(self.eval_expr(e)?);
                }
                Ok(Value::List(items))
            }
            Expression::Exists(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::exists(&v)
            }
            Expression::Count(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::count(&v)
            }
            Expression::Sum(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::sum(&v)
            }
            Expression::Min(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::min(&v)
            }
            Expression::Max(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::max(&v)
            }
            Expression::Avg(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::avg(&v)
            }
            Expression::First(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::first(&v)
            }
            Expression::Last(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::last(&v)
            }
            Expression::Flatten(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::flatten(&v)
            }
            Expression::Distinct(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::distinct(&v)
            }
            Expression::SingletonFrom(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::lists::singleton_from(&v)
            }
            Expression::Indexer(bin) => {
                let (list, idx) = self.eval_binary_args(bin)?;
                match (&list, &idx) {
                    (Value::List(items), Value::Integer(i)) => {
                        if *i < 0 {
                            return Ok(Value::Null);
                        }
                        let i = *i as usize; // 0-based
                        Ok(items.get(i).cloned().unwrap_or(Value::Null))
                    }
                    (Value::String(_), Value::Integer(_)) => {
                        super::operators::indexer_str(&list, &idx)
                    }
                    (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                    _ => Err(EvalError::General("Indexer: unsupported types".to_string())),
                }
            }
            Expression::Slice(slice_expr) => {
                let source = self.eval_expr_opt(slice_expr.source.as_deref())?;
                let start = self.eval_expr_opt(slice_expr.start_index.as_deref())?;
                let end = match slice_expr.end_index.as_deref() {
                    Some(expr) => Some(self.eval_expr(expr)?),
                    None => None,
                };
                super::lists::slice(&source, &start, end.as_ref())
            }
            Expression::Union(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => super::lists::union_list(&a, &b),
                    _ => super::intervals::union_interval(&a, &b),
                }
            }
            Expression::Intersect(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => {
                        super::lists::intersect_list(&a, &b)
                    }
                    _ => super::intervals::intersect_interval(&a, &b),
                }
            }
            Expression::Except(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => super::lists::except_list(&a, &b),
                    _ => super::intervals::except_interval(&a, &b),
                }
            }
            Expression::Sort(sort) => {
                let v = self.eval_expr_opt(sort.source.as_deref())?;
                let descending = sort
                    .by
                    .first()
                    .and_then(|item| item.direction.as_ref())
                    .map(|d| *d == crate::elm::SortDirection::Desc)
                    .unwrap_or(false);
                super::lists::sort_list(&v, descending)
            }
            Expression::Median(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::median(&v)
            }
            Expression::Mode(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::mode(&v)
            }
            Expression::Variance(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::variance(&v)
            }
            Expression::StdDev(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::std_dev(&v)
            }
            Expression::PopulationVariance(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::population_variance(&v)
            }
            Expression::PopulationStdDev(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::population_std_dev(&v)
            }
            Expression::AllTrue(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::all_true(&v)
            }
            Expression::AnyTrue(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref())?;
                super::lists::any_true(&v)
            }
            Expression::Repeat(repeat) => {
                // Repeat: deterministic fixpoint iteration.
                // Evaluates element_expr for each element in source (binding via scope),
                // deduplicates, and repeats until the result stabilises or a guard
                // iteration limit is reached.
                let source = self.eval_expr_opt(repeat.source.as_deref())?;
                let Some(element_expr) = repeat.element_expr.as_deref() else {
                    return Ok(source);
                };
                let scope_name = repeat.scope.as_deref().unwrap_or("$this");

                let mut current = match source {
                    Value::List(items) => items,
                    Value::Null => vec![],
                    v => vec![v],
                };

                const MAX_ITER: usize = 100;
                for _ in 0..MAX_ITER {
                    let mut next: Vec<Value> = Vec::new();
                    for item in &current {
                        let mut frame = std::collections::BTreeMap::new();
                        frame.insert(scope_name.to_string(), item.clone());
                        self.push_scope(frame);
                        let result = self.eval_expr(element_expr)?;
                        self.pop_scope();
                        match result {
                            Value::List(sub) => next.extend(sub),
                            Value::Null => {} // nulls excluded
                            v => next.push(v),
                        }
                    }
                    // Deduplicate while preserving order.
                    let mut deduped: Vec<Value> = Vec::new();
                    for v in &next {
                        if !deduped.contains(v) {
                            deduped.push(v.clone());
                        }
                    }
                    if deduped == current {
                        break; // fixpoint reached
                    }
                    current = deduped;
                }
                Ok(Value::List(current))
            }
            _ => unreachable!("eval_list_expr: unexpected expression"),
        }
    }

    /// Evaluate interval operators, including mixed interval/list membership
    /// tests that dispatch on runtime type:
    /// `Interval`, `Start`, `End`, `Width`, `PointFrom`, `Contains`, `In`,
    /// `Overlaps`, `OverlapsBefore`, `OverlapsAfter`, `Meets`, `MeetsBefore`,
    /// `MeetsAfter`, `Includes`, `IncludedIn`, `Starts`, `Ends`, `Collapse`,
    /// `Expand`, `ProperContains`, `ProperIn`, `ProperIncludes`,
    /// `ProperIncludedIn`.
    fn eval_interval_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            Expression::Interval(iv) => {
                let low_closed = iv.low_closed.unwrap_or(true);
                let high_closed = iv.high_closed.unwrap_or(true);
                let low = match &iv.low {
                    Some(e) => Some(Box::new(self.eval_expr(e)?)),
                    None => None,
                };
                let high = match &iv.high {
                    Some(e) => Some(Box::new(self.eval_expr(e)?)),
                    None => None,
                };
                // If a specified (concrete) bound evaluates to null, the interval is null.
                // An unspecified bound (None) means unbounded (±∞), which is valid.
                let low_is_null = low.as_ref().is_some_and(|v| matches!(**v, Value::Null));
                let high_is_null = high.as_ref().is_some_and(|v| matches!(**v, Value::Null));
                // Closed null bound always → null interval.
                // Both bounds null (regardless of open/closed) → null interval.
                let both_null = low_is_null && high_is_null;
                let low_null = (low_closed && low_is_null) || both_null;
                let high_null = (high_closed && high_is_null) || both_null;
                if low_null || high_null {
                    return Ok(Value::Null);
                }
                Ok(Value::Interval {
                    low,
                    high,
                    low_closed,
                    high_closed,
                })
            }
            Expression::Start(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::intervals::start(&v)
            }
            Expression::End(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::intervals::end(&v)
            }
            Expression::Width(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::intervals::width(&v)
            }
            Expression::PointFrom(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::intervals::point_from(&v)
            }
            Expression::Contains(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match &a {
                    Value::Interval { .. } => super::intervals::contains(&a, &b),
                    Value::List(_) => super::lists::list_contains(&a, &b),
                    // Null interval/list = empty, doesn't contain anything
                    Value::Null => Ok(Value::Boolean(false)),
                    _ => Err(EvalError::General(
                        "Contains: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::In(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                match &b {
                    Value::Interval { .. } => super::intervals::in_interval(&a, &b),
                    Value::List(_) => super::lists::in_list(&a, &b),
                    // Null interval/list = empty, element can't be in it
                    Value::Null => Ok(Value::Boolean(false)),
                    _ => Err(EvalError::General(
                        "In: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::Overlaps(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                super::intervals::overlaps(&a, &b)
            }
            Expression::OverlapsBefore(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                super::intervals::overlaps_before(&a, &b)
            }
            Expression::OverlapsAfter(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                super::intervals::overlaps_after(&a, &b)
            }
            Expression::Meets(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                super::intervals::meets(&a, &b)
            }
            Expression::MeetsBefore(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                super::intervals::meets_before(&a, &b)
            }
            Expression::MeetsAfter(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                super::intervals::meets_after(&a, &b)
            }
            Expression::Includes(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                match (&a, &b) {
                    // Includes(list, null-element): null element propagation → Null
                    (Value::List(_), Value::Null) => Ok(Value::Null),
                    // List includes (null container is treated as empty list)
                    (Value::List(_) | Value::Null, Value::List(_) | Value::Null) => {
                        super::lists::list_includes(&a, &b)
                    }
                    _ => super::intervals::includes(&a, &b),
                }
            }
            Expression::IncludedIn(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                match (&a, &b) {
                    // IncludedIn(null-element, list): null element propagation → Null
                    (Value::Null, Value::List(_)) => Ok(Value::Null),
                    // List included in (null container is treated as empty list)
                    (Value::List(_) | Value::Null, Value::List(_) | Value::Null) => {
                        super::lists::list_included_in(&a, &b)
                    }
                    _ => super::intervals::included_in(&a, &b),
                }
            }
            Expression::Starts(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                super::intervals::starts(&a, &b)
            }
            Expression::Ends(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                super::intervals::ends(&a, &b)
            }
            Expression::Collapse(unary) => {
                let v = self.eval_unary_arg(unary)?;
                super::intervals::collapse(&v, None)
            }
            Expression::Expand(bin) => {
                let (source, per) = self.eval_binary_args(bin)?;
                let per_opt = if matches!(per, Value::Null) {
                    None
                } else {
                    Some(per)
                };
                super::intervals::expand(&source, per_opt.as_ref())
            }
            Expression::ProperContains(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match &a {
                    Value::Interval { .. } => super::intervals::proper_contains(&a, &b),
                    Value::List(items) => {
                        if matches!(b, Value::Null) {
                            // ProperContains with null element: check if null is physically in list
                            Ok(Value::Boolean(
                                items.iter().any(|v| matches!(v, Value::Null)),
                            ))
                        } else {
                            super::lists::list_contains(&a, &b)
                        }
                    }
                    // Null list = empty list, doesn't contain anything
                    Value::Null => Ok(Value::Boolean(false)),
                    _ => Err(EvalError::General(
                        "ProperContains: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::ProperIn(bin) => {
                let (a, b) = self.eval_binary_args(bin)?;
                match &b {
                    Value::Interval { .. } => super::intervals::proper_in(&a, &b),
                    Value::List(items) => {
                        if matches!(a, Value::Null) {
                            // ProperIn with null element: check if null is physically in list
                            Ok(Value::Boolean(
                                items.iter().any(|v| matches!(v, Value::Null)),
                            ))
                        } else {
                            super::lists::in_list(&a, &b)
                        }
                    }
                    // Null list = empty list, element can't be in it
                    Value::Null => Ok(Value::Boolean(false)),
                    _ => Err(EvalError::General(
                        "ProperIn: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::ProperIncludes(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                match (&a, &b) {
                    (Value::Interval { .. }, _) | (_, Value::Interval { .. }) => {
                        super::intervals::proper_includes(&a, &b)
                    }
                    // Null container = empty list, can't properly include anything
                    (Value::Null, _) => Ok(Value::Boolean(false)),
                    // List properly includes null (scalar element): check literal null membership
                    (Value::List(items), Value::Null) => Ok(Value::Boolean(
                        items.iter().any(|v| matches!(v, Value::Null)),
                    )),
                    // List properly includes a scalar element = Contains
                    (Value::List(_), _) if !matches!(b, Value::List(_)) => {
                        super::lists::list_contains(&a, &b)
                    }
                    // Proper list inclusion: a includes b AND b doesn't include a
                    (Value::List(_), Value::List(_)) => {
                        let inc_a_b = super::lists::list_includes(&a, &b)?;
                        if inc_a_b != Value::Boolean(true) {
                            return Ok(Value::Boolean(false));
                        }
                        let inc_b_a = super::lists::list_includes(&b, &a)?;
                        Ok(Value::Boolean(inc_b_a != Value::Boolean(true)))
                    }
                    _ => Ok(Value::Null),
                }
            }
            Expression::ProperIncludedIn(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first())?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1))?;
                match (&a, &b) {
                    (Value::Interval { .. }, _) | (_, Value::Interval { .. }) => {
                        super::intervals::proper_included_in(&a, &b)
                    }
                    // Null container = empty list, nothing is properly included in it
                    (_, Value::Null) => Ok(Value::Boolean(false)),
                    // Null element properly included in list: check literal null membership
                    (Value::Null, Value::List(items)) => Ok(Value::Boolean(
                        items.iter().any(|v| matches!(v, Value::Null)),
                    )),
                    // Scalar properly included in list = In (list contains scalar)
                    (_, Value::List(_)) if !matches!(a, Value::List(_)) => {
                        super::lists::in_list(&a, &b)
                    }
                    // Proper list inclusion: b includes a AND a doesn't include b
                    (Value::List(_), Value::List(_)) => {
                        let inc_b_a = super::lists::list_includes(&b, &a)?;
                        if inc_b_a != Value::Boolean(true) {
                            return Ok(Value::Boolean(false));
                        }
                        let inc_a_b = super::lists::list_includes(&a, &b)?;
                        Ok(Value::Boolean(inc_a_b != Value::Boolean(true)))
                    }
                    _ => Ok(Value::Null),
                }
            }
            Expression::Size(unary) => {
                let v = self.eval_unary_arg(unary)?;
                match &v {
                    Value::Null => Ok(Value::Null),
                    Value::List(_) => super::lists::count(&v),
                    Value::Interval { .. } => super::intervals::width(&v),
                    _ => Err(EvalError::General(
                        "Size: expected List or Interval".to_string(),
                    )),
                }
            }
            _ => unreachable!("eval_interval_expr: unexpected expression"),
        }
    }

    /// Evaluate query and retrieve expressions: `Query`, `Retrieve`.
    fn eval_query_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        match expr {
            Expression::Query(q) => {
                let snapshot = self.snapshot_scope();
                super::queries::eval_query(q, &snapshot, &mut |expr, binds| {
                    self.push_scope(binds.clone());
                    let result = self.eval_expr(expr);
                    self.pop_scope();
                    result
                })
            }
            Expression::Retrieve(r) => {
                let raw_type = r.data_type.as_deref().unwrap_or("");
                // Strip Clark-notation namespace prefix `{uri}LocalName` → `LocalName`
                let data_type = if let Some(pos) = raw_type.find('}') {
                    &raw_type[pos + 1..]
                } else {
                    raw_type
                };
                let code_path = r.code_property.as_deref();
                let date_path = r.date_property.as_deref();
                let codes_val = match &r.codes {
                    Some(expr) => Some(self.eval_expr(expr)?),
                    None => None,
                };
                let date_range_val = match &r.date_range {
                    Some(expr) => Some(self.eval_expr(expr)?),
                    None => None,
                };
                let results = self.ctx.retrieve(
                    None,
                    data_type,
                    code_path,
                    codes_val.as_ref(),
                    date_path,
                    date_range_val.as_ref(),
                )?;
                Ok(Value::List(results))
            }
            _ => unreachable!("eval_query_expr: unexpected expression"),
        }
    }

    /// Evaluate terminology expressions:
    /// `Code`, `CodeRef`, `ConceptRef`, `ValueSetRef`, `CodeSystemRef`,
    /// `InValueSet`, `InCodeSystem`, `AnyInValueSet`, `AnyInCodeSystem`.
    ///
    /// `CodeRef` and `ConceptRef` resolve named definitions from the compiled
    /// library into runtime `Value::Code` / `Value::Concept`.  `ValueSetRef`
    /// and `CodeSystemRef` resolve to `Value::String` containing the canonical
    /// URI so they can be consumed by membership-test expressions.
    /// `InValueSet` / `InCodeSystem` delegate membership tests to the runtime
    /// [`EvalContext`].
    fn eval_terminology_expr(&mut self, expr: &Expression) -> Result<Value, EvalError> {
        use super::value::{CqlCode, CqlConcept};

        match expr {
            // ------ Value set / code system reference resolution ------
            Expression::ValueSetRef(vs_ref) => {
                let name = vs_ref.name.as_deref().unwrap_or("");
                let url = self
                    .library
                    .value_sets
                    .as_ref()
                    .and_then(|vs| vs.defs.iter().find(|d| d.name.as_deref() == Some(name)))
                    .and_then(|d| d.id.as_deref())
                    .unwrap_or(name) // fall back to name as-is if not found
                    .to_string();
                Ok(Value::String(url))
            }
            Expression::CodeSystemRef(cs_ref) => {
                let name = cs_ref.name.as_deref().unwrap_or("");
                let url = self
                    .library
                    .code_systems
                    .as_ref()
                    .and_then(|cs| cs.defs.iter().find(|d| d.name.as_deref() == Some(name)))
                    .and_then(|d| d.id.as_deref())
                    .unwrap_or(name)
                    .to_string();
                Ok(Value::String(url))
            }

            // ------ Inline code literal ------
            Expression::Code(code_expr) => {
                let system_url = code_expr
                    .system
                    .as_ref()
                    .and_then(|cs_ref| {
                        let cs_name = cs_ref.name.as_deref()?;
                        self.library
                            .code_systems
                            .as_ref()?
                            .defs
                            .iter()
                            .find(|d| d.name.as_deref() == Some(cs_name))
                            .and_then(|d| d.id.as_deref())
                            .map(str::to_string)
                    })
                    .unwrap_or_default();
                Ok(Value::Code(CqlCode {
                    code: code_expr.code.clone().unwrap_or_default(),
                    system: system_url,
                    display: code_expr.display.clone(),
                    version: None,
                }))
            }

            // ------ Named code / concept lookups ------
            Expression::CodeRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                let code_def = self
                    .library
                    .codes
                    .as_ref()
                    .and_then(|c| c.defs.iter().find(|d| d.name.as_deref() == Some(name)))
                    .ok_or_else(|| {
                        EvalError::General(format!("CodeRef: code '{name}' not found in library"))
                    })?
                    .clone();
                let system_url = code_def
                    .code_system
                    .as_ref()
                    .and_then(|cs_ref| {
                        self.library
                            .code_systems
                            .as_ref()?
                            .defs
                            .iter()
                            .find(|d| d.name.as_deref() == cs_ref.name.as_deref())
                            .and_then(|d| d.id.as_deref())
                            .map(str::to_string)
                    })
                    .unwrap_or_default();
                Ok(Value::Code(CqlCode {
                    code: code_def.id.clone().unwrap_or_default(),
                    system: system_url,
                    display: code_def.display.clone(),
                    version: None,
                }))
            }
            Expression::ConceptRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                let concept_def = self
                    .library
                    .concepts
                    .as_ref()
                    .and_then(|c| c.defs.iter().find(|d| d.name.as_deref() == Some(name)))
                    .ok_or_else(|| {
                        EvalError::General(format!(
                            "ConceptRef: concept '{name}' not found in library"
                        ))
                    })?
                    .clone();
                // Resolve each member code through the library's code definitions.
                let codes: Vec<CqlCode> = concept_def
                    .code
                    .iter()
                    .filter_map(|code_ref| {
                        let code_name = code_ref.name.as_deref()?;
                        let code_def = self
                            .library
                            .codes
                            .as_ref()?
                            .defs
                            .iter()
                            .find(|d| d.name.as_deref() == Some(code_name))?;
                        let system_url = code_def
                            .code_system
                            .as_ref()
                            .and_then(|cs_ref| {
                                self.library
                                    .code_systems
                                    .as_ref()?
                                    .defs
                                    .iter()
                                    .find(|d| d.name.as_deref() == cs_ref.name.as_deref())
                                    .and_then(|d| d.id.as_deref())
                                    .map(str::to_string)
                            })
                            .unwrap_or_default();
                        Some(CqlCode {
                            code: code_def.id.clone().unwrap_or_default(),
                            system: system_url,
                            display: code_def.display.clone(),
                            version: None,
                        })
                    })
                    .collect();
                Ok(Value::Concept(CqlConcept {
                    codes,
                    display: concept_def.display.clone(),
                }))
            }

            // ------ Membership tests ------
            Expression::InValueSet(ivs) => {
                let code_val = self.eval_expr_opt(ivs.code.as_deref())?;
                // valueset_expression takes precedence over valueset per ELM spec.
                let vs_expr = ivs
                    .valueset_expression
                    .as_deref()
                    .or(ivs.valueset.as_deref());
                let vs_url = match vs_expr {
                    Some(e) => match self.eval_expr(e)? {
                        Value::String(s) => s,
                        _ => return Ok(Value::Null),
                    },
                    None => return Ok(Value::Null),
                };
                match code_val {
                    Value::Code(ref c) => {
                        let result = self.ctx.in_valueset(c, &vs_url)?;
                        Ok(Value::Boolean(result))
                    }
                    Value::Null => Ok(Value::Null),
                    _ => Ok(Value::Null),
                }
            }
            Expression::InCodeSystem(ics) => {
                let code_val = self.eval_expr_opt(ics.code.as_deref())?;
                let cs_expr = ics
                    .codesystem_expression
                    .as_deref()
                    .or(ics.codesystem.as_deref());
                let cs_url = match cs_expr {
                    Some(e) => match self.eval_expr(e)? {
                        Value::String(s) => s,
                        _ => return Ok(Value::Null),
                    },
                    None => return Ok(Value::Null),
                };
                match code_val {
                    Value::Code(ref c) => Ok(Value::Boolean(c.system == cs_url)),
                    Value::Null => Ok(Value::Null),
                    _ => Ok(Value::Null),
                }
            }
            Expression::AnyInValueSet(aivs) => {
                let codes_val = self.eval_expr_opt(aivs.codes.as_deref())?;
                let vs_url = match aivs.valueset.as_deref() {
                    Some(e) => match self.eval_expr(e)? {
                        Value::String(s) => s,
                        _ => return Ok(Value::Null),
                    },
                    None => return Ok(Value::Null),
                };
                let codes = match codes_val {
                    Value::List(items) => items,
                    Value::Null => return Ok(Value::Null),
                    _ => return Ok(Value::Null),
                };
                let mut any_match = false;
                for item in &codes {
                    if let Value::Code(c) = item {
                        if self.ctx.in_valueset(c, &vs_url)? {
                            any_match = true;
                            break;
                        }
                    }
                }
                Ok(Value::Boolean(any_match))
            }
            Expression::AnyInCodeSystem(aics) => {
                let codes_val = self.eval_expr_opt(aics.codes.as_deref())?;
                let cs_url = match aics.codesystem.as_deref() {
                    Some(e) => match self.eval_expr(e)? {
                        Value::String(s) => s,
                        _ => return Ok(Value::Null),
                    },
                    None => return Ok(Value::Null),
                };
                let codes = match codes_val {
                    Value::List(items) => items,
                    Value::Null => return Ok(Value::Null),
                    _ => return Ok(Value::Null),
                };
                let any_match = codes
                    .iter()
                    .any(|item| matches!(item, Value::Code(c) if c.system == cs_url));
                Ok(Value::Boolean(any_match))
            }

            _ => unreachable!("eval_terminology_expr: unexpected expression"),
        }
    }

    // Like eval_expr_opt but also returns the last-emitted trace event id.
    fn eval_operand_with_id(
        &mut self,
        expr: Option<&Expression>,
    ) -> Result<(Value, Option<u64>), EvalError> {
        let before = self.next_event_id;
        let val = self.eval_expr_opt(expr)?;
        let id = if self.next_event_id > before {
            Some(self.next_event_id - 1)
        } else {
            None
        };
        Ok((val, id))
    }

    // Evaluate a UnaryExpression operand and return (value, Option<trace_id>).
    fn eval_unary_arg_with_id(
        &mut self,
        unary: &UnaryExpression,
    ) -> Result<(Value, Option<u64>), EvalError> {
        self.eval_operand_with_id(unary.operand.as_deref())
    }

    // Emit a trace event for a binary operator, collecting child event ids.
    #[allow(clippy::too_many_arguments)]
    fn trace_binary(
        &mut self,
        op: &str,
        elm_node_id: Option<String>,
        a: Value,
        id_a: Option<u64>,
        b: Value,
        id_b: Option<u64>,
        result: Value,
    ) -> u64 {
        let children: Vec<u64> = [id_a, id_b].into_iter().flatten().collect();
        self.record_trace(op, vec![a, b], result, children, elm_node_id)
    }

    // Emit a trace event for a unary operator, collecting the child event id.
    fn trace_unary(
        &mut self,
        op: &str,
        elm_node_id: Option<String>,
        v: Value,
        id_v: Option<u64>,
        result: Value,
    ) -> u64 {
        let children: Vec<u64> = id_v.into_iter().collect();
        self.record_trace(op, vec![v], result, children, elm_node_id)
    }

    // ----- Helpers -----

    fn eval_nary_args(&mut self, nary: &NaryExpression) -> Result<Vec<Value>, EvalError> {
        nary.operand.iter().map(|e| self.eval_expr(e)).collect()
    }

    fn eval_binary_args(&mut self, bin: &BinaryExpression) -> Result<(Value, Value), EvalError> {
        let a = bin
            .operand
            .first()
            .map(|e| self.eval_expr(e))
            .unwrap_or(Ok(Value::Null))?;
        let b = bin
            .operand
            .get(1)
            .map(|e| self.eval_expr(e))
            .unwrap_or(Ok(Value::Null))?;
        Ok((a, b))
    }

    fn eval_unary_arg(&mut self, unary: &UnaryExpression) -> Result<Value, EvalError> {
        match &unary.operand {
            Some(e) => self.eval_expr(e),
            None => Ok(Value::Null),
        }
    }

    fn eval_expr_opt(&mut self, expr: Option<&Expression>) -> Result<Value, EvalError> {
        match expr {
            Some(e) => self.eval_expr(e),
            None => Ok(Value::Null),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers (free-standing utilities used by the dispatch)
// ---------------------------------------------------------------------------
// Note: eval_literal, eval_builtin_function, and strip_elm_namespace live in
// super::operators so they can be reused without depending on the Engine.

// ---------------------------------------------------------------------------
// Tests (Task 9.23 — integration tests)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{
        BinaryExpression, Expression, ExpressionDef, ExpressionDefs, ExpressionRef, Library,
        Literal, StatementDef,
    };
    use crate::eval::context::{EvalContextBuilder, FixedClock};
    use crate::eval::value::CqlDateTime;

    fn make_library(name: &str, expr: Expression) -> Library {
        Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some(name.to_string()),
                    expression: Some(Box::new(expr)),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        }
    }

    fn fixed_ctx() -> EvalContext {
        let clock = FixedClock::new(CqlDateTime {
            year: 2024,
            month: Some(1),
            day: Some(1),
            hour: Some(0),
            minute: Some(0),
            second: Some(0),
            millisecond: None,
            offset_seconds: None,
        });
        EvalContextBuilder::new(clock).build()
    }

    fn int_literal(v: i64) -> Expression {
        Expression::Literal(Literal {
            value: Some(v.to_string()),
            value_type: Some("Integer".to_string()),
            ..Default::default()
        })
    }

    fn bool_literal(b: bool) -> Expression {
        Expression::Literal(Literal {
            value: Some(b.to_string()),
            value_type: Some("Boolean".to_string()),
            ..Default::default()
        })
    }

    #[test]
    fn eval_integer_literal() {
        let lib = make_library("X", int_literal(42));
        let result = evaluate_elm(&lib, "X", &fixed_ctx()).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn eval_boolean_literal() {
        let lib = make_library("X", bool_literal(true));
        let result = evaluate_elm(&lib, "X", &fixed_ctx()).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn eval_add() {
        let expr = Expression::Add(BinaryExpression {
            operand: vec![int_literal(3), int_literal(4)],
            ..Default::default()
        });
        let lib = make_library("Add3Plus4", expr);
        assert_eq!(
            evaluate_elm(&lib, "Add3Plus4", &fixed_ctx()).unwrap(),
            Value::Integer(7)
        );
    }

    #[test]
    fn eval_not_found() {
        let lib = Library::default();
        assert!(evaluate_elm(&lib, "Missing", &fixed_ctx()).is_err());
    }

    #[test]
    fn eval_with_trace() {
        let lib = make_library("X", int_literal(5));
        let (val, trace) = evaluate_elm_with_trace(&lib, "X", &fixed_ctx()).unwrap();
        assert_eq!(val, Value::Integer(5));
        assert!(!trace.is_empty());
        assert_eq!(trace[0].op, "Literal");
        assert_eq!(trace[0].output, Value::Integer(5));
    }

    #[test]
    fn eval_expression_ref() {
        let lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![
                    StatementDef::Expression(ExpressionDef {
                        name: Some("Base".to_string()),
                        expression: Some(Box::new(int_literal(99))),
                        ..Default::default()
                    }),
                    StatementDef::Expression(ExpressionDef {
                        name: Some("Ref".to_string()),
                        expression: Some(Box::new(Expression::ExpressionRef(ExpressionRef {
                            name: Some("Base".to_string()),
                            ..Default::default()
                        }))),
                        ..Default::default()
                    }),
                ],
            }),
            ..Default::default()
        };
        assert_eq!(
            evaluate_elm(&lib, "Ref", &fixed_ctx()).unwrap(),
            Value::Integer(99)
        );
    }
    #[test]
    fn eval_add_trace_children() {
        // Spec scenario: evaluate_elm_with_trace on "X = 2 + 3" must contain
        // an Add event with inputs [Integer(2), Integer(3)], output Integer(5),
        // plus child Literal events.
        use crate::elm::{
            BinaryExpression, ElementFields, Expression, ExpressionDef, ExpressionDefs, Library,
            StatementDef,
        };

        let add_expr = Expression::Add(BinaryExpression {
            element: ElementFields {
                local_id: Some("add1".to_string()),
                ..Default::default()
            },
            operand: vec![int_literal(2), int_literal(3)],
            ..Default::default()
        });

        let lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("X".to_string()),
                    expression: Some(Box::new(add_expr)),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };

        let (val, trace) = evaluate_elm_with_trace(&lib, "X", &fixed_ctx()).unwrap();
        assert_eq!(val, Value::Integer(5));

        let add_event = trace
            .iter()
            .find(|e| e.op == "Add")
            .expect("no Add trace event");
        assert_eq!(add_event.inputs, vec![Value::Integer(2), Value::Integer(3)]);
        assert_eq!(add_event.output, Value::Integer(5));
        assert_eq!(add_event.elm_node_id, Some("add1".to_string()));
        // Two child Literal events should exist
        let literal_count = trace.iter().filter(|e| e.op == "Literal").count();
        assert_eq!(literal_count, 2);
        // Children vec on Add event should reference both literals
        assert_eq!(add_event.children.len(), 2);
    }

    #[test]
    fn eval_expression_ref_cross_library_error() {
        // Without an included map, a cross-library ExpressionRef should return
        // LibraryNotFound rather than a generic error.
        let lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("Main".to_string()),
                    expression: Some(Box::new(Expression::ExpressionRef(ExpressionRef {
                        name: Some("InpatientEncounter".to_string()),
                        library_name: Some("Global".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };
        let err = evaluate_elm(&lib, "Main", &fixed_ctx()).unwrap_err();
        assert_eq!(
            err,
            EvalError::LibraryNotFound { alias: "Global".to_string() },
            "expected LibraryNotFound, got: {err}"
        );
    }

    #[test]
    fn eval_function_ref_cross_library_unknown_gives_clear_error() {
        use crate::elm::FunctionRef;
        // Without an included map, a cross-library FunctionRef that is not a
        // builtin should return LibraryNotFound.
        let lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("Main".to_string()),
                    expression: Some(Box::new(Expression::FunctionRef(FunctionRef {
                        name: Some("ToInterval".to_string()),
                        library_name: Some("FHIRHelpers".to_string()),
                        operand: vec![int_literal(1)],
                        ..Default::default()
                    }))),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };
        let err = evaluate_elm(&lib, "Main", &fixed_ctx()).unwrap_err();
        assert_eq!(
            err,
            EvalError::LibraryNotFound { alias: "FHIRHelpers".to_string() },
            "expected LibraryNotFound, got: {err}"
        );
    }

    #[test]
    fn evaluate_elm_with_libraries_resolves_cross_library_expression_ref() {
        // Included library "Helpers" defines expression "Answer" = 42.
        let helpers_lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("Answer".to_string()),
                    expression: Some(Box::new(int_literal(42))),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };
        // Main library has expression "Main" = Helpers."Answer"
        let main_lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("Main".to_string()),
                    expression: Some(Box::new(Expression::ExpressionRef(ExpressionRef {
                        name: Some("Answer".to_string()),
                        library_name: Some("Helpers".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };
        let included: HashMap<String, Library> =
            [("Helpers".to_string(), helpers_lib)].into_iter().collect();
        let val =
            evaluate_elm_with_libraries(&main_lib, &included, "Main", &fixed_ctx()).unwrap();
        assert_eq!(val, Value::Integer(42));
    }

    #[test]
    fn evaluate_elm_with_libraries_returns_library_not_found_for_unknown_alias() {
        let main_lib = Library {
            statements: Some(ExpressionDefs {
                defs: vec![StatementDef::Expression(ExpressionDef {
                    name: Some("Main".to_string()),
                    expression: Some(Box::new(Expression::ExpressionRef(ExpressionRef {
                        name: Some("Answer".to_string()),
                        library_name: Some("Unknown".to_string()),
                        ..Default::default()
                    }))),
                    ..Default::default()
                })],
            }),
            ..Default::default()
        };
        let included: HashMap<String, Library> = HashMap::new();
        let err =
            evaluate_elm_with_libraries(&main_lib, &included, "Main", &fixed_ctx()).unwrap_err();
        assert_eq!(err, EvalError::LibraryNotFound { alias: "Unknown".to_string() });
    }
}
