import re

with open("crates/rh-cql/src/semantics/analyzer.rs", "r") as f:
    text = f.read()

fns_to_add = """
    fn analyze_query(&mut self, e: &ast::Query) -> TypedNode<TypedExpression> {
        self.scope_manager.push_query_scope();

        let mut sources = Vec::new();
        for source in &e.sources {
            let expr = self.analyze_expression(&source.expression);
            // register alias in query scope
            self.scope_manager.register_query_alias(source.alias.clone(), expr.data_type.clone());
            sources.push(crate::semantics::typed_ast::TypedQuerySource {
                expression: Box::new(expr),
                alias: source.alias.clone(),
                location: source.location,
            });
        }

        let mut let_clauses = Vec::new();
        for let_c in &e.let_clauses {
            let expr = self.analyze_expression(&let_c.expression);
            self.scope_manager.register_symbol(
                Symbol::new(let_c.identifier.clone(), SymbolKind::Let)
                    .with_type(expr.data_type.clone())
            );
            let_clauses.push(crate::semantics::typed_ast::TypedLetClause {
                identifier: let_c.identifier.clone(),
                expression: Box::new(expr),
                location: let_c.location,
            });
        }

        let mut relationships = Vec::new();
        for rel in &e.relationships {
            let expr = self.analyze_expression(&rel.source.expression);
            
            // Push a temporary scope for such_that condition
            self.scope_manager.push_query_scope();
            self.scope_manager.register_query_alias(rel.source.alias.clone(), expr.data_type.clone());
            
            let such_that = rel.such_that.as_ref().map(|st| Box::new(self.analyze_expression(st)));
            self.scope_manager.pop_query_scope();

            relationships.push(crate::semantics::typed_ast::TypedRelationshipClause {
                kind: rel.kind,
                source: crate::semantics::typed_ast::TypedQuerySource {
                    expression: Box::new(expr),
                    alias: rel.source.alias.clone(),
                    location: rel.source.location,
                },
                such_that,
                location: rel.location,
            });
        }

        let where_clause = e.where_clause.as_ref().map(|w| Box::new(self.analyze_expression(w)));
        let return_clause = e.return_clause.as_ref().map(|r| {
            crate::semantics::typed_ast::TypedReturnClause {
                distinct: r.distinct,
                all: r.all,
                expression: Box::new(self.analyze_expression(&r.expression)),
                location: r.location,
            }
        });
        
        let sort_clause = e.sort_clause.as_ref().map(|s| {
            crate::semantics::typed_ast::TypedSortClause {
                items: s.items.iter().map(|item| crate::semantics::typed_ast::TypedSortItem {
                    expression: Box::new(self.analyze_expression(&item.expression)),
                    direction: item.direction,
                    location: item.location,
                }).collect(),
                location: s.location,
            }
        });

        // Compute output type. Generally it's a List of the return type, or List of the primary source type.
        let mut dt = DataType::Unknown;
        if let Some(r) = &return_clause {
            dt = DataType::List(Box::new(r.expression.data_type.clone()));
            if return_clause.as_ref().map(|rc| rc.expression.data_type.clone()) == Some(DataType::System(crate::datatype::SystemType::Any)) {
               dt = DataType::List(Box::new(DataType::Unknown));
            }
        } else if let Some(s) = sources.first() {
            dt = DataType::List(Box::new(s.expression.data_type.clone()));
        }

        self.scope_manager.pop_query_scope();

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::Query(crate::semantics::typed_ast::TypedQuery {
                sources,
                let_clauses,
                relationships,
                where_clause,
                return_clause,
                sort_clause,
                location: e.location,
            }),
        }
    }

    fn analyze_retrieve(&mut self, e: &ast::Retrieve) -> TypedNode<TypedExpression> {
        let mut dt = DataType::Unknown;
        let mut codes = None;
        let mut date_range = None;

        if let Some(c) = &e.codes {
           codes = Some(Box::new(self.analyze_expression(c)));
        }
        if let Some(d) = &e.date_range {
           date_range = Some(Box::new(self.analyze_expression(d)));
        }

        let namespace = e.data_type.namespace.clone().unwrap_or_else(|| "FHIR".to_string());
        let name = e.data_type.name.clone();
        
        if let Some(model_info) = self._model_provider.get_model(&namespace, None) {
             for ti in &model_info.type_info {
                 if let crate::modelinfo::TypeInfo::ClassInfo(ci) = ti {
                     if ci.name.as_deref() == Some(&name) {
                         dt = DataType::List(Box::new(DataType::Model {
                             namespace,
                             name,
                         }));
                         break;
                     }
                 }
             }
        }
        
        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::Retrieve(crate::semantics::typed_ast::TypedRetrieve {
                data_type: e.data_type.clone(),
                codes,
                date_range,
                location: e.location,
            }),
        }
    }

    fn analyze_if_then_else(&mut self, e: &ast::IfThenElse) -> TypedNode<TypedExpression> {
        let condition = self.analyze_expression(&e.condition);
        let then_expr = self.analyze_expression(&e.then_expr);
        let else_expr = self.analyze_expression(&e.else_expr);

        // TODO unify types
        let dt = then_expr.data_type.clone();

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::IfThenElse(
                Box::new(condition),
                Box::new(then_expr),
                Box::new(else_expr),
            ),
        }
    }

    fn analyze_case(&mut self, e: &ast::Case) -> TypedNode<TypedExpression> {
        let comparand = e.comparand.as_ref().map(|c| Box::new(self.analyze_expression(c)));
        
        let mut case_items = Vec::new();
        for item in &e.case_items {
            case_items.push(crate::semantics::typed_ast::TypedCaseItem {
                when: Box::new(self.analyze_expression(&item.when)),
                then: Box::new(self.analyze_expression(&item.then)),
            });
        }
        let else_expr = Box::new(self.analyze_expression(&e.else_expr));

        // TODO unify types
        let dt = else_expr.data_type.clone();

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::Case(crate::semantics::typed_ast::TypedCase {
                comparand,
                case_items,
                else_expr,
            }),
        }
    }

    fn analyze_interval_expression(&mut self, e: &ast::IntervalExpression) -> TypedNode<TypedExpression> {
        let low = e.low.as_ref().map(|l| Box::new(self.analyze_expression(l)));
        let high = e.high.as_ref().map(|h| Box::new(self.analyze_expression(h)));

        let dt = DataType::Interval(Box::new(DataType::Unknown));

        TypedNode {
            node_id: self.generate_node_id(),
            data_type: dt,
            span: SourceSpan::default(),
            meta: SemanticMeta::default(),
            inner: TypedExpression::IntervalExpression(crate::semantics::typed_ast::TypedIntervalExpression {
                low,
                low_closed: e.low_closed,
                high,
                high_closed: e.high_closed,
            }),
        }
    }

"""

