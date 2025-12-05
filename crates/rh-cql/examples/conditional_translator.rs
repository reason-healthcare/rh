//! Example: CQL Conditional (If/Case) to ELM Translation (Phase 4.5f)
//!
//! This example demonstrates CQL conditional expressions (if-then-else and case)
//! and their ELM representations.
//!
//! Run with: cargo run -p rh-cql --example conditional_translator

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::ExpressionTranslator;

fn main() {
    println!("=== CQL Conditional to ELM Translation Example ===\n");

    let mut translator = ExpressionTranslator::new();

    // Example 1: If-Then-Else from AST
    println!("1. If-Then-Else Expression");
    println!("   CQL: if true then 'yes' else 'no'");

    let ast_if = ast::IfThenElse {
        condition: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
        then_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
            "yes".to_string(),
        ))),
        else_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
            "no".to_string(),
        ))),
        location: Default::default(),
    };

    let elm_if = translator.translate_if_then_else(&ast_if, translate_literal, None);
    print_expression(&elm_if, "   ");
    println!();

    // Example 2: Case Expression (Simple - with comparand)
    println!("2. Case Expression with Comparand (Simple Case)");
    println!("   CQL: case 1");
    println!("         when 1 then 'one'");
    println!("         else 'other'");
    println!("        end");

    let ast_case_simple = ast::Case {
        comparand: Some(Box::new(ast::Expression::Literal(ast::Literal::Integer(1)))),
        items: vec![ast::CaseItem {
            when: Box::new(ast::Expression::Literal(ast::Literal::Integer(1))),
            then: Box::new(ast::Expression::Literal(ast::Literal::String(
                "one".to_string(),
            ))),
        }],
        else_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
            "other".to_string(),
        ))),
        location: Default::default(),
    };

    let elm_case_simple = translator.translate_case(&ast_case_simple, translate_literal, None);
    print_expression(&elm_case_simple, "   ");
    println!();

    // Example 3: Case Expression (Searched - no comparand)
    println!("3. Case Expression without Comparand (Searched Case)");
    println!("   CQL: case");
    println!("         when true then 'match'");
    println!("         else 'no match'");
    println!("        end");

    let ast_case_searched = ast::Case {
        comparand: None,
        items: vec![ast::CaseItem {
            when: Box::new(ast::Expression::Literal(ast::Literal::Boolean(true))),
            then: Box::new(ast::Expression::Literal(ast::Literal::String(
                "match".to_string(),
            ))),
        }],
        else_expr: Box::new(ast::Expression::Literal(ast::Literal::String(
            "no match".to_string(),
        ))),
        location: Default::default(),
    };

    let elm_case_searched = translator.translate_case(&ast_case_searched, translate_literal, None);
    print_expression(&elm_case_searched, "   ");
    println!();

    // Example 4: Builder-style if expression
    println!("4. Builder-style If Expression");
    println!("   Creating IfExpr directly with make_if()");

    let condition = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".to_string()),
        value: Some("true".to_string()),
        ..Default::default()
    });
    let then_val = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
        value: Some("42".to_string()),
        ..Default::default()
    });
    let else_val = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
        value: Some("0".to_string()),
        ..Default::default()
    });

    let if_expr = translator.make_if(condition, then_val, else_val, None);
    print_expression(&if_expr, "   ");
    println!();

    // Example 5: Builder-style case expression
    println!("5. Builder-style Case Expression");
    println!("   Creating Case directly with make_case()");

    let when1 = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
        value: Some("A".to_string()),
        ..Default::default()
    });
    let then1 = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
        value: Some("Active".to_string()),
        ..Default::default()
    });
    let else_val2 = elm::Expression::Literal(elm::Literal {
        value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
        value: Some("Unknown".to_string()),
        ..Default::default()
    });

    // make_case takes Vec<(Expression, Expression)> tuples for (when, then) pairs
    let case_items = vec![(when1, then1)];

    let case_expr = translator.make_case(None, case_items, else_val2, None);
    print_expression(&case_expr, "   ");
    println!();

    println!("=== Phase 4.5f: Conditional Translation Complete ===");
    println!("Supported conditional expressions:");
    println!("  - if-then-else (IfThenElse → IfExpr)");
    println!("  - case with comparand (simple case)");
    println!("  - case without comparand (searched case)");
    println!("  - builder-style make_if() and make_case()");
}

fn translate_literal(
    _translator: &mut ExpressionTranslator,
    expr: &ast::Expression,
) -> elm::Expression {
    match expr {
        ast::Expression::Literal(ast::Literal::Boolean(b)) => {
            elm::Expression::Literal(elm::Literal {
                value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".to_string()),
                value: Some(b.to_string()),
                ..Default::default()
            })
        }
        ast::Expression::Literal(ast::Literal::Integer(i)) => {
            elm::Expression::Literal(elm::Literal {
                value_type: Some("{urn:hl7-org:elm-types:r1}Integer".to_string()),
                value: Some(i.to_string()),
                ..Default::default()
            })
        }
        ast::Expression::Literal(ast::Literal::String(s)) => {
            elm::Expression::Literal(elm::Literal {
                value_type: Some("{urn:hl7-org:elm-types:r1}String".to_string()),
                value: Some(s.clone()),
                ..Default::default()
            })
        }
        _ => elm::Expression::Null(elm::Null::default()),
    }
}

fn print_expression(expr: &elm::Expression, indent: &str) {
    match expr {
        elm::Expression::If(if_expr) => {
            println!("{indent}ELM IfExpr:");
            println!(
                "{indent}  condition: present={}",
                if_expr.condition.is_some()
            );
            println!(
                "{indent}  then_expr: present={}",
                if_expr.then_expr.is_some()
            );
            println!(
                "{indent}  else_expr: present={}",
                if_expr.else_expr.is_some()
            );
        }
        elm::Expression::Case(case_expr) => {
            println!("{indent}ELM Case:");
            println!(
                "{indent}  comparand: {}",
                if case_expr.comparand.is_some() {
                    "present (simple case)"
                } else {
                    "none (searched case)"
                }
            );
            println!("{indent}  case_items: {}", case_expr.case_item.len());
            println!(
                "{indent}  else_expr: present={}",
                case_expr.else_expr.is_some()
            );
        }
        elm::Expression::Literal(lit) => {
            println!(
                "{indent}ELM Literal: value={:?}",
                lit.value.as_deref().unwrap_or("none")
            );
        }
        _ => {
            println!("{indent}Other expression type");
        }
    }
}
