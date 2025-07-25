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
        assert_eq!(result, FhirPathValue::Integer(42));

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

    #[test]
    fn test_arithmetic_expressions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({
            "resourceType": "Patient",
            "age": 30,
            "weight": 70.5,
            "height": 175
        }));

        // Test arithmetic parsing and evaluation
        let test_cases = vec![
            ("1 + 2", FhirPathValue::Integer(3)),
            ("10 - 3", FhirPathValue::Integer(7)),
            ("4 * 5", FhirPathValue::Integer(20)),
            ("15 / 3", FhirPathValue::Number(5.0)),
            ("17 div 5", FhirPathValue::Integer(3)),
            ("17 mod 5", FhirPathValue::Integer(2)),
            ("2.5 + 3", FhirPathValue::Number(5.5)),
            ("'Hello' & ' World'", FhirPathValue::String("Hello World".to_string())),
        ];

        for (expr_str, expected) in test_cases {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Parsed arithmetic: {} -> {}", expr_str, expr);
                    match evaluator.evaluate(&expr, &context) {
                        Ok(result) => {
                            println!("✓ Evaluated: {} = {:?}", expr_str, result);
                            assert_eq!(result, expected);
                        }
                        Err(e) => {
                            println!("✗ Failed to evaluate: {} -> {:?}", expr_str, e);
                            panic!("Evaluation failed for {}", expr_str);
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to parse arithmetic: {} -> {:?}", expr_str, e);
                    panic!("Parsing failed for {}", expr_str);
                }
            }
        }

        // Test operator precedence
        let precedence_cases = vec![
            ("2 + 3 * 4", FhirPathValue::Integer(14)), // Should be 2 + (3 * 4) = 14
            ("20 - 12 / 3", FhirPathValue::Number(16.0)), // Should be 20 - (12 / 3) = 16 (division always returns Number)
            ("10 - 3 - 2", FhirPathValue::Integer(5)), // Should be (10 - 3) - 2 = 5
        ];

        for (expr_str, expected) in precedence_cases {
            let expr = parser.parse(expr_str).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            println!("✓ Precedence test: {} = {:?}", expr_str, result);
            assert_eq!(result, expected);
        }
    }
}
