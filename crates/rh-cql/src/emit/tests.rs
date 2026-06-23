use super::*;
use crate::datatype::{DataType, SystemType, TupleElement};
use crate::options::{CompilerOption, CompilerOptions};
use crate::parser::ast::{
    FunctionParameter, ListTypeSpecifier as AstListTypeSpecifier, Literal,
    NamedTypeSpecifier as AstNamedTypeSpecifier, TypeSpecifier as AstTypeSpecifier,
};
use crate::semantics::typed_ast::{
    NodeId, SemanticMeta, SourceLocation, SourceSpan, TypedCase, TypedCaseItem, TypedExpression,
    TypedNode, TypedStatement,
};

// -----------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------

fn node_int(v: i64) -> TypedNode<TypedExpression> {
    TypedNode {
        node_id: NodeId(0),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::Literal(Literal::Integer(v)),
    }
}

fn node_bool(v: bool) -> TypedNode<TypedExpression> {
    TypedNode {
        node_id: NodeId(0),
        data_type: DataType::boolean(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::Literal(Literal::Boolean(v)),
    }
}

#[allow(dead_code)]
fn node_str(s: &str) -> TypedNode<TypedExpression> {
    TypedNode {
        node_id: NodeId(0),
        data_type: DataType::string(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::Literal(Literal::String(s.to_string())),
    }
}

fn emitter() -> ElmEmitter {
    // Use CompilerOptions::new() (no options) for a predictable baseline
    ElmEmitter::new(CompilerOptions::new())
}

fn emitter_with_options(opts: CompilerOptions) -> ElmEmitter {
    ElmEmitter::new(opts)
}

fn empty_typed_library_with_statement(statement: TypedNode<TypedStatement>) -> TypedLibrary {
    TypedLibrary {
        identifier: None,
        usings: vec![],
        includes: vec![],
        codesystems: vec![],
        valuesets: vec![],
        codes: vec![],
        concepts: vec![],
        parameters: vec![],
        contexts: vec![],
        statements: vec![statement],
    }
}

fn ast_named_type(name: &str) -> AstTypeSpecifier {
    AstTypeSpecifier::Named(AstNamedTypeSpecifier {
        namespace: None,
        name: name.to_string(),
    })
}

fn ast_list_type(element_type: AstTypeSpecifier) -> AstTypeSpecifier {
    AstTypeSpecifier::List(AstListTypeSpecifier {
        element_type: Box::new(element_type),
    })
}

// -----------------------------------------------------------------------
// mod.rs: emit_empty_library
// -----------------------------------------------------------------------

#[test]
fn test_emit_empty_library() {
    let typed_lib = TypedLibrary {
        identifier: None,
        usings: vec![],
        includes: vec![],
        codesystems: vec![],
        valuesets: vec![],
        codes: vec![],
        concepts: vec![],
        parameters: vec![],
        contexts: vec![],
        statements: vec![],
    };

    let mut emitter = emitter();
    let elm_lib = emitter.emit(typed_lib);
    assert_eq!(elm_lib.identifier.unwrap().id.unwrap(), "Anonymous");
}

// -----------------------------------------------------------------------
// mod.rs: element_fields — locator (4.11) and result_type_name (4.12)
// -----------------------------------------------------------------------

#[test]
fn test_element_fields_no_options_by_default() {
    let node = node_int(1);
    let mut emitter = emitter();
    let fields = emitter.element_fields(&node);
    assert!(fields.local_id.is_none());
    assert!(fields.locator.is_none());
    assert!(fields.result_type_name.is_none());
}

#[test]
fn test_element_fields_locator_populated_when_enabled() {
    let mut node = node_int(1);
    node.span = SourceSpan {
        start: SourceLocation {
            line: 3,
            column: 5,
            offset: 0,
        },
        end: SourceLocation {
            line: 3,
            column: 12,
            offset: 0,
        },
    };
    let opts = CompilerOptions::default().with_option(CompilerOption::EnableLocators);
    let mut emitter = emitter_with_options(opts);
    let fields = emitter.element_fields(&node);
    assert!(fields.locator.is_some());
    let loc = fields.locator.unwrap();
    // locator encodes start and end positions
    assert!(loc.contains("3"), "locator should contain line 3: {loc}");
}

#[test]
fn test_element_fields_result_type_name_populated_when_enabled() {
    let node = node_int(42);
    let opts = CompilerOptions::default().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let fields = emitter.element_fields(&node);
    assert_eq!(
        fields.result_type_name.as_deref(),
        Some("{urn:hl7-org:elm-types:r1}Integer")
    );
}

#[test]
fn test_element_fields_structural_result_type_specifier_when_enabled() {
    let mut node = node_int(42);
    node.data_type = DataType::list(DataType::integer());
    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let fields = emitter.element_fields(&node);

    assert!(fields.result_type_name.is_none());
    match fields.result_type_specifier {
        Some(crate::elm::TypeSpecifier::List(list)) => {
            assert!(matches!(
                list.element_type.as_deref(),
                Some(crate::elm::TypeSpecifier::Named(_))
            ));
        }
        other => panic!("expected ListTypeSpecifier, got {other:?}"),
    }
}

#[test]
fn test_element_fields_local_id_populated_when_annotations_enabled() {
    let node = node_int(1);
    let opts = CompilerOptions::default().with_option(CompilerOption::EnableAnnotations);
    let mut emitter = emitter_with_options(opts);
    let _fields1 = emitter.element_fields(&node);
    let fields2 = emitter.element_fields(&node);
    // IDs should be monotonically increasing strings
    assert!(fields2.local_id.is_some());
}

// -----------------------------------------------------------------------
// mod.rs: datatype_to_qname (4.12)
// -----------------------------------------------------------------------

#[test]
fn test_datatype_to_qname_system_integer() {
    let qn = datatype_to_qname(&DataType::integer());
    assert_eq!(qn, "{urn:hl7-org:elm-types:r1}Integer");
}

#[test]
fn test_datatype_to_qname_system_string() {
    let qn = datatype_to_qname(&DataType::string());
    assert_eq!(qn, "{urn:hl7-org:elm-types:r1}String");
}

#[test]
fn test_datatype_to_qname_list() {
    let qn = datatype_to_qname(&DataType::list(DataType::string()));
    assert_eq!(qn, "{urn:hl7-org:elm-types:r1}List<String>");
}

#[test]
fn test_datatype_to_result_type_named_uses_result_type_name() {
    let metadata = datatype_to_result_type(&DataType::model("http://hl7.org/fhir", "Reference"));

    assert_eq!(
        metadata.result_type_name.as_deref(),
        Some("{http://hl7.org/fhir}Reference")
    );
    assert!(metadata.result_type_specifier.is_none());
}

#[test]
fn test_datatype_to_result_type_list_uses_result_type_specifier() {
    let metadata = datatype_to_result_type(&DataType::list(DataType::model(
        "http://hl7.org/fhir",
        "Observation",
    )));

    assert!(metadata.result_type_name.is_none());
    match metadata.result_type_specifier {
        Some(crate::elm::TypeSpecifier::List(list)) => match list.element_type.as_deref() {
            Some(crate::elm::TypeSpecifier::Named(named)) => {
                assert_eq!(named.name, "{http://hl7.org/fhir}Observation");
            }
            other => panic!("expected named element type, got {other:?}"),
        },
        other => panic!("expected ListTypeSpecifier, got {other:?}"),
    }
}

#[test]
fn test_datatype_to_result_type_tuple_uses_result_type_specifier() {
    let metadata = datatype_to_result_type(&DataType::tuple(vec![
        TupleElement {
            name: "reference".to_string(),
            element_type: Box::new(DataType::model("http://hl7.org/fhir", "Reference")),
        },
        TupleElement {
            name: "label".to_string(),
            element_type: Box::new(DataType::string()),
        },
    ]));

    assert!(metadata.result_type_name.is_none());
    match metadata.result_type_specifier {
        Some(crate::elm::TypeSpecifier::Tuple(tuple)) => {
            assert_eq!(tuple.element.len(), 2);
            assert_eq!(tuple.element[0].name, "reference");
            assert_eq!(tuple.element[1].name, "label");
        }
        other => panic!("expected TupleTypeSpecifier, got {other:?}"),
    }
}

#[test]
fn test_result_type_metadata_is_mutually_exclusive() {
    let named = datatype_to_result_type(&DataType::integer());
    assert!(named.result_type_name.is_some());
    assert!(named.result_type_specifier.is_none());

    let structural = datatype_to_result_type(&DataType::list(DataType::integer()));
    assert!(structural.result_type_name.is_none());
    assert!(structural.result_type_specifier.is_some());

    let unknown = datatype_to_result_type(&DataType::Unknown);
    assert!(unknown.result_type_name.is_none());
    assert!(unknown.result_type_specifier.is_none());
}

#[test]
fn test_expression_def_result_type_metadata_when_enabled() {
    let typed_lib = empty_typed_library_with_statement(TypedNode {
        node_id: NodeId(1),
        data_type: DataType::list(DataType::integer()),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedStatement::ExpressionDef {
            name: "Numbers".to_string(),
            body: TypedNode {
                node_id: NodeId(2),
                data_type: DataType::list(DataType::integer()),
                span: SourceSpan::default(),
                meta: SemanticMeta::default(),
                inner: TypedExpression::ListExpression(vec![node_int(1), node_int(2)]),
            },
        },
    });

    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let elm_lib = emitter.emit(typed_lib);
    let statements = elm_lib.statements.expect("statements");
    let crate::elm::StatementDef::Expression(def) = &statements.defs[0] else {
        panic!("expected expression def");
    };

    assert!(def.result_type_name.is_none());
    assert!(matches!(
        def.result_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));
}

#[test]
fn test_expression_def_and_body_result_type_metadata_are_cohesive() {
    let typed_lib = empty_typed_library_with_statement(TypedNode {
        node_id: NodeId(1),
        data_type: DataType::list(DataType::integer()),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedStatement::ExpressionDef {
            name: "Numbers".to_string(),
            body: TypedNode {
                node_id: NodeId(2),
                data_type: DataType::list(DataType::integer()),
                span: SourceSpan::default(),
                meta: SemanticMeta::default(),
                inner: TypedExpression::ListExpression(vec![node_int(1), node_int(2)]),
            },
        },
    });

    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let elm_lib = emitter.emit(typed_lib);
    let statements = elm_lib.statements.expect("statements");
    let crate::elm::StatementDef::Expression(def) = &statements.defs[0] else {
        panic!("expected expression def");
    };

    assert!(def.result_type_name.is_none());
    assert!(matches!(
        def.result_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));

    let expression = def.expression.as_deref().expect("expression body");
    let crate::elm::Expression::List(list) = expression else {
        panic!("expected list expression body, got {expression:?}");
    };
    assert!(list.element.result_type_name.is_none());
    assert!(matches!(
        list.element.result_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));
}

#[test]
fn test_function_def_operand_body_and_result_type_metadata_are_cohesive() {
    let list_type = ast_list_type(ast_named_type("Integer"));
    let typed_lib = empty_typed_library_with_statement(TypedNode {
        node_id: NodeId(1),
        data_type: DataType::list(DataType::integer()),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedStatement::FunctionDef {
            name: "EchoList".to_string(),
            parameters: vec![FunctionParameter {
                name: "items".to_string(),
                type_specifier: Some(list_type.clone()),
            }],
            return_type: Some(list_type),
            body: Some(TypedNode {
                node_id: NodeId(2),
                data_type: DataType::list(DataType::integer()),
                span: SourceSpan::default(),
                meta: SemanticMeta::default(),
                inner: TypedExpression::ListExpression(vec![node_int(1)]),
            }),
            fluent: false,
        },
    });

    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let elm_lib = emitter.emit(typed_lib);
    let statements = elm_lib.statements.expect("statements");
    let crate::elm::StatementDef::Function(def) = &statements.defs[0] else {
        panic!("expected function def");
    };

    assert!(def.result_type_name.is_none());
    assert!(matches!(
        def.result_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));
    assert_eq!(def.operand.len(), 1);
    assert!(def.operand[0].operand_type_name.is_none());
    assert!(matches!(
        def.operand[0].operand_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));

    let expression = def.expression.as_deref().expect("function body");
    let crate::elm::Expression::List(list) = expression else {
        panic!("expected list function body, got {expression:?}");
    };
    assert!(list.element.result_type_name.is_none());
    assert!(matches!(
        list.element.result_type_specifier,
        Some(crate::elm::TypeSpecifier::List(_))
    ));
}

#[test]
fn test_named_function_def_and_body_result_type_metadata_are_cohesive() {
    let typed_lib = empty_typed_library_with_statement(TypedNode {
        node_id: NodeId(1),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedStatement::FunctionDef {
            name: "One".to_string(),
            parameters: vec![],
            return_type: Some(ast_named_type("Integer")),
            body: Some(node_int(1)),
            fluent: false,
        },
    });

    let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
    let mut emitter = emitter_with_options(opts);
    let elm_lib = emitter.emit(typed_lib);
    let statements = elm_lib.statements.expect("statements");
    let crate::elm::StatementDef::Function(def) = &statements.defs[0] else {
        panic!("expected function def");
    };

    assert_eq!(
        def.result_type_name.as_deref(),
        Some("{urn:hl7-org:elm-types:r1}Integer")
    );
    assert!(def.result_type_specifier.is_none());

    let expression = def.expression.as_deref().expect("function body");
    let crate::elm::Expression::Literal(literal) = expression else {
        panic!("expected literal function body, got {expression:?}");
    };
    assert_eq!(
        literal.element.result_type_name.as_deref(),
        Some("{urn:hl7-org:elm-types:r1}Integer")
    );
    assert!(literal.element.result_type_specifier.is_none());
}

#[test]
fn test_datatype_to_qname_interval() {
    let qn = datatype_to_qname(&DataType::interval(DataType::System(SystemType::Date)));
    assert_eq!(qn, "{urn:hl7-org:elm-types:r1}Interval<Date>");
}

#[test]
fn test_datatype_to_qname_model() {
    let qn = datatype_to_qname(&DataType::Model {
        namespace: "http://hl7.org/fhir".to_string(),
        name: "Patient".to_string(),
    });
    assert_eq!(qn, "{http://hl7.org/fhir}Patient");
}

// -----------------------------------------------------------------------
// literals: emit_literal (4.15)
// -----------------------------------------------------------------------

#[test]
fn test_emit_literal_integer() {
    let node = node_int(42);
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::Literal(lit) = expr {
        assert_eq!(lit.value.as_deref(), Some("42"));
        assert_eq!(
            lit.value_type.as_deref(),
            Some("{urn:hl7-org:elm-types:r1}Integer")
        );
    } else {
        panic!("Expected Literal, got {:?}", expr);
    }
}

#[test]
fn test_emit_literal_boolean_true() {
    let node = node_bool(true);
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::Literal(lit) = expr {
        assert_eq!(lit.value.as_deref(), Some("true"));
    } else {
        panic!("Expected Literal");
    }
}

#[test]
fn test_emit_literal_null() {
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::Unknown,
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::Literal(Literal::Null),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    assert!(matches!(expr, elm::Expression::Null(_)));
}

// -----------------------------------------------------------------------
// operators: emit_unary_operator (4.15)
// -----------------------------------------------------------------------

#[test]
fn test_emit_unary_not() {
    use crate::parser::ast::UnaryOperator;
    let operand = Box::new(node_bool(true));
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::boolean(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::UnaryExpression(UnaryOperator::Not, operand),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    assert!(matches!(expr, elm::Expression::Not(_)));
}

#[test]
fn test_emit_binary_add() {
    use crate::parser::ast::BinaryOperator;
    let left = Box::new(node_int(1));
    let right = Box::new(node_int(2));
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::BinaryExpression(BinaryOperator::Add, left, right, None),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::Add(bin) = &expr {
        assert_eq!(bin.operand.len(), 2);
    } else {
        panic!("Expected Add, got {:?}", expr);
    }
}

#[test]
fn test_emit_binary_divide_promotes_integer_to_decimal() {
    use crate::parser::ast::BinaryOperator;
    let left = Box::new(node_int(4));
    let right = Box::new(node_int(2));
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::System(SystemType::Decimal),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::BinaryExpression(BinaryOperator::Divide, left, right, None),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::Divide(bin) = &expr {
        // Both operands should be wrapped in ToDecimal
        assert!(matches!(&bin.operand[0], elm::Expression::ToDecimal(_)));
        assert!(matches!(&bin.operand[1], elm::Expression::ToDecimal(_)));
    } else {
        panic!("Expected Divide, got {:?}", expr);
    }
}

// -----------------------------------------------------------------------
// conditionals: emit_if / emit_case (4.15)
// -----------------------------------------------------------------------

#[test]
fn test_emit_if_then_else() {
    let cond = Box::new(node_bool(true));
    let then_expr = Box::new(node_int(1));
    let else_expr = Box::new(node_int(2));
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::IfThenElse(cond, then_expr, else_expr),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::If(if_expr) = expr {
        assert!(if_expr.condition.is_some());
        assert!(if_expr.then_expr.is_some());
        assert!(if_expr.else_expr.is_some());
    } else {
        panic!("Expected If, got {:?}", expr);
    }
}

#[test]
fn test_emit_case_with_items() {
    let when = Box::new(node_bool(true));
    let then = Box::new(node_int(1));
    let else_expr = Box::new(node_int(0));
    let case_items = vec![TypedCaseItem { when, then }];
    let typed_case = TypedCase {
        comparand: None,
        case_items,
        else_expr,
    };
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::Case(typed_case),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::Case(case_expr) = expr {
        assert_eq!(case_expr.case_item.len(), 1);
        assert!(case_expr.comparand.is_none());
    } else {
        panic!("Expected Case, got {:?}", expr);
    }
}

// -----------------------------------------------------------------------
// types: emit_type_expression (4.15)
// -----------------------------------------------------------------------

#[test]
fn test_emit_is_type_expression() {
    use crate::parser::ast::{NamedTypeSpecifier, TypeOperator, TypeSpecifier};
    use crate::semantics::typed_ast::TypedTypeExpression;
    let operand = Box::new(node_int(1));
    let type_expr = TypedTypeExpression {
        operator: TypeOperator::Is,
        operand,
        type_specifier: TypeSpecifier::Named(NamedTypeSpecifier {
            namespace: None,
            name: "Integer".to_string(),
        }),
    };
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::boolean(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::TypeExpression(type_expr),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    assert!(matches!(expr, elm::Expression::Is(_)));
}

// -----------------------------------------------------------------------
// identifier refs (4.15)
// -----------------------------------------------------------------------

#[test]
fn test_emit_identifier_ref() {
    use crate::parser::ast::IdentifierRef;
    let node = TypedNode {
        node_id: NodeId(0),
        data_type: DataType::integer(),
        span: SourceSpan::default(),
        meta: SemanticMeta::default(),
        inner: TypedExpression::IdentifierRef(IdentifierRef {
            name: "MyDef".to_string(),
            location: None,
        }),
    };
    let mut ctx = emitter();
    let expr = ctx.emit_expression(&node);
    if let elm::Expression::ExpressionRef(eref) = expr {
        assert_eq!(eref.name.as_deref(), Some("MyDef"));
        assert!(eref.library_name.is_none());
    } else {
        panic!("Expected ExpressionRef");
    }
}

// -----------------------------------------------------------------------
// includes field (task 2.2, 2.3)
// -----------------------------------------------------------------------

#[test]
fn test_emit_includes_one_entry() {
    use crate::compile;
    let src = r#"
        library Test version '1.0'
        include SomeLib version '2.0' called SL
        define X: 1
    "#;
    let result = compile(src, None).expect("pipeline error");
    let includes = result.library.includes.expect("includes should be Some");
    assert_eq!(includes.defs.len(), 1);
    let inc = &includes.defs[0];
    assert_eq!(inc.path.as_deref(), Some("SomeLib"));
    assert_eq!(inc.local_identifier.as_deref(), Some("SL"));
    assert_eq!(inc.version.as_deref(), Some("2.0"));
}

#[test]
fn test_emit_includes_empty_when_no_includes() {
    use crate::compile;
    let src = "library Test version '1.0' define X: 1";
    let result = compile(src, None).expect("pipeline error");
    // includes should be None (no includes defined)
    assert!(result.library.includes.is_none());
}
