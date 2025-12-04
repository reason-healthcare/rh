//! ELM expression types.
//!
//! This module defines the core expression types in ELM. Expressions are the
//! fundamental building blocks of clinical logic in CQL/ELM.

use serde::{Deserialize, Serialize};

use super::{QName, SortDirection, TypeSpecifier};

/// The main expression enum representing all possible ELM expressions.
///
/// This enum uses serde's internally tagged representation, where the `type`
/// field determines which variant is being serialized/deserialized.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    // === Literals ===
    Literal(Literal),
    Null(Null),

    // === References ===
    IdentifierRef(IdentifierRef),
    ExpressionRef(ExpressionRef),
    FunctionRef(FunctionRef),
    ParameterRef(ParameterRef),
    OperandRef(OperandRef),
    AliasRef(AliasRef),
    QueryLetRef(QueryLetRef),
    CodeSystemRef(CodeSystemRef),
    ValueSetRef(ValueSetRef),
    CodeRef(CodeRef),
    ConceptRef(ConceptRef),

    // === Property Access ===
    Property(Property),

    // === Control Flow ===
    If(IfExpr),
    Case(Case),

    // === Logical Operators ===
    And(NaryExpression),
    Or(NaryExpression),
    Xor(BinaryExpression),
    Not(UnaryExpression),
    Implies(BinaryExpression),

    // === Nullological ===
    IsNull(UnaryExpression),
    IsTrue(UnaryExpression),
    IsFalse(UnaryExpression),
    Coalesce(NaryExpression),

    // === Comparison ===
    Equal(BinaryExpression),
    Equivalent(BinaryExpression),
    NotEqual(BinaryExpression),
    Less(BinaryExpression),
    Greater(BinaryExpression),
    LessOrEqual(BinaryExpression),
    GreaterOrEqual(BinaryExpression),

    // === Arithmetic ===
    Add(BinaryExpression),
    Subtract(BinaryExpression),
    Multiply(BinaryExpression),
    Divide(BinaryExpression),
    TruncatedDivide(BinaryExpression),
    Modulo(BinaryExpression),
    Ceiling(UnaryExpression),
    Floor(UnaryExpression),
    Truncate(UnaryExpression),
    Abs(UnaryExpression),
    Negate(UnaryExpression),
    Round(UnaryExpression),
    Ln(UnaryExpression),
    Exp(UnaryExpression),
    Log(BinaryExpression),
    Power(BinaryExpression),
    Successor(UnaryExpression),
    Predecessor(UnaryExpression),
    MinValue(TypedExpression),
    MaxValue(TypedExpression),

    // === String ===
    Concatenate(NaryExpression),
    Combine(Combine),
    Split(Split),
    SplitOnMatches(SplitOnMatches),
    Length(UnaryExpression),
    Upper(UnaryExpression),
    Lower(UnaryExpression),
    PositionOf(PositionOf),
    LastPositionOf(PositionOf),
    Substring(Substring),
    StartsWith(BinaryExpression),
    EndsWith(BinaryExpression),
    Matches(BinaryExpression),
    ReplaceMatches(TernaryExpression),

    // === Date/Time ===
    Today(NullaryExpression),
    Now(NullaryExpression),
    TimeOfDay(NullaryExpression),
    DateTime(DateTimeExpr),
    Date(DateExpr),
    Time(TimeExpr),
    DateTimeComponentFrom(DateTimeComponentFrom),
    DifferenceBetween(TimeBinaryExpression),
    DurationBetween(TimeBinaryExpression),
    SameAs(TimeBinaryExpression),
    SameOrBefore(TimeBinaryExpression),
    SameOrAfter(TimeBinaryExpression),

    // === Interval ===
    Interval(IntervalExpr),
    Width(UnaryExpression),
    Size(UnaryExpression),
    Start(UnaryExpression),
    End(UnaryExpression),
    Contains(BinaryExpression),
    ProperContains(BinaryExpression),
    In(BinaryExpression),
    ProperIn(BinaryExpression),
    Includes(BinaryExpression),
    ProperIncludes(BinaryExpression),
    IncludedIn(BinaryExpression),
    ProperIncludedIn(BinaryExpression),
    Before(TimeBinaryExpression),
    After(TimeBinaryExpression),
    Meets(BinaryExpression),
    MeetsBefore(BinaryExpression),
    MeetsAfter(BinaryExpression),
    Overlaps(TimeBinaryExpression),
    OverlapsBefore(TimeBinaryExpression),
    OverlapsAfter(TimeBinaryExpression),
    Starts(TimeBinaryExpression),
    Ends(TimeBinaryExpression),
    Union(BinaryExpression),
    Intersect(BinaryExpression),
    Except(BinaryExpression),
    Collapse(UnaryExpression),
    Expand(BinaryExpression),
    PointFrom(UnaryExpression),

    // === List ===
    List(ListExpr),
    Exists(UnaryExpression),
    Times(BinaryExpression),
    First(UnaryExpression),
    Last(UnaryExpression),
    Indexer(BinaryExpression),
    Flatten(UnaryExpression),
    Sort(Sort),
    Distinct(UnaryExpression),
    Current(Current),
    Iteration(Iteration),
    SingletonFrom(UnaryExpression),
    Slice(Slice),
    Repeat(Repeat),

    // === Aggregate ===
    Count(AggregateExpression),
    Sum(AggregateExpression),
    Min(AggregateExpression),
    Max(AggregateExpression),
    Avg(AggregateExpression),
    Median(AggregateExpression),
    Mode(AggregateExpression),
    Variance(AggregateExpression),
    StdDev(AggregateExpression),
    PopulationVariance(AggregateExpression),
    PopulationStdDev(AggregateExpression),
    AllTrue(AggregateExpression),
    AnyTrue(AggregateExpression),
    Aggregate(AggregateClause),

    // === Type Operations ===
    As(AsExpr),
    Convert(ConvertExpr),
    Is(IsExpr),
    ToBoolean(UnaryExpression),
    ToConcept(UnaryExpression),
    ToDateTime(UnaryExpression),
    ToDate(UnaryExpression),
    ToDecimal(UnaryExpression),
    ToInteger(UnaryExpression),
    ToLong(UnaryExpression),
    ToQuantity(UnaryExpression),
    ToRatio(UnaryExpression),
    #[serde(rename = "ToString")]
    ToStringExpr(UnaryExpression),
    ToTime(UnaryExpression),
    ToList(UnaryExpression),
    ToChars(UnaryExpression),
    CanConvert(CanConvert),
    CanConvertQuantity(BinaryExpression),

    // === Clinical ===
    CalculateAge(UnaryExpression),
    CalculateAgeAt(BinaryExpression),
    InValueSet(InValueSet),
    InCodeSystem(InCodeSystem),
    AnyInValueSet(AnyInValueSet),
    AnyInCodeSystem(AnyInCodeSystem),
    Subsumes(BinaryExpression),
    SubsumedBy(BinaryExpression),
    ExpandValueSet(UnaryExpression),

    // === Query ===
    Query(Query),
    Retrieve(Retrieve),

    // === Structured Values ===
    Tuple(TupleExpr),
    Instance(Instance),

    // === Quantity ===
    Quantity(QuantityExpr),
    Ratio(RatioExpr),
    ConvertQuantity(BinaryExpression),

    // === Code ===
    Code(CodeExpr),
    Concept(ConceptExpr),

    // === Message ===
    Message(Message),

    // === Fallback ===
    #[serde(other)]
    Unknown,
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Null(Null::default())
    }
}

