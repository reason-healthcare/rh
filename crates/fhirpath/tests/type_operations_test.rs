//! Integration tests for type operations (is, as)

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_type_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test is operator
    let expr = parser.parse("true is Boolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("'hello' is String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("42 is Integer").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test type mismatches
    let expr = parser.parse("true is String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test as operator (successful cast)
    let expr = parser.parse("'hello' as String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::String(s) if s == "hello"));

    // Test as operator (failed cast)
    let expr = parser.parse("true as String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    println!("All type operation tests passed!");
}

#[test]
fn test_system_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test System namespace types
    let expr = parser.parse("true is System.Boolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("'hello' is System.String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    println!("System type tests passed!");
}

#[test]
fn test_type_precedence() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test that type operations have correct precedence with other operators
    // Let's test with a clear example where we can verify precedence
    let expr = parser.parse("42 is String and true is Boolean").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!("Result of '42 is String and true is Boolean': {:?}", result);
    // This should be (42 is String) and (true is Boolean) = false and true = false
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test another case
    let expr = parser.parse("42 is Integer and 'hello' is String").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!(
        "Result of '42 is Integer and hello is String': {:?}",
        result
    );
    // This should be (42 is Integer) and ('hello' is String) = true and true = true
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    println!("Type precedence tests passed!");
}
