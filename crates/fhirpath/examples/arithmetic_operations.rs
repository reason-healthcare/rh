/// FHIRPath - Arithmetic Operations Example
///
/// This example demonstrates mathematical operations in FHIRPath expressions.
use anyhow::Result;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("🧮 FHIRPath Arithmetic Operations Examples");
    println!("==========================================");

    // Example 1: Basic addition
    println!("\n1️⃣ Addition Examples:");
    let expressions = vec![
        "2 + 3",  // Integer addition
        "10 + 0", // Adding zero
        "5 + 10", // Positive numbers
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // Example 2: Subtraction
    println!("\n2️⃣ Subtraction Examples:");
    let expressions = vec![
        "10 - 3",    // Basic subtraction
        "5 - 10",    // Negative result
        "100 - 100", // Zero result
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // Example 3: Literal values
    println!("\n3️⃣ Literal Value Examples:");
    let expressions = vec![
        "42",      // Integer literal
        "true",    // Boolean literal
        "'hello'", // String literal
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    println!("\n✅ Arithmetic examples completed!");
    Ok(())
}
