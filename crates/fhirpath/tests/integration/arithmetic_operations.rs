//! Arithmetic operation tests
//! 
//! Tests for mathematical operations in FHIRPath expressions

use super::*;

#[test]
fn test_addition() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer addition
    let expr = parser.parse("2 + 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(sum) = result {
        assert_eq!(sum, 5);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }

    // Test decimal addition
    let expr = parser.parse("2.5 + 1.5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Number(sum) = result {
        assert!((sum - 4.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected decimal value, got {:?}", result);
    }
}

#[test]
fn test_subtraction() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer subtraction
    let expr = parser.parse("10 - 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(diff) = result {
        assert_eq!(diff, 7);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }
}

#[test]
fn test_multiplication() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer multiplication
    let expr = parser.parse("4 * 5").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(product) = result {
        assert_eq!(product, 20);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }
}

#[test]
fn test_division() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer division (should return decimal)
    let expr = parser.parse("10 / 2").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Number(quotient) = result {
        assert!((quotient - 5.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected decimal value for division, got {:?}", result);
    }
}

#[test]
fn test_modulo() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer modulo
    let expr = parser.parse("10 mod 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(remainder) = result {
        assert_eq!(remainder, 1);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }
}

#[test]
fn test_integer_division() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test integer division (div operator)
    let expr = parser.parse("10 div 3").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(quotient) = result {
        assert_eq!(quotient, 3);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }
}

#[test]
fn test_operator_precedence() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test multiplication before addition: 2 + 3 * 4 = 2 + 12 = 14
    let expr = parser.parse("2 + 3 * 4").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Integer(result_val) = result {
        assert_eq!(result_val, 14);
    } else {
        panic!("Expected integer value, got {:?}", result);
    }
}
