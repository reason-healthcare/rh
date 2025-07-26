//! Additional comprehensive string function tests

#[cfg(test)]
mod comprehensive_string_tests {
    use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
    use serde_json::json;

    #[test]
    fn test_string_chaining() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test chaining string functions: trim -> upper
        let expr = parser.parse("'  hello world  '.trim().upper()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String("HELLO WORLD".to_string()));
    }

    #[test]
    fn test_string_replace_function() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test replace function
        let expr = parser
            .parse("'hello world'.replace('world', 'universe')")
            .unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String("hello universe".to_string()));
    }

    #[test]
    fn test_string_startswith_endswith() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test startsWith
        let expr = parser.parse("'hello world'.startsWith('hello')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test endsWith
        let expr = parser.parse("'hello world'.endsWith('world')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test startsWith false case
        let expr = parser.parse("'hello world'.startsWith('world')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_string_indexof() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test indexOf found
        let expr = parser.parse("'hello world'.indexOf('world')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(6));

        // Test indexOf not found
        let expr = parser.parse("'hello world'.indexOf('foo')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(-1));
    }

    #[test]
    fn test_string_split_join() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test split function
        let expr = parser.parse("'a,b,c'.split(',')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], FhirPathValue::String("a".to_string()));
                assert_eq!(items[1], FhirPathValue::String("b".to_string()));
                assert_eq!(items[2], FhirPathValue::String("c".to_string()));
            }
            _ => panic!("Expected Collection result from split"),
        }
    }
}
