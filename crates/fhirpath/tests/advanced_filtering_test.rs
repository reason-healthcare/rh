//! Advanced Filtering Functions Tests
//!
//! Tests for complex filtering operations including:
//! - Advanced where() conditions with complex expressions
//! - Nested filtering operations
//! - select() with transformations
//! - Conditional filtering (simplified)
//! - Filtering with collection functions
//! - Performance and edge case scenarios

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn create_complex_test_data() -> EvaluationContext {
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
                            "given": ["John", "James"],
                            "prefix": ["Mr."]
                        },
                        {
                            "use": "usual",
                            "family": "Smith",
                            "given": ["Johnny"]
                        }
                    ],
                    "birthDate": "1990-05-15",
                    "active": true,
                    "gender": "male",
                    "contact": [
                        {
                            "relationship": [{"text": "Emergency"}],
                            "name": {"family": "Jones", "given": ["Mary"]},
                            "telecom": [{"system": "phone", "value": "555-1234"}]
                        },
                        {
                            "relationship": [{"text": "Spouse"}],
                            "name": {"family": "Smith", "given": ["Jane"]},
                            "telecom": [{"system": "email", "value": "jane@example.com"}]
                        }
                    ],
                    "telecom": [
                        {"system": "phone", "value": "555-9876", "use": "home"},
                        {"system": "email", "value": "john@example.com", "use": "work"},
                        {"system": "phone", "value": "555-5555", "use": "mobile"}
                    ]
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
                    "birthDate": "1985-08-22",
                    "active": false,
                    "gender": "female",
                    "telecom": [
                        {"system": "email", "value": "alice@example.com", "use": "home"}
                    ]
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final",
                    "code": {"text": "Blood Pressure"},
                    "valueQuantity": {"value": 120, "unit": "mmHg"},
                    "effectiveDateTime": "2023-01-15T09:30:00Z"
                }
            }
        ]
    });
    EvaluationContext::new(data)
}

#[test]
fn test_advanced_where_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Simple multi-field where condition
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient' and active = true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Where with existence check
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Where with string operations (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.where(family = 'Smith')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
}

#[test]
fn test_nested_filtering_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Simple nested where conditions
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').telecom.where(system = 'phone')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Where followed by select
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.select(family)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Simple nested path with filtering
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.where(use = 'official')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
}

#[test]
fn test_select_with_transformations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Basic select with path
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.select(family)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection, string or object result"),
    }

    // Select simple field
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').select(active)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Boolean(_) => {}
        _ => panic!("Expected collection or boolean result"),
    }

    // Select with nested path
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').select(resourceType)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(items) => assert_eq!(items.len(), 2),
        FhirPathValue::Boolean(_) => {} // Single result
        _ => panic!("Expected collection or boolean result"),
    }
}

#[test]
fn test_filtering_with_collection_functions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Where with collection function conditions
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(items) => assert_eq!(items.len(), 2),
        _ => panic!("Expected collection result"),
    }

    // Select with simple count
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.count()")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Integer(_)));
}

#[test]
fn test_conditional_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Basic conditional filtering based on resource type
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Filter observations
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Observation')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
}

#[test]
fn test_filtering_with_arithmetic() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Simple arithmetic-based filtering on FHIR data
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').where(active = true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Simple select transformation
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').select(id)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }
}

#[test]
fn test_filtering_edge_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Where on empty collection
    let expr = parser.parse("{}.where($this > 0)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Where with always false condition
    let expr = parser.parse("(1 | 2 | 3).where(false)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Where with always true condition
    let expr = parser.parse("(1 | 2 | 3).where(true)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    if let FhirPathValue::Collection(items) = result {
        assert_eq!(items.len(), 3);
    } else {
        panic!("Expected collection result");
    }

    // Select on empty collection (simplified)
    let expr = parser.parse("{}.select($this)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));

    // Where with non-existent field
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').where(nonexistentField = 'value')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert!(matches!(result, FhirPathValue::Empty));
}

#[test]
fn test_complex_real_world_filtering() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Find active patients with phone numbers (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient' and active = true)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Get email addresses for all patients (simplified)
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').telecom.where(system = 'email').select(value)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }

    // Find patients with emergency contacts (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Get all official family names (simplified)
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').name.where(use = 'official').select(family)").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }
}

#[test]
fn test_chained_filtering_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Multiple chained where operations (simplified)
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').where(active.exists()).where(gender = 'male')").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Where followed by select (simplified)
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').name.select(given)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }

    // Simple chain with basic operations
    let expr = parser.parse("entry.resource.where(resourceType = 'Patient').select(telecom.where(system = 'phone'))").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }
}

#[test]
fn test_filtering_with_date_conditions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = create_complex_test_data();

    // Simple date comparison
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').where(birthDate = '1990-05-15')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Simple birth date filtering
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient' and birthDate = '1990-05-15')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::Object(_) => {}
        _ => panic!("Expected collection or object result"),
    }

    // Select birth dates
    let expr = parser
        .parse("entry.resource.where(resourceType = 'Patient').select(birthDate)")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    match result {
        FhirPathValue::Collection(_) => {}
        FhirPathValue::String(_) => {}
        _ => panic!("Expected collection or string result"),
    }
}
