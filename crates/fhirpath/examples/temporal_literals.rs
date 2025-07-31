//! Demonstrates FHIRPath temporal literal support
//!
//! This example shows how to parse and work with date, datetime, and time literals in FHIRPath expressions.

use rh_fhirpath::FhirPathParser;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();

    // Example FHIR patient resource with temporal data
    let _patient = json!({
        "resourceType": "Patient",
        "birthDate": "1985-03-15",
        "identifier": [{
            "period": {
                "start": "2020-01-01T00:00:00Z",
                "end": "2025-12-31T23:59:59Z"
            }
        }],
        "appointment": {
            "scheduledTime": "T14:30:00"
        }
    });

    println!("🚀 FHIRPath Temporal Literals Demo");
    println!("==================================\n");

    // Date literal examples
    println!("📅 Date Literals:");
    let expressions = vec!["@1985-03-15", "@2023-01-01", "@1990-12-25"];

    for expr in expressions {
        match parser.parse(expr) {
            Ok(parsed) => println!("✓ Parsed date literal: {expr} -> {:?}", parsed.root),
            Err(e) => println!("✗ Failed to parse {expr}: {e}"),
        }
    }

    // DateTime literal examples
    println!("\n📅⏰ DateTime Literals:");
    let expressions = vec![
        "@2023-01-01T12:30:45",
        "@2023-01-01T00:00:00Z",
        "@2023-01-01T12:30:45+05:30",
        "@2023-01-01T12:30:45-08:00",
    ];

    for expr in expressions {
        match parser.parse(expr) {
            Ok(parsed) => println!("✓ Parsed datetime literal: {expr} -> {:?}", parsed.root),
            Err(e) => println!("✗ Failed to parse {expr}: {e}"),
        }
    }

    // Time literal examples
    println!("\n⏰ Time Literals:");
    let expressions = vec!["@T12:30:45", "@T00:00:00", "@T23:59:59"];

    for expr in expressions {
        match parser.parse(expr) {
            Ok(parsed) => println!("✓ Parsed time literal: {expr} -> {:?}", parsed.root),
            Err(e) => println!("✗ Failed to parse {expr}: {e}"),
        }
    }

    println!("\n🎯 Real-world Usage Examples:");
    println!("These temporal literals enable powerful date/time comparisons:");
    println!("- birthDate >= @1980-01-01");
    println!("- identifier.period.start <= @2023-01-01T00:00:00Z");
    println!("- appointment.scheduledTime = @T14:30:00");
    println!("\n✨ Temporal literals follow ISO 8601 format and integrate seamlessly");
    println!("   with FHIRPath's type system for robust date/time operations!");

    Ok(())
}
