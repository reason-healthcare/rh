//! Core FHIRPath expression evaluator

use crate::ast::*;
use crate::error::*;
use crate::evaluator::{
    core::context::EvaluationContext,
    functions::FunctionRegistry,
    operations::{
        arithmetic::ArithmeticEvaluator, collection::CollectionEvaluator,
        comparison::ComparisonEvaluator,
    },
    types::{operations::TypeEvaluator, FhirPathValue},
};
use rust_decimal::Decimal;

fn is_empty_sort_key(value: &FhirPathValue) -> bool {
    matches!(value, FhirPathValue::Empty)
        || matches!(
            value,
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty()
        )
}

/// FHIRPath expression evaluator
pub struct FhirPathEvaluator {
    /// Built-in functions registry
    function_registry: FunctionRegistry,
}

impl FhirPathEvaluator {
    /// Create a new evaluator with built-in functions
    pub fn new() -> Self {
        Self {
            function_registry: FunctionRegistry::new(),
        }
    }

    /// Evaluate a FHIRPath expression against a FHIR resource
    pub fn evaluate(
        &self,
        expression: &FhirPathExpression,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let result = self.evaluate_expression(&expression.root, context)?;
        Ok(strip_fhir_primitive(result))
    }

    /// Evaluate an expression against a context
    fn evaluate_expression(
        &self,
        expression: &Expression,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match expression {
            Expression::Term(term) => self.evaluate_term(term, context),
            Expression::Invocation { left, invocation } => {
                let left_result = self.evaluate_expression(left, context)?;
                // FhirPrimitive carries _fieldName extension data only needed by
                // extension().  For all other invocations, strip to the inner value
                // so the rest of the evaluator doesn't need to handle the wrapper.
                let is_extension_call = matches!(
                    invocation,
                    Invocation::Function { name, .. } if name == "extension"
                );
                let effective_left = if is_extension_call {
                    left_result
                } else {
                    strip_fhir_primitive(left_result)
                };
                self.evaluate_invocation(&effective_left, invocation, context)
            }
            Expression::Indexer { left, index } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let index_result = self.evaluate_expression(index, context)?;
                CollectionEvaluator::evaluate_indexer(&left_result, &index_result)
            }
            Expression::Union { left, right } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                CollectionEvaluator::evaluate_union(&left_result, &right_result)
            }
            Expression::And { left, right } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_and(&left_result, &right_result)
            }
            Expression::Or {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_or(&left_result, operator, &right_result)
            }
            Expression::Implies { left, right } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_implies(&left_result, &right_result)
            }
            Expression::Equality {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_equality(&left_result, operator, &right_result)
            }
            Expression::Inequality {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_inequality(&left_result, operator, &right_result)
            }
            Expression::Membership {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ComparisonEvaluator::evaluate_membership(&left_result, operator, &right_result)
            }
            Expression::Additive {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ArithmeticEvaluator::evaluate_additive(&left_result, operator, &right_result)
            }
            Expression::Multiplicative {
                left,
                operator,
                right,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                let right_result = strip_fhir_primitive(self.evaluate_expression(right, context)?);
                ArithmeticEvaluator::evaluate_multiplicative(&left_result, operator, &right_result)
            }
            Expression::Polarity { operator, operand } => {
                let operand_result =
                    strip_fhir_primitive(self.evaluate_expression(operand, context)?);
                ArithmeticEvaluator::evaluate_polarity(operator, &operand_result)
            }
            Expression::Type {
                left,
                operator,
                type_specifier,
            } => {
                let left_result = strip_fhir_primitive(self.evaluate_expression(left, context)?);
                TypeEvaluator::evaluate_type_operation(&left_result, operator, type_specifier)
            }
        }
    }

    /// Evaluate a term
    fn evaluate_term(
        &self,
        term: &Term,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match term {
            Term::Literal(literal) => self.evaluate_literal(literal),
            Term::Invocation(invocation) => {
                let current_value = FhirPathValue::from_json(&context.current);
                self.evaluate_invocation(&current_value, invocation, context)
            }
            Term::ExternalConstant(name) => {
                if let Some(value) = context.constants.get(name) {
                    Ok(value.clone())
                } else {
                    match name.as_str() {
                        "context" => Ok(FhirPathValue::from_json(&context.current)),
                        "resource" => Ok(FhirPathValue::from_json(&context.root)),
                        _ => {
                            // Try extension variables
                            let extension_registry = crate::extensions::ExtensionRegistry::new();
                            match extension_registry.resolve_variable(name, context)? {
                                Some(value) => Ok(value),
                                None => Ok(FhirPathValue::Empty),
                            }
                        }
                    }
                }
            }
            Term::Parenthesized(expr) => self.evaluate_expression(expr, context),
        }
    }

    /// Evaluate a literal value
    fn evaluate_literal(&self, literal: &Literal) -> FhirPathResult<FhirPathValue> {
        match literal {
            Literal::Boolean(b) => Ok(FhirPathValue::Boolean(*b)),
            Literal::String(s) => Ok(FhirPathValue::String(s.clone())),
            Literal::Number(n) => Ok(FhirPathValue::Number(*n)),
            Literal::Integer(i) => Ok(FhirPathValue::Integer(*i)),
            Literal::LongNumber(i) => Ok(FhirPathValue::Long(*i)),
            Literal::Date(d) => Ok(FhirPathValue::Date(d.clone())),
            Literal::DateTime(dt) => Ok(FhirPathValue::DateTime(dt.clone())),
            Literal::Time(t) => Ok(FhirPathValue::Time(
                t.strip_prefix('T').unwrap_or(t).to_string(),
            )),
            Literal::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: *value,
                unit: unit.clone(),
            }),
            Literal::DateTimePrecision(precision) => {
                Ok(FhirPathValue::DateTimePrecision(precision.clone()))
            }
            Literal::Null => Ok(FhirPathValue::Empty),
        }
    }

    /// Evaluate an invocation (member access or function call)
    fn evaluate_invocation(
        &self,
        target: &FhirPathValue,
        invocation: &Invocation,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match invocation {
            Invocation::Member(name) => self.evaluate_member_access(target, name, context),
            Invocation::Function { name, parameters } => {
                // Special handling for functions that need to evaluate expressions themselves
                match name.as_str() {
                    "where" => self.evaluate_where_function(target, parameters, context),
                    "select" => self.evaluate_select_function(target, parameters, context),
                    "repeat" => self.evaluate_repeat_function(target, parameters, context),
                    "ofType" => self.evaluate_of_type_function(target, parameters, context),
                    "is" => self.evaluate_is_function(target, parameters, context),
                    "as" => self.evaluate_as_function(target, parameters, context),
                    "trace" => self.evaluate_trace_function(target, parameters, context),
                    "aggregate" => self.evaluate_aggregate_function(target, parameters, context),
                    "iif" => self.evaluate_iif_lazy(target, parameters, context),
                    "sort" if !parameters.is_empty() => {
                        self.evaluate_sort_by_expression(target, parameters, context)
                    }
                    "exists" if !parameters.is_empty() => {
                        self.evaluate_exists_with_criterion(target, parameters, context)
                    }
                    "all" if !parameters.is_empty() => {
                        self.evaluate_all_with_criterion(target, parameters, context)
                    }
                    "first" | "last" | "tail" => {
                        let check = context.check_ordered_functions;
                        let param_values: Result<Vec<_>, _> = parameters
                            .iter()
                            .map(|p| self.evaluate_expression(p, context))
                            .collect();
                        let param_values = param_values?;
                        self.evaluate_ordered_collection_fn(target, name, &param_values, check)
                    }
                    "skip" => {
                        let check = context.check_ordered_functions;
                        let param_values: Result<Vec<_>, _> = parameters
                            .iter()
                            .map(|p| self.evaluate_expression(p, context))
                            .collect();
                        let param_values = param_values?;
                        self.evaluate_ordered_collection_fn(target, name, &param_values, check)
                    }
                    _ => {
                        // Regular functions: evaluate parameters first
                        let param_values: Result<Vec<_>, _> = parameters
                            .iter()
                            .map(|p| self.evaluate_expression(p, context))
                            .collect();
                        let param_values = param_values?;
                        self.evaluate_function_call(target, name, &param_values)
                    }
                }
            }
            Invocation::This => {
                // Return the stored $this value, or fall back to context current
                if let Some(this_value) = &context.this_value {
                    Ok(this_value.clone())
                } else {
                    Ok(FhirPathValue::Object(context.current.clone()))
                }
            }
            Invocation::Index => {
                if let Some(idx) = context.index_value {
                    Ok(FhirPathValue::Integer(idx))
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            Invocation::Total => {
                if let Some(total) = &context.total_value {
                    Ok(total.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Evaluate member access on a value
    #[allow(clippy::only_used_in_recursion)]
    fn evaluate_member_access(
        &self,
        target: &FhirPathValue,
        member: &str,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Object(obj) | FhirPathValue::TypedObject { value: obj, .. } => {
                // Extract resource type for metadata lookup
                let resource_type = obj
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                // Check if member is the resourceType - if so, return the object itself
                // This enables expressions like Patient.id to work on Patient resources
                if let Some(ref resource_type_str) = resource_type {
                    if resource_type_str == member {
                        // The member matches the resourceType, return the object itself
                        return Ok(target.clone());
                    }
                }

                if let Some(value) = obj.get(member) {
                    // In strict mode, reject choice-type suffixed keys (e.g.
                    // `valueQuantity`); the canonical base name (`value`) must
                    // be used instead per the FHIRPath spec.
                    if context.strict_mode {
                        if let Some(base) = choice_type_base(resource_type.as_deref(), member) {
                            return Err(FhirPathError::EvaluationError {
                                message: format!(
                                    "Invalid choice-type access: use '{}[x]' instead of '{}'",
                                    base, member
                                ),
                            });
                        }
                    }

                    // Check for a `_fieldName` sibling carrying extension data for primitives
                    let underscore_key = format!("_{member}");
                    let ext_sibling = obj
                        .get(&underscore_key)
                        .and_then(|v| v.as_object())
                        .cloned();

                    // Apply FHIR type metadata if we have resource type information
                    let typed = if let Some(ref rt) = resource_type {
                        crate::evaluator::metadata::apply_fhir_typing(value, rt, member)
                    } else {
                        FhirPathValue::from_json(value)
                    };

                    // Wrap in FhirPrimitive if there's extension sibling data
                    if let Some(ext_map) = ext_sibling {
                        // Only wrap if the value is a primitive (not a collection/object)
                        if !matches!(
                            typed,
                            FhirPathValue::Object(_) | FhirPathValue::Collection(_)
                        ) {
                            return Ok(FhirPathValue::FhirPrimitive {
                                inner: Box::new(typed),
                                extensions: ext_map,
                            });
                        }
                    }
                    Ok(typed)
                } else {
                    // Check for FHIR choice type polymorphic access (e.g., value[x])
                    if let Some(obj_map) = obj.as_object() {
                        for (key, value) in obj_map {
                            if key.starts_with(member)
                                && key.len() > member.len()
                                && key
                                    .chars()
                                    .nth(member.len())
                                    .is_some_and(|c| c.is_uppercase())
                            {
                                if context.strict_mode {
                                    return Err(FhirPathError::EvaluationError {
                                        message: format!(
                                            "Invalid choice-type access: use '{}[x]' instead of '{}' in strict mode",
                                            member, key
                                        ),
                                    });
                                }
                                let suffix = &key[member.len()..];
                                if let Some(typed) = construct_typed_choice_value(suffix, value) {
                                    return Ok(typed);
                                }
                                if let Some(ref rt) = resource_type {
                                    return Ok(crate::evaluator::metadata::apply_fhir_typing(
                                        value, rt, key,
                                    ));
                                }
                                return Ok(FhirPathValue::from_json(value));
                            }
                        }
                    }
                    Ok(FhirPathValue::Empty)
                }
            }
            FhirPathValue::FhirPrimitive { inner, .. } => {
                // Delegate member access to the inner value
                self.evaluate_member_access(inner, member, context)
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                let mut result_items = Vec::new();
                for item in items {
                    let member_result = self.evaluate_member_access(item, member, context)?;
                    match member_result {
                        FhirPathValue::Collection(mut nested_items)
                        | FhirPathValue::UnorderedCollection(mut nested_items) => {
                            result_items.append(&mut nested_items);
                        }
                        FhirPathValue::Empty => {}
                        value => {
                            result_items.push(value);
                        }
                    }
                }

                if result_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else if result_items.len() == 1 {
                    Ok(result_items.into_iter().next().unwrap())
                } else {
                    Ok(FhirPathValue::Collection(result_items))
                }
            }
            // `Quantity.value` / `.unit` member access per FHIR data model —
            // matches what the suite expects after choice-type access
            // promotes `valueQuantity` to a typed Quantity.
            FhirPathValue::Quantity { value, unit } => match member {
                "value" => Ok(FhirPathValue::Number(*value)),
                "unit" => Ok(unit
                    .as_ref()
                    .map(|u| FhirPathValue::String(u.clone()))
                    .unwrap_or(FhirPathValue::Empty)),
                _ => Ok(FhirPathValue::Empty),
            },
            _ => Ok(FhirPathValue::Empty),
        }
    }

    /// Evaluate order-sensitive collection functions (first, last, tail, skip)
    /// with UnorderedCollection checking.
    fn evaluate_ordered_collection_fn(
        &self,
        target: &FhirPathValue,
        name: &str,
        params: &[FhirPathValue],
        check_ordered: bool,
    ) -> FhirPathResult<FhirPathValue> {
        use crate::evaluator::operations::collection::CollectionEvaluator;

        // Normalize UnorderedCollection → Collection for order-sensitive ops,
        // but error if check_ordered is enabled and target is unordered.
        let normalized = match target {
            FhirPathValue::UnorderedCollection(_) if check_ordered => {
                return Err(FhirPathError::EvaluationError {
                    message: format!(
                        "Cannot apply order-dependent function '{}' to unordered collection",
                        name
                    ),
                });
            }
            FhirPathValue::UnorderedCollection(items) => FhirPathValue::Collection(items.clone()),
            other => other.clone(),
        };

        match name {
            "first" => CollectionEvaluator::first(&normalized, check_ordered),
            "last" => CollectionEvaluator::last(&normalized, check_ordered),
            "tail" => CollectionEvaluator::tail(&normalized, check_ordered),
            "skip" => {
                let count = match params.first() {
                    Some(FhirPathValue::Integer(n)) => *n,
                    Some(_) => {
                        return Err(FhirPathError::InvalidOperation {
                            message: "skip() count parameter must be an integer".to_string(),
                        })
                    }
                    None => {
                        return Err(FhirPathError::InvalidOperation {
                            message: "skip() requires exactly one parameter (count)".to_string(),
                        })
                    }
                };
                CollectionEvaluator::skip(&normalized, count, check_ordered)
            }
            _ => Err(FhirPathError::FunctionError {
                message: format!("Unknown order-dependent function: {name}"),
            }),
        }
    }

    /// Evaluate function call
    fn evaluate_function_call(
        &self,
        target: &FhirPathValue,
        name: &str,
        parameters: &[FhirPathValue],
    ) -> FhirPathResult<FhirPathValue> {
        if let Some(func) = self.function_registry.get_function(name) {
            func(target, parameters)
        } else {
            Err(FhirPathError::FunctionError {
                message: format!("Unknown function: {name}"),
            })
        }
    }

    /// Evaluate where function that filters collections based on boolean criteria
    fn evaluate_where_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "where() function requires exactly one parameter".to_string(),
            });
        }

        let criteria_expr = &parameters[0];

        match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                let mut filtered_items = Vec::new();

                for (idx, item) in items.iter().enumerate() {
                    // Create new context with current item as $this and $index
                    let item_context = context.with_this_and_index(item.clone(), idx as i64);

                    // Evaluate the criteria expression in the item context
                    let criteria_result = self.evaluate_expression(criteria_expr, &item_context)?;

                    // If criteria evaluates to true, include the item
                    if criteria_result.is_truthy() {
                        filtered_items.push(item.clone());
                    }
                }

                if filtered_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else if filtered_items.len() == 1 {
                    Ok(filtered_items.into_iter().next().unwrap())
                } else {
                    Ok(FhirPathValue::Collection(filtered_items))
                }
            }
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            _ => {
                // For single values, treat as single-item collection
                let item_context = context.with_this_value(target.clone());

                let criteria_result = self.evaluate_expression(criteria_expr, &item_context)?;
                if criteria_result.is_truthy() {
                    Ok(target.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// iif(criterion, true-result[, otherwise-result]) with lazy evaluation.
    /// Evaluates criterion first; only evaluates the selected branch.
    fn evaluate_iif_lazy(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() < 2 || parameters.len() > 3 {
            return Err(FhirPathError::FunctionError {
                message: "iif() requires 2 or 3 parameters".to_string(),
            });
        }

        // iif() on a multi-element collection is an error per FHIRPath spec.
        // Single-element collections are unwrapped; empty collections become Empty.
        // However, non-collection values (objects, booleans, etc.) are fine.
        if let FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) = target
        {
            if items.len() > 1 {
                return Err(FhirPathError::EvaluationError {
                    message: "iif() can only be called on a singleton or empty collection"
                        .to_string(),
                });
            }
        }

        // Set the receiver as the evaluation focus (both $this and current).
        // This matches FHIRPath semantics: x.iif(criterion, t, f) evaluates
        // criterion/t/f with x as the focus. When iif has no explicit receiver
        // the target is the root context object, so focus = root (correct).
        let iif_ctx = if context.index_value.is_some() {
            context.with_this_value(target.clone())
        } else {
            context.with_this_and_index(target.clone(), 0)
        };
        let criterion_val = self.evaluate_expression(&parameters[0], &iif_ctx)?;

        let effective = match &criterion_val {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() == 1 =>
            {
                items[0].clone()
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty() =>
            {
                FhirPathValue::Empty
            }
            FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_) => {
                return Err(FhirPathError::EvaluationError {
                    message:
                        "iif() criterion must be a single Boolean; got a multi-item collection"
                            .to_string(),
                });
            }
            other => other.clone(),
        };

        let is_truthy = match effective {
            FhirPathValue::Boolean(b) => b,
            FhirPathValue::TypedBoolean { value, .. } => value,
            FhirPathValue::Empty => false,
            other => {
                return Err(FhirPathError::TypeError {
                    message: format!("iif() criterion must be Boolean, got {other:?}"),
                });
            }
        };

        // Lazy: only evaluate the selected branch.
        if is_truthy {
            self.evaluate_expression(&parameters[1], &iif_ctx)
        } else if let Some(otherwise) = parameters.get(2) {
            self.evaluate_expression(otherwise, &iif_ctx)
        } else {
            Ok(FhirPathValue::Empty)
        }
    }

    /// sort(key-expression...) — sort by one or more key expressions.
    /// A key expression wrapped in unary `-` means sort DESCENDING by the inner
    /// expression (rather than actually negating the value). This allows
    /// `sort(-$this)` for descending numeric sorts and `sort(-family)` for
    /// descending string sorts without raising a type error.
    fn evaluate_sort_by_expression(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        // Order-sensitive: error on UnorderedCollection when check_ordered_functions
        if matches!(target, FhirPathValue::UnorderedCollection(_))
            && context.check_ordered_functions
        {
            return Err(FhirPathError::EvaluationError {
                message: "Cannot apply order-dependent function to unordered collection"
                    .to_string(),
            });
        }

        let items: Vec<FhirPathValue> = match target {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(v) | FhirPathValue::UnorderedCollection(v) => v.clone(),
            other => return Ok(other.clone()),
        };

        // For each key expression, determine direction and the inner expression.
        let key_specs: Vec<(bool, &Expression)> = parameters
            .iter()
            .map(|p| match p {
                Expression::Invocation {
                    left,
                    invocation: Invocation::Member(direction),
                } if direction == "asc" => (false, left.as_ref()),
                Expression::Invocation {
                    left,
                    invocation: Invocation::Member(direction),
                } if direction == "desc" => (true, left.as_ref()),
                Expression::Polarity {
                    operator: PolarityOperator::Minus,
                    operand,
                } => (true, operand.as_ref()),
                other => (false, other),
            })
            .collect();

        // Build (item, key_tuple) pairs
        let mut keyed: Vec<(FhirPathValue, Vec<(bool, FhirPathValue)>)> = Vec::new();
        for (idx, item) in items.iter().enumerate() {
            let item_ctx = context.with_this_and_index(item.clone(), idx as i64);
            let mut keys = Vec::new();
            for (descending, expr) in &key_specs {
                let key = self.evaluate_expression(expr, &item_ctx)?;
                keys.push((*descending, key));
            }
            keyed.push((item.clone(), keys));
        }

        keyed.sort_by(|(_, ka), (_, kb)| {
            for ((desc, a), (_, b)) in ka.iter().zip(kb.iter()) {
                let ord =
                    crate::evaluator::functions::collection_functions::compare_for_sort_pub(a, b);
                if ord != std::cmp::Ordering::Equal {
                    if is_empty_sort_key(a) || is_empty_sort_key(b) {
                        return ord;
                    }
                    return if *desc { ord.reverse() } else { ord };
                }
            }
            std::cmp::Ordering::Equal
        });

        let sorted: Vec<FhirPathValue> = keyed.into_iter().map(|(item, _)| item).collect();
        if sorted.is_empty() {
            Ok(FhirPathValue::Empty)
        } else if sorted.len() == 1 {
            Ok(sorted.into_iter().next().unwrap())
        } else {
            Ok(FhirPathValue::Collection(sorted))
        }
    }

    /// exists(criterion) — returns true if any element satisfies the criterion.
    fn evaluate_exists_with_criterion(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "exists() accepts at most one parameter (criterion)".to_string(),
            });
        }
        let criterion = &parameters[0];
        let items: Vec<&FhirPathValue> = match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.iter().collect()
            }
            FhirPathValue::Empty => return Ok(FhirPathValue::Boolean(false)),
            v => vec![v],
        };
        for (idx, item) in items.iter().enumerate() {
            let item_ctx = context.with_this_and_index((*item).clone(), idx as i64);
            let result = self.evaluate_expression(criterion, &item_ctx)?;
            if result.is_truthy() {
                return Ok(FhirPathValue::Boolean(true));
            }
        }
        Ok(FhirPathValue::Boolean(false))
    }

    /// all(criterion) — returns true if every element satisfies the criterion.
    fn evaluate_all_with_criterion(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "all() requires exactly one parameter (criterion)".to_string(),
            });
        }
        let criterion = &parameters[0];
        let items: Vec<&FhirPathValue> = match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.iter().collect()
            }
            FhirPathValue::Empty => return Ok(FhirPathValue::Boolean(true)),
            v => vec![v],
        };
        for (idx, item) in items.iter().enumerate() {
            let item_ctx = context.with_this_and_index((*item).clone(), idx as i64);
            let result = self.evaluate_expression(criterion, &item_ctx)?;
            if !result.is_truthy() {
                return Ok(FhirPathValue::Boolean(false));
            }
        }
        Ok(FhirPathValue::Boolean(true))
    }

    /// Evaluate select function that transforms collection items using projection expressions
    fn evaluate_select_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "select() function requires exactly one parameter".to_string(),
            });
        }

        let projection_expr = &parameters[0];

        match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                let mut selected_items = Vec::new();

                for (idx, item) in items.iter().enumerate() {
                    let item_context = context.with_this_and_index(item.clone(), idx as i64);

                    let projection_result =
                        self.evaluate_expression(projection_expr, &item_context)?;

                    match projection_result {
                        FhirPathValue::Collection(mut items)
                        | FhirPathValue::UnorderedCollection(mut items) => {
                            selected_items.append(&mut items);
                        }
                        FhirPathValue::Empty => {}
                        value => {
                            selected_items.push(value);
                        }
                    }
                }

                if selected_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else if selected_items.len() == 1 {
                    Ok(selected_items.into_iter().next().unwrap())
                } else {
                    Ok(FhirPathValue::Collection(selected_items))
                }
            }
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            _ => {
                // For single values, treat as single-item collection
                let item_context = context.with_this_value(target.clone());

                self.evaluate_expression(projection_expr, &item_context)
            }
        }
    }

    /// Evaluate trace function that logs a collection with a name and returns input unchanged
    /// Signature: trace(name : String [, projection: Expression]) : collection
    fn evaluate_trace_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.is_empty() || parameters.len() > 2 {
            return Err(FhirPathError::FunctionError {
                message: "trace() function requires 1 or 2 parameters (name, [projection])"
                    .to_string(),
            });
        }

        // Evaluate the name parameter
        let name_result = self.evaluate_expression(&parameters[0], context)?;
        let name = match name_result {
            FhirPathValue::String(s) => s,
            _ => {
                return Err(FhirPathError::FunctionError {
                    message: "trace() name parameter must evaluate to a string".to_string(),
                })
            }
        };

        // If projection parameter is provided, evaluate it on the target
        // Otherwise, log the target itself
        let value_to_log = if parameters.len() == 2 {
            let projection_expr = &parameters[1];

            match target {
                FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                    let mut projected_items = Vec::new();

                    for (idx, item) in items.iter().enumerate() {
                        let item_context = context.with_this_and_index(item.clone(), idx as i64);
                        let projection_result =
                            self.evaluate_expression(projection_expr, &item_context)?;

                        match projection_result {
                            FhirPathValue::Collection(mut items)
                            | FhirPathValue::UnorderedCollection(mut items) => {
                                projected_items.append(&mut items);
                            }
                            FhirPathValue::Empty => {}
                            value => {
                                projected_items.push(value);
                            }
                        }
                    }

                    if projected_items.is_empty() {
                        FhirPathValue::Empty
                    } else if projected_items.len() == 1 {
                        projected_items.into_iter().next().unwrap()
                    } else {
                        FhirPathValue::Collection(projected_items)
                    }
                }
                FhirPathValue::Empty => FhirPathValue::Empty,
                _ => {
                    let item_context = context.with_this_value(target.clone());
                    self.evaluate_expression(projection_expr, &item_context)?
                }
            }
        } else {
            target.clone()
        };

        if std::env::var_os("RH_FHIRPATH_TRACE_STDERR").is_some() {
            eprintln!("[TRACE:{name}] {value_to_log:?}");
        }

        context.add_trace_log(name, format!("{value_to_log:?}"));

        // Return the original input unchanged
        Ok(target.clone())
    }

    /// Evaluate repeat function that recursively applies a projection expression
    /// until no new items are found (transitive closure)
    fn evaluate_repeat_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "repeat() function requires exactly one parameter".to_string(),
            });
        }

        let projection_expr = &parameters[0];
        let mut all_results = Vec::new();
        let mut current_collection = vec![];

        // Initialize the current collection with the target
        match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                current_collection.extend(items.clone());
            }
            FhirPathValue::Empty => {
                return Ok(FhirPathValue::Empty);
            }
            value => {
                current_collection.push(value.clone());
            }
        }

        // Per FHIRPath spec, repeat() returns only the projected items (not the
        // initial input). We keep the initial collection in `current_collection`
        // to start projecting, but `all_results` starts empty.
        let mut seen: Vec<FhirPathValue> = current_collection.clone();

        loop {
            let mut next_collection = Vec::new();

            // Apply the projection expression to each item in the current collection
            for (idx, item) in current_collection.iter().enumerate() {
                let item_context = context.with_this_and_index(item.clone(), idx as i64);

                // Evaluate the projection expression in the item context
                let projection_result = self.evaluate_expression(projection_expr, &item_context)?;

                // Collect new items from the projection result
                match projection_result {
                    FhirPathValue::Collection(items)
                    | FhirPathValue::UnorderedCollection(items) => {
                        for new_item in items {
                            if !seen
                                .iter()
                                .any(|e| FhirPathValue::equals_static(e, &new_item))
                            {
                                seen.push(new_item.clone());
                                all_results.push(new_item.clone());
                                next_collection.push(new_item.clone());
                            }
                        }
                    }
                    FhirPathValue::Empty => {}
                    value => {
                        if !seen.iter().any(|e| FhirPathValue::equals_static(e, &value)) {
                            seen.push(value.clone());
                            all_results.push(value.clone());
                            next_collection.push(value.clone());
                        }
                    }
                }
            }

            // If no new items were found, we're done
            if next_collection.is_empty() {
                break;
            }

            // Continue with the new collection
            current_collection = next_collection;
        }

        // Return the results as a collection (repeat always returns a collection)
        if all_results.is_empty() {
            Ok(FhirPathValue::Empty)
        } else {
            Ok(FhirPathValue::Collection(all_results))
        }
    }

    /// Evaluate ofType function that filters collections by type specifier
    fn evaluate_of_type_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "ofType() function requires exactly one parameter (type specifier)"
                    .to_string(),
            });
        }

        let type_name = self.resolve_type_argument(&parameters[0], context, "ofType")?;
        let type_specifier = TypeSpecifier {
            qualified_name: vec![type_name],
        };
        TypeEvaluator::evaluate_of_type(target, &type_specifier)
    }

    fn evaluate_is_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "is() function requires exactly one parameter (type specifier)"
                    .to_string(),
            });
        }
        let type_name = self.resolve_type_argument(&parameters[0], context, "is")?;
        let type_specifier = TypeSpecifier {
            qualified_name: vec![type_name],
        };
        TypeEvaluator::evaluate_type_operation(target, &TypeOperator::Is, &type_specifier)
    }

    fn evaluate_as_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() != 1 {
            return Err(FhirPathError::FunctionError {
                message: "as() function requires exactly one parameter (type specifier)"
                    .to_string(),
            });
        }
        let type_name = self.resolve_type_argument(&parameters[0], context, "as")?;
        let type_specifier = TypeSpecifier {
            qualified_name: vec![type_name],
        };
        TypeEvaluator::evaluate_type_operation(target, &TypeOperator::As, &type_specifier)
    }

    /// Resolve the type-name argument for `is()` / `as()` / `ofType()`.
    /// Accepts a simple identifier, a qualified chain like `System.Boolean`,
    /// or a String literal.
    fn resolve_type_argument(
        &self,
        param: &Expression,
        context: &EvaluationContext,
        fn_name: &str,
    ) -> FhirPathResult<String> {
        if let Some(qualified) = extract_qualified_type_name(param) {
            return Ok(qualified);
        }
        match self.evaluate_expression(param, context)? {
            FhirPathValue::String(s) => Ok(s),
            _ => Err(FhirPathError::FunctionError {
                message: format!("{fn_name}() function requires a type specifier as parameter"),
            }),
        }
    }

    /// `aggregate(expression [, init])` — fold a collection using `$this`
    /// (current item) and `$total` (accumulator).
    fn evaluate_aggregate_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.is_empty() || parameters.len() > 2 {
            return Err(FhirPathError::FunctionError {
                message: "aggregate() requires 1–2 parameters: expression [, init]".to_string(),
            });
        }
        let expr = &parameters[0];
        let init = if parameters.len() == 2 {
            self.evaluate_expression(&parameters[1], context)?
        } else {
            FhirPathValue::Empty
        };

        let items: Vec<FhirPathValue> = match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            FhirPathValue::Empty => return Ok(init),
            other => vec![other.clone()],
        };

        let mut total = init;
        for (idx, item) in items.into_iter().enumerate() {
            let iter_ctx = context.with_aggregate_vars_and_index(item, total, idx as i64);
            total = self.evaluate_expression(expr, &iter_ctx)?;
        }
        Ok(total)
    }
}

