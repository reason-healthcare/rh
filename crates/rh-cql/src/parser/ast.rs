//! CQL Abstract Syntax Tree Types
//!
//! This module defines the AST types for CQL parsing. These types represent
//! the syntactic structure of CQL source code before semantic analysis
//! and translation to ELM.
//!
//! ## Note on AST vs ELM
//!
//! The AST types here are distinct from ELM types:
//! - **AST**: Syntactic representation of CQL source code
//! - **ELM**: Semantic representation after type checking and resolution
//!
//! The translator (Phase 4) converts AST to ELM.

use crate::parser::span::SourceLocation;
use serde::{Deserialize, Serialize};

// ============================================================================
// Library Structure
// ============================================================================

/// A CQL Library (top-level compilation unit)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Library {
    /// Library identifier (name and optional version)
    pub identifier: Option<LibraryIdentifier>,
    /// Model definitions (using statements)
    pub usings: Vec<UsingDef>,
    /// Included libraries
    pub includes: Vec<IncludeDef>,
    /// CodeSystem definitions
    pub codesystems: Vec<CodeSystemDef>,
    /// ValueSet definitions
    pub valuesets: Vec<ValueSetDef>,
    /// Code definitions
    pub codes: Vec<CodeDef>,
    /// Concept definitions
    pub concepts: Vec<ConceptDef>,
    /// Parameter definitions
    pub parameters: Vec<ParameterDef>,
    /// Context definition
    pub contexts: Vec<ContextDef>,
    /// Expression and function definitions
    pub statements: Vec<Statement>,
}

/// Library identifier with optional version
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryIdentifier {
    /// Library name (may be qualified like "CMS.Common")
    pub name: String,
    /// Library version
    pub version: Option<String>,
}

/// Using definition (data model reference)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsingDef {
    /// Model name (e.g., "FHIR")
    pub model_name: String,
    /// Optional version
    pub version: Option<String>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Include definition (library inclusion)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncludeDef {
    /// Library path (may be qualified)
    pub path: String,
    /// Library version
    pub version: Option<String>,
    /// Local alias
    pub alias: Option<String>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// CodeSystem definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSystemDef {
    /// Name of the code system
    pub name: String,
    /// OID or URL
    pub id: String,
    /// Optional version
    pub version: Option<String>,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// ValueSet definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueSetDef {
    /// Name of the value set
    pub name: String,
    /// OID or URL
    pub id: String,
    /// Optional version
    pub version: Option<String>,
    /// Code systems
    pub codesystems: Vec<String>,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Code definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeDef {
    /// Name of the code
    pub name: String,
    /// Code value
    pub code: String,
    /// Code system reference
    pub codesystem: String,
    /// Optional display
    pub display: Option<String>,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Concept definition (aggregation of codes)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptDef {
    /// Name of the concept
    pub name: String,
    /// Code references
    pub codes: Vec<String>,
    /// Optional display
    pub display: Option<String>,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Parameter definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDef {
    /// Parameter name
    pub name: String,
    /// Type specifier
    pub type_specifier: Option<TypeSpecifier>,
    /// Default value
    pub default: Option<Expression>,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Context definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextDef {
    /// Context name (e.g., "Patient", "Practitioner")
    pub name: String,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Access modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AccessModifier {
    #[default]
    Public,
    Private,
}

// ============================================================================
// Statements
// ============================================================================

/// A statement (expression or function definition)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Expression definition
    ExpressionDef(ExpressionDef),
    /// Function definition
    FunctionDef(FunctionDef),
}

/// Expression definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionDef {
    /// Name of the expression
    pub name: String,
    /// The expression body
    pub expression: Expression,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Parameters
    pub parameters: Vec<FunctionParameter>,
    /// Return type
    pub return_type: Option<TypeSpecifier>,
    /// Function body (None for external functions)
    pub body: Option<Expression>,
    /// Whether this is a fluent function
    pub fluent: bool,
    /// Whether this is external
    pub external: bool,
    /// Access modifier
    pub access: AccessModifier,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionParameter {
    /// Parameter name
    pub name: String,
    /// Type specifier
    pub type_specifier: Option<TypeSpecifier>,
}

// ============================================================================
// Type Specifiers
// ============================================================================

/// Type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeSpecifier {
    /// Named type (e.g., "Integer", "FHIR.Patient")
    Named(NamedTypeSpecifier),
    /// List type (e.g., "List<Integer>")
    List(ListTypeSpecifier),
    /// Interval type (e.g., "Interval<Date>")
    Interval(IntervalTypeSpecifier),
    /// Tuple type
    Tuple(TupleTypeSpecifier),
    /// Choice type
    Choice(ChoiceTypeSpecifier),
}

/// Named type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedTypeSpecifier {
    /// Namespace (optional)
    pub namespace: Option<String>,
    /// Type name
    pub name: String,
}

/// List type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTypeSpecifier {
    /// Element type
    pub element_type: Box<TypeSpecifier>,
}

/// Interval type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalTypeSpecifier {
    /// Point type
    pub point_type: Box<TypeSpecifier>,
}

