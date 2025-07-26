//! Quick verification test for string functions integration

#[cfg(test)]
mod quick_string_tests {
    use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};
    use serde_json::json;

    #[test]
    fn test_string_length_integration() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test if string length function works through the full pipeline
        let expr = parser.parse("'hello'.length()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(5));
    }

    #[test]
    fn test_string_upper_integration() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test if string upper function works through the full pipeline
        let expr = parser.parse("'hello'.upper()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String("HELLO".to_string()));
    }

    #[test]
    fn test_string_substring_integration() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test if string substring function works through the full pipeline
        let expr = parser.parse("'hello world'.substring(6)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String("world".to_string()));
    }
}
