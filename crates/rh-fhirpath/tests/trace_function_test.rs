//! Tests for the trace() function

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_trace_with_name_only() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["Brian"],
                "family": "Smith"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    // trace() should log the value and return it unchanged
    let expr = parser.parse("Patient.name.trace('patient-names')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Result should be the collection unchanged
    match result {
        FhirPathValue::Collection(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                FhirPathValue::Object(obj) => {
                    assert_eq!(obj.get("family").and_then(|v| v.as_str()), Some("Smith"));
                }
                _ => panic!("Expected Object in collection"),
            }
        }
        _ => panic!("Expected Collection, got {result:?}"),
    }
}

#[test]
fn test_trace_with_projection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "id": "1",
                "given": ["Brian"],
                "family": "Smith"
            },
            {
                "id": "2",
                "given": ["John"],
                "family": "Doe"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    // trace() with projection should log the projected values but return original
    let expr = parser.parse("Patient.name.trace('name-ids', id)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Result should be the original collection unchanged
    match result {
        FhirPathValue::Collection(items) => {
            assert_eq!(items.len(), 2);
            // Verify original objects are returned
            match &items[0] {
                FhirPathValue::Object(obj) => {
                    assert_eq!(obj.get("id").and_then(|v| v.as_str()), Some("1"));
                }
                _ => panic!("Expected Object"),
            }
        }
        _ => panic!("Expected Collection, got {result:?}"),
    }
}

#[test]
fn test_trace_with_empty_collection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": []
    });

    let context = EvaluationContext::new(data);

    // trace() on empty collection should return empty
    let expr = parser.parse("Patient.name.trace('empty-names')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    assert_eq!(result, FhirPathValue::Empty);
}

#[test]
fn test_trace_with_single_value() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "birthDate": "1974-12-25"
    });

    let context = EvaluationContext::new(data);

    // trace() on single value should return it unchanged
    let expr = parser
        .parse("Patient.birthDate.trace('birth-date')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::Date(d) => assert_eq!(d, "1974-12-25"),
        _ => panic!("Expected Date, got {result:?}"),
    }
}

#[test]
fn test_trace_chaining() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["Brian"],
                "family": "Smith"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    // trace() should work in chains
    let expr = parser
        .parse("Patient.name.trace('step1').family.trace('step2')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::String(s) => assert_eq!(s, "Smith"),
        _ => panic!("Expected String, got {result:?}"),
    }
}

#[test]
fn test_trace_in_where_clause() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "1",
                    "active": true
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "2",
                    "active": false
                }
            }
        ]
    });

    let context = EvaluationContext::new(data);

    // trace() should work within where clauses
    let expr = parser
        .parse("Bundle.entry.resource.trace('all-patients').where(active)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Should return only the active patient
    match result {
        FhirPathValue::Object(obj) => {
            assert_eq!(obj.get("id").and_then(|v| v.as_str()), Some("1"));
        }
        _ => panic!("Expected Object, got {result:?}"),
    }
}

#[test]
fn test_trace_error_no_parameters() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient"
    });

    let context = EvaluationContext::new(data);

    // trace() without parameters should error
    let expr = parser.parse("Patient.trace()").unwrap();
    let result = evaluator.evaluate(&expr, &context);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(format!("{e:?}").contains("requires 1 or 2 parameters"));
    }
}

#[test]
fn test_trace_error_non_string_name() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient"
    });

    let context = EvaluationContext::new(data);

    // trace() with non-string name should error
    let expr = parser.parse("Patient.trace(123)").unwrap();
    let result = evaluator.evaluate(&expr, &context);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(format!("{e:?}").contains("must evaluate to a string"));
    }
}

#[test]
fn test_trace_preserves_type() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "active": true,
        "multipleBirthInteger": 2
    });

    let context = EvaluationContext::new(data);

    // trace() should preserve the type of the value
    let expr = parser.parse("Patient.active.trace('active-flag')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean, got {result:?}"),
    }

    let expr = parser
        .parse("Patient.multipleBirthInteger.trace('birth-count')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Integer(i) => assert_eq!(i, 2),
        _ => panic!("Expected Integer, got {result:?}"),
    }
}