/// Common fields for all expressions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type_name: Option<QName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type_specifier: Option<TypeSpecifier>,
}

/// A literal value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<QName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A null value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Null {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<QName>,
}

/// Nullary expression (no operands).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NullaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
}

/// Unary expression (one operand).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

/// Binary expression (two operands).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<Expression>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

/// Ternary expression (three operands).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TernaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<Expression>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

/// N-ary expression (variable operands).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<Expression>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

/// Aggregate expression.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

/// Typed expression (for min/max value).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<QName>,
}

/// Binary expression with precision (for date/time comparisons).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBinaryExpression {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

// === References ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operand: Vec<Expression>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<TypeSpecifier>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperandRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AliasRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLetRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptRef {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,
}

// === Property Access ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// === Control Flow ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<Expression>>,
    #[serde(rename = "then", skip_serializing_if = "Option::is_none")]
    pub then_expr: Option<Box<Expression>>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub else_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Case {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparand: Option<Box<Expression>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub case_item: Vec<CaseItem>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub else_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseItem {
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when_expr: Option<Box<Expression>>,
    #[serde(rename = "then", skip_serializing_if = "Option::is_none")]
    pub then_expr: Option<Box<Expression>>,
}

// === String Operations ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Combine {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_to_split: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitOnMatches {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_to_split: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_pattern: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionOf {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substring {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_to_sub: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Box<Expression>>,
}

// === Date/Time ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millisecond: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millisecond: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeComponentFrom {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
}

// === Interval ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntervalExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_closed_expression: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_closed_expression: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_closed: Option<bool>,
}

// === List ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_specifier: Option<TypeSpecifier>,
    #[serde(default, rename = "element", skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub by: Vec<SortByItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortByItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iteration {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slice {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repeat {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_expr: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// === Aggregate ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateClause {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
}

