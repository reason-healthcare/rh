//! Conformance comparison tests for the ELM emitter.
//!
//! These tests parse sample CQL, run semantic analysis, then emit ELM via
//! `ElmEmitter::emit_expression` and verify the structural output against
//! known expectations derived from the Java reference translator.
//!
//! Task 4.16: Write conformance comparison tests — emit ELM from sample CQL,
//! compare with Java reference translator output for the comparison files.

use rh_cql::elm;
use rh_cql::emit::{datatype_to_qname, ElmEmitter};
use rh_cql::options::{CompilerOption, CompilerOptions};
use rh_cql::parser::CqlParser;
use rh_cql::provider::{MemoryModelInfoProvider, ModelInfoProvider};
use rh_cql::semantics::analyzer::SemanticAnalyzer;
use rh_cql::semantics::typed_ast::TypedStatement;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn analyze(
    cql: &str,
) -> (
    rh_cql::semantics::typed_ast::TypedLibrary,
    Vec<rh_cql::CqlCompilerException>,
) {
    let parser = CqlParser::new();
    let ast_lib = parser.parse(cql).expect("parse failed");
    let provider: Arc<dyn ModelInfoProvider> = Arc::new(MemoryModelInfoProvider::new());
    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let analyzer = SemanticAnalyzer::new(provider, opts);
    analyzer.analyze(ast_lib)
}

fn emitter_with_result_types() -> ElmEmitter {
    ElmEmitter::new(CompilerOptions::new().with_option(CompilerOption::EnableResultTypes))
}

/// Extract the typed expression body of the first `ExpressionDef` statement.
fn first_expr_body(
    lib: &rh_cql::semantics::typed_ast::TypedLibrary,
) -> &rh_cql::semantics::typed_ast::TypedNode<rh_cql::semantics::typed_ast::TypedExpression> {
    for stmt in &lib.statements {
        if let TypedStatement::ExpressionDef { body, .. } = &stmt.inner {
            return body;
        }
    }
    panic!("No ExpressionDef found in typed library");
}

// ---------------------------------------------------------------------------
// Comparison file test-0: arithmetic expression
// Matches `test-0-input.cql` (simple integer add)
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_integer_add() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: 1 + 2\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    // Should produce Add(BinaryExpression) with two integer literal operands.
    if let elm::Expression::Add(bin) = &expr {
        assert_eq!(bin.operand.len(), 2, "Add should have 2 operands");
        assert!(
            matches!(&bin.operand[0], elm::Expression::Literal(_)),
            "left operand should be a literal"
        );
        assert!(
            matches!(&bin.operand[1], elm::Expression::Literal(_)),
            "right operand should be a literal"
        );
    } else {
        panic!("Expected elm::Expression::Add, got {:?}", expr);
    }
}

// ---------------------------------------------------------------------------
// Comparison file test-1: boolean logic
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_boolean_and() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: true and false\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    assert!(
        matches!(expr, elm::Expression::And(_)),
        "Expected And, got {expr:?}"
    );
}

// ---------------------------------------------------------------------------
// Comparison file test-2: string literal
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_string_literal() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: 'hello world'\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    if let elm::Expression::Literal(lit) = &expr {
        assert_eq!(lit.value.as_deref(), Some("hello world"));
        assert_eq!(
            lit.value_type.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}String")
        );
    } else {
        panic!("Expected Literal, got {expr:?}");
    }
}

// ---------------------------------------------------------------------------
// SimpleTest.cql: 1 + 1
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_simple_test_add() {
    let (typed_lib, diags) = analyze(
        "library SimpleTest version '1.0.0'\nusing FHIR version '4.0.1'\ncontext Patient\ndefine TestExpression: 1 + 1\n",
    );
    // Diagnostics may include FHIR-model-related items; just check parse+analysis worked.
    let _ = diags;

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    assert!(
        matches!(expr, elm::Expression::Add(_)),
        "Expected Add for '1 + 1', got {expr:?}"
    );
}

// ---------------------------------------------------------------------------
// result_type_name propagation
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_result_type_name_on_integer_literal() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: 42\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    if let elm::Expression::Literal(lit) = &expr {
        assert_eq!(
            lit.element.result_type_name.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}Integer"),
            "result_type_name for integer should be set"
        );
    } else {
        panic!("Expected Literal, got {expr:?}");
    }
}

