//! Example demonstrating the trace() function for debugging FHIRPath expressions
//!
//! The trace() function logs diagnostic information while evaluating expressions,
//! which is useful for debugging complex FHIRPath queries.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [
            {
                "use": "official",
                "family": "Chalmers",
                "given": ["Peter", "James"]
            },
            {
                "use": "usual",
                "given": ["Jim"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "(03) 5555 6473",
                "use": "work"
            }
        ],
        "active": true
    });

    println!("=== FHIRPath trace() Function Examples ===\n");

    // Example 1: Basic trace with name only
    println!("Example 1: Basic trace");
    println!("Expression: Patient.name.trace('all-names').family");
    let context = EvaluationContext::new(patient.clone());
    let expr = parser
        .parse("Patient.name.trace('all-names').family")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!("Result: {result:?}");
    println!("Trace logs captured: {}", context.get_trace_logs().len());
    println!();

    // Example 2: Trace with projection
    println!("Example 2: Trace with projection");
    println!("Expression: Patient.name.trace('family-names', family).given");
    let context = EvaluationContext::new(patient.clone());
    let expr = parser
        .parse("Patient.name.trace('family-names', family).given")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!("Result: {result:?}");
    println!("Trace logs:");
    for log in context.get_trace_logs() {
        println!("  [{}] {}", log.name, log.value);
    }
    println!();

    // Example 3: Multiple traces in a chain
    println!("Example 3: Multiple traces in a chain");
    println!("Expression: Patient.name.trace('step1').where(use='official').trace('step2').family");
    let context = EvaluationContext::new(patient.clone());
    let expr = parser
        .parse("Patient.name.trace('step1').where(use='official').trace('step2').family")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!("Result: {result:?}");
    println!("Trace logs:");
    for log in context.get_trace_logs() {
        println!("  [{}] {}", log.name, log.value);
    }
    println!();

    // Example 4: Trace in complex filtering
    println!("Example 4: Trace in complex filtering");
    println!("Expression: Patient.telecom.trace('all-telecom').where(use='work').trace('work-only').value");
    let context = EvaluationContext::new(patient.clone());
    let expr = parser
        .parse("Patient.telecom.trace('all-telecom').where(use='work').trace('work-only').value")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    println!("Result: {result:?}");
    println!("Trace logs:");
    for log in context.get_trace_logs() {
        println!("  [{}] {}", log.name, log.value);
    }
    println!();

    // Example 5: Accessing trace logs programmatically
    println!("Example 5: Programmatic access to trace logs");
    let context = EvaluationContext::new(patient);
    let expr = parser
        .parse("Patient.name.trace('names', given).family.trace('families')")
        .unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    let logs = context.get_trace_logs();
    println!("Result: {result:?}");
    println!("Captured {} trace entries:", logs.len());
    for (i, log) in logs.iter().enumerate() {
        println!("  {}. Name: '{}', Value: {}", i + 1, log.name, log.value);
    }

    // Note: trace() also outputs to stderr (eprintln!) for backward compatibility
    // and immediate visibility during debugging
}
