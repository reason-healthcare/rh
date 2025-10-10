//! Abstract Syntax Tree types for VCL expressions
//!
//! This module defines the AST types that represent parsed VCL expressions.

use serde::{Deserialize, Serialize};

/// Top-level VCL expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VclExpression {
    pub expr: Expression,
}

/// Main expression type that can have conjunctions, disjunctions, or exclusions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    pub sub_expr: SubExpression,
    pub operation: Option<Operation>,
}

/// Operations that can be applied to sub-expressions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    /// Conjunction (comma-separated): expr1, expr2, expr3
    Conjunction(Vec<SubExpression>),
    /// Disjunction (semicolon-separated): expr1; expr2; expr3
    Disjunction(Vec<SubExpression>),
    /// Exclusion (dash): expr1 - expr2
    Exclusion(SubExpression),
}

/// Sub-expression that can have an optional system URI and contains a simple expression or nested expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubExpression {
    pub system_uri: Option<SystemUri>,
    pub content: SubExpressionContent,
}

/// Content of a sub-expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubExpressionContent {
    /// Simple expression (code, filter, etc.)
    Simple(SimpleExpression),
    /// Nested expression in parentheses
    Nested(Box<Expression>),
}

/// Simple expression types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimpleExpression {
    /// Wildcard (*) - matches everything
    Wildcard,
    /// Specific code
    Code(Code),
    /// Filter expression
    Filter(Filter),
    /// Include ValueSet (^uri or ^(uri))
    IncludeValueSet(IncludeValueSet),
}

/// System URI wrapped in parentheses: (http://example.org/system)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemUri {
    pub uri: String,
}

/// Include ValueSet expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncludeValueSet {
    /// Direct URI: ^http://example.org/valueset
    Uri(String),
    /// System URI: ^(http://example.org/system)
    SystemUri(SystemUri),
}

/// Filter expressions for constraining codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    /// Property-based filter: property = code, property << code, etc.
    PropertyFilter {
        property: Code,
        operator: FilterOperator,
        value: FilterValue,
    },
    /// "Of" operation: (code|codeList|*|uri|filterList).property
    OfOperation { source: OfSource, property: Code },
}

/// Filter operators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperator {
    /// Equals (=)
    Equals,
    /// Is-a (<<)
    IsA,
    /// Is-not-a (~<<)
    IsNotA,
    /// Descendant-of (<)
    DescendantOf,
    /// Regular expression match (/)
    Regex,
    /// In (^)
    In,
    /// Not-in (~^)
    NotIn,
    /// Generalizes (>>)
    Generalizes,
    /// Child-of (<!)
    ChildOf,
    /// Descendant-leaf (!!<)
    DescendantLeaf,
    /// Exists (?)
    Exists,
}

/// Values that can be used in filter operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterValue {
    /// Single code
    Code(Code),
    /// String value (for regex)
    String(String),
    /// List of codes
    CodeList(Vec<Code>),
    /// URI reference
    Uri(String),
    /// Nested filter list
    FilterList(Vec<Filter>),
}

/// Source for "of" operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfSource {
    /// Single code
    Code(Code),
    /// List of codes
    CodeList(Vec<Code>),
    /// Wildcard
    Wildcard,
    /// URI reference
    Uri(String),
    /// Nested filter list
    FilterList(Vec<Filter>),
}

/// Code representation - can be simple or quoted
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Code {
    /// Simple code: alphanumeric with hyphens and underscores
    Simple(String),
    /// Quoted value: "string with spaces and special chars"
    Quoted(String),
}

impl Code {
    /// Get the string value of the code, removing quotes if present
    pub fn value(&self) -> &str {
        match self {
            Code::Simple(s) => s,
            Code::Quoted(s) => s,
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::Simple(s) => write!(f, "{s}"),
            Code::Quoted(s) => write!(f, "\"{s}\""),
        }
    }
}

impl VclExpression {
    /// Check if this expression contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        self.expr.contains_wildcards()
    }

    /// Get all system URIs referenced in this expression
    pub fn system_uris(&self) -> Vec<&str> {
        self.expr.system_uris()
    }

    /// Get all codes referenced in this expression
    pub fn codes(&self) -> Vec<&Code> {
        self.expr.codes()
    }
}

impl Expression {
    /// Check if this expression contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        self.sub_expr.contains_wildcards()
            || self
                .operation
                .as_ref()
                .is_some_and(|op| op.contains_wildcards())
    }

    /// Get all system URIs referenced in this expression
    pub fn system_uris(&self) -> Vec<&str> {
        let mut uris = self.sub_expr.system_uris();
        if let Some(op) = &self.operation {
            uris.extend(op.system_uris());
        }
        uris
    }

    /// Get all codes referenced in this expression
    pub fn codes(&self) -> Vec<&Code> {
        let mut codes = self.sub_expr.codes();
        if let Some(op) = &self.operation {
            codes.extend(op.codes());
        }
        codes
    }
}

