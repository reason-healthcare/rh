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
    functions: std::collections::HashMap<String, Box<dyn Fn(&[FhirPathValue]) -> FhirPathResult<FhirPathValue>>>,
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
                let param_values: Result<Vec<_>, _> = parameters
                    .iter()
                    .map(|p| self.evaluate_expression(p, context))
                    .collect();
                let param_values = param_values?;
                self.evaluate_function_call(target, name, &param_values)
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
        _target: &FhirPathValue,
        name: &str,
        parameters: &[FhirPathValue],
    ) -> FhirPathResult<FhirPathValue> {
        if let Some(func) = self.functions.get(name) {
            func(parameters)
        } else {
            Err(FhirPathError::FunctionError {
                message: format!("Unknown function: {}", name),
            })
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

    /// Check if two values are equal
    fn values_equal(&self, left: &FhirPathValue, right: &FhirPathValue) -> bool {
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
        // TODO: Implement built-in functions like empty(), exists(), count(), etc.
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
}
