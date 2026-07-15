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
use rh_cql::CompilationContext;
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

/// Like `analyze` but uses the full FHIR R4 model provider so that
/// `primaryCodePath` and other model-info fields are available.
fn analyze_fhir(
    cql: &str,
) -> (
    rh_cql::semantics::typed_ast::TypedLibrary,
    Vec<rh_cql::CqlCompilerException>,
) {
    let parser = CqlParser::new();
    let ast_lib = parser.parse(cql).expect("parse failed");
    let ctx = CompilationContext::new(
        CompilerOptions::new().with_option(CompilerOption::EnableResultTypes),
        None, // resolves to fhir_r4_provider_from_package()
    );
    let analyzer = SemanticAnalyzer::with_context(&ctx);
    analyzer.analyze(ast_lib)
}

fn emitter_with_result_types() -> ElmEmitter {
    ElmEmitter::new(CompilerOptions::new().with_option(CompilerOption::EnableResultTypes))
}

fn emit_source_expression(source: &str) -> elm::Expression {
    let cql = format!("library Test version '1.0'\ndefine Expr: {source}\n");
    let (typed_lib, diags) = analyze(&cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    let body = first_expr_body(&typed_lib);
    emitter_with_result_types().emit_expression(body)
}

#[derive(Clone, Copy)]
enum LiteralTest {
    Null,
    True,
    False,
}

impl LiteralTest {
    fn matches(self, expression: &elm::Expression) -> bool {
        matches!(
            (self, expression),
            (Self::Null, elm::Expression::IsNull(_))
                | (Self::True, elm::Expression::IsTrue(_))
                | (Self::False, elm::Expression::IsFalse(_))
        )
    }
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

#[test]
fn test_literal_is_expressions_emit_canonical_elm() {
    for (source, expected) in [
        ("null is null", LiteralTest::Null),
        ("true is true", LiteralTest::True),
        ("false is false", LiteralTest::False),
    ] {
        let expression = emit_source_expression(source);
        assert!(
            expected.matches(&expression),
            "unexpected ELM for {source}: {expression:?}"
        );
    }
}

#[test]
fn test_negated_literal_is_expressions_wrap_canonical_elm() {
    for (source, expected) in [
        ("null is not null", LiteralTest::Null),
        ("true is not true", LiteralTest::True),
        ("false is not false", LiteralTest::False),
    ] {
        let expression = emit_source_expression(source);
        let operand = match expression {
            elm::Expression::Not(unary) => unary.operand,
            other => panic!("expected Not for {source}, got {other:?}"),
        };
        assert!(
            operand
                .as_deref()
                .is_some_and(|operand| expected.matches(operand)),
            "unexpected ELM for {source}: {operand:?}"
        );
    }
}

#[test]
fn test_named_is_expression_remains_generic_elm() {
    assert!(matches!(
        emit_source_expression("1 is Integer"),
        elm::Expression::Is(_)
    ));
}

#[test]
fn test_zero_offset_temporal_relationships_emit_canonical_elm() {
    let before = emit_source_expression("@2024-01-01 before @2024-02-01");
    assert!(matches!(before, elm::Expression::Before(_)));

    let after = emit_source_expression("@2024-02-01 after @2024-01-01");
    assert!(matches!(after, elm::Expression::After(_)));

    for source in [
        "@2024-01-01 on or before month of @2024-02-01",
        "@2024-01-01 before or on month of @2024-02-01",
    ] {
        let expression = emit_source_expression(source);
        assert!(matches!(
            expression,
            elm::Expression::SameOrBefore(ref timing)
                if timing.precision.as_deref() == Some("month")
        ));
    }

    for source in [
        "@T10:00 on or after hour of @T09:00",
        "@T10:00 after or on hour of @T09:00",
    ] {
        let expression = emit_source_expression(source);
        assert!(matches!(
            expression,
            elm::Expression::SameOrAfter(ref timing)
                if timing.precision.as_deref() == Some("hour")
        ));
    }
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

// ---------------------------------------------------------------------------
// Fix: ExpressionDef.context propagated from library context declaration
// ---------------------------------------------------------------------------

/// A library with `context Patient` must emit `context: Some("Patient")` on
/// every ExpressionDef.  Previously the field was always emitted as `None`.
#[test]
fn test_expression_def_context_propagated_from_library_context() {
    let cql = r#"
library ContextTest version '1.0'
using FHIR version '4.0.1'
context Patient
define "IsAlive": true
define "PatientName": 'test'
"#;
    let (typed_lib, diags) = analyze(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let mut emitter = emitter_with_result_types();
    let elm_lib = emitter.emit(typed_lib);

    let stmts = elm_lib.statements.expect("statements missing").defs;
    assert!(!stmts.is_empty(), "no statements emitted");
    for stmt in &stmts {
        match stmt {
            elm::StatementDef::Expression(ed) => {
                assert_eq!(
                    ed.context.as_deref(),
                    Some("Patient"),
                    "ExpressionDef '{}' should have context Patient, got {:?}",
                    ed.name.as_deref().unwrap_or("?"),
                    ed.context
                );
            }
            elm::StatementDef::Function(fd) => {
                assert_eq!(
                    fd.context.as_deref(),
                    Some("Patient"),
                    "FunctionDef '{}' should have context Patient",
                    fd.name.as_deref().unwrap_or("?")
                );
            }
        }
    }
}

/// A library without any context declaration must not emit a context on
/// ExpressionDef (context should be None).
#[test]
fn test_expression_def_context_none_without_context_declaration() {
    let cql = "library NoCtx version '1.0'\ndefine \"X\": 1\n";
    let (typed_lib, diags) = analyze(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let mut emitter = emitter_with_result_types();
    let elm_lib = emitter.emit(typed_lib);

    let stmts = elm_lib.statements.expect("statements missing").defs;
    for stmt in &stmts {
        if let elm::StatementDef::Expression(ed) = stmt {
            assert_eq!(
                ed.context, None,
                "ExpressionDef without context declaration should emit context: None"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Fix: Retrieve.code_property populated from model primaryCodePath
// ---------------------------------------------------------------------------

/// `[Observation: "some-code"]` must emit `code_property: Some("code")` because
/// the FHIR model info declares Observation.primaryCodePath = "code".
/// Previously the field was always emitted as `None`.
#[test]
fn test_retrieve_code_property_from_model_primary_code_path() {
    let cql = r#"
library RetrieveTest version '1.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'
codesystem "LOINC": 'http://loinc.org'
code "HER2": '85319-2' from "LOINC"
context Patient
define "HER2Obs": [Observation: "HER2"]
"#;
    let (typed_lib, diags) = analyze_fhir(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let retrieve = match &body.inner {
        rh_cql::semantics::typed_ast::TypedExpression::Retrieve(r) => r,
        other => panic!("expected Retrieve, got {other:?}"),
    };
    assert_eq!(
        retrieve.code_property.as_deref(),
        Some("code"),
        "Observation retrieve should have code_property = 'code'"
    );

    let mut emitter = emitter_with_result_types();
    let elm_lib = emitter.emit(typed_lib);
    let stmts = elm_lib.statements.expect("statements").defs;
    let expr_def = match stmts.first().expect("stmt") {
        elm::StatementDef::Expression(e) => e,
        other => panic!("expected ExpressionDef: {other:?}"),
    };
    let retrieve_elm = match expr_def.expression.as_deref().expect("expression") {
        elm::Expression::Retrieve(r) => r,
        other => panic!("expected Retrieve in ELM: {other:?}"),
    };
    assert_eq!(
        retrieve_elm.code_property.as_deref(),
        Some("code"),
        "ELM Retrieve.code_property should be 'code'"
    );
}

#[test]
fn test_retrieve_explicit_code_path_overrides_model_primary_code_path() {
    let cql = r#"
library RetrieveTest version '1.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'
valueset "Asthma": 'urn:oid:1.2.3'
context Patient
define "Asthma Encounters":
  [Encounter: reason in "Asthma"] E
    where true
"#;
    let (typed_lib, diags) = analyze_fhir(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = first_expr_body(&typed_lib);
    let query = match &body.inner {
        rh_cql::semantics::typed_ast::TypedExpression::Query(q) => q,
        other => panic!("expected Query, got {other:?}"),
    };
    let retrieve = match &query.sources[0].expression.inner {
        rh_cql::semantics::typed_ast::TypedExpression::Retrieve(r) => r,
        other => panic!("expected Retrieve source, got {other:?}"),
    };
    assert_eq!(
        retrieve.code_property.as_deref(),
        Some("reason"),
        "explicit retrieve code path should override model primaryCodePath"
    );

    let mut emitter = emitter_with_result_types();
    let elm_lib = emitter.emit(typed_lib);
    let stmts = elm_lib.statements.expect("statements").defs;
    let expr_def = match stmts.first().expect("stmt") {
        elm::StatementDef::Expression(e) => e,
        other => panic!("expected ExpressionDef: {other:?}"),
    };
    let query_elm = match expr_def.expression.as_deref().expect("expression") {
        elm::Expression::Query(q) => q,
        other => panic!("expected Query in ELM: {other:?}"),
    };
    let retrieve_elm = match query_elm
        .source
        .first()
        .expect("source")
        .expression
        .as_deref()
    {
        Some(elm::Expression::Retrieve(r)) => r,
        other => panic!("expected Retrieve source in ELM: {other:?}"),
    };
    assert_eq!(
        retrieve_elm.code_property.as_deref(),
        Some("reason"),
        "ELM Retrieve.code_property should preserve explicit code path"
    );
}

// ---------------------------------------------------------------------------
// Fix: IdentifierRef to a code def emits CodeRef, not ExpressionRef
// ---------------------------------------------------------------------------

/// When `Retrieve.codes` refers to a declared `code` definition, the emitted
/// ELM node must be `CodeRef` so that evaluators look it up in `library.codes`
/// rather than `library.expressions`.
#[test]
fn test_code_ref_emitted_for_code_definition_in_retrieve() {
    let cql = r#"
library CodeRefTest version '1.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'
codesystem "LOINC": 'http://loinc.org'
code "HER2": '85319-2' from "LOINC"
context Patient
define "HER2Obs": [Observation: "HER2"]
"#;
    let (typed_lib, diags) = analyze_fhir(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let mut emitter = emitter_with_result_types();
    let elm_lib = emitter.emit(typed_lib);
    let stmts = elm_lib.statements.expect("statements").defs;
    let expr_def = match stmts.first().expect("stmt") {
        elm::StatementDef::Expression(e) => e,
        other => panic!("expected ExpressionDef: {other:?}"),
    };
    let retrieve = match expr_def.expression.as_deref().expect("expression") {
        elm::Expression::Retrieve(r) => r,
        other => panic!("expected Retrieve: {other:?}"),
    };
    // codes is now wrapped in a List([CodeRef]) so evaluators receive [Code]
    // rather than a bare Code — required by cql-execution's typeIsArray check.
    let codes_expr = retrieve
        .codes
        .as_deref()
        .expect("Retrieve.codes should be set");
    let code_ref = match codes_expr {
        elm::Expression::List(list) => list.elements.first().expect("List should have one element"),
        other => panic!("Expected List wrapper around CodeRef, got {other:?}"),
    };
    assert!(
        matches!(code_ref, elm::Expression::CodeRef(_)),
        "Retrieve.codes element must be CodeRef, got {code_ref:?}"
    );
    if let elm::Expression::CodeRef(cr) = code_ref {
        assert_eq!(cr.name.as_deref(), Some("HER2"));
    }
}

/// An `ExpressionRef` to a regular define must still emit `ExpressionRef`
/// (regression guard: fix must not affect non-code references).
#[test]
fn test_expression_ref_still_emitted_for_expression_define() {
    let cql = r#"
library ExprRefTest version '1.0'
define "MyBool": true
define "UseMyBool": "MyBool"
"#;
    let (typed_lib, diags) = analyze(cql);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    let body = {
        let stmts = &typed_lib.statements;
        // Second statement: UseMyBool
        match &stmts[1].inner {
            rh_cql::semantics::typed_ast::TypedStatement::ExpressionDef { body, .. } => {
                body.clone()
            }
            _ => panic!("expected ExpressionDef"),
        }
    };

    let mut emitter = emitter_with_result_types();
    let elm_expr = emitter.emit_expression(&body);
    assert!(
        matches!(elm_expr, elm::Expression::ExpressionRef(_)),
        "Reference to a regular define should emit ExpressionRef, got {elm_expr:?}"
    );
}
