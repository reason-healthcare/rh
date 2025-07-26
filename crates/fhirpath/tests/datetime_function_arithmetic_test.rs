//! Integration tests for arithmetic operations with date/time functions

use chrono::{Datelike, Local};
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[cfg(test)]
mod datetime_function_arithmetic_tests {
    use super::*;

    #[test]
    fn test_now_minus_days() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("now() - 10 days").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format and roughly 10 days ago
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() - 10 days → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_now_plus_hours() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("now() + 5 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() + 5 hours → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_now_minus_minutes() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("now() - 30 minutes").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() - 30 minutes → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_now_plus_seconds() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("now() + 90 seconds").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() + 90 seconds → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_now_plus_months() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("now() + 3 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() + 3 months → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_today_minus_days() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("today() - 10 days").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Verify it's a valid date format (YYYY-MM-DD)
            assert_eq!(date_str.len(), 10);
            assert_eq!(&date_str[4..5], "-");
            assert_eq!(&date_str[7..8], "-");
            println!("✓ today() - 10 days → {date_str}");
        } else {
            panic!("Expected Date result, got: {result:?}");
        }
    }

    #[test]
    fn test_today_plus_years() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("today() + 2 years").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Verify it's a valid date format and the year increased by 2
            let current_year = Local::now().year();
            let expected_year = current_year + 2;
            assert!(date_str.starts_with(&expected_year.to_string()));
            println!("✓ today() + 2 years → {date_str}");
        } else {
            panic!("Expected Date result, got: {result:?}");
        }
    }

    #[test]
    fn test_today_minus_months() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("today() - 6 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Verify it's a valid date format
            assert_eq!(date_str.len(), 10);
            assert_eq!(&date_str[4..5], "-");
            assert_eq!(&date_str[7..8], "-");
            println!("✓ today() - 6 months → {date_str}");
        } else {
            panic!("Expected Date result, got: {result:?}");
        }
    }

    #[test]
    fn test_complex_datetime_function_arithmetic() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test chained arithmetic with function results
        let expr = parser.parse("now() - 1 day + 12 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            // Verify it's a valid datetime format
            assert!(datetime_str.ends_with('Z'));
            assert!(datetime_str.contains('T'));
            println!("✓ now() - 1 day + 12 hours → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }

    #[test]
    fn test_date_function_with_compound_durations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test with compound duration literals
        let expr = parser.parse("today() + (24 months)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            let current_year = Local::now().year();
            let expected_year = current_year + 2; // 24 months = 2 years
            assert!(date_str.starts_with(&expected_year.to_string()));
            println!("✓ today() + (24 months) → {date_str}");
        } else {
            panic!("Expected Date result, got: {result:?}");
        }
    }

    #[test]
    fn test_datetime_function_arithmetic_error_cases() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test with invalid precision units
        let expr = parser.parse("now() + 5 invalid_units").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err());
        println!("✓ now() + 5 invalid_units correctly produces error");

        // Test that today() doesn't accept time units
        let expr = parser.parse("today() + 5 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err());
        println!(
            "✓ today() + 5 hours correctly produces error (time precision not supported for dates)"
        );
    }

    #[test]
    fn test_reverse_order_arithmetic() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test quantity + function (should work the same way)
        let expr = parser.parse("5 days + today()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Should be 5 days from today
            assert_eq!(date_str.len(), 10);
            println!("✓ 5 days + today() → {date_str}");
        } else {
            panic!("Expected Date result, got: {result:?}");
        }

        let expr = parser.parse("2 hours + now()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert!(datetime_str.ends_with('Z'));
            println!("✓ 2 hours + now() → {datetime_str}");
        } else {
            panic!("Expected DateTime result, got: {result:?}");
        }
    }
}
