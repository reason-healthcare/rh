//! CQL Parser Module
//!
//! This module provides a parser for CQL (Clinical Quality Language) using
//! `nom` parser combinators, following the same patterns as `rh-fhirpath`.
//!
//! ## Architecture
//!
//! The parser is organized into several submodules:
//!
//! - [`lexer`]: Tokenization utilities (keywords, operators, whitespace, comments)
//! - [`span`]: Source location tracking for error reporting
//! - [`ast`]: CQL Abstract Syntax Tree types (separate from ELM output)
//! - [`expression`]: Expression parsing with operator precedence
//! - [`statement`]: Library header and definition parsing
//!
//! ## CQL Grammar Reference
//!
//! Based on CQL version 1.5.3 specification. The grammar is translated from
//! the official ANTLR4 grammar to nom parser combinators.

pub mod ast;
pub mod expression;
pub mod lexer;
pub mod span;
pub mod statement;

use crate::error::{CqlError, Result};
use span::Span;

/// CQL Parser
///
/// Parses CQL source code into an Abstract Syntax Tree (AST).
///
/// # Example
///
/// ```
/// use rh_cql::parser::CqlParser;
///
/// let parser = CqlParser::new();
/// let library = parser.parse(r#"
///     library Example version '1.0.0'
///     using FHIR version '4.0.1'
///     context Patient
///     define InPatient: true
/// "#).unwrap();
///
/// assert_eq!(library.identifier.unwrap().name, "Example");
/// ```
#[derive(Debug, Clone)]
pub struct CqlParser {
    /// Track source locations for error reporting
    #[allow(dead_code)]
    track_locations: bool,
}

impl Default for CqlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CqlParser {
    /// Create a new CQL parser
    pub fn new() -> Self {
        Self {
            track_locations: true,
        }
    }

    /// Create a parser without source location tracking (faster)
    pub fn without_locations() -> Self {
        Self {
            track_locations: false,
        }
    }

    /// Parse CQL source code into a Library AST
    ///
    /// # Arguments
    ///
    /// * `source` - The CQL source code to parse
    ///
    /// # Returns
    ///
    /// The parsed Library AST, or an error if parsing fails
    ///
    /// # Example
    ///
    /// ```
    /// use rh_cql::parser::CqlParser;
    ///
    /// let parser = CqlParser::new();
    /// let library = parser.parse("library Test define Foo: 42").unwrap();
    /// assert_eq!(library.statements.len(), 1);
    /// ```
    pub fn parse(&self, source: &str) -> Result<ast::Library> {
        let span = Span::new(source);
        self.parse_library(span)
    }

    /// Parse a library from a span
    fn parse_library(&self, span: Span<'_>) -> Result<ast::Library> {
        match statement::parse_library(span) {
            Ok((remaining, library)) => {
                let remaining_str = remaining.fragment().trim();
                if remaining_str.is_empty() {
                    Ok(library)
                } else {
                    let loc = remaining.location();
                    Err(CqlError::ParseError {
                        message: format!(
                            "Unexpected content after library: {}",
                            &remaining_str[..remaining_str.len().min(50)]
                        ),
                        line: loc.line,
                        column: loc.column,
                    })
                }
            }
            Err(e) => Err(CqlError::ParseError {
                message: format!("Parse error: {e:?}"),
                line: 1,
                column: 1,
            }),
        }
    }

    /// Parse a single expression (useful for testing)
    ///
    /// # Example
    ///
    /// ```
    /// use rh_cql::parser::CqlParser;
    /// use rh_cql::parser::ast::Expression;
    ///
    /// let parser = CqlParser::new();
    /// let expr = parser.parse_expression("1 + 2 * 3").unwrap();
    /// // Parses as 1 + (2 * 3) due to precedence
    /// ```
    pub fn parse_expression(&self, source: &str) -> Result<ast::Expression> {
        let span = Span::new(source);
        self.parse_expr(span)
    }

