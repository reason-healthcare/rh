use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("=== FHIRPath matchesFull() vs matches() Comparison ===");
    println!();

    println!(
        "The matchesFull() function performs exact pattern matching with implied ^ and $ anchors,"
    );
    println!("while matches() performs partial pattern matching anywhere in the string.");
    println!();

    // Healthcare ID validation
    println!("Healthcare ID Validation:");
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'N80001231'.matches('N[0-9]{8}')",
        "'N80001231'.matchesFull('N[0-9]{8}')",
        "Exact 8-digit patient ID",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'N8000123123'.matches('N[0-9]{8}')",
        "'N8000123123'.matchesFull('N[0-9]{8}')",
        "10-digit patient ID (extra digits)",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'N8000123'.matches('N[0-9]{8}')",
        "'N8000123'.matchesFull('N[0-9]{8}')",
        "7-digit patient ID (missing digits)",
    );
    println!();

    // Date validation
    println!("Date Format Validation:");
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'2023-12-25'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "'2023-12-25'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "Valid ISO date",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'2023-12-25T10:30:00'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "'2023-12-25T10:30:00'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "DateTime with date pattern",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'Created: 2023-12-25'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "'Created: 2023-12-25'.matchesFull('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "Text containing date",
    );
    println!();

    // Phone number validation
    println!("Phone Number Validation:");
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'555-123-4567'.matches('[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "'555-123-4567'.matchesFull('[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "Exact phone number",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'Call 555-123-4567 now'.matches('[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "'Call 555-123-4567 now'.matchesFull('[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "Text containing phone number",
    );
    println!();

    // Word matching
    println!("Word Matching:");
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'world'.matches('world')",
        "'world'.matchesFull('world')",
        "Exact word match",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'hello world'.matches('world')",
        "'hello world'.matchesFull('world')",
        "Text containing word",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'worldwide'.matches('world')",
        "'worldwide'.matchesFull('world')",
        "Word as substring",
    );
    println!();

    // Email validation
    println!("Email Validation:");
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'user@example.com'.matches('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "'user@example.com'.matchesFull('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "Valid email address",
    );
    compare_functions(
        &parser,
        &evaluator,
        &context,
        "'Email: user@example.com'.matches('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "'Email: user@example.com'.matchesFull('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "Text containing email",
    );
    println!();

    println!("=== Summary ===");
    println!("Use matches() when you want to find a pattern anywhere in the string.");
    println!(
        "Use matchesFull() when you want to validate that the entire string matches the pattern."
    );
    println!("matchesFull(pattern) is equivalent to matches('^' + pattern + '$')");
}

fn compare_functions(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    matches_expr: &str,
    matches_full_expr: &str,
    description: &str,
) {
    let matches_result = evaluate_expression(parser, evaluator, context, matches_expr);
    let matches_full_result = evaluate_expression(parser, evaluator, context, matches_full_expr);

    println!("  {description}:");
    println!("    matches()     → {matches_result:?}");
    println!("    matchesFull() → {matches_full_result:?}");

    match (&matches_result, &matches_full_result) {
        (Ok(r1), Ok(r2)) if r1 != r2 => {
            println!("    ⚠ Different results - matchesFull() requires exact match");
        }
        (Ok(_), Ok(_)) => {
            println!("    ✓ Same result");
        }
        _ => {
            println!("    ⚠ Error in evaluation");
        }
    }
    println!();
}

fn evaluate_expression(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
) -> Result<fhirpath::FhirPathValue, fhirpath::FhirPathError> {
    match parser.parse(expression) {
        Ok(expr) => evaluator.evaluate(&expr, context),
        Err(e) => Err(fhirpath::FhirPathError::TypeError {
            message: format!("Parse error: {e:?}"),
        }),
    }
}
