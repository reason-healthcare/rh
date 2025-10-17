use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_resource_type_as_root_path() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create a Patient resource
    let patient = json!({
        "resourceType": "Patient",
        "id": "example-123",
        "name": [{
            "family": "Smith",
            "given": ["John"]
        }],
        "active": true
    });

    let context = EvaluationContext::new(patient.clone());

    // Test 1: Patient.id should work like id
    let expr1 = parser.parse("Patient.id").unwrap();
    let result1 = evaluator.evaluate(&expr1, &context).unwrap();

    let expr2 = parser.parse("id").unwrap();
    let result2 = evaluator.evaluate(&expr2, &context).unwrap();

    assert_eq!(result1, result2);
    assert_eq!(result1, FhirPathValue::String("example-123".to_string()));

    // Test 2: Patient.name.family should work
    let expr3 = parser.parse("Patient.name.family").unwrap();
    let result3 = evaluator.evaluate(&expr3, &context).unwrap();

    let expr4 = parser.parse("name.family").unwrap();
    let result4 = evaluator.evaluate(&expr4, &context).unwrap();

    assert_eq!(result3, result4);

    // Test 3: Patient.active should work
    let expr5 = parser.parse("Patient.active").unwrap();
    let result5 = evaluator.evaluate(&expr5, &context).unwrap();
    assert_eq!(result5, FhirPathValue::Boolean(true));

    // Test 4: Non-matching resourceType should return empty
    let expr6 = parser.parse("Observation.id").unwrap();
    let result6 = evaluator.evaluate(&expr6, &context).unwrap();
    assert_eq!(result6, FhirPathValue::Empty);
}

#[test]
fn test_resource_type_with_different_resources() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test with Observation
    let observation = json!({
        "resourceType": "Observation",
        "id": "obs-456",
        "status": "final"
    });

    let context = EvaluationContext::new(observation);

    let expr = parser.parse("Observation.status").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("final".to_string()));

    // Test with Medication
    let medication = json!({
        "resourceType": "Medication",
        "id": "med-789",
        "code": {
            "text": "Aspirin"
        }
    });

    let context = EvaluationContext::new(medication);

    let expr = parser.parse("Medication.code.text").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(result, FhirPathValue::String("Aspirin".to_string()));
}

#[test]
fn test_resource_type_root_equivalence() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient = json!({
        "resourceType": "Patient",
        "id": "test-patient"
    });

    let context = EvaluationContext::new(patient);

    // These should all be equivalent
    let expressions = vec!["Patient.id", "id", "%resource.id"];

    let mut results = Vec::new();
    for expr_str in expressions {
        let expr = parser.parse(expr_str).unwrap();
        let result = evaluator.evaluate(&expr, &context).unwrap();
        results.push(result);
    }

    // All results should be equal
    for i in 1..results.len() {
        assert_eq!(
            results[0], results[i],
            "Expression results should be equal but differ at index {i}"
        );
    }
}
