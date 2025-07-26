//! Tests for FHIRPath quantity unit conversion functionality

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_mass_unit_conversion() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test adding 1 kg + 500 g should equal 1.5 kg (in kg units)
    let expr_str = "1.0'kg' + 500.0'g'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 1.5).abs() < 0.001);
        assert_eq!(unit, &Some("kg".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_length_unit_conversion() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test adding 1 m + 50 cm should equal 1.5 m
    let expr_str = "1.0'm' + 50.0'cm'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 1.5).abs() < 0.001);
        assert_eq!(unit, &Some("m".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_volume_unit_conversion() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test adding 1 L + 500 mL should equal 1.5 L
    let expr_str = "1.0'L' + 500.0'mL'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 1.5).abs() < 0.001);
        assert_eq!(unit, &Some("L".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_subtraction_with_conversion() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test subtracting 1 kg - 200 g should equal 0.8 kg
    let expr_str = "1.0'kg' - 200.0'g'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 0.8).abs() < 0.001);
        assert_eq!(unit, &Some("kg".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_division_same_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test dividing quantities with same units should give dimensionless result
    let expr_str = "10.0'kg' / 2.0'kg'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Number(ratio) = &result {
        assert!((ratio - 5.0).abs() < 0.001);
    } else {
        panic!("Expected number result, got {result:?}");
    }
}

#[test]
fn test_division_compatible_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test dividing quantities with compatible units (kg/g) should give dimensionless result
    let expr_str = "1.0'kg' / 500.0'g'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Number(ratio) = &result {
        assert!((ratio - 2.0).abs() < 0.001);
    } else {
        panic!("Expected number result, got {result:?}");
    }
}

#[test]
fn test_scalar_multiplication() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test multiplying quantity by scalar
    let expr_str = "5.0'g' * 3.0";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 15.0).abs() < 0.001);
        assert_eq!(unit, &Some("g".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_scalar_division() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test dividing quantity by scalar
    let expr_str = "15.0'g' / 3.0";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 5.0).abs() < 0.001);
        assert_eq!(unit, &Some("g".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}

#[test]
fn test_incompatible_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test adding incompatible units should fail
    let expr_str = "1.0'kg' + 1.0'm'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context);

    assert!(result.is_err());
}

#[test]
fn test_time_unit_conversion() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test adding time units: 1 h + 30 min should equal 1.5 h
    let expr_str = "1.0'h' + 30.0'min'";
    let expr = parser.parse(expr_str).unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Quantity { value, unit } = &result {
        assert!((value - 1.5).abs() < 0.001);
        assert_eq!(unit, &Some("h".to_string()));
    } else {
        panic!("Expected quantity result, got {result:?}");
    }
}
