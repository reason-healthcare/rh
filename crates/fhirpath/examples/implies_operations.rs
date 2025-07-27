//! Logical Implies Operation Example
//!
//! This example demonstrates the `implies` logical operator in FHIRPath expressions.
//! The implies operator implements logical implication: A implies B

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath Implies Operator Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Section 1: Basic implies truth table
    println!("1. Basic Implies Truth Table:");
    println!("{}", "-".repeat(50));

    let truth_table = vec![
        ("true implies true", "true → true = true"),
        ("true implies false", "true → false = false"),
        ("false implies true", "false → true = true"),
        ("false implies false", "false → false = true"),
    ];

    for (expression, description) in truth_table {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    Logic: {}", description);
        println!();
    }

    // Section 2: Implies with expressions
    println!("2. Implies with Expressions:");
    println!("{}", "-".repeat(50));

    let expression_examples = vec![
        ("(5 > 3) implies (2 < 4)", "true implies true = true"),
        ("(5 > 3) implies (4 < 2)", "true implies false = false"),
        ("(3 > 5) implies (4 < 2)", "false implies false = true"),
        ("(3 > 5) implies (2 < 4)", "false implies true = true"),
    ];

    for (expression, description) in expression_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    Evaluation: {}", description);
        println!();
    }

    // Section 3: Operator precedence with implies
    println!("3. Operator Precedence:");
    println!("{}", "-".repeat(50));

    println!("  The implies operator has lower precedence than 'and' but higher than 'or'");
    println!("  Examples:");
    println!();

    // Test precedence: "true and false implies true" should parse as "(true and false) implies true"
    let expr = parser.parse("true and false implies true")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "  'true and false implies true' -> {}",
        format_result(&result)
    );
    println!("    Parses as: (true and false) implies true = false implies true = true");
    println!();

    // Test precedence: "false implies true and false" should parse as "false implies (true and false)"
    let expr = parser.parse("false implies true and false")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "  'false implies true and false' -> {}",
        format_result(&result)
    );
    println!("    Parses as: false implies (true and false) = false implies false = true");
    println!();

    // Section 4: Implies with empty values
    println!("4. Implies with Empty Values:");
    println!("{}", "-".repeat(50));

    println!("  According to FHIRPath specification:");
    println!("  - If left is empty and right is true, return true");
    println!("  - If left is empty and right is false, return empty");
    println!();

    let empty_examples = vec![
        ("{}[0] implies true", "empty implies true = true"),
        ("{}[0] implies false", "empty implies false = empty"),
    ];

    for (expression, description) in empty_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    Logic: {}", description);
        println!();
    }

    // Section 5: Practical use cases
    println!("5. Practical Use Cases:");
    println!("{}", "-".repeat(50));

    println!("  The implies operator is useful for conditional logic:");
    println!();
    println!("  Example 1: Data validation");
    println!("    If a patient has a birthDate, then it must be a valid date format");
    println!("    patient.birthDate.exists() implies patient.birthDate.matches('[0-9]{{4}}-[0-9]{{2}}-[0-9]{{2}}')");
    println!();
    println!("  Example 2: Business rules");
    println!("    If a medication has a strength, then it must have a unit");
    println!("    medication.strength.exists() implies medication.strength.unit.exists()");
    println!();
    println!("  Example 3: Conditional requirements");
    println!("    If observation status is 'final', then it must have a value");
    println!("    observation.status = 'final' implies observation.value.exists()");
    println!();

    // Section 6: Mathematical interpretation
    println!("6. Mathematical Interpretation:");
    println!("{}", "-".repeat(50));

    println!("  The implies operator (→) is equivalent to: NOT A OR B");
    println!("  This means 'A implies B' is false only when A is true and B is false");
    println!();

    let mathematical_examples = vec![
        ("true implies true", "¬true ∨ true = false ∨ true = true"),
        (
            "true implies false",
            "¬true ∨ false = false ∨ false = false",
        ),
        ("false implies true", "¬false ∨ true = true ∨ true = true"),
        (
            "false implies false",
            "¬false ∨ false = true ∨ false = true",
        ),
    ];

    for (expression, math_equiv) in mathematical_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    Mathematical: {}", math_equiv);
        println!();
    }

    println!("=== Implies Operator Example Complete ===");

    Ok(())
}

/// Format a FhirPathValue for display
fn format_result(value: &FhirPathValue) -> String {
    match value {
        FhirPathValue::Boolean(b) => format!("Boolean({})", b),
        FhirPathValue::String(s) => format!("String(\"{}\")", s),
        FhirPathValue::Integer(i) => format!("Integer({})", i),
        FhirPathValue::Number(n) => format!("Number({})", n),
        FhirPathValue::Empty => "Empty".to_string(),
        FhirPathValue::Collection(items) => {
            if items.is_empty() {
                "Collection(empty)".to_string()
            } else {
                format!("Collection({} items)", items.len())
            }
        }
        _ => format!("{:?}", value),
    }
}
