//! ofType Function Tests
//!
//! Comprehensive tests for the ofType(type) function that filters collections based on type.
//! The ofType function returns only those items in a collection that match the specified type.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

/// Create a test context with mixed type collection
fn create_mixed_type_context() -> EvaluationContext {
    let data = json!({
        "mixedCollection": [
            "hello",
            42,
            true,
            3.15,
            null,
            {"resourceType": "Patient", "id": "patient1"},
            {"resourceType": "Observation", "id": "obs1"},
            "world"
        ],
        "numbers": [1, 2, 3.5, 4],
        "strings": ["apple", "banana", "cherry"],
        "booleans": [true, false, true],
        "resources": [
            {"resourceType": "Patient", "id": "p1", "active": true},
            {"resourceType": "Patient", "id": "p2", "active": false},
            {"resourceType": "Observation", "id": "o1", "status": "final"},
            {"resourceType": "Practitioner", "id": "pr1", "active": true}
        ]
    });

    EvaluationContext::new(data)
}

#[test]
fn test_of_type_basic_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter strings from mixed collection
    let expr = parser.parse("mixedCollection.ofType(String)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        for item in items {
            assert!(matches!(item, FhirPathValue::String(_)));
        }
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_integer_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter integers
    let expr = parser.parse("mixedCollection.ofType(Integer)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        assert!(matches!(items[0], FhirPathValue::Integer(42)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_boolean_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter booleans
    let expr = parser.parse("mixedCollection.ofType(Boolean)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        assert!(matches!(items[0], FhirPathValue::Boolean(true)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_decimal_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter decimals/numbers
    let expr = parser.parse("mixedCollection.ofType(Decimal)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        assert!(matches!(items[0], FhirPathValue::Number(_)));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_resource_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter Patient resources
    let expr = parser.parse("resources.ofType(Patient)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
        for item in items {
            if let FhirPathValue::Object(obj) = item {
                assert_eq!(
                    obj.get("resourceType").unwrap().as_str().unwrap(),
                    "Patient"
                );
            } else {
                panic!("Expected object result");
            }
        }
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_observation_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter Observation resources
    let expr = parser.parse("resources.ofType(Observation)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        if let FhirPathValue::Object(obj) = &items[0] {
            assert_eq!(
                obj.get("resourceType").unwrap().as_str().unwrap(),
                "Observation"
            );
            assert_eq!(obj.get("id").unwrap().as_str().unwrap(), "o1");
        } else {
            panic!("Expected object result");
        }
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_of_type_no_matches() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter for type that doesn't exist
    let expr = parser.parse("strings.ofType(Integer)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_of_type_on_single_item() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Filter single string item
    let expr = parser.parse("strings[0].ofType(String)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "apple");
    } else {
        panic!("Expected string result, got: {result:?}");
    }
}

#[test]
fn test_of_type_on_single_item_wrong_type() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Try to filter string as integer
    let expr = parser.parse("strings[0].ofType(Integer)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_of_type_chained_with_other_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Chain ofType with count
    let expr = parser.parse("resources.ofType(Patient).count()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Integer(count) = result {
        assert_eq!(count, 2);
    } else {
        panic!("Expected integer result");
    }
}

#[test]
fn test_of_type_with_where_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Chain with where clause
    let expr = parser
        .parse("resources.ofType(Patient).where(active = true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 1);
        if let FhirPathValue::Object(obj) = &items[0] {
            assert_eq!(obj.get("id").unwrap().as_str().unwrap(), "p1");
            assert!(obj.get("active").unwrap().as_bool().unwrap());
        }
    } else if let FhirPathValue::Object(obj) = result {
        // Handle single object result
        assert_eq!(obj.get("id").unwrap().as_str().unwrap(), "p1");
        assert!(obj.get("active").unwrap().as_bool().unwrap());
    } else {
        panic!("Expected collection or object result, got: {result:?}");
    }
}

#[test]
fn test_of_type_empty_collection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({"empty": []}));

    // Filter empty collection
    let expr = parser.parse("empty.ofType(String)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_of_type_case_insensitive_resource_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_mixed_type_context();

    // Test case insensitive matching for resource types
    let expr = parser.parse("resources.ofType(patient)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection result");
    }
}
