use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn setup_test_environment() -> (FhirPathParser, FhirPathEvaluator, EvaluationContext) {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));
    (parser, evaluator, context)
}

#[test]
fn test_converts_to_boolean_with_empty_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    let empty_context = EvaluationContext::new(json!({
        "emptyArray": []
    }));

    let expr = parser.parse("emptyArray.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &empty_context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_with_boolean_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test true boolean
    let expr = parser.parse("true.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test false boolean
    let expr = parser.parse("false.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_boolean_with_integer_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test integer 1 (convertible)
    let expr = parser.parse("1.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test integer 0 (convertible)
    let expr = parser.parse("0.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test other integers (not convertible)
    let expr = parser.parse("2.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("(-1).convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_with_decimal_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test decimal 1.0 (convertible)
    let expr = parser.parse("1.0.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test decimal 0.0 (convertible)
    let expr = parser.parse("0.0.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test other decimals (not convertible)
    let expr = parser.parse("2.5.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("0.1.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_with_string_values_convertible() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test convertible strings
    let convertible_strings = vec![
        "'true'", "'false'", "'t'", "'f'", "'yes'", "'no'", "'y'", "'n'", "'1'", "'0'", "'1.0'",
        "'0.0'",
    ];

    for string_val in convertible_strings {
        let expr = parser
            .parse(&format!("{string_val}.convertsToBoolean()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Failed for string: {string_val}"
        );
    }
}

#[test]
fn test_converts_to_boolean_with_string_case_insensitive() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test case insensitivity for convertible strings
    let case_variations = vec![
        "'TRUE'", "'True'", "'T'", "'YES'", "'Yes'", "'Y'", "'FALSE'", "'False'", "'F'", "'NO'",
        "'No'", "'N'",
    ];

    for string_val in case_variations {
        let expr = parser
            .parse(&format!("{string_val}.convertsToBoolean()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(
            result,
            FhirPathValue::Boolean(true),
            "Failed for string: {string_val}"
        );
    }
}

#[test]
fn test_converts_to_boolean_with_invalid_string_values() {
    let (parser, evaluator, context) = setup_test_environment();

    // Test strings that are not convertible
    let invalid_strings = vec!["'maybe'", "'2'", "'0.5'", "'hello'", "'world'", "''"];

    for string_val in invalid_strings {
        let expr = parser
            .parse(&format!("{string_val}.convertsToBoolean()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(
            result,
            FhirPathValue::Boolean(false),
            "Failed for string: {string_val}"
        );
    }
}

#[test]
fn test_converts_to_boolean_with_single_item_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Create a context with a single boolean in a collection
    let context = EvaluationContext::new(json!({
        "values": [true]
    }));

    let expr = parser.parse("values.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_converts_to_boolean_with_multiple_item_collection() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Create a context with multiple items in a collection
    let context = EvaluationContext::new(json!({
        "values": [true, false]
    }));

    // Multiple items should return false (not convertible)
    let expr = parser.parse("values.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_with_non_convertible_types() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Create a context with various non-convertible types
    let context = EvaluationContext::new(json!({
        "dateValue": "2023-01-01",
        "objectValue": {"key": "value"}
    }));

    // Date string should return false (not convertible)
    let expr = parser.parse("dateValue.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Object should return false (not convertible)
    let expr = parser.parse("objectValue.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_real_world_scenarios() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Test with FHIR-like boolean fields
    let context = EvaluationContext::new(json!({
        "Patient": {
            "active": "true",
            "deceasedBoolean": false,
            "multipleBirthInteger": 1,
            "gender": "male"
        }
    }));

    // String "true" is convertible
    let expr = parser.parse("Patient.active.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Boolean is convertible
    let expr = parser
        .parse("Patient.deceasedBoolean.convertsToBoolean()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Integer 1 is convertible
    let expr = parser
        .parse("Patient.multipleBirthInteger.convertsToBoolean()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // String "male" is not convertible
    let expr = parser.parse("Patient.gender.convertsToBoolean()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));
}

#[test]
fn test_converts_to_boolean_vs_to_boolean_comparison() {
    let (parser, evaluator, _context) = setup_test_environment();

    // Test cases showing the difference between convertsToBoolean() and toBoolean()
    let test_cases = vec![
        ("'true'", true, true),    // convertible=true, converts to true
        ("'false'", true, false),  // convertible=true, converts to false
        ("'maybe'", false, false), // convertible=false, toBoolean returns empty (false in boolean context)
        ("1", true, true),         // convertible=true, converts to true
        ("0", true, false),        // convertible=true, converts to false
        ("2", false, false),       // convertible=false, toBoolean returns empty
    ];

    for (input, expected_convertible, expected_boolean) in test_cases {
        let context = EvaluationContext::new(json!({}));

        // Test convertsToBoolean()
        let expr = parser
            .parse(&format!("{input}.convertsToBoolean()"))
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(
            result,
            FhirPathValue::Boolean(expected_convertible),
            "convertsToBoolean() failed for input: {input}"
        );

        // Test toBoolean() for convertible values
        if expected_convertible {
            let expr = parser.parse(&format!("{input}.toBoolean()")).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            assert_eq!(
                result,
                FhirPathValue::Boolean(expected_boolean),
                "toBoolean() failed for input: {input}"
            );
        }
    }
}
