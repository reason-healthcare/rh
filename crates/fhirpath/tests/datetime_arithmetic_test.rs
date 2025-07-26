//! Tests for date/time arithmetic operations

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[cfg(test)]
mod datetime_arithmetic_tests {
    use super::*;

    #[test]
    fn test_date_year_only_parsing() {
        let parser = FhirPathParser::new();

        // Test year-only date parsing
        let expr = parser.parse("@2025").unwrap();
        println!("✓ Parsed year-only date: @2025 -> {expr}");

        // Test evaluation
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025");
            println!("✓ Evaluated year-only date: @2025 -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_date_year_month_parsing() {
        let parser = FhirPathParser::new();

        // Test year-month date parsing
        let expr = parser.parse("@2025-03").unwrap();
        println!("✓ Parsed year-month date: @2025-03 -> {expr}");

        // Test evaluation
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025-03");
            println!("✓ Evaluated year-month date: @2025-03 -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_date_plus_months() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2025 + 6 months
        let expr = parser.parse("@2025 + 6 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025"); // Year should stay same, this adds to Jan 1st
            println!("✓ @2025 + 6 months -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }

        // Test @2019-03-01 + 6 months
        let expr = parser.parse("@2019-03-01 + 6 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2019-09-01");
            println!("✓ @2019-03-01 + 6 months -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_date_minus_months() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2019-03-01 - 24 months
        let expr = parser.parse("@2019-03-01 - 24 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2017-03-01");
            println!("✓ @2019-03-01 - 24 months -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }

        // Test @2025-12 - 6 months
        let expr = parser.parse("@2025-12 - 6 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025-06");
            println!("✓ @2025-12 - 6 months -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_date_plus_years() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2020 + 5 years
        let expr = parser.parse("@2020 + 5 years").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025");
            println!("✓ @2020 + 5 years -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }

        // Test @2019-02-28 + 1 year
        let expr = parser.parse("@2019-02-28 + 1 year").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2020-02-28");
            println!("✓ @2019-02-28 + 1 year -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_date_plus_days_weeks() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2025-01-01 + 7 days
        let expr = parser.parse("@2025-01-01 + 7 days").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025-01-08");
            println!("✓ @2025-01-01 + 7 days -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }

        // Test @2025-01-01 + 2 weeks
        let expr = parser.parse("@2025-01-01 + 2 weeks").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025-01-15");
            println!("✓ @2025-01-01 + 2 weeks -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_datetime_plus_precision() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2025-01-01T12:00:00 + 6 hours
        let expr = parser.parse("@2025-01-01T12:00:00 + 6 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert_eq!(datetime_str, "2025-01-01T18:00:00");
            println!("✓ @2025-01-01T12:00:00 + 6 hours -> {datetime_str}");
        } else {
            panic!("Expected DateTime value, got: {result:?}");
        }

        // Test @2025-01-01T12:00:00 + 30 minutes
        let expr = parser.parse("@2025-01-01T12:00:00 + 30 minutes").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert_eq!(datetime_str, "2025-01-01T12:30:00");
            println!("✓ @2025-01-01T12:00:00 + 30 minutes -> {datetime_str}");
        } else {
            panic!("Expected DateTime value, got: {result:?}");
        }

        // Test @2025-01-01T12:00:00Z + 1 day (with timezone)
        let expr = parser.parse("@2025-01-01T12:00:00Z + 1 day").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert_eq!(datetime_str, "2025-01-02T12:00:00Z");
            println!("✓ @2025-01-01T12:00:00Z + 1 day -> {datetime_str}");
        } else {
            panic!("Expected DateTime value, got: {result:?}");
        }
    }

    #[test]
    fn test_datetime_minus_precision() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test @2025-01-01T12:00:00 - 2 hours
        let expr = parser.parse("@2025-01-01T12:00:00 - 2 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert_eq!(datetime_str, "2025-01-01T10:00:00");
            println!("✓ @2025-01-01T12:00:00 - 2 hours -> {datetime_str}");
        } else {
            panic!("Expected DateTime value, got: {result:?}");
        }

        // Test @2025-01-01T12:00:00+05:30 - 3 months (with timezone)
        let expr = parser
            .parse("@2025-01-01T12:00:00+05:30 - 3 months")
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::DateTime(datetime_str) = result {
            assert_eq!(datetime_str, "2024-10-01T12:00:00+05:30");
            println!("✓ @2025-01-01T12:00:00+05:30 - 3 months -> {datetime_str}");
        } else {
            panic!("Expected DateTime value, got: {result:?}");
        }
    }

    #[test]
    fn test_number_plus_precision_compound() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test that "6 months" creates a compound duration quantity
        let expr = parser.parse("6 months").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        // Should create a quantity representing the compound duration
        if let FhirPathValue::Quantity { value, unit } = result {
            assert_eq!(value, 6.0);
            assert_eq!(unit, Some("month".to_string()));
            println!("✓ 6 months creates quantity: value={value}, unit={unit:?}");
        } else {
            panic!("Expected Quantity value for compound duration, got: {result:?}");
        }

        // Test using it with date
        let expr = parser.parse("@2025-01-01 + (6 months)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            assert_eq!(date_str, "2025-07-01");
            println!("✓ @2025-01-01 + (6 months) -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_leap_year_handling() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test leap year handling: Feb 29 + 1 year
        let expr = parser.parse("@2020-02-29 + 1 year").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Should become Feb 28 in non-leap year
            assert_eq!(date_str, "2021-02-28");
            println!("✓ @2020-02-29 + 1 year -> {date_str} (leap year handling)");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }

        // Test month overflow: January 31 + 1 month
        let expr = parser.parse("@2025-01-31 + 1 month").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // Should become Feb 28 (February has fewer days)
            assert_eq!(date_str, "2025-02-28");
            println!("✓ @2025-01-31 + 1 month -> {date_str} (month overflow handling)");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_complex_datetime_arithmetic_expressions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test multiple operations: @2025-01-01 + 6 months - 2 weeks
        let expr = parser.parse("@2025-01-01 + 6 months - 2 weeks").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        if let FhirPathValue::Date(date_str) = result {
            // January + 6 months = July, then - 2 weeks
            assert_eq!(date_str, "2025-06-17");
            println!("✓ @2025-01-01 + 6 months - 2 weeks -> {date_str}");
        } else {
            panic!("Expected Date value, got: {result:?}");
        }
    }

    #[test]
    fn test_datetime_arithmetic_error_cases() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test invalid operations: should error appropriately

        // Adding time precision to date should error for sub-day precisions
        let expr = parser.parse("@2025-01-01 + 6 hours").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error when adding hours to date");
        println!("✓ @2025-01-01 + 6 hours correctly errors");

        // Adding minutes to date should error
        let expr = parser.parse("@2025-01-01 + 30 minutes").unwrap();
        let result = evaluator.evaluate(&expr, &context);
        assert!(result.is_err(), "Should error when adding minutes to date");
        println!("✓ @2025-01-01 + 30 minutes correctly errors");
    }
}
