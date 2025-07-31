//! Example demonstrating the not() function for boolean negation
//!
//! The not() function performs logical negation on boolean values:
//! - not(true) returns false
//! - not(false) returns true
//! - not(()) returns () (empty collection)  
//! - Multiple items return empty collection
//! - Non-boolean values return empty collection

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    println!("=== FHIRPath not() Function Examples ===\n");

    // Create parser and evaluator
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Use empty context for basic boolean operations
    let context = EvaluationContext::new(json!({}));

    // Basic boolean negation
    let expr = parser.parse("true.not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("true.not() = {result:?}");

    let expr = parser.parse("false.not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("false.not() = {result:?}");

    // Empty collection
    let expr = parser.parse("({}).not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("({{}}).not() = {result:?}");

    // With expressions
    let expr = parser.parse("(5 > 3).not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(5 > 3).not() = {result:?}");

    let expr = parser.parse("(2 > 5).not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(2 > 5).not() = {result:?}");

    // Chaining with other boolean functions
    let expr = parser.parse("(true | false).allTrue().not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(true | false).allTrue().not() = {result:?}");

    let expr = parser.parse("(false | false).allFalse().not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(false | false).allFalse().not() = {result:?}");

    // Double negation
    let expr = parser.parse("true.not().not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("true.not().not() = {result:?}");

    println!("\n=== Error Cases (FHIRPath behavior: return empty) ===");

    // Non-boolean values return empty (FHIRPath behavior)
    let expr = parser.parse("'hello'.not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("'hello'.not() = {result:?} (returns empty for non-boolean)");

    let expr = parser.parse("42.not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("42.not() = {result:?} (returns empty for non-boolean)");

    // Multiple items return empty
    let expr = parser.parse("(true | false).not()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("(true | false).not() = {result:?} (returns empty for multiple items)");

    Ok(())
}
