pub mod clinical;
pub mod conditionals;
pub mod literals;
pub mod operators;
pub mod queries;
pub mod references;
pub mod types;

#[cfg(test)]
mod tests;

use crate::datatype::{DataType, SystemType};
use crate::elm;
use crate::options::CompilerOptions;
use crate::semantics::typed_ast::{TypedExpression, TypedLibrary, TypedNode};

pub struct ElmEmitter {
    _local_id_counter: usize,
    _options: CompilerOptions,
    source_map: crate::sourcemap::SourceMap,
}

impl ElmEmitter {
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            _local_id_counter: 1,
            _options: options,
            source_map: crate::sourcemap::SourceMap::new(),
        }
    }

    /// Consume the emitter and return the built source map.
    pub fn take_source_map(self) -> crate::sourcemap::SourceMap {
        self.source_map
    }

    /// Build `ElementFields` for a typed node, populating:
    /// - `local_id`        when annotations are enabled  (task 4.13)
    /// - `locator`         when locators are enabled      (task 4.11)
    /// - `result_type_name` when result types are enabled (task 4.12)
    ///
    /// Also records an [`ElmNodeMeta`] and (when the span is non-trivial) a
    /// [`SourceElmMapping`] in the source-map side-channel (task 7.4).
    pub fn element_fields<T>(&mut self, node: &TypedNode<T>) -> elm::ElementFields {
        let mut fields = elm::ElementFields::default();

        // 4.13 – annotations enabled → assign a monotonic local id
        let elm_id = if self._options.annotations_enabled() {
            let id = self.generate_local_id();
            fields.local_id = Some(id.clone());
            id
        } else {
            // Derive a stable id from the node's stable NodeId
            format!("n{}", node.node_id.0)
        };

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

        // Task 7.4 – record NodeId → ElmNodeMeta in source-map side-channel
        self.source_map
            .elm_node_metas
            .push(crate::sourcemap::ElmNodeMeta {
                elm_node_id: elm_id.clone(),
                elm_path: String::new(),
                elm_kind: String::from("expression"),
                parent_id: None,
            });
        // Only record a span mapping when the node has non-trivial source info
        let s = &node.span;
        if s.start.line > 0 || s.start.offset > 0 {
            let doc_id = String::new(); // populated by compile_to_elm_with_sourcemap
            let elm_node_ids = vec![elm_id.clone()];
            let mapping_id = crate::sourcemap::generate_mapping_id(
                &doc_id,
                s.start.offset,
                s.end.offset,
                &elm_node_ids,
                "direct",
            );
            self.source_map
                .mappings
                .push(crate::sourcemap::SourceElmMapping {
                    mapping_id,
                    doc_id,
                    span: crate::sourcemap::SourceSpan {
                        start: crate::sourcemap::SourceLocation {
                            line: s.start.line,
                            column: s.start.column,
                            offset: s.start.offset,
                        },
                        end: crate::sourcemap::SourceLocation {
                            line: s.end.line,
                            column: s.end.column,
                            offset: s.end.offset,
                        },
                    },
                    role: crate::sourcemap::MappingRole::Direct,
                    elm_node_ids,
                    confidence: Some(crate::sourcemap::Confidence::Exact),
                    note: None,
                });
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

            TE::IdentifierRef(id_ref) => references::emit_identifier_ref(id_ref, node, self),

            TE::QualifiedIdentifierRef(qid) => {
                references::emit_qualified_identifier_ref(qid, node, self)
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
                references::emit_function_invocation(fi, node, self, |n, ctx| {
                    ctx.emit_expression(n)
                })
            }

            TE::MemberInvocation(mi) => {
                references::emit_member_invocation(mi, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::IndexInvocation(ii) => {
                references::emit_index_invocation(ii, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::LetClause(name, value) => {
                references::emit_let_clause_standalone(name, value, node, self, |n, ctx| {
                    ctx.emit_expression(n)
                })
            }

            TE::Parenthesized(inner) => self.emit_expression(inner),

            TE::MinValue(ts) => types::emit_min_value(ts, node, self),

            TE::MaxValue(ts) => types::emit_max_value(ts, node, self),

            TE::TimingExpression(te) => {
                operators::emit_timing_expression(te, node, self, |n, ctx| ctx.emit_expression(n))
            }

            TE::DateTimeComponentFrom(dtc) => {
                operators::emit_datetime_component_from(dtc, node, self, |n, ctx| {
                    ctx.emit_expression(n)
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
        for s in typed_library.statements {
            use crate::semantics::typed_ast::TypedStatement;
            match s.inner {
                TypedStatement::ExpressionDef { name, body } => {
                    let expression = self.emit_expression(&body);
                    statements.push(elm::StatementDef::Expression(elm::ExpressionDef {
                        name: Some(name),
                        context: None,
                        access_level: Some(elm::AccessModifier::Public),
                        expression: Some(Box::new(expression)),
                        local_id: None,
                        locator: None,
                        result_type_name: None,
                        result_type_specifier: None,
                        annotation: vec![],
                    }));
                }
                TypedStatement::FunctionDef { name, parameters: _params, return_type: _ret, body, fluent: _fluent } => {
                    let expression = body.as_ref().map(|b| Box::new(self.emit_expression(b)));
                    statements.push(elm::StatementDef::Function(elm::FunctionDef {
                        name: Some(name),
                        context: None,
                        access_level: Some(elm::AccessModifier::Public),
                        expression,
                        local_id: None,
                        locator: None,
                        result_type_name: None,
                        result_type_specifier: None,
                        operand: vec![],
                        external: None,
                        fluent: None,
                        annotation: vec![],
                    }));
                }
            }
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

/// Convert a [`DataType`] to an ELM QName string.\n
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

/// Convert a CQL `TypeSpecifier` (from the parser AST) to an ELM `QName`.
pub(crate) fn type_specifier_to_qname(ts: &crate::parser::ast::TypeSpecifier) -> elm::QName {
    use crate::parser::ast::TypeSpecifier;
    match ts {
        TypeSpecifier::Named(n) => {
            if let Some(ns) = &n.namespace {
                format!("{{{ns}}}{}", n.name)
            } else {
                qname_system(&n.name)
            }
        }
        TypeSpecifier::List(l) => {
            let inner = type_specifier_to_qname(&l.element_type);
            let name = extract_type_name(&inner);
            qname_system(&format!("List<{name}>"))
        }
        TypeSpecifier::Interval(i) => {
            let inner = type_specifier_to_qname(&i.point_type);
            let name = extract_type_name(&inner);
            qname_system(&format!("Interval<{name}>"))
        }
        TypeSpecifier::Tuple(_) => qname_system("Tuple"),
        TypeSpecifier::Choice(_) => qname_system("Any"),
    }
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
