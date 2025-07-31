//! FHIRPath Unit Conversion Example
//!
//! This example demonstrates the enhanced unit conversion capabilities
//! of the FHIRPath library using a custom unit conversion system.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("FHIRPath Unit Conversion Examples");
    println!("=================================\n");

    // Mass unit conversions
    println!("Mass Unit Conversions:");

    let expr = parser.parse("1.0'kg' + 500.0'g'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("1.0 kg + 500.0 g = {result:?}");

    let expr = parser.parse("2000.0'mg' + 3.0'g'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("2000.0 mg + 3.0 g = {result:?}");

    // Length unit conversions
    println!("\nLength Unit Conversions:");

    let expr = parser.parse("1.5'm' + 50.0'cm'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("1.5 m + 50.0 cm = {result:?}");

    let expr = parser.parse("12.0'[in_i]' + 1.0'ft'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("12.0 inches + 1.0 ft = {result:?}");

    // Volume unit conversions
    println!("\nVolume Unit Conversions:");

    let expr = parser.parse("2.0'L' + 250.0'mL'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("2.0 L + 250.0 mL = {result:?}");

    // Time unit conversions
    println!("\nTime Unit Conversions:");

    let expr = parser.parse("1.0'h' + 30.0'min'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("1.0 h + 30.0 min = {result:?}");

    let expr = parser.parse("2.0'd' + 12.0'h'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("2.0 d + 12.0 h = {result:?}");

    // Unit division (creates dimensionless ratios)
    println!("\nUnit Division (Dimensionless Ratios):");

    let expr = parser.parse("10.0'kg' / 2.0'kg'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("10.0 kg / 2.0 kg = {result:?}");

    let expr = parser.parse("1.0'kg' / 500.0'g'")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("1.0 kg / 500.0 g = {result:?}");

    // Scalar operations
    println!("\nScalar Operations:");

    let expr = parser.parse("5.0'mg' * 3.0")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("5.0 mg * 3.0 = {result:?}");

    let expr = parser.parse("100.0'mL' / 4.0")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("100.0 mL / 4.0 = {result:?}");

    // Error cases
    println!("\nError Cases:");

    match parser.parse("1.0'kg' + 1.0'm'") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => println!("1.0 kg + 1.0 m = {result:?} (unexpected!)"),
            Err(e) => println!("1.0 kg + 1.0 m = Error: {e}"),
        },
        Err(e) => println!("Parse error: {e}"),
    }

    Ok(())
}
