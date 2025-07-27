//! Comprehensive collection functions tests
//!
//! Tests for advanced collection operations including:
//! - skip() and take() with various parameters
//! - intersect() and exclude() operations  
//! - Complex collection transformations
//! - Edge cases and error conditions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_test_context() -> EvaluationContext {
    let patient_data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["John", "James"],
                "family": "Doe"
            },
            {
                "given": ["Jane"],
                "family": "Smith"
            }
        ],
        "telecom": [
            {"system": "phone", "value": "555-1234"},
            {"system": "email", "value": "john@example.com"},
            {"system": "phone", "value": "555-5678"}
        ]
    });
    EvaluationContext::new(patient_data)
}

#[test]
fn test_skip_with_various_counts() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Skip 0 elements (should return original collection)
    let expr = parser.parse("(1 | 2 | 3 | 4 | 5).skip(0)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 5);
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[4], FhirPathValue::Integer(5)));
    } else {
        panic!("Expected collection result");
    }

    // Skip 2 elements
    let expr = parser.parse("(1 | 2 | 3 | 4 | 5).skip(2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::Integer(3)));
        assert!(matches!(items[2], FhirPathValue::Integer(5)));
    } else {
        panic!("Expected collection result");
    }

    // Skip more elements than collection size
    let expr = parser.parse("(1 | 2 | 3).skip(10)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Skip negative count (should return error)
    let expr = parser.parse("(1 | 2 | 3).skip(-1)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("skip() count cannot be negative"));
}

#[test]
fn test_take_with_various_counts() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Take 0 elements
    let expr = parser.parse("(1 | 2 | 3 | 4 | 5).take(0)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Take 3 elements
    let expr = parser.parse("(1 | 2 | 3 | 4 | 5).take(3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[2], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }

    // Take more elements than collection size
    let expr = parser.parse("(1 | 2 | 3).take(10)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[2], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }

    // Take negative count (should return error)
    let expr = parser.parse("(1 | 2 | 3).take(-1)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("take() count cannot be negative"));
}

#[test]
fn test_intersect_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Intersect with common elements
    let expr = parser
        .parse("(1 | 2 | 3 | 4).intersect(2 | 3 | 5 | 6)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
    } else {
        panic!("Expected collection result");
    }

    // Intersect with no common elements
    let expr = parser.parse("(1 | 2 | 3).intersect(4 | 5 | 6)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Intersect with empty collection
    let expr = parser.parse("(1 | 2 | 3).intersect({})").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Intersect with strings (single result should be returned as single value)
    let expr = parser
        .parse("('apple' | 'banana' | 'cherry').intersect('banana' | 'date' | 'elderberry')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::String(ref s) if s == "banana"));
}

#[test]
fn test_exclude_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Exclude some elements
    let expr = parser.parse("(1 | 2 | 3 | 4 | 5).exclude(2 | 4)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(5))));
        assert!(!items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(!items.iter().any(|v| matches!(v, FhirPathValue::Integer(4))));
    } else {
        panic!("Expected collection result");
    }

    // Exclude non-existent elements
    let expr = parser.parse("(1 | 2 | 3).exclude(4 | 5 | 6)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
    } else {
        panic!("Expected collection result");
    }

    // Exclude all elements
    let expr = parser.parse("(1 | 2 | 3).exclude(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Exclude from empty collection
    let expr = parser.parse("{}.exclude(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_combined_collection_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Chain skip and take
    let expr = parser
        .parse("(1 | 2 | 3 | 4 | 5 | 6 | 7 | 8).skip(2).take(3)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::Integer(3)));
        assert!(matches!(items[1], FhirPathValue::Integer(4)));
        assert!(matches!(items[2], FhirPathValue::Integer(5)));
    } else {
        panic!("Expected collection result");
    }

    // Distinct then intersect
    let expr = parser
        .parse("(1 | 2 | 2 | 3 | 3 | 4).distinct().intersect(2 | 3 | 5)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
    } else {
        panic!("Expected collection result");
    }

    // First after exclude
    let expr = parser
        .parse("(1 | 2 | 3 | 4 | 5).exclude(1 | 2).first()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(3)));

    // Count after multiple operations
    let expr = parser
        .parse("(1 | 2 | 3 | 4 | 5 | 6).skip(1).take(4).exclude(3).count()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(3)));
}

#[test]
fn test_collection_functions_with_mixed_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Intersect with mixed types (should only match exact type and value)
    let expr = parser
        .parse("(1 | 'hello' | true | 2).intersect('hello' | 1 | false)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "hello")));
    } else {
        panic!("Expected collection result");
    }

    // Exclude with mixed types
    let expr = parser
        .parse("(1 | 'test' | true | 2.5).exclude('test' | true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Number(_))));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_collection_functions_with_fhir_data() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Test with actual FHIR paths
    let expr = parser.parse("name.given.skip(1).take(2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        // Should skip first "John" and take "James" and "Jane"
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection result");
    }

    // Test exclude with telecom systems
    let expr = parser.parse("telecom.system.exclude('email')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        // Should have 2 phone systems, no email
        assert_eq!(items.len(), 2);
        for item in items {
            if let FhirPathValue::String(s) = item {
                assert_eq!(s, "phone");
            }
        }
    } else {
        panic!("Expected collection result");
    }

    // Test intersect with given names
    let expr = parser
        .parse("name.given.intersect('John' | 'Mary' | 'Jane')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        // Should find "John" and "Jane"
        assert_eq!(items.len(), 2);
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "John")));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "Jane")));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_edge_cases_and_error_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Skip/take on empty collection
    let expr = parser.parse("{}.skip(5)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    let expr = parser.parse("{}.take(5)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Intersect/exclude on single value (not collection)
    let expr = parser.parse("42.intersect(42 | 43)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    let expr = parser.parse("42.exclude(42)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Large skip/take values
    let expr = parser.parse("(1 | 2 | 3).skip(1000000)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    let expr = parser.parse("(1 | 2 | 3).take(1000000)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
    } else {
        panic!("Expected collection result");
    }
}
