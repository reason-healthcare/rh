/// FHIRPath - Arithmetic Operations Example
///
/// This example demonstrates mathematical operations in FHIRPath expressions.
use anyhow::Result;
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("🧮 FHIRPath Arithmetic Operations Examples");
    println!("==========================================\n");

    // Example 1: Basic addition
    println!("1️⃣ Addition Examples:");
    let expressions = vec![
        "2 + 3",     // Integer addition
        "2.5 + 1.5", // Decimal addition
        "10 + 0",    // Adding zero
        "-5 + 10",   // Negative numbers
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 2: Subtraction
    println!("\n2️⃣ Subtraction Examples:");
    let expressions = vec![
        "10 - 3",    // Basic subtraction
        "2.5 - 1.5", // Decimal subtraction
        "0 - 5",     // Negative result
        "100 - 100", // Zero result
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 3: Multiplication
    println!("\n3️⃣ Multiplication Examples:");
    let expressions = vec![
        "4 * 5",     // Integer multiplication
        "2.5 * 2.0", // Decimal multiplication
        "7 * 0",     // Multiply by zero
        "-3 * 4",    // Negative multiplication
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 4: Division
    println!("\n4️⃣ Division Examples:");
    let expressions = vec![
        "10 / 2",    // Integer division
        "7.5 / 2.5", // Decimal division
        "1 / 3",     // Fractional result
        "0 / 5",     // Zero divided by number
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 5: Modulo
    println!("\n5️⃣ Modulo Examples:");
    let expressions = vec![
        "10 mod 3", // Basic modulo
        "15 mod 4", // Remainder calculation
        "8 mod 2",  // Even division (remainder 0)
        "7 mod 7",  // Same numbers
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 6: Complex expressions with operator precedence
    println!("\n6️⃣ Complex Expressions (operator precedence):");
    let expressions = vec![
        "2 + 3 * 4",     // Multiplication before addition
        "(2 + 3) * 4",   // Parentheses override precedence
        "10 - 6 / 2",    // Division before subtraction
        "2 * 3 + 4 * 5", // Multiple operations
        "100 / (2 * 5)", // Parentheses with division
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 7: Working with FHIR data and arithmetic
    println!("\n7️⃣ Arithmetic with FHIR Data:");

    let patient_data = json!({
        "resourceType": "Patient",
        "age": 30,
        "scores": [85, 92, 78, 95, 88]
    });

    let fhir_context = EvaluationContext::new(patient_data);

    let expressions = vec![
        "age + 5", // Add to patient age
        "age * 2", // Double the age
                   // Note: Collection operations would need more advanced FHIRPath features
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &fhir_context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    println!("\n✅ All arithmetic examples completed successfully!");
    println!("💡 These operations follow standard mathematical precedence rules");
    println!("💡 Use parentheses to override default operator precedence");

    Ok(())
}
