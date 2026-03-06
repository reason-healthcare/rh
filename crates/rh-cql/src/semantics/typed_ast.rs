use crate::datatype::DataType;
use crate::parser::ast::{
    Case, DateTimeComponentFromExpr, IdentifierRef, Instance, IntervalExpression, Literal,
    QualifiedIdentifierRef, Query, Retrieve, TimingExpression, TupleExpression, TypeExpression,
};
use std::hash::Hash;

/// A stable, hash-based identifier for nodes in the Typed AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Represents a source location with line, column, and byte offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

/// Represents a span of source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

/// Metadata about semantic decisions made during analysis.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SemanticMeta {
    pub resolved_overload: Option<String>,
    pub implicit_conversions: Vec<String>,
    pub list_promotion: bool,
    pub resolved_symbol: Option<String>,
}

/// A wrapper struct representing a typed node in the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedNode<T> {
    pub node_id: NodeId,
    pub data_type: DataType,
    pub span: SourceSpan,
    pub meta: SemanticMeta,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedFunctionInvocation {
    pub function: String,
    pub arguments: Vec<TypedNode<TypedExpression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedMemberInvocation {
    pub expression: Box<TypedNode<TypedExpression>>,
    pub member: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIndexInvocation {
    pub expression: Box<TypedNode<TypedExpression>>,
    pub index: Box<TypedNode<TypedExpression>>,
}

/// An expression that has been through semantic analysis and type resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpression {
    Literal(Literal),
    IdentifierRef(IdentifierRef),
    QualifiedIdentifierRef(QualifiedIdentifierRef),
    UnaryExpression(
        crate::parser::ast::UnaryOperator,
        Box<TypedNode<TypedExpression>>,
    ),
    BinaryExpression(
        crate::parser::ast::BinaryOperator,
        Box<TypedNode<TypedExpression>>,
        Box<TypedNode<TypedExpression>>,
    ),
    TernaryExpression(
        crate::parser::ast::TernaryOperator,
        Box<TypedNode<TypedExpression>>,
        Box<TypedNode<TypedExpression>>,
        Box<TypedNode<TypedExpression>>,
    ),
    DateTimeComponentFrom(DateTimeComponentFromExpr),
    TypeExpression(TypeExpression),
    TimingExpression(TimingExpression),
    FunctionInvocation(TypedFunctionInvocation),
    MemberInvocation(TypedMemberInvocation),
    IndexInvocation(TypedIndexInvocation),
    Query(Query),
    Retrieve(Retrieve),
    IfThenElse(
        Box<TypedNode<TypedExpression>>,
        Box<TypedNode<TypedExpression>>,
        Box<TypedNode<TypedExpression>>,
    ),
    Case(Case),
    IntervalExpression(IntervalExpression),
    ListExpression(Vec<TypedNode<TypedExpression>>),
    TupleExpression(TupleExpression),
    Instance(Instance),
    LetClause(String, Box<TypedNode<TypedExpression>>),
    Parenthesized(Box<TypedNode<TypedExpression>>),
}

/// A statement that has been through semantic analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum TypedStatement {
    ExpressionDef {
        name: String,
        body: TypedNode<TypedExpression>,
    },
    FunctionDef {
        name: String,
        parameters: Vec<crate::parser::ast::FunctionParameter>,
        return_type: Option<crate::parser::ast::TypeSpecifier>,
        body: Option<TypedNode<TypedExpression>>,
        fluent: bool,
    },
}

/// A parameter that has been through semantic analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedParameterDef {
    pub name: String,
    pub type_specifier: Option<crate::parser::ast::TypeSpecifier>,
    pub default: Option<TypedNode<TypedExpression>>,
    pub access: crate::parser::ast::AccessModifier,
}

/// A library that has been through semantic analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedLibrary {
    pub identifier: Option<crate::parser::ast::LibraryIdentifier>,
    pub usings: Vec<crate::parser::ast::UsingDef>,
    pub includes: Vec<crate::parser::ast::IncludeDef>,
    pub codesystems: Vec<crate::parser::ast::CodeSystemDef>,
    pub valuesets: Vec<crate::parser::ast::ValueSetDef>,
    pub codes: Vec<crate::parser::ast::CodeDef>,
    pub concepts: Vec<crate::parser::ast::ConceptDef>,
    pub parameters: Vec<TypedParameterDef>,
    pub contexts: Vec<crate::parser::ast::ContextDef>,
    pub statements: Vec<TypedNode<TypedStatement>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::{DataType, SystemType};

    #[test]
    fn test_node_id_stability() {
        let id1 = NodeId(42);
        let id2 = NodeId(42);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_typed_node_construction() {
        let id = NodeId(1);
        let dt = DataType::System(SystemType::Integer);
        let span = SourceSpan::default();
        let meta = SemanticMeta::default();
        let inner = TypedExpression::Literal(Literal::Integer(42));

        let node = TypedNode {
            node_id: id,
            data_type: dt,
            span,
            meta,
            inner,
        };

        assert_eq!(node.node_id, NodeId(1));
    }
}
