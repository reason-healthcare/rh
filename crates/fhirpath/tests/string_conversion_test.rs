//! Tests for string conversion functions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_to_string_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test String to String (identity)
    let expr = parser.parse("'hello world'.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("hello world".to_string()));

    // Test Boolean to String
    let expr = parser.parse("true.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("true".to_string()));

    let expr = parser.parse("false.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("false".to_string()));

    // Test Integer to String
    let expr = parser.parse("42.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("42".to_string()));

    let expr = parser.parse("(-123).toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("-123".to_string()));

    // Test Long to String
    let expr = parser.parse("42L.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("42".to_string()));

    // Test Number to String
    let expr = parser.parse("3.14.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("3.14".to_string()));

    let expr = parser.parse("42.0.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("42".to_string()));

    // Test Date to String
    let expr = parser.parse("@2023-01-15.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("2023-01-15".to_string()));

    // Test DateTime to String
    let expr = parser.parse("@2023-01-15T10:30:45.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::String("2023-01-15T10:30:45".to_string())
    );

    // Test Time to String
    let expr = parser.parse("@T10:30:45.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("10:30:45".to_string()));

    // Test Quantity to String
    let expr = parser.parse("5'mg'.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("5 'mg'".to_string()));

    // Test Quantity to String
    let expr = parser.parse("5 'mg'.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("5 'mg'".to_string()));
}

#[test]
fn test_converts_to_string_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test String can convert to String
    let expr = parser.parse("'hello'.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Boolean can convert to String
    let expr = parser.parse("true.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("false.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Integer can convert to String
    let expr = parser.parse("42.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Long can convert to String
    let expr = parser.parse("42L.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Number can convert to String
    let expr = parser.parse("3.14.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Date can convert to String
    let expr = parser.parse("@2023-01-15.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test DateTime can convert to String
    let expr = parser
        .parse("@2023-01-15T10:30:45.convertsToString()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Time can convert to String
    let expr = parser.parse("@T10:30:45.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Quantity can convert to String
    let expr = parser.parse("5'mg'.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_string_conversion_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test empty collection
    let expr = parser.parse("{}.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("{}.convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test multiple items (should return empty/false)
    let expr = parser.parse("(1 | 2).toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("(1 | 2).convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test single item collection
    let expr = parser.parse("(42).toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("42".to_string()));

    let expr = parser.parse("(42).convertsToString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test number formatting edge cases
    let expr = parser.parse("0.0.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("0".to_string()));

    let expr = parser.parse("123.450.toString()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("123.45".to_string()));
}

#[test]
fn test_string_conversion_comprehensive() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test all basic types can convert to string
    let test_cases = vec![
        ("'text'", "text"),
        ("true", "true"),
        ("false", "false"),
        ("42", "42"),
        ("42L", "42"),
        ("3.14", "3.14"),
        ("0", "0"),
        ("0.0", "0"),
        ("@2023-01-15", "2023-01-15"),
        ("@2023-01-15T10:30:45", "2023-01-15T10:30:45"),
        ("@T10:30:45", "10:30:45"),
    ];

    for (input, expected) in test_cases {
        let expr = parser.parse(&format!("{input}.toString()")).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String(expected.to_string()));

        let expr = parser
            .parse(&format!("{input}.convertsToString()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }
}
