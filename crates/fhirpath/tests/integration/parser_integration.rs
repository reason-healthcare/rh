//! Parser integration tests
//! 
//! Tests for parsing various FHIRPath expressions without evaluation

use super::*;

#[test]
fn test_basic_expression_parsing() {
    let parser = FhirPathParser::new();

    // Test various FHIRPath expressions
    let expressions = vec![
        "Patient",
        "Patient.name",
        "Patient.name[0]",
        "Patient.name.given",
        "Patient.name.where(use = 'official')",
        "Patient.birthDate",
        "name.count()",
        "name.exists()",
        "name.empty()",
        "true",
        "false",
        "42",
        "3.14",
        "'hello world'",
        "{}",
        "$this",
        "$index",
        "%context",
        "name.given | name.family",
        "active = true",
        "birthDate >= @1980-01-01",
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => {
                println!("✓ Parsed: {expr_str} -> {expr}");
            }
            Err(e) => {
                println!("✗ Failed to parse: {expr_str} -> {e:?}");
                panic!("Failed to parse valid expression: {expr_str}");
            }
        }
    }
}

#[test]
fn test_function_parsing() {
    let parser = FhirPathParser::new();

    let function_cases = vec![
        "name.count()",
        "name.exists()",
        "name.empty()",
        "telecom.empty()",
        "name.distinct()",
        "name.isDistinct()",
        "name.where(use = 'official')",
        "name.select(family)",
        "name.select(given)",
        "name.where(family = 'Doe')",
    ];

    for expr_str in function_cases {
        match parser.parse(expr_str) {
            Ok(expr) => {
                println!("✓ Parsed function: {expr_str} -> {expr}");
            }
            Err(e) => {
                panic!("Failed to parse function {expr_str}: {e:?}");
            }
        }
    }
}

#[test]
fn test_complex_expression_parsing() {
    let parser = FhirPathParser::new();

    let complex_cases = vec![
        "name.where(use = 'official').given",
        "name.given | name.family",
        "name.where(family = 'Doe').select(given)",
        "Patient.name[0].given.first()",
        "telecom.where(system = 'phone').value",
    ];

    for expr_str in complex_cases {
        match parser.parse(expr_str) {
            Ok(expr) => {
                println!("✓ Parsed complex: {expr_str} -> {expr}");
            }
            Err(e) => {
                panic!("Failed to parse complex expression {expr_str}: {e:?}");
            }
        }
    }
}

#[test]
fn test_precedence_parsing() {
    let parser = FhirPathParser::new();

    let precedence_cases = vec![
        "a = b in collection",       // Should parse as (a = b) in collection
        "value contains 'x' = true", // Should parse as (value contains 'x') = true
        "2 + 3 * 4",                // Should parse as 2 + (3 * 4)
        "10 - 5 < 8",               // Should parse as (10 - 5) < 8
    ];

    for expr_str in precedence_cases {
        match parser.parse(expr_str) {
            Ok(expr) => {
                println!("✓ Parsed precedence: {expr_str} -> {expr}");
            }
            Err(e) => {
                panic!("Failed to parse precedence expression {expr_str}: {e:?}");
            }
        }
    }
}
