#[cfg(test)]
mod long_conversion_tests {
    use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
    use serde_json::json;

    #[test]
    fn test_long_literal() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test 42L explicitly as Long literal
        let expr = parser.parse("42L").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 42),
            _ => panic!("Expected Long value, got {result:?}"),
        }

        // Test that regular integers are treated as Integers (via LongNumber in AST)
        // and can be converted to Long
        let expr = parser.parse("42.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 42),
            _ => panic!("Expected Long value from 42.toLong(), got {result:?}"),
        }
    }

    #[test]
    fn test_to_long_from_boolean() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // true should convert to 1L
        let expr = parser.parse("true.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 1),
            _ => panic!("Expected Long(1) from true.toLong(), got {result:?}"),
        }

        // false should convert to 0L
        let expr = parser.parse("false.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 0),
            _ => panic!("Expected Long(0) from false.toLong(), got {result:?}"),
        }
    }

    #[test]
    fn test_to_long_from_integer() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        let expr = parser.parse("42.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 42),
            _ => panic!("Expected Long(42) from 42.toLong(), got {result:?}"),
        }
    }

    #[test]
    fn test_to_long_from_string() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Valid string conversion
        let expr = parser.parse("'123'.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 123),
            _ => panic!("Expected Long(123) from '123'.toLong(), got {result:?}"),
        }

        // Invalid string conversion should return empty
        let expr = parser.parse("'not a number'.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Empty => {} // Expected
            _ => panic!("Expected Empty from 'not a number'.toLong(), got {result:?}"),
        }
    }

    #[test]
    fn test_to_long_from_number() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Whole number should convert
        let expr = parser.parse("42.0.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Long(val) => assert_eq!(val, 42),
            _ => panic!("Expected Long(42) from 42.0.toLong(), got {result:?}"),
        }

        // Fractional number should return empty
        let expr = parser.parse("42.5.toLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Empty => {} // Expected
            _ => panic!("Expected Empty from 42.5.toLong(), got {result:?}"),
        }
    }

    #[test]
    fn test_converts_to_long() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Boolean should convert
        let expr = parser.parse("true.convertsToLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Boolean(val) => assert!(val),
            _ => panic!("Expected Boolean(true) from true.convertsToLong(), got {result:?}"),
        }

        // Integer should convert
        let expr = parser.parse("42.convertsToLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Boolean(val) => assert!(val),
            _ => panic!("Expected Boolean(true) from 42.convertsToLong(), got {result:?}"),
        }

        // Invalid string should not convert
        let expr = parser.parse("'not a number'.convertsToLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Boolean(val) => assert!(!val),
            _ => panic!(
                "Expected Boolean(false) from 'not a number'.convertsToLong(), got {result:?}"
            ),
        }

        // Fractional number should not convert
        let expr = parser.parse("42.5.convertsToLong()").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Boolean(val) => assert!(!val),
            _ => panic!("Expected Boolean(false) from 42.5.convertsToLong(), got {result:?}"),
        }
    }
}
