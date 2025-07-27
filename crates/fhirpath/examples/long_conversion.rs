//! Example demonstrating Long type conversion functions
//!
//! This example shows how to use toLong() and convertsToLong() functions
//! with different input types, including the explicit Long literal syntax.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("=== Long Type Conversion Examples ===\n");

    // Long literals with L suffix
    println!("1. Long Literals:");
    evaluate_and_print(&parser, &evaluator, &context, "42L");
    evaluate_and_print(&parser, &evaluator, &context, "9223372036854775807L"); // i64::MAX
    println!();

    // Boolean to Long conversions
    println!("2. Boolean → Long:");
    evaluate_and_print(&parser, &evaluator, &context, "true.toLong()");
    evaluate_and_print(&parser, &evaluator, &context, "false.toLong()");
    println!();

    // Integer to Long conversions
    println!("3. Integer → Long:");
    evaluate_and_print(&parser, &evaluator, &context, "42.toLong()");
    evaluate_and_print(&parser, &evaluator, &context, "(-123).toLong()");
    println!();

    // String to Long conversions
    println!("4. String → Long:");
    evaluate_and_print(&parser, &evaluator, &context, "'123'.toLong()");
    evaluate_and_print(&parser, &evaluator, &context, "'-456'.toLong()");
    evaluate_and_print(&parser, &evaluator, &context, "'not a number'.toLong()"); // Returns empty
    evaluate_and_print(&parser, &evaluator, &context, "'  789  '.toLong()"); // Handles whitespace
    println!();

    // Number to Long conversions
    println!("5. Number → Long:");
    evaluate_and_print(&parser, &evaluator, &context, "42.0.toLong()"); // Whole number
    evaluate_and_print(&parser, &evaluator, &context, "42.5.toLong()"); // Fractional (returns empty)
    evaluate_and_print(&parser, &evaluator, &context, "(-100.0).toLong()");
    println!();

    // Test conversion compatibility with convertsToLong()
    println!("6. Conversion Tests (convertsToLong()):");
    evaluate_and_print(&parser, &evaluator, &context, "42.convertsToLong()");
    evaluate_and_print(&parser, &evaluator, &context, "true.convertsToLong()");
    evaluate_and_print(&parser, &evaluator, &context, "'123'.convertsToLong()");
    evaluate_and_print(&parser, &evaluator, &context, "'abc'.convertsToLong()");
    evaluate_and_print(&parser, &evaluator, &context, "42.0.convertsToLong()");
    evaluate_and_print(&parser, &evaluator, &context, "42.5.convertsToLong()");
    println!();

    // Empty collection handling
    println!("7. Empty Collection Handling:");
    evaluate_and_print(&parser, &evaluator, &context, "{}.toLong()"); // Empty returns false for convertsToLong
    evaluate_and_print(&parser, &evaluator, &context, "{}.convertsToLong()");
    println!();

    // Multiple items (should return empty/false)
    println!("8. Multiple Items (returns Empty/false):");
    evaluate_and_print(&parser, &evaluator, &context, "(1 | 2).toLong()");
    evaluate_and_print(&parser, &evaluator, &context, "(1 | 2).convertsToLong()");
}

fn evaluate_and_print(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
) {
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, context) {
            Ok(result) => {
                let result_str = match result {
                    FhirPathValue::Long(l) => format!("Long({})", l),
                    FhirPathValue::Integer(i) => format!("Integer({})", i),
                    FhirPathValue::Boolean(b) => format!("Boolean({})", b),
                    FhirPathValue::Empty => "Empty".to_string(),
                    other => format!("{:?}", other),
                };
                println!("  {} → {}", expression, result_str);
            }
            Err(e) => println!("  {} → Error: {}", expression, e),
        },
        Err(e) => println!("  {} → Parse Error: {}", expression, e),
    }
}
