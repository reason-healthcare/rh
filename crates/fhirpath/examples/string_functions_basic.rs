//! Example demonstrating basic string functions: length() and toChars() in FHIRPath
//!
//! The length() function returns the number of characters in a string as an Integer.
//! The toChars() function converts a string into a collection of single-character strings.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath Basic String Functions Examples ===\n");

    // Example 1: length() function
    println!("1. String length() function:");
    let data = json!({
        "name": "John Doe",
        "shortName": "Joe",
        "empty": ""
    });
    let context = EvaluationContext::new(data.clone());

    let expr = parser.parse("name.length()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'John Doe'.length() = {result:?}");

    let expr = parser.parse("shortName.length()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'Joe'.length() = {result:?}");

    let expr = parser.parse("empty.length()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   ''.length() = {result:?}\n");

    // Example 2: toChars() function - basic usage
    println!("2. String toChars() function - basic usage:");
    let expr = parser.parse("shortName.toChars()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'Joe'.toChars() = {result:?}");

    let expr = parser.parse("name.toChars()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'John Doe'.toChars() = {result:?}\n");

    // Example 3: toChars() with empty string
    println!("3. toChars() with empty string:");
    let expr = parser.parse("empty.toChars()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   ''.toChars() = {result:?}\n");

    // Example 4: Unicode characters
    println!("4. Unicode character handling:");
    let data = json!({
        "greeting": "café",
        "emoji": "👋🌍"
    });
    let context = EvaluationContext::new(data);

    let expr = parser.parse("greeting.length()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'café'.length() = {result:?}");

    let expr = parser.parse("greeting.toChars()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'café'.toChars() = {result:?}");

    let expr = parser.parse("emoji.length()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '👋🌍'.length() = {result:?}");

    let expr = parser.parse("emoji.toChars()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   '👋🌍'.toChars() = {result:?}\n");

    // Example 5: Combining functions
    println!("5. Combining string functions:");
    let data = json!({
        "text": "Hello World"
    });
    let context = EvaluationContext::new(data);

    // Count characters
    let expr = parser.parse("text.toChars().count()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'Hello World'.toChars().count() = {result:?}");

    // First and last characters
    let expr = parser.parse("text.toChars().first()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'Hello World'.toChars().first() = {result:?}");

    let expr = parser.parse("text.toChars().last()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   'Hello World'.toChars().last() = {result:?}");

    // Filter for vowels (case-sensitive)
    let expr = parser.parse("text.toChars().where($this in ('a' | 'e' | 'i' | 'o' | 'u'))")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Vowels in 'Hello World': {result:?}");

    // Get unique characters
    let expr = parser.parse("text.toChars().distinct()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Unique chars in 'Hello World': {result:?}\n");

    // Example 6: Medical use case - parsing identifiers
    println!("6. Medical use case - parsing patient identifier:");
    let data = json!({
        "patientId": "P12345",
        "mrn": "MRN-ABC-123"
    });
    let context = EvaluationContext::new(data);

    // Count alphabetic characters
    let expr = parser.parse("patientId.toChars().where($this.matches('[A-Z]')).count()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Alphabetic chars in 'P12345': {result:?}");

    // Count numeric characters
    let expr = parser.parse("patientId.toChars().where($this.matches('[0-9]')).count()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Numeric chars in 'P12345': {result:?}");

    // Count dashes in MRN
    let expr = parser.parse("mrn.toChars().where($this = '-').count()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Number of dashes in 'MRN-ABC-123': {result:?}");

    Ok(())
}
