pub mod clinical;
pub mod conditionals;
pub mod functions;
pub mod literals;
pub mod operators;
pub mod queries;
pub mod statements;
pub mod types;

use crate::datatype::{DataType, SystemType};
use crate::elm;
use crate::options::CompilerOptions;
use crate::semantics::typed_ast::{TypedExpression, TypedLibrary, TypedNode};

pub struct ElmEmitter {
    _local_id_counter: usize,
    _options: CompilerOptions,
    // optional source-map builder integration could go here
}

impl ElmEmitter {
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            _local_id_counter: 1,
            _options: options,
        }
    }

    /// Build `ElementFields` for a typed node, populating:
    /// - `local_id`        when annotations are enabled  (task 4.13)
    /// - `locator`         when locators are enabled      (task 4.11)
    /// - `result_type_name` when result types are enabled (task 4.12)
    pub fn element_fields<T>(&mut self, node: &TypedNode<T>) -> elm::ElementFields {
        let mut fields = elm::ElementFields::default();

        // 4.13 – annotations enabled → assign a monotonic local id
        if self._options.annotations_enabled() {
            fields.local_id = Some(self.generate_local_id());
        }

        // 4.11 – locators enabled → encode the source span
        if self._options.locators_enabled() {
            let s = &node.span;
            fields.locator = Some(format!(
                "{}-{}:{}-{}",
                s.start.line, s.start.column, s.end.line, s.end.column
            ));
        }

        // 4.12 – result types enabled → derive the QName from the data type
        if self._options.result_types_enabled() {
            fields.result_type_name = Some(datatype_to_qname(&node.data_type));
        }

        fields
    }

    fn generate_local_id(&mut self) -> String {
        let id = self._local_id_counter;
        self._local_id_counter += 1;
        id.to_string()
    }

    /// Central expression dispatcher.  Delegates to the appropriate submodule
    /// helper, threading `self` as the mutable context.
    pub fn emit_expression(&mut self, node: &TypedNode<TypedExpression>) -> elm::Expression {
        use crate::semantics::typed_ast::TypedExpression as TE;

        match &node.inner {
            TE::Literal(lit) => literals::emit_literal(lit, node, self),

            TE::IdentifierRef(id_ref) => {
                let element = self.element_fields(node);
                elm::Expression::ExpressionRef(elm::ExpressionRef {
                    element,
                    name: Some(id_ref.name.clone()),
                    library_name: None,
                })
            }

            TE::QualifiedIdentifierRef(qid) => {
                let element = self.element_fields(node);
                elm::Expression::ExpressionRef(elm::ExpressionRef {
                    element,
                    name: Some(qid.name.clone()),
                    library_name: Some(qid.qualifier.clone()),
                })
            }

            TE::UnaryExpression(op, operand) => {
                operators::emit_unary_operator(op, operand, node, self, |n, ctx| {
                    ctx.emit_expression(n)
                })
            }

            TE::BinaryExpression(op, left, right) => {
                operators::emit_binary_operator(op, left, right, node, self, |n, ctx| {
                    ctx.emit_expression(n)
                })
            }

            TE::TernaryExpression(op, a, b, c) => {
                operators::emit_ternary_operator(op, a, b, c, node, self, |n, ctx| {
                    ctx.emit_expression(n)
                })
            }

            TE::IfThenElse(cond, then_expr, else_expr) => {
                self.emit_if(cond, then_expr, else_expr, node)
            }

            TE::Case(typed_case) => self.emit_case(typed_case, node),

            TE::Query(q) => queries::emit_query(q, node, self, |n, ctx| ctx.emit_expression(n)),

            TE::Retrieve(r) => {
                queries::emit_retrieve(r, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::TypeExpression(te) => {
                types::emit_type_expression(te, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::IntervalExpression(ie) => {
                types::emit_interval_expression(ie, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::ListExpression(items) => {
                types::emit_list_expression(items, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::TupleExpression(elements) => {
                types::emit_tuple_expression(elements, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::Instance(inst) => {
                types::emit_instance(inst, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::FunctionInvocation(fi) => {
                let element = self.element_fields(node);
                // First try built-in system functions; fall through to FunctionRef
                if let Some(expr) = operators::emit_system_function(
                    &fi.function,
                    &fi.arguments,
                    node,
                    self,
                    &|n, ctx| ctx.emit_expression(n),
                ) {
                    return expr;
                }
                let operand = fi
                    .arguments
                    .iter()
                    .map(|a| self.emit_expression(a))
                    .collect();
                elm::Expression::FunctionRef(elm::FunctionRef {
                    element,
                    name: Some(fi.function.clone()),
                    library_name: None,
                    operand,
                    signature: Vec::new(),
                })
            }

            TE::MemberInvocation(mi) => {
                let element = self.element_fields(node);
                let source = Box::new(self.emit_expression(&mi.expression));
                elm::Expression::Property(elm::Property {
                    element,
                    path: Some(mi.member.clone()),
                    source: Some(source),
                    scope: None,
                })
            }

            TE::IndexInvocation(ii) => {
                let element = self.element_fields(node);
                let operand = vec![
                    self.emit_expression(&ii.expression),
                    self.emit_expression(&ii.index),
                ];
                elm::Expression::Indexer(elm::BinaryExpression {
                    element,
                    operand,
                    signature: Vec::new(),
                })
            }

            TE::LetClause(name, value) => {
                // LetClauses are handled at the query level; if encountered standalone,
                // emit as an alias reference.
                let element = self.element_fields(node);
                let _ = self.emit_expression(value); // ensure value is visited
                elm::Expression::AliasRef(elm::AliasRef {
                    element,
                    name: Some(name.clone()),
                })
            }

            TE::Parenthesized(inner) => self.emit_expression(inner),

            TE::TimingExpression(te) => {
                // Timing expressions are complex interval operations; delegate through binary
                let element = self.element_fields(node);
                // Emit as an Included-In as a placeholder — proper timing handling
                // requires precision-aware dispatch which is covered in the timing module.
                let left = self.emit_expression(&te.left);
                let right = self.emit_expression(&te.right);
                elm::Expression::IncludedIn(elm::TimeBinaryExpression {
                    element,
                    operand: vec![left, right],
                    signature: Vec::new(),
                    precision: None,
                })
            }

            TE::DateTimeComponentFrom(dtc) => {
                let element = self.element_fields(node);
                let operand = Some(Box::new(self.emit_expression(&dtc.operand)));
                elm::Expression::DateTimeComponentFrom(elm::DateTimeComponentFrom {
                    element,
                    operand,
                    precision: None,
                })
            }
        }
    }

    pub fn emit(&mut self, typed_library: TypedLibrary) -> elm::Library {
        // Create an empty ELM library for now

        let mut usings = Vec::new();
        for u in typed_library.usings {
            usings.push(elm::UsingDef {
                local_identifier: Some(u.model_name.clone()),
                uri: None,
                version: u.version.clone(),
            });
        }
        let usings = if usings.is_empty() {
            None
        } else {
            Some(elm::UsingDefs { defs: usings })
        };

        let mut parameters = Vec::new();
        for p in typed_library.parameters {
            parameters.push(elm::ParameterDef {
                name: Some(p.name),
                access_level: Some(elm::AccessModifier::Public),
                parameter_type_name: None,
                parameter_type_specifier: None,
                default_value: None,
            });
        }
        let parameters = if parameters.is_empty() {
            None
        } else {
            Some(elm::ParameterDefs { defs: parameters })
        };

        let mut code_systems = Vec::new();
        for cs in typed_library.codesystems {
            code_systems.push(elm::CodeSystemDef {
                name: Some(cs.name),
                id: Some(cs.id),
                version: cs.version,
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let code_systems = if code_systems.is_empty() {
            None
        } else {
            Some(elm::CodeSystemDefs { defs: code_systems })
        };

        let mut value_sets = Vec::new();
        for vs in typed_library.valuesets {
            value_sets.push(elm::ValueSetDef {
                name: Some(vs.name),
                id: Some(vs.id),
                version: vs.version,
                access_level: Some(elm::AccessModifier::Public),
                code_system: Vec::new(),
            });
        }
        let value_sets = if value_sets.is_empty() {
            None
        } else {
            Some(elm::ValueSetDefs { defs: value_sets })
        };

        let mut contexts = Vec::new();
        for ctx in typed_library.contexts {
            contexts.push(elm::ContextDef {
                name: Some(ctx.name),
            });
        }
        let contexts = if contexts.is_empty() {
            None
        } else {
            Some(elm::ContextDefs { defs: contexts })
        };

        let mut codes = Vec::new();
        for c in typed_library.codes {
            codes.push(elm::CodeDef {
                name: Some(c.name),
                id: Some(c.code),
                display: c.display,
                code_system: Some(elm::CodeSystemDefRef {
                    name: Some(c.codesystem),
                    library_name: None,
                }),
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let codes = if codes.is_empty() {
            None
        } else {
            Some(elm::CodeDefs { defs: codes })
        };

        let mut concepts = Vec::new();
        for c in typed_library.concepts {
            concepts.push(elm::ConceptDef {
                name: Some(c.name),
                display: c.display,
                code: Vec::new(),
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let concepts = if concepts.is_empty() {
            None
        } else {
            Some(elm::ConceptDefs { defs: concepts })
        };

        let mut statements = Vec::new();
        for _s in typed_library.statements {
            // Need actual statement translation
            statements.push(elm::StatementDef::Expression(elm::ExpressionDef {
                name: Some("TodoStatement".to_string()),
                context: None,
                access_level: Some(elm::AccessModifier::Public),
                expression: Some(Box::new(elm::Expression::default())),
                local_id: None,
                locator: None,
                result_type_name: None,
                result_type_specifier: None,
                annotation: vec![],
            }));
        }
        let statements = if statements.is_empty() {
            None
        } else {
            Some(elm::ExpressionDefs { defs: statements })
        };

        elm::Library {
            local_id: None,
            annotation: vec![],
            identifier: Some(elm::VersionedIdentifier {
                id: Some(
                    typed_library
                        .identifier
                        .as_ref()
                        .map(|i| i.name.clone())
                        .unwrap_or_else(|| "Anonymous".to_string()),
                ),
                system: None,
                version: typed_library
                    .identifier
                    .as_ref()
                    .and_then(|i| i.version.clone()),
            }),
            schema_identifier: Some(elm::VersionedIdentifier {
                id: Some("urn:hl7-org:elm".to_string()),
                system: None,
                version: Some("r1".to_string()),
            }),
            usings,
            includes: None, // TODO mappings
            parameters,
            code_systems,
            value_sets,
            codes,
            concepts,
            contexts,
            statements,
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert a [`DataType`] to an ELM QName string.
///
/// Mirrors the logic in `translator::datatype_to_qname`.
pub fn datatype_to_qname(dt: &DataType) -> elm::QName {
    match dt {
        DataType::System(sys) => qname_system(system_type_name(sys)),
        DataType::Model { namespace, name } => format!("{{{namespace}}}{name}"),
        DataType::List(elem) => {
            let inner = datatype_to_qname(elem);
            let name = extract_type_name(&inner);
            qname_system(&format!("List<{name}>"))
        }
        DataType::Interval(point) => {
            let inner = datatype_to_qname(point);
            let name = extract_type_name(&inner);
            qname_system(&format!("Interval<{name}>"))
        }
        DataType::Tuple(_) => qname_system("Tuple"),
        DataType::Choice(_) => qname_system("Any"),
        DataType::TypeParameter(name) => qname_system(name),
        DataType::Unknown => qname_system("Any"),
    }
}

fn qname_system(name: &str) -> elm::QName {
    format!("{{urn:hl7-org:elm-types:r1}}{name}")
}

fn extract_type_name(qname: &str) -> &str {
    if let Some(pos) = qname.rfind('}') {
        &qname[pos + 1..]
    } else {
        qname
    }
}

fn system_type_name(sys: &SystemType) -> &'static str {
    match sys {
        SystemType::Any => "Any",
        SystemType::Boolean => "Boolean",
        SystemType::Integer => "Integer",
        SystemType::Long => "Long",
        SystemType::Decimal => "Decimal",
        SystemType::String => "String",
        SystemType::Date => "Date",
        SystemType::DateTime => "DateTime",
        SystemType::Time => "Time",
        SystemType::Quantity => "Quantity",
        SystemType::Ratio => "Ratio",
        SystemType::Code => "Code",
        SystemType::Concept => "Concept",
        SystemType::Vocabulary => "Vocabulary",
    }
}

#[cfg(test)]
mod tests {
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
}
