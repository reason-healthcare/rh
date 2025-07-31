//! Integration tests for implies operation

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_implies_basic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test case 1: true implies true -> true
    let expr = parser.parse("true implies true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test case 2: true implies false -> false
    let expr = parser.parse("true implies false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test case 3: false implies true -> true
    let expr = parser.parse("false implies true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test case 4: false implies false -> true
    let expr = parser.parse("false implies false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    println!("Basic implies tests passed!");
}

#[test]
fn test_implies_with_expressions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test with expressions that evaluate to boolean
    let expr = parser.parse("(5 > 3) implies (2 < 4)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test with expressions: true implies false
    let expr = parser.parse("(5 > 3) implies (4 < 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test with expressions: false implies anything
    let expr = parser.parse("(3 > 5) implies (4 < 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    println!("Expression implies tests passed!");
}

#[test]
fn test_implies_precedence() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test precedence: implies should bind looser than and
    // "true and false implies true" should parse as "(true and false) implies true"
    // which is "false implies true" = true
    let expr = parser.parse("true and false implies true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test another precedence case
    // "false implies true and false" should parse as "false implies (true and false)"
    // which is "false implies false" = true
    let expr = parser.parse("false implies true and false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    println!("Precedence implies tests passed!");
}

#[test]
fn test_implies_with_empty() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // This test requires us to create empty values
    // For now, let's test with expressions that might return empty
    // We can use indexing out of bounds to get empty collections

    // Test: empty implies true -> true  (according to FHIRPath spec)
    // We'll use {}[0] to get an empty value
    let expr = parser.parse("{}[0] implies true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test: empty implies false -> empty (according to FHIRPath spec)
    let expr = parser.parse("{}[0] implies false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    println!("Empty implies tests passed!");
}
