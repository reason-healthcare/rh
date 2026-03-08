//! ELM evaluation engine entry points.
//!
//! Implements:
//! - [`evaluate_elm`]  — evaluate a named expression from an ELM library
//! - [`evaluate_elm_with_trace`] — same but return step-by-step trace events
//! - [`TraceEvent`]    — per-node evaluation record

use std::collections::BTreeMap;

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
    let bindings = engine.build_initial_bindings();
    engine.eval_expr(&expr, &bindings)
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
    let bindings = engine.build_initial_bindings();
    let value = engine.eval_expr(&expr, &bindings)?;
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
}

impl<'lib, 'ctx> Engine<'lib, 'ctx> {
    fn new(library: &'lib Library, ctx: &'ctx EvalContext) -> Self {
        Self {
            library,
            ctx,
            trace: Vec::new(),
            next_event_id: 1,
        }
    }

    fn find_expression(&self, name: &str) -> Result<Expression, EvalError> {
        if let Some(ref stmts) = self.library.statements {
            for def in &stmts.defs {
                if let StatementDef::Expression(expr_def) = def {
                    if expr_def.name.as_deref() == Some(name) {
                        return expr_def
                            .expression
                            .as_ref()
                            .map(|e| *e.clone())
                            .ok_or_else(|| {
                                EvalError::General(format!("Expression '{name}' has no body"))
                            });
                    }
                }
            }
        }
        Err(EvalError::General(format!(
            "Expression '{name}' not found in library"
        )))
    }

    /// Return true if `name` is declared as a parameter in the library.
    fn is_library_parameter(&self, name: &str) -> bool {
        if let Some(ref params) = self.library.parameters {
            return params.defs.iter().any(|p| p.name.as_deref() == Some(name));
        }
        false
    }

