with open("crates/rh-cql/src/semantics/typed_ast.rs", "r") as f:
    text = f.read()

types_to_add = """
#[derive(Debug, Clone, PartialEq)]
pub struct TypedQuery {
    pub sources: Vec<TypedQuerySource>,
    pub let_clauses: Vec<TypedLetClause>,
    pub relationships: Vec<TypedRelationshipClause>,
    pub where_clause: Option<Box<TypedNode<TypedExpression>>>,
    pub return_clause: Option<TypedReturnClause>,
    pub sort_clause: Option<TypedSortClause>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedQuerySource {
    pub expression: Box<TypedNode<TypedExpression>>,
    pub alias: String,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedLetClause {
    pub identifier: String,
    pub expression: Box<TypedNode<TypedExpression>>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedRelationshipClause {
    pub kind: crate::parser::ast::RelationshipKind,
    pub source: TypedQuerySource,
    pub such_that: Option<Box<TypedNode<TypedExpression>>>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedReturnClause {
    pub distinct: bool,
    pub all: bool,
    pub expression: Box<TypedNode<TypedExpression>>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedSortClause {
    pub items: Vec<TypedSortItem>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedSortItem {
    pub expression: Box<TypedNode<TypedExpression>>,
    pub direction: crate::parser::ast::SortDirection,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedRetrieve {
    pub data_type: crate::parser::ast::NamedTypeSpecifier,
    pub codes: Option<Box<TypedNode<TypedExpression>>>,
    pub date_range: Option<Box<TypedNode<TypedExpression>>>,
    pub location: Option<crate::parser::ast::SourceLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedCase {
    pub comparand: Option<Box<TypedNode<TypedExpression>>>,
    pub case_items: Vec<TypedCaseItem>,
    pub else_expr: Box<TypedNode<TypedExpression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedCaseItem {
    pub when: Box<TypedNode<TypedExpression>>,
    pub then: Box<TypedNode<TypedExpression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIntervalExpression {
    pub low: Option<Box<TypedNode<TypedExpression>>>,
    pub low_closed: bool,
    pub high: Option<Box<TypedNode<TypedExpression>>>,
    pub high_closed: bool,
}

"""

if "pub struct TypedQuery" not in text:
    text = text.replace("pub struct TypedIndexInvocation {", types_to_add + "\npub struct TypedIndexInvocation {")

text = text.replace("Query(Query)", "Query(TypedQuery)")
text = text.replace("Retrieve(Retrieve)", "Retrieve(TypedRetrieve)")
text = text.replace("Case(Case)", "Case(TypedCase)")
text = text.replace("IntervalExpression(IntervalExpression)", "IntervalExpression(TypedIntervalExpression)")

with open("crates/rh-cql/src/semantics/typed_ast.rs", "w") as f:
    f.write(text)
