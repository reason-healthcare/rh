import sys

file_path = "crates/rh-cql/src/emit/queries.rs"
with open(file_path, "r") as f:
    text = f.read()

target = """fn emit_sort_clause(
    sort: &TypedSortClause,
    ctx: &mut ElmEmitter,
    emit_expr: &impl Fn(&TypedNode<TypedExpression>, &mut ElmEmitter) -> elm::Expression,
) -> elm::SortClause {
    elm::SortClause {
        by: sort.items.iter().map(|item| emit_sort_item(item)).collect(),
    }
}

fn emit_sort_item(
    item: &TypedSortItem,
) -> elm::SortByItem {
    elm::SortByItem {
        sort_by_type: Some("ByExpression".to_string()),
        direction: match item.direction {
            crate::parser::ast::SortDirection::Ascending => Some(crate::elm::SortDirection::Asc),
            crate::parser::ast::SortDirection::Descending => Some(crate::elm::SortDirection::Desc),
        },
        path: None,
    }
}"""

replacement = """fn emit_sort_clause(
    sort: &TypedSortClause,
) -> elm::SortClause {
    elm::SortClause {
        by: sort.items.iter().map(|item| emit_sort_item(item)).collect(),
    }
}

fn extract_sort_path(expr: &TypedNode<TypedExpression>) -> Option<String> {
    match &expr.inner {
        TypedExpression::IdentifierRef(id) => Some(id.name.clone()),
        // TypedExpression::Property(prop) => Some(prop.path.clone()), // We might not have this imported
        _ => None, // more complex path extraction may be needed
    }
}

fn emit_sort_item(
    item: &TypedSortItem,
) -> elm::SortByItem {
    let path = extract_sort_path(&item.expression);
    let sort_by_type = if path.is_some() { "ByColumn" } else { "ByExpression" };

    elm::SortByItem {
        sort_by_type: Some(sort_by_type.to_string()),
        direction: match item.direction {
            crate::parser::ast::SortDirection::Ascending => Some(crate::elm::SortDirection::Asc),
            crate::parser::ast::SortDirection::Descending => Some(crate::elm::SortDirection::Desc),
        },
        path,
    }
}"""

text = text.replace(target, replacement)

call_target = """    let sort = query
        .sort_clause
        .as_ref()
        .map(|s| emit_sort_clause(s, ctx, &emit_expr));"""

call_replacement = """    let sort = query
        .sort_clause
        .as_ref()
        .map(|s| emit_sort_clause(s));"""

text = text.replace(call_target, call_replacement)

with open(file_path, "w") as f:
    f.write(text)
