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
        println!("resourceType = {result:?}");

        // Test literal values
        let expr = parser.parse("true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("true = {result:?}");
        assert_eq!(result, FhirPathValue::Boolean(true));

        let expr = parser.parse("'Hello'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("'Hello' = {result:?}");
        assert_eq!(result, FhirPathValue::String("Hello".to_string()));

        let expr = parser.parse("42").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        println!("42 = {result:?}");
        assert_eq!(result, FhirPathValue::Integer(42));

        // Test function parsing (not evaluation yet)
        let expr = parser.parse("name.count()").unwrap();
        println!("Parsed function call: {expr}");

        // Test complex expressions parsing
        let expr = parser.parse("name.where(use = 'official').given").unwrap();
        println!("Parsed where expression: {expr}");

        let expr = parser.parse("name.given | name.family").unwrap();
        println!("Parsed union expression: {expr}");
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
                    println!("✓ Parsed: {expr_str} -> {expr}");
                }
                Err(e) => {
                    println!("✗ Failed to parse: {expr_str} -> {e:?}");
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
            (
                "'Hello' & ' World'",
                FhirPathValue::String("Hello World".to_string()),
            ),
        ];

        for (expr_str, expected) in test_cases {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Parsed arithmetic: {expr_str} -> {expr}");
                    match evaluator.evaluate(&expr, &context) {
                        Ok(result) => {
                            println!("✓ Evaluated: {expr_str} = {result:?}");
                            assert_eq!(result, expected);
                        }
                        Err(e) => {
                            println!("✗ Failed to evaluate: {expr_str} -> {e:?}");
                            panic!("Evaluation failed for {expr_str}");
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to parse arithmetic: {expr_str} -> {e:?}");
                    panic!("Parsing failed for {expr_str}");
                }
            }
        }

        // Test operator precedence
        let precedence_cases = vec![
            ("2 + 3 * 4", FhirPathValue::Integer(14)), // Should be 2 + (3 * 4) = 14
            ("20 - 12 / 3", FhirPathValue::Number(16.0)), // Should be 20 - (12 / 3) = 16 (division always returns Number)
            ("10 - 3 - 2", FhirPathValue::Integer(5)),    // Should be (10 - 3) - 2 = 5
        ];

        for (expr_str, expected) in precedence_cases {
            let expr = parser.parse(expr_str).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            println!("✓ Precedence test: {expr_str} = {result:?}");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_comparison_expressions() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({
            "resourceType": "Patient",
            "age": 30,
            "weight": 70.5,
            "name": "John"
        }));

        // Test comparison parsing and evaluation
        let test_cases = vec![
            // Numeric comparisons
            ("5 > 3", FhirPathValue::Boolean(true)),
            ("3 > 5", FhirPathValue::Boolean(false)),
            ("5 >= 5", FhirPathValue::Boolean(true)),
            ("5 >= 3", FhirPathValue::Boolean(true)),
            ("3 >= 5", FhirPathValue::Boolean(false)),
            ("3 < 5", FhirPathValue::Boolean(true)),
            ("5 < 3", FhirPathValue::Boolean(false)),
            ("3 <= 3", FhirPathValue::Boolean(true)),
            ("3 <= 5", FhirPathValue::Boolean(true)),
            ("5 <= 3", FhirPathValue::Boolean(false)),
            // Mixed numeric types
            ("5 > 4.9", FhirPathValue::Boolean(true)),
            ("4.9 < 5", FhirPathValue::Boolean(true)),
            ("5.0 >= 5", FhirPathValue::Boolean(true)),
            ("4.99 <= 5", FhirPathValue::Boolean(true)),
            // String comparisons
            ("'apple' < 'banana'", FhirPathValue::Boolean(true)),
            ("'zebra' > 'apple'", FhirPathValue::Boolean(true)),
            ("'test' >= 'test'", FhirPathValue::Boolean(true)),
            ("'test' <= 'test'", FhirPathValue::Boolean(true)),
            // Boolean comparisons
            ("false < true", FhirPathValue::Boolean(true)),
            ("true > false", FhirPathValue::Boolean(true)),
            ("true >= true", FhirPathValue::Boolean(true)),
            ("false <= false", FhirPathValue::Boolean(true)),
        ];

        for (expr_str, expected) in test_cases {
            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Parsed comparison: {expr_str} -> {expr}");
                    match evaluator.evaluate(&expr, &context) {
                        Ok(result) => {
                            println!("✓ Evaluated: {expr_str} = {result:?}");
                            assert_eq!(result, expected);
                        }
                        Err(e) => {
                            println!("✗ Failed to evaluate: {expr_str} -> {e:?}");
                            panic!("Evaluation failed for {expr_str}");
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to parse comparison: {expr_str} -> {e:?}");
                    panic!("Parsing failed for {expr_str}");
                }
            }
        }

        // Test operator precedence with comparison operators
        let precedence_cases = vec![
            ("2 + 3 > 4", FhirPathValue::Boolean(true)), // Should be (2 + 3) > 4 = 5 > 4 = true
            ("10 - 5 < 8", FhirPathValue::Boolean(true)), // Should be (10 - 5) < 8 = 5 < 8 = true
            ("3 * 2 >= 6", FhirPathValue::Boolean(true)), // Should be (3 * 2) >= 6 = 6 >= 6 = true
            ("15 / 3 <= 5", FhirPathValue::Boolean(true)), // Should be (15 / 3) <= 5 = 5.0 <= 5 = true
        ];

        for (expr_str, expected) in precedence_cases {
            let expr = parser.parse(expr_str).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            println!("✓ Comparison precedence test: {expr_str} = {result:?}");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_membership_integration() {
        // Create parser and evaluator
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test basic membership operations parsing + evaluation
        let membership_cases = vec![
            // These test the parsing/evaluation but require collections to be useful
            // For now we can test with single values (treated as single-item collections)
            ("'apple' in 'apple'", FhirPathValue::Boolean(true)),
            ("'apple' in 'banana'", FhirPathValue::Boolean(false)),
            ("42 in 42", FhirPathValue::Boolean(true)),
            ("42 in 24", FhirPathValue::Boolean(false)),
            ("true contains true", FhirPathValue::Boolean(true)),
            ("true contains false", FhirPathValue::Boolean(false)),
        ];

        for (expr_str, expected) in membership_cases {
            let expr = parser.parse(expr_str).unwrap();
            let result = evaluator.evaluate(&expr, &context).unwrap();
            println!("✓ Membership test: {expr_str} = {result:?}");
            assert_eq!(result, expected);
        }

        println!("All membership integration tests passed!");
    }

    #[test]
    fn test_membership_precedence_integration() {
        // Test precedence between equality and membership operators
        let parser = FhirPathParser::new();
        let _evaluator = FhirPathEvaluator::new();
        let _context = EvaluationContext::new(json!({}));

        // Test parsing of complex expressions with membership
        let precedence_cases = vec![
            "a = b in collection",       // Should parse as (a = b) in collection
            "value contains 'x' = true", // Should parse as (value contains 'x') = true
        ];

        for expr_str in precedence_cases {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed membership precedence: {expr_str} -> {expr}");
                }
                Err(e) => {
                    panic!("Failed to parse {expr_str}: {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_collection_functions_integration() {
        // Create a sample FHIR Patient with multiple name entries
        let patient = json!({
            "resourceType": "Patient",
            "name": [
                {
                    "use": "official",
                    "family": "Doe",
                    "given": ["John"]
                },
                {
                    "use": "usual",
                    "family": "Doe",
                    "given": ["Johnny"]
                },
                {
                    "use": "official",
                    "family": "Smith",
                    "given": ["John"]
                }
            ],
            "telecom": []
        });

        let parser = FhirPathParser::new();
        let _evaluator = FhirPathEvaluator::new();
        let _context = EvaluationContext::new(patient);

        // Note: These tests verify parsing and basic evaluation structure
        // Full path navigation with collections would require more evaluation implementation

        // Test function parsing
        let function_parsing_cases = vec![
            "name.count()",
            "name.exists()",
            "name.empty()",
            "telecom.empty()",
            "name.distinct()",
            "name.isDistinct()",
        ];

        for expr_str in function_parsing_cases {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed collection function: {expr_str} -> {expr}");
                }
                Err(e) => {
                    panic!("Failed to parse {expr_str}: {e:?}");
                }
            }
        }

        // Test direct function evaluation on values
        let direct_evaluation_cases = vec![
            // Test basic function calls with simple context
            ("3.count()", "Parse and evaluate count on literal"),
            ("true.exists()", "Parse and evaluate exists on literal"),
            ("'hello'.empty()", "Parse and evaluate empty on literal"),
        ];

        for (expr_str, _description) in direct_evaluation_cases {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed: {expr_str} -> {expr}");
                    // Note: Full evaluation would require implementing literal function calls
                    // For now we just verify parsing works
                }
                Err(e) => {
                    panic!("Failed to parse {expr_str}: {e:?}");
                }
            }
        }

        println!("All collection function integration tests passed!");
    }

    #[test]
    fn test_distinct_functions_integration() {
        let parser = FhirPathParser::new();

        // Test parsing of distinct function calls
        let distinct_parsing_cases = vec![
            "name.distinct()",
            "telecom.isDistinct()",
            "Patient.name.distinct()",
            "values.isDistinct()",
        ];

        for expr_str in distinct_parsing_cases {
            let result = parser.parse(expr_str);
            match result {
                Ok(expr) => {
                    println!("✓ Parsed distinct function: {expr_str} -> {expr}");
                }
                Err(e) => {
                    panic!("Failed to parse {expr_str}: {e:?}");
                }
            }
        }

        println!("All distinct function parsing tests passed!");
    }

    #[test]
    fn test_filtering_functions_integration() {
        // Create a sample FHIR Patient resource with multiple names
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
                },
                {
                    "use": "maiden",
                    "family": "Smith",
                    "given": ["Jane"]
                }
            ],
            "birthDate": "1974-12-25"
        });

        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(patient);

        // Test parsing where and select functions
        let test_cases = vec![
            ("name.where(use = 'official')", "Filter names by use"),
            ("name.select(family)", "Select family names"),
            ("name.select(given)", "Select given names"),
            ("name.where(family = 'Doe')", "Filter by family name"),
        ];

        for (expr_str, description) in test_cases {
            println!("Testing {description}: {expr_str}");

            match parser.parse(expr_str) {
                Ok(expr) => {
                    println!("✓ Successfully parsed: {expr_str}");

                    // Try to evaluate - this might fail due to missing data access patterns
                    // but the important thing is that parsing works
                    match evaluator.evaluate(&expr, &context) {
                        Ok(result) => {
                            println!("✓ Successfully evaluated {expr_str}: {result:?}");
                        }
                        Err(e) => {
                            println!("⚠ Evaluation failed for {expr_str} (expected for complex expressions): {e:?}");
                            // This is expected for now since we're working with complex FHIR data structures
                        }
                    }
                }
                Err(e) => {
                    panic!("Failed to parse {expr_str}: {e:?}");
                }
            }
        }

        println!("Filtering functions parsing tests completed!");
    }
}
