//! Quantity Conversion Examples
//!
//! This example demonstrates the toQuantity() and convertsToQuantity() functions
//! in FHIRPath. These functions allow converting various value types to Quantity
//! types with optional unit specifications.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚖️  FHIRPath Quantity Conversion Examples\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // === Basic Quantity Conversions ===
    println!("📏 Basic toQuantity() Conversions:");

    // Convert numbers to quantities without units
    let expr = parser.parse("42.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.toQuantity() → {result:?}");

    let expr = parser.parse("3.14.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("3.14.toQuantity() → {result:?}");

    let expr = parser.parse("100L.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("100L.toQuantity() → {result:?}");

    println!();

    // === Quantity Conversions with Unit Parameters ===
    println!("🏷️  toQuantity() with Unit Parameters:");

    let expr = parser.parse("5.toQuantity('mg')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("5.toQuantity('mg') → {result:?}");

    let expr = parser.parse("37.2.toQuantity('Cel')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("37.2.toQuantity('Cel') → {result:?}");

    let expr = parser.parse("120.toQuantity('mm[Hg]')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("120.toQuantity('mm[Hg]') → {result:?}");

    let expr = parser.parse("2.5.toQuantity('L')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("2.5.toQuantity('L') → {result:?}");

    println!();

    // === String to Quantity Conversions ===
    println!("📝 String to Quantity Conversions:");

    let expr = parser.parse("'42.7'.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'42.7'.toQuantity() → {result:?}");

    let expr = parser.parse("'98.6'.toQuantity('[degF]')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'98.6'.toQuantity('[degF]') → {result:?}");

    let expr = parser.parse("'0.5'.toQuantity('kg')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'0.5'.toQuantity('kg') → {result:?}");

    println!();

    // === Existing Quantity Conversions (Unit Override) ===
    println!("🔄 Existing Quantity Unit Override:");

    let expr = parser.parse("5'mg'.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("5'mg'.toQuantity() → {result:?}");

    let expr = parser.parse("5'mg'.toQuantity('g')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("5'mg'.toQuantity('g') → {result:?}");

    let expr = parser.parse("100'mmHg'.toQuantity('Pa')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("100'mmHg'.toQuantity('Pa') → {result:?}");

    println!();

    // === Conversion Compatibility Testing ===
    println!("✅ convertsToQuantity() Tests:");

    let test_cases = [
        "42.convertsToQuantity()",
        "3.14.convertsToQuantity()",
        "100L.convertsToQuantity()",
        "'123.45'.convertsToQuantity()",
        "'not-a-number'.convertsToQuantity()",
        "true.convertsToQuantity()",
        "5'mg'.convertsToQuantity()",
        "@2023-01-15.convertsToQuantity()",
        "{}.convertsToQuantity()",
        "(1 | 2).convertsToQuantity()",
    ];

    for example in test_cases {
        let expr = parser.parse(example)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("{example} → {result:?}");
    }

    println!();

    // === Edge Cases and Error Handling ===
    println!("⚠️  Edge Cases:");

    let expr = parser.parse("'invalid'.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'invalid'.toQuantity() → {result:?}");

    let expr = parser.parse("true.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("true.toQuantity() → {result:?}");

    let expr = parser.parse("{}.toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("{{}}.toQuantity() → {result:?}");

    let expr = parser.parse("(1 | 2).toQuantity()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(1 | 2).toQuantity() → {result:?}");

    println!();

    // === Healthcare-Specific Examples ===
    println!("🏥 Healthcare Examples:");

    // Convert vital signs to quantities
    let expr = parser.parse("'98.6'.toQuantity('[degF]')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert temperature: '98.6'.toQuantity('[degF]') → {result:?}");

    let expr = parser.parse("'75'.toQuantity('bpm')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert heart rate: '75'.toQuantity('bpm') → {result:?}");

    let expr = parser.parse("'65'.toQuantity('kg')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert weight: '65'.toQuantity('kg') → {result:?}");

    let expr = parser.parse("'175'.toQuantity('cm')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert height: '175'.toQuantity('cm') → {result:?}");

    println!();

    // === Medication Dosage Examples ===
    println!("💊 Medication Dosage Examples:");

    let expr = parser.parse("10.toQuantity('mg')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Dosage amount: 10.toQuantity('mg') → {result:?}");

    let expr = parser.parse("'2.5'.toQuantity('mL')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Liquid volume: '2.5'.toQuantity('mL') → {result:?}");

    let expr = parser.parse("500.toQuantity('mg/day')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Daily dose: 500.toQuantity('mg/day') → {result:?}");

    println!();

    // === Lab Result Examples ===
    println!("🧪 Laboratory Result Examples:");

    let expr = parser.parse("'7.4'.toQuantity('pH')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Blood pH: '7.4'.toQuantity('pH') → {result:?}");

    let expr = parser.parse("'145'.toQuantity('mEq/L')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Sodium level: '145'.toQuantity('mEq/L') → {result:?}");

    let expr = parser.parse("'10.5'.toQuantity('g/dL')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Hemoglobin: '10.5'.toQuantity('g/dL') → {result:?}");

    println!();

    // === Practical Usage Patterns ===
    println!("🔧 Practical Usage:");

    // Check if a value can be converted before conversion
    let check_and_convert = [
        ("'42.5'", "'kg'"),
        ("'not-a-number'", "'mg'"),
        ("123", "'L'"),
        ("true", "'°C'"),
    ];

    for (value, unit) in check_and_convert {
        let check_expr = parser.parse(&format!("{value}.convertsToQuantity()"))?;
        let check_result = evaluator.evaluate(&check_expr, &context)?;

        println!("Can convert {value}? → {check_result:?}");

        if let FhirPathValue::Boolean(true) = check_result {
            let convert_expr = parser.parse(&format!("{value}.toQuantity({unit})"))?;
            let convert_result = evaluator.evaluate(&convert_expr, &context)?;
            println!("  Converting: {value}.toQuantity({unit}) → {convert_result:?}");
        } else {
            println!("  Conversion would return empty");
        }
    }

    println!();

    // === Unit Substitution ===
    println!("🔄 Unit Substitution Examples:");

    let expr = parser.parse("37.2'Cel'.toQuantity('[degF]')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Change temperature units: 37.2'Cel'.toQuantity('[degF]') → {result:?}");

    let expr = parser.parse("1000'g'.toQuantity('kg')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Change mass units: 1000'g'.toQuantity('kg') → {result:?}");

    let expr = parser.parse("2.5'L'.toQuantity('mL')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Change volume units: 2.5'L'.toQuantity('mL') → {result:?}");

    println!();
    println!("✨ Quantity conversion functions provide flexible type conversion");
    println!("   with optional unit specification for healthcare calculations!");

    Ok(())
}
