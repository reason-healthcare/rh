//! Example demonstrating FHIRPath string conversion functions
//!
//! This example shows how to use toString() and convertsToString()
//! functions in FHIRPath expressions.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("FHIRPath String Conversion Examples");
    println!("===================================\n");

    // toString() function examples
    println!("toString() Function:");
    println!("-------------------");

    // String to String (identity)
    let expr = parser.parse("'hello world'.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello world'.toString() → {result:?}");

    // Boolean to String
    let expr = parser.parse("true.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("true.toString() → {result:?}");

    let expr = parser.parse("false.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("false.toString() → {result:?}");

    // Integer to String
    let expr = parser.parse("42.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.toString() → {result:?}");

    let expr = parser.parse("(-123).toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(-123).toString() → {result:?}");

    // Long to String
    let expr = parser.parse("42L.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42L.toString() → {result:?}");

    // Number to String
    let expr = parser.parse("3.14.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("3.14.toString() → {result:?}");

    let expr = parser.parse("42.0.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.0.toString() → {result:?}");

    let expr = parser.parse("0.0.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("0.0.toString() → {result:?}");

    println!();

    // Date/Time to String examples
    println!("Date/Time toString():");
    println!("--------------------");

    let expr = parser.parse("@2023-01-15.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15.toString() → {result:?}");

    let expr = parser.parse("@2023-01-15T10:30:45.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.toString() → {result:?}");

    let expr = parser.parse("@T10:30:45.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@T10:30:45.toString() → {result:?}");

    println!();

    // Quantity to String examples
    println!("Quantity toString():");
    println!("-------------------");

    let expr = parser.parse("5'mg'.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("5'mg'.toString() → {result:?}");

    let expr = parser.parse("37.2'Cel'.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("37.2'Cel'.toString() → {result:?}");

    println!();

    // convertsToString() function examples
    println!("convertsToString() Function:");
    println!("---------------------------");

    let basic_types = vec![
        "'hello'",
        "true",
        "false",
        "42",
        "42L",
        "3.14",
        "@2023-01-15",
        "@2023-01-15T10:30:45",
        "@T10:30:45",
        "5'mg'",
    ];

    for type_expr in basic_types {
        let expr = parser.parse(&format!("{type_expr}.convertsToString()"))?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("{type_expr}.convertsToString() → {result:?}");
    }

    println!();

    // Edge cases
    println!("Edge Cases:");
    println!("-----------");

    // Empty collection
    let expr = parser.parse("{}.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("{{}}.toString() → {result:?}");

    let expr = parser.parse("{}.convertsToString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("{{}}.convertsToString() → {result:?}");

    // Multiple items
    let expr = parser.parse("(1 | 2).toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(1 | 2).toString() → {result:?}");

    let expr = parser.parse("(1 | 2).convertsToString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(1 | 2).convertsToString() → {result:?}");

    // Single item collection
    let expr = parser.parse("(42).toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(42).toString() → {result:?}");

    println!();

    // Practical examples
    println!("Practical Examples:");
    println!("------------------");

    // Convert different types for display
    let expr = parser.parse("(true | 42 | 3.14).select(toString())")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert mixed types to strings: {result:?}");

    // Check if values can be stringified before converting
    let expr = parser.parse("42.convertsToString() and 42.toString()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Check convertibility then convert: {result:?}");

    // Format numbers as strings
    let expr = parser.parse("(1 | 2.5 | 42L).select(toString())")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Format numbers as strings: {result:?}");

    Ok(())
}