if "fn analyze_query" not in text:
    text = text.replace("fn generate_node_id", fns_to_add + "\n    fn generate_node_id")

text = text.replace("ast::Expression::Query(e) => TypedExpression::Query(e.clone()),", "ast::Expression::Query(e) => return self.analyze_query(e),")
text = text.replace("ast::Expression::Retrieve(e) => TypedExpression::Retrieve(e.clone()),", "ast::Expression::Retrieve(e) => return self.analyze_retrieve(e),")
text = text.replace("""ast::Expression::IfThenElse(e) => TypedExpression::IfThenElse(
                Box::new(self.analyze_expression(&e.condition)),
                Box::new(self.analyze_expression(&e.then_expr)),
                Box::new(self.analyze_expression(&e.else_expr)),
            ),""", "ast::Expression::IfThenElse(e) => return self.analyze_if_then_else(e),")
text = text.replace("ast::Expression::Case(e) => TypedExpression::Case(e.clone()),", "ast::Expression::Case(e) => return self.analyze_case(e),")
text = text.replace("""ast::Expression::IntervalExpression(e) => {
                TypedExpression::IntervalExpression(e.clone())
            }""", "ast::Expression::IntervalExpression(e) => return self.analyze_interval_expression(e),")

with open("crates/rh-cql/src/semantics/analyzer.rs", "w") as f:
    f.write(text)