    fn build_initial_bindings(&self) -> BTreeMap<String, Value> {
        let mut bindings = BTreeMap::new();
        // Inject parameter values from context.
        for (k, v) in &self.ctx.parameters {
            bindings.insert(k.clone(), v.clone());
        }
        bindings
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

    fn eval_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
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
                // Check bindings first (for query aliases / let clauses).
                if let Some(v) = bindings.get(name) {
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
                let val = self.eval_expr(&expr, bindings)?;
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
                bindings
                    .get(name)
                    .cloned()
                    .ok_or_else(|| EvalError::General(format!("Operand '{name}' not found")))
            }
            Expression::AliasRef(r) => {
                let name = r.name.as_deref().unwrap_or("");
                bindings
                    .get(name)
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
            | Expression::Coalesce(_) => self.eval_logical_expr(expr, bindings),

            // ----- Comparison -----
            Expression::Equal(bin) => {
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
            Expression::Round(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                round(&v, None)
            }
            Expression::Ln(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let vals = self.eval_nary_args(nary, bindings)?;
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
                let list = self.eval_expr_opt(c.source.as_deref(), bindings)?;
                let sep = match &c.separator {
                    Some(s) => Some(self.eval_expr(s, bindings)?),
                    None => None,
                };
                super::operators::combine(&list, sep.as_ref())
            }
            Expression::Split(s) => {
                let str_val = self.eval_expr_opt(s.string_to_split.as_deref(), bindings)?;
                let sep = self.eval_expr_opt(s.separator.as_deref(), bindings)?;
                super::operators::split(&str_val, &sep)
            }
            Expression::Length(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                match &v {
                    Value::String(_) => super::operators::length_str(&v),
                    Value::List(items) => Ok(Value::Integer(items.len() as i64)),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General("Length: unsupported type".to_string())),
                }
            }
            Expression::Upper(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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
                let (a, id_a) = self.eval_operand_with_id(bin.operand.first(), bindings)?;
                let (b, id_b) = self.eval_operand_with_id(bin.operand.get(1), bindings)?;
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

            // ----- Type Conversions -----
            Expression::ToBoolean(unary) => {
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let (v, id_v) = self.eval_unary_arg_with_id(unary, bindings)?;
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
                let v = self.eval_unary_arg(unary, bindings)?;
                match v {
                    Value::Null => Ok(Value::List(vec![])),
                    Value::List(_) => Ok(v),
                    other => Ok(Value::List(vec![other])),
                }
            }
            Expression::Is(is_expr) => {
                let v = self.eval_expr_opt(is_expr.operand.as_deref(), bindings)?;
                let raw = is_expr.is_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                // "{urn:hl7-org:elm-types:r1}null" → is null check
                if type_name.eq_ignore_ascii_case("null") {
                    return Ok(Value::Boolean(v == Value::Null));
                }
                Ok(super::operators::is_type(&v, type_name))
            }
            Expression::As(as_expr) => {
                let v = self.eval_expr_opt(as_expr.operand.as_deref(), bindings)?;
                let raw = as_expr.as_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                Ok(super::operators::as_type(&v, type_name))
            }
            Expression::Convert(conv_expr) => {
                let v = self.eval_expr_opt(conv_expr.operand.as_deref(), bindings)?;
                let raw = conv_expr.to_type.as_deref().unwrap_or("");
                let type_name = strip_elm_namespace(raw);
                super::operators::convert(&v, type_name)
            }
            Expression::CanConvert(can_conv) => {
                let v = self.eval_expr_opt(can_conv.operand.as_deref(), bindings)?;
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
                let (lhs, rhs) = self.eval_binary_args(binary, bindings)?;
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
            | Expression::Union(_)
            | Expression::Intersect(_)
            | Expression::Except(_) => self.eval_list_expr(expr, bindings),

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
            | Expression::Collapse(_) => self.eval_interval_expr(expr, bindings),

            // ----- Control flow -----
            Expression::If(_) | Expression::Case(_) => self.eval_control_flow_expr(expr, bindings),

            // ----- Query & retrieve -----
            Expression::Query(_) | Expression::Retrieve(_) => self.eval_query_expr(expr, bindings),

            // ----- Tuple / Instance -----
            Expression::Tuple(tup) => {
                let mut fields = BTreeMap::new();
                for element in &tup.elements {
                    if let (Some(name), Some(val_expr)) = (&element.name, &element.value) {
                        fields.insert(name.clone(), self.eval_expr(val_expr, bindings)?);
                    }
                }
                Ok(Value::Tuple(fields))
            }

            // ----- Property -----
            Expression::Property(prop) => {
                let source = match &prop.source {
                    Some(e) => self.eval_expr(e, bindings)?,
                    None => {
                        let scope = prop.scope.as_deref().unwrap_or("$this");
                        bindings.get(scope).cloned().unwrap_or(Value::Null)
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
            Expression::SameAs(tb) => {
                let a = self.eval_expr_opt(tb.operand.first(), bindings)?;
                let b = self.eval_expr_opt(tb.operand.get(1), bindings)?;
                super::operators::same_as(&a, &b, tb.precision.as_deref())
            }
            Expression::SameOrBefore(tb) => {
                let a = self.eval_expr_opt(tb.operand.first(), bindings)?;
                let b = self.eval_expr_opt(tb.operand.get(1), bindings)?;
                super::operators::same_or_before(&a, &b, tb.precision.as_deref())
            }
            Expression::SameOrAfter(tb) => {
                let a = self.eval_expr_opt(tb.operand.first(), bindings)?;
                let b = self.eval_expr_opt(tb.operand.get(1), bindings)?;
                super::operators::same_or_after(&a, &b, tb.precision.as_deref())
            }
            Expression::DurationBetween(tb) => {
                let a = self.eval_expr_opt(tb.operand.first(), bindings)?;
                let b = self.eval_expr_opt(tb.operand.get(1), bindings)?;
                super::operators::duration_between(&a, &b, tb.precision.as_deref().unwrap_or("day"))
            }
            Expression::DifferenceBetween(tb) => {
                let a = self.eval_expr_opt(tb.operand.first(), bindings)?;
                let b = self.eval_expr_opt(tb.operand.get(1), bindings)?;
                super::operators::difference_between(
                    &a,
                    &b,
                    tb.precision.as_deref().unwrap_or("day"),
                )
            }

            // ----- Interval: proper / expand -----
            Expression::OverlapsBefore(_)
            | Expression::OverlapsAfter(_)
            | Expression::Expand(_)
            | Expression::ProperContains(_)
            | Expression::ProperIn(_)
            | Expression::ProperIncludes(_)
            | Expression::ProperIncludedIn(_) => self.eval_interval_expr(expr, bindings),

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
            | Expression::Repeat(_) => self.eval_list_expr(expr, bindings),

            // ----- Terminology -----
            Expression::Code(_)
            | Expression::CodeRef(_)
            | Expression::ConceptRef(_)
            | Expression::ValueSetRef(_)
            | Expression::CodeSystemRef(_)
            | Expression::InValueSet(_)
            | Expression::InCodeSystem(_)
            | Expression::AnyInValueSet(_)
            | Expression::AnyInCodeSystem(_) => self.eval_terminology_expr(expr, bindings),

            // ----- Built-in function dispatch -----
            Expression::FunctionRef(func_ref) => {
                let name = func_ref.name.as_deref().unwrap_or("");
                let mut args = Vec::new();
                for operand in &func_ref.operand {
                    args.push(self.eval_expr(operand, bindings)?);
                }
                eval_builtin_function(name, args)
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
    fn eval_logical_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::And(nary) => {
                let vals = self.eval_nary_args(nary, bindings)?;
                Ok(vals
                    .iter()
                    .fold(Value::Boolean(true), |acc, v| tvl_and(&acc, v)))
            }
            Expression::Or(nary) => {
                let vals = self.eval_nary_args(nary, bindings)?;
                Ok(vals
                    .iter()
                    .fold(Value::Boolean(false), |acc, v| tvl_or(&acc, v)))
            }
            Expression::Not(unary) => {
                let operand = self.eval_unary_arg(unary, bindings)?;
                Ok(tvl_not(&operand))
            }
            Expression::Xor(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                Ok(tvl_xor(&a, &b))
            }
            Expression::Implies(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                Ok(tvl_implies(&a, &b))
            }
            Expression::IsNull(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                Ok(Value::Boolean(matches!(v, Value::Null)))
            }
            Expression::IsTrue(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                Ok(Value::Boolean(v == Value::Boolean(true)))
            }
            Expression::IsFalse(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                Ok(Value::Boolean(v == Value::Boolean(false)))
            }
            Expression::Coalesce(nary) => {
                for op in &nary.operand {
                    let v = self.eval_expr(op, bindings)?;
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
    fn eval_control_flow_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::If(if_expr) => {
                let cond = self.eval_expr_opt(if_expr.condition.as_deref(), bindings)?;
                if cond == Value::Boolean(true) {
                    self.eval_expr_opt(if_expr.then_expr.as_deref(), bindings)
                } else {
                    self.eval_expr_opt(if_expr.else_expr.as_deref(), bindings)
                }
            }
            Expression::Case(case) => {
                let comparand = match &case.comparand {
                    Some(e) => Some(self.eval_expr(e, bindings)?),
                    None => None,
                };
                for item in &case.case_item {
                    let when_val = self.eval_expr_opt(item.when_expr.as_deref(), bindings)?;
                    let matched = match &comparand {
                        None => when_val == Value::Boolean(true),
                        Some(comp) => equal(comp, &when_val) == Value::Boolean(true),
                    };
                    if matched {
                        return self.eval_expr_opt(item.then_expr.as_deref(), bindings);
                    }
                }
                self.eval_expr_opt(case.else_expr.as_deref(), bindings)
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
    fn eval_list_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::List(list_expr) => {
                let mut items = Vec::new();
                for e in &list_expr.elements {
                    items.push(self.eval_expr(e, bindings)?);
                }
                Ok(Value::List(items))
            }
            Expression::Exists(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::exists(&v)
            }
            Expression::Count(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::count(&v)
            }
            Expression::Sum(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::sum(&v)
            }
            Expression::Min(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::min(&v)
            }
            Expression::Max(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::max(&v)
            }
            Expression::Avg(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::avg(&v)
            }
            Expression::First(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::first(&v)
            }
            Expression::Last(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::last(&v)
            }
            Expression::Flatten(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::flatten(&v)
            }
            Expression::Distinct(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::distinct(&v)
            }
            Expression::SingletonFrom(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::lists::singleton_from(&v)
            }
            Expression::Indexer(bin) => {
                let (list, idx) = self.eval_binary_args(bin, bindings)?;
                match (&list, &idx) {
                    (Value::List(items), Value::Integer(i)) => {
                        let i = (*i - 1) as usize; // 1-based
                        Ok(items.get(i).cloned().unwrap_or(Value::Null))
                    }
                    (Value::String(_), Value::Integer(_)) => {
                        super::operators::indexer_str(&list, &idx)
                    }
                    (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                    _ => Err(EvalError::General("Indexer: unsupported types".to_string())),
                }
            }
            Expression::Union(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => super::lists::union_list(&a, &b),
                    _ => super::intervals::union_interval(&a, &b),
                }
            }
            Expression::Intersect(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => {
                        super::lists::intersect_list(&a, &b)
                    }
                    _ => super::intervals::intersect_interval(&a, &b),
                }
            }
            Expression::Except(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match (&a, &b) {
                    (Value::List(_), _) | (_, Value::List(_)) => super::lists::except_list(&a, &b),
                    _ => super::intervals::except_interval(&a, &b),
                }
            }
            Expression::Sort(sort) => {
                let v = self.eval_expr_opt(sort.source.as_deref(), bindings)?;
                let descending = sort
                    .by
                    .first()
                    .and_then(|item| item.direction.as_ref())
                    .map(|d| *d == crate::elm::SortDirection::Desc)
                    .unwrap_or(false);
                super::lists::sort_list(&v, descending)
            }
            Expression::Median(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::median(&v)
            }
            Expression::Mode(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::mode(&v)
            }
            Expression::Variance(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::variance(&v)
            }
            Expression::StdDev(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::std_dev(&v)
            }
            Expression::PopulationVariance(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::population_variance(&v)
            }
            Expression::PopulationStdDev(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::population_std_dev(&v)
            }
            Expression::AllTrue(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::all_true(&v)
            }
            Expression::AnyTrue(agg) => {
                let v = self.eval_expr_opt(agg.source.as_deref(), bindings)?;
                super::lists::any_true(&v)
            }
            Expression::Repeat(repeat) => {
                // Repeat: fixpoint iteration — evaluate source only (stub; full
                // implementation requires per-element scope binding).
                self.eval_expr_opt(repeat.source.as_deref(), bindings)
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
    fn eval_interval_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::Interval(iv) => {
                let low = match &iv.low {
                    Some(e) => Some(Box::new(self.eval_expr(e, bindings)?)),
                    None => None,
                };
                let high = match &iv.high {
                    Some(e) => Some(Box::new(self.eval_expr(e, bindings)?)),
                    None => None,
                };
                Ok(Value::Interval {
                    low,
                    high,
                    low_closed: iv.low_closed.unwrap_or(true),
                    high_closed: iv.high_closed.unwrap_or(true),
                })
            }
            Expression::Start(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::intervals::start(&v)
            }
            Expression::End(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::intervals::end(&v)
            }
            Expression::Width(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::intervals::width(&v)
            }
            Expression::PointFrom(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::intervals::point_from(&v)
            }
            Expression::Contains(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match &a {
                    Value::Interval { .. } => super::intervals::contains(&a, &b),
                    Value::List(_) => super::lists::list_contains(&a, &b),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General(
                        "Contains: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::In(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                match &b {
                    Value::Interval { .. } => super::intervals::in_interval(&a, &b),
                    Value::List(_) => super::lists::in_list(&a, &b),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General(
                        "In: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::Overlaps(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                super::intervals::overlaps(&a, &b)
            }
            Expression::OverlapsBefore(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                super::intervals::overlaps_before(&a, &b)
            }
            Expression::OverlapsAfter(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                super::intervals::overlaps_after(&a, &b)
            }
            Expression::Meets(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                super::intervals::meets(&a, &b)
            }
            Expression::MeetsBefore(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                super::intervals::meets_before(&a, &b)
            }
            Expression::MeetsAfter(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                super::intervals::meets_after(&a, &b)
            }
            Expression::Includes(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                match (&a, &b) {
                    (Value::List(_), Value::List(_)) => super::lists::list_includes(&a, &b),
                    _ => super::intervals::includes(&a, &b),
                }
            }
            Expression::IncludedIn(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                match (&a, &b) {
                    (Value::List(_), Value::List(_)) => super::lists::list_included_in(&a, &b),
                    _ => super::intervals::included_in(&a, &b),
                }
            }
            Expression::Starts(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                super::intervals::starts(&a, &b)
            }
            Expression::Ends(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                super::intervals::ends(&a, &b)
            }
            Expression::Collapse(unary) => {
                let v = self.eval_unary_arg(unary, bindings)?;
                super::intervals::collapse(&v, None)
            }
            Expression::Expand(bin) => {
                let (source, per) = self.eval_binary_args(bin, bindings)?;
                let per_opt = if matches!(per, Value::Null) {
                    None
                } else {
                    Some(per)
                };
                super::intervals::expand(&source, per_opt.as_ref())
            }
            Expression::ProperContains(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match &a {
                    Value::Interval { .. } => super::intervals::proper_contains(&a, &b),
                    Value::List(_) => super::lists::list_contains(&a, &b),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General(
                        "ProperContains: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::ProperIn(bin) => {
                let (a, b) = self.eval_binary_args(bin, bindings)?;
                match &b {
                    Value::Interval { .. } => super::intervals::proper_in(&a, &b),
                    Value::List(_) => super::lists::in_list(&a, &b),
                    Value::Null => Ok(Value::Null),
                    _ => Err(EvalError::General(
                        "ProperIn: expected Interval or List".to_string(),
                    )),
                }
            }
            Expression::ProperIncludes(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                match (&a, &b) {
                    (Value::Interval { .. }, _) | (_, Value::Interval { .. }) => {
                        super::intervals::proper_includes(&a, &b)
                    }
                    (Value::List(_), Value::List(_)) => super::lists::list_includes(&a, &b),
                    _ => Ok(Value::Null),
                }
            }
            Expression::ProperIncludedIn(timed_bin) => {
                let a = self.eval_expr_opt(timed_bin.operand.first(), bindings)?;
                let b = self.eval_expr_opt(timed_bin.operand.get(1), bindings)?;
                match (&a, &b) {
                    (Value::Interval { .. }, _) | (_, Value::Interval { .. }) => {
                        super::intervals::proper_included_in(&a, &b)
                    }
                    (Value::List(_), Value::List(_)) => super::lists::list_included_in(&a, &b),
                    _ => Ok(Value::Null),
                }
            }
            _ => unreachable!("eval_interval_expr: unexpected expression"),
        }
    }

    /// Evaluate query and retrieve expressions: `Query`, `Retrieve`.
    fn eval_query_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::Query(q) => super::queries::eval_query(q, bindings, &mut |expr, binds| {
                self.eval_expr(expr, binds)
            }),
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
                    Some(expr) => Some(self.eval_expr(expr, bindings)?),
                    None => None,
                };
                let date_range_val = match &r.date_range {
                    Some(expr) => Some(self.eval_expr(expr, bindings)?),
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
    fn eval_terminology_expr(
        &mut self,
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
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
                let code_val = self.eval_expr_opt(ivs.code.as_deref(), bindings)?;
                // valueset_expression takes precedence over valueset per ELM spec.
                let vs_expr = ivs
                    .valueset_expression
                    .as_deref()
                    .or(ivs.valueset.as_deref());
                let vs_url = match vs_expr {
                    Some(e) => match self.eval_expr(e, bindings)? {
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
                let code_val = self.eval_expr_opt(ics.code.as_deref(), bindings)?;
                let cs_expr = ics
                    .codesystem_expression
                    .as_deref()
                    .or(ics.codesystem.as_deref());
                let cs_url = match cs_expr {
                    Some(e) => match self.eval_expr(e, bindings)? {
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
                let codes_val = self.eval_expr_opt(aivs.codes.as_deref(), bindings)?;
                let vs_url = match aivs.valueset.as_deref() {
                    Some(e) => match self.eval_expr(e, bindings)? {
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
                let codes_val = self.eval_expr_opt(aics.codes.as_deref(), bindings)?;
                let cs_url = match aics.codesystem.as_deref() {
                    Some(e) => match self.eval_expr(e, bindings)? {
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
        bindings: &BTreeMap<String, Value>,
    ) -> Result<(Value, Option<u64>), EvalError> {
        let before = self.next_event_id;
        let val = self.eval_expr_opt(expr, bindings)?;
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
        bindings: &BTreeMap<String, Value>,
    ) -> Result<(Value, Option<u64>), EvalError> {
        self.eval_operand_with_id(unary.operand.as_deref(), bindings)
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

    fn eval_nary_args(
        &mut self,
        nary: &NaryExpression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Vec<Value>, EvalError> {
        nary.operand
            .iter()
            .map(|e| self.eval_expr(e, bindings))
            .collect()
    }

    fn eval_binary_args(
        &mut self,
        bin: &BinaryExpression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<(Value, Value), EvalError> {
        let a = bin
            .operand
            .first()
            .map(|e| self.eval_expr(e, bindings))
            .unwrap_or(Ok(Value::Null))?;
        let b = bin
            .operand
            .get(1)
            .map(|e| self.eval_expr(e, bindings))
            .unwrap_or(Ok(Value::Null))?;
        Ok((a, b))
    }

    fn eval_unary_arg(
        &mut self,
        unary: &UnaryExpression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match &unary.operand {
            Some(e) => self.eval_expr(e, bindings),
            None => Ok(Value::Null),
        }
    }

    fn eval_expr_opt(
        &mut self,
        expr: Option<&Expression>,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Some(e) => self.eval_expr(e, bindings),
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
}
