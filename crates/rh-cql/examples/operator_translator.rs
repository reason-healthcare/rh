//! Example: CQL Operator Translation (Phase 4.5c)
//!
//! This example demonstrates how to translate CQL operators to ELM expressions,
//! covering unary, binary, and ternary operators.
//!
//! Run with: cargo run -p rh-cql --example operator_translator

use rh_cql::elm;
use rh_cql::parser::ast;
use rh_cql::translator::ExpressionTranslator;

fn main() {
    println!("=== CQL Operator Translator Example ===\n");

    let mut translator = ExpressionTranslator::new();

    // ========================================================================
    // Unary Operators
    // ========================================================================
    println!("--- Unary Operators ---\n");

    // NOT operator
    let true_val = translator.translate_literal(&ast::Literal::Boolean(true));
    let not_expr = translator.translate_unary_operator(ast::UnaryOperator::Not, true_val, None);
    println!("not true => {:?}", expr_type(&not_expr));

    // Negate operator
    let five = translator.translate_literal(&ast::Literal::Integer(5));
    let negate = translator.translate_unary_operator(ast::UnaryOperator::Negate, five, None);
    println!("-5 => {:?}", expr_type(&negate));

    // IsNull operator
    let null_val = translator.translate_literal(&ast::Literal::Null);
    let is_null = translator.translate_unary_operator(ast::UnaryOperator::IsNull, null_val, None);
    println!("null is null => {:?}", expr_type(&is_null));

    // Exists operator
    let null_val2 = translator.translate_literal(&ast::Literal::Null);
    let exists = translator.translate_unary_operator(ast::UnaryOperator::Exists, null_val2, None);
    println!("exists <list> => {:?}", expr_type(&exists));

    // Type conversion operators
    let str_val = translator.translate_literal(&ast::Literal::String("42".to_string()));
    let to_int = translator.translate_unary_operator(ast::UnaryOperator::ToInteger, str_val, None);
    println!("ToInteger('42') => {:?}", expr_type(&to_int));

    let int_val = translator.translate_literal(&ast::Literal::Integer(100));
    let to_str = translator.translate_unary_operator(ast::UnaryOperator::ToString, int_val, None);
    println!("ToString(100) => {:?}", expr_type(&to_str));

    // ========================================================================
    // Binary Operators - Arithmetic
    // ========================================================================
    println!("\n--- Binary Operators (Arithmetic) ---\n");

    let a = translator.translate_literal(&ast::Literal::Integer(10));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let add = translator.translate_binary_operator(ast::BinaryOperator::Add, a, b, None);
    println!("10 + 3 => {:?}", expr_type(&add));

    let a = translator.translate_literal(&ast::Literal::Integer(10));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let sub = translator.translate_binary_operator(ast::BinaryOperator::Subtract, a, b, None);
    println!("10 - 3 => {:?}", expr_type(&sub));

    let a = translator.translate_literal(&ast::Literal::Integer(10));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let mul = translator.translate_binary_operator(ast::BinaryOperator::Multiply, a, b, None);
    println!("10 * 3 => {:?}", expr_type(&mul));

    let a = translator.translate_literal(&ast::Literal::Decimal(10.0));
    let b = translator.translate_literal(&ast::Literal::Decimal(3.0));
    let div = translator.translate_binary_operator(ast::BinaryOperator::Divide, a, b, None);
    println!("10.0 / 3.0 => {:?}", expr_type(&div));

    let a = translator.translate_literal(&ast::Literal::Integer(10));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let modulo = translator.translate_binary_operator(ast::BinaryOperator::Modulo, a, b, None);
    println!("10 mod 3 => {:?}", expr_type(&modulo));

    let base = translator.translate_literal(&ast::Literal::Integer(2));
    let exp = translator.translate_literal(&ast::Literal::Integer(8));
    let power = translator.translate_binary_operator(ast::BinaryOperator::Power, base, exp, None);
    println!("2 ^ 8 => {:?}", expr_type(&power));

    // ========================================================================
    // Binary Operators - Comparison
    // ========================================================================
    println!("\n--- Binary Operators (Comparison) ---\n");

    let a = translator.translate_literal(&ast::Literal::Integer(5));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let eq = translator.translate_binary_operator(ast::BinaryOperator::Equal, a, b, None);
    println!("5 = 3 => {:?}", expr_type(&eq));

    let a = translator.translate_literal(&ast::Literal::Integer(5));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let neq = translator.translate_binary_operator(ast::BinaryOperator::NotEqual, a, b, None);
    println!("5 != 3 => {:?}", expr_type(&neq));

    let a = translator.translate_literal(&ast::Literal::Integer(5));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let less = translator.translate_binary_operator(ast::BinaryOperator::Less, a, b, None);
    println!("5 < 3 => {:?}", expr_type(&less));

    let a = translator.translate_literal(&ast::Literal::Integer(5));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let greater = translator.translate_binary_operator(ast::BinaryOperator::Greater, a, b, None);
    println!("5 > 3 => {:?}", expr_type(&greater));

    let a = translator.translate_literal(&ast::Literal::Integer(5));
    let b = translator.translate_literal(&ast::Literal::Integer(3));
    let equiv = translator.translate_binary_operator(ast::BinaryOperator::Equivalent, a, b, None);
    println!("5 ~ 3 => {:?}", expr_type(&equiv));

    // ========================================================================
    // Binary Operators - Logical
    // ========================================================================
    println!("\n--- Binary Operators (Logical) ---\n");

    let a = translator.translate_literal(&ast::Literal::Boolean(true));
    let b = translator.translate_literal(&ast::Literal::Boolean(false));
    let and_expr = translator.translate_binary_operator(ast::BinaryOperator::And, a, b, None);
    println!("true and false => {:?}", expr_type(&and_expr));

    let a = translator.translate_literal(&ast::Literal::Boolean(true));
    let b = translator.translate_literal(&ast::Literal::Boolean(false));
    let or_expr = translator.translate_binary_operator(ast::BinaryOperator::Or, a, b, None);
    println!("true or false => {:?}", expr_type(&or_expr));

    let a = translator.translate_literal(&ast::Literal::Boolean(true));
    let b = translator.translate_literal(&ast::Literal::Boolean(false));
    let xor_expr = translator.translate_binary_operator(ast::BinaryOperator::Xor, a, b, None);
    println!("true xor false => {:?}", expr_type(&xor_expr));

    let a = translator.translate_literal(&ast::Literal::Boolean(true));
    let b = translator.translate_literal(&ast::Literal::Boolean(true));
    let implies = translator.translate_binary_operator(ast::BinaryOperator::Implies, a, b, None);
    println!("true implies true => {:?}", expr_type(&implies));

    // ========================================================================
    // Binary Operators - String
    // ========================================================================
    println!("\n--- Binary Operators (String) ---\n");

    let hello = translator.translate_literal(&ast::Literal::String("hello".to_string()));
    let world = translator.translate_literal(&ast::Literal::String(" world".to_string()));
    let concat =
        translator.translate_binary_operator(ast::BinaryOperator::Concatenate, hello, world, None);
    println!("'hello' & ' world' => {:?}", expr_type(&concat));

    // ========================================================================
    // Binary Operators - Set Operations
    // ========================================================================
    println!("\n--- Binary Operators (Set Operations) ---\n");

    let list1 = translator.translate_literal(&ast::Literal::Null);
    let list2 = translator.translate_literal(&ast::Literal::Null);
    let union =
        translator.translate_binary_operator(ast::BinaryOperator::Union, list1, list2, None);
    println!("list1 union list2 => {:?}", expr_type(&union));

    let list1 = translator.translate_literal(&ast::Literal::Null);
    let list2 = translator.translate_literal(&ast::Literal::Null);
    let intersect =
        translator.translate_binary_operator(ast::BinaryOperator::Intersect, list1, list2, None);
    println!("list1 intersect list2 => {:?}", expr_type(&intersect));

    let list1 = translator.translate_literal(&ast::Literal::Null);
    let list2 = translator.translate_literal(&ast::Literal::Null);
    let except =
        translator.translate_binary_operator(ast::BinaryOperator::Except, list1, list2, None);
    println!("list1 except list2 => {:?}", expr_type(&except));

    // ========================================================================
    // Binary Operators - Membership
    // ========================================================================
    println!("\n--- Binary Operators (Membership) ---\n");

    let elem = translator.translate_literal(&ast::Literal::Integer(1));
    let list = translator.translate_literal(&ast::Literal::Null);
    let in_expr = translator.translate_binary_operator(ast::BinaryOperator::In, elem, list, None);
    println!("1 in list => {:?}", expr_type(&in_expr));

    let list = translator.translate_literal(&ast::Literal::Null);
    let elem = translator.translate_literal(&ast::Literal::Integer(1));
    let contains =
        translator.translate_binary_operator(ast::BinaryOperator::Contains, list, elem, None);
    println!("list contains 1 => {:?}", expr_type(&contains));

    let list1 = translator.translate_literal(&ast::Literal::Null);
    let list2 = translator.translate_literal(&ast::Literal::Null);
    let includes =
        translator.translate_binary_operator(ast::BinaryOperator::Includes, list1, list2, None);
    println!("list1 includes list2 => {:?}", expr_type(&includes));

    // ========================================================================
    // Binary Operators - Interval/Temporal
    // ========================================================================
    println!("\n--- Binary Operators (Interval/Temporal) ---\n");

    let interval1 = translator.translate_literal(&ast::Literal::Null);
    let interval2 = translator.translate_literal(&ast::Literal::Null);
    let overlaps = translator.translate_binary_operator(
        ast::BinaryOperator::Overlaps,
        interval1,
        interval2,
        None,
    );
    println!("interval1 overlaps interval2 => {:?}", expr_type(&overlaps));

    let interval1 = translator.translate_literal(&ast::Literal::Null);
    let interval2 = translator.translate_literal(&ast::Literal::Null);
    let meets = translator.translate_binary_operator(
        ast::BinaryOperator::Meets,
        interval1,
        interval2,
        None,
    );
    println!("interval1 meets interval2 => {:?}", expr_type(&meets));

    let date1 = translator.translate_literal(&ast::Literal::Null);
    let date2 = translator.translate_literal(&ast::Literal::Null);
    let before =
        translator.translate_binary_operator(ast::BinaryOperator::Before, date1, date2, None);
    println!("date1 before date2 => {:?}", expr_type(&before));

    let date1 = translator.translate_literal(&ast::Literal::Null);
    let date2 = translator.translate_literal(&ast::Literal::Null);
    let after =
        translator.translate_binary_operator(ast::BinaryOperator::After, date1, date2, None);
    println!("date1 after date2 => {:?}", expr_type(&after));

    // ========================================================================
    // Ternary Operators
    // ========================================================================
    println!("\n--- Ternary Operators ---\n");

    // Between: x between low and high
    let value = translator.translate_literal(&ast::Literal::Integer(5));
    let low = translator.translate_literal(&ast::Literal::Integer(1));
    let high = translator.translate_literal(&ast::Literal::Integer(10));
    let between = translator.translate_ternary_operator(
        ast::TernaryOperator::Between,
        value,
        low,
        high,
        None,
    );
    println!("5 between 1 and 10 => {:?}", expr_type(&between));
    // Note: Between translates to And(GreaterOrEqual, LessOrEqual)

    // ReplaceMatches: ReplaceMatches(input, pattern, replacement)
    let input = translator.translate_literal(&ast::Literal::String("hello world".to_string()));
    let pattern = translator.translate_literal(&ast::Literal::String("world".to_string()));
    let replacement = translator.translate_literal(&ast::Literal::String("universe".to_string()));
    let replace = translator.translate_ternary_operator(
        ast::TernaryOperator::ReplaceMatches,
        input,
        pattern,
        replacement,
        None,
    );
    println!(
        "ReplaceMatches('hello world', 'world', 'universe') => {:?}",
        expr_type(&replace)
    );

    // ========================================================================
    // AST Expression Translation
    // ========================================================================
    println!("\n--- AST Expression Translation ---\n");

    // Translate full AST unary expression
    let ast_unary = ast::UnaryExpression {
        operator: ast::UnaryOperator::Not,
        operand: Box::new(ast::Expression::Literal(ast::Literal::Boolean(false))),
        location: None,
    };
    let elm_unary = translator.translate_ast_unary_expression(&ast_unary, |t, expr| {
        if let ast::Expression::Literal(lit) = expr {
            t.translate_literal(lit)
        } else {
            panic!("Expected literal");
        }
    });
    println!("AST: not false => {:?}", expr_type(&elm_unary));

    // Translate full AST binary expression
    let ast_binary = ast::BinaryExpression {
        operator: ast::BinaryOperator::Add,
        left: Box::new(ast::Expression::Literal(ast::Literal::Integer(100))),
        right: Box::new(ast::Expression::Literal(ast::Literal::Integer(200))),
        location: None,
    };
    let elm_binary = translator.translate_ast_binary_expression(&ast_binary, |t, expr| {
        if let ast::Expression::Literal(lit) = expr {
            t.translate_literal(lit)
        } else {
            panic!("Expected literal");
        }
    });
    println!("AST: 100 + 200 => {:?}", expr_type(&elm_binary));

    // Translate full AST ternary expression
    let ast_ternary = ast::TernaryExpression {
        operator: ast::TernaryOperator::Between,
        first: Box::new(ast::Expression::Literal(ast::Literal::Integer(50))),
        second: Box::new(ast::Expression::Literal(ast::Literal::Integer(0))),
        third: Box::new(ast::Expression::Literal(ast::Literal::Integer(100))),
        location: None,
    };
    let elm_ternary = translator.translate_ast_ternary_expression(&ast_ternary, |t, expr| {
        if let ast::Expression::Literal(lit) = expr {
            t.translate_literal(lit)
        } else {
            panic!("Expected literal");
        }
    });
    println!("AST: 50 between 0 and 100 => {:?}", expr_type(&elm_ternary));

    println!("\n=== Operator Translation Complete ===");
}