impl Operation {
    /// Check if this operation contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        match self {
            Operation::Conjunction(exprs) | Operation::Disjunction(exprs) => {
                exprs.iter().any(|e| e.contains_wildcards())
            }
            Operation::Exclusion(expr) => expr.contains_wildcards(),
        }
    }

    /// Get all system URIs referenced in this operation
    pub fn system_uris(&self) -> Vec<&str> {
        match self {
            Operation::Conjunction(exprs) | Operation::Disjunction(exprs) => {
                exprs.iter().flat_map(|e| e.system_uris()).collect()
            }
            Operation::Exclusion(expr) => expr.system_uris(),
        }
    }

    /// Get all codes referenced in this operation
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            Operation::Conjunction(exprs) | Operation::Disjunction(exprs) => {
                exprs.iter().flat_map(|e| e.codes()).collect()
            }
            Operation::Exclusion(expr) => expr.codes(),
        }
    }
}

impl SubExpression {
    /// Check if this sub-expression contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        self.content.contains_wildcards()
    }

    /// Get all system URIs referenced in this sub-expression
    pub fn system_uris(&self) -> Vec<&str> {
        let mut uris = Vec::new();
        if let Some(system_uri) = &self.system_uri {
            uris.push(system_uri.uri.as_str());
        }
        uris.extend(self.content.system_uris());
        uris
    }

    /// Get all codes referenced in this sub-expression
    pub fn codes(&self) -> Vec<&Code> {
        self.content.codes()
    }
}

impl SubExpressionContent {
    /// Check if this content contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        match self {
            SubExpressionContent::Simple(expr) => expr.contains_wildcards(),
            SubExpressionContent::Nested(expr) => expr.contains_wildcards(),
        }
    }

    /// Get all system URIs referenced in this content
    pub fn system_uris(&self) -> Vec<&str> {
        match self {
            SubExpressionContent::Simple(expr) => expr.system_uris(),
            SubExpressionContent::Nested(expr) => expr.system_uris(),
        }
    }

    /// Get all codes referenced in this content
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            SubExpressionContent::Simple(expr) => expr.codes(),
            SubExpressionContent::Nested(expr) => expr.codes(),
        }
    }
}

impl SimpleExpression {
    /// Check if this simple expression contains any wildcards
    pub fn contains_wildcards(&self) -> bool {
        matches!(self, SimpleExpression::Wildcard)
    }

    /// Get all system URIs referenced in this simple expression
    pub fn system_uris(&self) -> Vec<&str> {
        match self {
            SimpleExpression::IncludeValueSet(vs) => vs.system_uris(),
            _ => Vec::new(),
        }
    }

    /// Get all codes referenced in this simple expression
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            SimpleExpression::Code(code) => vec![code],
            SimpleExpression::Filter(filter) => filter.codes(),
            _ => Vec::new(),
        }
    }
}

impl IncludeValueSet {
    /// Get all system URIs referenced in this include valueset
    pub fn system_uris(&self) -> Vec<&str> {
        match self {
            IncludeValueSet::Uri(uri) => vec![uri.as_str()],
            IncludeValueSet::SystemUri(system_uri) => vec![system_uri.uri.as_str()],
        }
    }
}

impl Filter {
    /// Get all codes referenced in this filter
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            Filter::PropertyFilter {
                property, value, ..
            } => {
                let mut codes = vec![property];
                codes.extend(value.codes());
                codes
            }
            Filter::OfOperation { source, property } => {
                let mut codes = vec![property];
                codes.extend(source.codes());
                codes
            }
        }
    }
}

impl FilterValue {
    /// Get all codes referenced in this filter value
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            FilterValue::Code(code) => vec![code],
            FilterValue::CodeList(codes) => codes.iter().collect(),
            FilterValue::FilterList(filters) => filters.iter().flat_map(|f| f.codes()).collect(),
            _ => Vec::new(),
        }
    }
}

impl OfSource {
    /// Get all codes referenced in this of source
    pub fn codes(&self) -> Vec<&Code> {
        match self {
            OfSource::Code(code) => vec![code],
            OfSource::CodeList(codes) => codes.iter().collect(),
            OfSource::FilterList(filters) => filters.iter().flat_map(|f| f.codes()).collect(),
            _ => Vec::new(),
        }
    }
}
