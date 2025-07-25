//! Abstract Syntax Tree (AST) types for FHIRPath expressions

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a complete FHIRPath expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FhirPathExpression {
    pub root: Expression,
}

/// Main expression types in FHIRPath
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Simple term expression
    Term(Term),
    /// Member invocation (e.g., Patient.name)
    Invocation {
        left: Box<Expression>,
        invocation: Invocation,
    },
    /// Indexer expression (e.g., Patient.name[0])
    Indexer {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    /// Polarity expression (e.g., -5, +10)
    Polarity {
        operator: PolarityOperator,
        operand: Box<Expression>,
    },
    /// Mathematical operations (*, /, div, mod)
    Multiplicative {
        left: Box<Expression>,
        operator: MultiplicativeOperator,
        right: Box<Expression>,
    },
    /// Additive operations (+, -, &)
    Additive {
        left: Box<Expression>,
        operator: AdditiveOperator,
        right: Box<Expression>,
    },
    /// Type operations (is, as)
    Type {
        left: Box<Expression>,
        operator: TypeOperator,
        type_specifier: TypeSpecifier,
    },
    /// Union operation (|)
    Union {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Inequality operations (<, <=, >, >=)
    Inequality {
        left: Box<Expression>,
        operator: InequalityOperator,
        right: Box<Expression>,
    },
    /// Equality operations (=, ~, !=, !~)
    Equality {
        left: Box<Expression>,
        operator: EqualityOperator,
        right: Box<Expression>,
    },
    /// Membership operations (in, contains)
    Membership {
        left: Box<Expression>,
        operator: MembershipOperator,
        right: Box<Expression>,
    },
    /// Logical AND
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Logical operations (or, xor)
    Or {
        left: Box<Expression>,
        operator: OrOperator,
        right: Box<Expression>,
    },
    /// Implies operation
    Implies {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

/// Term types in FHIRPath
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Term {
    /// Function or member invocation
    Invocation(Invocation),
    /// Literal value
    Literal(Literal),
    /// External constant (e.g., %context)
    ExternalConstant(String),
    /// Parenthesized expression
    Parenthesized(Box<Expression>),
}

/// Literal value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Null literal ({})
    Null,
    /// Boolean literal (true, false)
    Boolean(bool),
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Long number literal
    LongNumber(i64),
    /// Date literal
    Date(String),
    /// DateTime literal
    DateTime(String),
    /// Time literal
    Time(String),
    /// Quantity literal (number with unit)
    Quantity { value: f64, unit: Option<String> },
}

/// Invocation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Invocation {
    /// Member access by identifier
    Member(String),
    /// Function call
    Function {
        name: String,
        parameters: Vec<Expression>,
    },
    /// $this reference
    This,
    /// $index reference
    Index,
    /// $total reference
    Total,
}

/// Type specifier for type operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeSpecifier {
    pub qualified_name: Vec<String>,
}

/// Operators for various expression types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolarityOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MultiplicativeOperator {
    Multiply,
    Divide,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdditiveOperator {
    Add,
    Subtract,
    Concatenate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeOperator {
    Is,
    As,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InequalityOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EqualityOperator {
    Equal,
    Equivalent,
    NotEqual,
    NotEquivalent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MembershipOperator {
    In,
    Contains,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrOperator {
    Or,
    Xor,
}

impl fmt::Display for FhirPathExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Term(term) => write!(f, "{}", term),
            Expression::Invocation { left, invocation } => {
                write!(f, "{}.{}", left, invocation)
            }
            Expression::Indexer { left, index } => write!(f, "{}[{}]", left, index),
            Expression::Polarity { operator, operand } => {
                write!(f, "{}{}", operator, operand)
            }
            Expression::Multiplicative {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Additive {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Type {
                left,
                operator,
                type_specifier,
            } => write!(f, "{} {} {}", left, operator, type_specifier),
            Expression::Union { left, right } => write!(f, "{} | {}", left, right),
            Expression::Inequality {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Equality {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Membership {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::And { left, right } => write!(f, "{} and {}", left, right),
            Expression::Or {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::Implies { left, right } => write!(f, "{} implies {}", left, right),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Invocation(inv) => write!(f, "{}", inv),
            Term::Literal(lit) => write!(f, "{}", lit),
            Term::ExternalConstant(name) => write!(f, "%{}", name),
            Term::Parenthesized(expr) => write!(f, "({})", expr),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Null => write!(f, "{{}}"),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::String(s) => write!(f, "'{}'", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::LongNumber(n) => write!(f, "{}", n),
            Literal::Date(d) => write!(f, "{}", d),
            Literal::DateTime(dt) => write!(f, "{}", dt),
            Literal::Time(t) => write!(f, "{}", t),
            Literal::Quantity { value, unit } => {
                if let Some(unit) = unit {
                    write!(f, "{} {}", value, unit)
                } else {
                    write!(f, "{}", value)
                }
            }
        }
    }
}

impl fmt::Display for Invocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Invocation::Member(name) => write!(f, "{}", name),
            Invocation::Function { name, parameters } => {
                write!(f, "{}(", name)?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")")
            }
            Invocation::This => write!(f, "$this"),
            Invocation::Index => write!(f, "$index"),
            Invocation::Total => write!(f, "$total"),
        }
    }
}

impl fmt::Display for TypeSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.qualified_name.join("."))
    }
}

// Display implementations for operators
impl fmt::Display for PolarityOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolarityOperator::Plus => write!(f, "+"),
            PolarityOperator::Minus => write!(f, "-"),
        }
    }
}

impl fmt::Display for MultiplicativeOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiplicativeOperator::Multiply => write!(f, "*"),
            MultiplicativeOperator::Divide => write!(f, "/"),
            MultiplicativeOperator::Div => write!(f, "div"),
            MultiplicativeOperator::Mod => write!(f, "mod"),
        }
    }
}

impl fmt::Display for AdditiveOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdditiveOperator::Add => write!(f, "+"),
            AdditiveOperator::Subtract => write!(f, "-"),
            AdditiveOperator::Concatenate => write!(f, "&"),
        }
    }
}

impl fmt::Display for TypeOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeOperator::Is => write!(f, "is"),
            TypeOperator::As => write!(f, "as"),
        }
    }
}

impl fmt::Display for InequalityOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InequalityOperator::LessThan => write!(f, "<"),
            InequalityOperator::LessThanOrEqual => write!(f, "<="),
            InequalityOperator::GreaterThan => write!(f, ">"),
            InequalityOperator::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

impl fmt::Display for EqualityOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EqualityOperator::Equal => write!(f, "="),
            EqualityOperator::Equivalent => write!(f, "~"),
            EqualityOperator::NotEqual => write!(f, "!="),
            EqualityOperator::NotEquivalent => write!(f, "!~"),
        }
    }
}

impl fmt::Display for MembershipOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MembershipOperator::In => write!(f, "in"),
            MembershipOperator::Contains => write!(f, "contains"),
        }
    }
}

impl fmt::Display for OrOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrOperator::Or => write!(f, "or"),
            OrOperator::Xor => write!(f, "xor"),
        }
    }
}
