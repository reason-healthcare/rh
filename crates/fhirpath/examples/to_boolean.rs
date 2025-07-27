use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath toBoolean() Function Examples ===\n");

    // Example 1: Boolean values
    println!("1. Boolean Values:");
    let context = EvaluationContext::new(json!({}));

    let expr = parser.parse("true.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   true.toBoolean() = {result:?}");

    let expr = parser.parse("false.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   false.toBoolean() = {result:?}"); // Example 2: Integer values
    println!("\n2. Integer Values:");
    let expr = parser.parse("1.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   1.toBoolean() = {result:?}");

    let expr = parser.parse("0.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.toBoolean() = {result:?}");

    let expr = parser.parse("2.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.toBoolean() = {result:?}"); // Example 3: Decimal values
    println!("\n3. Decimal Values:");
    let expr = parser.parse("1.0.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   1.0.toBoolean() = {result:?}");

    let expr = parser.parse("0.0.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.0.toBoolean() = {result:?}");

    let expr = parser.parse("2.5.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.5.toBoolean() = {result:?}"); // Example 4: String values
    println!("\n4. String Values (case-insensitive):");
    let expr = parser.parse("'true'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'true'.toBoolean() = {result:?}");

    let expr = parser.parse("'FALSE'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'FALSE'.toBoolean() = {result:?}");

    let expr = parser.parse("'yes'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'yes'.toBoolean() = {result:?}");

    let expr = parser.parse("'n'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'n'.toBoolean() = {result:?}");

    let expr = parser.parse("'1'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '1'.toBoolean() = {result:?}");

    let expr = parser.parse("'0'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '0'.toBoolean() = {result:?}");

    let expr = parser.parse("'maybe'.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'maybe'.toBoolean() = {result:?}"); // Example 5: Real-world FHIR scenarios
    println!("\n5. FHIR-like Data:");
    let fhir_context = EvaluationContext::new(json!({
        "Patient": {
            "active": "true",
            "deceasedBoolean": false,
            "multipleBirthInteger": 1,
            "gender": "male"
        }
    }));

    let expr = parser.parse("Patient.active.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.active.toBoolean() = {result:?}");

    let expr = parser.parse("Patient.deceasedBoolean.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.deceasedBoolean.toBoolean() = {result:?}");

    let expr = parser.parse("Patient.multipleBirthInteger.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.multipleBirthInteger.toBoolean() = {result:?}");

    let expr = parser.parse("Patient.gender.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.gender.toBoolean() = {result:?}"); // Example 6: Collection handling
    println!("\n6. Collection Handling:");
    let collection_context = EvaluationContext::new(json!({
        "flags": ["true", "false", "1", "0", "maybe"],
        "singleFlag": ["yes"]
    }));

    let expr = parser.parse("singleFlag.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   singleFlag.toBoolean() = {result:?}");

    let expr = parser.parse("flags.toBoolean()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   flags.toBoolean() = {result:?}");
    println!("\n=== Summary ===");
    println!("The toBoolean() function converts values according to FHIRPath spec:");
    println!("- Boolean: true/false → true/false");
    println!("- Integer: 1 → true, 0 → false, others → empty");
    println!("- Decimal: 1.0 → true, 0.0 → false, others → empty");
    println!("- String: 'true','t','yes','y','1','1.0' → true (case-insensitive)");
    println!("          'false','f','no','n','0','0.0' → false (case-insensitive)");
    println!("          others → empty");
    println!("- Collections: single item → convert item, multiple items → empty");

    Ok(())
}
