//! Subset and Superset Operations Tests
//!
//! Tests for subsetOf() and supersetOf() collection functions including:
//! - Basic subset/superset relationships
//! - Empty collection handling
//! - Mixed type collections
//! - FHIR-specific use cases
//! - Edge cases and error conditions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_test_data() -> EvaluationContext {
    let data = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient1",
                    "name": [
                        {
                            "use": "official",
                            "family": "Smith",
                            "given": ["John", "James"]
                        }
                    ],
                    "telecom": [
                        {"system": "phone", "value": "555-1234"},
                        {"system": "email", "value": "john@example.com"}
                    ],
                    "status": ["active", "verified"],
                    "categories": ["outpatient", "emergency", "inpatient"]
                }
            }
        ]
    });
    EvaluationContext::new(data)
}

#[test]
fn test_subset_of_basic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test basic subset relationship
    let expr = parser.parse("(1 | 2).subsetOf(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for subset relationship"),
    }

    // Test non-subset relationship
    let expr = parser.parse("(1 | 4).subsetOf(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false for non-subset relationship"),
    }

    // Test equal sets (subset of itself)
    let expr = parser.parse("(1 | 2 | 3).subsetOf(3 | 2 | 1)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for equal sets"),
    }
}

#[test]
fn test_superset_of_basic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test basic superset relationship
    let expr = parser.parse("(1 | 2 | 3).supersetOf(1 | 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for superset relationship"),
    }

    // Test non-superset relationship
    let expr = parser.parse("(1 | 2).supersetOf(1 | 4)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false for non-superset relationship"),
    }

    // Test equal sets (superset of itself)
    let expr = parser.parse("(1 | 2 | 3).supersetOf(3 | 2 | 1)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for equal sets"),
    }
}

#[test]
fn test_empty_collection_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Empty set is subset of any set
    let expr = parser.parse("{}.subsetOf(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - empty set is subset of any set"),
    }

    // Empty set is subset of empty set
    let expr = parser.parse("{}.subsetOf({})").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - empty set is subset of empty set"),
    }

    // Any set is superset of empty set
    let expr = parser.parse("(1 | 2 | 3).supersetOf({})").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - any set is superset of empty set"),
    }

    // Empty set is superset of empty set
    let expr = parser.parse("{}.supersetOf({})").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - empty set is superset of empty set"),
    }
}

#[test]
fn test_string_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test string subset
    let expr = parser
        .parse("('apple' | 'banana').subsetOf('apple' | 'banana' | 'cherry')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for string subset"),
    }

    // Test string superset
    let expr = parser
        .parse("('apple' | 'banana' | 'cherry').supersetOf('apple' | 'banana')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for string superset"),
    }

    // Test case sensitivity
    let expr = parser
        .parse("('Apple').subsetOf('apple' | 'APPLE')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false - case sensitive comparison"),
    }
}

#[test]
fn test_mixed_type_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test mixed types - numbers should work
    let expr = parser.parse("(1 | 2.0).subsetOf(1 | 2 | 3.0)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for mixed numeric types"),
    }

    // Test mixed types with strings and numbers
    let expr = parser
        .parse("(1 | 'test').subsetOf(1 | 'test' | true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for mixed types"),
    }

    // Test type mismatch
    let expr = parser.parse("('1').subsetOf(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false - string '1' != integer 1"),
    }
}

#[test]
fn test_fhir_specific_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Test FHIR telecom systems subset
    let expr = parser.parse("entry.resource.telecom.system.where($this in ('phone')).subsetOf(entry.resource.telecom.system)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - phone systems are subset of all systems"),
    }

    // Test status categories
    let expr = parser
        .parse("('active').subsetOf(entry.resource.status)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - active is in status collection"),
    }

    // Test categories superset
    let expr = parser
        .parse("entry.resource.categories.supersetOf('outpatient' | 'emergency')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - categories contains outpatient and emergency"),
    }
}

#[test]
fn test_single_element_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Single element subset
    let expr = parser
        .parse("'apple'.subsetOf('apple' | 'banana')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for single element subset"),
    }

    // Single element superset
    let expr = parser
        .parse("('apple' | 'banana').supersetOf('apple')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for single element superset"),
    }

    // Single element not in collection
    let expr = parser
        .parse("'cherry'.subsetOf('apple' | 'banana')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false - cherry not in collection"),
    }
}

#[test]
fn test_boolean_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Boolean subset
    let expr = parser.parse("(true).subsetOf(true | false)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for boolean subset"),
    }

    // Boolean superset
    let expr = parser.parse("(true | false).supersetOf(false)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true for boolean superset"),
    }
}

#[test]
fn test_duplicate_elements() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_test_data();

    // Duplicates should still work
    let expr = parser.parse("(1 | 1 | 2).subsetOf(1 | 2 | 3)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - duplicates don't affect subset relationship"),
    }

    // Superset with duplicates
    let expr = parser.parse("(1 | 2 | 2 | 3).supersetOf(1 | 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true - duplicates don't affect superset relationship"),
    }
}
