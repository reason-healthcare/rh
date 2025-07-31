//! Type Operations Example
//!
//! This example demonstrates the `is` and `as` type operations in FHIRPath expressions.
//! These operations allow you to check types and perform type casting.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath Type Operations Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Section 1: Type Checking with `is` operator
    println!("1. Type Checking with `is` operator:");
    println!("{}", "-".repeat(40));

    let type_check_examples = vec![
        ("true is Boolean", "Check if true is a Boolean"),
        ("'hello' is String", "Check if 'hello' is a String"),
        ("42 is Integer", "Check if 42 is an Integer"),
        ("3.14 is Number", "Check if 3.14 is a Number"),
        ("true is String", "Check if true is a String (false)"),
        (
            "'hello' is Integer",
            "Check if 'hello' is an Integer (false)",
        ),
    ];

    for (expression, description) in type_check_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 2: System namespace types
    println!("2. System Namespace Types:");
    println!("{}", "-".repeat(40));

    let system_type_examples = vec![
        ("true is System.Boolean", "Using System.Boolean type"),
        ("'world' is System.String", "Using System.String type"),
        ("100 is System.Integer", "Using System.Integer type"),
        ("2.718 is System.Decimal", "Using System.Decimal type"),
    ];

    for (expression, description) in system_type_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 3: Type Casting with `as` operator
    println!("3. Type Casting with `as` operator:");
    println!("{}", "-".repeat(40));

    let type_cast_examples = vec![
        ("'hello' as String", "Cast string to String (identity)"),
        ("42 as Integer", "Cast integer to Integer (identity)"),
        ("true as Boolean", "Cast boolean to Boolean (identity)"),
        ("3.14 as Number", "Cast number to Number (identity)"),
    ];

    for (expression, description) in type_cast_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 4: Failed type casting (returns empty)
    println!("4. Failed Type Casting (returns empty):");
    println!("{}", "-".repeat(40));

    let failed_cast_examples = vec![
        ("true as String", "Cannot cast Boolean to String"),
        ("'hello' as Integer", "Cannot cast String to Integer"),
        ("42 as Boolean", "Cannot cast Integer to Boolean"),
        ("'text' as Number", "Cannot cast String to Number"),
    ];

    for (expression, description) in failed_cast_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 5: Practical use cases
    println!("5. Practical Use Cases:");
    println!("{}", "-".repeat(40));

    // Example 1: Type-safe operations
    println!("Example 1: Conditional operations based on type");
    let conditional_examples = vec![
        (
            "42 is Integer",
            "Check if value is Integer before math operations",
        ),
        (
            "'test' is String",
            "Check if value is String before string operations",
        ),
    ];

    for (expression, description) in conditional_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    Use case: {description}");
        println!();
    }

    // Example 2: Type validation in data processing
    println!("Example 2: Data validation scenarios");
    println!("  In real FHIR data processing, you might use:");
    println!("  - patient.birthDate is System.String  // Validate date format");
    println!("  - observation.value is Quantity       // Check measurement type");
    println!("  - condition.severity is CodeableConcept // Validate coding");
    println!();

    // Section 6: Operator precedence demonstration
    println!("6. Operator Precedence (Type operations at level 13):");
    println!("{}", "-".repeat(40));

    // Type operations have precedence 13, between equality (12) and logical AND (11)
    println!("  Type operations bind tighter than logical operators:");
    println!("  Example: true is Boolean and false is Boolean");
    println!("  This parses as: (true is Boolean) and (false is Boolean)");
    println!();

    let precedence_expr = parser.parse("true is Boolean and false is Boolean")?;
    let precedence_result = evaluator.evaluate(&precedence_expr, &context)?;
    println!("  Result: {}", format_result(&precedence_result));
    println!();

    println!("  Related operators: 'implies' has lower precedence than 'and' but higher than 'or'");
    println!("  Try 'cargo run --example implies_operations' to see implies operator examples");
    println!();

    println!("=== Type Operations Example Complete ===");

    Ok(())
}

/// Format a FhirPathValue for display
fn format_result(value: &FhirPathValue) -> String {
    match value {
        FhirPathValue::Boolean(b) => format!("Boolean({b})"),
        FhirPathValue::String(s) => format!("String(\"{s}\")"),
        FhirPathValue::Integer(i) => format!("Integer({i})"),
        FhirPathValue::Number(n) => format!("Number({n})"),
        FhirPathValue::Empty => "Empty".to_string(),
        FhirPathValue::Collection(items) => {
            if items.is_empty() {
                "Collection(empty)".to_string()
            } else {
                format!("Collection({} items)", items.len())
            }
        }
        _ => format!("{value:?}"),
    }
}
