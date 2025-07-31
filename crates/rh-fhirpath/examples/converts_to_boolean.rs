use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath convertsToBoolean() Function Examples ===\n");

    // Example 1: Boolean values
    println!("1. Boolean Values:");
    let context = EvaluationContext::new(json!({}));

    let expr = parser.parse("true.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   true.convertsToBoolean() = {result:?}");

    let expr = parser.parse("false.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   false.convertsToBoolean() = {result:?}");

    // Example 2: Integer values
    println!("\n2. Integer Values:");
    let expr = parser.parse("1.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   1.convertsToBoolean() = {result:?}");

    let expr = parser.parse("0.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.convertsToBoolean() = {result:?}");

    let expr = parser.parse("2.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.convertsToBoolean() = {result:?}");

    // Example 3: Decimal values
    println!("\n3. Decimal Values:");
    let expr = parser.parse("1.0.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   1.0.convertsToBoolean() = {result:?}");

    let expr = parser.parse("0.0.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.0.convertsToBoolean() = {result:?}");

    let expr = parser.parse("2.5.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.5.convertsToBoolean() = {result:?}");

    // Example 4: String values
    println!("\n4. String Values (case-insensitive):");
    let expr = parser.parse("'true'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'true'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'FALSE'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'FALSE'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'yes'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'yes'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'n'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'n'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'1'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '1'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'0'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '0'.convertsToBoolean() = {result:?}");

    let expr = parser.parse("'maybe'.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'maybe'.convertsToBoolean() = {result:?}");

    // Example 5: Real-world FHIR scenarios
    println!("\n5. FHIR-like Data:");
    let fhir_context = EvaluationContext::new(json!({
        "Patient": {
            "active": "true",
            "deceasedBoolean": false,
            "multipleBirthInteger": 1,
            "gender": "male"
        }
    }));

    let expr = parser.parse("Patient.active.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.active.convertsToBoolean() = {result:?}");

    let expr = parser.parse("Patient.deceasedBoolean.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.deceasedBoolean.convertsToBoolean() = {result:?}");

    let expr = parser.parse("Patient.multipleBirthInteger.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.multipleBirthInteger.convertsToBoolean() = {result:?}");

    let expr = parser.parse("Patient.gender.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.gender.convertsToBoolean() = {result:?}");

    // Example 6: Collection handling
    println!("\n6. Collection Handling:");
    let collection_context = EvaluationContext::new(json!({
        "flags": ["true", "false", "1", "0", "maybe"],
        "singleFlag": ["yes"],
        "emptyArray": []
    }));

    let expr = parser.parse("singleFlag.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   singleFlag.convertsToBoolean() = {result:?}");

    let expr = parser.parse("flags.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   flags.convertsToBoolean() = {result:?}");

    let expr = parser.parse("emptyArray.convertsToBoolean()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   emptyArray.convertsToBoolean() = {result:?}");

    // Example 7: Comparison with toBoolean()
    println!("\n7. Comparison with toBoolean():");
    println!("   Value    | convertsToBoolean() | toBoolean()");
    println!("   ---------|--------------------|-----------");

    let test_values = vec!["'true'", "'false'", "'maybe'", "1", "0", "2"];

    for value in test_values {
        let converts_expr = parser.parse(&format!("{value}.convertsToBoolean()"))?;
        let converts_result = evaluator.evaluate(&converts_expr, &context)?;

        let to_bool_expr = parser.parse(&format!("{value}.toBoolean()"))?;
        let to_bool_result = evaluator.evaluate(&to_bool_expr, &context)?;

        println!(
            "   {:<8} | {:<18} | {to_bool_result:?}",
            value,
            format!("{converts_result:?}")
        );
    }

    println!("\n=== Summary ===");
    println!(
        "The convertsToBoolean() function returns true if a value can be converted to boolean:"
    );
    println!("- Boolean: Always true (true/false both convertible)");
    println!("- Integer: true if 1 or 0, false otherwise");
    println!("- Decimal: true if 1.0 or 0.0, false otherwise");
    println!("- String: true if 'true','t','yes','y','1','1.0','false','f','no','n','0','0.0' (case-insensitive)");
    println!("          false otherwise");
    println!("- Collections: true if single convertible item, false if multiple items or empty");
    println!("- Other types: Always false");
    println!("\nThis differs from toBoolean() which returns the actual converted boolean value");
    println!("or Empty for non-convertible values.");

    Ok(())
}
