//! combine Function Tests
//!
//! Comprehensive tests for the combine(other) function that merges collections.
//! The combine function merges the input and other collections into a single collection
//! without eliminating duplicate values, maintaining no specific order.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

/// Create a test context with various collections for combining
fn create_combine_test_context() -> EvaluationContext {
    let data = json!({
        "numbers1": [1, 2, 3],
        "numbers2": [3, 4, 5],
        "strings1": ["apple", "banana"],
        "strings2": ["banana", "cherry", "date"],
        "mixed1": [1, "hello", true],
        "mixed2": [false, "world", 2],
        "empty": [],
        "single": ["lonely"],
        "resources1": [
            {"resourceType": "Patient", "id": "p1"},
            {"resourceType": "Patient", "id": "p2"}
        ],
        "resources2": [
            {"resourceType": "Patient", "id": "p2"},
            {"resourceType": "Observation", "id": "o1"}
        ],
        "duplicates": [1, 1, 2, 2, 3]
    });

    EvaluationContext::new(data)
}

#[test]
fn test_combine_basic_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine two number collections
    let expr = parser.parse("numbers1.combine(numbers2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 6); // [1, 2, 3, 3, 4, 5]
                                    // Should contain all items from both collections, including duplicate 3
        let mut found_first_three = false;
        let mut found_second_three = false;
        for item in &items {
            if let FhirPathValue::Integer(3) = item {
                if !found_first_three {
                    found_first_three = true;
                } else {
                    found_second_three = true;
                }
            }
        }
        assert!(
            found_first_three && found_second_three,
            "Should preserve duplicate 3 from both collections"
        );
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_string_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine string collections with overlap
    let expr = parser.parse("strings1.combine(strings2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 5); // ["apple", "banana", "banana", "cherry", "date"]
                                    // Should contain duplicate "banana"
        let banana_count = items
            .iter()
            .filter(|item| matches!(item, FhirPathValue::String(s) if s == "banana"))
            .count();
        assert_eq!(
            banana_count, 2,
            "Should preserve duplicate 'banana' from both collections"
        );
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_mixed_type_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine mixed type collections
    let expr = parser.parse("mixed1.combine(mixed2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 6); // [1, "hello", true, false, "world", 2]

        // Check we have the expected types
        let mut has_integer = false;
        let mut has_string = false;
        let mut has_boolean = false;

        for item in items {
            match item {
                FhirPathValue::Integer(_) => has_integer = true,
                FhirPathValue::String(_) => has_string = true,
                FhirPathValue::Boolean(_) => has_boolean = true,
                _ => {}
            }
        }

        assert!(
            has_integer && has_string && has_boolean,
            "Should preserve all types"
        );
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_with_empty_collection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine non-empty with empty - should return non-empty
    let expr = parser.parse("numbers1.combine(empty)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3); // [1, 2, 3]
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[1], FhirPathValue::Integer(2)));
        assert!(matches!(items[2], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }

    // Combine empty with non-empty - should return non-empty
    let expr = parser.parse("empty.combine(numbers1)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3); // [1, 2, 3]
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[1], FhirPathValue::Integer(2)));
        assert!(matches!(items[2], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_both_empty_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine two empty collections
    let expr = parser.parse("empty.combine(empty)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_combine_single_item_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine single item with another collection
    let expr = parser.parse("single.combine(numbers1)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // ["lonely", 1, 2, 3]
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "lonely"));
        assert!(matches!(items[1], FhirPathValue::Integer(1)));
        assert!(matches!(items[2], FhirPathValue::Integer(2)));
        assert!(matches!(items[3], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_resource_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine resource collections with duplicate patient
    let expr = parser.parse("resources1.combine(resources2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4);
        // Should have: Patient p1, Patient p2, Patient p2 (duplicate), Observation o1

        let patient_count = items
            .iter()
            .filter(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("resourceType").and_then(|v| v.as_str()) == Some("Patient")
                } else {
                    false
                }
            })
            .count();

        let observation_count = items
            .iter()
            .filter(|item| {
                if let FhirPathValue::Object(obj) = item {
                    obj.get("resourceType").and_then(|v| v.as_str()) == Some("Observation")
                } else {
                    false
                }
            })
            .count();

        assert_eq!(
            patient_count, 3,
            "Should have 3 Patient resources (including duplicate)"
        );
        assert_eq!(observation_count, 1, "Should have 1 Observation resource");
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_preserves_duplicates() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine collections that both have internal duplicates
    let expr = parser.parse("duplicates.combine(duplicates)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 10); // [1, 1, 2, 2, 3] + [1, 1, 2, 2, 3] = 10 items

        // Count occurrences of each value
        let count_1 = items
            .iter()
            .filter(|item| matches!(item, FhirPathValue::Integer(1)))
            .count();
        let count_2 = items
            .iter()
            .filter(|item| matches!(item, FhirPathValue::Integer(2)))
            .count();
        let count_3 = items
            .iter()
            .filter(|item| matches!(item, FhirPathValue::Integer(3)))
            .count();

        assert_eq!(count_1, 4, "Should have 4 occurrences of 1");
        assert_eq!(count_2, 4, "Should have 4 occurrences of 2");
        assert_eq!(count_3, 2, "Should have 2 occurrences of 3");
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_chained_with_other_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Chain combine with count
    let expr = parser.parse("numbers1.combine(numbers2).count()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Integer(count) = result {
        assert_eq!(count, 6);
    } else {
        panic!("Expected integer result");
    }

    // Chain combine with distinct to remove duplicates
    let expr = parser
        .parse("numbers1.combine(numbers2).distinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 5); // [1, 2, 3, 4, 5] after removing duplicate 3
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_order_independence() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Test that A.combine(B) and B.combine(A) both work (order may differ but content should be equivalent)
    let expr1 = parser.parse("numbers1.combine(numbers2).count()").unwrap();
    let result1 = evaluator.evaluate(&expr1, &context).unwrap();

    let expr2 = parser.parse("numbers2.combine(numbers1).count()").unwrap();
    let result2 = evaluator.evaluate(&expr2, &context).unwrap();

    // Both should have same count
    assert_eq!(result1, result2);
}

#[test]
fn test_combine_single_item_target() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine single item with collection
    let expr = parser.parse("numbers1[0].combine(numbers2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // [1, 3, 4, 5] - single 1 plus numbers2
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_single_item_parameter() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine collection with single item
    let expr = parser.parse("numbers1.combine(numbers2[0])").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // [1, 2, 3, 3] - numbers1 plus single 3
        assert!(matches!(items[3], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_combine_two_single_items() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_combine_test_context();

    // Combine two single items
    let expr = parser.parse("numbers1[0].combine(numbers2[0])").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2); // [1, 3]
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[1], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }
}
