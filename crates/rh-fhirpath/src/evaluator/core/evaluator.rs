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
use std::{borrow::Cow, rc::Rc};

mod construction;
mod invocation;
mod navigation;
mod scoped;

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
        expression.root.check_depth()?;
        let mut context = Cow::Borrowed(context);
        if context.initial_focus.is_none() {
            let initial_focus = context.current_focus().into_owned();
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
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                CollectionEvaluator::evaluate_union(&left_result, &right_result)
            }
            Expression::And { left, right } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                ComparisonEvaluator::evaluate_and(&left_result, &right_result)
            }
            Expression::Or {
                left,
                operator,
                right,
            } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                ComparisonEvaluator::evaluate_or(&left_result, operator, &right_result)
            }
            Expression::Implies { left, right } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                ComparisonEvaluator::evaluate_implies(&left_result, &right_result)
            }
            Expression::Equality {
                left,
                operator,
                right,
            } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                ComparisonEvaluator::evaluate_equality(&left_result, operator, &right_result)
            }
            Expression::Inequality {
                left,
                operator,
                right,
            } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
                ComparisonEvaluator::evaluate_inequality(&left_result, operator, &right_result)
            }
            Expression::Membership {
                left,
                operator,
                right,
            } => {
                let (left_result, right_result) = self.evaluate_operands(left, right, context)?;
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

    fn evaluate_operands(
        &self,
        left: &Expression,
        right: &Expression,
        context: &EvaluationContext,
    ) -> FhirPathResult<(FhirPathValue, FhirPathValue)> {
        let left = self.evaluate_expression(left, context)?;
        let right = self.evaluate_expression(right, context)?;
        Ok((left, right))
    }

    fn evaluate_chain(
        &self,
        expression: &Expression,
        context: &mut Cow<'_, EvaluationContext>,
    ) -> FhirPathResult<FhirPathValue> {
        match expression {
            Expression::Invocation { left, invocation } => {
                let target = self.evaluate_chain(left, context)?;
                if context.strict_mode && target.is_empty() {
                    navigation::validate_cast_member(left, invocation, context.model)?;
                }
                self.evaluate_invocation(Some(target), invocation, context)
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
            Expression::Term(Term::Invocation(invocation)) => {
                self.evaluate_invocation(None, invocation, context)
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
        let (name, projection) = match parameters {
            [name] => (name, None),
            [name, projection] => (name, Some(projection)),
            _ => {
                return Err(FhirPathError::function_error(
                    "defineVariable() requires a name and an optional projection",
                ));
            }
        };
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
        let value = match projection {
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
                self.evaluate_invocation(None, invocation, &mut Cow::Borrowed(context))
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

    /// Evaluate a literal value
    fn evaluate_literal(&self, literal: &Literal) -> FhirPathResult<FhirPathValue> {
        if let Literal::Date(value) | Literal::DateTime(value) | Literal::Time(value) = literal {
            use crate::evaluator::operations::temporal::{parse_temporal, parse_time_only};
            let temporal = if matches!(literal, Literal::Time(_)) {
                parse_time_only(value)
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

fn collect_member_chain<'a>(expr: &'a Expression, out: &mut Vec<&'a str>) -> Option<()> {
    match expr {
        Expression::Term(Term::Invocation(Invocation::Member(name))) => {
            out.push(name);
            Some(())
        }
        Expression::Invocation { left, invocation } => {
            collect_member_chain(left, out)?;
            match invocation {
                Invocation::Member(name) => {
                    out.push(name);
                    Some(())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

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
