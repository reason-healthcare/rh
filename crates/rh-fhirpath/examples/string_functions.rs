/// FHIRPath - String Functions Example
///
/// This example demonstrates string manipulation functions in FHIRPath expressions.
use anyhow::Result;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("🔤 FHIRPath String Functions Examples");
    println!("=====================================\n");

    // Example 1: String length
    println!("1️⃣ Length Function:");
    let expressions = vec![
        "'hello'.length()",
        "''.length()",
        "'hello world'.length()",
        "'FHIRPath'.length()",
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 2: Case conversion
    println!("\n2️⃣ Case Conversion:");
    let expressions = vec![
        "'hello'.upper()",
        "'WORLD'.lower()",
        "'FhIrPaTh'.upper()",
        "'MiXeD cAsE'.lower()",
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 3: Substring operations
    println!("\n3️⃣ Substring Operations:");
    let expressions = vec![
        "'hello world'.substring(0, 5)", // Extract first 5 characters
        "'FHIRPath'.substring(4)",       // From index 4 to end
        "'programming'.substring(3, 7)", // Extract middle part
        "'test'.substring(0, 2)",        // First 2 characters
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 4: String searching
    println!("\n4️⃣ String Search Functions:");
    let expressions = vec![
        "'hello world'.contains('world')",
        "'FHIRPath'.contains('PATH')", // Case sensitive
        "'programming'.startsWith('prog')",
        "'testing'.endsWith('ing')",
        "'hello world'.indexOf('o')", // Find first occurrence
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 5: String replacement and trimming
    println!("\n5️⃣ String Modification:");
    let expressions = vec![
        "'hello world'.replace('world', 'FHIRPath')",
        "'  spaces  '.trim()",
        "'remove_underscores'.replace('_', ' ')",
        "'FHIR-Path'.replace('-', '')",
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 6: String concatenation
    println!("\n6️⃣ String Concatenation:");
    let expressions = vec![
        "'Hello' + ' ' + 'World'",
        "'FHIR' + 'Path'",
        "'Patient' + '-' + '123'",
        "'Version: ' + '4.0.1'",
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    // Example 7: Working with FHIR data
    println!("\n7️⃣ String Functions with FHIR Data:");

    let patient_data = json!({
        "resourceType": "Patient",
        "id": "patient-123",
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "James"]
            },
            {
                "use": "nickname",
                "given": ["johnny"]
            }
        ],
        "telecom": [
            {
                "system": "email",
                "value": "john.doe@example.com"
            },
            {
                "system": "phone",
                "value": "+1-555-123-4567"
            }
        ]
    });

    let fhir_context = EvaluationContext::new(patient_data);

    let expressions = vec![
        "id.upper()",          // Convert ID to uppercase
        "name.family.upper()", // Convert family name to uppercase
        "name.given.lower()",  // Convert given names to lowercase
                               // Note: These expressions may need adjustment based on actual FHIRPath implementation
                               // for handling collections and nested fields
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &fhir_context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // Example 8: Practical string validation patterns
    println!("\n8️⃣ String Validation Patterns:");

    let expressions = vec![
        "'john.doe@example.com'.contains('@')", // Simple email check
        "'Patient-123'.startsWith('Patient')",  // ID prefix validation
        "'4.0.1'.contains('.')",                // Version format check
        "'US'.length() = 2",                    // Country code length
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {expr_str} = {result:?}");
    }

    println!("\n✅ All string function examples completed successfully!");
    println!("💡 String functions are case-sensitive unless otherwise specified");
    println!("💡 Use these functions to clean, validate, and transform text data");
    println!("💡 Combine string functions for complex text processing workflows");

    Ok(())
}