/// Tuple type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleTypeSpecifier {
    /// Element definitions
    pub elements: Vec<TupleElementDef>,
}

/// Tuple element definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleElementDef {
    /// Element name
    pub name: String,
    /// Element type
    pub element_type: TypeSpecifier,
}

/// Choice type specifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceTypeSpecifier {
    /// Choice types
    pub types: Vec<TypeSpecifier>,
}

// ============================================================================
// Expressions
// ============================================================================

/// CQL Expression (AST node)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    // Literals
    Literal(Literal),

    // References
    IdentifierRef(IdentifierRef),
    QualifiedIdentifierRef(QualifiedIdentifierRef),

    // Operators
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    TernaryExpression(TernaryExpression),

    // DateTime component extraction (year from, month from, etc.)
    DateTimeComponentFrom(DateTimeComponentFromExpr),

    // Type operations
    TypeExpression(TypeExpression),

    // Function/invocation
    FunctionInvocation(FunctionInvocation),
    MemberInvocation(MemberInvocation),
    IndexInvocation(IndexInvocation),

    // Query
    Query(Query),
    Retrieve(Retrieve),

    // Conditionals
    IfThenElse(IfThenElse),
    Case(Case),

    // Interval/List constructors
    IntervalExpression(IntervalExpression),
    ListExpression(ListExpression),
    TupleExpression(TupleExpression),

    // Instance
    Instance(Instance),

    // Let
    Let(LetClause),

    // Parenthesized
    Parenthesized(Box<Expression>),
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Null,
    Boolean(bool),
    Integer(i64),
    Long(i64),
    Decimal(f64),
    String(String),
    Date(String),
    DateTime(String),
    Time(String),
    Quantity {
        value: f64,
        unit: String,
    },
    Ratio {
        numerator: Box<Literal>,
        denominator: Box<Literal>,
    },
    Code {
        code: String,
        system: Option<String>,
        display: Option<String>,
    },
}

/// Simple identifier reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentifierRef {
    pub name: String,
    pub location: Option<SourceLocation>,
}

/// Qualified identifier reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifiedIdentifierRef {
    pub qualifier: String,
    pub name: String,
    pub location: Option<SourceLocation>,
}

/// DateTime component extraction (year from, month from, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTimeComponentFromExpr {
    pub precision: DateTimePrecision,
    pub operand: Box<Expression>,
    pub location: Option<SourceLocation>,
}

/// Unary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub location: Option<SourceLocation>,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Negate,
    Exists,
    IsNull,
    IsTrue,
    IsFalse,
    Predecessor,
    Successor,
    Distinct,
    Flatten,
    Start,
    End,
    Width,
    PointFrom,
    Collapse,
    Expand,
    Singleton,
    DateFrom,
    TimeFrom,
    TimezoneOffsetFrom,
    ToBoolean,
    ToInteger,
    ToLong,
    ToDecimal,
    ToString,
    ToDate,
    ToDateTime,
    ToTime,
    ToQuantity,
    ToConcept,
}

/// Binary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    /// Precision for temporal operators (day, month, year, etc.)
    pub precision: Option<DateTimePrecision>,
    pub location: Option<SourceLocation>,
}

/// Date/time precision for temporal operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateTimePrecision {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    TruncatedDivide,
    Modulo,
    Power,
    Log,

    // Comparison
    Equal,
    NotEqual,
    Equivalent,
    NotEquivalent,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,

    // Logical
    And,
    Or,
    Xor,
    Implies,

    // String
    Concatenate,

    // Membership
    In,
    Contains,

    // Interval
    Includes,
    IncludedIn,
    ProperlyIncludes,
    ProperlyIncludedIn,
    Overlaps,
    OverlapsBefore,
    OverlapsAfter,
    Meets,
    MeetsBefore,
    MeetsAfter,
    Starts,
    Ends,
    During,
    Before,
    After,
    SameAs,
    SameOrBefore,
    SameOrAfter,
    Within,

    // List
    Union,
    Intersect,
    Except,
    IndexOf,
}

impl BinaryOperator {
    /// Returns true if this is a comparison operator.
    ///
    /// Comparison operators compare two values and return a Boolean result.
    /// This includes equality, equivalence, and ordering comparisons.
    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::Equivalent
                | BinaryOperator::NotEquivalent
                | BinaryOperator::Less
                | BinaryOperator::LessOrEqual
                | BinaryOperator::Greater
                | BinaryOperator::GreaterOrEqual
        )
    }
}

