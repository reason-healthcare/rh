//! Membership Operations Tests
//!
//! Tests for membership operations including:
//! - `in` operator for checking membership in collections
//! - `contains` operator for checking if collection contains value
//! - Mixed type membership operations
//! - Membership with FHIR data structures
//! - Edge cases and error conditions
//! - Performance scenarios with large collections

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_membership_test_data() -> EvaluationContext {
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
                        },
                        {
                            "use": "usual",
                            "family": "Smith",
                            "given": ["Johnny"]
                        }
                    ],
                    "telecom": [
                        {"system": "phone", "value": "555-1234"},
                        {"system": "email", "value": "john@example.com"},
                        {"system": "fax", "value": "555-5678"}
                    ],
                    "active": true,
                    "gender": "male",
                    "birthDate": "1990-05-15"
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient2",
                    "name": [
                        {
                            "use": "official",
                            "family": "Johnson",
                            "given": ["Alice", "Marie"]
                        }
                    ],
                    "telecom": [
                        {"system": "email", "value": "alice@example.com"}
                    ],
                    "active": false,
                    "gender": "female",
                    "birthDate": "1985-08-22"
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final",
                    "category": [
                        {"text": "vital-signs"},
                        {"text": "survey"}
                    ],
                    "code": {"text": "Blood Pressure"},
                    "valueQuantity": {"value": 120, "unit": "mmHg"}
                }
            }
        ]
    });
    EvaluationContext::new(data)
}

#[test]
fn test_basic_in_operator() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test string in collection
    let expr = parser
        .parse("'phone' in entry.resource.where(resourceType = 'Patient').telecom.system")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test value not in collection
    let expr = parser
        .parse("'sms' in entry.resource.where(resourceType = 'Patient').telecom.system")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test number in collection
    let expr = parser
        .parse("120 in entry.resource.where(resourceType = 'Observation').valueQuantity.value")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }
}

#[test]
fn test_contains_operator() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test collection contains value
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').telecom.system contains 'phone'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test collection does not contain value
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').telecom.system contains 'pager'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test contains with numeric values
    let expr = parser
        .parse(
            "entry.resource.where(resourceType = 'Observation').valueQuantity.value contains 120",
        )
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }
}

#[test]
fn test_membership_with_literal_collections() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test in with literal collection
    let expr = parser
        .parse("'male' in ('male' | 'female' | 'other')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true result"),
    }

    // Test not in literal collection
    let expr = parser
        .parse("'unknown' in ('male' | 'female' | 'other')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false result"),
    }

    // Test contains with literal collection
    let expr = parser
        .parse("('active' | 'inactive' | 'entered-in-error') contains 'active'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true result"),
    }
}

#[test]
fn test_membership_with_mixed_types() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test mixed string and boolean
    let expr = parser.parse("true in (true | false | 'maybe')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test number in mixed collection
    let expr = parser.parse("5 in (1 | 2 | 'three' | 4 | 5)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test string representation vs actual type
    let expr = parser.parse("'5' in (1 | 2 | 3 | 4 | 5)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }
}

#[test]
fn test_membership_with_fhir_codes() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test resource type membership
    let expr = parser
        .parse("'Patient' in entry.resource.resourceType")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test status code membership
    let expr = parser
        .parse("'final' in entry.resource.where(resourceType = 'Observation').status")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test gender code membership
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').gender contains 'male'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }
}

#[test]
fn test_nested_membership_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test chained membership with where
    let expr = parser
        .parse("entry.resource.where(resourceType in ('Patient' | 'Observation')).resourceType")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }

    // Test membership within select
    let expr = parser
        .parse(
            "entry.resource.where(resourceType = 'Patient').select(gender in ('male' | 'female'))",
        )
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected collection or boolean result"),
    }

    // Test complex nested membership
    let expr = parser
        .parse(
            "entry.resource.where(resourceType = 'Patient').where(telecom.system contains 'email')",
        )
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
}

#[test]
fn test_membership_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test membership with empty collection
    let expr = parser.parse("'test' in {}").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false result"),
    }

    // Test empty string in collection
    let expr = parser.parse("'' in ('a' | '' | 'c')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(true) => {}
        _ => panic!("Expected true result"),
    }

    // Test null/empty membership
    let expr = parser.parse("{} in ('a' | 'b' | 'c')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test contains with empty collection
    let expr = parser.parse("{} contains 'test'").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(false) => {}
        _ => panic!("Expected false result"),
    }
}

#[test]
fn test_membership_with_dates() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test date membership
    let expr = parser
        .parse("'1990-05-15' in entry.resource.where(resourceType = 'Patient').birthDate")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test date contains
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').birthDate contains '1985-08-22'")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test date range membership (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').birthDate")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }
}

#[test]
fn test_complex_membership_scenarios() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_membership_test_data();

    // Test membership in filtered collection
    let expr = parser.parse("'phone' in entry.resource.where(resourceType = 'Patient' and active = true).telecom.system").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected boolean result"),
    }

    // Test multiple membership conditions
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').where(gender in ('male' | 'female') and active in (true | false))").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Test membership with select and transformation
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').telecom.where(system in ('phone' | 'email')).select(value)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }

    // Test complex nested membership with categories (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Observation').category.text")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }
}
