//! Tests for distinct() and isDistinct() functions
//!
//! Tests cover:
//! - distinct() function with various collections
//! - isDistinct() function validation
//! - Edge cases and empty collections
//! - Mixed data types

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_test_context() -> EvaluationContext {
    let patient_data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["John", "James", "John"],  // Contains duplicates
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
            {"system": "phone", "value": "555-1234"}  // Duplicate phone
        ]
    });
    EvaluationContext::new(patient_data)
}

#[test]
fn test_distinct_with_integer_duplicates() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Test distinct() with duplicates
    let expr = parser
        .parse("(1 | 2 | 2 | 3 | 1 | 4 | 3).distinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(4))));

        // Verify no duplicates remain
        let mut seen = std::collections::HashSet::new();
        for item in &items {
            if let FhirPathValue::Integer(n) = item {
                assert!(seen.insert(*n), "Found duplicate: {n}");
            }
        }
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_distinct_with_string_duplicates() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    let expr = parser
        .parse("('apple' | 'banana' | 'apple' | 'cherry' | 'banana').distinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "apple")));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "banana")));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "cherry")));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_distinct_with_mixed_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    let expr = parser
        .parse("(1 | 'hello' | true | 1 | 'hello' | false | true).distinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 4);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(1))));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::String(ref s) if s == "hello")));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::Boolean(true))));
        assert!(items
            .iter()
            .any(|v| matches!(v, FhirPathValue::Boolean(false))));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_distinct_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Empty collection
    let expr = parser.parse("{}.distinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Single item
    let expr = parser.parse("42.distinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    // Single item in collection (should return as single value)
    let expr = parser.parse("(42).distinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    // All identical items
    let expr = parser.parse("(5 | 5 | 5 | 5).distinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(5)));
}

#[test]
fn test_is_distinct_with_unique_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Collection with all unique items
    let expr = parser.parse("(1 | 2 | 3 | 4).isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Collection with unique strings
    let expr = parser
        .parse("('apple' | 'banana' | 'cherry').isDistinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Mixed unique types
    let expr = parser
        .parse("(1 | 'hello' | true | 2.5).isDistinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_is_distinct_with_duplicates() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Collection with duplicate integers
    let expr = parser.parse("(1 | 2 | 3 | 2).isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Collection with duplicate strings
    let expr = parser
        .parse("('apple' | 'banana' | 'apple').isDistinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Mixed types with duplicates
    let expr = parser
        .parse("(1 | 'hello' | true | 1).isDistinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));
}

#[test]
fn test_is_distinct_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Empty collection (should be considered distinct)
    let expr = parser.parse("{}.isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Single item (always distinct)
    let expr = parser.parse("42.isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Single item in collection
    let expr = parser.parse("(42).isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_distinct_functions_with_fhir_data() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Test distinct() on FHIR path with duplicates
    let expr = parser.parse("name.given.distinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        // Should remove duplicate "John" but keep "James" and "Jane"
        assert_eq!(items.len(), 3);
        let mut john_count = 0;
        let mut james_count = 0;
        let mut jane_count = 0;

        for item in &items {
            if let FhirPathValue::String(s) = item {
                match s.as_str() {
                    "John" => john_count += 1,
                    "James" => james_count += 1,
                    "Jane" => jane_count += 1,
                    _ => {}
                }
            }
        }

        assert_eq!(
            john_count, 1,
            "Should have exactly one John after distinct()"
        );
        assert_eq!(james_count, 1, "Should have exactly one James");
        assert_eq!(jane_count, 1, "Should have exactly one Jane");
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Test isDistinct() on FHIR path with duplicates (should be false)
    let expr = parser.parse("name.given.isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test isDistinct() on family names (should be true - no duplicates)
    let expr = parser.parse("name.family.isDistinct()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}

#[test]
fn test_combined_distinct_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_context();

    // Chain distinct() with other operations
    let expr = parser
        .parse("(1 | 2 | 2 | 3 | 1 | 4).distinct().count()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(4)));

    // Use distinct() before intersect
    let expr = parser
        .parse("(1 | 2 | 2 | 3 | 3).distinct().intersect(2 | 3 | 4)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(2))));
        assert!(items.iter().any(|v| matches!(v, FhirPathValue::Integer(3))));
    } else {
        panic!("Expected collection result, got: {result:?}");
    }

    // Check if result of distinct() is distinct
    let expr = parser
        .parse("(1 | 2 | 2 | 3 | 1).distinct().isDistinct()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));
}
