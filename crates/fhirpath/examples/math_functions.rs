//! Mathematical Functions Example
//!
//! This example demonstrates the comprehensive math function support in FHIRPath expressions.
//! Math functions are essential for healthcare calculations like BMI, dosage calculations,
//! statistical analysis, and data normalization.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() {
    println!("🔢 FHIRPath Mathematical Functions Examples\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // === Basic Mathematical Functions ===
    println!("📊 Basic Mathematical Functions:");

    // Absolute value - useful for distance calculations
    demo_math(
        &parser,
        &evaluator,
        "(-5.7).abs()",
        "Absolute value of -5.7",
    );
    demo_math(&parser, &evaluator, "(-42).abs()", "Absolute value of -42");

    // Rounding functions - essential for displaying results
    demo_math(
        &parser,
        &evaluator,
        "(3.14159).round(2)",
        "Round π to 2 decimal places",
    );
    demo_math(
        &parser,
        &evaluator,
        "(3.7).round()",
        "Round 3.7 to nearest integer",
    );
    demo_math(
        &parser,
        &evaluator,
        "(2.3).ceiling()",
        "Round 2.3 up (ceiling)",
    );
    demo_math(
        &parser,
        &evaluator,
        "(2.9).floor()",
        "Round 2.9 down (floor)",
    );
    demo_math(
        &parser,
        &evaluator,
        "(3.7).truncate()",
        "Truncate 3.7 (toward zero)",
    );
    demo_math(
        &parser,
        &evaluator,
        "(-2.9).truncate()",
        "Truncate -2.9 (toward zero)",
    );

    // Power and root functions - for formulas and calculations
    demo_math(&parser, &evaluator, "(2).power(3)", "2 to the power of 3");
    demo_math(&parser, &evaluator, "(16).sqrt()", "Square root of 16");
    demo_math(&parser, &evaluator, "(2.25).sqrt()", "Square root of 2.25");

    // Logarithmic functions - for scientific calculations
    demo_math(&parser, &evaluator, "(8).log(2)", "Log base 2 of 8");
    demo_math(
        &parser,
        &evaluator,
        "(100).log(10)",
        "Log base 10 of 100 (log₁₀)",
    );
    demo_math(
        &parser,
        &evaluator,
        "(2.718281828).ln()",
        "Natural logarithm of e",
    );

    // Exponential function - for growth calculations
    demo_math(&parser, &evaluator, "(1).exp()", "e to the power of 1");
    demo_math(&parser, &evaluator, "(0).exp()", "e to the power of 0");

    println!("\n🧮 Function Chaining Examples:");

    // Complex calculations using function chaining
    demo_math(
        &parser,
        &evaluator,
        "(-3.7).abs().ceiling()",
        "|-3.7| rounded up",
    );
    demo_math(
        &parser,
        &evaluator,
        "(25).sqrt().round(1)",
        "√25 rounded to 1 decimal",
    );
    demo_math(
        &parser,
        &evaluator,
        "(2.power(3) + 1).sqrt()",
        "√(2³ + 1) = √9",
    );
    demo_math(&parser, &evaluator, "(-10).abs().log(10)", "log₁₀(|-10|)");

    // === Statistical Calculations ===
    println!("\n📈 Statistical Examples:");

    let lab_data = json!({
        "glucose_readings": [95, 108, 112, 98, 105, 110, 92, 103],
        "reference_range": {"min": 70, "max": 99}
    });

    let lab_context = EvaluationContext::new(lab_data);

    // Calculate mean glucose (simplified - would need sum function in real implementation)
    // For demo, showing how math functions would be used
    println!("Glucose readings: [95, 108, 112, 98, 105, 110, 92, 103]");

    // Standard deviation calculation components (simplified)
    demo_math_with_context(
        &parser,
        &evaluator,
        &lab_context,
        "glucose_readings[0].power(2).sqrt()",
        "√(95²) = 95",
    );

    // Check if reading is within normal range using abs
    demo_math_with_context(
        &parser,
        &evaluator,
        &lab_context,
        "(glucose_readings[0] - reference_range.max).abs()",
        "Distance from upper limit",
    );

    // === Dosage Calculations ===
    println!("\n💊 Medication Dosage Examples:");

    let medication_data = json!({
        "patient": {
            "weight_kg": 75,
            "age_years": 45,
            "creatinine": 1.2
        },
        "medication": {
            "standard_dose_mg_per_kg": 5,
            "max_daily_dose_mg": 500
        }
    });

    let med_context = EvaluationContext::new(medication_data);

    // Weight-based dosing
    let expr = parser
        .parse("patient.weight_kg * medication.standard_dose_mg_per_kg")
        .unwrap();
    let result = evaluator.evaluate(&expr, &med_context).unwrap();
    println!(
        "  Weight-based dose: {}mg (75kg × 5mg/kg)",
        format_result(&result)
    );

    // Ensure dose doesn't exceed maximum
    let expr = parser
        .parse("(patient.weight_kg * medication.standard_dose_mg_per_kg).power(1)")
        .unwrap();
    let calculated_dose = evaluator.evaluate(&expr, &med_context).unwrap();
    let expr = parser.parse("medication.max_daily_dose_mg").unwrap();
    let max_dose = evaluator.evaluate(&expr, &med_context).unwrap();

    println!(
        "  Calculated: {}mg, Maximum: {}mg",
        format_result(&calculated_dose),
        format_result(&max_dose)
    );

    // === Advanced Mathematical Expressions ===
    println!("\n🔬 Advanced Mathematical Expressions:");

    // Complex formula combining multiple functions
    demo_math(
        &parser,
        &evaluator,
        "(((2.power(3) * 3) + 5).sqrt() - 1).round(2)",
        "Complex: √((2³×3)+5) - 1",
    );

    // Error handling examples
    println!("\n⚠️  Error Handling Examples:");

    // These will demonstrate error handling (should fail gracefully)
    demo_math_error(
        &parser,
        &evaluator,
        "(-1).sqrt()",
        "Square root of negative number",
    );
    demo_math_error(
        &parser,
        &evaluator,
        "(-5).ln()",
        "Natural log of negative number",
    );
    demo_math_error(
        &parser,
        &evaluator,
        "(10).log(1)",
        "Log with invalid base (1)",
    );
    demo_math_error(
        &parser,
        &evaluator,
        "(10).log(0)",
        "Log with invalid base (0)",
    );

    println!("\n✅ Math functions demonstration complete!");
    println!("These examples show how FHIRPath math functions can be used for:");
    println!("  • Statistical analysis (mean, standard deviation components)");
    println!("  • Medication dosage calculations");
    println!("  • Complex mathematical expressions");
    println!("  • Robust error handling for invalid operations");
}

