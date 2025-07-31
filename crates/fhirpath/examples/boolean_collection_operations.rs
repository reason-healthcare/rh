//! Boolean Collection Operations Example
//!
//! This example demonstrates the boolean collection functions in FHIRPath expressions:
//! - all() - checks if all items in collection evaluate to truthy
//! - allTrue() - checks if all items are boolean true values  
//! - anyTrue() - checks if any item is boolean true
//! - allFalse() - checks if all items are boolean false values
//! - anyFalse() - checks if any item is boolean false

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath Boolean Collection Functions Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Section 1: all() function - checks if all items evaluate to truthy
    println!("1. all() Function - Checks if all items are truthy:");
    println!("{}", "-".repeat(60));

    let all_examples = vec![
        ("(true | 1 | 'text').all()", "All truthy values"),
        ("(true | false | 1).all()", "Mixed truthy/falsy values"),
        ("(false | 0).all()", "All falsy values"),
        ("true.all()", "Single truthy value"),
        ("false.all()", "Single falsy value"),
        ("{}.all()", "Empty collection (vacuous truth)"),
    ];

    for (expression, description) in all_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 2: allTrue() function - checks if all items are boolean true
    println!("2. allTrue() Function - Checks if all items are boolean true:");
    println!("{}", "-".repeat(60));

    let all_true_examples = vec![
        ("(true | true | true).allTrue()", "All boolean true values"),
        ("(true | false | true).allTrue()", "Mixed boolean values"),
        (
            "(true | 1 | true).allTrue()",
            "Boolean true with non-boolean",
        ),
        ("true.allTrue()", "Single boolean true"),
        ("false.allTrue()", "Single boolean false"),
        ("1.allTrue()", "Non-boolean truthy value"),
        ("{}.allTrue()", "Empty collection"),
    ];

    for (expression, description) in all_true_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 3: anyTrue() function - checks if any item is boolean true
    println!("3. anyTrue() Function - Checks if any item is boolean true:");
    println!("{}", "-".repeat(60));

    let any_true_examples = vec![
        ("(false | true | false).anyTrue()", "One boolean true value"),
        (
            "(false | false | false).anyTrue()",
            "No boolean true values",
        ),
        (
            "(1 | 'test' | false).anyTrue()",
            "Truthy but not boolean true",
        ),
        (
            "(1 | true | 'test').anyTrue()",
            "Mixed with one boolean true",
        ),
        ("true.anyTrue()", "Single boolean true"),
        ("1.anyTrue()", "Single non-boolean truthy"),
        ("{}.anyTrue()", "Empty collection"),
    ];

    for (expression, description) in any_true_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 4: allFalse() function - checks if all items are boolean false
    println!("4. allFalse() Function - Checks if all items are boolean false:");
    println!("{}", "-".repeat(60));

    let all_false_examples = vec![
        (
            "(false | false | false).allFalse()",
            "All boolean false values",
        ),
        ("(false | true | false).allFalse()", "Mixed boolean values"),
        (
            "(false | 0 | false).allFalse()",
            "Boolean false with non-boolean",
        ),
        ("false.allFalse()", "Single boolean false"),
        ("true.allFalse()", "Single boolean true"),
        ("0.allFalse()", "Non-boolean falsy value"),
        ("{}.allFalse()", "Empty collection"),
    ];

    for (expression, description) in all_false_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 5: anyFalse() function - checks if any item is boolean false
    println!("5. anyFalse() Function - Checks if any item is boolean false:");
    println!("{}", "-".repeat(60));

    let any_false_examples = vec![
        (
            "(true | false | true).anyFalse()",
            "One boolean false value",
        ),
        ("(true | true | true).anyFalse()", "No boolean false values"),
        ("(1 | 0 | true).anyFalse()", "Falsy but not boolean false"),
        (
            "(1 | false | 'test').anyFalse()",
            "Mixed with one boolean false",
        ),
        ("false.anyFalse()", "Single boolean false"),
        ("0.anyFalse()", "Single non-boolean falsy"),
        ("{}.anyFalse()", "Empty collection"),
    ];

    for (expression, description) in any_false_examples {
        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("  {} -> {}", expression, format_result(&result));
        println!("    {description}");
        println!();
    }

    // Section 6: Practical use cases with FHIR data
    println!("6. Practical FHIR Use Cases:");
    println!("{}", "-".repeat(60));

    println!("  Example FHIR scenarios where boolean collection functions are useful:");
    println!();
    println!("  Data Validation:");
    println!("    patient.name.family.exists().all()           // All names have family name");
    println!("    observation.component.value.exists().all()   // All components have values");
    println!();
    println!("  Status Checking:");
    println!("    medication.status = 'active'.allTrue()       // All medications active");
    println!("    condition.clinicalStatus = 'resolved'.anyTrue() // Any resolved conditions");
    println!();
    println!("  Flag Analysis:");
    println!("    patient.active.allTrue()                     // Patient records all active");
    println!("    encounter.status != 'cancelled'.all()        // No cancelled encounters");
    println!();
    println!("  Boolean Field Validation:");
    println!("    questionnaire.item.required.anyTrue()        // Any required questions");
    println!("    consent.policy.authority.exists().allFalse() // No authority specified");
    println!();

    // Section 7: Comparison with other functions
    println!("7. Comparison with Related Functions:");
    println!("{}", "-".repeat(60));

    println!("  Key differences:");
    println!("  - all() vs allTrue(): all() uses truthiness, allTrue() requires boolean true");
    println!(
        "  - exists() vs anyTrue(): exists() checks presence, anyTrue() checks boolean true values"
    );
    println!("  - empty() vs allFalse(): empty() checks collection size, allFalse() checks boolean false values");
    println!();

    let comparison_examples = vec![
        (
            "(1 | 2 | 3).all()",
            "(1 | 2 | 3).allTrue()",
            "Truthy vs boolean true",
        ),
        (
            "(true | true).exists()",
            "(true | true).anyTrue()",
            "Existence vs boolean true",
        ),
        (
            "{}.empty()",
            "{}.allFalse()",
            "Empty collection vs all false",
        ),
    ];

    for (expr1, expr2, description) in comparison_examples {
        let result1 = evaluator.evaluate(&parser.parse(expr1)?, &context)?;
        let result2 = evaluator.evaluate(&parser.parse(expr2)?, &context)?;
        println!(
            "  {} -> {} vs {} -> {}",
            expr1,
            format_result(&result1),
            expr2,
            format_result(&result2)
        );
        println!("    {description}");
        println!();
    }

    println!("=== Boolean Collection Functions Example Complete ===");

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
