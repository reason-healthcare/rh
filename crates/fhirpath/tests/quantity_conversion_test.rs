//! Comprehensive tests for toQuantity() and convertsToQuantity() functions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_to_quantity_basic_conversions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test numeric types to quantity without unit
    let expr = parser.parse("42.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, None);
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test decimal to quantity without unit
    let expr = parser.parse("2.5.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 2.5);
            assert_eq!(unit, None);
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test long to quantity without unit
    let expr = parser.parse("42L.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, None);
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_to_quantity_with_unit_parameter() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test numeric to quantity with unit parameter
    let expr = parser.parse("42.toQuantity('mg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, Some("mg".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test decimal with unit parameter
    let expr = parser.parse("37.2.toQuantity('Cel')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 37.2);
            assert_eq!(unit, Some("Cel".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test long with unit parameter
    let expr = parser.parse("100L.toQuantity('mm[Hg]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 100.0);
            assert_eq!(unit, Some("mm[Hg]".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_to_quantity_string_conversions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test string with numeric format (since quantity string parsing is complex)
    // Most strings won't contain embedded quotes in real usage
    let expr = parser.parse("'42.7'.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.7);
            assert_eq!(unit, None);
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test plain numeric string with unit parameter
    let expr = parser.parse("'42.7'.toQuantity('L')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.7);
            assert_eq!(unit, Some("L".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test string quantity with unit parameter override
    let expr = parser.parse("'5.0'.toQuantity('g')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 5.0);
            assert_eq!(unit, Some("g".to_string())); // Should use parameter unit
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_to_quantity_existing_quantity() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test existing quantity (identity conversion)
    let expr = parser.parse("5'mg'.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 5.0);
            assert_eq!(unit, Some("mg".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test existing quantity with unit parameter (unit override)
    let expr = parser.parse("5'mg'.toQuantity('g')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 5.0);
            assert_eq!(unit, Some("g".to_string())); // Should use parameter unit
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test quantity without unit getting unit parameter
    let expr = parser.parse("42''.toQuantity('kg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, Some("kg".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_to_quantity_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test invalid string
    let expr = parser.parse("'not-a-number'.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test boolean (should be empty)
    let expr = parser.parse("true.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test empty collection
    let expr = parser.parse("{}.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test multiple items collection (should return empty)
    let expr = parser.parse("(1 | 2).toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test single item collection
    let expr = parser.parse("(42).toQuantity('mg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, Some("mg".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test invalid unit parameter (non-string)
    let expr = parser.parse("42.toQuantity(123)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 42.0);
            assert_eq!(unit, None); // Invalid unit parameter ignored
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_converts_to_quantity_basic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test numeric types
    let test_cases = [
        ("42.convertsToQuantity()", true),
        ("3.14.convertsToQuantity()", true),
        ("42L.convertsToQuantity()", true),
        ("(-5).convertsToQuantity()", true),
        ("0.convertsToQuantity()", true),
        ("0.0.convertsToQuantity()", true),
    ];

    for (input, expected) in test_cases {
        let expr = parser.parse(input).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Boolean(b) => assert_eq!(b, expected, "Failed for: {input}"),
            _ => panic!("Expected Boolean for {input}, got: {result:?}"),
        }
    }
}

#[test]
fn test_converts_to_quantity_strings() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test string conversions
    let test_cases = [
        ("'42'.convertsToQuantity()", true),
        ("'3.14'.convertsToQuantity()", true),
        ("'-5.7'.convertsToQuantity()", true),
        ("'0'.convertsToQuantity()", true),
        ("'not-a-number'.convertsToQuantity()", false),
        ("''.convertsToQuantity()", false),
        ("'abc123'.convertsToQuantity()", false),
        ("'12.34.56'.convertsToQuantity()", false),
    ];

    for (input, expected) in test_cases {
        let expr = parser.parse(input).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Boolean(b) => assert_eq!(b, expected, "Failed for: {input}"),
            _ => panic!("Expected Boolean for {input}, got: {result:?}"),
        }
    }
}

#[test]
fn test_converts_to_quantity_other_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test existing quantities
    let expr = parser.parse("5'mg'.convertsToQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser.parse("37.2'Cel'.convertsToQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    // Test non-convertible types
    let test_cases = [
        ("true.convertsToQuantity()", false),
        ("false.convertsToQuantity()", false),
        ("@2023-01-15.convertsToQuantity()", false),
        ("@2023-01-15T10:30:45.convertsToQuantity()", false),
        ("@T10:30:45.convertsToQuantity()", false),
        ("{}.convertsToQuantity()", false),
        ("(1 | 2).convertsToQuantity()", false), // Multiple items
    ];

    for (input, expected) in test_cases {
        let expr = parser.parse(input).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Boolean(b) => assert_eq!(b, expected, "Failed for: {input}"),
            _ => panic!("Expected Boolean for {input}, got: {result:?}"),
        }
    }

    // Test single item collection
    let expr = parser.parse("(42).convertsToQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));
}

#[test]
fn test_quantity_conversion_with_unit_parameters() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test convertsToQuantity with unit parameter (should still work)
    let expr = parser.parse("42.convertsToQuantity('mg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser
        .parse("'not-a-number'.convertsToQuantity('kg')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test toQuantity with various unit formats
    let expr = parser.parse("100.toQuantity('mm[Hg]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 100.0);
            assert_eq!(unit, Some("mm[Hg]".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    let expr = parser.parse("98.6.toQuantity('[degF]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, 98.6);
            assert_eq!(unit, Some("[degF]".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}
