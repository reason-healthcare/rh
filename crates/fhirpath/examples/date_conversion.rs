//! Example demonstrating FHIRPath date conversion functions
//!
//! This example shows how to use toDate(), convertsToDate(), toDateTime(), and convertsToDateTime()
//! functions in FHIRPath expressions.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("FHIRPath Date Conversion Examples");
    println!("==================================\n");

    // toDate() function examples
    println!("toDate() Function:");
    println!("-----------------");

    // Date to Date (identity)
    let expr = parser.parse("@2023-01-15.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15.toDate() → {result:?}");

    // DateTime to Date (extract date part)
    let expr = parser.parse("@2023-01-15T10:30:45.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.toDate() → {result:?}");

    // String date to Date
    let expr = parser.parse("'2023-12-25'.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25'.toDate() → {result:?}");

    // String datetime to Date
    let expr = parser.parse("'2023-12-25T23:59:59Z'.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25T23:59:59Z'.toDate() → {result:?}");

    // Invalid conversion
    let expr = parser.parse("'not-a-date'.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'not-a-date'.toDate() → {result:?}");

    println!();

    // convertsToDate() function examples
    println!("convertsToDate() Function:");
    println!("-------------------------");

    let expr = parser.parse("@2023-01-15.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15.convertsToDate() → {result:?}");

    let expr = parser.parse("@2023-01-15T10:30:45.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.convertsToDate() → {result:?}");

    let expr = parser.parse("'2023-12-25'.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25'.convertsToDate() → {result:?}");

    let expr = parser.parse("'not-a-date'.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'not-a-date'.convertsToDate() → {result:?}");

    let expr = parser.parse("42.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.convertsToDate() → {result:?}");

    println!();

    // toDateTime() function examples
    println!("toDateTime() Function:");
    println!("---------------------");

    // DateTime to DateTime (identity)
    let expr = parser.parse("@2023-01-15T10:30:45.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.toDateTime() → {result:?}");

    // Date to DateTime (add midnight time)
    let expr = parser.parse("@2023-01-15.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15.toDateTime() → {result:?}");

    // String datetime to DateTime
    let expr = parser.parse("'2023-12-25T23:59:59'.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25T23:59:59'.toDateTime() → {result:?}");

    // String date to DateTime
    let expr = parser.parse("'2023-12-25'.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25'.toDateTime() → {result:?}");

    // Invalid conversion
    let expr = parser.parse("'not-a-datetime'.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'not-a-datetime'.toDateTime() → {result:?}");

    println!();

    // convertsToDateTime() function examples
    println!("convertsToDateTime() Function:");
    println!("-----------------------------");

    let expr = parser.parse("@2023-01-15T10:30:45.convertsToDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15T10:30:45.convertsToDateTime() → {result:?}");

    let expr = parser.parse("@2023-01-15.convertsToDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("@2023-01-15.convertsToDateTime() → {result:?}");

    let expr = parser.parse("'2023-12-25'.convertsToDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'2023-12-25'.convertsToDateTime() → {result:?}");

    let expr = parser.parse("'not-a-datetime'.convertsToDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'not-a-datetime'.convertsToDateTime() → {result:?}");

    let expr = parser.parse("42.convertsToDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.convertsToDateTime() → {result:?}");

    println!();

    // Practical examples
    println!("Practical Examples:");
    println!("------------------");

    // Extract date from various formats
    let expr = parser.parse("'2023-05-15T14:30:00Z'.toDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Extract date from ISO datetime: {result:?}");

    // Convert date to datetime for comparison
    let expr = parser.parse("'2023-05-15'.toDateTime()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Convert date to datetime: {result:?}");

    // Check if values can be converted before converting
    let expr = parser.parse("'2023-05-15T10:30:45'.convertsToDate()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("Can convert datetime string to date: {result:?}");

    Ok(())
}
