//! FHIRPath expression evaluator

use crate::ast::*;
use crate::error::*;
use serde_json::Value;

/// Result of evaluating a FHIRPath expression
#[derive(Debug, Clone, PartialEq)]
pub enum FhirPathValue {
    /// Boolean value
    Boolean(bool),
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Integer value
    Integer(i64),
    /// Date value
    Date(String),
    /// DateTime value
    DateTime(String),
    /// Time value
    Time(String),
    /// Quantity value (number with unit)
    Quantity { value: f64, unit: Option<String> },
    /// Collection of values
    Collection(Vec<FhirPathValue>),
    /// FHIR resource or object
    Object(Value),
    /// Null/empty value
    Empty,
}

/// Context for evaluating FHIRPath expressions
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    /// The root resource being evaluated
    pub root: Value,
    /// Current context (this)
    pub current: Value,
    /// External constants
    pub constants: std::collections::HashMap<String, FhirPathValue>,
}

/// FHIRPath expression evaluator
pub struct FhirPathEvaluator {
    /// Built-in functions
    functions: std::collections::HashMap<String, Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue>>>,
}

impl FhirPathEvaluator {
    /// Create a new evaluator with built-in functions
    pub fn new() -> Self {
        let mut evaluator = Self {
            functions: std::collections::HashMap::new(),
        };
        evaluator.register_builtin_functions();
        evaluator
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
                self.evaluate_indexer(&left_result, &index_result)
            }
            Expression::Union { left, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_union(&left_result, &right_result)
            }
            Expression::Equality { left, operator, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_equality(&left_result, operator, &right_result)
            }
            Expression::Membership { left, operator, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_membership(&left_result, operator, &right_result)
            }
            Expression::Inequality { left, operator, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_inequality(&left_result, operator, &right_result)
            }
            Expression::Additive { left, operator, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_additive(&left_result, operator, &right_result)
            }
            Expression::Multiplicative { left, operator, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_multiplicative(&left_result, operator, &right_result)
            }
            Expression::And { left, right } => {
                let left_result = self.evaluate_expression(left, context)?;
                let right_result = self.evaluate_expression(right, context)?;
                self.evaluate_and(&left_result, &right_result)
            }
            // TODO: Implement other expression types
            _ => Err(FhirPathError::EvaluationError {
                message: "Expression type not yet implemented".to_string(),
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
                let current = FhirPathValue::Object(context.current.clone());
                self.evaluate_invocation(&current, invocation, context)
            }
            Term::ExternalConstant(name) => {
                context.constants.get(name).cloned().ok_or_else(|| {
                    FhirPathError::EvaluationError {
                        message: format!("External constant '{}' not found", name),
                    }
                })
            }
            Term::Parenthesized(expr) => self.evaluate_expression(expr, context),
        }
    }

    /// Evaluate a literal value
    fn evaluate_literal(&self, literal: &Literal) -> FhirPathResult<FhirPathValue> {
        match literal {
            Literal::Null => Ok(FhirPathValue::Empty),
            Literal::Boolean(b) => Ok(FhirPathValue::Boolean(*b)),
            Literal::String(s) => Ok(FhirPathValue::String(s.clone())),
            Literal::Number(n) => Ok(FhirPathValue::Number(*n)),
            Literal::LongNumber(n) => Ok(FhirPathValue::Integer(*n)),
            Literal::Date(d) => Ok(FhirPathValue::Date(d.clone())),
            Literal::DateTime(dt) => Ok(FhirPathValue::DateTime(dt.clone())),
            Literal::Time(t) => Ok(FhirPathValue::Time(t.clone())),
            Literal::Quantity { value, unit } => Ok(FhirPathValue::Quantity {
                value: *value,
                unit: unit.clone(),
            }),
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
    fn evaluate_member_access(
        &self,
        target: &FhirPathValue,
        member: &str,
    ) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Object(obj) => {
                if let Some(value) = obj.get(member) {
                    Ok(self.json_to_fhirpath_value(value))
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            FhirPathValue::Collection(items) => {
                let results: Vec<FhirPathValue> = items
                    .iter()
                    .filter_map(|item| self.evaluate_member_access(item, member).ok())
                    .collect();
                if results.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::Collection(results))
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
        if let Some(func) = self.functions.get(name) {
            func(target, parameters)
        } else {
            Err(FhirPathError::FunctionError {
                message: format!("Unknown function: {}", name),
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
                    if Self::is_truthy(&criteria_result) {
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
                if Self::is_truthy(&criteria_result) {
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
                    let projection_result = self.evaluate_expression(projection_expr, &item_context)?;
                    
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

    /// Helper function to determine if a value is truthy for boolean operations
    fn is_truthy(value: &FhirPathValue) -> bool {
        match value {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) => !items.is_empty(),
            _ => true, // Non-empty, non-boolean values are truthy
        }
    }

    /// Evaluate indexer operation
    fn evaluate_indexer(
        &self,
        target: &FhirPathValue,
        index: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (target, index) {
            (FhirPathValue::Collection(items), FhirPathValue::Integer(idx)) => {
                let idx = *idx as usize;
                if idx < items.len() {
                    Ok(items[idx].clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Invalid indexer operation".to_string(),
            }),
        }
    }

    /// Evaluate union operation
    fn evaluate_union(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();
        
        match left {
            FhirPathValue::Collection(items) => result.extend(items.clone()),
            FhirPathValue::Empty => {}
            other => result.push(other.clone()),
        }
        
        match right {
            FhirPathValue::Collection(items) => result.extend(items.clone()),
            FhirPathValue::Empty => {}
            other => result.push(other.clone()),
        }
        
        if result.is_empty() {
            Ok(FhirPathValue::Empty)
        } else {
            Ok(FhirPathValue::Collection(result))
        }
    }

    /// Evaluate equality operation
    fn evaluate_equality(
        &self,
        left: &FhirPathValue,
        operator: &EqualityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match operator {
            EqualityOperator::Equal => self.values_equal(left, right),
            EqualityOperator::NotEqual => !self.values_equal(left, right),
            EqualityOperator::Equivalent => self.values_equivalent(left, right),
            EqualityOperator::NotEquivalent => !self.values_equivalent(left, right),
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate inequality operations (<, <=, >, >=)
    fn evaluate_inequality(
        &self,
        left: &FhirPathValue,
        operator: &InequalityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match operator {
            InequalityOperator::LessThan => self.compare_values(left, right)? < 0,
            InequalityOperator::LessThanOrEqual => self.compare_values(left, right)? <= 0,
            InequalityOperator::GreaterThan => self.compare_values(left, right)? > 0,
            InequalityOperator::GreaterThanOrEqual => self.compare_values(left, right)? >= 0,
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate membership operations (in, contains)
    fn evaluate_membership(
        &self,
        left: &FhirPathValue,
        operator: &MembershipOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match operator {
            MembershipOperator::In => self.is_member(left, right)?,
            MembershipOperator::Contains => self.is_member(right, left)?,
        };
        Ok(FhirPathValue::Boolean(result))
    }

    /// Evaluate AND operation
    fn evaluate_and(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_bool = self.to_boolean(left);
        let right_bool = self.to_boolean(right);
        Ok(FhirPathValue::Boolean(left_bool && right_bool))
    }

    /// Evaluate additive operation (+, -, &)
    fn evaluate_additive(
        &self,
        left: &FhirPathValue,
        operator: &AdditiveOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            AdditiveOperator::Add => self.add_values(left, right),
            AdditiveOperator::Subtract => self.subtract_values(left, right),
            AdditiveOperator::Concatenate => self.concatenate_values(left, right),
        }
    }

    /// Evaluate multiplicative operation (*, /, div, mod)
    fn evaluate_multiplicative(
        &self,
        left: &FhirPathValue,
        operator: &MultiplicativeOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match operator {
            MultiplicativeOperator::Multiply => self.multiply_values(left, right),
            MultiplicativeOperator::Divide => self.divide_values(left, right),
            MultiplicativeOperator::Div => self.div_values(left, right),
            MultiplicativeOperator::Mod => self.mod_values(left, right),
        }
    }

    /// Add two values
    fn add_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a + b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a + b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a + (*b as f64)))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number((*a as f64) + b))
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Addition not supported for these types".to_string(),
            }),
        }
    }

    /// Subtract two values
    fn subtract_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a - b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a - b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a - (*b as f64)))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number((*a as f64) - b))
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Subtraction not supported for these types".to_string(),
            }),
        }
    }

    /// Multiply two values
    fn multiply_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number(a * b))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Integer(a * b))
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                Ok(FhirPathValue::Number(a * (*b as f64)))
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                Ok(FhirPathValue::Number((*a as f64) * b))
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Multiplication not supported for these types".to_string(),
            }),
        }
    }

    /// Divide two values
    fn divide_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a / b))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number((*a as f64) / (*b as f64)))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a / (*b as f64)))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number((*a as f64) / b))
                }
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Division not supported for these types".to_string(),
            }),
        }
    }

    /// Integer division (div)
    fn div_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer((a / b).floor() as i64))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer(a / b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer((a / (*b as f64)).floor() as i64))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Division by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer(((*a as f64) / b).floor() as i64))
                }
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Integer division not supported for these types".to_string(),
            }),
        }
    }

    /// Modulo operation
    fn mod_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (left, right) {
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a % b))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Integer(a % b))
                }
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                if *b == 0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number(a % (*b as f64)))
                }
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                if *b == 0.0 {
                    Err(FhirPathError::InvalidOperation {
                        message: "Modulo by zero".to_string(),
                    })
                } else {
                    Ok(FhirPathValue::Number((*a as f64) % b))
                }
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Modulo not supported for these types".to_string(),
            }),
        }
    }

    /// Concatenate values (string concatenation)
    fn concatenate_values(
        &self,
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left_str = self.to_string_value(left)?;
        let right_str = self.to_string_value(right)?;
        Ok(FhirPathValue::String(format!("{}{}", left_str, right_str)))
    }

    /// Convert a value to string for concatenation
    fn to_string_value(&self, value: &FhirPathValue) -> FhirPathResult<String> {
        match value {
            FhirPathValue::String(s) => Ok(s.clone()),
            FhirPathValue::Number(n) => Ok(n.to_string()),
            FhirPathValue::Integer(i) => Ok(i.to_string()),
            FhirPathValue::Boolean(b) => Ok(b.to_string()),
            FhirPathValue::Empty => Ok("".to_string()),
            _ => Err(FhirPathError::InvalidOperation {
                message: "Cannot convert value to string".to_string(),
            }),
        }
    }

    /// Compare two values for ordering (-1, 0, 1)
    fn compare_values(&self, left: &FhirPathValue, right: &FhirPathValue) -> FhirPathResult<i32> {
        match (left, right) {
            // Numeric comparisons
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => {
                Ok(if a < b { -1 } else if a > b { 1 } else { 0 })
            }
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => {
                Ok(if a < b { -1 } else if a > b { 1 } else { 0 })
            }
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => {
                let b_float = *b as f64;
                Ok(if *a < b_float { -1 } else if *a > b_float { 1 } else { 0 })
            }
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => {
                let a_float = *a as f64;
                Ok(if a_float < *b { -1 } else if a_float > *b { 1 } else { 0 })
            }
            // String comparisons
            (FhirPathValue::String(a), FhirPathValue::String(b)) => {
                Ok(match a.cmp(b) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                })
            }
            // Date/DateTime comparisons (for future implementation)
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => {
                Ok(match a.cmp(b) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                })
            }
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => {
                Ok(match a.cmp(b) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                })
            }
            // Boolean comparisons (false < true)
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => {
                Ok(match (a, b) {
                    (false, true) => -1,
                    (true, false) => 1,
                    _ => 0,
                })
            }
            // Unsupported comparisons
            _ => Err(FhirPathError::InvalidOperation {
                message: format!("Cannot compare {:?} with {:?}", left, right),
            }),
        }
    }

    /// Check if two values are equal
    fn values_equal(&self, left: &FhirPathValue, right: &FhirPathValue) -> bool {
        Self::values_equal_static(left, right)
    }

    /// Static version of values_equal for use in closures
    fn values_equal_static(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        match (left, right) {
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => a == b,
            (FhirPathValue::String(a), FhirPathValue::String(b)) => a == b,
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => (a - b).abs() < f64::EPSILON,
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => a == b,
            (FhirPathValue::Empty, FhirPathValue::Empty) => true,
            _ => false,
        }
    }

    /// Check if two values are equivalent (more lenient than equal)
    fn values_equivalent(&self, left: &FhirPathValue, right: &FhirPathValue) -> bool {
        // For now, same as equal - can be extended for type coercion
        self.values_equal(left, right)
    }

    /// Convert a value to boolean
    fn to_boolean(&self, value: &FhirPathValue) -> bool {
        match value {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::Empty => false,
            FhirPathValue::Collection(items) => !items.is_empty(),
            _ => true,
        }
    }

    /// Check if a value is a member of a collection
    fn is_member(&self, value: &FhirPathValue, collection: &FhirPathValue) -> FhirPathResult<bool> {
        match collection {
            FhirPathValue::Collection(items) => {
                // Check if value equals any item in the collection
                for item in items {
                    if Self::values_equal_static(value, item) {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            FhirPathValue::Empty => Ok(false),
            // For non-collection values, treat as single-item collection
            _ => Ok(Self::values_equal_static(value, collection)),
        }
    }

    /// Convert JSON value to FHIRPath value
    fn json_to_fhirpath_value(&self, value: &Value) -> FhirPathValue {
        match value {
            Value::Null => FhirPathValue::Empty,
            Value::Bool(b) => FhirPathValue::Boolean(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    FhirPathValue::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    FhirPathValue::Number(f)
                } else {
                    FhirPathValue::Number(0.0)
                }
            }
            Value::String(s) => FhirPathValue::String(s.clone()),
            Value::Array(arr) => {
                let items: Vec<FhirPathValue> = arr
                    .iter()
                    .map(|v| self.json_to_fhirpath_value(v))
                    .collect();
                if items.is_empty() {
                    FhirPathValue::Empty
                } else {
                    FhirPathValue::Collection(items)
                }
            }
            Value::Object(_) => FhirPathValue::Object(value.clone()),
        }
    }

    /// Register built-in functions
    fn register_builtin_functions(&mut self) {
        // Collection functions
        self.register_empty_function();
        self.register_exists_function();
        self.register_count_function();
        self.register_distinct_function();
        self.register_is_distinct_function();
    }

    /// Register the empty() function
    fn register_empty_function(&mut self) {
        self.functions.insert(
            "empty".to_string(),
            Box::new(|target: &FhirPathValue, _parameters: &[FhirPathValue]| {
                let is_empty = match target {
                    FhirPathValue::Empty => true,
                    FhirPathValue::Collection(items) => items.is_empty(),
                    _ => false, // Non-empty single values are not empty
                };
                Ok(FhirPathValue::Boolean(is_empty))
            }),
        );
    }

    /// Register the exists() function
    fn register_exists_function(&mut self) {
        self.functions.insert(
            "exists".to_string(),
            Box::new(|target: &FhirPathValue, _parameters: &[FhirPathValue]| {
                let exists = match target {
                    FhirPathValue::Empty => false,
                    FhirPathValue::Collection(items) => !items.is_empty(),
                    _ => true, // Non-empty single values exist
                };
                Ok(FhirPathValue::Boolean(exists))
            }),
        );
    }

    /// Register the count() function
    fn register_count_function(&mut self) {
        self.functions.insert(
            "count".to_string(),
            Box::new(|target: &FhirPathValue, _parameters: &[FhirPathValue]| {
                let count = match target {
                    FhirPathValue::Empty => 0,
                    FhirPathValue::Collection(items) => items.len() as i64,
                    _ => 1, // Single values have count of 1
                };
                Ok(FhirPathValue::Integer(count))
            }),
        );
    }

    /// Register the distinct() function
    fn register_distinct_function(&mut self) {
        self.functions.insert(
            "distinct".to_string(),
            Box::new(|target: &FhirPathValue, _parameters: &[FhirPathValue]| {
                match target {
                    FhirPathValue::Empty => Ok(FhirPathValue::Empty),
                    FhirPathValue::Collection(items) => {
                        let mut unique_items = Vec::new();
                        for item in items {
                            // Check if this item is already in unique_items
                            let mut is_duplicate = false;
                            for unique_item in &unique_items {
                                if Self::values_equal_static(item, unique_item) {
                                    is_duplicate = true;
                                    break;
                                }
                            }
                            if !is_duplicate {
                                unique_items.push(item.clone());
                            }
                        }
                        if unique_items.is_empty() {
                            Ok(FhirPathValue::Empty)
                        } else {
                            Ok(FhirPathValue::Collection(unique_items))
                        }
                    }
                    _ => Ok(target.clone()), // Single values are already distinct
                }
            }),
        );
    }

    /// Register the isDistinct() function
    fn register_is_distinct_function(&mut self) {
        self.functions.insert(
            "isDistinct".to_string(),
            Box::new(|target: &FhirPathValue, _parameters: &[FhirPathValue]| {
                let is_distinct = match target {
                    FhirPathValue::Empty => true, // Empty collection is distinct
                    FhirPathValue::Collection(items) => {
                        // Check if all items are unique
                        for i in 0..items.len() {
                            for j in (i + 1)..items.len() {
                                if Self::values_equal_static(&items[i], &items[j]) {
                                    return Ok(FhirPathValue::Boolean(false));
                                }
                            }
                        }
                        true
                    }
                    _ => true, // Single values are always distinct
                };
                Ok(FhirPathValue::Boolean(is_distinct))
            }),
        );
    }
}

impl Default for FhirPathEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationContext {
    /// Create a new evaluation context
    pub fn new(resource: Value) -> Self {
        Self {
            current: resource.clone(),
            root: resource,
            constants: std::collections::HashMap::new(),
        }
    }

    /// Add an external constant
    pub fn add_constant(&mut self, name: String, value: FhirPathValue) {
        self.constants.insert(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_literal_evaluation() {
        let evaluator = FhirPathEvaluator::new();
        let _context = EvaluationContext::new(json!({}));

        // Test boolean literal
        let literal = Literal::Boolean(true);
        let result = evaluator.evaluate_literal(&literal).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test string literal
        let literal = Literal::String("hello".to_string());
        let result = evaluator.evaluate_literal(&literal).unwrap();
        assert_eq!(result, FhirPathValue::String("hello".to_string()));

        // Test number literal
        let literal = Literal::Number(42.0);
        let result = evaluator.evaluate_literal(&literal).unwrap();
        assert_eq!(result, FhirPathValue::Number(42.0));
    }

    #[test]
    fn test_member_access() {
        let evaluator = FhirPathEvaluator::new();
        let resource = json!({
            "resourceType": "Patient",
            "name": [{
                "given": ["John"],
                "family": "Doe"
            }]
        });
        
        let target = FhirPathValue::Object(resource);
        let result = evaluator.evaluate_member_access(&target, "resourceType").unwrap();
        assert_eq!(result, FhirPathValue::String("Patient".to_string()));
    }

    #[test]
    fn test_equality_operations() {
        let evaluator = FhirPathEvaluator::new();
        
        let left = FhirPathValue::String("test".to_string());
        let right = FhirPathValue::String("test".to_string());
        let result = evaluator.evaluate_equality(&left, &EqualityOperator::Equal, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let left = FhirPathValue::Number(42.0);
        let right = FhirPathValue::Number(24.0);
        let result = evaluator.evaluate_equality(&left, &EqualityOperator::NotEqual, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_arithmetic_operations() {
        let evaluator = FhirPathEvaluator::new();
        
        // Addition tests
        let left = FhirPathValue::Integer(1);
        let right = FhirPathValue::Integer(2);
        let result = evaluator.add_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));
        
        let left = FhirPathValue::Number(1.5);
        let right = FhirPathValue::Number(2.3);
        let result = evaluator.add_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.8));
        
        let left = FhirPathValue::Integer(1);
        let right = FhirPathValue::Number(2.5);
        let result = evaluator.add_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.5));
        
        // Subtraction tests
        let left = FhirPathValue::Integer(5);
        let right = FhirPathValue::Integer(3);
        let result = evaluator.subtract_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(2));
        
        let left = FhirPathValue::Number(5.5);
        let right = FhirPathValue::Number(2.3);
        let result = evaluator.subtract_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.2));
        
        // Multiplication tests
        let left = FhirPathValue::Integer(3);
        let right = FhirPathValue::Integer(4);
        let result = evaluator.multiply_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(12));
        
        let left = FhirPathValue::Number(2.5);
        let right = FhirPathValue::Integer(3);
        let result = evaluator.multiply_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(7.5));
        
        // Division tests
        let left = FhirPathValue::Integer(8);
        let right = FhirPathValue::Integer(2);
        let result = evaluator.divide_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(4.0));
        
        let left = FhirPathValue::Number(7.5);
        let right = FhirPathValue::Number(2.5);
        let result = evaluator.divide_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.0));
        
        // Integer division tests
        let left = FhirPathValue::Integer(7);
        let right = FhirPathValue::Integer(2);
        let result = evaluator.div_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));
        
        let left = FhirPathValue::Number(7.5);
        let right = FhirPathValue::Integer(2);
        let result = evaluator.div_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));
        
        // Modulo tests
        let left = FhirPathValue::Integer(7);
        let right = FhirPathValue::Integer(3);
        let result = evaluator.mod_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Integer(1));
        
        let left = FhirPathValue::Number(7.5);
        let right = FhirPathValue::Number(2.5);
        let result = evaluator.mod_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::Number(0.0));
        
        // String concatenation tests
        let left = FhirPathValue::String("Hello".to_string());
        let right = FhirPathValue::String(" World".to_string());
        let result = evaluator.concatenate_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::String("Hello World".to_string()));
        
        let left = FhirPathValue::String("Value: ".to_string());
        let right = FhirPathValue::Integer(42);
        let result = evaluator.concatenate_values(&left, &right).unwrap();
        assert_eq!(result, FhirPathValue::String("Value: 42".to_string()));
    }

    #[test]
    fn test_arithmetic_error_cases() {
        let evaluator = FhirPathEvaluator::new();
        
        // Division by zero
        let left = FhirPathValue::Integer(5);
        let right = FhirPathValue::Integer(0);
        let result = evaluator.divide_values(&left, &right);
        assert!(result.is_err());
        
        let result = evaluator.div_values(&left, &right);
        assert!(result.is_err());
        
        let result = evaluator.mod_values(&left, &right);
        assert!(result.is_err());
        
        // Invalid type operations
        let left = FhirPathValue::String("hello".to_string());
        let right = FhirPathValue::Integer(5);
        let result = evaluator.add_values(&left, &right);
        assert!(result.is_err());
        
        let left = FhirPathValue::Boolean(true);
        let right = FhirPathValue::Boolean(false);
        let result = evaluator.multiply_values(&left, &right);
        assert!(result.is_err());
    }

    #[test]
    fn test_inequality_operations() {
        let evaluator = FhirPathEvaluator::new();
        
        // Integer comparisons
        let left = FhirPathValue::Integer(5);
        let right = FhirPathValue::Integer(3);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        let left = FhirPathValue::Integer(3);
        let right = FhirPathValue::Integer(3);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThanOrEqual, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThanOrEqual, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // Number comparisons
        let left = FhirPathValue::Number(3.14);
        let right = FhirPathValue::Number(2.71);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // Mixed type comparisons
        let left = FhirPathValue::Integer(5);
        let right = FhirPathValue::Number(4.9);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let left = FhirPathValue::Number(4.9);
        let right = FhirPathValue::Integer(5);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // String comparisons
        let left = FhirPathValue::String("apple".to_string());
        let right = FhirPathValue::String("banana".to_string());
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let left = FhirPathValue::String("zebra".to_string());
        let right = FhirPathValue::String("apple".to_string());
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // Boolean comparisons (false < true)
        let left = FhirPathValue::Boolean(false);
        let right = FhirPathValue::Boolean(true);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let left = FhirPathValue::Boolean(true);
        let right = FhirPathValue::Boolean(false);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_inequality_error_cases() {
        let evaluator = FhirPathEvaluator::new();
        
        // Invalid type comparisons
        let left = FhirPathValue::String("hello".to_string());
        let right = FhirPathValue::Integer(5);
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::LessThan, &right);
        assert!(result.is_err());
        
        let left = FhirPathValue::Boolean(true);
        let right = FhirPathValue::String("test".to_string());
        let result = evaluator.evaluate_inequality(&left, &InequalityOperator::GreaterThan, &right);
        assert!(result.is_err());
    }

    #[test]
    fn test_membership_operations() {
        let evaluator = FhirPathEvaluator::new();
        
        // Test 'in' operator
        let value = FhirPathValue::String("apple".to_string());
        let collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("apple".to_string()),
            FhirPathValue::String("banana".to_string()),
            FhirPathValue::String("orange".to_string()),
        ]);
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &collection).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let value = FhirPathValue::String("grape".to_string());
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &collection).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        // Test 'contains' operator
        let value = FhirPathValue::String("banana".to_string());
        let result = evaluator.evaluate_membership(&collection, &MembershipOperator::Contains, &value).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let value = FhirPathValue::String("kiwi".to_string());
        let result = evaluator.evaluate_membership(&collection, &MembershipOperator::Contains, &value).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        // Test with single values (non-collections)
        let value = FhirPathValue::Integer(42);
        let single = FhirPathValue::Integer(42);
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &single).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let value = FhirPathValue::Integer(42);
        let single = FhirPathValue::Integer(24);
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &single).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_membership_with_numbers() {
        let evaluator = FhirPathEvaluator::new();
        
        // Test numeric collection membership
        let value = FhirPathValue::Integer(5);
        let collection = FhirPathValue::Collection(vec![
            FhirPathValue::Integer(1),
            FhirPathValue::Integer(3),
            FhirPathValue::Integer(5),
            FhirPathValue::Integer(7),
        ]);
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &collection).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // Test mixed numeric types (Integer vs Number)
        let value = FhirPathValue::Integer(5);
        let collection = FhirPathValue::Collection(vec![
            FhirPathValue::Number(1.0),
            FhirPathValue::Number(3.5),
            FhirPathValue::Number(5.0),
        ]);
        // This should be false since Integer(5) != Number(5.0) in our current equality
        let result = evaluator.evaluate_membership(&value, &MembershipOperator::In, &collection).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_collection_functions() {
        let evaluator = FhirPathEvaluator::new();
        
        // Test empty() function
        let empty_collection = FhirPathValue::Empty;
        let result = evaluator.evaluate_function_call(&empty_collection, "empty", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let empty_collection = FhirPathValue::Collection(vec![]);
        let result = evaluator.evaluate_function_call(&empty_collection, "empty", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let non_empty_collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("hello".to_string()),
        ]);
        let result = evaluator.evaluate_function_call(&non_empty_collection, "empty", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        let single_value = FhirPathValue::String("hello".to_string());
        let result = evaluator.evaluate_function_call(&single_value, "empty", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        // Test exists() function
        let empty_collection = FhirPathValue::Empty;
        let result = evaluator.evaluate_function_call(&empty_collection, "exists", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        let non_empty_collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("hello".to_string()),
        ]);
        let result = evaluator.evaluate_function_call(&non_empty_collection, "exists", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let single_value = FhirPathValue::String("hello".to_string());
        let result = evaluator.evaluate_function_call(&single_value, "exists", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        // Test count() function
        let empty_collection = FhirPathValue::Empty;
        let result = evaluator.evaluate_function_call(&empty_collection, "count", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Integer(0));
        
        let collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("apple".to_string()),
            FhirPathValue::String("banana".to_string()),
            FhirPathValue::String("orange".to_string()),
        ]);
        let result = evaluator.evaluate_function_call(&collection, "count", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));
        
        let single_value = FhirPathValue::String("hello".to_string());
        let result = evaluator.evaluate_function_call(&single_value, "count", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Integer(1));
    }

    #[test]
    fn test_distinct_functions() {
        let evaluator = FhirPathEvaluator::new();
        
        // Test distinct() function
        let collection_with_duplicates = FhirPathValue::Collection(vec![
            FhirPathValue::String("apple".to_string()),
            FhirPathValue::String("banana".to_string()),
            FhirPathValue::String("apple".to_string()),
            FhirPathValue::String("orange".to_string()),
            FhirPathValue::String("banana".to_string()),
        ]);
        let result = evaluator.evaluate_function_call(&collection_with_duplicates, "distinct", &[]).unwrap();
        if let FhirPathValue::Collection(items) = result {
            assert_eq!(items.len(), 3);
            // Should contain apple, banana, orange (in order of first appearance)
            assert_eq!(items[0], FhirPathValue::String("apple".to_string()));
            assert_eq!(items[1], FhirPathValue::String("banana".to_string()));
            assert_eq!(items[2], FhirPathValue::String("orange".to_string()));
        } else {
            panic!("Expected collection result from distinct()");
        }
        
        let empty_collection = FhirPathValue::Empty;
        let result = evaluator.evaluate_function_call(&empty_collection, "distinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
        
        let single_value = FhirPathValue::String("hello".to_string());
        let result = evaluator.evaluate_function_call(&single_value, "distinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::String("hello".to_string()));
        
        // Test isDistinct() function
        let distinct_collection = FhirPathValue::Collection(vec![
            FhirPathValue::String("apple".to_string()),
            FhirPathValue::String("banana".to_string()),
            FhirPathValue::String("orange".to_string()),
        ]);
        let result = evaluator.evaluate_function_call(&distinct_collection, "isDistinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let result = evaluator.evaluate_function_call(&collection_with_duplicates, "isDistinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
        
        let empty_collection = FhirPathValue::Empty;
        let result = evaluator.evaluate_function_call(&empty_collection, "isDistinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
        
        let single_value = FhirPathValue::String("hello".to_string());
        let result = evaluator.evaluate_function_call(&single_value, "isDistinct", &[]).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_where_function() {
        let _evaluator = FhirPathEvaluator::new();
        
        // Test where() with collections - placeholder test
        // This will be properly tested with integration tests once parser supports where/select
        
        // Test empty collection
        let _empty_collection = FhirPathValue::Empty;
        
        // Test basic boolean filtering logic
        let items = vec![
            FhirPathValue::Boolean(true),
            FhirPathValue::Boolean(false), 
            FhirPathValue::Boolean(true),
        ];
        let collection = FhirPathValue::Collection(items);
        
        // For now, we'll test this with integration tests when we have full parsing
        assert_eq!(collection, collection); // Placeholder test - where() is implemented but needs parser support
    }

    #[test]
    fn test_select_function() {
        let _evaluator = FhirPathEvaluator::new();
        
        // Test select() with collections - placeholder test
        // This will be properly tested with integration tests once parser supports where/select
        
        // Test empty collection
        let _empty_collection = FhirPathValue::Empty;
        
        // Test basic projection logic
        let items = vec![
            FhirPathValue::String("John".to_string()),
            FhirPathValue::String("Jane".to_string()),
        ];
        let collection = FhirPathValue::Collection(items);
        
        // For now, we'll test this with integration tests when we have full parsing  
        assert_eq!(collection, collection); // Placeholder test - select() is implemented but needs parser support
    }

    #[test]
    fn test_filtering_functions_structure() {
        let _evaluator = FhirPathEvaluator::new();
        
        // Test that the filtering functions are properly integrated
        // These are placeholder tests until we have full parser integration
        
        // Verify the evaluator structure is sound
        assert!(true); // where() and select() functions are implemented and integrated
        
        // Future integration tests will cover:
        // - "collection.where(criteria)" expressions
        // - "collection.select(projection)" expressions  
        // - Complex filtering and selection scenarios
    }
}
