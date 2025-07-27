use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn setup_test_environment() -> (FhirPathParser, FhirPathEvaluator, EvaluationContext) {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));
    (parser, evaluator, context)
}

#[test]
fn test_to_integer_with_empty_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    let empty_context = EvaluationContext::new(json!({
        "emptyArray": []
    }));

    let expr = parser.parse("emptyArray.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &empty_context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_to_integer_with_integer_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test positive integer
    let expr = parser.parse("42.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(42));

    // Test zero
    let expr = parser.parse("0.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));

    // Test negative integer
    let expr = parser.parse("(-123).toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(-123));
}

#[test]
fn test_to_integer_with_boolean_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test true boolean (should be 1)
    let expr = parser.parse("true.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(1));

    // Test false boolean (should be 0)
    let expr = parser.parse("false.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));
}

#[test]
fn test_to_integer_with_decimal_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test whole number decimals
    let expr = parser.parse("42.0.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(42));

    let expr = parser.parse("0.0.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));

    let expr = parser.parse("(-123.0).toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(-123));

    // Test non-whole decimals (should be empty)
    let expr = parser.parse("2.5.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("0.1.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("(-3.14).toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_to_integer_with_string_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test valid integer strings
    let expr = parser.parse("'42'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(42));

    let expr = parser.parse("'0'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(0));

    let expr = parser.parse("'-123'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(-123));

    // Test strings with whitespace
    let expr = parser.parse("'  42  '.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(42));

    // Test invalid strings (should be empty)
    let expr = parser.parse("'abc'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("'42.5'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("''.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("'true'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_to_integer_with_other_types() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Test with DateTime (should be empty)
    let datetime_context = EvaluationContext::new(json!({
        "date": "2023-01-01"
    }));

    let expr = parser.parse("date.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &datetime_context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_to_integer_with_multiple_items() {
    let (parser, evaluator, _context) = setup_test_environment();

    let multi_context = EvaluationContext::new(json!({
        "numbers": [1, 2, 3]
    }));

    // Multiple items should return empty
    let expr = parser.parse("numbers.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &multi_context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_to_integer_with_single_item_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    let single_context = EvaluationContext::new(json!({
        "singleNumber": [42]
    }));

    // Single item collection should work
    let expr = parser.parse("singleNumber.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &single_context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(42));
}

#[test]
fn test_to_integer_edge_cases() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test large numbers (but within reasonable range)
    let expr = parser.parse("1000000.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(1000000));

    let expr = parser.parse("(-1000000).toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(-1000000));

    // Test string representation of large numbers
    let expr = parser.parse("'1000000'.toInteger()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Integer(1000000));
}

#[test]
fn test_to_integer_with_infinity_and_nan() {
    let (_parser, _evaluator, _context) = setup_test_environment();

    // These should be handled as empty since they're not finite
    // Note: The actual handling depends on how these values are represented in the test
    // This test structure is ready if we need to handle these cases explicitly
}

#[test]
fn test_to_integer_comprehensive() {
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
            "testCases.validInteger.toInteger()",
            FhirPathValue::Integer(42),
        ),
        ("testCases.zero.toInteger()", FhirPathValue::Integer(0)),
        (
            "testCases.negative.toInteger()",
            FhirPathValue::Integer(-123),
        ),
        (
            "testCases.trueBoolean.toInteger()",
            FhirPathValue::Integer(1),
        ),
        (
            "testCases.falseBoolean.toInteger()",
            FhirPathValue::Integer(0),
        ),
        (
            "testCases.wholeDecimal.toInteger()",
            FhirPathValue::Integer(42),
        ),
        (
            "testCases.fractionalDecimal.toInteger()",
            FhirPathValue::Empty,
        ),
        (
            "testCases.validStringInteger.toInteger()",
            FhirPathValue::Integer(123),
        ),
        ("testCases.invalidString.toInteger()", FhirPathValue::Empty),
        ("testCases.emptyString.toInteger()", FhirPathValue::Empty),
        (
            "testCases.whitespaceString.toInteger()",
            FhirPathValue::Integer(456),
        ),
        ("testCases.decimalString.toInteger()", FhirPathValue::Empty),
    ];

    for (expression, expected) in test_cases {
        let expr = parser.parse(expression).unwrap();
        let result = evaluator.evaluate(&expr, &comprehensive_context).unwrap();
        assert_eq!(result, expected, "Failed for expression: {expression}");
    }
}
