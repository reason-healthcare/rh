//! Tests for datetime precision parsing and evaluation

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use fhirpath::ast::DateTimePrecision;
use serde_json::json;

#[cfg(test)]
mod datetime_precision_tests {
    use super::*;

    #[test]
    fn test_datetime_precision_parsing() {
        let parser = FhirPathParser::new();

        // Test singular forms
        let test_cases = [
            ("year", DateTimePrecision::Year),
            ("month", DateTimePrecision::Month),
            ("week", DateTimePrecision::Week),
            ("day", DateTimePrecision::Day),
            ("hour", DateTimePrecision::Hour),
            ("minute", DateTimePrecision::Minute),
            ("second", DateTimePrecision::Second),
            ("millisecond", DateTimePrecision::Millisecond),
        ];

        for (input, expected) in test_cases {
            let result = parser.parse(input).unwrap();
            println!("✓ Parsed precision literal: {input} -> {result}");
            
            // Verify it's the expected precision type
            if let fhirpath::ast::Expression::Term(fhirpath::ast::Term::Literal(fhirpath::ast::Literal::DateTimePrecision(precision))) = result.root {
                assert_eq!(precision, expected, "Precision mismatch for {input}");
            } else {
                panic!("Expected DateTimePrecision literal for {input}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_datetime_precision_parsing_plural() {
        let parser = FhirPathParser::new();

        // Test plural forms
        let test_cases = [
            ("years", DateTimePrecision::Year),
            ("months", DateTimePrecision::Month),
            ("weeks", DateTimePrecision::Week),
            ("days", DateTimePrecision::Day),
            ("hours", DateTimePrecision::Hour),
            ("minutes", DateTimePrecision::Minute),
            ("seconds", DateTimePrecision::Second),
            ("milliseconds", DateTimePrecision::Millisecond),
        ];

        for (input, expected) in test_cases {
            let result = parser.parse(input).unwrap();
            println!("✓ Parsed precision literal (plural): {input} -> {result}");
            
            // Verify it's the expected precision type
            if let fhirpath::ast::Expression::Term(fhirpath::ast::Term::Literal(fhirpath::ast::Literal::DateTimePrecision(precision))) = result.root {
                assert_eq!(precision, expected, "Precision mismatch for {input}");
            } else {
                panic!("Expected DateTimePrecision literal for {input}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_datetime_precision_evaluation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test evaluation of precision literals
        let test_cases = [
            ("year", DateTimePrecision::Year),
            ("months", DateTimePrecision::Month),
            ("week", DateTimePrecision::Week),
            ("days", DateTimePrecision::Day),
            ("hour", DateTimePrecision::Hour),
            ("minutes", DateTimePrecision::Minute),
            ("second", DateTimePrecision::Second),
            ("milliseconds", DateTimePrecision::Millisecond),
        ];

        for (input, expected) in test_cases {
            let expr = parser.parse(input).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            
            if let FhirPathValue::DateTimePrecision(precision) = result {
                assert_eq!(precision, expected, "Evaluation precision mismatch for {input}");
                println!("✓ Evaluated precision: {input} -> {precision:?}");
            } else {
                panic!("Expected DateTimePrecision value for {input}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_datetime_precision_equality() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test that singular and plural forms are equal
        let equality_tests = [
            ("year = years", true),
            ("month = months", true),
            ("week = weeks", true),
            ("day = days", true),
            ("hour = hours", true),
            ("minute = minutes", true),
            ("second = seconds", true),
            ("millisecond = milliseconds", true),
            ("year = month", false),
            ("hour = minute", false),
        ];

        for (expression, expected) in equality_tests {
            let expr = parser.parse(expression).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            
            if let FhirPathValue::Boolean(actual) = result {
                assert_eq!(actual, expected, "Equality test failed for: {expression}");
                println!("✓ Equality test: {expression} -> {actual}");
            } else {
                panic!("Expected Boolean result for {expression}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_datetime_precision_in_collections() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test precision values in unions and collections
        let collection_tests = [
            ("(year | month | day).count()", 3),
            ("(hours | minutes | seconds).count()", 3),
        ];

        for (expression, expected_count) in collection_tests {
            let expr = parser.parse(expression).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            
            if let FhirPathValue::Integer(count) = result {
                assert_eq!(count, expected_count, "Collection count test failed for: {expression}");
                println!("✓ Collection test: {expression} -> {count}");
            } else {
                panic!("Expected Integer result for {expression}, got: {result:?}");
            }
        }
    }

    #[test]
    fn test_datetime_precision_display() {
        use fhirpath::ast::DateTimePrecision;
        
        // Test Display implementation
        assert_eq!(format!("{}", DateTimePrecision::Year), "year");
        assert_eq!(format!("{}", DateTimePrecision::Month), "month");
        assert_eq!(format!("{}", DateTimePrecision::Week), "week");
        assert_eq!(format!("{}", DateTimePrecision::Day), "day");
        assert_eq!(format!("{}", DateTimePrecision::Hour), "hour");
        assert_eq!(format!("{}", DateTimePrecision::Minute), "minute");
        assert_eq!(format!("{}", DateTimePrecision::Second), "second");
        assert_eq!(format!("{}", DateTimePrecision::Millisecond), "millisecond");
        
        println!("✓ All Display implementations work correctly");
    }
}