// ---------------------------------------------------------------------------
// If-Then-Else ELM structure
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_if_then_else_structure() {
    let (typed_lib, diags) =
        analyze("library Test version '1.0'\ndefine Expr: if true then 1 else 2\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    if let elm::Expression::If(if_expr) = &expr {
        assert!(if_expr.condition.is_some(), "condition must be present");
        assert!(if_expr.then_expr.is_some(), "then branch must be present");
        assert!(if_expr.else_expr.is_some(), "else branch must be present");
    } else {
        panic!("Expected If, got {expr:?}");
    }
}

// ---------------------------------------------------------------------------
// Integer division promotesoperands to Decimal
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_integer_divide_promotes_to_decimal() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: 10 / 3\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    if let elm::Expression::Divide(bin) = &expr {
        assert!(
            matches!(&bin.operand[0], elm::Expression::ToDecimal(_)),
            "numerator should be wrapped in ToDecimal"
        );
        assert!(
            matches!(&bin.operand[1], elm::Expression::ToDecimal(_)),
            "denominator should be wrapped in ToDecimal"
        );
    } else {
        panic!("Expected Divide, got {expr:?}");
    }
}

// ---------------------------------------------------------------------------
// datatype_to_qname round-trip tests (used by result_type_name population)
// ---------------------------------------------------------------------------

#[test]
fn test_conformance_datatype_to_qname_matches_java_format() {
    use rh_cql::datatype::DataType;

    // Java reference format: {urn:hl7-org:elm-types:r1}Integer
    assert_eq!(
        datatype_to_qname(&DataType::integer()),
        "{urn:hl7-org:elm-types:r1}Integer"
    );
    assert_eq!(
        datatype_to_qname(&DataType::boolean()),
        "{urn:hl7-org:elm-types:r1}Boolean"
    );
    assert_eq!(
        datatype_to_qname(&DataType::string()),
        "{urn:hl7-org:elm-types:r1}String"
    );
    assert_eq!(
        datatype_to_qname(&DataType::list(DataType::integer())),
        "{urn:hl7-org:elm-types:r1}List<Integer>"
    );
}

#[test]
fn test_conformance_system_function_abs_emits_native() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: Abs(-5)\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    assert!(
        matches!(expr, elm::Expression::Abs(_)),
        "Expected Abs, got {expr:?}"
    );
}

#[test]
fn test_conformance_system_function_log_emits_native() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: Log(100, 10)\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    assert!(
        matches!(expr, elm::Expression::Log(_)),
        "Expected Log, got {expr:?}"
    );
}

#[test]
fn test_conformance_negative_integer_literal_emits_negate_literal() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: -5\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    match expr {
        elm::Expression::Negate(unary) => match unary.operand {
            Some(inner) => match *inner {
                elm::Expression::Literal(lit) => {
                    assert_eq!(lit.value.as_deref(), Some("5"));
                    assert_eq!(
                        lit.value_type.as_deref(),
                        Some("{urn:hl7-org:elm-types:r1}Integer")
                    );
                }
                other => panic!("Expected Literal inside Negate, got {other:?}"),
            },
            None => panic!("Negate operand missing"),
        },
        other => panic!("Expected Negate, got {other:?}"),
    }
}

#[test]
fn test_conformance_substring_emits_native_node() {
    let (typed_lib, diags) =
        analyze("library Test version '1.0'\ndefine Expr: Substring('abc', 1, 1)\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    assert!(
        matches!(expr, elm::Expression::Substring(_)),
        "Expected Substring, got {expr:?}"
    );
}

#[test]
fn test_conformance_substring_2arg_emits_native_node() {
    let (typed_lib, diags) =
        analyze("library Test version '1.0'\ndefine Expr: Substring('ab', 1)\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    match &expr {
        elm::Expression::Substring(s) => {
            assert!(
                s.length.is_none(),
                "2-arg Substring should have length: None, got {:?}",
                s.length
            );
        }
        other => panic!("Expected Substring, got {other:?}"),
    }
}

#[test]
fn test_conformance_negative_decimal_literal_emits_negate_literal() {
    let (typed_lib, diags) = analyze("library Test version '1.0'\ndefine Expr: -3.14\n");
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let mut ctx = emitter_with_result_types();
    let expr = ctx.emit_expression(body);

    match expr {
        elm::Expression::Negate(unary) => match unary.operand {
            Some(inner) => match *inner {
                elm::Expression::Literal(lit) => {
                    assert_eq!(lit.value.as_deref(), Some("3.14"));
                    assert_eq!(
                        lit.value_type.as_deref(),
                        Some("{urn:hl7-org:elm-types:r1}Decimal")
                    );
                }
                other => panic!("Expected Literal inside Negate, got {other:?}"),
            },
            None => panic!("Negate operand missing"),
        },
        other => panic!("Expected Negate, got {other:?}"),
    }
}
