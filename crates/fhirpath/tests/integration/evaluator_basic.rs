//! Basic evaluator tests
//! 
//! Tests for fundamental evaluation functionality

use super::*;

#[test]
fn test_basic_evaluation() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();
    let context = EvaluationContext::new(patient);

    // Test basic path evaluation
    let expr = parser.parse("name").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    
    // The result should be a collection containing the name array
    match result {
        FhirPathValue::Collection(items) => {
            assert!(items.len() >= 1);
        }
        _ => panic!("Expected collection result"),
    }
    
    // Test property access
    let expr = parser.parse("resourceType").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(resource_type) = result {
        assert_eq!(resource_type, "Patient");
    } else {
        panic!("Expected string value for resourceType");
    }
}

#[test]
fn test_literal_values() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let patient = sample_patient();
    let context = EvaluationContext::new(patient);

    // Test boolean literals
    let expr = parser.parse("true").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    let expr = parser.parse("false").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(false)));

    // Test numeric literals
    let expr = parser.parse("42").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    // Test string literals
    let expr = parser.parse("'hello world'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "hello world");
    } else {
        panic!("Expected string value");
    }
}
