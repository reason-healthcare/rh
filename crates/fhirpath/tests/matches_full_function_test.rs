use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_matches_full_exact_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test exact match - should pass
    let expr = parser
        .parse("'N80001231'.matchesFull('N[0-9]{8}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test partial match - should fail (unlike matches())
    let expr = parser
        .parse("'N8000123123'.matchesFull('N[0-9]{8}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test with manually anchored pattern - should work the same
    let expr = parser
        .parse("'N80001231'.matchesFull('^N[0-9]{8}$')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_full_vs_matches() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test case where matches() would pass but matchesFull() should fail
    let expr = parser.parse("'hello world'.matches('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'hello world'.matchesFull('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test case where both should pass (exact match)
    let expr = parser.parse("'world'.matches('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'world'.matchesFull('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_full_date_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Valid full date
    let expr = parser
        .parse("'2023-12-25'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Invalid - too short
    let expr = parser
        .parse("'23-12-25'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Invalid - extra text
    let expr = parser
        .parse("'2023-12-25T00:00:00'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_matches_full_healthcare_ids() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Exact patient ID
    let expr = parser
        .parse("'N80001231'.matchesFull('N[0-9]{8}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Too long - should fail
    let expr = parser
        .parse("'N800012312'.matchesFull('N[0-9]{8}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Too short - should fail
    let expr = parser.parse("'N8000123'.matchesFull('N[0-9]{8}')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_matches_full_error_handling() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Invalid regex pattern - should return false
    let expr = parser.parse("'test'.matchesFull('[invalid')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Type error - non-string target
    let expr = parser.parse("42.matchesFull('test')").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());

    // Type error - non-string pattern
    let expr = parser.parse("'test'.matchesFull(42)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
}

#[test]
fn test_matches_full_complex_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Email pattern - exact match
    let expr = parser
        .parse("'user@example.com'.matchesFull('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Phone number - exact match
    let expr = parser
        .parse("'555-123-4567'.matchesFull('[0-9]{3}-[0-9]{3}-[0-9]{4}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Phone number with extra text - should fail
    let expr = parser
        .parse("'Call 555-123-4567 now'.matchesFull('[0-9]{3}-[0-9]{3}-[0-9]{4}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}
