//! Tests for time conversion functions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_to_time_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test Time to Time (identity)
    let expr = parser.parse("@T10:30:45.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T10:30:45".to_string()));

    let expr = parser.parse("@T23:59:59.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T23:59:59".to_string()));

    // Test String to Time
    let expr = parser.parse("'10:30:45'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T10:30:45".to_string()));

    let expr = parser.parse("'23:59:59.123'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T23:59:59.123".to_string()));

    let expr = parser.parse("'00:00:00'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T00:00:00".to_string()));

    // Test String with T prefix to Time
    let expr = parser.parse("'T14:25:36'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T14:25:36".to_string()));

    // Test DateTime to Time (extract time part)
    let expr = parser.parse("@2023-01-15T10:30:45.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T10:30:45".to_string()));

    let expr = parser.parse("@2023-01-15T23:59:59Z.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T23:59:59".to_string()));

    let expr = parser.parse("@2023-01-15T14:25:36+05:30.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T14:25:36".to_string()));

    // Test invalid strings (should return empty)
    let expr = parser.parse("'not-a-time'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("'25:00:00'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("'12:60:00'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("'12:30:60'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test non-convertible types (should return empty)
    let expr = parser.parse("42.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("true.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("@2023-01-15.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_converts_to_time_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test Time can convert to Time
    let expr = parser.parse("@T10:30:45.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("@T23:59:59.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test valid time strings can convert to Time
    let expr = parser.parse("'10:30:45'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'23:59:59.123'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'00:00:00'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'T14:25:36'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test DateTime can convert to Time
    let expr = parser
        .parse("@2023-01-15T10:30:45.convertsToTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser
        .parse("@2023-01-15T23:59:59Z.convertsToTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser
        .parse("@2023-01-15T14:25:36+05:30.convertsToTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid strings cannot convert to Time
    let expr = parser.parse("'not-a-time'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'25:00:00'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'12:60:00'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'12:30:60'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test non-convertible types
    let expr = parser.parse("42.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("true.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("@2023-01-15.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'hello'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_time_conversion_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test empty collection
    let expr = parser.parse("{}.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("{}.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test multiple items (should return empty/false)
    let expr = parser.parse("('10:30:45' | '14:25:36').toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser
        .parse("('10:30:45' | '14:25:36').convertsToTime()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test single item collection
    let expr = parser.parse("('10:30:45').toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T10:30:45".to_string()));

    let expr = parser.parse("('10:30:45').convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test time format edge cases
    let expr = parser.parse("'0:0:0'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'0:0:0'.toTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Time("T0:0:0".to_string()));

    // Test fractional seconds variations
    let expr = parser.parse("'12:30:45.1'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'12:30:45.12'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'12:30:45.123'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid fractional seconds (too many digits)
    let expr = parser.parse("'12:30:45.1234'.convertsToTime()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_time_conversion_comprehensive() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test all valid time conversion patterns
    let test_cases = vec![
        // Time literals (identity)
        ("@T10:30:45", "T10:30:45", true),
        ("@T00:00:00", "T00:00:00", true),
        ("@T23:59:59", "T23:59:59", true),
        // String time formats
        ("'10:30:45'", "T10:30:45", true),
        ("'00:00:00'", "T00:00:00", true),
        ("'23:59:59'", "T23:59:59", true),
        ("'12:30:45.123'", "T12:30:45.123", true),
        ("'T14:25:36'", "T14:25:36", true),
        // DateTime extraction
        ("@2023-01-15T10:30:45", "T10:30:45", true),
        ("@2023-01-15T00:00:00Z", "T00:00:00", true),
        ("@2023-01-15T23:59:59+05:30", "T23:59:59", true),
    ];

    for (input, expected_time, should_convert) in test_cases {
        // Test toTime conversion
        let expr = parser.parse(&format!("{input}.toTime()")).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if should_convert {
            assert_eq!(result, FhirPathValue::Time(expected_time.to_string()));
        } else {
            assert_eq!(result, FhirPathValue::Empty);
        }

        // Test convertsToTime check
        let expr = parser.parse(&format!("{input}.convertsToTime()")).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(should_convert));
    }

    // Test invalid conversions
    let invalid_cases = vec![
        "42",
        "true",
        "3.14",
        "'not-a-time'",
        "'25:00:00'",
        "'12:60:00'",
        "'12:30:60'",
        "@2023-01-15",
        "5'mg'",
    ];

    for input in invalid_cases {
        let expr = parser.parse(&format!("{input}.toTime()")).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Empty);

        let expr = parser.parse(&format!("{input}.convertsToTime()")).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }
}
