//! Example showcasing temperature unit conversion in FHIRPath
//!
//! This example demonstrates the temperature conversion capabilities
//! including Kelvin, Celsius, and Fahrenheit units with automatic
//! conversion during arithmetic operations.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FHIRPath Temperature Unit Conversion Example");
    println!("===========================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Temperature literals
    println!("🌡️  Temperature Literals:");
    evaluate_and_print(&parser, &evaluator, &context, "37.0'Cel'")?;
    evaluate_and_print(&parser, &evaluator, &context, "98.6'[degF]'")?;
    evaluate_and_print(&parser, &evaluator, &context, "310.15'K'")?;

    // Same unit arithmetic
    println!("\n🧮 Same Unit Arithmetic:");
    evaluate_and_print(&parser, &evaluator, &context, "20.0'Cel' + 5.0'Cel'")?;
    evaluate_and_print(
        &parser,
        &evaluator,
        &context,
        "100.0'[degF]' - 32.0'[degF]'",
    )?;

    // Cross-unit conversions
    println!("\n🔄 Cross-Unit Conversions:");
    evaluate_and_print(&parser, &evaluator, &context, "0.0'Cel' + 273.15'K'")?;
    evaluate_and_print(&parser, &evaluator, &context, "32.0'[degF]' + 0.0'Cel'")?;

    // Scalar operations
    println!("\n✖️  Scalar Operations:");
    evaluate_and_print(&parser, &evaluator, &context, "100.0'Cel' * 2.0")?;
    evaluate_and_print(&parser, &evaluator, &context, "212.0'[degF]' / 2.0")?;

    // Division of compatible units (dimensionless result)
    println!("\n➗ Division (Dimensionless Results):");
    evaluate_and_print(&parser, &evaluator, &context, "100.0'Cel' / 50.0'Cel'")?;
    evaluate_and_print(&parser, &evaluator, &context, "300.0'K' / 273.15'K'")?;

    println!("\n📝 Notes:");
    println!("   • Temperature arithmetic converts to Celsius (base unit), performs operation, then converts back");
    println!("   • Result unit matches the left operand's unit");
    println!("   • Celsius (Cel), Kelvin (K), and Fahrenheit ([degF]) are supported");
    println!("   • Temperature addition now works intuitively: 20°C + 5°C = 25°C");
    println!("   • Cross-unit conversions: 0°C + 273.15K = 0°C (273.15K equals 0°C)");

    Ok(())
}

fn evaluate_and_print(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, context) {
            Ok(FhirPathValue::Collection(values)) if values.len() == 1 => {
                println!("   {expression} => {:?}", values[0]);
            }
            Ok(value) => {
                println!("   {expression} => {value:?}");
            }
            Err(e) => {
                println!("   {expression} => ERROR: {e}");
            }
        },
        Err(e) => {
            println!("   {expression} => PARSE ERROR: {e}");
        }
    }
    Ok(())
}
