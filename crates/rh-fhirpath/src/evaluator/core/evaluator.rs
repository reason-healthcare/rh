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
use serde_json::Value;
use std::{borrow::Cow, cmp::Ordering, rc::Rc};

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
        let result = self.evaluate_nodes(expression, context)?;
        Ok(strip_fhir_primitive(result))
    }

    /// Evaluate while preserving logical FHIR node types and source paths.
    /// Use this when the output will become the starting focus for another expression.
    pub fn evaluate_nodes(
        &self,
        expression: &FhirPathExpression,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let mut context = Cow::Borrowed(context);
        if context.initial_focus.is_none() {
            let initial_focus = context.current_focus();
            context.to_mut().initial_focus = Some(Rc::new(initial_focus));
        }
        self.evaluate_expression(&expression.root, &context)
    }

    /// Evaluate an expression against a context
    fn evaluate_expression(
        &self,
        expression: &Expression,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match expression {
            Expression::Term(Term::Invocation(Invocation::Function { name, .. }))
                if name == "defineVariable" =>
            {
                self.evaluate_chain(expression, &mut Cow::Borrowed(context))
            }
            Expression::Term(term) => self.evaluate_term(term, context),
            Expression::SortKey { .. } => Err(FhirPathError::syntax_error(
                1,
                0,
                "Sort directions are only valid in sort() selectors",
            )),
            Expression::Invocation { .. }
            | Expression::Indexer { .. }
            | Expression::Type { .. } => {
                self.evaluate_chain(expression, &mut Cow::Borrowed(context))
            }
            Expression::Union { left, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                CollectionEvaluator::evaluate_union(&left_result, &right_result)
            }
            Expression::And { left, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ComparisonEvaluator::evaluate_and(&left_result, &right_result)
            }
            Expression::Or {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ComparisonEvaluator::evaluate_or(&left_result, operator, &right_result)
            }
            Expression::Implies { left, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ComparisonEvaluator::evaluate_implies(&left_result, &right_result)
            }
            Expression::Equality {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ComparisonEvaluator::evaluate_equality(&left_result, operator, &right_result)
            }
            Expression::Inequality {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ComparisonEvaluator::evaluate_inequality(&left_result, operator, &right_result)
            }
            Expression::Membership {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
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
        }
    }

    // Each expression starts with its enclosing variables. Only a receiver
    // chain carries the bindings attached to its intermediate collections.
    fn evaluate_chain(
        &self,
        expression: &Expression,
        context: &mut Cow<'_, EvaluationContext>,
    ) -> FhirPathResult<FhirPathValue> {
        match expression {
            Expression::Invocation { left, invocation } => {
                let target = self.evaluate_chain(left, context)?;
                if context.strict_mode && target.is_empty() {
                    if let Invocation::Member(member) = invocation {
                        if let Some(ty) = declared_cast_type(left) {
                            let ty = ty.strip_prefix("FHIR.").unwrap_or(&ty);
                            if context.model.is_type(ty)
                                && context.model.field_type(ty, member).is_none()
                                && context
                                    .model
                                    .field_type(ty, &format!("{member}[x]"))
                                    .is_none()
                            {
                                return Err(FhirPathError::type_error(format!(
                                    "Unknown member '{member}' of FHIR.{ty}"
                                )));
                            }
                        }
                    }
                }
                if let Invocation::Function { name, parameters } = invocation {
                    if name == "defineVariable" {
                        return self.evaluate_define_variable(target, parameters, context);
                    }
                }
                self.evaluate_invocation(&target, invocation, context)
            }
            Expression::Indexer { left, index } => {
                let target = self.evaluate_chain(left, context)?;
                let index = strip_fhir_primitive(self.evaluate_expression(index, context)?);
                CollectionEvaluator::evaluate_indexer(&target, &index)
            }
            Expression::Type {
                left,
                operator,
                type_specifier,
            } => {
                TypeEvaluator::validate_type(
                    context.model,
                    &type_specifier.qualified_name.join("."),
                )?;
                let target = self.evaluate_chain(left, context)?;
                TypeEvaluator::evaluate_type_operation(&target, operator, type_specifier)
            }
            Expression::Term(Term::Invocation(Invocation::Function { name, parameters }))
                if name == "defineVariable" =>
            {
                self.evaluate_define_variable(context.current_focus(), parameters, context)
            }
            Expression::Term(Term::Parenthesized(expression)) => {
                self.evaluate_chain(expression, context)
            }
            _ => self.evaluate_expression(expression, context),
        }
    }

    fn resolve_external_variable(
        &self,
        name: &str,
        context: &EvaluationContext,
    ) -> FhirPathResult<Option<FhirPathValue>> {
        use crate::evaluator::metadata;
        if let Some(value) = context
            .variables
            .get(name)
            .or_else(|| context.constants.get(name))
        {
            return Ok(Some(value.clone()));
        }
        match name {
            "context" => Ok(Some(
                context
                    .initial_focus
                    .as_deref()
                    .cloned()
                    .unwrap_or_else(|| metadata::root_or_current_node(context)),
            )),
            "resource" => Ok(Some(metadata::resource_for_focus(context))),
            "rootResource" => Ok(Some(metadata::root_node(context))),
            _ => crate::extensions::ExtensionRegistry::new().resolve_variable(name, context),
        }
    }

    fn evaluate_define_variable(
        &self,
        target: FhirPathValue,
        parameters: &[Expression],
        context: &mut Cow<'_, EvaluationContext>,
    ) -> FhirPathResult<FhirPathValue> {
        let [name, projection @ ..] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: "defineVariable() requires a name and an optional projection".into(),
            });
        };
        if projection.len() > 1 {
            return Err(FhirPathError::FunctionError {
                message: "defineVariable() requires a name and an optional projection".into(),
            });
        }
        let name = strip_fhir_primitive(self.evaluate_expression(name, context)?);
        let name = name
            .singleton("defineVariable() name")?
            .and_then(FhirPathValue::as_str)
            .ok_or_else(|| FhirPathError::TypeError {
                message: "defineVariable() name must be a String".into(),
            })?;
        if self.resolve_external_variable(name, context)?.is_some() {
            return Err(FhirPathError::EvaluationError {
                message: format!("Environment variable %{name} already defined"),
            });
        }
        let value = match projection.first() {
            Some(projection) => {
                let projection_context = context.with_invocation_focus(target.clone());
                self.evaluate_expression(projection, &projection_context)?
            }
            None => target.clone(),
        };
        context.to_mut().variables.insert(name.into(), value);
        Ok(target)
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
                self.evaluate_invocation(&context.current_focus(), invocation, context)
            }
            Term::ExternalConstant(name) => self
                .resolve_external_variable(name, context)?
                .ok_or_else(|| {
                    FhirPathError::evaluation_error(format!(
                        "Undefined environment variable: %{name}"
                    ))
                }),
            Term::Parenthesized(expr) => self.evaluate_expression(expr, context),
            Term::InstanceSelector {
                type_specifier,
                elements,
            } => self.evaluate_instance_selector(type_specifier, elements, context),
        }
    }

    fn evaluate_instance_selector(
        &self,
        type_specifier: &TypeSpecifier,
        elements: &[(String, Expression)],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        use crate::evaluator::metadata::FhirNode;

        let focus = context.current_focus();
        focus.singleton("Instance selector input")?;
        if focus.is_empty() && context.this_value.is_some() {
            return Ok(FhirPathValue::Empty);
        }

        let type_name = match type_specifier.qualified_name.as_slice() {
            [name] => name,
            [namespace, name] if namespace == "FHIR" => name,
            _ => {
                return Err(FhirPathError::type_error(format!(
                    "Unknown constructor type {type_specifier}"
                )))
            }
        };
        let model = context.model;
        if !model.is_type(type_name) {
            return Err(FhirPathError::type_error(format!(
                "Unknown constructor type {type_specifier}"
            )));
        }

        let is_primitive = type_name.starts_with(|c: char| c.is_ascii_lowercase());
        let mut fields = serde_json::Map::new();
        let mut assigned = std::collections::HashSet::new();
        for (name, expression) in elements {
            if !assigned.insert(name) {
                return Err(FhirPathError::type_error(format!(
                    "Duplicate constructor field {type_specifier}.{name}"
                )));
            }
            let value = self.evaluate_expression(expression, context)?;
            let direct_type = model.field_type(type_name, name);
            let choice = model.field_type(type_name, &format!("{name}[x]")).is_some();
            if value.is_empty() && (direct_type.is_some() || choice) {
                continue;
            }

            let key = if direct_type.is_none() && choice {
                let item = value
                    .singleton("Choice constructor element")?
                    .ok_or_else(|| {
                        FhirPathError::type_error(
                            "Cannot infer constructor choice from an empty value",
                        )
                    })?;
                let item_type = constructor_value_type(item).ok_or_else(|| {
                    FhirPathError::type_error(format!(
                        "Cannot infer the type of {type_specifier}.{name}"
                    ))
                })?;
                model
                    .choice_key(type_name, name, item_type)
                    .ok_or_else(|| {
                        FhirPathError::type_error(format!(
                            "{item_type} cannot populate {type_specifier}.{name}"
                        ))
                    })?
            } else {
                name.clone()
            };
            let field_type = model.field_type(type_name, &key).ok_or_else(|| {
                FhirPathError::type_error(format!(
                    "Unknown constructor field {type_specifier}.{name}"
                ))
            })?;
            if fields.contains_key(&key) {
                return Err(FhirPathError::type_error(format!(
                    "Duplicate constructor field {type_specifier}.{key}"
                )));
            }

            let built = if model.field_is_repeating(type_name, &key) {
                let items = value
                    .items()
                    .iter()
                    .map(|item| constructor_json(item, field_type, model))
                    .collect::<FhirPathResult<Vec<_>>>()?;
                Value::Array(items)
            } else {
                let Some(item) = value.singleton("Constructor element")? else {
                    continue;
                };
                constructor_json(item, field_type, model)?
            };
            fields.insert(key, built);
        }

        let node = if is_primitive {
            let value = fields.remove("value").unwrap_or(Value::Null);
            let metadata = (!fields.is_empty()).then_some(Value::Object(fields));
            FhirNode::primitive(type_name, &value, metadata, model)
        } else {
            if model.is_subtype(type_name, "Resource") {
                fields.insert("resourceType".into(), Value::String(type_name.clone()));
            }
            FhirNode::detached(&Value::Object(fields), type_name, model)
        };
        Ok(FhirPathValue::Node(Box::new(node)))
    }

    /// Evaluate a literal value
    fn evaluate_literal(&self, literal: &Literal) -> FhirPathResult<FhirPathValue> {
        if let Literal::Date(value) | Literal::DateTime(value) | Literal::Time(value) = literal {
            use crate::evaluator::operations::temporal::parse_temporal;
            let temporal = if matches!(literal, Literal::Time(_)) && !value.starts_with('T') {
                parse_temporal(&format!("T{value}"))
            } else {
                parse_temporal(value)
            };
            if temporal.is_none_or(|parts| parts.year == Some(0)) {
                return Err(FhirPathError::evaluation_error(format!(
                    "Invalid temporal literal @{value}"
                )));
            }
        }
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
                    "repeat" | "repeatAll" => {
                        self.evaluate_repeat_function(target, name, parameters, context)
                    }
                    "ofType" => self.evaluate_of_type_function(target, parameters, context),
                    "is" => self.evaluate_is_function(target, parameters, context),
                    "as" => self.evaluate_as_function(target, parameters, context),
                    "trace" => self.evaluate_trace_function(target, parameters, context),
                    "aggregate" => self.evaluate_aggregate_function(target, parameters, context),
                    "iif" => self.evaluate_iif_lazy(target, parameters, context),
                    "coalesce" => self.evaluate_coalesce(target, parameters, context),
                    "sort" => self.evaluate_sort_by_expression(target, parameters, context),
                    "exists" if !parameters.is_empty() => {
                        self.evaluate_exists_with_criterion(target, parameters, context)
                    }
                    "all" if !parameters.is_empty() => {
                        self.evaluate_all_with_criterion(target, parameters, context)
                    }
                    "first" | "last" | "tail" | "skip" | "take" => {
                        let parameter_values = parameters
                            .iter()
                            .map(|parameter| self.evaluate_expression(parameter, context))
                            .collect::<FhirPathResult<Vec<_>>>()?;
                        self.evaluate_ordered_collection_fn(
                            target,
                            name,
                            &parameter_values,
                            context.check_ordered_functions,
                        )
                    }
                    _ => {
                        // Regular functions: evaluate parameters first
                        let param_values: Result<Vec<_>, _> = parameters
                            .iter()
                            .map(|p| self.evaluate_expression(p, context))
                            .collect();
                        let param_values = param_values?;
                        use crate::extensions::fhir::host;
                        if host::handles(name) {
                            return host::invoke(name, target, &param_values, context);
                        }
                        if name == "toString" && param_values.is_empty() {
                            if let [FhirPathValue::Node(node)] = target.items() {
                                if let Some(text) = node.extreme_numeric_string() {
                                    return Ok(FhirPathValue::String(text));
                                }
                            }
                        }
                        // Collection operations preserve source identity. Scalar functions
                        // consume the implicit system value of a FHIR primitive.
                        if matches!(
                            name.as_str(),
                            "not"
                                | "type"
                                | "lowBoundary"
                                | "highBoundary"
                                | "precision"
                                | "comparable"
                                | "min"
                                | "max"
                                | "single"
                                | "tail"
                                | "first"
                                | "last"
                                | "take"
                                | "skip"
                                | "reverse"
                                | "union"
                                | "combine"
                                | "intersect"
                                | "exclude"
                                | "distinct"
                                | "count"
                                | "empty"
                                | "exists"
                                | "isDistinct"
                                | "subsetOf"
                                | "supersetOf"
                        ) {
                            self.evaluate_function_call(target, name, &param_values)
                        } else {
                            let params: Vec<_> =
                                param_values.into_iter().map(strip_fhir_primitive).collect();
                            self.evaluate_function_call(
                                &strip_fhir_primitive(target.clone()),
                                name,
                                &params,
                            )
                        }
                    }
                }
            }
            Invocation::This => Ok(context
                .this_value
                .as_ref()
                .or(context.initial_focus.as_deref())
                .cloned()
                .unwrap_or_else(|| context.current_focus())),
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
            FhirPathValue::Node(node) => node.member(member, context.strict_mode),
            FhirPathValue::TypedObject { value, fhir_type } if context.model.is_type(fhir_type) => {
                crate::evaluator::metadata::FhirNode::detached(value, fhir_type, context.model)
                    .member(member, context.strict_mode)
            }

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
        if check_ordered && target.is_unordered() {
            return Err(FhirPathError::EvaluationError {
                message: format!(
                    "Cannot apply order-dependent function '{name}' to unordered collection"
                ),
            });
        }
        match name {
            "first" if params.is_empty() => CollectionEvaluator::first(target, check_ordered),
            "last" if params.is_empty() => CollectionEvaluator::last(target, check_ordered),
            "tail" if params.is_empty() => CollectionEvaluator::tail(target, check_ordered),
            "skip" | "take" => {
                match CollectionEvaluator::subsetting_count(params, &format!("{name}()"))? {
                    None => Ok(FhirPathValue::Empty),
                    Some(count) if name == "skip" => {
                        CollectionEvaluator::skip(target, count, check_ordered)
                    }
                    Some(count) => CollectionEvaluator::take(target, count),
                }
            }
            _ => Err(FhirPathError::FunctionError {
                message: format!("{name}() takes no arguments"),
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

        let mut filtered = Vec::new();
        for (index, item) in target.items().iter().enumerate() {
            let item_context = context.with_this_and_index(item.clone(), index as i64);
            let criterion = self.evaluate_expression(&parameters[0], &item_context)?;
            if criterion.singleton_boolean("where() criterion")? == Some(true) {
                filtered.push(item.clone());
            }
        }
        Ok(FhirPathValue::from_items(filtered))
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

        let focus = target
            .singleton("iif() input")?
            .cloned()
            .unwrap_or(FhirPathValue::Empty);
        let iif_ctx = context.with_this_value(focus);
        let criterion = self.evaluate_expression(&parameters[0], &iif_ctx)?;
        let is_truthy = criterion.singleton_boolean("iif() criterion")? == Some(true);

        // Lazy: only evaluate the selected branch.
        if is_truthy {
            self.evaluate_expression(&parameters[1], &iif_ctx)
        } else if let Some(otherwise) = parameters.get(2) {
            self.evaluate_expression(otherwise, &iif_ctx)
        } else {
            Ok(FhirPathValue::Empty)
        }
    }

    fn evaluate_coalesce(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.is_empty() {
            return Err(FhirPathError::FunctionError {
                message: "coalesce() requires at least one argument".into(),
            });
        }
        let context = context.with_invocation_focus(target.clone());
        for parameter in parameters {
            let result = self.evaluate_expression(parameter, &context)?;
            if !result.items().is_empty() {
                return Ok(result);
            }
        }
        Ok(FhirPathValue::Empty)
    }

    /// Sort stably using singleton keys. Evaluate later keys only for ties.
    fn evaluate_sort_by_expression(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let items = target.items();
        let key_specs: Vec<_> = parameters
            .iter()
            .map(|parameter| match parameter {
                Expression::SortKey { key, descending } => (key.as_ref(), *descending),
                parameter => (parameter, false),
            })
            .collect();

        let evaluate_key = |item: &FhirPathValue, expression: &Expression| {
            let item_context = context.with_unindexed_focus(item.clone());
            let result = self.evaluate_expression(expression, &item_context)?;
            Ok::<_, FhirPathError>(
                result
                    .singleton("sort() key")?
                    .cloned()
                    .unwrap_or(FhirPathValue::Empty),
            )
        };

        let mut keys = items
            .iter()
            .map(|item| {
                let key = if let Some((expression, _)) = key_specs.first() {
                    evaluate_key(item, expression)?
                } else {
                    item.clone()
                };
                Ok(vec![key])
            })
            .collect::<FhirPathResult<Vec<_>>>()?;

        let mut indices: Vec<usize> = (0..items.len()).collect();
        let mut scratch = indices.clone();
        CollectionEvaluator::sort_indices_by(&mut indices, &mut scratch, &mut |a, b| {
            for key_index in 0..key_specs.len().max(1) {
                if key_index >= keys[a].len() {
                    keys[a].push(evaluate_key(&items[a], key_specs[key_index].0)?);
                }
                if key_index >= keys[b].len() {
                    keys[b].push(evaluate_key(&items[b], key_specs[key_index].0)?);
                }
                let order = CollectionEvaluator::compare_sort_keys(
                    &keys[a][key_index],
                    &keys[b][key_index],
                )?;
                if order != Ordering::Equal {
                    let descending = key_specs.get(key_index).is_some_and(|key| key.1);
                    return Ok(if descending { order.reverse() } else { order });
                }
            }
            Ok(Ordering::Equal)
        })?;
        Ok(FhirPathValue::from_items(
            indices
                .into_iter()
                .map(|index| items[index].clone())
                .collect(),
        ))
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
        for (index, item) in target.items().iter().enumerate() {
            let item_context = context.with_this_and_index(item.clone(), index as i64);
            let result = self.evaluate_expression(&parameters[0], &item_context)?;
            if result.singleton_boolean("exists() criterion")? == Some(true) {
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
        for (index, item) in target.items().iter().enumerate() {
            let item_context = context.with_this_and_index(item.clone(), index as i64);
            let result = self.evaluate_expression(&parameters[0], &item_context)?;
            if result.singleton_boolean("all() criterion")? != Some(true) {
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

        let mut selected = Vec::new();
        for (index, item) in target.items().iter().enumerate() {
            let item_context = context.with_this_and_index(item.clone(), index as i64);
            let result = self.evaluate_expression(&parameters[0], &item_context)?;
            selected.extend(result.into_items());
        }
        Ok(FhirPathValue::from_items(selected))
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

        let name = self.evaluate_expression(&parameters[0], context)?;
        let name = name
            .singleton("trace() name")?
            .and_then(|value| value.system_value().as_str())
            .ok_or_else(|| FhirPathError::FunctionError {
                message: "trace() name parameter must evaluate to a string".into(),
            })?;

        let value_to_log = if parameters.len() == 2 {
            self.evaluate_select_function(target, &parameters[1..], context)?
        } else {
            target.clone()
        };

        if std::env::var_os("RH_FHIRPATH_TRACE_STDERR").is_some() {
            eprintln!("[TRACE:{name}] {value_to_log:?}");
        }

        context.add_trace_log(name.to_owned(), format!("{value_to_log:?}"));

        // Return the original input unchanged
        Ok(target.clone())
    }

    /// Project each generation, returning only the projected items.
    fn evaluate_repeat_function(
        &self,
        target: &FhirPathValue,
        name: &str,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [projection] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: format!("{name}() function requires exactly one parameter"),
            });
        };
        let allow_duplicates = name == "repeatAll";
        let mut result = Vec::new();
        let mut pending: std::collections::VecDeque<_> = target
            .items()
            .iter()
            .cloned()
            .map(|item| (item, 0))
            .collect();
        while let Some((item, depth)) = pending.pop_front() {
            let item_context = context.with_unindexed_focus(item);
            let projected = self.evaluate_expression(projection, &item_context)?;
            for value in projected.into_items() {
                if !allow_duplicates
                    && result
                        .iter()
                        .any(|item| FhirPathValue::equals_static(item, &value))
                {
                    continue;
                }
                if depth >= 1_000 || result.len() >= 100_000 {
                    return Err(FhirPathError::EvaluationError {
                        message: format!("{name}() exceeded its recursive projection limit"),
                    });
                }
                pending.push_back((value.clone(), depth + 1));
                result.push(value);
            }
        }
        Ok(FhirPathValue::from_items(result))
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
        TypeEvaluator::validate_type(context.model, &type_name)?;
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
        TypeEvaluator::validate_type(context.model, &type_name)?;
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
        TypeEvaluator::validate_type(context.model, &type_name)?;
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

fn constructor_value_type(value: &FhirPathValue) -> Option<&str> {
    match value {
        FhirPathValue::Node(node) if node.is_system_projection() => {
            constructor_value_type(&node.inner)
        }
        FhirPathValue::Node(node) => Some(&node.fhir_type),
        FhirPathValue::TypedObject { fhir_type, .. } => Some(fhir_type),
        FhirPathValue::Object(value) => value.get("resourceType").and_then(Value::as_str),
        FhirPathValue::String(_) | FhirPathValue::TypedString { .. } => Some("string"),
        FhirPathValue::Boolean(_) | FhirPathValue::TypedBoolean { .. } => Some("boolean"),
        FhirPathValue::Integer(_) => Some("integer"),
        FhirPathValue::Long(_) => Some("integer64"),
        FhirPathValue::Number(_) => Some("decimal"),
        FhirPathValue::Date(_) => Some("date"),
        FhirPathValue::DateTime(_) | FhirPathValue::TypedDateTime { .. } => Some("dateTime"),
        FhirPathValue::Time(_) => Some("time"),
        FhirPathValue::Quantity { .. } => Some("Quantity"),
        _ => None,
    }
}

fn constructor_json(
    item: &FhirPathValue,
    expected_type: &str,
    model: crate::evaluator::metadata::FhirModel,
) -> FhirPathResult<Value> {
    let json = item.to_json();
    let accepted = match expected_type {
        "integer64" if json.is_i64() => return Ok(Value::String(json.to_string())),
        "integer64" => json
            .as_str()
            .is_some_and(|value| value.parse::<i64>().is_ok()),
        "boolean" => json.is_boolean(),
        "integer" => json.as_i64().is_some_and(|i| i32::try_from(i).is_ok()),
        "positiveInt" => json
            .as_i64()
            .is_some_and(|i| (1..=i64::from(i32::MAX)).contains(&i)),
        "unsignedInt" => json
            .as_i64()
            .is_some_and(|i| (0..=i64::from(i32::MAX)).contains(&i)),
        "decimal" => json.is_number(),
        name if name.starts_with(|c: char| c.is_ascii_lowercase()) => json.is_string(),
        name => constructor_value_type(item).is_some_and(|actual| model.is_subtype(actual, name)),
    };
    if !accepted {
        return Err(FhirPathError::type_error(format!(
            "Cannot populate {expected_type} with {item:?}"
        )));
    }
    Ok(json)
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

/// The result type of an explicit cast remains known when the cast has no matches.
fn declared_cast_type(expression: &Expression) -> Option<String> {
    match expression {
        Expression::Term(Term::Parenthesized(inner)) | Expression::Indexer { left: inner, .. } => {
            declared_cast_type(inner)
        }
        Expression::Type {
            operator: TypeOperator::As,
            type_specifier,
            ..
        } => Some(type_specifier.qualified_name.join(".")),
        Expression::Invocation {
            invocation: Invocation::Function { name, parameters },
            ..
        }
        | Expression::Term(Term::Invocation(Invocation::Function { name, parameters }))
            if matches!(name.as_str(), "as" | "ofType") =>
        {
            extract_qualified_type_name(parameters.first()?)
        }
        _ => None,
    }
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
        FhirPathValue::Node(node) => node.inner,
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
