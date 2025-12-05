//! Example: Literal Translation (Phase 4.5a)
//!
//! Demonstrates translating CQL AST literals to ELM expressions.
//!
//! Run with: `cargo run --example literal_translator`

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::ExpressionTranslator;

fn main() {
    println!("=== CQL Literal Translation Example ===\n");

    // Create a translator
    let mut translator = ExpressionTranslator::new();

    // -------------------------------------------------------------------------
    // Null Literal
    // -------------------------------------------------------------------------
    println!("--- Null Literal ---");
    let null_lit = ast::Literal::Null;
    let null_elm = translator.translate_literal(&null_lit);
    println!("CQL: null");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&null_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Boolean Literals
    // -------------------------------------------------------------------------
    println!("--- Boolean Literals ---");
    let true_lit = ast::Literal::Boolean(true);
    let true_elm = translator.translate_literal(&true_lit);
    println!("CQL: true");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&true_elm).unwrap()
    );

    let false_lit = ast::Literal::Boolean(false);
    let false_elm = translator.translate_literal(&false_lit);
    println!("CQL: false");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&false_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Integer Literal
    // -------------------------------------------------------------------------
    println!("--- Integer Literal ---");
    let int_lit = ast::Literal::Integer(42);
    let int_elm = translator.translate_literal(&int_lit);
    println!("CQL: 42");
    println!("ELM: {}\n", serde_json::to_string_pretty(&int_elm).unwrap());

    // -------------------------------------------------------------------------
    // Long Literal
    // -------------------------------------------------------------------------
    println!("--- Long Literal ---");
    let long_lit = ast::Literal::Long(9999999999);
    let long_elm = translator.translate_literal(&long_lit);
    println!("CQL: 9999999999L");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&long_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Decimal Literal
    // -------------------------------------------------------------------------
    println!("--- Decimal Literal ---");
    let decimal_lit = ast::Literal::Decimal(3.5);
    let decimal_elm = translator.translate_literal(&decimal_lit);
    println!("CQL: 3.5");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&decimal_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // String Literal
    // -------------------------------------------------------------------------
    println!("--- String Literal ---");
    let string_lit = ast::Literal::String("Hello, CQL!".to_string());
    let string_elm = translator.translate_literal(&string_lit);
    println!("CQL: 'Hello, CQL!'");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&string_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Date Literal
    // -------------------------------------------------------------------------
    println!("--- Date Literal ---");
    let date_lit = ast::Literal::Date("2024-01-15".to_string());
    let date_elm = translator.translate_literal(&date_lit);
    println!("CQL: @2024-01-15");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&date_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // DateTime Literal
    // -------------------------------------------------------------------------
    println!("--- DateTime Literal ---");
    let datetime_lit = ast::Literal::DateTime("2024-01-15T10:30:00".to_string());
    let datetime_elm = translator.translate_literal(&datetime_lit);
    println!("CQL: @2024-01-15T10:30:00");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&datetime_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Time Literal
    // -------------------------------------------------------------------------
    println!("--- Time Literal ---");
    let time_lit = ast::Literal::Time("10:30:00".to_string());
    let time_elm = translator.translate_literal(&time_lit);
    println!("CQL: @T10:30:00");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&time_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Quantity Literal
    // -------------------------------------------------------------------------
    println!("--- Quantity Literal ---");
    let quantity_lit = ast::Literal::Quantity {
        value: 100.0,
        unit: "mg".to_string(),
    };
    let quantity_elm = translator.translate_literal(&quantity_lit);
    println!("CQL: 100 'mg'");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&quantity_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Ratio Literal
    // -------------------------------------------------------------------------
    println!("--- Ratio Literal ---");
    let ratio_lit = ast::Literal::Ratio {
        numerator: Box::new(ast::Literal::Quantity {
            value: 1.0,
            unit: "mg".to_string(),
        }),
        denominator: Box::new(ast::Literal::Quantity {
            value: 1.0,
            unit: "mL".to_string(),
        }),
    };
    let ratio_elm = translator.translate_literal(&ratio_lit);
    println!("CQL: 1 'mg' : 1 'mL'");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&ratio_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Code Literal
    // -------------------------------------------------------------------------
    println!("--- Code Literal ---");
    let code_lit = ast::Literal::Code {
        code: "29463-7".to_string(),
        system: Some("LOINC".to_string()),
        display: Some("Body Weight".to_string()),
    };
    let code_elm = translator.translate_literal(&code_lit);
    println!("CQL: Code '29463-7' from \"LOINC\" display 'Body Weight'");
    println!(
        "ELM: {}\n",
        serde_json::to_string_pretty(&code_elm).unwrap()
    );

    // -------------------------------------------------------------------------
    // Local ID Generation
    // -------------------------------------------------------------------------
    println!("--- Local ID Generation ---");
    let mut translator_with_ids = ExpressionTranslator::new().with_local_ids();
    let lit1 = translator_with_ids.translate_literal(&ast::Literal::Integer(1));
    let lit2 = translator_with_ids.translate_literal(&ast::Literal::Integer(2));
    let lit3 = translator_with_ids.translate_literal(&ast::Literal::Integer(3));

    println!("With local IDs enabled, each expression gets a unique ID:");
    if let elm::Expression::Literal(l) = &lit1 {
        println!("  Integer 1 -> localId: {:?}", l.element.local_id);
    }
    if let elm::Expression::Literal(l) = &lit2 {
        println!("  Integer 2 -> localId: {:?}", l.element.local_id);
    }
    if let elm::Expression::Literal(l) = &lit3 {
        println!("  Integer 3 -> localId: {:?}", l.element.local_id);
    }

    // -------------------------------------------------------------------------
    // Type Inference
    // -------------------------------------------------------------------------
    println!("\n--- Type Inference ---");
    let literals = vec![
        ("null", ast::Literal::Null),
        ("true", ast::Literal::Boolean(true)),
        ("42", ast::Literal::Integer(42)),
        ("9999999999L", ast::Literal::Long(9999999999)),
        ("3.5", ast::Literal::Decimal(3.5)),
        ("'hello'", ast::Literal::String("hello".to_string())),
        ("@2024-01-15", ast::Literal::Date("2024-01-15".to_string())),
        (
            "@2024-01-15T10:30:00",
            ast::Literal::DateTime("2024-01-15T10:30:00".to_string()),
        ),
        ("@T10:30:00", ast::Literal::Time("10:30:00".to_string())),
        (
            "100 'mg'",
            ast::Literal::Quantity {
                value: 100.0,
                unit: "mg".to_string(),
            },
        ),
    ];

    println!("Literal type inference:");
    for (cql, lit) in literals {
        let datatype = ExpressionTranslator::literal_type(&lit);
        println!("  {cql} -> {datatype}");
    }

    println!("\n=== Example Complete ===");
}
