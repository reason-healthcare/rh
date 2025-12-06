//! Example: Compiling CQL to ELM
//!
//! This example demonstrates the main public API for compiling CQL source code
//! to ELM (Expression Logical Model).
//!
//! Run with: `cargo run --example compile_cql`

use rh_cql::{compile, compile_to_json, validate, CompilerOptions, SignatureLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CQL Compiler Example ===\n");

    // Example 1: Simple compilation
    simple_compilation()?;

    // Example 2: Compilation with custom options
    compilation_with_options()?;

    // Example 3: Validation only
    validation_example()?;

    // Example 4: Direct to JSON
    direct_to_json()?;

    // Example 5: Complex library
    complex_library()?;

    Ok(())
}

/// Example 1: Simple compilation with default options
fn simple_compilation() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 1: Simple Compilation ---");

    let source = r#"
        library SimpleExample version '1.0.0'
        
        define Greeting: 'Hello, CQL!'
        define Answer: 42
        define IsTrue: true
    "#;

    let result = compile(source, None)?;

    if result.is_success() {
        println!("✓ Compilation successful!");

        let lib = &result.library;
        if let Some(id) = &lib.identifier {
            println!(
                "  Library: {} v{}",
                id.id.as_deref().unwrap_or("?"),
                id.version.as_deref().unwrap_or("?")
            );
        }

        if let Some(stmts) = &lib.statements {
            println!("  Definitions: {}", stmts.defs.len());
            for def in &stmts.defs {
                if let Some(name) = &def.name {
                    println!("    - {name}");
                }
            }
        }
    } else {
        println!("✗ Compilation failed with {} errors", result.errors.len());
        for err in &result.errors {
            println!("  Error: {}", err.message());
        }
    }

    println!();
    Ok(())
}

/// Example 2: Compilation with custom options
fn compilation_with_options() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 2: Custom Options ---");

    let source = r#"
        library OptionsExample version '1.0'
        define Sum: 1 + 2 + 3
    "#;

    // Use debug options with all signatures
    let options = CompilerOptions::debug().with_signature_level(SignatureLevel::All);

    let result = compile(source, Some(options))?;

    println!("✓ Compiled with debug options");
    println!("  Success: {}", result.is_success());
    println!("  Warnings: {}", result.warnings.len());
    println!("  Messages: {}", result.messages.len());

    println!();
    Ok(())
}

/// Example 3: Validation without full compilation
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 3: Validation ---");

    // Valid CQL
    let valid_source = "library Valid version '1.0' define X: 42";
    let result = validate(valid_source, None)?;
    println!("Valid CQL: is_valid = {}", result.is_valid());

    // Invalid CQL (parse error)
    let invalid_source = "this is not valid CQL!";
    match validate(invalid_source, None) {
        Ok(result) => println!("Invalid CQL: is_valid = {}", result.is_valid()),
        Err(e) => println!("Invalid CQL: parse error - {e}"),
    }

    println!();
    Ok(())
}

/// Example 4: Compile directly to JSON
fn direct_to_json() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 4: Direct to JSON ---");

    let source = r#"
        library JsonExample version '1.0'
        define Pi: 3.14159
    "#;

    // Pretty-printed JSON
    let json = compile_to_json(source, None, true)?;

    // Show first 500 chars
    let preview: String = json.chars().take(500).collect();
    println!("ELM JSON (preview):\n{preview}");
    if json.len() > 500 {
        println!("... ({} more characters)", json.len() - 500);
    }

    // Compact JSON (no formatting)
    let compact = compile_to_json(source, None, false)?;
    println!("\nCompact JSON length: {} bytes", compact.len());

    println!();
    Ok(())
}

/// Example 5: Complex library with multiple sections
fn complex_library() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Example 5: Complex Library ---");

    let source = r#"
        library MeasureLib version '2.0.0'
        using FHIR version '4.0.1'
        
        codesystem "LOINC": 'http://loinc.org'
        codesystem "SNOMED": 'http://snomed.info/sct'
        
        valueset "Blood Pressure Codes": 'http://example.org/vs/bp'
        
        code "Systolic BP": '8480-6' from "LOINC" display 'Systolic blood pressure'
        
        parameter MeasurementPeriod Interval<DateTime>
        
        context Patient
        
        define "Adult Patient":
            AgeInYears() >= 18
        
        define "Senior Patient":
            AgeInYears() >= 65
    "#;

    let result = compile(source, None)?;

    if result.is_success() {
        println!("✓ Complex library compiled successfully!");

        let lib = &result.library;

        // Count sections
        let usings = lib.usings.as_ref().map(|u| u.defs.len()).unwrap_or(0);
        let codesystems = lib.code_systems.as_ref().map(|c| c.defs.len()).unwrap_or(0);
        let valuesets = lib.value_sets.as_ref().map(|v| v.defs.len()).unwrap_or(0);
        let codes = lib.codes.as_ref().map(|c| c.defs.len()).unwrap_or(0);
        let parameters = lib.parameters.as_ref().map(|p| p.defs.len()).unwrap_or(0);
        let contexts = lib.contexts.as_ref().map(|c| c.defs.len()).unwrap_or(0);
        let statements = lib.statements.as_ref().map(|s| s.defs.len()).unwrap_or(0);

        println!("  Library sections:");
        println!("    Using statements:   {usings}");
        println!("    Code systems:       {codesystems}");
        println!("    Value sets:         {valuesets}");
        println!("    Codes:              {codes}");
        println!("    Parameters:         {parameters}");
        println!("    Contexts:           {contexts}");
        println!("    Expression defs:    {statements}");
    } else {
        println!("✗ Compilation failed:");
        for err in &result.errors {
            println!("  {}", err.message());
        }
    }

    println!();
    Ok(())
}
