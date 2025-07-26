//! Debug union expression evaluation
//!
//! This test helps debug why (10 | 20 | 30)[0] fails

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debugging Union Expression Indexing");
    println!("=======================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test the union expression first
    println!("Step 1: Test (10 | 20 | 30)");
    match parser.parse("(10 | 20 | 30)") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ (10 | 20 | 30) = {result:?}");
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test indexing the union expression
    println!("\nStep 2: Test (10 | 20 | 30)[0]");
    match parser.parse("(10 | 20 | 30)[0]") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ (10 | 20 | 30)[0] = {result:?}");
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test empty collection
    println!("\nStep 3: Test {{}}");
    match parser.parse("{}") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ {{}} = {result:?}");
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test indexing empty collection
    println!("\nStep 4: Test {{}}[0]");
    match parser.parse("{}[0]") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ {{}}[0] = {result:?}");
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    Ok(())
}
