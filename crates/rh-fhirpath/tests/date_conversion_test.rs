//! Tests for date conversion functions

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_to_date_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test Date to Date (identity)
    let expr = parser.parse("@2023-01-15.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Date("2023-01-15".to_string()));

    // Test DateTime to Date (extract date part)
    let expr = parser.parse("@2023-01-15T10:30:45.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Date("2023-01-15".to_string()));

    // Test String date to Date
    let expr = parser.parse("'2023-12-25'.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Date("2023-12-25".to_string()));

    // Test String datetime to Date
    let expr = parser.parse("'2023-12-25T23:59:59'.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Date("2023-12-25".to_string()));

    // Test invalid string conversion
    let expr = parser.parse("'not-a-date'.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test integer conversion (should be empty)
    let expr = parser.parse("42.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_converts_to_date_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test Date can convert to Date
    let expr = parser.parse("@2023-01-15.convertsToDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test DateTime can convert to Date
    let expr = parser
        .parse("@2023-01-15T10:30:45.convertsToDate()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test String date can convert to Date
    let expr = parser.parse("'2023-12-25'.convertsToDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test String datetime can convert to Date
    let expr = parser
        .parse("'2023-12-25T23:59:59'.convertsToDate()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid string cannot convert
    let expr = parser.parse("'not-a-date'.convertsToDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test integer cannot convert
    let expr = parser.parse("42.convertsToDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_to_datetime_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test DateTime to DateTime (identity)
    let expr = parser.parse("@2023-01-15T10:30:45.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::DateTime("2023-01-15T10:30:45".to_string())
    );

    // Test Date to DateTime (add midnight time)
    let expr = parser.parse("@2023-01-15.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::DateTime("2023-01-15T00:00:00".to_string())
    );

    // Test String datetime to DateTime
    let expr = parser.parse("'2023-12-25T23:59:59'.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::DateTime("2023-12-25T23:59:59".to_string())
    );

    // Test String date to DateTime
    let expr = parser.parse("'2023-12-25'.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::DateTime("2023-12-25T00:00:00".to_string())
    );

    // Test invalid string conversion
    let expr = parser.parse("'not-a-datetime'.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test integer conversion (should be empty)
    let expr = parser.parse("42.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_converts_to_datetime_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test DateTime can convert to DateTime
    let expr = parser
        .parse("@2023-01-15T10:30:45.convertsToDateTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test Date can convert to DateTime
    let expr = parser.parse("@2023-01-15.convertsToDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test String datetime can convert to DateTime
    let expr = parser
        .parse("'2023-12-25T23:59:59'.convertsToDateTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test String date can convert to DateTime
    let expr = parser.parse("'2023-12-25'.convertsToDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid string cannot convert
    let expr = parser
        .parse("'not-a-datetime'.convertsToDateTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test integer cannot convert
    let expr = parser.parse("42.convertsToDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_date_conversion_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test empty collection
    let expr = parser.parse("{}.toDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("{}.convertsToDate()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test multiple items (should return empty)
    let expr = parser
        .parse("('2023-01-01' | '2023-01-02').toDate()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser
        .parse("('2023-01-01' | '2023-01-02').convertsToDate()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test DateTime with timezone
    let expr = parser.parse("'2023-12-25T10:30:45Z'.toDateTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::DateTime("2023-12-25T10:30:45Z".to_string())
    );

    let expr = parser
        .parse("'2023-12-25T10:30:45+05:30'.convertsToDateTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}
