//! iif Function Tests
//!
//! Comprehensive tests for the iif(criterion, true-result [, otherwise-result]) function.
//! The iif function is an immediate if, also known as a conditional operator (like C's ? : operator).
//! It evaluates the criterion and returns true-result if truthy, otherwise returns otherwise-result.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

/// Create a test context with various values for iif testing
fn create_iif_test_context() -> EvaluationContext {
    let data = json!({
        "patient": {
            "resourceType": "Patient",
            "id": "patient1",
            "active": true,
            "name": [{"family": "Smith", "given": ["John"]}],
            "birthDate": "1990-01-01",
            "telecom": [
                {"system": "phone", "value": "555-1234"},
                {"system": "email", "value": "john@example.com"}
            ]
        },
        "observation": {
            "resourceType": "Observation",
            "id": "obs1",
            "status": "final",
            "code": {"text": "Blood Pressure"},
            "valueQuantity": {"value": 120, "unit": "mmHg"}
        },
        "testValues": {
            "trueBool": true,
            "falseBool": false,
            "nonZeroInt": 42,
            "zeroInt": 0,
            "nonEmptyString": "hello",
            "emptyString": "",
            "nonEmptyArray": [1, 2, 3],
            "emptyArray": [],
            "nullValue": null
        },
        "results": {
            "success": "SUCCESS",
            "failure": "FAILURE",
            "defaultResult": "DEFAULT",
            "numbers": [1, 2, 3],
            "strings": ["a", "b", "c"]
        }
    });

    EvaluationContext::new(data)
}

#[test]
fn test_iif_basic_boolean_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with true boolean - should return true-result
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }

    // Test with false boolean - should return otherwise-result
    let expr = parser
        .parse("''.iif(testValues.falseBool, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "FAILURE");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_without_otherwise_result() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with true condition and no otherwise-result
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.success)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }

    // Test with false condition and no otherwise-result - should return empty
    let expr = parser
        .parse("''.iif(testValues.falseBool, results.success)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_iif_with_non_boolean_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with non-zero integer (truthy)
    let expr = parser
        .parse("''.iif(testValues.nonZeroInt, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }

    // Test with zero integer (truthy in FHIRPath)
    let expr = parser
        .parse("''.iif(testValues.zeroInt, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS"); // Zero is truthy in FHIRPath
    } else {
        panic!("Expected string result");
    }

    // Test with non-empty string (truthy)
    let expr = parser
        .parse("''.iif(testValues.nonEmptyString, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }

    // Test with empty string (truthy in FHIRPath)
    let expr = parser
        .parse("''.iif(testValues.emptyString, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS"); // Empty string is truthy in FHIRPath
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_with_collection_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with non-empty array (truthy)
    let expr = parser
        .parse("''.iif(testValues.nonEmptyArray, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }

    // Test with empty array (falsy)
    let expr = parser
        .parse("''.iif(testValues.emptyArray, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "FAILURE");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_with_collection_results() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test returning collection as true-result
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.numbers, results.strings)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::Integer(1)));
        assert!(matches!(items[1], FhirPathValue::Integer(2)));
        assert!(matches!(items[2], FhirPathValue::Integer(3)));
    } else {
        panic!("Expected collection result");
    }

    // Test returning collection as otherwise-result
    let expr = parser
        .parse("''.iif(testValues.falseBool, results.numbers, results.strings)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], FhirPathValue::String(ref s) if s == "a"));
        assert!(matches!(items[1], FhirPathValue::String(ref s) if s == "b"));
        assert!(matches!(items[2], FhirPathValue::String(ref s) if s == "c"));
    } else {
        panic!("Expected collection result");
    }
}

#[test]
fn test_iif_with_complex_expressions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with comparison expression as criterion
    let expr = parser
        .parse("''.iif(patient.active = true, 'Active Patient', 'Inactive Patient')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Active Patient");
    } else {
        panic!("Expected string result");
    }

    // Test with exists() expression as criterion
    let expr = parser
        .parse("''.iif(patient.name.exists(), patient.name.family, 'No Name')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Smith");
    } else {
        panic!("Expected string result, got: {:?}", result);
    }

    // Test with count() expression as criterion
    let expr = parser
        .parse("''.iif(patient.telecom.count() > 1, 'Multiple Contacts', 'Single Contact')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Multiple Contacts");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_nested_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test nested iif calls
    let expr = parser.parse("''.iif(patient.active, ''.iif(patient.name.exists(), 'Active with Name', 'Active without Name'), 'Inactive')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Active with Name");
    } else {
        panic!("Expected string result");
    }

    // Test nested iif with false outer condition
    let expr = parser.parse("''.iif(testValues.falseBool, ''.iif(testValues.trueBool, 'Inner True', 'Inner False'), 'Outer False')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Outer False");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_with_empty_and_null_values() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with null value as criterion (should be falsy)
    let expr = parser
        .parse("''.iif(testValues.nullValue, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "FAILURE");
    } else {
        panic!("Expected string result");
    }

    // Test with non-existent field as criterion (should be falsy/empty)
    let expr = parser
        .parse("''.iif(patient.nonExistentField, results.success, results.failure)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "FAILURE");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_return_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test returning boolean
    let expr = parser
        .parse("''.iif(testValues.trueBool, testValues.trueBool, testValues.falseBool)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Boolean(true)));

    // Test returning integer
    let expr = parser
        .parse("''.iif(testValues.trueBool, testValues.nonZeroInt, testValues.zeroInt)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(42)));

    // Test returning object
    let expr = parser
        .parse("''.iif(testValues.trueBool, patient, observation)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Object(obj) = result {
        assert_eq!(
            obj.get("resourceType").and_then(|v| v.as_str()),
            Some("Patient")
        );
        assert_eq!(obj.get("id").and_then(|v| v.as_str()), Some("patient1"));
    } else {
        panic!("Expected object result");
    }
}

#[test]
fn test_iif_chained_with_other_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test chaining iif result with count()
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.numbers, results.strings).count()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(3)));

    // Test chaining iif result with first()
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.numbers, results.strings).first()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(1)));

    // Test chaining iif result with string function
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.success, results.failure).upper()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "SUCCESS");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_iif_parameter_validation() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Test with too few parameters (only 1 parameter)
    let expr = parser.parse("''.iif(testValues.trueBool)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("iif() requires 2 or 3 parameters"));

    // Test with too many parameters (4 parameters)
    let expr = parser
        .parse("''.iif(testValues.trueBool, results.success, results.failure, 'extra')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("iif() requires 2 or 3 parameters"));
}

#[test]
fn test_iif_real_world_scenarios() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_iif_test_context();

    // Scenario 1: Simple contact method selection (simplified without where clause)
    let expr = parser
        .parse("''.iif(patient.telecom.exists(), patient.telecom[0].value, 'No contact')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "555-1234");
    } else {
        panic!("Expected string result, got: {:?}", result);
    }

    // Scenario 2: Age-based categorization (simulated)
    let expr = parser
        .parse("''.iif(patient.birthDate = '1990-01-01', 'Adult', 'Other')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::String(s) = result {
        assert_eq!(s, "Adult");
    } else {
        panic!("Expected string result");
    }

    // Scenario 3: Status-based result interpretation
    let expr = parser
        .parse("''.iif(observation.status = 'final', observation.valueQuantity.value, 'Pending')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(120)));
}
