use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    println!("=== FHIRPath matches() Function Examples ===");
    println!();

    // Healthcare identifiers
    println!("Healthcare Identifiers:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'N8000123123'.matches('N[0-9]{8}')",
        "Patient ID with N prefix and 8+ digits",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'N80001231'.matches('^N[0-9]{8}$')",
        "Exact 8-digit patient ID",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'MRN-12345'.matches('MRN-[0-9]+')",
        "Medical record number pattern",
    );
    println!();

    // Date patterns
    println!("Date Validation:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'2023-12-25'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "ISO date format",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'23-12-25'.matches('^[0-9]{4}-[0-9]{2}-[0-9]{2}$')",
        "Invalid date format (2-digit year)",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'2023-02-29'.matches('[0-9]{4}-[0-9]{2}-[0-9]{2}')",
        "Date pattern (regardless of validity)",
    );
    println!();

    // Phone numbers
    println!("Phone Number Patterns:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'555-123-4567'.matches('[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "US phone number format",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'(555) 123-4567'.matches('\\([0-9]{3}\\) [0-9]{3}-[0-9]{4}')",
        "US phone with parentheses",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'+1-555-123-4567'.matches('\\+1-[0-9]{3}-[0-9]{3}-[0-9]{4}')",
        "International format",
    );
    println!();

    // Email patterns
    println!("Email Validation:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'user@example.com'.matches('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "Simple email pattern",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'patient.doe@hospital.org'.matches('[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}')",
        "Complex email pattern",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'invalid.email'.matches('[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}')",
        "Invalid email (no @)",
    );
    println!();

    // FHIR codes and IDs
    println!("FHIR Resource Patterns:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'patient/12345'.matches('[a-z]+/[0-9]+')",
        "FHIR resource reference",
    );
    print_result(&parser, &evaluator, &context, "'urn:uuid:12345678-1234-1234-1234-123456789012'.matches('urn:uuid:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}')", "UUID pattern");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'http://loinc.org|12345-6'.matches('http://[a-zA-Z0-9.-]+\\|[0-9]+-[0-9]')",
        "LOINC code system",
    );
    println!();

    // Case sensitivity
    println!("Case Sensitivity:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'Hello World'.matches('[Hh]ello')",
        "Case-sensitive pattern",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'PATIENT'.matches('[Pp][Aa][Tt][Ii][Ee][Nn][Tt]')",
        "Case-insensitive pattern",
    );
    println!();

    // Error cases
    println!("Error Handling:");
    print_result(
        &parser,
        &evaluator,
        &context,
        "'test'.matches('[invalid')",
        "Invalid regex pattern",
    );
    print_result(
        &parser,
        &evaluator,
        &context,
        "'test'.matches('(unclosed')",
        "Unclosed group pattern",
    );
    println!();

    println!("=== Summary ===");
    println!("The matches() function now supports full regular expression patterns,");
    println!("enabling powerful pattern matching for healthcare data validation,");
    println!("identifier formats, and complex string matching scenarios.");
}

fn print_result(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
    description: &str,
) {
    match parser.parse(expression) {
        Ok(expr) => match evaluator.evaluate(&expr, context) {
            Ok(result) => {
                println!("  {description} → {result:?}");
            }
            Err(e) => {
                println!("  {description} → Error: {e:?}");
            }
        },
        Err(e) => {
            println!("  {description} → Parse Error: {e:?}");
        }
    }
}