impl Default for FhirPathEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract a qualified type name (e.g. "System.Boolean", "FHIR.Patient") from
/// the parameter expression of `is()` / `as()` / `ofType()`.
///
/// Matches a simple member chain like `System.Boolean`, `FHIR.Patient`, or
/// `FHIR.\`Patient\`` (delimited identifier). Returns the joined name —
/// "System.Boolean" — which downstream `TypeEvaluator::value_matches_type`
/// can compare case-insensitively.
fn extract_qualified_type_name(expr: &Expression) -> Option<String> {
    let mut parts = Vec::new();
    collect_member_chain(expr, &mut parts)?;
    Some(parts.join("."))
}

fn collect_member_chain(expr: &Expression, out: &mut Vec<String>) -> Option<()> {
    match expr {
        Expression::Term(Term::Invocation(Invocation::Member(name))) => {
            out.push(name.clone());
            Some(())
        }
        Expression::Invocation { left, invocation } => {
            collect_member_chain(left, out)?;
            match invocation {
                Invocation::Member(name) => {
                    out.push(name.clone());
                    Some(())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Strip a `FhirPrimitive` wrapper to its inner value.
/// Makes FhirPrimitive transparent in all operations except `extension()`.
fn strip_fhir_primitive(value: FhirPathValue) -> FhirPathValue {
    match value {
        FhirPathValue::FhirPrimitive { inner, .. } => *inner,
        FhirPathValue::Collection(items) => {
            FhirPathValue::Collection(items.into_iter().map(strip_fhir_primitive).collect())
        }
        FhirPathValue::UnorderedCollection(items) => FhirPathValue::UnorderedCollection(
            items.into_iter().map(strip_fhir_primitive).collect(),
        ),
        other => other,
    }
}

/// Check if `member` is a FHIR choice-type suffixed key (e.g. `valueQuantity`
/// is `value` + `Quantity`). Returns the base name (`value`) if so.
///
/// Uses the FHIR metadata registry to distinguish real element names like
/// `birthDate` (registered) from suffixed choice-type keys like
/// `valueQuantity` (not registered; the real element is `value[x]`).
fn choice_type_base<'a>(resource_type: Option<&'a str>, member: &'a str) -> Option<&'a str> {
    const FHIR_TYPE_SUFFIXES: &[&str] = &[
        "Quantity",
        "Age",
        "Distance",
        "Duration",
        "Count",
        "Money",
        "Boolean",
        "Integer",
        "UnsignedInt",
        "PositiveInt",
        "Decimal",
        "String",
        "Code",
        "Uri",
        "Uuid",
        "Url",
        "Canonical",
        "Oid",
        "Id",
        "Markdown",
        "Base64Binary",
        "Date",
        "DateTime",
        "Instant",
        "Time",
        "Reference",
        "Identifier",
        "Annotation",
        "Attachment",
        "CodeableConcept",
        "Coding",
        "Ratio",
        "SampledData",
        "HumanName",
        "Address",
        "ContactPoint",
        "Period",
        "Meta",
        "Narrative",
        "Dosage",
    ];
    for suffix in FHIR_TYPE_SUFFIXES {
        if member.ends_with(suffix) && member.len() > suffix.len() {
            let split_at = member.len() - suffix.len();
            if member
                .as_bytes()
                .get(split_at)
                .is_some_and(|&c| c.is_ascii_uppercase())
            {
                let base = &member[..split_at];
                // Only flag as choice-type if the base + "[x]" element exists
                // in the FHIR metadata for the resource. This distinguishes
                // `valueQuantity` (from `value[x]`) from `linkId` (a regular
                // BackboneElement sub-field) and `birthDate` (a regular date).
                if let Some(rt) = resource_type {
                    let choice_key = format!("{}[x]", base);
                    if crate::evaluator::metadata::get_field_type(rt, &choice_key).is_some() {
                        return Some(base);
                    }
                }
                // If no resource type metadata is available, we can't
                // reliably detect choice-type suffixed keys, so we don't
                // flag them.
            }
        }
    }
    None
}

/// Build a typed `FhirPathValue` for a FHIR choice-type field based on the
/// type suffix of its JSON key (e.g. `valueQuantity` → suffix "Quantity").
///
/// Returns `Some(typed)` for suffixes that map onto a concrete FhirPathValue
/// variant and whose JSON shape matches; `None` to let the caller fall back
/// to metadata-driven typing.
fn construct_typed_choice_value(suffix: &str, value: &serde_json::Value) -> Option<FhirPathValue> {
    use serde_json::Value;
    match suffix {
        "Quantity" => {
            let obj = value.as_object()?;
            let num = obj.get("value")?.as_f64()?;
            let unit = obj
                .get("unit")
                .and_then(Value::as_str)
                .or_else(|| obj.get("code").and_then(Value::as_str))
                .map(str::to_string);
            Some(FhirPathValue::Quantity {
                value: Decimal::from_f64_retain(num).unwrap_or(Decimal::ZERO),
                unit,
            })
        }
        // Quantity profiles: return as TypedObject so that `is Age`, `is Quantity`
        // (subtype), and member access (`.value`, `.unit`) all work correctly.
        "Age" | "Distance" | "Duration" | "Count" | "Money" => Some(FhirPathValue::TypedObject {
            value: value.clone(),
            fhir_type: suffix.to_string(),
        }),
        "Boolean" => value.as_bool().map(|b| FhirPathValue::TypedBoolean {
            value: b,
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Boolean,
        }),
        "Integer" | "UnsignedInt" | "PositiveInt" => value.as_i64().map(FhirPathValue::Integer),
        "Decimal" => value
            .as_f64()
            .map(|f| FhirPathValue::Number(Decimal::from_f64_retain(f).unwrap_or(Decimal::ZERO))),
        "String" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::String,
        }),
        "Code" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Code,
        }),
        "Uri" | "Uuid" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Uri,
        }),
        "Url" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Url,
        }),
        "Canonical" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Canonical,
        }),
        "Oid" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Oid,
        }),
        "Id" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Id,
        }),
        "Markdown" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Markdown,
        }),
        "Base64Binary" => value.as_str().map(|s| FhirPathValue::TypedString {
            value: s.to_string(),
            fhir_type: rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Base64Binary,
        }),
        "Date" => value.as_str().map(|s| FhirPathValue::Date(s.to_string())),
        "DateTime" | "Instant" => value.as_str().map(|s| FhirPathValue::TypedDateTime {
            value: s.to_string(),
            fhir_type: if suffix == "Instant" {
                rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::Instant
            } else {
                rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType::DateTime
            },
        }),
        "Time" => value
            .as_str()
            .map(|s| FhirPathValue::Time(s.strip_prefix('T').unwrap_or(s).to_string())),
        // Complex types not yet modelled: CodeableConcept, Coding, Reference,
        // Identifier, etc. Caller falls back to metadata/JSON-object handling.
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FhirPathParser;
    use serde_json::json;

    #[test]
    fn test_is_function_basic() {
        let data = json!({
            "string_value": "test"
        });

        let context = EvaluationContext::new(data);
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        let ast = parser
            .parse("string_value.is(String)")
            .expect("Failed to parse");
        let result = evaluator
            .evaluate(&ast, &context)
            .expect("Failed to evaluate");
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_as_function_basic() {
        let data = json!({
            "string_value": "test"
        });

        let context = EvaluationContext::new(data);
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        let ast = parser
            .parse("string_value.as(String)")
            .expect("Failed to parse");
        let result = evaluator
            .evaluate(&ast, &context)
            .expect("Failed to evaluate");
        assert_eq!(result, FhirPathValue::String("test".to_string()));
    }
}
