
    use super::*;
    use crate::datatype::{DataType, SystemType};
    use crate::options::{CompilerOption, CompilerOptions};
    use crate::parser::ast::Literal;
    use crate::semantics::typed_ast::{
        NodeId, SemanticMeta, SourceLocation, SourceSpan, TypedCase, TypedCaseItem,
        TypedExpression, TypedNode,
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
            inner: TypedExpression::BinaryExpression(BinaryOperator::Add, left, right),
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
            inner: TypedExpression::BinaryExpression(BinaryOperator::Divide, left, right),
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
