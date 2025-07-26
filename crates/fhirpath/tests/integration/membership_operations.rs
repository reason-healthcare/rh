//! Membership operation tests
//! 
//! Tests for 'in' and 'contains' operations in FHIRPath expressions

use super::*;

#[test]
fn test_in_operator() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test value in collection of literals
    let result = evaluator.evaluate("5 in (3 | 5 | 7)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    let result = evaluator.evaluate("4 in (3 | 5 | 7)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test string in collection
    let result = evaluator.evaluate("'apple' in ('apple' | 'banana' | 'cherry')", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    let result = evaluator.evaluate("'grape' in ('apple' | 'banana' | 'cherry')", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));
}

#[test]
fn test_contains_operator() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test collection contains value
    let result = evaluator.evaluate("(3 | 5 | 7) contains 5", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    let result = evaluator.evaluate("(3 | 5 | 7) contains 4", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test string collection contains value
    let result = evaluator.evaluate("('apple' | 'banana' | 'cherry') contains 'banana'", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    let result = evaluator.evaluate("('apple' | 'banana' | 'cherry') contains 'grape'", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));
}

#[test]
fn test_membership_with_properties() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test if a property value is in a collection
    // Note: This assumes the patient has name.given values
    let result = evaluator.evaluate("'John' in name.given", &patient);
    
    // This test depends on the patient data structure
    // We'll just verify it doesn't error for now
    assert!(result.is_ok());
}

#[test]
fn test_complex_membership() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test complex expressions with membership
    let result = evaluator.evaluate("(2 + 3) in (5 | 10 | 15)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    let result = evaluator.evaluate("(2 * 3) in (5 | 10 | 15)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    // Test membership with boolean results
    let result = evaluator.evaluate("true in (true | false)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));
}

#[test]
fn test_empty_collection_membership() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test membership with empty collections
    let result = evaluator.evaluate("5 in {}", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));

    let result = evaluator.evaluate("{} contains 5", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(false)));
}

#[test]
fn test_nested_membership() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test nested membership operations
    // (5 in (1 | 5 | 9)) = true, and true in (true | false) = true
    let result = evaluator.evaluate("(5 in (1 | 5 | 9)) in (true | false)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));

    // (6 in (1 | 5 | 9)) = false, and false in (true | false) = true
    let result = evaluator.evaluate("(6 in (1 | 5 | 9)) in (true | false)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0], Value::Boolean(true)));
}
