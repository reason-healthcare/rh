//! Example demonstrating the lastIndexOf() string function
//!
//! The lastIndexOf() function finds the 0-based index of the last occurrence
//! of a substring within a string. Returns -1 if not found.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    println!("=== FHIRPath lastIndexOf() Function Examples ===\n");

    // Create parser and evaluator
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Use context with a string field
    let context = EvaluationContext::new(json!({
        "text": "hello world world"
    }));

    // Basic lastIndexOf usage
    let expr = parser.parse("'hello world world'.lastIndexOf('world')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello world world'.lastIndexOf('world') = {result:?}");

    // Compare with indexOf
    let expr = parser.parse("'hello world world'.indexOf('world')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello world world'.indexOf('world') = {result:?}");

    // Not found case
    let expr = parser.parse("'hello world'.lastIndexOf('foo')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello world'.lastIndexOf('foo') = {result:?}");

    // Single occurrence (same as indexOf)
    let expr = parser.parse("'hello world'.lastIndexOf('hello')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello world'.lastIndexOf('hello') = {result:?}");

    // Empty string case
    let expr = parser.parse("'hello'.lastIndexOf('')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello'.lastIndexOf('') = {result:?}");

    // Using with FHIR data context
    let expr = parser.parse("text.lastIndexOf('world')")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("text.lastIndexOf('world') = {result:?}");

    Ok(())
}
