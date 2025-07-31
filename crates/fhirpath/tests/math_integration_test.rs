//! Integration tests for math functions in FHIRPath expressions

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[cfg(test)]
mod math_integration_tests {
    use super::*;

    #[test]
    fn test_abs_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({"value": -5}));

        let expr = parser.parse("(-10).abs()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(10));

        let expr = parser.parse("(-3.15).abs()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.15));
    }

    #[test]
    fn test_ceiling_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(3.14).ceiling()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(4));

        let expr = parser.parse("(-2.7).ceiling()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-2));
    }

    #[test]
    fn test_floor_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(3.14).floor()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));

        let expr = parser.parse("(-2.7).floor()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-3));
    }

    #[test]
    fn test_sqrt_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(16).sqrt()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(4.0));

        let expr = parser.parse("(2.25).sqrt()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(1.5));
    }

    #[test]
    fn test_truncate_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(3.14).truncate()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));

        let expr = parser.parse("(-2.7).truncate()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-2));
    }

    #[test]
    fn test_power_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(2).power(3)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(8));

        let expr = parser.parse("(2.0).power(0.5)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        // sqrt(2) ≈ computed value
        if let FhirPathValue::Number(n) = result {
            assert!((n - 2.0_f64.sqrt()).abs() < 1e-10);
        } else {
            panic!("Expected Number result");
        }
    }

    #[test]
    fn test_round_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Round with precision
        let expr = parser.parse("(3.15159).round(2)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.15));

        // Round without precision (defaults to 0)
        let expr = parser.parse("(3.7).round()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(4.0));
    }

    #[test]
    fn test_log_and_ln_functions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Natural logarithm
        let expr = parser.parse("(2.718281828459045).ln()").unwrap(); // e
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Number(n) = result {
            assert!((n - 1.0).abs() < 1e-10);
        } else {
            panic!("Expected Number result");
        }

        // Logarithm with base 10
        let expr = parser.parse("(100).log(10)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(2.0));

        // Logarithm with base 2
        let expr = parser.parse("(8).log(2)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(3.0));
    }

    #[test]
    fn test_exp_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("(1).exp()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::Number(n) = result {
            // e ≈ computed value
            assert!((n - 1.0_f64.exp()).abs() < 1e-10);
        } else {
            panic!("Expected Number result");
        }

        let expr = parser.parse("(0).exp()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(1.0));
    }

    #[test]
    fn test_math_function_chaining() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Chain functions: sqrt(16).abs().floor()
        let expr = parser.parse("(16).sqrt().abs().floor()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(4));

        // More complex: (-3.7).abs().ceiling()
        let expr = parser.parse("(-3.7).abs().ceiling()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(4));
    }

    #[test]
    fn test_math_error_cases() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // sqrt of negative number should error
        let expr = parser.parse("(-1).sqrt()").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err());

        // ln of negative number should error
        let expr = parser.parse("(-1).ln()").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err());

        // log with invalid base should error
        let expr = parser.parse("(10).log(1)").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err());
    }
}
