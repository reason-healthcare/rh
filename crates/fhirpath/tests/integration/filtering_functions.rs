//! Filtering function tests
//! 
//! Tests for the where() and select() filtering functions

use super::*;

#[test]
fn test_where_function_basic() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test where with property equality
    let result = evaluator.evaluate("name.where(family = 'Doe')", &patient).unwrap();
    assert!(result.len() <= 1); // Should find at most one matching name
    
    // Test where with boolean property
    let result = evaluator.evaluate("name.where(true)", &patient).unwrap();
    assert_eq!(result.len(), 1); // Should return the name since condition is always true
}

#[test]
fn test_where_function_with_literals() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test where on literal collection with numeric condition
    let result = evaluator.evaluate("(1 | 2 | 3 | 4 | 5).where($this > 3)", &patient).unwrap();
    assert_eq!(result.len(), 2); // Should return [4, 5]
    
    for value in &result {
        if let Value::Integer(i) = value {
            assert!(*i > 3);
        } else {
            panic!("Expected integer value in where result");
        }
    }

    // Test where with equality condition
    let result = evaluator.evaluate("(1 | 2 | 3 | 2 | 1).where($this = 2)", &patient).unwrap();
    assert_eq!(result.len(), 2); // Should return both 2s
    
    for value in &result {
        if let Value::Integer(i) = value {
            assert_eq!(*i, 2);
        } else {
            panic!("Expected integer value in where result");
        }
    }
}

#[test]
fn test_where_function_complex_conditions() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test where with range condition
    let result = evaluator.evaluate("(1 | 2 | 3 | 4 | 5).where($this >= 2 and $this <= 4)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should return [2, 3, 4]
    
    for value in &result {
        if let Value::Integer(i) = value {
            assert!(*i >= 2 && *i <= 4);
        } else {
            panic!("Expected integer value in where result");
        }
    }

    // Test where with modulo condition
    let result = evaluator.evaluate("(1 | 2 | 3 | 4 | 5 | 6).where($this mod 2 = 0)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should return [2, 4, 6]
    
    for value in &result {
        if let Value::Integer(i) = value {
            assert_eq!(*i % 2, 0);
        } else {
            panic!("Expected integer value in where result");
        }
    }
}

#[test]
fn test_where_function_empty_results() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test where with condition that matches nothing
    let result = evaluator.evaluate("(1 | 2 | 3).where($this > 10)", &patient).unwrap();
    assert_eq!(result.len(), 0);

    // Test where on empty collection
    let result = evaluator.evaluate("{}.where($this = 1)", &patient).unwrap();
    assert_eq!(result.len(), 0);

    // Test where with false condition
    let result = evaluator.evaluate("(1 | 2 | 3).where(false)", &patient).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_select_function_basic() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test select with property access
    let result = evaluator.evaluate("name.select(family)", &patient).unwrap();
    assert_eq!(result.len(), 1);
    if let Value::String(family) = &result[0] {
        assert_eq!(family, "Doe");
    } else {
        panic!("Expected string value from select");
    }

    // Test select with given names
    let result = evaluator.evaluate("name.select(given)", &patient).unwrap();
    assert!(result.len() >= 1); // Should have at least one given name
}

#[test]
fn test_select_function_with_literals() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test select with arithmetic transformation
    let result = evaluator.evaluate("(1 | 2 | 3).select($this * 2)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should return [2, 4, 6]
    
    let expected_values = vec![2, 4, 6];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in select result");
        }
    }

    // Test select with string transformation
    let result = evaluator.evaluate("(1 | 2 | 3).select($this + 10)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should return [11, 12, 13]
    
    let expected_values = vec![11, 12, 13];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in select result");
        }
    }
}

#[test]
fn test_select_function_complex_expressions() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test select with complex arithmetic
    let result = evaluator.evaluate("(1 | 2 | 3).select($this * $this + 1)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should return [2, 5, 10] (1^2+1, 2^2+1, 3^2+1)
    
    let expected_values = vec![2, 5, 10];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in select result");
        }
    }

    // Test select with conditional expression
    let result = evaluator.evaluate("(1 | 2 | 3 | 4).select(iif($this mod 2 = 0, $this * 10, $this))", &patient).unwrap();
    assert_eq!(result.len(), 4); // Should return [1, 20, 3, 40]
    
    let expected_values = vec![1, 20, 3, 40];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in select result");
        }
    }
}

#[test]
fn test_select_function_empty_results() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test select on empty collection
    let result = evaluator.evaluate("{}.select($this * 2)", &patient).unwrap();
    assert_eq!(result.len(), 0);

    // Test select that might return empty values
    let result = evaluator.evaluate("name.select(nonexistent)", &patient).unwrap();
    // This should return empty collection since 'nonexistent' property doesn't exist
    assert_eq!(result.len(), 0);
}

#[test]
fn test_chained_where_and_select() {
    let mut evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();

    // Test chaining where and select
    let result = evaluator.evaluate("(1 | 2 | 3 | 4 | 5).where($this > 2).select($this * 2)", &patient).unwrap();
    assert_eq!(result.len(), 3); // Should filter to [3, 4, 5] then transform to [6, 8, 10]
    
    let expected_values = vec![6, 8, 10];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in chained result");
        }
    }

    // Test chaining select and where
    let result = evaluator.evaluate("(1 | 2 | 3 | 4).select($this * 2).where($this > 4)", &patient).unwrap();
    assert_eq!(result.len(), 2); // Should transform to [2, 4, 6, 8] then filter to [6, 8]
    
    let expected_values = vec![6, 8];
    for (i, value) in result.iter().enumerate() {
        if let Value::Integer(val) = value {
            assert_eq!(*val, expected_values[i]);
        } else {
            panic!("Expected integer value in chained result");
        }
    }
}
