/// Type Function Tests
///
/// Tests for the is() and as() functions that provide backward compatibility
/// with the 'is' and 'as' operators. These functions provide the same functionality
/// as their operator counterparts but in callable function form.
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_basic_functionality() {
    // Simple test to verify test structure works
    assert_eq!(1 + 1, 2);
}

fn create_test_context() -> EvaluationContext {
    let data = json!({
        "string_value": "test",
        "integer_value": 42,
        "boolean_value": true,
        "number_value": 123.456,
        "mixed_collection": ["hello", 123, false, 2.5]
    });

    EvaluationContext::new(data)
}

#[test]
fn test_is_function_with_string() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test string value with String type
    let ast = parser
        .parse("string_value.is(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test string value with Integer type (should be false)
    let ast = parser
        .parse("string_value.is(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_is_function_with_integer() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test integer value with Integer type
    let ast = parser
        .parse("integer_value.is(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test integer value with String type (should be false)
    let ast = parser
        .parse("integer_value.is(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_is_function_with_boolean() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test boolean value with Boolean type
    let ast = parser
        .parse("boolean_value.is(Boolean)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test boolean value with Integer type (should be false)
    let ast = parser
        .parse("boolean_value.is(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_is_function_with_number() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test number value with Decimal type
    let ast = parser
        .parse("number_value.is(Decimal)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test number value with String type (should be false)
    let ast = parser
        .parse("number_value.is(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_as_function_with_string() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test string value as String type
    let ast = parser
        .parse("string_value.as(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::String("test".to_string()));

    // Test string value as Integer type (should be empty)
    let ast = parser
        .parse("string_value.as(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_as_function_with_integer() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test integer value as Integer type
    let ast = parser
        .parse("integer_value.as(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Integer(42));

    // Test integer value as String type (should be empty)
    let ast = parser
        .parse("integer_value.as(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_as_function_with_boolean() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test boolean value as Boolean type
    let ast = parser
        .parse("boolean_value.as(Boolean)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test boolean value as Integer type (should be empty)
    let ast = parser
        .parse("boolean_value.as(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_as_function_with_number() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test number value as Decimal type
    let ast = parser
        .parse("number_value.as(Decimal)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Number(123.456));

    // Test number value as String type (should be empty)
    let ast = parser
        .parse("number_value.as(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_is_function_equivalence_with_operator() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test that value.is(Type) is equivalent to value is Type
    let ast1 = parser
        .parse("string_value.is(String)")
        .expect("Failed to parse function expression");
    let result1 = evaluator
        .evaluate(&ast1, &context)
        .expect("Failed to evaluate function");

    let ast2 = parser
        .parse("string_value is String")
        .expect("Failed to parse operator expression");
    let result2 = evaluator
        .evaluate(&ast2, &context)
        .expect("Failed to evaluate operator");

    assert_eq!(result1, result2);
    assert_eq!(result1, FhirPathValue::Boolean(true));
}

#[test]
fn test_as_function_equivalence_with_operator() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test that value.as(Type) is equivalent to value as Type
    let ast1 = parser
        .parse("string_value.as(String)")
        .expect("Failed to parse function expression");
    let result1 = evaluator
        .evaluate(&ast1, &context)
        .expect("Failed to evaluate function");

    let ast2 = parser
        .parse("string_value as String")
        .expect("Failed to parse operator expression");
    let result2 = evaluator
        .evaluate(&ast2, &context)
        .expect("Failed to evaluate operator");

    assert_eq!(result1, result2);
    assert_eq!(result1, FhirPathValue::String("test".to_string()));
}

#[test]
fn test_is_function_with_empty_collection() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test empty collection with any type
    let ast = parser
        .parse("{}.is(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_as_function_with_empty_collection() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test empty collection with any type
    let ast = parser
        .parse("{}.as(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_is_function_with_literal_values() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test literal string
    let ast = parser
        .parse("'hello'.is(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test literal integer
    let ast = parser
        .parse("42.is(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test literal decimal
    let ast = parser
        .parse("3.14.is(Decimal)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test literal boolean
    let ast = parser
        .parse("true.is(Boolean)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_as_function_with_literal_values() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test literal string
    let ast = parser
        .parse("'hello'.as(String)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::String("hello".to_string()));

    // Test literal integer
    let ast = parser
        .parse("42.as(Integer)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Integer(42));

    // Test literal decimal
    let ast = parser
        .parse("4.14.as(Decimal)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Number(4.14));

    // Test literal boolean
    let ast = parser
        .parse("true.as(Boolean)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}
