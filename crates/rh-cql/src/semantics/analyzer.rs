use crate::datatype::DataType;
use crate::operators::OperatorResolver;
use crate::options::CompilerOptions;
use crate::parser::ast;
use crate::provider::ModelInfoProvider;
use crate::reporting::SourceLocator;
use crate::semantics::scope::{ScopeManager, Symbol, SymbolKind};
use crate::semantics::typed_ast::{
    NodeId, SemanticMeta, SourceSpan, TypedExpression, TypedLibrary, TypedNode, TypedParameterDef,
    TypedStatement,
};
use crate::CqlCompilerException;
use std::sync::Arc;

pub struct SemanticAnalyzer {
    scope_manager: ScopeManager,
    _model_provider: Arc<dyn ModelInfoProvider>,
    _options: CompilerOptions,
    diagnostics: Vec<CqlCompilerException>,
    next_node_id: u64,
    operator_resolver: OperatorResolver,
    in_sort_context: bool,
}

impl SemanticAnalyzer {
    /// Construct a `SemanticAnalyzer` from an explicit model provider and
    /// options.
    pub fn new(model_provider: Arc<dyn ModelInfoProvider>, options: CompilerOptions) -> Self {
        SemanticAnalyzer {
            scope_manager: ScopeManager::new(),
            _model_provider: model_provider,
            _options: options,
            diagnostics: Vec::new(),
            next_node_id: 1,
            operator_resolver: OperatorResolver::new(),
            in_sort_context: false,
        }
    }

    /// Construct a `SemanticAnalyzer` from a [`CompilationContext`].
    ///
    /// This is the preferred constructor when a context is already available,
    /// as it avoids unpacking the individual fields.
    ///
    /// ```rust
    /// use rh_cql::{CompilationContext, CompilerOptions};
    /// use rh_cql::semantics::analyzer::SemanticAnalyzer;
    ///
    /// let ctx = CompilationContext::new(CompilerOptions::default(), None);
    /// let analyzer = SemanticAnalyzer::with_context(&ctx);
    /// ```
    pub fn with_context(context: &crate::compiler::CompilationContext) -> Self {
        SemanticAnalyzer::new(context.resolve_provider(), context.options.clone())
    }

    /// Register all public expressions and functions from a compiled ELM
    /// library into the scope manager under the given `alias`.
    ///
    /// This allows the semantic analyzer to resolve qualified references such
    /// as `CaseLogic."My Expression"` when `CaseLogic` is an include alias
    /// for a compiled dependency.
    pub fn register_included_library(&mut self, alias: &str, lib: &crate::elm::Library) {
        let alias_id = crate::library::LibraryIdentifier::unversioned(alias);

        if let Some(stmts) = &lib.statements {
            for def in &stmts.defs {
                match def {
                    crate::elm::StatementDef::Expression(e) => {
                        if let Some(name) = &e.name {
                            let mut sym =
                                Symbol::new(name.clone(), SymbolKind::Expression)
                                    .with_type(DataType::Unknown);
                            sym.library = Some(alias_id.clone());
                            self.scope_manager.register_symbol(sym);
                        }
                    }
                    crate::elm::StatementDef::Function(f) => {
                        if let Some(name) = &f.name {
                            let sig = crate::semantics::scope::FunctionSignature {
                                name: name.clone(),
                                operand_types: vec![],
                                result_type: DataType::Unknown,
                                is_fluent: false,
                                is_external: false,
                                library: Some(alias_id.clone()),
                            };
                            self.scope_manager.register_function(sig);
                        }
                    }
                }
            }
        }
    }


