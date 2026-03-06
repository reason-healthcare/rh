use super::ElmEmitter;
use crate::elm;
use crate::semantics::typed_ast::{
    TypedExpression, TypedLetClause, TypedNode, TypedQuery, TypedQuerySource,
    TypedRelationshipClause, TypedRetrieve, TypedReturnClause, TypedSortClause, TypedSortItem,
};

pub fn emit_query(
    query: &TypedQuery,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);

    let source = query
        .sources
        .iter()
        .map(|s| emit_query_source(s, ctx, &emit_expr))
        .collect();

    let let_clause = query
        .let_clauses
        .iter()
        .map(|l| emit_let_clause(l, ctx, &emit_expr))
        .collect();

    let relationship = query
        .relationships
        .iter()
        .map(|r| emit_relationship_clause(r, ctx, &emit_expr))
        .collect();

    let where_clause = query
        .where_clause
        .as_ref()
        .map(|w| Box::new(emit_expr(w, ctx)));

    let return_clause = query
        .return_clause
        .as_ref()
        .map(|r| emit_return_clause(r, ctx, &emit_expr));

    let sort = query.sort_clause.as_ref().map(emit_sort_clause);

    elm::Expression::Query(elm::Query {
        element,
        source,
        let_clause,
        relationship,
        where_clause,
        return_clause,
        aggregate: None,
        sort,
    })
}

fn emit_query_source(
    source: &TypedQuerySource,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::AliasedQuerySource {
    elm::AliasedQuerySource {
        alias: Some(source.alias.clone()),
        expression: Some(Box::new(emit_expr(&source.expression, ctx))),
    }
}

fn emit_let_clause(
    let_clause: &TypedLetClause,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::LetClause {
    elm::LetClause {
        identifier: Some(let_clause.identifier.clone()),
        expression: Some(Box::new(emit_expr(&let_clause.expression, ctx))),
    }
}

fn emit_relationship_clause(
    rel: &TypedRelationshipClause,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::RelationshipClause {
    let rel_type = match rel.kind {
        crate::parser::ast::RelationshipKind::With => "with",
        crate::parser::ast::RelationshipKind::Without => "without",
    };

    elm::RelationshipClause {
        relationship_type: Some(rel_type.to_string()),
        alias: Some(rel.source.alias.clone()),
        expression: Some(Box::new(emit_expr(&rel.source.expression, ctx))),
        such_that: rel
            .such_that
            .as_ref()
            .map(|st| Box::new(emit_expr(st, ctx))),
    }
}

fn emit_return_clause(
    ret: &TypedReturnClause,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::ReturnClause {
    elm::ReturnClause {
        distinct: if ret.distinct { Some(true) } else { None }, // Standard distinct translation
        expression: Some(Box::new(emit_expr(&ret.expression, ctx))),
    }
}

fn emit_sort_clause(sort: &TypedSortClause) -> elm::SortClause {
    elm::SortClause {
        by: sort.items.iter().map(emit_sort_item).collect(),
    }
}

fn extract_sort_path(expr: &TypedNode<TypedExpression>) -> Option<String> {
    match &expr.inner {
        TypedExpression::IdentifierRef(id) => Some(id.name.clone()),
        _ => None, // fallback
    }
}

fn emit_sort_item(item: &TypedSortItem) -> elm::SortByItem {
    let path = extract_sort_path(&item.expression);
    let sort_by_type = if path.is_some() {
        "ByColumn"
    } else {
        "ByExpression"
    };

    elm::SortByItem {
        sort_by_type: Some(sort_by_type.to_string()),
        direction: match item.direction {
            crate::parser::ast::SortDirection::Ascending => Some(crate::elm::SortDirection::Asc),
            crate::parser::ast::SortDirection::Descending => Some(crate::elm::SortDirection::Desc),
        },
        path,
    }
}

pub fn emit_retrieve(
    retrieve: &TypedRetrieve,
    node: &TypedNode<TypedExpression>,
    ctx: &mut ElmEmitter,
    emit_expr: impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::Expression {
    let element = ctx.element_fields(node);

    let ns = retrieve
        .data_type
        .namespace
        .as_deref()
        .unwrap_or("http://hl7.org/fhir");
    let data_type = format!("{{{}}}{}", ns, retrieve.data_type.name);

    elm::Expression::Retrieve(elm::Retrieve {
        element,
        data_type: Some(data_type),
        template_id: None,
        context: None,
        code_property: None,
        code_comparator: None,
        codes: retrieve.codes.as_ref().map(|c| Box::new(emit_expr(c, ctx))),
        date_property: None,
        date_range: retrieve
            .date_range
            .as_ref()
            .map(|r| Box::new(emit_expr(r, ctx))),
        include: vec![],
        code_filter: vec![],
        date_filter: vec![],
        other_filter: vec![],
    })
}
