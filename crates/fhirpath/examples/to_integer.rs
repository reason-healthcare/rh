use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath toInteger() Function Examples ===\n");

    // Example 1: Integer values
    println!("1. Integer Values:");
    let context = EvaluationContext::new(json!({}));

    let expr = parser.parse("42.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   42.toInteger() = {result:?}");

    let expr = parser.parse("0.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.toInteger() = {result:?}");

    let expr = parser.parse("(-123).toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   (-123).toInteger() = {result:?}");

    // Example 2: Boolean values
    println!("\n2. Boolean Values:");
    let expr = parser.parse("true.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   true.toInteger() = {result:?}");

    let expr = parser.parse("false.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   false.toInteger() = {result:?}");

    // Example 3: Decimal values
    println!("\n3. Decimal Values:");
    let expr = parser.parse("42.0.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   42.0.toInteger() = {result:?}");

    let expr = parser.parse("0.0.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   0.0.toInteger() = {result:?}");

    let expr = parser.parse("(-123.0).toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   (-123.0).toInteger() = {result:?}");

    let expr = parser.parse("2.5.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.5.toInteger() = {result:?}");

    let expr = parser.parse("3.14159.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   3.14159.toInteger() = {result:?}");

    // Example 4: String values
    println!("\n4. String Values:");
    let expr = parser.parse("'42'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '42'.toInteger() = {result:?}");

    let expr = parser.parse("'0'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '0'.toInteger() = {result:?}");

    let expr = parser.parse("'-123'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '-123'.toInteger() = {result:?}");

    let expr = parser.parse("'  456  '.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '  456  '.toInteger() = {result:?}");

    let expr = parser.parse("'abc'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'abc'.toInteger() = {result:?}");

    let expr = parser.parse("'42.5'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '42.5'.toInteger() = {result:?}");

    let expr = parser.parse("'true'.toInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'true'.toInteger() = {result:?}");

    // Example 5: Real-world FHIR scenarios
    println!("\n5. FHIR-like Data:");
    let fhir_context = EvaluationContext::new(json!({
        "Patient": {
            "multipleBirthInteger": 2,
            "deceasedBoolean": false,
            "age": "45",
            "weight": 70.5,
            "name": "John Doe"
        },
        "Observation": {
            "valueInteger": 120,
            "valueQuantity": {
                "value": 98.6
            },
            "valueString": "37"
        }
    }));

    let expr = parser.parse("Patient.multipleBirthInteger.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.multipleBirthInteger.toInteger() = {result:?}");

    let expr = parser.parse("Patient.deceasedBoolean.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.deceasedBoolean.toInteger() = {result:?}");

    let expr = parser.parse("Patient.age.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.age.toInteger() = {result:?}");

    let expr = parser.parse("Patient.weight.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.weight.toInteger() = {result:?}");

    let expr = parser.parse("Patient.name.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Patient.name.toInteger() = {result:?}");

    let expr = parser.parse("Observation.valueInteger.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Observation.valueInteger.toInteger() = {result:?}");

    let expr = parser.parse("Observation.valueString.toInteger()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Observation.valueString.toInteger() = {result:?}");

    // Example 6: Collection handling
    println!("\n6. Collection Handling:");
    let collection_context = EvaluationContext::new(json!({
        "values": ["42", "73", "invalid", "100"],
        "singleValue": [99],
        "booleans": [true, false],
        "decimals": [42.0, 73.5]
    }));

    let expr = parser.parse("singleValue.toInteger()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   singleValue.toInteger() = {result:?}");

    let expr = parser.parse("values.toInteger()")?;
    let result = evaluator.evaluate(&expr, &collection_context)?;
    println!("   values.toInteger() = {result:?}");

    // Example 7: convertsToInteger() function
    println!("\n7. Convertibility Testing with convertsToInteger():");

    let expr = parser.parse("42.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   42.convertsToInteger() = {result:?}");

    let expr = parser.parse("true.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   true.convertsToInteger() = {result:?}");

    let expr = parser.parse("42.0.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   42.0.convertsToInteger() = {result:?}");

    let expr = parser.parse("2.5.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   2.5.convertsToInteger() = {result:?}");

    let expr = parser.parse("'42'.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '42'.convertsToInteger() = {result:?}");

    let expr = parser.parse("'abc'.convertsToInteger()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'abc'.convertsToInteger() = {result:?}");

    println!("\n=== Summary ===");
    println!("The toInteger() function converts values according to FHIRPath spec:");
    println!("- Integer: any integer → same integer");
    println!("- Boolean: true → 1, false → 0");
    println!("- Decimal: whole numbers (42.0) → integer (42), fractional → empty");
    println!("- String: valid integer strings ('42', '-123') → integer, others → empty");
    println!("- Other types: → empty");
    println!("- Collections: single item → convert item, multiple items → empty");
    println!();
    println!("The convertsToInteger() function tests if a value can be converted:");
    println!("- Returns true if toInteger() would return a value");
    println!("- Returns false if toInteger() would return empty");

    Ok(())
}
