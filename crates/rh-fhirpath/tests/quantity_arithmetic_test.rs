//! Integration tests for quantity arithmetic operations
//!
//! Tests the parsing and evaluation of quantity literals and arithmetic operations

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::json;

#[cfg(test)]
mod quantity_tests {
    use super::*;

    #[test]
    fn test_quantity_literal_evaluation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let test_cases: Vec<(&str, Decimal, Option<String>)> = vec![
            (
                "5'mg'",
                Decimal::from_str_exact("5.0").unwrap(),
                Some("mg".to_string()),
            ),
            (
                "10.5'kg'",
                Decimal::from_str_exact("10.5").unwrap(),
                Some("kg".to_string()),
            ),
            (
                "37'Cel'",
                Decimal::from_str_exact("37.0").unwrap(),
                Some("Cel".to_string()),
            ),
            (
                "120'mm[Hg]'",
                Decimal::from_str_exact("120.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            ("42''", Decimal::from_str_exact("42.0").unwrap(), None),
        ];

        for (expr_str, expected_value, expected_unit) in test_cases {
            let expr = parser
                .parse(expr_str)
                .unwrap_or_else(|_| panic!("Failed to parse {expr_str}"));
            let result = evaluator
                .evaluate(&expr, &context)
                .unwrap_or_else(|_| panic!("Failed to evaluate {expr_str}"));

            if let FhirPathValue::Quantity { value, unit } = result {
                assert_eq!(value, expected_value, "Value mismatch for {expr_str}");
                assert_eq!(unit, expected_unit, "Unit mismatch for {expr_str}");
            } else {
                panic!("Expected Quantity value for {expr_str}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_quantity_with_spaces() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let test_cases: Vec<(&str, Decimal, Option<String>)> = vec![
            (
                "15 'mm[Hg]'",
                Decimal::from_str_exact("15.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            (
                "37.2 'Cel'",
                Decimal::from_str_exact("37.2").unwrap(),
                Some("Cel".to_string()),
            ),
            (
                "5 'mg'",
                Decimal::from_str_exact("5.0").unwrap(),
                Some("mg".to_string()),
            ),
            (
                "2.5 'kg'",
                Decimal::from_str_exact("2.5").unwrap(),
                Some("kg".to_string()),
            ),
            (
                "120 'mm[Hg]'",
                Decimal::from_str_exact("120.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            (
                "98.6 '[degF]'",
                Decimal::from_str_exact("98.6").unwrap(),
                Some("[degF]".to_string()),
            ),
        ];

        for (expr_str, expected_value, expected_unit) in test_cases {
            let expr = parser
                .parse(expr_str)
                .unwrap_or_else(|_| panic!("Failed to parse {expr_str}"));
            let result = evaluator
                .evaluate(&expr, &context)
                .unwrap_or_else(|_| panic!("Failed to evaluate {expr_str}"));

            if let FhirPathValue::Quantity { value, unit } = result {
                assert_eq!(value, expected_value, "Value mismatch for {expr_str}");
                assert_eq!(unit, expected_unit, "Unit mismatch for {expr_str}");
            } else {
                panic!("Expected Quantity value for {expr_str}, got: {result:?}");
            }
        }

        let arithmetic_tests: Vec<(&str, Decimal, Option<String>)> = vec![
            (
                "15'mm[Hg]' + 5 'mm[Hg]'",
                Decimal::from_str_exact("20.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            (
                "120 'mm[Hg]' - 10'mm[Hg]'",
                Decimal::from_str_exact("110.0").unwrap(),
                Some("mm[Hg]".to_string()),
            ),
            (
                "5 'mg' * 2",
                Decimal::from_str_exact("10.0").unwrap(),
                Some("mg".to_string()),
            ),
        ];

        for (expr_str, expected_value, expected_unit) in arithmetic_tests {
            let expr = parser
                .parse(expr_str)
                .unwrap_or_else(|_| panic!("Failed to parse {expr_str}"));
            let result = evaluator
                .evaluate(&expr, &context)
                .unwrap_or_else(|_| panic!("Failed to evaluate {expr_str}"));

            if let FhirPathValue::Quantity { value, unit } = result {
                assert_eq!(value, expected_value, "Value mismatch for {expr_str}");
                assert_eq!(unit, expected_unit, "Unit mismatch for {expr_str}");
            } else {
                panic!("Expected Quantity value for {expr_str}, got: {result:?}");
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
            assert_eq!(value, Decimal::from_str_exact("8.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Compatible units with conversion - should work
        let expr = parser.parse("5'mg' + 3'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("3000005.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result with unit conversion, got: {result:?}");
        }

        // Incompatible units - should error
        let expr = parser.parse("5'mg' + 3'm'").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(
            result.is_err(),
            "Should error when adding incompatible units"
        );

        // Dimensionless quantities
        let expr = parser.parse("5'' + 3''").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("8.0").unwrap());
            assert_eq!(unit, None);
        } else {
            panic!("Expected dimensionless Quantity result, got: {result:?}");
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
            assert_eq!(value, Decimal::from_str_exact("7.0").unwrap());
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Compatible units with conversion - should work
        let expr = parser.parse("10'kg' - 3'mg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from(9_999_997));
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result with unit conversion, got: {result:?}");
        }

        // Incompatible units - should error
        let expr = parser.parse("10'kg' - 3'm'").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(
            result.is_err(),
            "Should error when subtracting incompatible units"
        );
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
            assert_eq!(value, Decimal::from_str_exact("10.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Scalar * quantity
        let expr = parser.parse("3 * 4'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("12.0").unwrap());
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Decimal scalar
        let expr = parser.parse("2.5'L' * 1.5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("3.75").unwrap());
            assert_eq!(unit, Some("L".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        let expr = parser.parse("2.0 'cm' * 2.0 'm'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert!((value.to_f64().unwrap() - 0.04).abs() < 0.001);
            assert_eq!(unit.as_deref(), Some("m2"));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
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
            assert_eq!(value, Decimal::from_str_exact("5.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Same units divide to a dimensionless Quantity with unit "1"
        // per the UCUM spec, which FHIRPath inherits.
        let expr = parser.parse("10'kg' / 5'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("2.0").unwrap());
            assert_eq!(unit.as_deref(), Some("1"));
        } else {
            panic!("Expected Quantity{{1.0, '1'}}, got: {result:?}");
        }

        let expr = parser.parse("4.0 'g' / 2.0 'm'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("2.0").unwrap());
            assert_eq!(unit.as_deref(), Some("g/m"));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
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
            assert_eq!(value, Decimal::from_str_exact("8.0").unwrap());
            assert_eq!(unit, None);
        } else {
            panic!("Expected dimensionless Quantity result, got: {result:?}");
        }

        // Quantity with unit + number should error
        let expr = parser.parse("5'mg' + 3").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(
            result.is_err(),
            "Should error when adding number to quantity with units"
        );
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
            assert_eq!(value, Decimal::from_str_exact("30.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Mixed operations
        let expr = parser.parse("20'kg' - 5'kg' / 2").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("17.5").unwrap());
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
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
            assert_eq!(value, Decimal::from_str_exact("-5.0").unwrap());
            assert_eq!(unit, Some("mg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }

        // Subtraction resulting in negative
        let expr = parser.parse("3'kg' - 8'kg'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, Decimal::from_str_exact("-5.0").unwrap());
            assert_eq!(unit, Some("kg".to_string()));
        } else {
            panic!("Expected Quantity result, got: {result:?}");
        }
    }
}
