use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_matches_basic_regex_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test digit patterns
    let expr = parser.parse("'N8000123123'.matches('N[0-9]{8}')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test exact matching with anchors
    let expr = parser.parse("'N80001231'.matches('^N[0-9]{8}$')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test partial match should fail with full anchors
    let expr = parser
        .parse("'N8000123123'.matches('^N[0-9]{8}$')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test case sensitivity
    let expr = parser.parse("'Hello World'.matches('[Hh]ello')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_date_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test valid date pattern
    let expr = parser
        .parse("'2023-12-25'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid date format
    let expr = parser
        .parse("'23-12-25'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test date with full anchor
    let expr = parser
        .parse("'2023-12-25'.matches('^[0-9]{4}-[0-9]{2}-[0-9]{2}$')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_literal_strings() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test simple literal match
    let expr = parser.parse("'N8000123123'.matches('N8000')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test literal no match
    let expr = parser.parse("'N8000123123'.matches('X8000')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test word match without boundaries
    let expr = parser.parse("'hello world'.matches('world')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_invalid_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test invalid regex pattern - should return false
    let expr = parser.parse("'test'.matches('[invalid')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test another invalid pattern
    let expr = parser.parse("'test'.matches('(unclosed group')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_matches_complex_patterns() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test basic email pattern (simplified)
    let expr = parser
        .parse("'user@example.com'.matches('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test phone number pattern
    let expr = parser
        .parse("'555-123-4567'.matches('[0-9]{3}-[0-9]{3}-[0-9]{4}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test with dashes (simplified)
    let expr = parser
        .parse("'123-45-6789'.matches('[0-9]{3}-[0-9]{2}-[0-9]{4}')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_matches_type_errors() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test matches on non-string should error
    let expr = parser.parse("42.matches('test')").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());

    // Test matches with non-string pattern should error
    let expr = parser.parse("'test'.matches(42)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
}
