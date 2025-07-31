//! Union Function Tests
//!
//! Comprehensive tests for the union(other) function that merges collections while removing duplicates.
//! The union function merges the input and other collections into a single collection
//! while eliminating duplicate values, like combine() but with deduplication.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

/// Create a test context with various collections for union operations
fn create_union_test_context() -> EvaluationContext {
    let data = json!({
        "numbers1": [1, 2, 3],
        "numbers2": [3, 4, 5],
        "duplicates1": [1, 2, 2, 3],
        "duplicates2": [2, 3, 3, 4],
        "strings1": ["apple", "banana"],
        "strings2": ["banana", "cherry", "date"],
        "mixed1": [1, "hello", true],
        "mixed2": [false, "world", 2],
        "empty": [],
        "single": ["lonely"],
        "value1": "test",
        "value2": "test",
        "value3": "different"
    });
    EvaluationContext::new(data)
}

#[test]
fn test_union_basic_collections() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test union of two different arrays without overlap
    let ast = parser
        .parse("strings1.union(empty)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    // Should have 2 items (strings1 content, empty adds nothing)
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_union_with_duplicates() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test union removing duplicates
    let ast = parser
        .parse("duplicates1.union(duplicates2)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    // duplicates1: [1, 2, 2, 3], duplicates2: [2, 3, 3, 4]
    // Union should be: [1, 2, 3, 4] (4 unique items)
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4);

        // Check that we have exactly the unique values 1, 2, 3, 4
        let numbers: Vec<i64> = items
            .iter()
            .map(|v| match v {
                FhirPathValue::Integer(n) => *n,
                _ => panic!("Expected integer value"),
            })
            .collect();

        for expected in 1..=4 {
            assert_eq!(
                numbers.iter().filter(|&&n| n == expected).count(),
                1,
                "Number {expected} should appear exactly once"
            );
        }
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_union_vs_combine_difference() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test combine (preserves duplicates)
    let combine_ast = parser
        .parse("numbers1.combine(numbers2)")
        .expect("Failed to parse expression");
    let combine_result = evaluator
        .evaluate(&combine_ast, &context)
        .expect("Failed to evaluate");

    // Test union (removes duplicates)
    let union_ast = parser
        .parse("numbers1.union(numbers2)")
        .expect("Failed to parse expression");
    let union_result = evaluator
        .evaluate(&union_ast, &context)
        .expect("Failed to evaluate");

    // numbers1: [1, 2, 3], numbers2: [3, 4, 5]
    // Combine should have 6 items: [1, 2, 3, 3, 4, 5]
    if let FhirPathValue::Collection(combine_items) = combine_result {
        assert_eq!(combine_items.len(), 6);
    } else {
        panic!("Expected collection result for combine");
    }

    // Union should have 5 items: [1, 2, 3, 4, 5]
    if let FhirPathValue::Collection(union_items) = union_result {
        assert_eq!(union_items.len(), 5);
    } else {
        panic!("Expected collection result for union");
    }
}

#[test]
fn test_union_with_empty_collections() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Union with empty collection should return original collection
    let ast1 = parser
        .parse("numbers1.union(empty)")
        .expect("Failed to parse expression");
    let result1 = evaluator
        .evaluate(&ast1, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result1 {
        assert_eq!(items.len(), 3);
    } else {
        panic!("Expected collection result");
    }

    // Empty collection union with non-empty should return the non-empty
    let ast2 = parser
        .parse("empty.union(numbers1)")
        .expect("Failed to parse expression");
    let result2 = evaluator
        .evaluate(&ast2, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result2 {
        assert_eq!(items.len(), 3);
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_union_single_items() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test union of single items (not collections)
    let ast = parser
        .parse("value1.union(value3)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_union_identical_single_items() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test union of identical single items should deduplicate
    let ast = parser
        .parse("value1.union(value2)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    // Should return single item since they're identical
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "test");
    } else {
        panic!("Expected single string result, got: {result:?}");
    }
}

#[test]
fn test_union_mixed_types() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let ast = parser
        .parse("mixed1.union(mixed2)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 6); // All items are different types so no deduplication
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_union_with_complex_objects() {
    let patient_data = json!({
        "resourceType": "Patient",
        "name": [
            {"family": "Smith", "given": ["John"]},
            {"family": "Smith", "given": ["John"]} // Duplicate name
        ],
        "address": [
            {"city": "Boston", "state": "MA"}
        ]
    });
    let context = EvaluationContext::new(patient_data);
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let ast = parser
        .parse("name.union(address)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result {
        // Should have 2 items: 1 unique name + 1 address (duplicate name removed)
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_union_chained_operations() {
    let chained_data = json!({
        "collection1": [1, 2],
        "collection2": [2, 3],
        "collection3": [3, 4]
    });
    let context = EvaluationContext::new(chained_data);
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test chaining: collection1.union(collection2).union(collection3)
    let ast = parser
        .parse("collection1.union(collection2).union(collection3)")
        .expect("Failed to parse expression");
    let result = evaluator
        .evaluate(&ast, &context)
        .expect("Failed to evaluate");

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4); // [1, 2, 3, 4]
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_union_parameter_validation() {
    let context = create_union_test_context();
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with no parameters (should fail)
    let ast = parser
        .parse("numbers1.union()")
        .expect("Failed to parse expression");
    let result = evaluator.evaluate(&ast, &context);
    assert!(result.is_err(), "union() with no parameters should fail");

    // Test with too many parameters (should fail)
    let ast2 = parser
        .parse("numbers1.union(numbers2, strings1)")
        .expect("Failed to parse expression");
    let result2 = evaluator.evaluate(&ast2, &context);
    assert!(
        result2.is_err(),
        "union() with too many parameters should fail"
    );
}
