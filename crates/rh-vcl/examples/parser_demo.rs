//! Example demonstrating VCL parsing capabilities
//!
//! This example shows how to parse various VCL expressions and extract
//! information from the resulting AST.

use rh_vcl::ast::SubExpressionContent;
use rh_vcl::{parse_vcl, Operation, SimpleExpression, VclExpression};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example VCL expressions to parse
    let examples = vec![
        "*",                                                               // All codes
        "123456",                                                          // Simple code
        "(http://snomed.info/sct)123456",                                  // Code with system URI
        "status = \"active\"",                                             // Property filter
        "category << 123456",                                              // Is-a filter
        "^http://example.org/valueset",                                    // Include ValueSet
        "code1, code2, code3",                                             // Conjunction
        "code1; code2; code3",                                             // Disjunction
        "* - inactive",                                                    // Exclusion
        "(code1, code2)",                                                  // Nested expression
        "(http://snomed.info/sct)status = \"active\", category << 123456", // Complex example
        "*.category",                                                      // Of operation
        "{code1, code2}.property", // Code list with property
    ];

    println!("VCL Parser Examples\n");
    println!("==================\n");

    for (i, vcl_expr) in examples.iter().enumerate() {
        println!("Example {}: {}", i + 1, vcl_expr);

        match parse_vcl(vcl_expr) {
            Ok(parsed) => {
                analyze_expression(&parsed);
            }
            Err(e) => {
                println!("  ❌ Parse error: {}", e);
            }
        }

        println!(); // Empty line for readability
    }

    // Demonstrate error handling
    println!("Error Handling Examples\n");
    println!("======================\n");

    let error_examples = vec![
        "",                  // Empty
        "   ",               // Whitespace only
        "(unclosed",         // Unclosed parentheses
        "code1 $ code2",     // Invalid operator
        "\"unclosed string", // Unclosed quote
    ];

    for (i, vcl_expr) in error_examples.iter().enumerate() {
        println!("Error example {}: '{}'", i + 1, vcl_expr);

        match parse_vcl(vcl_expr) {
            Ok(_) => {
                println!("  ⚠️ Unexpected success!");
            }
            Err(e) => {
                println!("  ❌ Expected error: {}", e);
            }
        }

        println!(); // Empty line for readability
    }

    Ok(())
}

/// Analyze and display information about a parsed VCL expression
fn analyze_expression(expr: &VclExpression) {
    println!("  ✅ Parsed successfully!");

    // Check for wildcards
    if expr.contains_wildcards() {
        println!("  🃏 Contains wildcards");
    }

    // Display system URIs
    let uris = expr.system_uris();
    if !uris.is_empty() {
        println!("  🌐 System URIs: {}", uris.join(", "));
    }

    // Display codes
    let codes = expr.codes();
    if !codes.is_empty() {
        let code_strs: Vec<String> = codes.iter().map(|c| c.to_string()).collect();
        println!("  📋 Codes: {}", code_strs.join(", "));
    }

    // Analyze the main sub-expression type
    match &expr.expr.sub_expr.content {
        SubExpressionContent::Simple(simple) => match simple {
            SimpleExpression::Wildcard => println!("  🔍 Type: Wildcard"),
            SimpleExpression::Code(_) => println!("  🔍 Type: Simple code"),
            SimpleExpression::Filter(_) => println!("  🔍 Type: Filter expression"),
            SimpleExpression::IncludeValueSet(_) => println!("  🔍 Type: Include ValueSet"),
        },
        SubExpressionContent::Nested(_) => println!("  🔍 Type: Nested expression"),
    }

    // Check for operations
    if let Some(op) = &expr.expr.operation {
        match op {
            Operation::Conjunction(_) => println!("  ➕ Has conjunction (AND)"),
            Operation::Disjunction(_) => println!("  ⚡ Has disjunction (OR)"),
            Operation::Exclusion(_) => println!("  ➖ Has exclusion (NOT)"),
        }
    }
}
