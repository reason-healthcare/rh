use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn setup_test_environment() -> (FhirPathParser, FhirPathEvaluator, EvaluationContext) {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));
    (parser, evaluator, context)
}

#[test]
fn test_converts_to_integer_with_empty_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    let empty_context = EvaluationContext::new(json!({
        "emptyArray": []
    }));

    let expr = parser.parse("emptyArray.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &empty_context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_integer_with_integer_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // All integers should convert to integer
    let expr = parser.parse("42.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("0.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("(-123).convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_integer_with_boolean_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Both boolean values should convert to integer
    let expr = parser.parse("true.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("false.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_integer_with_decimal_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test whole number decimals (should be convertible)
    let expr = parser.parse("42.0.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("0.0.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("(-123.0).convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test non-whole decimals (should not be convertible)
    let expr = parser.parse("2.5.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("0.1.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("(-3.14).convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_integer_with_string_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test valid integer strings (should be convertible)
    let expr = parser.parse("'42'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'0'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("'-123'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test strings with whitespace (should be convertible)
    let expr = parser.parse("'  42  '.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test invalid strings (should not be convertible)
    let expr = parser.parse("'abc'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'42.5'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("''.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("'true'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_integer_with_other_types() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Test with DateTime (should not be convertible)
    let datetime_context = EvaluationContext::new(json!({
        "date": "2023-01-01"
    }));

    let expr = parser.parse("date.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &datetime_context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_integer_with_multiple_items() {
    let (parser, evaluator, _context) = setup_test_environment();

    let multi_context = EvaluationContext::new(json!({
        "numbers": [1, 2, 3]
    }));

    // Multiple items should not be convertible
    let expr = parser.parse("numbers.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &multi_context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_integer_with_single_item_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    let single_context = EvaluationContext::new(json!({
        "singleNumber": [42]
    }));

    // Single item collection should be convertible if the item is convertible
    let expr = parser.parse("singleNumber.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &single_context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_integer_edge_cases() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test large numbers (but within reasonable range)
    let expr = parser.parse("1000000.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("(-1000000).convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test string representation of large numbers
    let expr = parser.parse("'1000000'.convertsToInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_integer_with_infinity_and_nan() {
    let (_parser, _evaluator, _context) = setup_test_environment();

    // These should not be convertible since they're not finite
    // Note: The actual handling depends on how these values are represented in the test
    // This test structure is ready if we need to handle these cases explicitly
}

#[test]
fn test_converts_to_integer_comprehensive() {
    let (parser, evaluator, _context) = setup_test_environment();

    let comprehensive_context = EvaluationContext::new(json!({
        "testCases": {
            "validInteger": 42,
            "zero": 0,
            "negative": -123,
            "trueBoolean": true,
            "falseBoolean": false,
            "wholeDecimal": 42.0,
            "fractionalDecimal": 42.5,
            "validStringInteger": "123",
            "invalidString": "abc",
            "emptyString": "",
            "whitespaceString": "  456  ",
            "decimalString": "42.5"
        }
    }));

    let test_cases = vec![
        (
            "testCases.validInteger.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.zero.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.negative.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.trueBoolean.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.falseBoolean.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.wholeDecimal.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.fractionalDecimal.convertsToInteger()",
            FhirPathValue::Boolean(false),
        ),
        (
            "testCases.validStringInteger.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.invalidString.convertsToInteger()",
            FhirPathValue::Boolean(false),
        ),
        (
            "testCases.emptyString.convertsToInteger()",
            FhirPathValue::Boolean(false),
        ),
        (
            "testCases.whitespaceString.convertsToInteger()",
            FhirPathValue::Boolean(true),
        ),
        (
            "testCases.decimalString.convertsToInteger()",
            FhirPathValue::Boolean(false),
        ),
    ];

    for (expression, expected) in test_cases {
        let expr = parser.parse(expression).unwrap();
        let result = evaluator.evaluate(&expr, &comprehensive_context).unwrap();
        assert_eq!(result, expected, "Failed for expression: {expression}");
    }
}

#[test]
fn test_converts_to_integer_string_edge_cases() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test various string edge cases
    let string_test_cases = vec![
        ("'  123  '", true), // Whitespace should be trimmed
        ("'+123'", true),    // Leading plus sign
        ("'123abc'", false), // Mixed content
        ("'abc123'", false), // Mixed content
        ("'12 34'", false),  // Internal whitespace
        ("'1.0'", false),    // Decimal in string (not integer)
        ("'1e10'", false),   // Scientific notation
    ];

    for (string_val, expected) in string_test_cases {
        let expr = parser
            .parse(&format!("{string_val}.convertsToInteger()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(
            result,
            FhirPathValue::Boolean(expected),
            "Failed for string: {string_val}"
        );
    }
}