/// Ternary expression (between)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TernaryExpression {
    pub operator: TernaryOperator,
    pub first: Box<Expression>,
    pub second: Box<Expression>,
    pub third: Box<Expression>,
    pub location: Option<SourceLocation>,
}

/// Ternary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TernaryOperator {
    Between,
    ReplaceMatches,
}

/// Type expression (is, as, convert)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeExpression {
    pub operator: TypeOperator,
    pub operand: Box<Expression>,
    pub type_specifier: TypeSpecifier,
    pub location: Option<SourceLocation>,
}

/// Type operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeOperator {
    Is,
    As,
    Cast,
    Convert,
}

/// Function invocation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionInvocation {
    /// Optional library qualifier
    pub library: Option<String>,
    /// Function name
    pub name: String,
    /// Arguments
    pub arguments: Vec<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Member invocation (property access)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberInvocation {
    /// Source expression
    pub source: Box<Expression>,
    /// Member name
    pub name: String,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Index invocation (array access)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexInvocation {
    /// Source expression
    pub source: Box<Expression>,
    /// Index expression
    pub index: Box<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

// ============================================================================
// Query
// ============================================================================

/// CQL Query expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    /// Query sources
    pub sources: Vec<QuerySource>,
    /// Let clauses
    pub let_clauses: Vec<LetClause>,
    /// Relationship clauses (with/without)
    pub relationships: Vec<RelationshipClause>,
    /// Where clause
    pub where_clause: Option<Box<Expression>>,
    /// Return clause
    pub return_clause: Option<ReturnClause>,
    /// Sort clause
    pub sort_clause: Option<SortClause>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Query source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySource {
    /// Source expression
    pub expression: Box<Expression>,
    /// Alias
    pub alias: String,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Let clause
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetClause {
    /// Identifier
    pub identifier: String,
    /// Expression
    pub expression: Box<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Relationship clause (with/without)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipClause {
    /// With or without
    pub kind: RelationshipKind,
    /// Related source
    pub source: QuerySource,
    /// Such that condition
    pub such_that: Option<Box<Expression>>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Relationship kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipKind {
    With,
    Without,
}

/// Return clause
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnClause {
    /// Distinct flag
    pub distinct: bool,
    /// All flag
    pub all: bool,
    /// Return expression
    pub expression: Box<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Sort clause
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortClause {
    /// Sort items
    pub items: Vec<SortItem>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Sort item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortItem {
    /// Sort expression
    pub expression: Box<Expression>,
    /// Sort direction
    pub direction: SortDirection,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

// ============================================================================
// Retrieve
// ============================================================================

/// Retrieve expression (data access)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retrieve {
    /// Data type to retrieve
    pub data_type: TypeSpecifier,
    /// Context (optional)
    pub context: Option<Box<Expression>>,
    /// Code path (e.g., "code")
    pub code_path: Option<String>,
    /// Codes to filter by
    pub codes: Option<Box<Expression>>,
    /// Date path (e.g., "effectiveDateTime")
    pub date_path: Option<String>,
    /// Date range to filter by
    pub date_range: Option<Box<Expression>>,
    /// Source location
    pub location: Option<SourceLocation>,
}

// ============================================================================
// Conditionals
// ============================================================================

/// If-then-else expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfThenElse {
    /// Condition
    pub condition: Box<Expression>,
    /// Then expression
    pub then_expr: Box<Expression>,
    /// Else expression
    pub else_expr: Box<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Case expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Case {
    /// Comparand (for simple case)
    pub comparand: Option<Box<Expression>>,
    /// Case items
    pub items: Vec<CaseItem>,
    /// Else expression
    pub else_expr: Box<Expression>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Case item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseItem {
    /// When condition
    pub when: Box<Expression>,
    /// Then expression
    pub then: Box<Expression>,
}

// ============================================================================
// Collection Constructors
// ============================================================================

/// Interval expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalExpression {
    /// Low bound
    pub low: Option<Box<Expression>>,
    /// High bound
    pub high: Option<Box<Expression>>,
    /// Low closed
    pub low_closed: bool,
    /// High closed
    pub high_closed: bool,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// List expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListExpression {
    /// Elements
    pub elements: Vec<Expression>,
    /// Type hint
    pub type_specifier: Option<TypeSpecifier>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Tuple expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleExpression {
    /// Elements
    pub elements: Vec<TupleElement>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Tuple element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleElement {
    /// Element name
    pub name: String,
    /// Element value
    pub value: Box<Expression>,
}

/// Instance expression (type instantiation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    /// Class type
    pub class_type: TypeSpecifier,
    /// Elements
    pub elements: Vec<InstanceElement>,
    /// Source location
    pub location: Option<SourceLocation>,
}

/// Instance element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceElement {
    /// Element name
    pub name: String,
    /// Element value
    pub value: Box<Expression>,
}
