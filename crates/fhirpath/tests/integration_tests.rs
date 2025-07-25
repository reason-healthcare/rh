#[cfg(test)]
mod integration_tests {
    use fhirpath::*;
    use serde_json::json;

    #[test]
    fn test_fhirpath_integration() {
        // Create a sample FHIR Patient resource
        let patient = json!({
            "resourceType": "Patient",
            "id": "example",
            "name": [
                {
                    "use": "official",
                    "family": "Doe",
                    "given": ["John", "James"]
                },
                {
                    "use": "usual", 
                    "family": "Doe",
                    "given": ["Johnny"]
                }
            ],
            "birthDate": "1974-12-25"
        });

        // Create parser and evaluator
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(patient);

        // Test simple member access
        let expr = parser.parse("resourceType").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("resourceType = {:?}", result);

        // Test literal values
        let expr = parser.parse("true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("true = {:?}", result);
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("'Hello'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("'Hello' = {:?}", result);
        assert_eq!(result, FhirPathValue::String("Hello".to_string()));

        let expr = parser.parse("42").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("42 = {:?}", result);
        assert_eq!(result, FhirPathValue::Number(42.0));

        // Test function parsing (not evaluation yet)
        let expr = parser.parse("name.count()").unwrap();
        println!("Parsed function call: {}", expr);

        // Test complex expressions parsing
        let expr = parser.parse("name.where(use = 'official').given").unwrap();
        println!("Parsed where expression: {}", expr);

        let expr = parser.parse("name.given | name.family").unwrap();
        println!("Parsed union expression: {}", expr);
    }

    #[test]
    fn test_parser_examples() {
        let parser = FhirPathParser::new();

        // Test various FHIRPath expressions
        let expressions = vec![
            "Patient",
            "Patient.name",
            "Patient.name[0]",
            "Patient.name.given",
            "Patient.name.where(use = 'official')",
            "Patient.birthDate",
            "name.count()",
            "name.exists()",
            "name.empty()",
            "true",
            "false",
            "42",
            "3.14",
            "'hello world'",
            "{}",
            "$this",
            "$index",
            "%context",
            "name.given | name.family",
            "active = true",
            "birthDate >= @1980-01-01",
        ];

        for expr_str in expressions {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Parsed: {} -> {}", expr_str, expr);
                }
                Err(e) => {
                    println!("✗ Failed to parse: {} -> {:?}", expr_str, e);
                }
            }
        }
    }
}
