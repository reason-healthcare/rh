//! Comparison operation tests
//! 
//! Tests for equality and ordering operations in FHIRPath expressions

use super::*;

#[test]
fn test_equality() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer equality
    let expr = parser.parse("5 = 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("5 = 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test string equality
    let expr = parser.parse("'hello' = 'hello'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test boolean equality
    let expr = parser.parse("true = true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_inequality() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer inequality
    let expr = parser.parse("5 != 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("5 != 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_equivalent() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer equivalence
    let expr = parser.parse("5 ~ 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("5 ~ 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test string equivalence
    let expr = parser.parse("'hello' ~ 'hello'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("'hello' ~ 'world'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test boolean equivalence
    let expr = parser.parse("true ~ true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("true ~ false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test empty collection equivalence (both empty should be equivalent)
    let expr = parser.parse("{} ~ {}").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_not_equivalent() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer not equivalence
    let expr = parser.parse("5 !~ 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("5 !~ 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test string not equivalence
    let expr = parser.parse("'hello' !~ 'world'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("'hello' !~ 'hello'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test boolean not equivalence
    let expr = parser.parse("true !~ false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("true !~ true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test empty collection not equivalence
    let expr = parser.parse("{} !~ {}").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_less_than() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer less than
    let expr = parser.parse("3 < 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("5 < 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_greater_than() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer greater than
    let expr = parser.parse("5 > 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("3 > 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_string_comparison() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test string comparison (lexicographical)
    let expr = parser.parse("'apple' < 'banana'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("'zebra' > 'apple'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}
