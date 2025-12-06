//! Example: Error Reporting in CQL-to-ELM Translation
//!
//! This example demonstrates:
//! - Creating compiler exceptions with source locations
//! - Severity levels and filtering
//! - Converting to ELM annotation format
//! - Collecting and summarizing errors
//!
//! Run with: cargo run --example error_reporting

use rh_cql::options::ErrorSeverity;
use rh_cql::reporting::{
    CqlCompilerException, ExceptionCollector, ExceptionType, SourceLocator,
};

fn main() {
    println!("=== CQL Error Reporting Example ===\n");

    // Example 1: Creating exceptions with source locations
    println!("1. Creating exceptions with source locations:");
    println!("----------------------------------------------");

    let error = CqlCompilerException::new("Could not resolve identifier 'PatientAge'")
        .with_locator(SourceLocator::new(10, 5, 10, 15))
        .with_error_type(ExceptionType::Semantic);

    println!("Error: {error}");
    println!("  Message: {}", error.message());
    println!("  Severity: {}", error.severity());
    println!("  Type: {}", error.error_type());
    println!("  Location: {:?}", error.locator());
    println!();

    // Example 2: Different exception types
    println!("2. Different exception types:");
    println!("-----------------------------");

    let syntax_error = CqlCompilerException::syntax("Unexpected token 'then'")
        .with_locator(SourceLocator::new(5, 0, 5, 4));
    println!("Syntax Error: {syntax_error}");

    let warning = CqlCompilerException::warning("Expression always evaluates to null")
        .with_locator(SourceLocator::new(15, 10, 15, 25));
    println!("Warning: {warning}");

    let info = CqlCompilerException::info("Implicit conversion from Integer to Decimal");
    println!("Info: {info}");

    let include_error = CqlCompilerException::include("Could not resolve library")
        .with_target_library("FHIRHelpers")
        .with_cause("File not found: FHIRHelpers-4.0.1.cql");
    println!("Include Error: {include_error}");
    println!();

    // Example 3: Error collection and filtering
    println!("3. Error collection and filtering:");
    println!("-----------------------------------");

    // Collector that filters by minimum severity
    let mut collector = ExceptionCollector::new(ErrorSeverity::Warning);

    collector.add(CqlCompilerException::new("Error 1: Undefined symbol"));
    collector.add(CqlCompilerException::new("Error 2: Type mismatch"));
    collector.add(CqlCompilerException::warning("Warning: Deprecated function"));
    collector.add(CqlCompilerException::info("Info: This will be filtered out"));

    println!("Collector with Warning threshold:");
    println!("  Total collected: {}", collector.len());
    println!("  Errors: {}", collector.error_count());
    println!("  Warnings: {}", collector.warning_count());
    println!("  Summary: {}", collector.summary());
    println!();

    // Example 4: Converting to ELM format
    println!("4. Converting to ELM annotation format:");
    println!("---------------------------------------");

    let exc = CqlCompilerException::new("Could not resolve function 'AgeInYears'")
        .with_locator(SourceLocator::new(20, 8, 20, 18))
        .with_error_type(ExceptionType::Semantic);

    let elm_error = exc.to_elm_error();
    println!("ELM Error JSON:");
    println!(
        "{}",
        serde_json::to_string_pretty(&elm_error).unwrap()
    );
    println!();

    // Example 5: Full diagnostic output
    println!("5. Diagnostic output for all collected errors:");
    println!("-----------------------------------------------");

    let mut full_collector = ExceptionCollector::all();

    full_collector.add(
        CqlCompilerException::syntax("Missing 'end' keyword")
            .with_locator(SourceLocator::new(1, 0, 1, 10)),
    );
    full_collector.add(
        CqlCompilerException::semantic("Unknown type 'Observation'")
            .with_locator(SourceLocator::new(5, 4, 5, 15))
            .with_cause("Model 'FHIR' not loaded"),
    );
    full_collector.add(
        CqlCompilerException::warning("Result type is Any")
            .with_locator(SourceLocator::new(10, 0, 10, 20)),
    );

    for exc in &full_collector {
        println!("{}", exc.to_diagnostic_string());
    }
    println!();

    // Example 6: Batch conversion to ELM errors
    println!("6. Batch conversion for ELM annotation:");
    println!("---------------------------------------");

    let elm_errors = full_collector.to_elm_errors();
    println!("ELM Errors ({}): ", elm_errors.len());
    for (i, err) in elm_errors.iter().enumerate() {
        println!("  {}. {} at {}:{}", i + 1, err.message, err.start_line.unwrap_or(0), err.start_char.unwrap_or(0));
    }
    println!();

    // Example 7: Severity threshold checking
    println!("7. Severity threshold checking:");
    println!("-------------------------------");

    let error = CqlCompilerException::new("Some error");
    let warning = CqlCompilerException::warning("Some warning");
    let info = CqlCompilerException::info("Some info");

    for (name, exc) in [("Error", &error), ("Warning", &warning), ("Info", &info)] {
        println!("{name}:");
        println!(
            "  meets Error threshold: {}",
            exc.meets_threshold(ErrorSeverity::Error)
        );
        println!(
            "  meets Warning threshold: {}",
            exc.meets_threshold(ErrorSeverity::Warning)
        );
        println!(
            "  meets Info threshold: {}",
            exc.meets_threshold(ErrorSeverity::Info)
        );
    }
    println!();

    // Example 8: Source locator formatting
    println!("8. Source locator formatting:");
    println!("-----------------------------");

    let point = SourceLocator::point(5, 10);
    println!("Point location: {point}");

    let range_same_line = SourceLocator::new(5, 10, 5, 20);
    println!("Same-line range: {range_same_line}");

    let multiline = SourceLocator::new(5, 10, 8, 5);
    println!("Multi-line range: {multiline}");

    println!("\n=== Example Complete ===");
}
