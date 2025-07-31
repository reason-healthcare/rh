//! Example demonstrating date/time component extraction functions
//!
//! This example shows how to use FHIRPath's component extraction functions
//! to extract specific parts from Date, DateTime, and Time values.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("FHIRPath Date/Time Component Extraction Functions Example");
    println!("=========================================================\n");

    // Sample date, time, and datetime values (without milliseconds - not supported in parser)
    let sample_date = "@2023-07-15";
    let sample_datetime = "@2023-07-15T14:30:25Z";
    let sample_datetime_tz = "@2023-12-25T09:45:30-05:00";
    let sample_time = "@T14:30:25";

    println!("Sample Values:");
    println!("  Date: {sample_date}");
    println!("  DateTime (UTC): {sample_datetime}");
    println!("  DateTime (EST): {sample_datetime_tz}");
    println!("  Time: {sample_time}");
    println!();

    // Date component extraction
    println!("📅 Date Component Extraction");
    println!("----------------------------");

    let expressions = vec![
        (format!("{sample_date}.yearOf()"), "Year from date"),
        (format!("{sample_date}.monthOf()"), "Month from date"),
        (format!("{sample_date}.dayOf()"), "Day from date"),
        (format!("{sample_datetime}.yearOf()"), "Year from datetime"),
        (
            format!("{sample_datetime}.monthOf()"),
            "Month from datetime",
        ),
        (format!("{sample_datetime}.dayOf()"), "Day from datetime"),
    ];

    for (expr, description) in expressions {
        match parser.parse(&expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    let value = format_result(&result);
                    println!("  {description}: {value}");
                }
                Err(e) => println!("  {description}: Error - {e}"),
            },
            Err(e) => println!("  {description}: Parse Error - {e}"),
        }
    }
    println!();

    // Time component extraction
    println!("⏰ Time Component Extraction");
    println!("----------------------------");

    let time_expressions = vec![
        (format!("{sample_time}.hourOf()"), "Hour from time"),
        (format!("{sample_time}.minuteOf()"), "Minute from time"),
        (format!("{sample_time}.secondOf()"), "Second from time"),
        (
            format!("{sample_time}.millisecondOf()"),
            "Millisecond from time",
        ),
        (format!("{sample_datetime}.hourOf()"), "Hour from datetime"),
        (
            format!("{sample_datetime}.minuteOf()"),
            "Minute from datetime",
        ),
        (
            format!("{sample_datetime}.secondOf()"),
            "Second from datetime",
        ),
        (
            format!("{sample_datetime}.millisecondOf()"),
            "Millisecond from datetime",
        ),
    ];

    for (expr, description) in time_expressions {
        match parser.parse(&expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    let value = format_result(&result);
                    println!("  {description}: {value}");
                }
                Err(e) => println!("  {description}: Error - {e}"),
            },
            Err(e) => println!("  {description}: Parse Error - {e}"),
        }
    }
    println!();

    // Timezone and component extraction
    println!("🌍 Timezone and Component Extraction");
    println!("------------------------------------");

    let tz_expressions = vec![
        (
            format!("{sample_datetime}.timezoneOffsetOf()"),
            "UTC timezone offset",
        ),
        (
            format!("{sample_datetime_tz}.timezoneOffsetOf()"),
            "EST timezone offset",
        ),
        (
            format!("{sample_datetime}.dateOf()"),
            "Date part from datetime",
        ),
        (
            format!("{sample_datetime}.timeOf()"),
            "Time part from datetime",
        ),
        (
            format!("{sample_datetime_tz}.dateOf()"),
            "Date part from EST datetime",
        ),
        (
            format!("{sample_datetime_tz}.timeOf()"),
            "Time part from EST datetime",
        ),
    ];

    for (expr, description) in tz_expressions {
        match parser.parse(&expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    let value = format_result(&result);
                    println!("  {description}: {value}");
                }
                Err(e) => println!("  {description}: Error - {e}"),
            },
            Err(e) => println!("  {description}: Parse Error - {e}"),
        }
    }
    println!();

    // Complex expressions combining components
    println!("🔗 Complex Component Operations");
    println!("------------------------------");

    let complex_expressions = vec![
        ("now().yearOf() > 2020", "Current year is after 2020"),
        (
            "today().monthOf() >= 1 and today().monthOf() <= 12",
            "Current month is valid",
        ),
        ("now().dateOf() = today()", "Date part of now equals today"),
        (
            "@2023-07-15T14:30:25Z.yearOf() + @2023-07-15T14:30:25Z.monthOf()",
            "Year + month",
        ),
        (
            "timeOfDay().hourOf() >= 0 and timeOfDay().hourOf() <= 23",
            "Current hour is valid",
        ),
    ];

    for (expr, description) in complex_expressions {
        match parser.parse(expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    let value = format_result(&result);
                    println!("  {description}: {value}");
                }
                Err(e) => println!("  {description}: Error - {e}"),
            },
            Err(e) => println!("  {description}: Parse Error - {e}"),
        }
    }
    println!();

    // Error cases
    println!("❌ Error Cases");
    println!("--------------");

    let error_expressions = vec![
        ("'not a date'.yearOf()", "yearOf on string"),
        ("123.hourOf()", "hourOf on number"),
        (
            "@2023-07-15.timezoneOffsetOf()",
            "timezoneOffsetOf on date (no timezone)",
        ),
        ("@T14:30:25.dateOf()", "dateOf on time (no date part)"),
        ("true.millisecondOf()", "millisecondOf on boolean"),
    ];

    for (expr, description) in error_expressions {
        match parser.parse(expr) {
            Ok(parsed) => match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    let value = format_result(&result);
                    println!("  {description}: {value} (unexpected success)");
                }
                Err(e) => println!("  {description}: Expected error - {e}"),
            },
            Err(e) => println!("  {description}: Parse Error - {e}"),
        }
    }

    println!("\n✅ Component extraction functions allow precise extraction of date/time parts!");
    println!("   These functions are essential for date/time validation and manipulation in FHIR expressions.");

    Ok(())
}

/// Format a FhirPathValue result for display
fn format_result(result: &FhirPathValue) -> String {
    match result {
        FhirPathValue::Empty => "empty".to_string(),
        FhirPathValue::Collection(items) => {
            if items.is_empty() {
                "empty collection".to_string()
            } else if items.len() == 1 {
                format!("{:?}", items[0])
            } else {
                format!(
                    "[{}]",
                    items
                        .iter()
                        .map(|v| format!("{v:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        _ => format!("{result:?}"),
    }
}
