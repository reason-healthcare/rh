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

#[test]
fn test_children_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let tree_data = json!({
        "patient": {
            "name": "John Doe",
            "age": 30,
            "active": true,
            "nested": {
                "value": "test"
            }
        },
        "empty_object": {},
        "simple_value": "test"
    });

    let context = EvaluationContext::new(tree_data);

    // Test children() on object with properties
    let expr = parser.parse("patient.children()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // name, age, active, nested

        // Check that we have the expected values (order is undefined)
        let values: Vec<String> = items
            .iter()
            .map(|item| match item {
                FhirPathValue::String(s) => s.clone(),
                FhirPathValue::Integer(i) => i.to_string(),
                FhirPathValue::Boolean(b) => b.to_string(),
                FhirPathValue::Object(_) => "object".to_string(),
                _ => "unknown".to_string(),
            })
            .collect();

        assert!(values.contains(&"John Doe".to_string()));
        assert!(values.contains(&"30".to_string()));
        assert!(values.contains(&"true".to_string()));
        assert!(values.contains(&"object".to_string()));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test children() on empty object
    let expr = parser.parse("empty_object.children()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Test children() on primitive value (should return empty)
    let expr = parser.parse("simple_value.children()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Test children() on nested object
    let expr = parser.parse("patient.nested.children()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::String(s) => {
            assert_eq!(s, "test");
        }
        FhirPathValue::Collection(items) => {
            assert_eq!(items.len(), 1);
            if let FhirPathValue::String(s) = &items[0] {
                assert_eq!(s, "test");
            } else {
                panic!("Expected string in collection, got: {:?}", items[0]);
            }
        }
        _ => panic!("Expected string or collection result, got: {result:?}"),
    }

    // Test children() on collection of objects
    let collection_data = json!({
        "items": [
            {"a": 1, "b": 2},
            {"c": 3, "d": 4}
        ]
    });
    let collection_context = EvaluationContext::new(collection_data);

    let expr = parser.parse("items.children()").unwrap();
    let result = evaluator.evaluate(&expr, &collection_context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // 1, 2, 3, 4

        let values: Vec<i64> = items
            .iter()
            .map(|item| match item {
                FhirPathValue::Integer(i) => *i,
                _ => panic!("Expected integer"),
            })
            .collect();

        // Check all values are present (order undefined)
        assert!(values.contains(&1));
        assert!(values.contains(&2));
        assert!(values.contains(&3));
        assert!(values.contains(&4));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_descendants_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let tree_data = json!({
        "patient": {
            "name": {
                "given": ["John", "William"],
                "family": "Doe"
            },
            "age": 30,
            "active": true,
            "address": {
                "line": ["123 Main St", "Apt 4B"],
                "city": "Springfield",
                "state": "IL",
                "postalCode": "62701",
                "country": {
                    "code": "US",
                    "display": "United States"
                }
            }
        },
        "empty_object": {},
        "simple_value": "test"
    });

    let context = EvaluationContext::new(tree_data);

    // Test descendants() on complex nested object
    let expr = parser.parse("patient.descendants()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        // Should contain all descendants: name object, given array, family string, age, active, address object,
        // line array, city, state, postalCode, country object, code, display, plus all array elements
        assert!(
            items.len() >= 10,
            "Expected at least 10 descendants, got {}",
            items.len()
        );

        // Check that we have some key expected values
        let contains_john = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "John"));
        let contains_william = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "William"));
        let contains_doe = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "Doe"));
        let contains_30 = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::Integer(i) if *i == 30));
        let contains_true = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::Boolean(b) if *b));
        let contains_springfield = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "Springfield"));
        let contains_us = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "US"));

        assert!(contains_john, "Should contain 'John' from nested array");
        assert!(
            contains_william,
            "Should contain 'William' from nested array"
        );
        assert!(contains_doe, "Should contain 'Doe' from nested object");
        assert!(contains_30, "Should contain age 30");
        assert!(contains_true, "Should contain active true");
        assert!(
            contains_springfield,
            "Should contain 'Springfield' from deeply nested object"
        );
        assert!(contains_us, "Should contain 'US' from deeply nested object");
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test descendants() on empty object
    let expr = parser.parse("empty_object.descendants()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Test descendants() on primitive value (should return empty)
    let expr = parser.parse("simple_value.descendants()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Test descendants() on collection of objects
    let collection_data = json!({
        "items": [
            {
                "name": "Item1",
                "details": {
                    "category": "A",
                    "price": 10
                }
            },
            {
                "name": "Item2",
                "details": {
                    "category": "B",
                    "price": 20
                }
            }
        ]
    });

    let context = EvaluationContext::new(collection_data);
    let expr = parser.parse("items.descendants()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        // Should contain all descendants from both items in the collection
        assert!(items.len() >= 8); // At least 8 descendants

        let contains_item1 = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "Item1"));
        let contains_item2 = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "Item2"));
        let contains_a = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "A"));
        let contains_b = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::String(s) if s == "B"));
        let contains_10 = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::Integer(i) if *i == 10));
        let contains_20 = items
            .iter()
            .any(|item| matches!(item, FhirPathValue::Integer(i) if *i == 20));

        assert!(contains_item1, "Should contain 'Item1'");
        assert!(contains_item2, "Should contain 'Item2'");
        assert!(contains_a, "Should contain category 'A'");
        assert!(contains_b, "Should contain category 'B'");
        assert!(contains_10, "Should contain price 10");
        assert!(contains_20, "Should contain price 20");
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}
