//! Tests for trace log collection

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_trace_logs_are_captured() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["John"],
                "family": "Doe"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    let expr = parser.parse("Patient.name.trace('step1').family").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Verify result is correct
    assert_eq!(result, FhirPathValue::String("Doe".to_string()));

    // Verify trace logs were captured
    let logs = context.get_trace_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].name, "step1");
    assert!(logs[0].value.contains("Doe"));
}

#[test]
fn test_multiple_trace_logs() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["John"],
                "family": "Doe"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    let expr = parser
        .parse("Patient.name.trace('names').family.trace('family')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Verify result is correct
    assert_eq!(result, FhirPathValue::String("Doe".to_string()));

    // Verify both trace logs were captured
    let logs = context.get_trace_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].name, "names");
    assert_eq!(logs[1].name, "family");
}

#[test]
fn test_trace_logs_with_projection() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [
            {
                "given": ["John"],
                "family": "Doe"
            }
        ]
    });

    let context = EvaluationContext::new(data);

    let expr = parser
        .parse("Patient.name.trace('families', family)")
        .unwrap();
    evaluator.evaluate(&expr, &context).unwrap();

    // Verify trace log shows the projection
    let logs = context.get_trace_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].name, "families");
    assert!(logs[0].value.contains("Doe"));
    assert!(!logs[0].value.contains("John")); // Should not contain given name
}

#[test]
fn test_trace_logs_can_be_cleared() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "active": true
    });

    let context = EvaluationContext::new(data);

    let expr = parser.parse("Patient.active.trace('first')").unwrap();
    evaluator.evaluate(&expr, &context).unwrap();

    assert_eq!(context.get_trace_logs().len(), 1);

    // Clear logs
    context.clear_trace_logs();
    assert_eq!(context.get_trace_logs().len(), 0);

    // Evaluate again
    let expr2 = parser.parse("Patient.active.trace('second')").unwrap();
    evaluator.evaluate(&expr2, &context).unwrap();

    // Should only have the second trace
    let logs = context.get_trace_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].name, "second");
}

#[test]
fn test_trace_logs_shared_across_context_clones() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let data = json!({
        "resourceType": "Patient",
        "name": [{"family": "Doe"}]
    });

    let context = EvaluationContext::new(data);

    // Trace logs should be shared via Rc<RefCell<>>
    let expr = parser.parse("Patient.name.trace('test')").unwrap();
    evaluator.evaluate(&expr, &context).unwrap();

    // Get logs from original context
    let logs = context.get_trace_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].name, "test");
}
