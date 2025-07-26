//! Core FHIRPath expression evaluator

use crate::ast::*;
use crate::error::*;
use crate::evaluator::{
    arithmetic::ArithmeticEvaluator, collection::CollectionEvaluator,
    comparison::ComparisonEvaluator, context::EvaluationContext, functions::FunctionRegistry,
    values::FhirPathValue,
};

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
        self.evaluate_expression(&expression.root, context)
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
                self.evaluate_invocation(&left_result, invocation, context)
            }
            Expression::Indexer { left, index } => {
                let left_result = self.evaluate_expression(left, context)?;
                let index_result = self.evaluate_expression(index, context)?;
                CollectionEvaluator::evaluate_indexer(&left_result, &index_result)
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
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ArithmeticEvaluator::evaluate_additive(&left_result, operator, &right_result)
            }
            Expression::Multiplicative {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                ArithmeticEvaluator::evaluate_multiplicative(&left_result, operator, &right_result)
            }
            _ => Err(FhirPathError::EvaluationError {
                message: format!("Unsupported expression type: {expression:?}"),
            }),
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
                let current_value = FhirPathValue::Object(context.current.clone());
                self.evaluate_invocation(&current_value, invocation, context)
            }
            Term::ExternalConstant(name) => {
                if let Some(value) = context.constants.get(name) {
                    Ok(value.clone())
                } else {
                    match name.as_str() {
                        "context" => Ok(FhirPathValue::Object(context.current.clone())),
                        "resource" => Ok(FhirPathValue::Object(context.root.clone())),
                        _ => Ok(FhirPathValue::Empty),
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
            Literal::LongNumber(i) => Ok(FhirPathValue::Integer(*i)),
            Literal::Date(d) => Ok(FhirPathValue::Date(d.clone())),
            Literal::DateTime(dt) => Ok(FhirPathValue::DateTime(dt.clone())),
            Literal::Time(t) => Ok(FhirPathValue::Time(t.clone())),
            Literal::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: *value,
                unit: unit.clone(),
            }),
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
            Invocation::Member(name) => self.evaluate_member_access(target, name),
            Invocation::Function { name, parameters } => {
                // Special handling for functions that need to evaluate expressions themselves
                match name.as_str() {
                    "where" => self.evaluate_where_function(target, parameters, context),
                    "select" => self.evaluate_select_function(target, parameters, context),
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
            Invocation::This => Ok(FhirPathValue::Object(context.current.clone())),
            Invocation::Index => Err(FhirPathError::EvaluationError {
                message: "$index not implemented yet".to_string(),
            }),
            Invocation::Total => Err(FhirPathError::EvaluationError {
                message: "$total not implemented yet".to_string(),
            }),
        }
    }

    /// Evaluate member access on a value
    #[allow(clippy::only_used_in_recursion)]
    fn evaluate_member_access(
        &self,
        target: &FhirPathValue,
        member: &str,
    ) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Object(obj) => {
                if let Some(value) = obj.get(member) {
                    Ok(FhirPathValue::from_json(value))
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            FhirPathValue::Collection(items) => {
                let mut result_items = Vec::new();
                for item in items {
                    let member_result = self.evaluate_member_access(item, member)?;
                    match member_result {
                        FhirPathValue::Collection(mut nested_items) => {
                            result_items.append(&mut nested_items);
                        }
                        FhirPathValue::Empty => {
                            // Skip empty results
                        }
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
            _ => Ok(FhirPathValue::Empty),
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
            FhirPathValue::Collection(items) => {
                let mut filtered_items = Vec::new();

                for item in items {
                    // Create new context with current item as $this
                    let item_context = EvaluationContext {
                        current: if let FhirPathValue::Object(obj) = item {
                            obj.clone()
                        } else {
                            // For non-object values, we need to wrap them somehow
                            // For now, let's use the current context but this might need refinement
                            context.current.clone()
                        },
                        ..context.clone()
                    };

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
                let item_context = EvaluationContext {
                    current: if let FhirPathValue::Object(obj) = target {
                        obj.clone()
                    } else {
                        context.current.clone()
                    },
                    ..context.clone()
                };

                let criteria_result = self.evaluate_expression(criteria_expr, &item_context)?;
                if criteria_result.is_truthy() {
                    Ok(target.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
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
            FhirPathValue::Collection(items) => {
                let mut selected_items = Vec::new();

                for item in items {
                    // Create new context with current item as $this
                    let item_context = EvaluationContext {
                        current: if let FhirPathValue::Object(obj) = item {
                            obj.clone()
                        } else {
                            context.current.clone()
                        },
                        ..context.clone()
                    };

                    // Evaluate the projection expression in the item context
                    let projection_result =
                        self.evaluate_expression(projection_expr, &item_context)?;

                    // Add the result to the selected items
                    match projection_result {
                        FhirPathValue::Collection(mut items) => {
                            selected_items.append(&mut items);
                        }
                        FhirPathValue::Empty => {
                            // Don't add empty values
                        }
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
                let item_context = EvaluationContext {
                    current: if let FhirPathValue::Object(obj) = target {
                        obj.clone()
                    } else {
                        context.current.clone()
                    },
                    ..context.clone()
                };

                self.evaluate_expression(projection_expr, &item_context)
            }
        }
    }
}

impl Default for FhirPathEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
