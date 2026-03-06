import sys

file_path = "crates/rh-cql/src/emit/queries.rs"
with open(file_path, "r") as f:
    text = f.read()

text = text.replace("crate::parser::ast::SortDirection::Asc", "crate::parser::ast::SortDirection::Ascending")
text = text.replace("crate::parser::ast::SortDirection::Desc", "crate::parser::ast::SortDirection::Descending")

qname_old = """    let data_type = elm::QName {
        name: Some(retrieve.data_type.name.clone()),
        namespace_uri: retrieve.data_type.specifier.as_ref().map(|s| s.clone()),
    };"""

qname_new = """    let ns = retrieve.data_type.namespace.as_deref().unwrap_or("http://hl7.org/fhir");
    let data_type = format!("{{{}}}{}", ns, retrieve.data_type.name);"""

text = text.replace(qname_old, qname_new)

# handle SortByItem struct initialization
sort_by_old = """    elm::SortByItem::ByExpression(elm::ByExpression {
        direction: Some(direction.to_string()),
        expression: emit_expr(&item.expression, ctx),
    })"""
sort_by_new = """    elm::SortByItem {
        sort_by_type: Some("ByExpression".to_string()),
        direction: match item.direction {
            crate::parser::ast::SortDirection::Ascending => Some(crate::elm::SortDirection::Asc),
            crate::parser::ast::SortDirection::Descending => Some(crate::elm::SortDirection::Desc),
        },
        path: None,
    }"""
text = text.replace(sort_by_old, sort_by_new)
text = text.replace("""    let direction = match item.direction {
        crate::parser::ast::SortDirection::Ascending => "asc",
        crate::parser::ast::SortDirection::Descending => "desc",
    };
    
""", "")

with open(file_path, "w") as f:
    f.write(text)

