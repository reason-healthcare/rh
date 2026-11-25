//! Boolean collection function tests

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_all_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test all() on collection of all truthy values
    let expr = parser.parse("(true | 1 | 'test').all()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test all() on collection with some falsy values
    let expr = parser.parse("(true | false | 1).all()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test all() on empty collection
    let expr = parser.parse("{}.all()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_all_true_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test allTrue() on collection of all true values
    let expr = parser.parse("(true | true | true).allTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test allTrue() on collection with some false values
    let expr = parser.parse("(true | false | true).allTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test allTrue() rejects a collection with non-Boolean values
    let expr = parser.parse("(true | 1 | true).allTrue()").unwrap();
    assert!(evaluator.evaluate(&expr, &context).is_err());

    // Test allTrue() on empty collection
    let expr = parser.parse("{}.allTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_any_true_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test anyTrue() on collection with some true values
    let expr = parser.parse("(false | true | false).anyTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test anyTrue() on collection with no true values
    let expr = parser.parse("(false | false | false).anyTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test anyTrue() rejects a collection with non-Boolean values
    let expr = parser.parse("(1 | 'test' | false).anyTrue()").unwrap();
    assert!(evaluator.evaluate(&expr, &context).is_err());

    // Test anyTrue() on empty collection
    let expr = parser.parse("{}.anyTrue()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_all_false_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test allFalse() on collection of all false values
    let expr = parser.parse("(false | false | false).allFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test allFalse() on collection with some true values
    let expr = parser.parse("(false | true | false).allFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test allFalse() rejects a collection with non-Boolean values
    let expr = parser.parse("(false | 0 | false).allFalse()").unwrap();
    assert!(evaluator.evaluate(&expr, &context).is_err());

    // Test allFalse() on empty collection
    let expr = parser.parse("{}.allFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_any_false_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test anyFalse() on collection with some false values
    let expr = parser.parse("(true | false | true).anyFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test anyFalse() on collection with no false values
    let expr = parser.parse("(true | true | true).anyFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test anyFalse() rejects a collection with non-Boolean values
    let expr = parser.parse("(1 | 'test' | true).anyFalse()").unwrap();
    assert!(evaluator.evaluate(&expr, &context).is_err());

    // Test anyFalse() on empty collection
    let expr = parser.parse("{}.anyFalse()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}
