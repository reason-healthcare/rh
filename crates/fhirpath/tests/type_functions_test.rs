/// Type Function Tests
///
/// Tests for the is() and as() functions that provide backward compatibility
/// with the 'is' and 'as' operators. These functions provide the same functionality
/// as their operator counterparts but in callable function form.
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

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

    // Test string value with 'String' type
    let ast = parser
        .parse("string_value.is('String')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test string value with 'Integer' type (should be false)
    let ast = parser
        .parse("string_value.is('Integer')")
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

    // Test integer value with 'Integer' type
    let ast = parser
        .parse("integer_value.is('Integer')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test integer value with 'Boolean' type (should be false)
    let ast = parser
        .parse("integer_value.is('Boolean')")
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

    // Test boolean value with 'Boolean' type
    let ast = parser
        .parse("boolean_value.is('Boolean')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_is_function_with_number() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test number value with 'Number' type
    let ast = parser
        .parse("number_value.is('Number')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test number value with 'Decimal' type (should also be true)
    let ast = parser
        .parse("number_value.is('Decimal')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_as_function_with_string() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test string value cast as 'String' (should return the value)
    let ast = parser
        .parse("string_value.as('String')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::String("test".to_string()));

    // Test string value cast as 'Integer' (should return empty)
    let ast = parser
        .parse("string_value.as('Integer')")
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

    // Test integer value cast as 'Integer' (should return the value)
    let ast = parser
        .parse("integer_value.as('Integer')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Integer(42));

    // Test integer value cast as 'String' (should return empty)
    let ast = parser
        .parse("integer_value.as('String')")
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

    // Test boolean value cast as 'Boolean' (should return the value)
    let ast = parser
        .parse("boolean_value.as('Boolean')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_as_function_with_number() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test number value cast as 'Number' (should return the value)
    let ast = parser
        .parse("number_value.as('Number')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Number(123.456));

    // Test number value cast as 'Decimal' (should also return the value)
    let ast = parser
        .parse("number_value.as('Decimal')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Number(123.456));
}

#[test]
fn test_is_function_parameter_validation() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test is() function with no parameters (should error)
    let ast = parser
        .parse("string_value.is()")
        .expect("Failed to parse expression");
    let result = evaluator.evaluate(&ast, &context);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("is() function requires exactly one parameter"));

    // Test is() function with multiple parameters (should error)
    let ast = parser
        .parse("string_value.is('String', 'Integer')")
        .expect("Failed to parse expression");
    let result = evaluator.evaluate(&ast, &context);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("is() function requires exactly one parameter"));
}

#[test]
fn test_as_function_parameter_validation() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test as() function with no parameters (should error)
    let ast = parser
        .parse("string_value.as()")
        .expect("Failed to parse expression");
    let result = evaluator.evaluate(&ast, &context);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("as() function requires exactly one parameter"));

    // Test as() function with multiple parameters (should error)
    let ast = parser
        .parse("string_value.as('String', 'Integer')")
        .expect("Failed to parse expression");
    let result = evaluator.evaluate(&ast, &context);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("as() function requires exactly one parameter"));
}

#[test]
fn test_is_vs_is_operator_equivalence() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test that is() function produces same result as is operator
    let function_ast = parser
        .parse("string_value.is('String')")
        .expect("Failed to parse function expression");
    let function_result = evaluator
        .evaluate(&function_ast, &context)
        .expect("Failed to evaluate function");

    let operator_ast = parser
        .parse("string_value is String")
        .expect("Failed to parse operator expression");
    let operator_result = evaluator
        .evaluate(&operator_ast, &context)
        .expect("Failed to evaluate operator");

    assert_eq!(function_result, operator_result);
    assert_eq!(function_result, FhirPathValue::Boolean(true));
}

#[test]
fn test_as_vs_as_operator_equivalence() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test that as() function produces same result as as operator
    let function_ast = parser
        .parse("string_value.as('String')")
        .expect("Failed to parse function expression");
    let function_result = evaluator
        .evaluate(&function_ast, &context)
        .expect("Failed to evaluate function");

    let operator_ast = parser
        .parse("string_value as String")
        .expect("Failed to parse operator expression");
    let operator_result = evaluator
        .evaluate(&operator_ast, &context)
        .expect("Failed to evaluate operator");

    assert_eq!(function_result, operator_result);
    assert_eq!(function_result, FhirPathValue::String("test".to_string()));
}

#[test]
fn test_case_insensitive_type_names() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test that type names are case-insensitive
    let ast = parser
        .parse("string_value.is('string')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test with mixed case
    let ast = parser
        .parse("integer_value.is('INTEGER')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_chained_type_operations() {
    let context = create_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test chaining as() and is() functions
    let ast = parser
        .parse("string_value.as('String').is('String')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test chaining with failed cast
    let ast = parser
        .parse("string_value.as('Integer').is('Integer')")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    // as('Integer') returns empty, and empty.is('Integer') should return false
    assert_eq!(result, FhirPathValue::Boolean(false));
}
