use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    // Create a sample FHIR Patient resource
    let patient = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "James"]
            },
            {
                "use": "usual",
                "family": "Doe",
                "given": ["Johnny"]
            }
        ],
        "birthDate": "1974-12-25"
    });

    // Create parser and evaluator
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(patient);

    // Example 1: Simple member access
    println!("=== Example 1: Simple member access ===");
    let expr = parser.parse("Patient.id")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Patient.id = {:?}", result);

    // Example 2: Array access
    println!("\n=== Example 2: Array access ===");
    let expr = parser.parse("Patient.name[0].family")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Patient.name[0].family = {:?}", result);

    // Example 3: Literal values
    println!("\n=== Example 3: Literal values ===");
    let expr = parser.parse("true")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("true = {:?}", result);

    let expr = parser.parse("'Hello, FHIRPath!'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'Hello, FHIRPath!' = {:?}", result);

    let expr = parser.parse("42")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42 = {:?}", result);

    // Example 4: Function calls (parsed but not yet fully implemented)
    println!("\n=== Example 4: Function calls (parsing only) ===");
    let expr = parser.parse("name.count()")?;
    println!("Parsed expression: {}", expr);

    // Example 5: Complex expressions
    println!("\n=== Example 5: Complex expressions (parsing only) ===");
    let expr = parser.parse("name.where(use = 'official').given")?;
    println!("Parsed expression: {}", expr);

    let expr = parser.parse("name.given | name.family")?;
    println!("Parsed union expression: {}", expr);

    Ok(())
}
