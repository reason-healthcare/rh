//! Integration tests for quantity arithmetic operations
//!
//! Tests the parsing and evaluation of quantity literals and arithmetic operations

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[cfg(test)]
mod quantity_tests {
    use super::*;

    #[test]
    fn test_quantity_literal_evaluation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test basic quantity evaluation
        let test_cases = [
            ("5'mg'", 5.0, Some("mg".to_string())),
            ("10.5'kg'", 10.5, Some("kg".to_string())),
            ("37'Cel'", 37.0, Some("Cel".to_string())),
            ("120'mm[Hg]'", 120.0, Some("mm[Hg]".to_string())),
            ("42''", 42.0, None), // Empty unit (dimensionless)
        ];

        for (expr_str, expected_value, expected_unit) in test_cases {
            let expr = parser.parse(expr_str).expect(&format!("Failed to parse {}", expr_str));
            let result = evaluator.evaluate(&expr, &context).expect(&format!("Failed to evaluate {}", expr_str));
            
            if let FhirPathValue::Quantity { value, unit } = result {
                assert_eq!(value, expected_value, "Value mismatch for {}", expr_str);
                assert_eq!(unit, expected_unit, "Unit mismatch for {}", expr_str);
            } else {
                panic!("Expected Quantity value for {}, got: {:?}", expr_str, result);
            }
        }
    }

    #[test]
    fn test_quantity_addition() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Same units - should work
        let expr = parser.parse("5'mg' + 3'mg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 8.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Different units - should error
        let expr = parser.parse("5'mg' + 3'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error when adding different units");

        // Dimensionless quantities
        let expr = parser.parse("5'' + 3''").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 8.0);
            assert_eq!(unit, None);
        } else {
            panic!("Expected dimensionless Quantity result, got: {:?}", result);
        }
    }

    #[test]
    fn test_quantity_subtraction() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Same units - should work
        let expr = parser.parse("10'kg' - 3'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 7.0);
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Different units - should error
        let expr = parser.parse("10'kg' - 3'mg'").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error when subtracting different units");
    }

    #[test]
    fn test_quantity_multiplication() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Quantity * scalar
        let expr = parser.parse("5'mg' * 2").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 10.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Scalar * quantity
        let expr = parser.parse("3 * 4'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 12.0);
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Decimal scalar
        let expr = parser.parse("2.5'L' * 1.5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 3.75);
            assert_eq!(unit, Some("L".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }
    }

    #[test]
    fn test_quantity_division() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Quantity / scalar
        let expr = parser.parse("10'mg' / 2").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 5.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Same units divide to dimensionless number
        let expr = parser.parse("10'kg' / 5'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Number(ratio) = result {
            assert_eq!(ratio, 2.0);
        } else {
            panic!("Expected Number result, got: {:?}", result);
        }

        // Division by zero
        let expr = parser.parse("10'mg' / 0").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error on division by zero");
    }

    #[test]
    fn test_quantity_with_dimensionless_operations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Dimensionless quantity + number
        let expr = parser.parse("5'' + 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 8.0);
            assert_eq!(unit, None);
        } else {
            panic!("Expected dimensionless Quantity result, got: {:?}", result);
        }

        // Quantity with unit + number should error
        let expr = parser.parse("5'mg' + 3").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error when adding number to quantity with units");
    }

    #[test]
    fn test_complex_quantity_expressions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Multiple operations
        let expr = parser.parse("(10'mg' + 5'mg') * 2").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 30.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Mixed operations
        let expr = parser.parse("20'kg' - 5'kg' / 2").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 17.5); // 20 - (5/2) = 20 - 2.5 = 17.5
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }
    }

    #[test]
    fn test_negative_quantities() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Negative quantity literal (polarity expression)
        let expr = parser.parse("-5'mg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, -5.0);
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }

        // Subtraction resulting in negative
        let expr = parser.parse("3'kg' - 8'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, -5.0);
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {:?}", result);
        }
    }
}
