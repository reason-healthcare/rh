//! Example: CQL Query Translation (Phase 4.5d)
//!
//! This example demonstrates how to translate CQL query expressions to ELM,
//! covering sources, let clauses, relationships, where, return, and sort.
//!
//! Run with: cargo run -p rh-cql --example query_translator

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::ExpressionTranslator;

fn main() {
    println!("=== CQL Query Translator Example ===\n");

    let mut translator = ExpressionTranslator::new();

    // ========================================================================
    // Simple Query: from Encounters E
    // ========================================================================
    println!("--- Simple Query ---\n");

    let simple_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Encounters".to_string(),
                location: None,
            })),
            alias: "E".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&simple_query, translate_expr, None);
    println!("from Encounters E");
    print_query_structure(&result);

    // ========================================================================
    // Query with Where: from Patients P where P.active
    // ========================================================================
    println!("\n--- Query with Where Clause ---\n");

    let where_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Patients".to_string(),
                location: None,
            })),
            alias: "P".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
            true,
        )))),
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&where_query, translate_expr, None);
    println!("from Patients P where P.active");
    print_query_structure(&result);

    // ========================================================================
    // Query with Return: from Conditions C return C.code
    // ========================================================================
    println!("\n--- Query with Return Clause ---\n");

    let return_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Conditions".to_string(),
                location: None,
            })),
            alias: "C".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: Some(ast::ReturnClause {
            distinct: false,
            all: false,
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "code".to_string(),
                location: None,
            })),
            location: None,
        }),
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&return_query, translate_expr, None);
    println!("from Conditions C return C.code");
    print_query_structure(&result);

    // ========================================================================
    // Query with Distinct Return: from Medications M return distinct M.code
    // ========================================================================
    println!("\n--- Query with Distinct Return ---\n");

    let distinct_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Medications".to_string(),
                location: None,
            })),
            alias: "M".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: Some(ast::ReturnClause {
            distinct: true,
            all: false,
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "code".to_string(),
                location: None,
            })),
            location: None,
        }),
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&distinct_query, translate_expr, None);
    println!("from Medications M return distinct M.code");
    print_query_structure(&result);

    // ========================================================================
    // Query with Let: from Observations O let Value = O.value
    // ========================================================================
    println!("\n--- Query with Let Clause ---\n");

    let let_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Observations".to_string(),
                location: None,
            })),
            alias: "O".to_string(),
            location: None,
        }],
        let_clauses: vec![ast::LetClause {
            identifier: "Value".to_string(),
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "value".to_string(),
                location: None,
            })),
            location: None,
        }],
        relationships: vec![],
        where_clause: None,
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&let_query, translate_expr, None);
    println!("from Observations O let Value = O.value");
    print_query_structure(&result);

    // ========================================================================
    // Query with With: from Encounters E with Diagnoses D such that ...
    // ========================================================================
    println!("\n--- Query with Relationship (With) ---\n");

    let with_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Encounters".to_string(),
                location: None,
            })),
            alias: "E".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![ast::RelationshipClause {
            kind: ast::RelationshipKind::With,
            source: ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Diagnoses".to_string(),
                    location: None,
                })),
                alias: "D".to_string(),
                location: None,
            },
            such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            location: None,
        }],
        where_clause: None,
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&with_query, translate_expr, None);
    println!("from Encounters E with Diagnoses D such that D.encounter = E.id");
    print_query_structure(&result);

    // ========================================================================
    // Query with Without: from Patients P without Allergies A such that ...
    // ========================================================================
    println!("\n--- Query with Relationship (Without) ---\n");

    let without_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Patients".to_string(),
                location: None,
            })),
            alias: "P".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![ast::RelationshipClause {
            kind: ast::RelationshipKind::Without,
            source: ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Allergies".to_string(),
                    location: None,
                })),
                alias: "A".to_string(),
                location: None,
            },
            such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            location: None,
        }],
        where_clause: None,
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&without_query, translate_expr, None);
    println!("from Patients P without Allergies A such that A.patient = P.id");
    print_query_structure(&result);

    // ========================================================================
    // Query with Sort: from Events Ev sort by date desc
    // ========================================================================
    println!("\n--- Query with Sort Clause ---\n");

    let sort_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Events".to_string(),
                location: None,
            })),
            alias: "Ev".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: None,
        sort_clause: Some(ast::SortClause {
            items: vec![ast::SortItem {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "date".to_string(),
                    location: None,
                })),
                direction: ast::SortDirection::Descending,
            }],
            location: None,
        }),
        location: None,
    };

    let result = translator.translate_query(&sort_query, translate_expr, None);
    println!("from Events Ev sort by date desc");
    print_query_structure(&result);

    // ========================================================================
    // Multi-sort Query: from Records R sort by category asc, date desc
    // ========================================================================
    println!("\n--- Query with Multiple Sort Items ---\n");

    let multi_sort_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Records".to_string(),
                location: None,
            })),
            alias: "R".to_string(),
            location: None,
        }],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: None,
        sort_clause: Some(ast::SortClause {
            items: vec![
                ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "category".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Ascending,
                },
                ast::SortItem {
                    expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                        name: "date".to_string(),
                        location: None,
                    })),
                    direction: ast::SortDirection::Descending,
                },
            ],
            location: None,
        }),
        location: None,
    };

    let result = translator.translate_query(&multi_sort_query, translate_expr, None);
    println!("from Records R sort by category asc, date desc");
    print_query_structure(&result);

    // ========================================================================
    // Complex Query: All clauses
    // ========================================================================
    println!("\n--- Complex Query (All Clauses) ---\n");

    let complex_query = ast::Query {
        sources: vec![ast::QuerySource {
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "Encounters".to_string(),
                location: None,
            })),
            alias: "E".to_string(),
            location: None,
        }],
        let_clauses: vec![ast::LetClause {
            identifier: "Duration".to_string(),
            expression: Box::new(ast::Expression::Literal(ast::Literal::Integer(30))),
            location: None,
        }],
        relationships: vec![ast::RelationshipClause {
            kind: ast::RelationshipKind::With,
            source: ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Diagnoses".to_string(),
                    location: None,
                })),
                alias: "D".to_string(),
                location: None,
            },
            such_that: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
                true,
            )))),
            location: None,
        }],
        where_clause: Some(Box::new(ast::Expression::Literal(ast::Literal::Boolean(
            true,
        )))),
        return_clause: Some(ast::ReturnClause {
            distinct: true,
            all: false,
            expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                name: "id".to_string(),
                location: None,
            })),
            location: None,
        }),
        sort_clause: Some(ast::SortClause {
            items: vec![ast::SortItem {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "date".to_string(),
                    location: None,
                })),
                direction: ast::SortDirection::Descending,
            }],
            location: None,
        }),
        location: None,
    };

    let result = translator.translate_query(&complex_query, translate_expr, None);
    println!("from Encounters E");
    println!("  let Duration = 30");
    println!("  with Diagnoses D such that D.encounter = E.id");
    println!("  where E.status = 'finished'");
    println!("  return distinct E.id");
    println!("  sort by date desc");
    print_query_structure(&result);

    // ========================================================================
    // Multi-source Query: from Encounters E, Conditions C
    // ========================================================================
    println!("\n--- Multi-Source Query ---\n");

    let multi_source_query = ast::Query {
        sources: vec![
            ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Encounters".to_string(),
                    location: None,
                })),
                alias: "E".to_string(),
                location: None,
            },
            ast::QuerySource {
                expression: Box::new(ast::Expression::IdentifierRef(ast::IdentifierRef {
                    name: "Conditions".to_string(),
                    location: None,
                })),
                alias: "C".to_string(),
                location: None,
            },
        ],
        let_clauses: vec![],
        relationships: vec![],
        where_clause: None,
        return_clause: None,
        sort_clause: None,
        location: None,
    };

    let result = translator.translate_query(&multi_source_query, translate_expr, None);
    println!("from Encounters E, Conditions C");
    print_query_structure(&result);

    println!("\n=== Query Translation Complete ===");
}

