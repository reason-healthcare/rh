//! Comprehensive tests for toQuantity() and convertsToQuantity() functions

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::Decimal;
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
            assert_eq!(value, Decimal::from_str_exact("42.0").unwrap());
            // FHIRPath spec: toQuantity defaults to UCUM unit "1".
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test decimal to quantity without unit
    let expr = parser.parse("2.5.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("2.5").unwrap());
            // FHIRPath spec: toQuantity defaults to UCUM unit "1".
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test long to quantity without unit
    let expr = parser.parse("42L.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("42.0").unwrap());
            // FHIRPath spec: toQuantity defaults to UCUM unit "1".
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}

#[test]
fn test_to_quantity_with_unit_parameter() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Numeric input converts to the UCUM default unit '1'. Physical-unit
    // targets are not compatible with '1' and therefore return empty.
    let expr = parser.parse("42.toQuantity('mg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test decimal with compatible default unit parameter
    let expr = parser.parse("37.2.toQuantity('1')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("37.2").unwrap());
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test long with incompatible physical unit parameter
    let expr = parser.parse("100L.toQuantity('mm[Hg]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);
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
            assert_eq!(value, Decimal::from_str_exact("42.7").unwrap());
            // FHIRPath spec: toQuantity defaults to UCUM unit "1".
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test plain numeric string with incompatible physical unit parameter
    let expr = parser.parse("'42.7'.toQuantity('L')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test plain numeric string with compatible default unit parameter
    let expr = parser.parse("'5.0'.toQuantity('g')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test string quantity with unit conversion
    let expr = parser.parse("'5.0 \\'mg\\''.toQuantity('g')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("0.005").unwrap());
            assert_eq!(unit, Some("g".to_string()));
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
            assert_eq!(value, Decimal::from_str_exact("5.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test existing quantity with compatible unit parameter conversion
    let expr = parser.parse("5'mg'.toQuantity('g')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("0.005").unwrap());
            assert_eq!(unit, Some("g".to_string()));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test quantity without unit cannot convert to a physical unit
    let expr = parser.parse("42''.toQuantity('kg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test quantity without unit can convert to the UCUM default unit
    let expr = parser.parse("42''.toQuantity('1')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("42.0").unwrap());
            assert_eq!(unit.as_deref(), Some("1"));
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

    // Boolean toQuantity per spec: true → 1 '1', false → 0 '1'.
    let expr = parser.parse("true.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::Quantity {
            value: Decimal::from_str_exact("1.0").unwrap(),
            unit: Some("1".to_string()),
        }
    );

    // Test empty collection
    let expr = parser.parse("{}.toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Test multiple items collection
    let expr = parser.parse("(1 | 2).toQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err(), "Expected error for multi-item input");

    // Test single item collection
    let expr = parser.parse("(42).toQuantity('1')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("42.0").unwrap());
            assert_eq!(unit.as_deref(), Some("1"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }

    // Test invalid unit parameter (non-string)
    let expr = parser.parse("42.toQuantity(123)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(value, Decimal::from_str_exact("42.0").unwrap());
            // FHIRPath spec: toQuantity defaults to UCUM unit "1".
            assert_eq!(unit.as_deref(), Some("1")); // Invalid unit parameter ignored
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

    // Booleans convert to Quantity per spec (true → 1 '1', false → 0 '1');
    // temporals do not.
    let test_cases = [
        ("true.convertsToQuantity()", true),
        ("false.convertsToQuantity()", true),
        ("@2023-01-15.convertsToQuantity()", false),
        ("@2023-01-15T10:30:45.convertsToQuantity()", false),
        ("@T10:30:45.convertsToQuantity()", false),
    ];

    for (input, expected) in test_cases {
        let expr = parser.parse(input).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Boolean(b) => assert_eq!(b, expected, "Failed for: {input}"),
            _ => panic!("Expected Boolean for {input}, got: {result:?}"),
        }
    }

    let expr = parser.parse("{}.convertsToQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("(1 | 2).convertsToQuantity()").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err(), "Expected error for multi-item input");

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

    // Test convertsToQuantity with unit parameter follows actual conversion
    let expr = parser.parse("42.convertsToQuantity('mg')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    let expr = parser.parse("42.convertsToQuantity('1')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(true));

    let expr = parser
        .parse("'not-a-number'.convertsToQuantity('kg')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Boolean(false));

    // Test toQuantity with incompatible unit formats on dimensionless input
    let expr = parser.parse("100.toQuantity('mm[Hg]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    let expr = parser.parse("98.6.toQuantity('[degF]')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::Empty);

    // Existing quantities can convert among compatible units.
    let expr = parser.parse("100'mm[Hg]'.toQuantity('Pa')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Quantity { value, unit } => {
            assert_eq!(
                value.round_dp(1),
                Decimal::from_str_exact("13332.2").unwrap()
            );
            assert_eq!(unit.as_deref(), Some("Pa"));
        }
        _ => panic!("Expected Quantity, got: {result:?}"),
    }
}
