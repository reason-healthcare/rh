use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

#[test]
fn test_quantity_comparison_same_unit() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test greater than with same unit
    let expr = parser.parse("5 'kg' > 3 'kg'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, rh_fhirpath::FhirPathValue::Boolean(true)));

    // Test less than with same unit
    let expr = parser.parse("3 'kg' < 5 'kg'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, rh_fhirpath::FhirPathValue::Boolean(true)));

    // Test equal with same unit
    let expr = parser.parse("5 'kg' = 5 'kg'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, rh_fhirpath::FhirPathValue::Boolean(true)));
}

#[test]
fn test_quantity_comparison_different_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test kg to g comparison: 5.4 kg > 5000 g
    // 5.4 kg = 5400 g, so 5400 > 5000 is true
    let expr = parser.parse("5.4 'kg' > 5000 'g'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        rh_fhirpath::FhirPathValue::Boolean(b) => {
            assert!(b, "Expected 5.4 kg > 5000 g to be true");
        }
        _ => panic!("Expected Boolean result, got: {result:?}"),
    }

    // Test g to kg comparison: 1000 g = 1 kg
    let expr = parser.parse("1000 'g' = 1 'kg'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        rh_fhirpath::FhirPathValue::Boolean(b) => {
            assert!(b, "Expected 1000 g = 1 kg to be true");
        }
        _ => panic!("Expected Boolean result, got: {result:?}"),
    }

    // Test mg to g comparison: 500 mg < 1 g
    let expr = parser.parse("500 'mg' < 1 'g'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        rh_fhirpath::FhirPathValue::Boolean(b) => {
            assert!(b, "Expected 500 mg < 1 g to be true");
        }
        _ => panic!("Expected Boolean result, got: {result:?}"),
    }
}

#[test]
fn test_quantity_comparison_length_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test cm to m comparison
    let expr = parser.parse("150 'cm' > 1 'm'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        rh_fhirpath::FhirPathValue::Boolean(b) => {
            assert!(b, "Expected 150 cm > 1 m to be true");
        }
        _ => panic!("Expected Boolean result, got: {result:?}"),
    }

    // Test m to cm comparison
    let expr = parser.parse("2 'm' = 200 'cm'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        rh_fhirpath::FhirPathValue::Boolean(b) => {
            assert!(b, "Expected 2 m = 200 cm to be true");
        }
        _ => panic!("Expected Boolean result, got: {result:?}"),
    }
}

#[test]
fn test_quantity_comparison_incompatible_units() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test incompatible units (mass vs length) should error
    let expr = parser.parse("5 'kg' > 100 'cm'").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(
        result.is_err(),
        "Expected error when comparing incompatible units"
    );
}

#[test]
fn test_quantity_comparison_temperature() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test Celsius comparison
    let expr = parser.parse("100 'Cel' > 37 'Cel'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, rh_fhirpath::FhirPathValue::Boolean(true)));

    // Test Fahrenheit to Celsius: 98.6 F ≈ 37 C
    let expr = parser.parse("98.6 '[degF]' > 36 'Cel'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, rh_fhirpath::FhirPathValue::Boolean(true)));
}
