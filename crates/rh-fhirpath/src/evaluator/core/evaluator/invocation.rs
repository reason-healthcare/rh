use super::{extract_qualified_type_name, strip_fhir_primitive, FhirPathEvaluator};
use crate::ast::{Expression, Invocation, TypeOperator, TypeSpecifier};
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::collection::CollectionEvaluator;
use crate::evaluator::types::operations::TypeEvaluator;
use crate::evaluator::{EvaluationContext, FhirPathValue};

use crate::extensions::fhir::host;
use std::borrow::Cow;

impl FhirPathEvaluator {
    pub(super) fn evaluate_invocation(
        &self,
        target: Option<FhirPathValue>,
        invocation: &Invocation,
        context: &mut Cow<'_, EvaluationContext>,
    ) -> FhirPathResult<FhirPathValue> {
        let target = target.map_or_else(|| context.current_focus(), Cow::Owned);
        match invocation {
            Invocation::Member(name) => super::navigation::navigate_member(
                &target,
                name,
                context.model,
                context.strict_mode,
            ),
            Invocation::Function { name, parameters } if name == "defineVariable" => {
                let target = target.into_owned();
                self.evaluate_define_variable(target, parameters, context)
            }
            Invocation::Function { name, parameters } => {
                self.evaluate_function(target, name, parameters, context)
            }
            Invocation::This => Ok(context
                .this_value
                .as_ref()
                .or(context.initial_focus.as_deref())
                .cloned()
                .unwrap_or_else(|| context.current_focus().into_owned())),
            Invocation::Index => Ok(context
                .index_value
                .map(FhirPathValue::Integer)
                .unwrap_or(FhirPathValue::Empty)),
            Invocation::Total => Ok(context.total_value.clone().unwrap_or(FhirPathValue::Empty)),
        }
    }

    fn evaluate_function(
        &self,
        target: Cow<'_, FhirPathValue>,
        name: &str,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        match name {
            "where" => self.evaluate_where_function(&target, parameters, context),
            "select" => self.evaluate_select_function(&target, parameters, context),
            "repeat" | "repeatAll" => {
                self.evaluate_repeat_function(&target, name, parameters, context)
            }
            "is" => {
                self.evaluate_type_function(&target, parameters, context, Some(TypeOperator::Is))
            }
            "as" => {
                self.evaluate_type_function(&target, parameters, context, Some(TypeOperator::As))
            }
            "ofType" => self.evaluate_type_function(&target, parameters, context, None),
            "trace" => self.evaluate_trace_function(target.into_owned(), parameters, context),
            "aggregate" => self.evaluate_aggregate_function(&target, parameters, context),
            "iif" => self.evaluate_iif_lazy(&target, parameters, context),
            "coalesce" => self.evaluate_coalesce(&target, parameters, context),
            "sort" => self.evaluate_sort_by_expression(&target, parameters, context),
            "exists" if !parameters.is_empty() => {
                self.evaluate_exists_with_criterion(&target, parameters, context)
            }
            "all" if !parameters.is_empty() => {
                self.evaluate_all_with_criterion(&target, parameters, context)
            }
            _ => {
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.evaluate_expression(parameter, context))
                    .collect::<FhirPathResult<Vec<_>>>()?;
                self.invoke_eager_function(target, name, parameters, context)
            }
        }
    }

    fn invoke_eager_function(
        &self,
        target: Cow<'_, FhirPathValue>,
        name: &str,
        parameters: Vec<FhirPathValue>,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if matches!(name, "first" | "last" | "tail" | "skip" | "take") {
            return Self::evaluate_ordered_collection_fn(
                &target,
                name,
                &parameters,
                context.check_ordered_functions,
            );
        }
        if host::handles(name) {
            return host::invoke(name, &target, &parameters, context);
        }
        if name == "toString" && parameters.is_empty() {
            if let [FhirPathValue::Node(node)] = target.items() {
                if let Some(text) = node.extreme_numeric_string() {
                    return Ok(FhirPathValue::String(text));
                }
            }
        }
        let func = self
            .function_registry
            .get_function(name)
            .ok_or_else(|| FhirPathError::function_error(format!("Unknown function: {name}")))?;
        if preserves_node_arguments(name) {
            func(&target, &parameters)
        } else {
            let parameters: Vec<_> = parameters.into_iter().map(strip_fhir_primitive).collect();
            match target {
                Cow::Owned(value) => func(&strip_fhir_primitive(value), &parameters),
                Cow::Borrowed(FhirPathValue::Node(node)) => func(&node.inner, &parameters),
                Cow::Borrowed(FhirPathValue::FhirPrimitive { inner, .. }) => {
                    func(inner, &parameters)
                }
                Cow::Borrowed(
                    value @ (FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_)),
                ) => func(&strip_fhir_primitive(value.clone()), &parameters),
                Cow::Borrowed(value) => func(value, &parameters),
            }
        }
    }

    /// Evaluate order-sensitive collection functions (first, last, tail, skip)
    /// with UnorderedCollection checking.
    fn evaluate_ordered_collection_fn(
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

    fn evaluate_type_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
        operator: Option<TypeOperator>,
    ) -> FhirPathResult<FhirPathValue> {
        let name = match operator {
            Some(TypeOperator::Is) => "is",
            Some(TypeOperator::As) => "as",
            None => "ofType",
        };
        let [parameter] = parameters else {
            return Err(FhirPathError::function_error(format!(
                "{name}() function requires exactly one parameter (type specifier)"
            )));
        };
        let type_name = self.resolve_type_argument(parameter, context, name)?;
        TypeEvaluator::validate_type(context.model, &type_name)?;
        let type_specifier = TypeSpecifier {
            qualified_name: vec![type_name],
        };
        match operator {
            Some(operator) => {
                TypeEvaluator::evaluate_type_operation(target, &operator, &type_specifier)
            }
            None => TypeEvaluator::evaluate_of_type(target, &type_specifier),
        }
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
}

fn preserves_node_arguments(name: &str) -> bool {
    matches!(
        name,
        "not"
            | "type"
            | "lowBoundary"
            | "highBoundary"
            | "precision"
            | "comparable"
            | "min"
            | "max"
            | "single"
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
    )
}
