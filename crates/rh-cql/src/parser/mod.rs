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
//! [`CqlParser::parse`] and [`statement::parse_library`] are complete-library
//! entry points and require the input to end after permitted trailing trivia.
//! [`CqlParser::parse_expression`] similarly requires a complete expression.
//! Internal expression and declaration parsers return their unconsumed
//! [`Span`] so callers can compose them into larger grammar productions.
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
        parse_complete(span, "library", statement::parse_library)
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
        parse_complete(span, "expression", expression::expression)
    }
}

fn parse_complete<'a, T>(
    span: Span<'a>,
    production: &str,
    parser: impl FnOnce(Span<'a>) -> nom::IResult<Span<'a>, T>,
) -> Result<T> {
    let fallback = span.location();
    match parser(span) {
        Ok((remaining, parsed)) => {
            let trailing = remaining.fragment().trim();
            if trailing.is_empty() {
                Ok(parsed)
            } else {
                let location = remaining.location();
                let excerpt: String = trailing.chars().take(50).collect();
                Err(CqlError::ParseError {
                    message: format!("Unexpected content after {production}: {excerpt}"),
                    line: location.line,
                    column: location.column,
                })
            }
        }
        Err(error) => Err(located_parse_error(error, fallback)),
    }
}

fn located_parse_error(
    error: nom::Err<nom::error::Error<Span<'_>>>,
    fallback: span::SourceLocation,
) -> CqlError {
    let message = format!("Parse error: {error:?}");
    let location = match &error {
        nom::Err::Error(error) | nom::Err::Failure(error) => error.input.location(),
        nom::Err::Incomplete(_) => fallback,
    };
    CqlError::ParseError {
        message,
        line: location.line,
        column: location.column,
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

    fn assert_parse_error_location(source: &str, expected_line: usize, expected_column: usize) {
        match CqlParser::new().parse(source) {
            Err(CqlError::ParseError { line, column, .. }) => {
                assert_eq!((line, column), (expected_line, expected_column));
            }
            other => panic!("expected located parse error, got {other:?}"),
        }
    }

    #[test]
    fn test_malformed_temporal_direction_reports_phrase_location() {
        assert_parse_error_location(
            "library Bad\ndefine X: @2024-01-01 on or sideways @2024-02-01",
            2,
            29,
        );
    }

    #[test]
    fn test_nested_right_operand_reports_deepest_location() {
        assert_parse_error_location("library Bad\ndefine X: true and (false or )", 2, 30);
    }

    #[test]
    fn test_invalid_declaration_reports_after_starter() {
        assert_parse_error_location("library Bad\ndefine X 1 + 2", 2, 10);
    }

    #[test]
    fn test_unexpected_trailing_construct_reports_its_location() {
        assert_parse_error_location("library Bad\ndefine X: 1\nbogus", 3, 1);
    }

    #[test]
    fn test_delimiter_sensitive_expressions_remain_valid() {
        let parser = CqlParser::new();

        assert!(matches!(
            parser.parse_expression("3 between 1 and 5").unwrap(),
            Expression::TernaryExpression(_)
        ));
        assert!(matches!(
            parser
                .parse_expression("from { 1, 2 } N where N > 1 return N")
                .unwrap(),
            Expression::Query(_)
        ));
        assert!(matches!(
            parser.parse_expression("{ 1, 2, 3 }").unwrap(),
            Expression::ListExpression(ref list) if list.elements.len() == 3
        ));
        assert!(matches!(
            parser
                .parse_expression("Tuple { first: 1, second: 2 }")
                .unwrap(),
            Expression::TupleExpression(ref tuple) if tuple.elements.len() == 2
        ));
    }

    #[test]
    fn test_alternate_statement_productions_remain_valid() {
        let library = CqlParser::new()
            .parse(
                "library Controls\n\
                 define Value: 1\n\
                 define function Identity(value Integer): value",
            )
            .unwrap();

        assert!(matches!(library.statements[0], Statement::ExpressionDef(_)));
        assert!(matches!(library.statements[1], Statement::FunctionDef(_)));
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

    // -----------------------------------------------------------------------
    // Wave-2 parser regression tests (task 2.1)
    // -----------------------------------------------------------------------

    /// `size of <expr>` parses as UnaryExpression(Size, operand) — not an error.
    #[test]
    fn test_parse_size_of_interval() {
        use ast::UnaryOperator;
        let parser = CqlParser::new();
        let expr = parser
            .parse_expression("size of Interval[1, 5]")
            .expect("`size of Interval[1, 5]` must parse without error");
        match expr {
            Expression::UnaryExpression(u) => {
                assert_eq!(u.operator, UnaryOperator::Size);
            }
            other => panic!("Expected UnaryExpression(Size), got {other:?}"),
        }
    }

    /// Neighboring interval unary phrases do not regress after the `size of` addition.
    #[test]
    fn test_parse_interval_unary_phrases_no_regression() {
        use ast::UnaryOperator;
        let parser = CqlParser::new();

        let start = parser
            .parse_expression("start of Interval[1, 5]")
            .expect("`start of` must parse");
        assert!(matches!(
            start,
            Expression::UnaryExpression(ref u) if u.operator == UnaryOperator::Start
        ));

        let end = parser
            .parse_expression("end of Interval[1, 5]")
            .expect("`end of` must parse");
        assert!(matches!(
            end,
            Expression::UnaryExpression(ref u) if u.operator == UnaryOperator::End
        ));
    }
}