/// Helper to translate expressions in query context
fn translate_expr(
    translator: &mut ExpressionTranslator,
    expr: &ast::Expression,
) -> elm::Expression {
    match expr {
        ast::Expression::IdentifierRef(id_ref) => translator.translate_ast_identifier_ref(id_ref),
        ast::Expression::Literal(lit) => translator.translate_literal(lit),
        _ => panic!("Unexpected expression type in example"),
    }
}

/// Print the structure of a translated query
fn print_query_structure(result: &elm::Expression) {
    if let elm::Expression::Query(q) = result {
        println!("  ELM Query:");
        println!("    sources: {}", q.source.len());
        for (i, src) in q.source.iter().enumerate() {
            println!(
                "      [{i}] alias: {:?}",
                src.alias.as_deref().unwrap_or("(none)")
            );
        }

        if !q.let_clause.is_empty() {
            println!("    let clauses: {}", q.let_clause.len());
            for lc in &q.let_clause {
                println!(
                    "      - identifier: {:?}",
                    lc.identifier.as_deref().unwrap_or("(none)")
                );
            }
        }

        if !q.relationship.is_empty() {
            println!("    relationships: {}", q.relationship.len());
            for rel in &q.relationship {
                println!(
                    "      - type: {:?}, alias: {:?}, has such_that: {}",
                    rel.relationship_type.as_deref().unwrap_or("(none)"),
                    rel.alias.as_deref().unwrap_or("(none)"),
                    rel.such_that.is_some()
                );
            }
        }

        if q.where_clause.is_some() {
            println!("    where: present");
        }

        if let Some(ret) = &q.return_clause {
            println!("    return: distinct={:?}", ret.distinct.unwrap_or(false));
        }

        if let Some(sort) = &q.sort {
            println!("    sort: {} item(s)", sort.by.len());
            for item in &sort.by {
                println!(
                    "      - path: {:?}, direction: {:?}",
                    item.path.as_deref().unwrap_or("(none)"),
                    item.direction
                );
            }
        }
    } else {
        println!("  (not a Query expression)");
    }
}
