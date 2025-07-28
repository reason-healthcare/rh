//! Example demonstrating the replaceMatches() function in FHIRPath
//!
//! The replaceMatches() function replaces all matches of a regular expression pattern
//! with a substitution string, supporting capture groups and advanced regex features.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath replaceMatches() Function Examples ===\n");

    // Example 1: Replace digits with placeholder
    println!("1. Replace all digits with placeholder:");
    let data = json!({
        "text": "Patient ID: 12345, Visit: 67890"
    });
    let expr = parser.parse("text.replaceMatches('\\\\d+', 'XXX')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: Patient ID: 12345, Visit: 67890");
    println!("   Pattern: \\\\d+");
    println!("   Result: {result:?}\n");

    // Example 2: Format phone numbers using capture groups
    println!("2. Format phone numbers using capture groups:");
    let data = json!({
        "phone": "1234567890"
    });
    let expr =
        parser.parse("phone.replaceMatches('(\\\\d{3})(\\\\d{3})(\\\\d{4})', '($1) $2-$3')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: 1234567890");
    println!("   Pattern: (\\\\d{{3}})(\\\\d{{3}})(\\\\d{{4}})");
    println!("   Substitution: ($1) $2-$3");
    println!("   Result: {result:?}\n");

    // Example 3: Extract and reformat email addresses
    println!("3. Reformat email addresses:");
    let data = json!({
        "email": "john.doe@example.com"
    });
    let expr =
        parser.parse("email.replaceMatches('(\\\\w+)\\\\.(\\\\w+)@(.+)', '$2, $1 from $3')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: john.doe@example.com");
    println!("   Pattern: (\\\\w+)\\\\.(\\\\w+)@(.+)");
    println!("   Substitution: $2, $1 from $3");
    println!("   Result: {result:?}\n");

    // Example 4: Replace whole words only
    println!("4. Replace whole words using word boundaries:");
    let data = json!({
        "text": "cat catastrophe scattered cats"
    });
    let expr = parser.parse("text.replaceMatches('\\\\bcat\\\\b', 'dog')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: cat catastrophe scattered cats");
    println!("   Pattern: \\\\bcat\\\\b (word boundary)");
    println!("   Result: {result:?}\n");

    // Example 5: Clean up whitespace
    println!("5. Clean up multiple whitespace:");
    let data = json!({
        "text": "  Multiple    spaces   and   tabs\t\there  "
    });
    let expr = parser.parse("text.replaceMatches('\\\\s+', ' ').trim()")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: '  Multiple    spaces   and   tabs\\t\\there  '");
    println!("   Pattern: \\\\s+ (multiple whitespace)");
    println!("   Result: {result:?}\n");

    // Example 6: Extract domain from URLs
    println!("6. Extract domain from URLs:");
    let data = json!({
        "url": "https://www.example.com/path/to/page?param=value"
    });
    let expr = parser.parse("url.replaceMatches('https?://(?:www\\\\.)?([^/]+).*', '$1')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: https://www.example.com/path/to/page?param=value");
    println!("   Pattern: https?://(?:www\\\\.)?([^/]+).*");
    println!("   Substitution: $1");
    println!("   Result: {result:?}\n");

    // Example 7: Format medical identifiers
    println!("7. Format medical identifiers:");
    let data = json!({
        "mrn": "MRN123456789"
    });
    let expr =
        parser.parse("mrn.replaceMatches('MRN(\\\\d{3})(\\\\d{3})(\\\\d{3})', 'MRN: $1-$2-$3')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: MRN123456789");
    println!("   Pattern: MRN(\\\\d{{3}})(\\\\d{{3}})(\\\\d{{3}})");
    println!("   Substitution: MRN: $1-$2-$3");
    println!("   Result: {result:?}\n");

    // Example 8: Redact sensitive information
    println!("8. Redact social security numbers:");
    let data = json!({
        "note": "Patient SSN is 123-45-6789 for reference"
    });
    let expr = parser.parse("note.replaceMatches('\\\\d{3}-\\\\d{2}-\\\\d{4}', 'XXX-XX-XXXX')")?;
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Input: Patient SSN is 123-45-6789 for reference");
    println!("   Pattern: \\\\d{{3}}-\\\\d{{2}}-\\\\d{{4}}");
    println!("   Result: {result:?}\n");

    Ok(())
}