    fn analyze_literal(&mut self, e: &ast::Literal) -> TypedNode<TypedExpression> {
        let dt = match e {
            ast::Literal::Null => DataType::system(crate::datatype::SystemType::Any),
            ast::Literal::Boolean(_) => DataType::system(crate::datatype::SystemType::Boolean),
            ast::Literal::Integer(_) => DataType::system(crate::datatype::SystemType::Integer),
            ast::Literal::Long(_) => DataType::system(crate::datatype::SystemType::Long),
            ast::Literal::Decimal(_) => DataType::system(crate::datatype::SystemType::Decimal),
            ast::Literal::String(_) => DataType::system(crate::datatype::SystemType::String),
            ast::Literal::Date(_) => DataType::system(crate::datatype::SystemType::Date),
            ast::Literal::DateTime(_) => DataType::system(crate::datatype::SystemType::DateTime),
            ast::Literal::Time(_) => DataType::system(crate::datatype::SystemType::Time),
            ast::Literal::Quantity { .. } => {
                DataType::system(crate::datatype::SystemType::Quantity)
            }
            ast::Literal::Ratio { .. } => DataType::system(crate::datatype::SystemType::Ratio),
            ast::Literal::Code { .. } => DataType::system(crate::datatype::SystemType::Code),
        };

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::Literal(e.clone()),
        }
    }

    fn analyze_identifier_ref(&mut self, e: &ast::IdentifierRef) -> TypedNode<TypedExpression> {
        let sym = self.scope_manager.resolve_symbol(None, &e.name);

        let (dt, resolved_symbol) = match sym {
            Some(s) => (
                s.result_type.clone().unwrap_or(DataType::Unknown),
                Some(s.name.clone()),
            ),
            None => {
                if !self.in_sort_context {
                    let exc = CqlCompilerException::new(format!(
                        "Could not resolve identifier: {}",
                        e.name
                    ));
                    let exc = match e.location {
                        Some(loc) => exc.with_locator(SourceLocator::from(loc)),
                        None => exc,
                    };
                    self.diagnostics.push(exc);
                }
                (DataType::Unknown, None)
            }
        };

        let meta = SemanticMeta {
            resolved_symbol,
            ..Default::default()
        };

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(), // TODO map from original if possible
            meta,
            inner: TypedExpression::IdentifierRef(e.clone()),
        }
    }

    fn analyze_qualified_identifier_ref(
        &mut self,
        e: &ast::QualifiedIdentifierRef,
    ) -> TypedNode<TypedExpression> {
        let sym = self
            .scope_manager
            .resolve_symbol(Some(&e.qualifier), &e.name);

        let (dt, resolved_symbol) = match sym {
            Some(s) => {
                let lib_name = s.library.as_ref().map(|l| l.name.as_str()).unwrap_or("");
                (
                    s.result_type.clone().unwrap_or(DataType::Unknown),
                    Some(format!("{}.{}", lib_name, s.name)),
                )
            }
            None => {
                let exc = CqlCompilerException::new(format!(
                    "Could not resolve qualified identifier: {}.{}",
                    e.qualifier, e.name
                ));
                let exc = match e.location {
                    Some(loc) => exc.with_locator(SourceLocator::from(loc)),
                    None => exc,
                };
                self.diagnostics.push(exc);
                (DataType::Unknown, None)
            }
        };

        let meta = SemanticMeta {
            resolved_symbol,
            ..Default::default()
        };

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(), // TODO map from original
            meta,
            inner: TypedExpression::QualifiedIdentifierRef(e.clone()),
        }
    }

    fn analyze_function_invocation(
        &mut self,
        e: &ast::FunctionInvocation,
    ) -> TypedNode<TypedExpression> {
        let mut arguments = Vec::new();
        let mut arg_types = Vec::new();

        for arg in &e.arguments {
            let typed_arg = self.analyze_expression(arg);
            arg_types.push(typed_arg.data_type.clone());
            arguments.push(typed_arg);
        }

        let mut dt = DataType::Unknown;
        let mut meta = SemanticMeta::default();

        if let Ok(res) = self
            .operator_resolver
            .resolve_with_operands(&e.name, &arg_types)
        {
            dt = res.result_type;
            meta.resolved_overload = Some(res.signature.name);
            if self._options.implicit_conversions_enabled() {
                for conv in res.conversions.iter().flatten() {
                    meta.implicit_conversions.push(format!("{:?}", conv));
                }
            }
        } else if let Some(funcs) = self.scope_manager.resolve_functions_unqualified(&e.name) {
            if let Some(f) = funcs
                .iter()
                .find(|f| f.operand_types.len() == arg_types.len())
            {
                dt = f.result_type.clone();
                meta.resolved_symbol = Some(e.name.clone());
            } else {
                // self.diagnostics.push(CqlCompilerException::new(format!("Could not find overload for function {}", e.name)));
            }
        } else {
            // self.diagnostics.push(CqlCompilerException::new(format!("Could not resolve function {}", e.name)));
        }

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta,
            inner: TypedExpression::FunctionInvocation(
                crate::semantics::typed_ast::TypedFunctionInvocation {
                    function: e.name.clone(),
                    arguments,
                },
            ),
        }
    }

    fn analyze_member_invocation(
        &mut self,
        e: &ast::MemberInvocation,
    ) -> TypedNode<TypedExpression> {
        let source = self.analyze_expression(&e.source);

        // If the source is an unqualified IdentifierRef that resolves to a
        // library include alias, reclassify as QualifiedIdentifierRef so the
        // emitter produces a correct ExpressionRef with library_name set.
        if let TypedExpression::IdentifierRef(id_ref) = &source.inner {
            if let Some(sym) = self.scope_manager.resolve_symbol(None, &id_ref.name) {
                if sym.kind == SymbolKind::Include {
                    let qid = ast::QualifiedIdentifierRef {
                        qualifier: id_ref.name.clone(),
                        name: e.name.clone(),
                        location: e.location,
                    };
                    return self.analyze_qualified_identifier_ref(&qid);
                }
            }
        }

        let mut dt = DataType::Unknown;

        match &source.data_type {
            DataType::Model {
                namespace,
                name: mt_name,
            } => {
                if let Some(model_info) = self._model_provider.get_model(namespace, None) {
                    for ti in &model_info.type_info {
                        if let crate::modelinfo::TypeInfo::ClassInfo(ci) = ti {
                            if ci.name.as_deref() == Some(mt_name) {
                                for el in &ci.element {
                                    if el.name.as_deref() == Some(&e.name) {
                                        // TODO: support full modelinfo::TypeSpecifier
                                        if let Some(type_str) = &el.element_type {
                                            if let Ok(res_dt) =
                                                self.resolve_qualified_name(type_str)
                                            {
                                                dt = res_dt;
                                            }
                                        } else if let Some(type_str) = &el.type_name {
                                            if let Ok(res_dt) =
                                                self.resolve_qualified_name(type_str)
                                            {
                                                dt = res_dt;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            DataType::Tuple(tup) => {
                for element in tup {
                    if element.name == e.name {
                        dt = *element.element_type.clone();
                        break;
                    }
                }
            }
            _ => {
                // If it's something else, can we resolve it? Maybe a system type property like String.length?
            }
        }

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::MemberInvocation(
                crate::semantics::typed_ast::TypedMemberInvocation {
                    expression: Box::new(source),
                    member: e.name.clone(),
                },
            ),
        }
    }

    fn analyze_index_invocation(&mut self, e: &ast::IndexInvocation) -> TypedNode<TypedExpression> {
        let source = self.analyze_expression(&e.source);
        let index = self.analyze_expression(&e.index);

        let dt = match &source.data_type {
            DataType::List(inner) => *inner.clone(),
            DataType::System(crate::datatype::SystemType::String) => {
                DataType::System(crate::datatype::SystemType::String)
            }
            _ => DataType::Unknown,
        };

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::IndexInvocation(
                crate::semantics::typed_ast::TypedIndexInvocation {
                    expression: Box::new(source),
                    index: Box::new(index),
                },
            ),
        }
    }

    fn analyze_unary_expression(&mut self, e: &ast::UnaryExpression) -> TypedNode<TypedExpression> {
        let operand = self.analyze_expression(&e.operand);
        let resolved = self
            .operator_resolver
            .resolve_unary(e.operator, &operand.data_type);

        let mut meta = SemanticMeta::default();
        let mut data_type = DataType::Unknown;

        match resolved {
            Ok(res) => {
                data_type = res.result_type;
                meta.resolved_overload = Some(res.signature.name);
                if self._options.implicit_conversions_enabled() {
                    for conv in res.conversions.iter().flatten() {
                        meta.implicit_conversions.push(format!("{:?}", conv));
                    }
                }
            }
            Err(_err) => {
                // self.diagnostics.push(CqlCompilerException::new(err.to_string()));
            }
        }

        TypedNode {
            node_id: self.generate_node_id(),
            data_type,
            span: SourceSpan::default(),
            meta,
            inner: TypedExpression::UnaryExpression(e.operator, Box::new(operand)),
        }
    }

    fn analyze_binary_expression(
        &mut self,
        e: &ast::BinaryExpression,
    ) -> TypedNode<TypedExpression> {
        let left = self.analyze_expression(&e.left);
        let right = self.analyze_expression(&e.right);
        let resolved =
            self.operator_resolver
                .resolve_binary(e.operator, &left.data_type, &right.data_type);

        let mut meta = SemanticMeta::default();
        let mut data_type = DataType::Unknown;

        match resolved {
            Ok(res) => {
                data_type = res.result_type;
                meta.resolved_overload = Some(res.signature.name);
                if self._options.implicit_conversions_enabled() {
                    for conv in res.conversions.iter().flatten() {
                        meta.implicit_conversions.push(format!("{:?}", conv));
                    }
                }
            }
            Err(_err) => {
                // self.diagnostics.push(CqlCompilerException::new(err.to_string()));
            }
        }

        TypedNode {
            node_id: self.generate_node_id(),
            data_type,
            span: SourceSpan::default(),
            meta,
            inner: TypedExpression::BinaryExpression(e.operator, Box::new(left), Box::new(right)),
        }
    }

    fn analyze_ternary_expression(
        &mut self,
        e: &ast::TernaryExpression,
    ) -> TypedNode<TypedExpression> {
        let first = self.analyze_expression(&e.first);
        let second = self.analyze_expression(&e.second);
        let third = self.analyze_expression(&e.third);
        let resolved = self.operator_resolver.resolve_ternary(
            e.operator,
            &first.data_type,
            &second.data_type,
            &third.data_type,
        );

        let mut meta = SemanticMeta::default();
        let mut data_type = DataType::Unknown;

        match resolved {
            Ok(res) => {
                data_type = res.result_type;
                meta.resolved_overload = Some(res.signature.name);
                if self._options.implicit_conversions_enabled() {
                    for conv in res.conversions.iter().flatten() {
                        meta.implicit_conversions.push(format!("{:?}", conv));
                    }
                }
            }
            Err(_err) => {
                // self.diagnostics.push(CqlCompilerException::new(err.to_string()));
            }
        }

        TypedNode {
            node_id: self.generate_node_id(),
            data_type,
            span: SourceSpan::default(),
            meta,
            inner: TypedExpression::TernaryExpression(
                e.operator,
                Box::new(first),
                Box::new(second),
                Box::new(third),
            ),
        }
    }

    fn generate_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        NodeId(id)
    }

    fn resolve_type_specifier(
        &self,
        spec: &ast::TypeSpecifier,
    ) -> Result<DataType, crate::types::TypeError> {
        let resolver =
            crate::types::TypeResolver::with_model_provider(self._model_provider.as_ref());
        resolver.resolve_type_specifier(spec)
    }

    fn resolve_qualified_name(&self, name: &str) -> Result<DataType, crate::types::TypeError> {
        let resolver =
            crate::types::TypeResolver::with_model_provider(self._model_provider.as_ref());
        resolver.resolve_qualified_name(name)
    }

    pub fn analyze(mut self, library: ast::Library) -> (TypedLibrary, Vec<CqlCompilerException>) {
        for u in &library.usings {
            self.scope_manager.register_symbol(
                Symbol::new(u.model_name.clone(), SymbolKind::Model).with_type(DataType::any()),
            );
        }

        for i in &library.includes {
            let name = i.alias.clone().unwrap_or_else(|| i.path.clone());
            self.scope_manager
                .register_symbol(Symbol::new(name, SymbolKind::Include).with_type(DataType::any()));
        }

        for cs in &library.codesystems {
            self.scope_manager.register_symbol(
                Symbol::new(cs.name.clone(), SymbolKind::CodeSystem).with_type(DataType::any()),
            );
        }

        for vs in &library.valuesets {
            self.scope_manager.register_symbol(
                Symbol::new(vs.name.clone(), SymbolKind::ValueSet).with_type(DataType::any()),
            );
        }

        for code in &library.codes {
            self.scope_manager.register_symbol(
                Symbol::new(code.name.clone(), SymbolKind::Code).with_type(DataType::any()),
            );
        }

        for concept in &library.concepts {
            self.scope_manager.register_symbol(
                Symbol::new(concept.name.clone(), SymbolKind::Concept).with_type(DataType::any()),
            );
        }

        let mut parameters = Vec::new();
        for param in library.parameters {
            let default_typed = param.default.map(|expr| self.analyze_expression(&expr));
            parameters.push(TypedParameterDef {
                name: param.name.clone(),
                type_specifier: param.type_specifier.clone(),
                default: default_typed,
                access: param.access,
            });

            self.scope_manager.register_symbol(
                Symbol::new(param.name.clone(), SymbolKind::Parameter).with_type(DataType::any()),
            );
        }

        let mut statements = Vec::new();

        for ctx in &library.contexts {
            self.scope_manager.register_symbol(
                Symbol::new(ctx.name.clone(), SymbolKind::Context).with_type(DataType::any()),
            );
        }

        // Pass 1: pre-register all define/function names so forward references resolve.
        for stmt in &library.statements {
            match stmt {
                ast::Statement::ExpressionDef(ed) => {
                    self.scope_manager.register_symbol(
                        Symbol::new(ed.name.clone(), SymbolKind::Expression)
                            .with_type(DataType::any()),
                    );
                }
                ast::Statement::FunctionDef(fd) => {
                    self.scope_manager.register_symbol(
                        Symbol::new(fd.name.clone(), SymbolKind::Function)
                            .with_type(DataType::any()),
                    );
                }
            }
        }

        // Pass 2: analyze statement bodies (all names now in scope).
        for stmt in library.statements {
            let typed_stmt = match stmt {
                ast::Statement::ExpressionDef(ed) => {
                    let id = self.generate_node_id();
                    let body = self.analyze_expression(&ed.expression);
                    let typed_stmt = TypedNode {
                        node_id: id,
                        data_type: DataType::any(),
                        span: SourceSpan::default(),
                        meta: SemanticMeta::default(),
                        inner: TypedStatement::ExpressionDef {
                            name: ed.name.clone(),
                            body,
                        },
                    };

                    self.scope_manager.register_symbol(
                        Symbol::new(ed.name.clone(), SymbolKind::Expression)
                            .with_type(DataType::any()),
                    );

                    typed_stmt
                }
                ast::Statement::FunctionDef(fd) => {
                    let id = self.generate_node_id();
                    let body = fd.body.as_ref().map(|b| self.analyze_expression(b));
                    let typed_stmt = TypedNode {
                        node_id: id,
                        data_type: DataType::any(),
                        span: SourceSpan::default(),
                        meta: SemanticMeta::default(),
                        inner: TypedStatement::FunctionDef {
                            name: fd.name.clone(),
                            parameters: fd.parameters.clone(),
                            return_type: fd.return_type.clone(),
                            body,
                            fluent: fd.fluent,
                        },
                    };

                    self.scope_manager.register_symbol(
                        Symbol::new(fd.name.clone(), SymbolKind::Function)
                            .with_type(DataType::any()),
                    );

                    typed_stmt
                }
            };
            statements.push(typed_stmt);
        }

        (
            TypedLibrary {
                identifier: library.identifier,
                usings: library.usings,
                includes: library.includes,
                codesystems: library.codesystems,
                valuesets: library.valuesets,
                codes: library.codes,
                concepts: library.concepts,
                parameters,
                contexts: library.contexts,
                statements,
            },
            self.diagnostics,
        )
    }

    pub fn analyze_expression(&mut self, expr: &ast::Expression) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let dt = DataType::any(); // TODO exact parse
        let span = SourceSpan::default();
        let meta = SemanticMeta::default();

        let inner = match expr {
            ast::Expression::Literal(e) => return self.analyze_literal(e),
            ast::Expression::IdentifierRef(e) => return self.analyze_identifier_ref(e),
            ast::Expression::QualifiedIdentifierRef(e) => {
                return self.analyze_qualified_identifier_ref(e)
            }
            ast::Expression::UnaryExpression(e) => return self.analyze_unary_expression(e),
            ast::Expression::BinaryExpression(e) => return self.analyze_binary_expression(e),
            ast::Expression::TernaryExpression(e) => return self.analyze_ternary_expression(e),
            ast::Expression::DateTimeComponentFrom(e) => {
                return self.analyze_datetime_component_from(e)
            }
            ast::Expression::TypeExpression(e) => return self.analyze_type_expression(e),
            ast::Expression::TimingExpression(e) => return self.analyze_timing_expression(e),
            ast::Expression::FunctionInvocation(e) => return self.analyze_function_invocation(e),
            ast::Expression::MemberInvocation(e) => return self.analyze_member_invocation(e),
            ast::Expression::IndexInvocation(e) => return self.analyze_index_invocation(e),
            ast::Expression::Query(e) => return self.analyze_query(e),
            ast::Expression::Retrieve(e) => return self.analyze_retrieve(e),
            ast::Expression::IfThenElse(e) => return self.analyze_if_then_else(e),
            ast::Expression::Case(e) => return self.analyze_case(e),
            ast::Expression::IntervalExpression(e) => return self.analyze_interval_expression(e),
            ast::Expression::ListExpression(e) => TypedExpression::ListExpression(
                e.elements
                    .iter()
                    .map(|elem| self.analyze_expression(elem))
                    .collect(),
            ),
            ast::Expression::TupleExpression(e) => return self.analyze_tuple_expression(e),
            ast::Expression::Instance(e) => return self.analyze_instance_expression(e),
            ast::Expression::Let(e) => TypedExpression::LetClause(
                e.identifier.clone(),
                Box::new(self.analyze_expression(&e.expression)),
            ),
            ast::Expression::Parenthesized(e) => {
                TypedExpression::Parenthesized(Box::new(self.analyze_expression(e)))
            }
            ast::Expression::MinValue(ts) => TypedExpression::MinValue(ts.clone()),
            ast::Expression::MaxValue(ts) => TypedExpression::MaxValue(ts.clone()),
        };

        TypedNode {
            node_id: id,
            data_type: dt,
            span,
            meta,
            inner,
        }
    }

    pub fn analyze_query(&mut self, e: &ast::Query) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let dt = DataType::any(); // TODO: determine query type

        // Analyze source expressions first (outside the query scope so aliases
        // are not yet in scope for the source expressions themselves).
        let mut sources = Vec::new();
        for s in &e.sources {
            let typed_source_expr = self.analyze_expression(&s.expression);
            // Derive the element type for the alias: if the source has a List
            // type, the alias refers to each element; otherwise use Any.
            let alias_type = match &typed_source_expr.data_type {
                DataType::List(elem) => *elem.clone(),
                _ => DataType::any(),
            };
            sources.push((s.alias.clone(), typed_source_expr, alias_type));
        }

        // Push a new query scope and register all source aliases so that the
        // where, return, let, aggregate, and sort clauses can resolve them.
        self.scope_manager.push_scope();
        for (alias, _, alias_type) in &sources {
            self.scope_manager
                .register_query_alias(alias.clone(), alias_type.clone());
        }

        // Convert collected sources into TypedQuerySource values.
        let typed_sources: Vec<_> = sources
            .into_iter()
            .map(
                |(alias, expression, _)| crate::semantics::typed_ast::TypedQuerySource {
                    alias,
                    expression: Box::new(expression),
                },
            )
            .collect();

        let let_clauses: Vec<_> = e
            .let_clauses
            .iter()
            .map(|lc| {
                let typed_expr = self.analyze_expression(&lc.expression);
                // Register let identifier in scope so subsequent clauses can reference it.
                let let_type = typed_expr.data_type.clone();
                let result = crate::semantics::typed_ast::TypedLetClause {
                    identifier: lc.identifier.clone(),
                    expression: Box::new(typed_expr),
                };
                self.scope_manager
                    .register_query_alias(lc.identifier.clone(), let_type);
                result
            })
            .collect();

        let relationships = e
            .relationships
            .iter()
            .map(|r| {
                let typed_rel_expr = self.analyze_expression(&r.source.expression);
                let rel_alias_type = match &typed_rel_expr.data_type {
                    DataType::List(elem) => *elem.clone(),
                    _ => DataType::any(),
                };
                self.scope_manager
                    .register_query_alias(r.source.alias.clone(), rel_alias_type);
                let source = crate::semantics::typed_ast::TypedQuerySource {
                    alias: r.source.alias.clone(),
                    expression: Box::new(typed_rel_expr),
                };
                crate::semantics::typed_ast::TypedRelationshipClause {
                    kind: r.kind,
                    source,
                    such_that: r
                        .such_that
                        .as_ref()
                        .map(|st| Box::new(self.analyze_expression(st))),
                }
            })
            .collect();

        let where_clause = e
            .where_clause
            .as_ref()
            .map(|w| Box::new(self.analyze_expression(w)));

        let return_clause =
            e.return_clause
                .as_ref()
                .map(|rc| crate::semantics::typed_ast::TypedReturnClause {
                    distinct: rc.distinct,
                    all: rc.all,
                    expression: Box::new(self.analyze_expression(&rc.expression)),
                });

        let aggregate_clause = e.aggregate_clause.as_ref().map(|ac| {
            crate::semantics::typed_ast::TypedAggregateClause {
                distinct: ac.distinct,
                identifier: ac.identifier.clone(),
                starting: ac
                    .starting
                    .as_ref()
                    .map(|s| Box::new(self.analyze_expression(&s.expression))),
                expression: Box::new(self.analyze_expression(&ac.expression)),
            }
        });

        self.in_sort_context = true;
        let sort_clause =
            e.sort_clause
                .as_ref()
                .map(|sc| crate::semantics::typed_ast::TypedSortClause {
                    items: sc
                        .items
                        .iter()
                        .map(|item| crate::semantics::typed_ast::TypedSortItem {
                            expression: Box::new(self.analyze_expression(&item.expression)),
                            direction: item.direction,
                        })
                        .collect(),
                });
        self.in_sort_context = false;

        // Pop the query scope — aliases are no longer visible after the query.
        self.scope_manager.pop_scope();

        let inner = TypedExpression::Query(crate::semantics::typed_ast::TypedQuery {
            sources: typed_sources,
            let_clauses,
            relationships,
            where_clause,
            return_clause,
            aggregate_clause,
            sort_clause,
        });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_retrieve(&mut self, e: &ast::Retrieve) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let base_dt = self
            .resolve_type_specifier(&e.data_type)
            .unwrap_or(DataType::any());
        let dt = DataType::List(Box::new(base_dt));

        let named_type = match &e.data_type {
            ast::TypeSpecifier::Named(n) => n.clone(),
            _ => ast::NamedTypeSpecifier {
                namespace: None,
                name: "Unknown".to_string(),
            },
        };

        let inner = TypedExpression::Retrieve(crate::semantics::typed_ast::TypedRetrieve {
            data_type: named_type.clone(),
            codes: e
                .codes
                .as_ref()
                .map(|c| Box::new(self.analyze_expression(c))),
            date_range: e
                .date_range
                .as_ref()
                .map(|d| Box::new(self.analyze_expression(d))),
        });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_if_then_else(&mut self, e: &ast::IfThenElse) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let cond = self.analyze_expression(&e.condition);
        let when = self.analyze_expression(&e.then_expr);
        let oth = self.analyze_expression(&e.else_expr);
        let dt = when.data_type.clone(); // basic, should coerce

        let inner = TypedExpression::IfThenElse(Box::new(cond), Box::new(when), Box::new(oth));

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_case(&mut self, e: &ast::Case) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let comparand = e
            .comparand
            .as_ref()
            .map(|c| Box::new(self.analyze_expression(c)));

        let mut case_items = Vec::new();
        let dt = DataType::any();

        for item in &e.items {
            let when = self.analyze_expression(&item.when);
            let then = self.analyze_expression(&item.then);
            case_items.push(crate::semantics::typed_ast::TypedCaseItem {
                when: Box::new(when),
                then: Box::new(then),
            });
        }

        let else_expr = self.analyze_expression(&e.else_expr);

        let inner = TypedExpression::Case(crate::semantics::typed_ast::TypedCase {
            comparand,
            case_items,
            else_expr: Box::new(else_expr),
        });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_interval_expression(
        &mut self,
        e: &ast::IntervalExpression,
    ) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let low = e.low.as_ref().map(|l| Box::new(self.analyze_expression(l)));
        let high = e
            .high
            .as_ref()
            .map(|h| Box::new(self.analyze_expression(h)));

        let mut inner_dt = DataType::any();
        if let Some(l) = &low {
            inner_dt = l.data_type.clone();
        } else if let Some(h) = &high {
            inner_dt = h.data_type.clone();
        }

        let dt = DataType::Interval(Box::new(inner_dt));

        let inner = TypedExpression::IntervalExpression(
            crate::semantics::typed_ast::TypedIntervalExpression {
                low_closed: e.low_closed,
                high_closed: e.high_closed,
                low,
                high,
            },
        );

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_tuple_expression(
        &mut self,
        e: &ast::TupleExpression,
    ) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let mut typed_elements = Vec::new();
        // tuple type is an aggregate of its element types. To keep things simpler we construct an Any type for now.
        let dt = DataType::any();

        for elem in &e.elements {
            let typed_val = self.analyze_expression(&elem.value);
            typed_elements.push(crate::semantics::typed_ast::TypedTupleElement {
                name: elem.name.clone(),
                value: Box::new(typed_val),
            });
        }

        let inner = TypedExpression::TupleExpression(typed_elements);

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_instance_expression(&mut self, e: &ast::Instance) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let dt = self
            .resolve_type_specifier(&e.class_type)
            .unwrap_or(DataType::any());

        let mut typed_elements = Vec::new();
        for elem in &e.elements {
            let typed_val = self.analyze_expression(&elem.value);
            typed_elements.push(crate::semantics::typed_ast::TypedInstanceElement {
                name: elem.name.clone(),
                value: Box::new(typed_val),
            });
        }

        let inner = TypedExpression::Instance(crate::semantics::typed_ast::TypedInstance {
            class_type: e.class_type.clone(),
            elements: typed_elements,
        });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_type_expression(
        &mut self,
        e: &ast::TypeExpression,
    ) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let typed_operand = self.analyze_expression(&e.operand);
        let dt = match e.operator {
            ast::TypeOperator::Is => DataType::system(crate::datatype::SystemType::Boolean),
            _ => self
                .resolve_type_specifier(&e.type_specifier)
                .unwrap_or(DataType::any()),
        };

        let inner =
            TypedExpression::TypeExpression(crate::semantics::typed_ast::TypedTypeExpression {
                operator: e.operator,
                operand: Box::new(typed_operand),
                type_specifier: e.type_specifier.clone(),
            });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_timing_expression(
        &mut self,
        e: &ast::TimingExpression,
    ) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let left = self.analyze_expression(&e.left);
        let right = self.analyze_expression(&e.right);
        let dt = DataType::system(crate::datatype::SystemType::Boolean);

        let inner =
            TypedExpression::TimingExpression(crate::semantics::typed_ast::TypedTimingExpression {
                left: Box::new(left),
                right: Box::new(right),
                timing: e.timing.clone(),
            });

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }

    pub fn analyze_datetime_component_from(
        &mut self,
        e: &ast::DateTimeComponentFromExpr,
    ) -> TypedNode<TypedExpression> {
        let id = self.generate_node_id();
        let meta = SemanticMeta::default();
        let span = SourceSpan::default();

        let operand = self.analyze_expression(&e.operand);
        let dt = DataType::system(crate::datatype::SystemType::Integer); // component extraction is integer

        let inner = TypedExpression::DateTimeComponentFrom(
            crate::semantics::typed_ast::TypedDateTimeComponentFrom {
                precision: e.precision,
                operand: Box::new(operand),
            },
        );

        TypedNode {
            node_id: id,
            inner,
            data_type: dt,
            meta,
            span,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compile;

    /// A `MemberInvocation` where the source identifier resolves to a library
    /// include alias must be reclassified as a `QualifiedIdentifierRef`, so the
    /// emitter produces `ExpressionRef { library_name: Some("CaseLogic"), ... }`.
    #[test]
    fn test_include_alias_member_becomes_qualified_ref() {
        let src = r#"
            library Test version '1.0'
            include SomeLib called CaseLogic
            define Foo: CaseLogic."My Expr"
        "#;
        let result = compile(src, None).expect("pipeline error");
        // The ELM must contain an ExpressionRef with library_name set.
        let stmts = result.library.statements.as_ref().expect("no statements");
        let foo_expr = stmts.defs.iter().find_map(|d| {
            if let crate::elm::StatementDef::Expression(e) = d {
                if e.name.as_deref() == Some("Foo") {
                    return e.expression.as_deref();
                }
            }
            None
        });
        let foo_expr = foo_expr.expect("Foo definition not found in ELM");
        match foo_expr {
            crate::elm::Expression::ExpressionRef(r) => {
                assert_eq!(r.library_name.as_deref(), Some("CaseLogic"));
                assert_eq!(r.name.as_deref(), Some("My Expr"));
            }
            other => panic!("expected ExpressionRef, got {:?}", std::mem::discriminant(other)),
        }
    }

    /// `Patient.name` where `Patient` is a model type (NOT a library alias) must
    /// remain a `Property` node — regression guard for the include-alias fix.
    #[test]
    fn test_model_member_stays_property() {
        let src = r#"
            library Test version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define PatName: Patient.name
        "#;
        let result = compile(src, None).expect("pipeline error");
        let stmts = result.library.statements.as_ref().expect("no statements");
        let pat_name_expr = stmts.defs.iter().find_map(|d| {
            if let crate::elm::StatementDef::Expression(e) = d {
                if e.name.as_deref() == Some("PatName") {
                    return e.expression.as_deref();
                }
            }
            None
        });
        let pat_name_expr = pat_name_expr.expect("PatName definition not found in ELM");
        assert!(
            matches!(pat_name_expr, crate::elm::Expression::Property(_)),
            "expected Property node for Patient.name, got {:?}",
            std::mem::discriminant(pat_name_expr)
        );
    }
}
