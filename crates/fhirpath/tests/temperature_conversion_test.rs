//! Temperature unit conversion integration tests

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_temperature_unit_conversions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test basic temperature literals
    let expr = parser.parse("37.0'Cel'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            assert_eq!(*value, 37.0);
            assert_eq!(*unit, Some("Cel".to_string()));
        }
    }

    let expr = parser.parse("98.6'[degF]'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            assert_eq!(*value, 98.6);
            assert_eq!(*unit, Some("[degF]".to_string()));
        }
    }

    let expr = parser.parse("310.15'K'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            assert_eq!(*value, 310.15);
            assert_eq!(*unit, Some("K".to_string()));
        }
    }

    // Test temperature addition (same units)
    let expr = parser.parse("20.0'Cel' + 5.0'Cel'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            // Now with Celsius base: 20°C + 5°C = 25°C (intuitive!)
            assert!((value - 25.0).abs() < 0.01);
            assert_eq!(*unit, Some("Cel".to_string()));
        }
    }

    // Test temperature subtraction (same units)
    let expr = parser.parse("100.0'[degF]' - 32.0'[degF]'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            // 100°F - 32°F = 68°F (result stays in Fahrenheit, left operand's unit)
            assert!((value - 68.0).abs() < 0.01);
            assert_eq!(*unit, Some("[degF]".to_string()));
        }
    }

    // Test temperature cross-unit conversion
    let expr = parser.parse("0.0'Cel' + 273.15'K'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            // 0°C + 273.15K: 0°C + 0°C = 0°C
            assert!((value - 0.0).abs() < 0.01);
            assert_eq!(*unit, Some("Cel".to_string()));
        }
    }

    // Test scalar multiplication
    let expr = parser.parse("100.0'Cel' * 2.0").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(values) = result {
        if let FhirPathValue::Quantity { value, unit } = &values[0] {
            assert_eq!(*value, 200.0);
            assert_eq!(*unit, Some("Cel".to_string()));
        }
    }
}

#[test]
fn test_temperature_incompatible_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test incompatible units (temperature + mass)
    let expr = parser.parse("37.0'Cel' + 5.0'kg'").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
}
