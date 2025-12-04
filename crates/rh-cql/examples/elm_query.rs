//! Example: ELM Query Expressions
//!
//! This example demonstrates working with ELM Query expressions, which are
//! the foundation of CQL's list processing capabilities.
//!
//! Run with: `cargo run --example elm_query`

use rh_cql::elm::{Expression, Query, Retrieve};

fn main() -> anyhow::Result<()> {
    // Example: A CQL query like:
    //   [Condition: "Diabetes"] C
    //   where C.clinicalStatus ~ 'active'
    //   return C.code

    let query_json = r#"{
        "type": "Query",
        "source": [
            {
                "alias": "C",
                "expression": {
                    "type": "Retrieve",
                    "dataType": "{http://hl7.org/fhir}Condition",
                    "codeProperty": "code",
                    "codes": {
                        "type": "ValueSetRef",
                        "name": "Diabetes"
                    }
                }
            }
        ],
        "relationship": [],
        "where": {
            "type": "Equivalent",
            "operand": [
                {
                    "type": "Property",
                    "path": "clinicalStatus",
                    "scope": "C"
                },
                {
                    "type": "Code",
                    "code": "active",
                    "system": {
                        "type": "CodeSystemRef",
                        "name": "ConditionClinicalStatusCodes"
                    }
                }
            ]
        },
        "return": {
            "expression": {
                "type": "Property",
                "path": "code",
                "scope": "C"
            }
        }
    }"#;

    let expr: Expression = serde_json::from_str(query_json)?;

    println!("=== Parsed Query Expression ===");
    if let Expression::Query(query) = &expr {
        analyze_query(query);
    }

    // Example: A simple Retrieve expression
    // [Patient]
    let retrieve_json = r#"{
        "type": "Retrieve",
        "dataType": "{http://hl7.org/fhir}Patient"
    }"#;

    let expr: Expression = serde_json::from_str(retrieve_json)?;
    println!("\n=== Simple Retrieve ===");
    if let Expression::Retrieve(retrieve) = &expr {
        analyze_retrieve(retrieve);
    }

    // Example: Retrieve with code filter
    // [Observation: "LDL Cholesterol"]
    let filtered_retrieve_json = r#"{
        "type": "Retrieve",
        "dataType": "{http://hl7.org/fhir}Observation",
        "codeProperty": "code",
        "codes": {
            "type": "ValueSetRef",
            "name": "LDL Cholesterol"
        }
    }"#;

    let expr: Expression = serde_json::from_str(filtered_retrieve_json)?;
    println!("\n=== Filtered Retrieve ===");
    if let Expression::Retrieve(retrieve) = &expr {
        analyze_retrieve(retrieve);
    }

    // Serialize to demonstrate round-trip
    let json = serde_json::to_string_pretty(&expr)?;
    println!("\n=== Round-trip JSON ===");
    println!("{json}");

    Ok(())
}

fn analyze_query(query: &Query) {
    println!("Query structure:");

    // Analyze sources
    println!("  Sources:");
    for source in &query.source {
        println!(
            "    - alias: {}",
            source.alias.as_deref().unwrap_or("(none)")
        );
        if let Some(expr) = &source.expression {
            println!("      expression type: {}", expression_type(expr));
        }
    }

    // Analyze relationships
    if !query.relationship.is_empty() {
        println!("  Relationships: {}", query.relationship.len());
    }

    // Analyze where clause
    if let Some(where_clause) = &query.where_clause {
        println!("  Where clause: {}", expression_type(where_clause));
    }

    // Analyze return clause
    if let Some(return_clause) = &query.return_clause {
        if let Some(expr) = &return_clause.expression {
            println!("  Return: {}", expression_type(expr));
        }
    }

    // Analyze sort clause
    if let Some(sort) = &query.sort {
        println!("  Sort by: {} items", sort.by.len());
    }
}

fn analyze_retrieve(retrieve: &Retrieve) {
    println!(
        "  Data type: {}",
        retrieve.data_type.as_deref().unwrap_or("(none)")
    );

    if let Some(code_prop) = &retrieve.code_property {
        println!("  Code property: {code_prop}");
    }

    if let Some(codes) = &retrieve.codes {
        println!("  Code filter: {}", expression_type(codes));
    }

    if let Some(context) = &retrieve.context {
        println!("  Context: {}", expression_type(context));
    }
}

fn expression_type(expr: &Expression) -> &'static str {
    match expr {
        Expression::Literal(_) => "Literal",
        Expression::Null(_) => "Null",
        Expression::ExpressionRef(_) => "ExpressionRef",
        Expression::FunctionRef(_) => "FunctionRef",
        Expression::ParameterRef(_) => "ParameterRef",
        Expression::ValueSetRef(_) => "ValueSetRef",
        Expression::CodeSystemRef(_) => "CodeSystemRef",
        Expression::CodeRef(_) => "CodeRef",
        Expression::Property(_) => "Property",
        Expression::Query(_) => "Query",
        Expression::Retrieve(_) => "Retrieve",
        Expression::If(_) => "If",
        Expression::And(_) => "And",
        Expression::Or(_) => "Or",
        Expression::Not(_) => "Not",
        Expression::Equal(_) => "Equal",
        Expression::Equivalent(_) => "Equivalent",
        Expression::Greater(_) => "Greater",
        Expression::Less(_) => "Less",
        Expression::Add(_) => "Add",
        Expression::Subtract(_) => "Subtract",
        Expression::Code(_) => "Code",
        _ => "Other",
    }
}
