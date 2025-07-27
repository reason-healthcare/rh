//! Integration tests for the FHIRPath crate
//!
//! These tests validate the complete functionality of the FHIRPath parser and evaluator
//! working together to process real-world FHIRPath expressions against FHIR data.
//!
//! Tests are organized into focused modules that mirror the source code structure
//! for better maintainability and easier navigation.
//!
//! Note: Example validation tests are in examples_test.rs - these ensure all
//! FHIRPath examples build and run correctly without exceptions.

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
            panic!("Expected string value for resourceType, got {result:?}");
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
            panic!("Expected string value for id, got {result:?}");
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
                panic!("Expected collection or object for name property, got {result:?}");
            }
        }
    }

    #[test]
    fn test_array_indexing() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        // Create a patient with multiple name entries for testing
        let patient_with_multiple_names = json!({
            "resourceType": "Patient",
            "id": "example",
            "active": true,
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

        let context = EvaluationContext::new(patient_with_multiple_names);

        // Test indexing name[0] - should get first name entry
        let expr = parser.parse("name[0]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Object(name_obj) => {
                // Verify it's the first name object with "official" use
                if let Some(serde_json::Value::String(use_str)) = name_obj.get("use") {
                    assert_eq!(use_str, "official");
                } else {
                    panic!("Expected 'use' field with string value in name[0]");
                }
            }
            _ => {
                panic!("Expected object value for name[0], got {result:?}");
            }
        }

        // Test indexing name[1] - should get second name entry
        let expr = parser.parse("name[1]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Object(name_obj) => {
                // Verify it's the second name object with "usual" use
                if let Some(serde_json::Value::String(use_str)) = name_obj.get("use") {
                    assert_eq!(use_str, "usual");
                } else {
                    panic!("Expected 'use' field with string value in name[1]");
                }
            }
            _ => {
                panic!("Expected object value for name[1], got {result:?}");
            }
        }

        // Test out of bounds indexing - should return empty collection
        let expr = parser.parse("name[10]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Empty => {
                // This is the expected result for out of bounds access
            }
            _ => {
                panic!("Expected Empty value for out of bounds index name[10], got {result:?}");
            }
        }

        // Test nested array indexing - accessing first given name from first name entry
        let expr = parser.parse("name[0].given[0]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::String(given_name) => {
                assert_eq!(given_name, "John");
            }
            _ => {
                panic!("Expected string value for name[0].given[0], got {result:?}");
            }
        }

        // Test nested array indexing - accessing second given name from first name entry
        let expr = parser.parse("name[0].given[1]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::String(given_name) => {
                assert_eq!(given_name, "James");
            }
            _ => {
                panic!("Expected string value for name[0].given[1], got {result:?}");
            }
        }

        // Test indexing on primitive collections
        let expr = parser.parse("(10 | 20 | 30)[0]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Integer(num) => {
                assert_eq!(num, 10);
            }
            _ => {
                panic!("Expected integer value for primitive collection[0], got {result:?}");
            }
        }

        // Test indexing on primitive collections - second element
        let expr = parser.parse("(10 | 20 | 30)[1]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Integer(num) => {
                assert_eq!(num, 20);
            }
            _ => {
                panic!("Expected integer value for primitive collection[1], got {result:?}");
            }
        }

        // Test out of bounds on primitive collection
        let expr = parser.parse("(10 | 20 | 30)[5]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Empty => {
                // This is the expected result for out of bounds access
            }
            _ => {
                panic!("Expected Empty value for out of bounds primitive collection[5], got {result:?}");
            }
        }

        // Test indexing on string collections
        let expr = parser.parse("('a' | 'b' | 'c')[0]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::String(s) => {
                assert_eq!(s, "a");
            }
            _ => {
                panic!("Expected string value for string collection[0], got {result:?}");
            }
        }

        // Test indexing on empty collection
        let expr = parser.parse("{}[0]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Empty => {
                // This is the expected result for indexing empty collection
            }
            _ => {
                panic!("Expected Empty value for empty collection[0], got {result:?}");
            }
        }

        println!("Array indexing test completed successfully!");
    }

    #[test]
    fn test_union_operations() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();

        // Create test data with various collections
        let patient = json!({
            "resourceType": "Patient",
            "id": "union-test",
            "name": [
                {
                    "use": "official",
                    "family": "Smith",
                    "given": ["John", "William"]
                },
                {
                    "use": "usual",
                    "family": "Smith",
                    "given": ["Johnny"]
                }
            ],
            "telecom": [
                {"system": "phone", "value": "555-1234"},
                {"system": "email", "value": "john@example.com"}
            ]
        });

        let context = EvaluationContext::new(patient);

        // Test 1: Simple union of primitive values
        let expr = parser.parse("(1 | 2 | 3)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], FhirPathValue::Integer(1));
                assert_eq!(items[1], FhirPathValue::Integer(2));
                assert_eq!(items[2], FhirPathValue::Integer(3));
            }
            _ => panic!("Expected collection from union operation, got {result:?}"),
        }

        // Test 2: Union with strings
        let expr = parser.parse("('apple' | 'banana' | 'cherry')").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], FhirPathValue::String("apple".to_string()));
                assert_eq!(items[1], FhirPathValue::String("banana".to_string()));
                assert_eq!(items[2], FhirPathValue::String("cherry".to_string()));
            }
            _ => panic!("Expected collection from string union, got {result:?}"),
        }

        // Test 3: Union with FHIR data - combining given names and family names
        let expr = parser.parse("name.given | name.family").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                // Should contain all given names and family names
                assert!(items.len() >= 5); // At least John, William, Johnny, Smith, Smith
                                           // Verify some expected values are present
                assert!(items.contains(&FhirPathValue::String("John".to_string())));
                assert!(items.contains(&FhirPathValue::String("Smith".to_string())));
            }
            _ => panic!("Expected collection from FHIR union, got {result:?}"),
        }

        // Test 4: Union with indexing
        let expr = parser.parse("(10 | 20 | 30)[1]").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Integer(20) => {
                // Perfect - indexing works on union results
            }
            _ => panic!("Expected Integer(20) from union indexing, got {result:?}"),
        }

        // Test 5: Union with empty values
        let expr = parser.parse("(1 | {} | 3)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 2); // Should only have 1 and 3, empty is not added
                assert_eq!(items[0], FhirPathValue::Integer(1));
                assert_eq!(items[1], FhirPathValue::Integer(3));
            }
            _ => panic!("Expected collection from union with empty, got {result:?}"),
        }

        // Test 6: Union with mixed types
        let expr = parser.parse("(42 | 'hello' | true)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], FhirPathValue::Integer(42));
                assert_eq!(items[1], FhirPathValue::String("hello".to_string()));
                assert_eq!(items[2], FhirPathValue::Boolean(true));
            }
            _ => panic!("Expected collection from mixed type union, got {result:?}"),
        }

        // Test 7: Nested unions
        let expr = parser.parse("((1 | 2) | (3 | 4))").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 4);
                assert_eq!(items[0], FhirPathValue::Integer(1));
                assert_eq!(items[1], FhirPathValue::Integer(2));
                assert_eq!(items[2], FhirPathValue::Integer(3));
                assert_eq!(items[3], FhirPathValue::Integer(4));
            }
            _ => panic!("Expected collection from nested union, got {result:?}"),
        }

        // Test 8: Union with single values (should work but return single value)
        let expr = parser.parse("42 | 'single'").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();

        match result {
            FhirPathValue::Collection(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], FhirPathValue::Integer(42));
                assert_eq!(items[1], FhirPathValue::String("single".to_string()));
            }
            _ => panic!("Expected collection from single value union, got {result:?}"),
        }

        println!("Union operations test completed successfully!");
    }

    #[test]
    fn test_implies_operator() {
        let parser = FhirPathParser::new();
        let evaluator = FhirPathEvaluator::new();
        let context = EvaluationContext::new(json!({}));

        // Test basic implies truth table
        let expr = parser.parse("true implies true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(true)));

        let expr = parser.parse("true implies false").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(false)));

        let expr = parser.parse("false implies true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(true)));

        let expr = parser.parse("false implies false").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(true)));

        // Test with expressions
        let expr = parser.parse("(5 > 3) implies (2 < 4)").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(true)));

        // Test precedence: "true and false implies true" should be "(true and false) implies true" = "false implies true" = true
        let expr = parser.parse("true and false implies true").unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        assert!(matches!(result, FhirPathValue::Boolean(true)));

        println!("Implies operator test completed successfully!");
    }
}
