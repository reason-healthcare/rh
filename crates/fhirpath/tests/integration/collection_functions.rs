//! Collection function tests
//! 
//! Tests for built-in collection functions like count(), exists(), empty(), etc.

use super::*;

#[test]
fn test_count_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test count on collection
    let result = evaluator.evaluate("name.count()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(count) = &result[0] {
        assert_eq!(*count, 1);
    } else {
        panic!("Expected integer value for count, got {:?}", result[0]);
    }

    // Test count on literal collection
    let result = evaluator.evaluate("(1 | 2 | 3).count()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(count) = &result[0] {
        assert_eq!(*count, 3);
    } else {
        panic!("Expected integer value for count, got {:?}", result[0]);
    }

    // Test count on empty collection
    let result = evaluator.evaluate("{}.count()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(count) = &result[0] {
        assert_eq!(*count, 0);
    } else {
        panic!("Expected integer value for count, got {:?}", result[0]);
    }
}

#[test]
fn test_exists_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test exists on non-empty collection
    let result = evaluator.evaluate("name.exists()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // Test exists on empty collection
    let result = evaluator.evaluate("nonexistent.exists()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test exists on literal collection
    let result = evaluator.evaluate("(1 | 2 | 3).exists()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // Test exists on empty literal collection
    let result = evaluator.evaluate("{}.exists()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));
}

#[test]
fn test_empty_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test empty on non-empty collection
    let result = evaluator.evaluate("name.empty()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test empty on empty collection
    let result = evaluator.evaluate("nonexistent.empty()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // Test empty on literal collection
    let result = evaluator.evaluate("(1 | 2 | 3).empty()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test empty on empty literal collection
    let result = evaluator.evaluate("{}.empty()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));
}

#[test]
fn test_distinct_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test distinct on collection with duplicates
    let result = evaluator.evaluate("(1 | 2 | 2 | 3 | 1).distinct()", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should be [1, 2, 3]
    
    // Verify we have the expected unique values
    let mut found_values = std::collections::HashSet::new();
    for value in &result {
        if let Value::Integer(i) = value {
            found_values.insert(*i);
        }
    }
    assert!(found_values.contains(&1));
    assert!(found_values.contains(&2));
    assert!(found_values.contains(&3));

    // Test distinct on collection without duplicates
    let result = evaluator.evaluate("(1 | 2 | 3).distinct()", &patient).unwrap();
    assert_eq!(result.len(), 3);

    // Test distinct on empty collection
    let result = evaluator.evaluate("{}.distinct()", &patient).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_is_distinct_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test isDistinct on collection with duplicates
    let result = evaluator.evaluate("(1 | 2 | 2 | 3).isDistinct()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test isDistinct on collection without duplicates
    let result = evaluator.evaluate("(1 | 2 | 3).isDistinct()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // Test isDistinct on empty collection
    let result = evaluator.evaluate("{}.isDistinct()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // Test isDistinct on single element collection
    let result = evaluator.evaluate("(42).isDistinct()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));
}

#[test]
fn test_first_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test first on collection
    let result = evaluator.evaluate("(10 | 20 | 30).first()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(first) = &result[0] {
        assert_eq!(*first, 10);
    } else {
        panic!("Expected integer value for first, got {:?}", result[0]);
    }

    // Test first on empty collection
    let result = evaluator.evaluate("{}.first()", &patient).unwrap();
    assert_eq!(result.len(), 0);

    // Test first on single element collection
    let result = evaluator.evaluate("(42).first()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(first) = &result[0] {
        assert_eq!(*first, 42);
    } else {
        panic!("Expected integer value for first, got {:?}", result[0]);
    }
}

#[test]
fn test_last_function() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test last on collection
    let result = evaluator.evaluate("(10 | 20 | 30).last()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(last) = &result[0] {
        assert_eq!(*last, 30);
    } else {
        panic!("Expected integer value for last, got {:?}", result[0]);
    }

    // Test last on empty collection
    let result = evaluator.evaluate("{}.last()", &patient).unwrap();
    assert_eq!(result.len(), 0);

    // Test last on single element collection
    let result = evaluator.evaluate("(42).last()", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::Integer(last) = &result[0] {
        assert_eq!(*last, 42);
    } else {
        panic!("Expected integer value for last, got {:?}", result[0]);
    }
}
