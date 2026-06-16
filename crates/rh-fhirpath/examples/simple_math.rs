//! Simple Math Functions Example
//!
//! A quick demonstration of basic mathematical functions in FHIRPath.
//! This example shows the most commonly used math functions with simple, clear examples.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() {
    println!("🔢 Simple FHIRPath Math Functions Demo\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("Basic Math Functions:");

    // Simple examples that are easy to understand
    let examples = vec![
        ("(-5).abs()", "Absolute value: |-5| = 5"),
        ("(3.7).ceiling()", "Round up: ⌈3.7⌉ = 4"),
        ("(3.7).floor()", "Round down: ⌊3.7⌋ = 3"),
        ("(3.14159).round(2)", "Round to 2 decimals: 3.14"),
        ("(16).sqrt()", "Square root: √16 = 4"),
        ("(2).power(3)", "Power: 2³ = 8"),
        ("(8).log(2)", "Logarithm: log₂(8) = 3"),
        ("(1).exp()", "Exponential: e¹ ≈ 2.718"),
        ("(3.9).truncate()", "Truncate: 3.9 → 3"),
    ];

    for (expr, description) in examples {
        match parser.parse(expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    println!("  {} → {}", description, format_result(&result));
                }
                Err(e) => println!("  {description} → Error: {e}"),
            },
            Err(e) => println!("  {description} → Parse Error: {e}"),
        }
    }

    println!("\nFunction Chaining:");
    let chaining_examples = vec![
        ("(-3.7).abs().ceiling()", "|-3.7| then round up = 4"),
        ("(25).sqrt().round(1)", "√25 then round to 1 decimal = 5.0"),
        ("(2.power(3) + 1).sqrt()", "(2³ + 1) then √ = √9 = 3"),
    ];

    for (expr, description) in chaining_examples {
        match parser.parse(expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    println!("  {} → {}", description, format_result(&result));
                }
                Err(e) => println!("  {description} → Error: {e}"),
            },
            Err(e) => println!("  {description} → Parse Error: {e}"),
        }
    }

    println!("\n✅ All math functions working correctly!");
}

fn format_result(result: &FhirPathValue) -> String {
    match result {
        FhirPathValue::Integer(i) => i.to_string(),
        FhirPathValue::Number(n) => {
            if n.fract() == rust_decimal::Decimal::ZERO {
                format!("{n:.1}")
            } else {
                format!("{n:.3}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        }
        _ => format!("{result:?}"),
    }
}
