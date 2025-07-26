//! Integration tests for the FHIRPath crate
//!
//! These tests validate the complete functionality of the FHIRPath parser and evaluator
//! working together to process real-world FHIRPath expressions against FHIR data.
//!
//! Tests are organized into focused modules that mirror the source code structure
//! for better maintainability and easier navigation.

#[cfg(test)]
mod tests {
    use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
    use serde_json::json;

    fn sample_patient() -> serde_json::Value {
        json!({
            "resourceType": "Patient",
            "id": "example",
            "active": true,
            "name": [{
                "use": "official",
                "family": "Doe",
                "given": ["John", "F."]
            }],
            "telecom": [
                {
                    "system": "phone",
                    "value": "555-1234",
                    "use": "home"
                },
                {
                    "system": "email",
                    "value": "john.doe@example.com",
                    "use": "work"
                }
            ],
            "gender": "male",
            "birthDate": "1980-01-01"
        })
    }

    #[test]
    fn test_basic_evaluation() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(sample_patient());

        // Test simple property access
        let expr = parser.parse("resourceType").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::String(resource_type) = result {
            assert_eq!(resource_type, "Patient");
        } else {
            panic!("Expected string value for resourceType, got {:?}", result);
        }

        // Test boolean literals
        let expr = parser.parse("true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("false").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        // Test integer literals
        let expr = parser.parse("42").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(42));

        // Test string literals
        let expr = parser.parse("'hello world'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::String("hello world".to_string()));
    }

    #[test]
    fn test_arithmetic_operations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test integer arithmetic
        let expr = parser.parse("2 + 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(5));

        let expr = parser.parse("10 - 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(7));

        let expr = parser.parse("4 * 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(20));

        let expr = parser.parse("15 / 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Number(5.0));

        let expr = parser.parse("17 mod 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(2));

        let expr = parser.parse("17 div 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(3));

        // Test operator precedence
        let expr = parser.parse("2 + 3 * 4").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Integer(14));
    }

    #[test]
    fn test_comparison_operations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test equality
        let expr = parser.parse("5 = 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("5 = 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        // Test inequality
        let expr = parser.parse("5 != 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test ordering
        let expr = parser.parse("3 < 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("5 > 3").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("5 >= 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("3 <= 5").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        // Test string comparison
        let expr = parser.parse("'apple' < 'banana'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_parsing_only() {
        let parser = FhirPathParser::new();

        // Test parsing of complex expressions that might not be fully evaluable yet
        let expressions = vec![
            "Patient.name",
            "Patient.name[0]",
            "Patient.name.given",
            "Patient.name.where(use = 'official')",
            "name.count()",
            "name.exists()",
            "name.empty()",
            "name.given | name.family",
            // Note: Date literals like @1980-01-01 are not yet implemented
            // "birthDate >= @1980-01-01",
        ];

        for expr_str in expressions {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Successfully parsed: {expr_str} -> {expr}");
                }
                Err(e) => {
                    panic!("Failed to parse valid expression {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_membership_operations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test basic membership operations with single values
        let expr = parser.parse("'apple' in 'apple'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("'apple' in 'banana'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        let expr = parser.parse("42 in 42").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("true contains true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_precedence_parsing() {
        let parser = FhirPathParser::new();

        // Test that these complex precedence expressions parse correctly
        let precedence_cases = vec![
            "a = b in collection",       // Should parse as (a = b) in collection
            "value contains 'x' = true", // Should parse as (value contains 'x') = true
            "2 + 3 * 4",                 // Should parse as 2 + (3 * 4)
            "10 - 5 < 8",                // Should parse as (10 - 5) < 8
        ];

        for expr_str in precedence_cases {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Parsed precedence expression: {expr_str} -> {expr}");
                }
                Err(e) => {
                    panic!("Failed to parse precedence expression {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_property_access() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let patient = sample_patient();
        let context = EvaluationContext::new(patient);

        // Test simple property access
        let expr = parser.parse("id").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        if let FhirPathValue::String(id) = result {
            assert_eq!(id, "example");
        } else {
            panic!("Expected string value for id, got {:?}", result);
        }

        // Test array access
        let expr = parser.parse("name").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        match result {
            FhirPathValue::Collection(_) => {
                // Collection results are expected for array properties
                println!("✓ Got collection for name property");
            }
            FhirPathValue::Object(_) => {
                // Might also be returned as an object
                println!("✓ Got object for name property");
            }
            _ => {
                panic!(
                    "Expected collection or object for name property, got {:?}",
                    result
                );
            }
        }
    }
}
