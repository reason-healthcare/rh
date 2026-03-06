import sys

file_path = "crates/rh-cql/src/emit/queries.rs"
with open(file_path, "r") as f:
    text = f.read()

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
        TypedExpression::Property(prop) => Some(prop.path.clone()),
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

import re
text = re.sub(r'fn emit_sort_clause\([^}]+\} *\n+fn emit_sort_item\([^}]+\}', replacement, text, flags=re.DOTALL)

with open(file_path, "w") as f:
    f.write(text)