// === Type Operations ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_type_specifier: Option<TypeSpecifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_type: Option<QName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type_specifier: Option<TypeSpecifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type: Option<QName>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_type_specifier: Option<TypeSpecifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_type: Option<QName>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanConvert {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type_specifier: Option<TypeSpecifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_type: Option<QName>,
}

// === Clinical ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InValueSet {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueset: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueset_expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InCodeSystem {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codesystem: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codesystem_expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyInValueSet {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueset: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyInCodeSystem {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codesystem: Option<Box<Expression>>,
}

// === Query ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<AliasedQuerySource>,
    #[serde(rename = "let", default, skip_serializing_if = "Vec::is_empty")]
    pub let_clause: Vec<LetClause>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<RelationshipClause>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub where_clause: Option<Box<Expression>>,
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_clause: Option<ReturnClause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateClause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortClause>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AliasedQuerySource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LetClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipClause {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
    #[serde(rename = "suchThat", skip_serializing_if = "Option::is_none")]
    pub such_that: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortClause {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub by: Vec<SortByItem>,
}

// === Retrieve ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retrieve {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<QName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_property: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_comparator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_property: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<Box<Expression>>,
}

// === Structured Values ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, rename = "element", skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<TupleElement>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupleElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_type: Option<QName>,
    #[serde(default, rename = "element", skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<InstanceElement>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Expression>>,
}

// === Quantity ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatioExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerator: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denominator: Option<Box<Expression>>,
}

// === Code ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<CodeSystemRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptExpr {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<CodeExpr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

// === Message ===

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(flatten)]
    pub element: ElementFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Expression>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_serialization() {
        let literal = Expression::Literal(Literal {
            value_type: Some("{urn:hl7-org:elm-types:r1}Integer".into()),
            value: Some("42".into()),
            ..Default::default()
        });
        let json = serde_json::to_string(&literal).unwrap();
        assert!(json.contains("\"type\":\"Literal\""));
        assert!(json.contains("\"value\":\"42\""));
    }

    #[test]
    fn test_null_serialization() {
        let null = Expression::Null(Null::default());
        let json = serde_json::to_string(&null).unwrap();
        assert!(json.contains("\"type\":\"Null\""));
    }

    #[test]
    fn test_expression_ref_serialization() {
        let expr_ref = Expression::ExpressionRef(ExpressionRef {
            name: Some("MyExpression".into()),
            library_name: Some("Common".into()),
            ..Default::default()
        });
        let json = serde_json::to_string(&expr_ref).unwrap();
        assert!(json.contains("\"type\":\"ExpressionRef\""));
        assert!(json.contains("\"name\":\"MyExpression\""));
    }

    #[test]
    fn test_if_expression_serialization() {
        let if_expr = Expression::If(IfExpr {
            condition: Some(Box::new(Expression::Literal(Literal {
                value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".into()),
                value: Some("true".into()),
                ..Default::default()
            }))),
            then_expr: Some(Box::new(Expression::Literal(Literal {
                value: Some("1".into()),
                ..Default::default()
            }))),
            else_expr: Some(Box::new(Expression::Literal(Literal {
                value: Some("0".into()),
                ..Default::default()
            }))),
            ..Default::default()
        });
        let json = serde_json::to_string(&if_expr).unwrap();
        assert!(json.contains("\"type\":\"If\""));
        assert!(json.contains("\"then\""));
        assert!(json.contains("\"else\""));
    }

    #[test]
    fn test_query_serialization() {
        let query = Expression::Query(Query {
            source: vec![AliasedQuerySource {
                alias: Some("P".into()),
                expression: Some(Box::new(Expression::Retrieve(Retrieve {
                    data_type: Some("{http://hl7.org/fhir}Patient".into()),
                    ..Default::default()
                }))),
            }],
            ..Default::default()
        });
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"type\":\"Query\""));
        assert!(json.contains("\"alias\":\"P\""));
    }

    #[test]
    fn test_unknown_expression_type() {
        let json = r#"{"type":"SomeUnknownType","foo":"bar"}"#;
        let expr: Expression = serde_json::from_str(json).unwrap();
        assert!(matches!(expr, Expression::Unknown));
    }

    #[test]
    fn test_roundtrip() {
        let expr = Expression::And(NaryExpression {
            operand: vec![
                Expression::Literal(Literal {
                    value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".into()),
                    value: Some("true".into()),
                    ..Default::default()
                }),
                Expression::Literal(Literal {
                    value_type: Some("{urn:hl7-org:elm-types:r1}Boolean".into()),
                    value: Some("false".into()),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let json = serde_json::to_string(&expr).unwrap();
        let roundtrip: Expression = serde_json::from_str(&json).unwrap();
        assert_eq!(expr, roundtrip);
    }
}
