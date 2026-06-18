//! Time Conversion Functions Example
//!
//! This example demonstrates the toTime() and convertsToTime() functions
//! in FHIRPath, showing how to convert various value types to Time and
//! check if values can be converted to Time.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("=== FHIRPath Time Conversion Functions ===\n");

    // === toTime() Examples ===
    println!("📝 toTime() Function Examples:\n");

    // Time to Time (identity)
    println!("🔄 Time to Time (Identity):");
    let expr = parser.parse("@T10:30:45.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@T10:30:45.toTime() → {result:?}");

    let expr = parser.parse("@T23:59:59.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@T23:59:59.toTime() → {result:?}");
    println!();

    // String to Time
    println!("📄 String to Time:");
    let expr = parser.parse("'10:30:45'.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'10:30:45'.toTime() → {result:?}");

    let expr = parser.parse("'23:59:59.123'.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'23:59:59.123'.toTime() → {result:?}");

    let expr = parser.parse("'00:00:00'.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'00:00:00'.toTime() → {result:?}");

    let expr = parser.parse("'T14:25:36'.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'T14:25:36'.toTime() → {result:?}");
    println!();

    // DateTime to Time (extract time part)
    println!("📅 DateTime to Time (Extract Time Part):");
    let expr = parser.parse("@2023-01-15T10:30:45.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.toTime() → {result:?}");

    let expr = parser.parse("@2023-01-15T23:59:59Z.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T23:59:59Z.toTime() → {result:?}");

    let expr = parser.parse("@2023-01-15T14:25:36+05:30.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T14:25:36+05:30.toTime() → {result:?}");
    println!();

    // Invalid time conversions (should return empty)
    println!("❌ Invalid Time Conversions (Return Empty):");
    let invalid_time_examples = vec![
        "'not-a-time'",
        "'25:00:00'",
        "'12:60:00'",
        "'12:30:60'",
        "42",
        "true",
        "@2023-01-15",
    ];

    for example in invalid_time_examples {
        let expr = parser.parse(&format!("{example}.toTime()"))?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("{example}.toTime() → {result:?}");
    }
    println!();

    // === convertsToTime() Examples ===
    println!("✅ convertsToTime() Function Examples:\n");

    println!("🔍 Testing Time Conversion Compatibility:");

    // Types that can convert to Time
    let convertible_examples = vec![
        "@T10:30:45",
        "'10:30:45'",
        "'23:59:59.123'",
        "'T14:25:36'",
        "@2023-01-15T10:30:45",
        "@2023-01-15T23:59:59Z",
    ];

    println!("✅ Convertible to Time:");
    for type_expr in convertible_examples {
        let expr = parser.parse(&format!("{type_expr}.convertsToTime()"))?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("{type_expr}.convertsToTime() → {result:?}");
    }
    println!();

    println!("❌ Not Convertible to Time:");
    let non_convertible_examples = vec![
        "42",
        "true",
        "3.14",
        "'not-a-time'",
        "'25:00:00'",
        "'12:60:00'",
        "@2023-01-15",
        "5'mg'",
    ];

    for type_expr in non_convertible_examples {
        let expr = parser.parse(&format!("{type_expr}.convertsToTime()"))?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("{type_expr}.convertsToTime() → {result:?}");
    }
    println!();

    // === Edge Cases ===
    println!("🔬 Edge Cases:\n");

    // Empty collection
    let expr = parser.parse("{}.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("{{}}.toTime() → {result:?}");

    let expr = parser.parse("{}.convertsToTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("{{}}.convertsToTime() → {result:?}");
    println!();

    // Multiple items should error for singleton conversion functions.
    let expr = parser.parse("('10:30:45' | '14:25:36').toTime()")?;
    let result = evaluator.evaluate(&expr, &context);
    println!("('10:30:45' | '14:25:36').toTime() → {result:?}");

    let expr = parser.parse("('10:30:45' | '14:25:36').convertsToTime()")?;
    let result = evaluator.evaluate(&expr, &context);
    println!("('10:30:45' | '14:25:36').convertsToTime() → {result:?}");
    println!();

    // Single item collection (should work)
    let expr = parser.parse("('10:30:45').toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("('10:30:45').toTime() → {result:?}");
    println!();

    // === Real-world Examples ===
    println!("🌍 Real-world Examples:\n");

    // Extract time from appointment datetime
    let expr = parser.parse("@2023-12-25T09:30:00.toTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Extract appointment time: @2023-12-25T09:30:00.toTime() → {result:?}");

    // Validate time input strings
    let expr = parser.parse("'08:45:30'.convertsToTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Validate time input: '08:45:30'.convertsToTime() → {result:?}");

    // Convert mixed time formats
    let expr = parser.parse(
        "('10:30:45' | @T14:25:36 | @2023-01-15T16:45:30).where($this.convertsToTime()).select(toTime())",
    )?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert mixed time formats: {result:?}");
    println!();

    // === Conditional Time Processing ===
    println!("🔄 Conditional Time Processing:\n");

    // Use iif with time conversion
    let expr =
        parser.parse("''.iif('14:30:00'.convertsToTime(), '14:30:00'.toTime(), 'Invalid time')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Check convertibility then convert: {result:?}");

    // Time formatting with string conversion
    let expr = parser.parse("('08:15:30'.toTime().toString() & ' UTC')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Format time as string: {result:?}");

    println!("\n=== Summary ===");
    println!("✅ Time conversion functions implemented successfully!");
    println!("• toTime(): Converts Time, String, and DateTime to Time");
    println!("• convertsToTime(): Tests if values can be converted to Time");
    println!("• Supports various time formats: HH:MM:SS, HH:MM:SS.sss");
    println!("• Extracts time component from DateTime values");
    println!("• Validates time ranges (00:00:00 to 23:59:59)");
    println!("• Handles fractional seconds (up to 3 digits)");

    Ok(())
}