/// Helper function to get the expression type name
fn expr_type(expr: &elm::Expression) -> &'static str {
    match expr {
        elm::Expression::Not(_) => "Not",
        elm::Expression::Negate(_) => "Negate",
        elm::Expression::IsNull(_) => "IsNull",
        elm::Expression::IsTrue(_) => "IsTrue",
        elm::Expression::IsFalse(_) => "IsFalse",
        elm::Expression::Exists(_) => "Exists",
        elm::Expression::Predecessor(_) => "Predecessor",
        elm::Expression::Successor(_) => "Successor",
        elm::Expression::Distinct(_) => "Distinct",
        elm::Expression::Flatten(_) => "Flatten",
        elm::Expression::ToInteger(_) => "ToInteger",
        elm::Expression::ToStringExpr(_) => "ToString",
        elm::Expression::ToBoolean(_) => "ToBoolean",
        elm::Expression::ToDecimal(_) => "ToDecimal",
        elm::Expression::ToDate(_) => "ToDate",
        elm::Expression::ToDateTime(_) => "ToDateTime",
        elm::Expression::ToTime(_) => "ToTime",
        elm::Expression::ToQuantity(_) => "ToQuantity",
        elm::Expression::Add(_) => "Add",
        elm::Expression::Subtract(_) => "Subtract",
        elm::Expression::Multiply(_) => "Multiply",
        elm::Expression::Divide(_) => "Divide",
        elm::Expression::TruncatedDivide(_) => "TruncatedDivide",
        elm::Expression::Modulo(_) => "Modulo",
        elm::Expression::Power(_) => "Power",
        elm::Expression::Log(_) => "Log",
        elm::Expression::Equal(_) => "Equal",
        elm::Expression::NotEqual(_) => "NotEqual",
        elm::Expression::Equivalent(_) => "Equivalent",
        elm::Expression::Less(_) => "Less",
        elm::Expression::LessOrEqual(_) => "LessOrEqual",
        elm::Expression::Greater(_) => "Greater",
        elm::Expression::GreaterOrEqual(_) => "GreaterOrEqual",
        elm::Expression::And(_) => "And",
        elm::Expression::Or(_) => "Or",
        elm::Expression::Xor(_) => "Xor",
        elm::Expression::Implies(_) => "Implies",
        elm::Expression::Concatenate(_) => "Concatenate",
        elm::Expression::Union(_) => "Union",
        elm::Expression::Intersect(_) => "Intersect",
        elm::Expression::Except(_) => "Except",
        elm::Expression::In(_) => "In",
        elm::Expression::Contains(_) => "Contains",
        elm::Expression::Includes(_) => "Includes",
        elm::Expression::IncludedIn(_) => "IncludedIn",
        elm::Expression::ProperIncludes(_) => "ProperIncludes",
        elm::Expression::ProperIncludedIn(_) => "ProperIncludedIn",
        elm::Expression::Overlaps(_) => "Overlaps",
        elm::Expression::Meets(_) => "Meets",
        elm::Expression::Before(_) => "Before",
        elm::Expression::After(_) => "After",
        elm::Expression::SameAs(_) => "SameAs",
        elm::Expression::SameOrBefore(_) => "SameOrBefore",
        elm::Expression::SameOrAfter(_) => "SameOrAfter",
        elm::Expression::Starts(_) => "Starts",
        elm::Expression::Ends(_) => "Ends",
        elm::Expression::ReplaceMatches(_) => "ReplaceMatches",
        _ => "Other",
    }
}
