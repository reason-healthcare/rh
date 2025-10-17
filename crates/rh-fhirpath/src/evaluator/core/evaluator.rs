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
            Expression::Polarity { operator, operand } => {
                let operand_result = self.evaluate_expression(operand, context)?;
                ArithmeticEvaluator::evaluate_polarity(operator, &operand_result)
            }
            Expression::Type {
                left,
                operator,
                type_specifier,
            } => {
                let left_result = self.evaluate_expression(left, context)?;
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
            Literal::Time(t) => Ok(FhirPathValue::Time(t.clone())),
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
            Invocation::Member(name) => self.evaluate_member_access(target, name),
            Invocation::Function { name, parameters } => {
                // Special handling for functions that need to evaluate expressions themselves
                match name.as_str() {
                    "where" => self.evaluate_where_function(target, parameters, context),
                    "select" => self.evaluate_select_function(target, parameters, context),
                    "repeat" => self.evaluate_repeat_function(target, parameters, context),
                    "ofType" => self.evaluate_of_type_function(target, parameters, context),
                    "is" => self.evaluate_is_function(target, parameters, context),
                    "as" => self.evaluate_as_function(target, parameters, context),
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
                // Check if member is the resourceType - if so, return the object itself
                // This enables expressions like Patient.id to work on Patient resources
                if let Some(resource_type) = obj.get("resourceType") {
                    if let Some(resource_type_str) = resource_type.as_str() {
                        if resource_type_str == member {
                            // The member matches the resourceType, return the object itself
                            return Ok(FhirPathValue::Object(obj.clone()));
                        }
                    }
                }

                if let Some(value) = obj.get(member) {
                    Ok(FhirPathValue::from_json(value))
                } else {
                    // Check for FHIR choice type polymorphic access (e.g., value[x])
                    if let Some(obj_map) = obj.as_object() {
                        // Look for fields that start with the member name (e.g., "value" matches "valueBoolean")
                        for (key, value) in obj_map {
                            if key.starts_with(member) && key.len() > member.len() {
                                // Check if the next character after the member name is uppercase
                                // This ensures "value" matches "valueBoolean" but not "valueFoo" with lowercase
                                if let Some(next_char) = key.chars().nth(member.len()) {
                                    if next_char.is_uppercase() {
                                        return Ok(FhirPathValue::from_json(value));
                                    }
                                }
                            }
                        }
                    }
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
                    let item_context = context.with_this_value(item.clone());

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
                    let item_context = context.with_this_value(item.clone());

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
                let item_context = context.with_this_value(target.clone());

                self.evaluate_expression(projection_expr, &item_context)
            }
        }
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
            FhirPathValue::Collection(items) => {
                current_collection.extend(items.clone());
            }
            FhirPathValue::Empty => {
                return Ok(FhirPathValue::Empty);
            }
            value => {
                current_collection.push(value.clone());
            }
        }

        // Add the initial collection to results
        all_results.extend(current_collection.clone());

        loop {
            let mut next_collection = Vec::new();

            // Apply the projection expression to each item in the current collection
            for item in &current_collection {
                let item_context = context.with_this_value(item.clone());

                // Evaluate the projection expression in the item context
                let projection_result = self.evaluate_expression(projection_expr, &item_context)?;

                // Collect new items from the projection result
                match projection_result {
                    FhirPathValue::Collection(items) => {
                        for new_item in items {
                            // Only add items that haven't been seen before
                            if !all_results
                                .iter()
                                .any(|existing| FhirPathValue::equals_static(existing, &new_item))
                            {
                                next_collection.push(new_item.clone());
                                all_results.push(new_item);
                            }
                        }
                    }
                    FhirPathValue::Empty => {
                        // Don't add empty values
                    }
                    value => {
                        // Only add items that haven't been seen before
                        if !all_results
                            .iter()
                            .any(|existing| FhirPathValue::equals_static(existing, &value))
                        {
                            next_collection.push(value.clone());
                            all_results.push(value);
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

        // Extract type name from the parameter expression
        let type_name = match &parameters[0] {
            Expression::Term(Term::Invocation(Invocation::Member(name))) => name.clone(),
            _ => {
                // Try to evaluate the parameter and extract the type name
                let param_result = self.evaluate_expression(&parameters[0], context)?;
                match param_result {
                    FhirPathValue::String(type_str) => type_str,
                    _ => {
                        return Err(FhirPathError::FunctionError {
                            message: "ofType() function requires a type specifier as parameter"
                                .to_string(),
                        });
                    }
                }
            }
        };

        // Create a simple TypeSpecifier from the type name
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

        // Extract type name from the parameter expression
        let type_name = match &parameters[0] {
            Expression::Term(Term::Invocation(Invocation::Member(name))) => name.clone(),
            _ => {
                // Try to evaluate the parameter and extract the type name
                let param_result = self.evaluate_expression(&parameters[0], context)?;
                match param_result {
                    FhirPathValue::String(type_str) => type_str,
                    _ => {
                        return Err(FhirPathError::FunctionError {
                            message: "is() function requires a type specifier as parameter"
                                .to_string(),
                        });
                    }
                }
            }
        };

        // Create a simple TypeSpecifier from the type name
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

        // Extract type name from the parameter expression
        let type_name = match &parameters[0] {
            Expression::Term(Term::Invocation(Invocation::Member(name))) => name.clone(),
            _ => {
                // Try to evaluate the parameter and extract the type name
                let param_result = self.evaluate_expression(&parameters[0], context)?;
                match param_result {
                    FhirPathValue::String(type_str) => type_str,
                    _ => {
                        return Err(FhirPathError::FunctionError {
                            message: "as() function requires a type specifier as parameter"
                                .to_string(),
                        });
                    }
                }
            }
        };

        // Create a simple TypeSpecifier from the type name
        let type_specifier = TypeSpecifier {
            qualified_name: vec![type_name],
        };

        TypeEvaluator::evaluate_type_operation(target, &TypeOperator::As, &type_specifier)
    }
}

impl Default for FhirPathEvaluator {
    fn default() -> Self {
        Self::new()
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
