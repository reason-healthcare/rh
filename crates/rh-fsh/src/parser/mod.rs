//! FSH parser module

pub mod ast;
pub mod entity;
pub mod lexer;
pub mod rules;
pub mod span;

pub use ast::*;
pub use span::{SourceLocation, SourceRange, Span};

use crate::error::FshError;

/// High-level FSH parser
pub struct FshParser;

impl FshParser {
    /// Parse an FSH source string into a `FshDocument`
    pub fn parse(input: &str, source_name: impl Into<String>) -> Result<FshDocument, FshError> {
        let span = Span::new(input);
        entity::parse_document(span, source_name.into())
    }
}