    /// Parse an expression from a span
    fn parse_expr(&self, span: Span<'_>) -> Result<ast::Expression> {
        match expression::expression(span) {
            Ok((remaining, expr)) => {
                let remaining_str = remaining.fragment().trim();
                if remaining_str.is_empty() {
                    Ok(expr)
                } else {
                    let loc = remaining.location();
                    Err(CqlError::ParseError {
                        message: format!(
                            "Unexpected content after expression: {}",
                            &remaining_str[..remaining_str.len().min(50)]
                        ),
                        line: loc.line,
                        column: loc.column,
                    })
                }
            }
            Err(e) => Err(CqlError::ParseError {
                message: format!("Parse error: {e:?}"),
                line: 1,
                column: 1,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{BinaryOperator, Expression, Literal, Statement};

    #[test]
    fn test_parser_creation() {
        let parser = CqlParser::new();
        assert!(parser.track_locations);

        let parser = CqlParser::without_locations();
        assert!(!parser.track_locations);
    }

    #[test]
    fn test_parser_default() {
        let parser = CqlParser::default();
        assert!(parser.track_locations);
    }

    #[test]
    fn test_parse_simple_expression() {
        let parser = CqlParser::new();
        let expr = parser.parse_expression("42").unwrap();
        assert!(matches!(expr, Expression::Literal(Literal::Integer(42))));
    }

    #[test]
    fn test_parse_binary_expression() {
        let parser = CqlParser::new();
        let expr = parser.parse_expression("1 + 2").unwrap();
        assert!(matches!(
            expr,
            Expression::BinaryExpression(ast::BinaryExpression {
                operator: BinaryOperator::Add,
                ..
            })
        ));
    }

    #[test]
    fn test_parse_simple_library() {
        let parser = CqlParser::new();
        let library = parser
            .parse(
                r#"
            library Test version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define InPatient: true
        "#,
            )
            .unwrap();

        assert!(library.identifier.is_some());
        assert_eq!(library.identifier.as_ref().unwrap().name, "Test");
        assert_eq!(library.usings.len(), 1);
        assert_eq!(library.usings[0].model_name, "FHIR");
        assert_eq!(library.contexts.len(), 1);
        assert_eq!(library.statements.len(), 1);
    }

    #[test]
    fn test_parse_library_with_arithmetic() {
        let parser = CqlParser::new();
        let library = parser
            .parse(
                r#"
            library Math
            define Sum: 1 + 2 + 3
            define Product: 2 * 3 * 4
        "#,
            )
            .unwrap();

        assert_eq!(library.statements.len(), 2);
    }

    #[test]
    fn test_parse_library_with_function() {
        let parser = CqlParser::new();
        let library = parser
            .parse(
                r#"
            library Functions
            define function Double(x Integer) returns Integer: x * 2
        "#,
            )
            .unwrap();

        assert_eq!(library.statements.len(), 1);
        match &library.statements[0] {
            Statement::FunctionDef(f) => {
                assert_eq!(f.name, "Double");
                assert_eq!(f.parameters.len(), 1);
            }
            _ => panic!("Expected function definition"),
        }
    }

    #[test]
    fn test_parse_expression_with_trailing_error() {
        let parser = CqlParser::new();
        let result = parser.parse_expression("42 garbage");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complex_expression() {
        let parser = CqlParser::new();

        // Test precedence: should parse as (a or (b and c))
        let expr = parser.parse_expression("a or b and c").unwrap();
        if let Expression::BinaryExpression(bin) = expr {
            assert_eq!(bin.operator, BinaryOperator::Or);
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_parse_if_then_else() {
        let parser = CqlParser::new();
        let expr = parser
            .parse_expression("if x > 0 then 'positive' else 'non-positive'")
            .unwrap();
        assert!(matches!(expr, Expression::IfThenElse(_)));
    }

    #[test]
    fn test_parse_list_literal() {
        let parser = CqlParser::new();
        let expr = parser.parse_expression("{ 1, 2, 3 }").unwrap();
        if let Expression::ListExpression(list) = expr {
            assert_eq!(list.elements.len(), 3);
        } else {
            panic!("Expected list expression");
        }
    }
}