fn demo_math(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    expression: &str,
    description: &str,
) {
    let empty_context = EvaluationContext::new(json!({}));
    demo_math_with_context(parser, evaluator, &empty_context, expression, description);
}

fn demo_math_with_context(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
    description: &str,
) {
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, context) {
            Ok(result) => {
                println!(
                    "  {}: {} = {}",
                    description,
                    expression,
                    format_result(&result)
                );
            }
            Err(e) => {
                println!("  {description}: {expression} → ERROR: {e}");
            }
        },
        Err(e) => {
            println!("  {description}: {expression} → PARSE ERROR: {e}");
        }
    }
}

fn demo_math_error(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    expression: &str,
    description: &str,
) {
    let empty_context = EvaluationContext::new(json!({}));
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, &empty_context) {
            Ok(result) => {
                println!(
                    "  {description}: {expression} = {} (unexpected success)",
                    format_result(&result)
                );
            }
            Err(e) => {
                println!("  {description}: {expression} → ✓ Correctly handled error: {e}");
            }
        },
        Err(e) => {
            println!("  {description}: {expression} → Parse error: {e}");
        }
    }
}

fn format_result(result: &FhirPathValue) -> String {
    match result {
        FhirPathValue::Integer(i) => i.to_string(),
        FhirPathValue::Number(n) => {
            if n.fract() == 0.0 {
                format!("{n:.1}")
            } else {
                format!("{n:.6}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        }
        _ => format!("{result:?}"),
    }
}
