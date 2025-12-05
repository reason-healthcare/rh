//! Example: CQL Parser Foundation
//!
//! This example demonstrates the lexer utilities for CQL parsing,
//! including tokenization of keywords, identifiers, operators, and literals.
//!
//! Run with: `cargo run -p rh-cql --example cql_lexer`

use nom::InputTake;
use rh_cql::parser::lexer::{
    date_literal, datetime_literal, decimal_literal, identifier, integer_literal, is_keyword,
    keyword, long_literal, operator, qualified_identifier, quantity_literal, skip_ws_and_comments,
    string_literal, time_literal,
};
use rh_cql::parser::span::Span;

fn main() {
    println!("=== CQL Lexer Foundation Example ===\n");

    // Example 1: Keyword Recognition
    println!("1. Keyword Recognition:");
    let keywords = ["define", "library", "using", "context", "where", "return"];
    let non_keywords = ["foo", "myFunction", "Patient", "getValue"];

    for kw in &keywords {
        let is_kw = is_keyword(kw);
        println!("   '{kw}' is keyword: {is_kw}");
    }
    for id in &non_keywords {
        let is_kw = is_keyword(id);
        println!("   '{id}' is keyword: {is_kw}");
    }

    // Example 2: Parsing Identifiers
    println!("\n2. Parsing Identifiers:");
    let id_examples = ["patientName", "_private", "camelCase123", "FHIR.Patient"];

    for input in &id_examples {
        let span = Span::new(input);
        if let Ok((rest, id)) = identifier(span) {
            println!(
                "   '{input}' -> identifier: '{id}', remaining: '{}'",
                rest.fragment()
            );
        }
    }

    // Qualified identifiers
    println!("\n   Qualified identifiers:");
    let qualified_examples = ["FHIR.Patient", "System.String", "Common.Helpers.GetAge"];
    for input in &qualified_examples {
        let span = Span::new(input);
        if let Ok((_, parts)) = qualified_identifier(span) {
            println!("   '{input}' -> {parts:?}");
        }
    }

    // Example 3: Operators
    println!("\n3. Operators:");
    let op_examples = ["+", "-", "*", "/", "=", "!=", "<=", ">=", "~", "!~", "->"];

    for input in &op_examples {
        let span = Span::new(input);
        if let Ok((_, op)) = operator(span) {
            println!("   '{input}' -> {op:?} (display: {op})");
        }
    }

    // Example 4: String Literals
    println!("\n4. String Literals:");
    let string_examples = [
        "'hello world'",
        "'with\\'escaped\\'quotes'",
        "'line\\nbreak'",
        "'日本語'",
    ];

    for input in &string_examples {
        let span = Span::new(input);
        if let Ok((_, s)) = string_literal(span) {
            println!("   {input} -> \"{s}\"");
        }
    }

    // Example 5: Number Literals
    println!("\n5. Number Literals:");

    // Integers
    let int_examples = ["42", "-123", "0"];
    for input in &int_examples {
        let span = Span::new(input);
        if let Ok((_, n)) = integer_literal(span) {
            println!("   Integer: '{input}' -> {n}");
        }
    }

    // Longs
    let long_examples = ["42L", "-9999999999L"];
    for input in &long_examples {
        let span = Span::new(input);
        if let Ok((_, n)) = long_literal(span) {
            println!("   Long: '{input}' -> {n}");
        }
    }

    // Decimals
    let decimal_examples = ["3.14159", "-2.5", "0.001"];
    for input in &decimal_examples {
        let span = Span::new(input);
        if let Ok((_, n)) = decimal_literal(span) {
            println!("   Decimal: '{input}' -> {n}");
        }
    }

    // Example 6: Quantity Literals
    println!("\n6. Quantity Literals:");
    let quantity_examples = ["5 'mg'", "10.5'kg'", "98.6 '[degF]'", "100 '[lb_av]'"];

    for input in &quantity_examples {
        let span = Span::new(input);
        if let Ok((_, (value, unit))) = quantity_literal(span) {
            println!("   '{input}' -> {value} {unit}");
        }
    }

    // Example 7: Date/Time Literals
    println!("\n7. Date/Time Literals:");

    // Dates
    println!("   Dates:");
    let date_examples = ["@2024-01-15", "@2024-01", "@2024"];
    for input in &date_examples {
        let span = Span::new(input);
        if let Ok((_, d)) = date_literal(span) {
            println!("      '{input}' -> {d}");
        }
    }

    // Times
    println!("   Times:");
    let time_examples = ["@T14:30:00", "@T14:30", "@T14:30:00.123"];
    for input in &time_examples {
        let span = Span::new(input);
        if let Ok((_, t)) = time_literal(span) {
            println!("      '{input}' -> {t}");
        }
    }

    // DateTimes
    println!("   DateTimes:");
    let datetime_examples = [
        "@2024-01-15T14:30:00",
        "@2024-01-15T14:30:00Z",
        "@2024-01-15T14:30:00-05:00",
        "@2024-01-15T14:30:00.123Z",
    ];
    for input in &datetime_examples {
        let span = Span::new(input);
        if let Ok((_, dt)) = datetime_literal(span) {
            println!("      '{input}' -> {dt}");
        }
    }

    // Example 8: Whitespace and Comments
    println!("\n8. Whitespace and Comment Handling:");
    let ws_examples = [
        "  identifier",
        "// line comment\nidentifier",
        "/* block comment */ identifier",
        "  \n// comment\n/* block */\n  identifier",
    ];

    for input in &ws_examples {
        let span = Span::new(input);
        if let Ok((rest, _)) = skip_ws_and_comments(span) {
            let display_input = input.replace('\n', "\\n");
            println!("   '{display_input}' -> '{}'", rest.fragment());
        }
    }

    // Example 9: Keyword Matching
    println!("\n9. Keyword Matching (case-insensitive):");
    let kw_examples = [
        ("define Foo", "define"),
        ("DEFINE Foo", "define"),
        ("Define Foo", "define"),
    ];

    for (input, kw) in &kw_examples {
        let span = Span::new(input);
        if let Ok((rest, matched)) = keyword(kw)(span) {
            println!(
                "   '{input}' with keyword '{kw}' -> matched '{}', rest: '{}'",
                matched.fragment(),
                rest.fragment()
            );
        }
    }

    // Example 10: Source Location Tracking
    println!("\n10. Source Location Tracking:");
    let source = "line1\nline2\nline3";
    let span = Span::new(source);

    println!("   Initial: line={}, column={}", span.line(), span.column());

    // Simulate consuming "line1\n"
    let (rest, _) = span.take_split(6);
    println!(
        "   After 'line1\\n': line={}, column={}",
        rest.line(),
        rest.column()
    );

    println!("\n=== Example Complete ===");
}
