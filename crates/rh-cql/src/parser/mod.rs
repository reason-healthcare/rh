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
//!
//! ## CQL Grammar Reference
//!
//! Based on CQL version 1.5.3 specification. The grammar is translated from
//! the official ANTLR4 grammar to nom parser combinators.

pub mod ast;
pub mod lexer;
pub mod span;

use crate::error::{CqlError, Result};
use span::Span;

/// CQL Parser
///
/// Parses CQL source code into an Abstract Syntax Tree (AST).
///
/// # Example
///
/// ```ignore
/// use rh_cql::parser::CqlParser;
///
/// let parser = CqlParser::new();
/// let library = parser.parse(r#"
///     library Example version '1.0.0'
///     define Foo: 42
/// "#)?;
/// ```
#[derive(Debug, Clone)]
pub struct CqlParser {
    /// Track source locations for error reporting
    #[allow(dead_code)] // Will be used in Phase 2.4
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
    pub fn parse(&self, source: &str) -> Result<ast::Library> {
        let span = Span::new(source);
        self.parse_library(span)
    }

    /// Parse a library from a span
    fn parse_library(&self, _span: Span<'_>) -> Result<ast::Library> {
        // TODO: Implement full library parsing in Phase 2.4
        Err(CqlError::ParseError {
            message: "Library parsing not yet implemented".to_string(),
            line: 1,
            column: 1,
        })
    }

    /// Parse a single expression (useful for testing)
    pub fn parse_expression(&self, source: &str) -> Result<ast::Expression> {
        let span = Span::new(source);
        self.parse_expr(span)
    }

    /// Parse an expression from a span
    fn parse_expr(&self, _span: Span<'_>) -> Result<ast::Expression> {
        // TODO: Implement expression parsing in Phase 2.4
        Err(CqlError::ParseError {
            message: "Expression parsing not yet implemented".to_string(),
            line: 1,
            column: 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
