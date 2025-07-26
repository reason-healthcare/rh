//! FHIRPath Date/Time Functions Example
//!
//! This example demonstrates the use of FHIRPath date/time functions:
//! - now() - returns current date and time
//! - today() - returns current date  
//! - timeOfDay() - returns current time
//!
//! These functions are useful for date/time calculations and comparisons in FHIR resources.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FHIRPath Date/Time Functions Example");
    println!("====================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("📅 Basic Date/Time Functions:");
    println!("-----------------------------");

    // Test the date/time functions
    let expressions = vec!["now()", "today()", "timeOfDay()"];

    for expression in &expressions {
        match parser.parse(expression) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(FhirPathValue::Collection(values)) if values.len() == 1 => {
                    println!("   {} => {:?}", expression, values[0]);
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
    }

    println!("\n🔍 Date/Time Functions in Expressions:");
    println!("--------------------------------------");

    // Test using date/time functions in more complex expressions
    let complex_expressions = vec![
        "now().exists()",
        "today().exists()",
        "timeOfDay().exists()",
        "(now() | today() | timeOfDay()).count()",
    ];

    for expression in &complex_expressions {
        match parser.parse(expression) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(FhirPathValue::Collection(values)) if values.len() == 1 => {
                    println!("   {} => {:?}", expression, values[0]);
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
    }

    println!("\n🚫 Error Cases:");
    println!("---------------");

    // Test error cases (functions with parameters)
    let error_expressions = vec!["now(5)", "today('test')", "timeOfDay(1, 2)"];

    for expression in &error_expressions {
        match parser.parse(expression) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(value) => {
                    println!("   {expression} => UNEXPECTED SUCCESS: {value:?}");
                }
                Err(e) => {
                    println!("   {expression} => ERROR (expected): {e}");
                }
            },
            Err(e) => {
                println!("   {expression} => PARSE ERROR: {e}");
            }
        }
    }

    println!("\n📝 Notes:");
    println!("--------");
    println!("   • now() returns UTC time in ISO 8601 format (YYYY-MM-DDTHH:mm:ss.sssZ)");
    println!("   • today() returns local date in ISO 8601 format (YYYY-MM-DD)");
    println!("   • timeOfDay() returns local time in ISO 8601 format (HH:mm:ss.sss)");
    println!("   • All functions take no parameters and return current system date/time");
    println!("   • These functions are useful for date/time comparisons in FHIR validation");
    println!("   • Functions can be used in expressions with other FHIRPath operations");

    Ok(())
}
