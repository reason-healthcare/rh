//! Example: Parsing ELM JSON
//!
//! This example demonstrates how to parse ELM (Expression Logical Model) JSON
//! into Rust types. ELM is the machine-readable representation of CQL.
//!
//! Run with: `cargo run --example parse_elm`

use rh_cql::elm::{Expression, Library, Literal};

fn main() -> anyhow::Result<()> {
    // Example 1: Parse a simple ELM library
    let library_json = r#"{
        "identifier": {
            "id": "TestLibrary",
            "version": "1.0.0"
        },
        "schemaIdentifier": {
            "id": "urn:hl7-org:elm",
            "version": "r1"
        },
        "usings": {
            "def": [
                {
                    "localIdentifier": "FHIR",
                    "uri": "http://hl7.org/fhir",
                    "version": "4.0.1"
                }
            ]
        },
        "statements": {
            "def": [
                {
                    "name": "Patient",
                    "context": "Patient",
                    "expression": {
                        "type": "ExpressionRef",
                        "name": "Patient"
                    }
                },
                {
                    "name": "IsAdult",
                    "context": "Patient",
                    "accessLevel": "Public",
                    "expression": {
                        "type": "GreaterOrEqual",
                        "operand": [
                            {
                                "type": "CalculateAgeAt",
                                "operand": [
                                    {
                                        "type": "Property",
                                        "path": "birthDate",
                                        "source": {
                                            "type": "ExpressionRef",
                                            "name": "Patient"
                                        }
                                    },
                                    {
                                        "type": "Today"
                                    }
                                ]
                            },
                            {
                                "type": "Literal",
                                "valueType": "{urn:hl7-org:elm-types:r1}Integer",
                                "value": "18"
                            }
                        ]
                    }
                }
            ]
        }
    }"#;

    let library: Library = serde_json::from_str(library_json)?;

    println!("=== Parsed ELM Library ===");
    println!(
        "Library: {} v{}",
        library
            .identifier
            .as_ref()
            .and_then(|id| id.id.as_deref())
            .unwrap_or("unknown"),
        library
            .identifier
            .as_ref()
            .and_then(|id| id.version.as_deref())
            .unwrap_or("unknown")
    );

    if let Some(usings) = &library.usings {
        println!("\nUsing declarations:");
        for using in &usings.defs {
            println!(
                "  - {} ({})",
                using.local_identifier.as_deref().unwrap_or("?"),
                using.uri.as_deref().unwrap_or("?")
            );
        }
    }

    if let Some(statements) = &library.statements {
        println!("\nExpression definitions:");
        for def in &statements.defs {
            println!(
                "  - {} (context: {})",
                def.name.as_deref().unwrap_or("?"),
                def.context.as_deref().unwrap_or("Unfiltered")
            );
        }
    }

    // Example 2: Parse individual expressions
    println!("\n=== Individual Expressions ===");

    // Parse a literal expression
    let literal_json = r#"{
        "type": "Literal",
        "valueType": "{urn:hl7-org:elm-types:r1}Boolean",
        "value": "true"
    }"#;
    let expr: Expression = serde_json::from_str(literal_json)?;
    println!("Literal expression: {expr:?}");

    // Parse a comparison expression
    let comparison_json = r#"{
        "type": "Greater",
        "operand": [
            {
                "type": "Literal",
                "valueType": "{urn:hl7-org:elm-types:r1}Integer",
                "value": "10"
            },
            {
                "type": "Literal",
                "valueType": "{urn:hl7-org:elm-types:r1}Integer",
                "value": "5"
            }
        ]
    }"#;
    let expr: Expression = serde_json::from_str(comparison_json)?;
    println!("Comparison expression: {expr:?}");

    // Example 3: Create expressions programmatically
    println!("\n=== Create Expressions ===");

    let my_literal = Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
        value: Some("Hello, ELM!".to_string()),
        ..Default::default()
    };
    let my_expr = Expression::Literal(my_literal);

    // Serialize back to JSON
    let json = serde_json::to_string_pretty(&my_expr)?;
    println!("Created expression as JSON:\n{json}");

    Ok(())
}
