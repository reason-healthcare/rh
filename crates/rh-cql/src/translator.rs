//! Expression Translator for CQL-to-ELM conversion.
//!
//! DEPRECATED -- This module is superseded by the three-stage pipeline:
//! [`crate::semantics::analyzer::SemanticAnalyzer`] (parse to analyze) and
//! [`crate::emit::ElmEmitter`] (typed AST to ELM). All types in this module
//! are deprecated and will be removed in a future release.
//!
//! For new code, use [`crate::compile`] or wire `SemanticAnalyzer + ElmEmitter`
//! directly. See `src/emit/` for the replacement implementation.

// Allow internal references to deprecated items within this shim.
#![allow(deprecated)]

use crate::elm;

// ============================================================================
// Translator Error
// ============================================================================

/// Errors that can occur during expression translation.
#[deprecated(since = "0.1.0", note = "Use SemanticAnalyzer + ElmEmitter instead.")]
#[derive(Debug, Clone, PartialEq)]
pub enum TranslatorError {
    /// Unsupported expression type.
    UnsupportedExpression { description: String },
    /// Type error during translation.
    TypeError { message: String },
    /// Invalid literal value.
    InvalidLiteral { message: String },
}

#[allow(deprecated)]
impl std::fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslatorError::UnsupportedExpression { description } => {
                write!(f, "Unsupported expression: {description}")
            }
            TranslatorError::TypeError { message } => write!(f, "Type error: {message}"),
            TranslatorError::InvalidLiteral { message } => {
                write!(f, "Invalid literal: {message}")
            }
        }
    }
}

#[allow(deprecated)]
impl std::error::Error for TranslatorError {}

/// Result type for translator operations.
#[deprecated(since = "0.1.0", note = "Use SemanticAnalyzer + ElmEmitter instead.")]
pub type TranslatorResult<T> = Result<T, TranslatorError>;

// ============================================================================
// Reference Resolution Types
// ============================================================================

/// The kind of reference an identifier resolves to.
#[deprecated(since = "0.1.0", note = "Use semantics::typed_ast types instead.")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedRefKind {
    /// An expression definition.
    Expression,
    /// A parameter definition.
    Parameter,
    /// A function operand.
    Operand,
    /// A query alias.
    QueryAlias,
    /// A let binding.
    Let,
    /// A code system definition.
    CodeSystem,
    /// A value set definition.
    ValueSet,
    /// A code definition.
    Code,
    /// A concept definition.
    Concept,
    /// A context variable (e.g., Patient).
    Context,
    /// Unknown/unresolved reference.
    Unknown,
}

/// The kind of a qualified reference.
#[deprecated(since = "0.1.0", note = "Use semantics::typed_ast types instead.")]
#[derive(Debug, Clone, PartialEq)]
pub enum QualifiedRefKind {
    /// Expression from an included library.
    LibraryExpression,
    /// Parameter from an included library.
    LibraryParameter,
    /// Code system from an included library.
    LibraryCodeSystem,
    /// Value set from an included library.
    LibraryValueSet,
    /// Code from an included library.
    LibraryCode,
    /// Concept from an included library.
    LibraryConcept,
    /// Property access on a source expression.
    Property {
        /// The source expression (if not a simple alias scope).
        source: Option<Box<elm::Expression>>,
    },
    /// Unknown/unresolved reference.
    Unknown,
}

// ============================================================================
// N-ary Operator Enum
// ============================================================================

/// Operators that support multiple operands (n-ary).
#[deprecated(since = "0.1.0", note = "No replacement needed outside the old translator.")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaryOperator {
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
    /// String concatenation.
    Concatenate,
    /// Null coalescing.
    Coalesce,
}

// ============================================================================
// Statement Translation
// ============================================================================

/// The result of translating a CQL statement.
///
/// Use [`crate::semantics::typed_ast::TypedStatement`] instead.
#[deprecated(
    since = "0.1.0",
    note = "Use TypedStatement from the new semantics pipeline instead."
)]
#[derive(Debug, Clone)]
pub enum StatementTranslation {
    /// An expression definition.
    Expression(elm::ExpressionDef),
    /// A function definition.
    Function(elm::FunctionDef),
}

// ============================================================================
// Expression Translator (stub shim -- all translation logic removed)
// ============================================================================

/// Deprecated expression-level translator shim.
///
/// This struct is retained only so that code referencing `ExpressionTranslator`
/// by type continues to compile after the old translation pipeline was removed.
/// Migrate to the new three-stage pipeline:
///
/// ```
/// use rh_cql::{compile, CompilerOptions};
/// let result = compile("library T version '1' define X: 1 + 1", None).unwrap();
/// assert!(result.is_success());
/// ```
#[deprecated(
    since = "0.1.0",
    note = "Use the new three-stage pipeline: SemanticAnalyzer + ElmEmitter. \
            See the `emit` module for replacement functionality."
)]
#[derive(Debug, Default)]
pub struct ExpressionTranslator {
    local_id_counter: u32,
    options: std::sync::Arc<crate::options::CompilerOptions>,
}

#[allow(deprecated)]
impl ExpressionTranslator {
    /// Create a new expression translator with default options.
    pub fn new() -> Self {
        Self {
            local_id_counter: 0,
            options: std::sync::Arc::new(crate::options::CompilerOptions::default()),
        }
    }

    /// Create a new expression translator with specific options.
    pub fn with_options(options: crate::options::CompilerOptions) -> Self {
        Self {
            local_id_counter: 0,
            options: std::sync::Arc::new(options),
        }
    }

    /// Create a new expression translator with shared options.
    pub fn with_shared_options(options: std::sync::Arc<crate::options::CompilerOptions>) -> Self {
        Self {
            local_id_counter: 0,
            options,
        }
    }

    /// Enable local ID generation (no-op in this shim).
    pub fn with_local_ids(self) -> Self {
        self
    }

    /// Get the current compiler options.
    pub fn options(&self) -> &crate::options::CompilerOptions {
        &self.options
    }

    /// Generate the next local ID if annotations are enabled.
    pub fn next_local_id(&mut self) -> Option<String> {
        if self.options.annotations_enabled() {
            self.local_id_counter += 1;
            Some(self.local_id_counter.to_string())
        } else {
            None
        }
    }

    /// Build a minimal `ElementFields` value.
    pub fn element_fields(&mut self) -> elm::ElementFields {
        let mut f = elm::ElementFields::default();
        f.local_id = self.next_local_id();
        f
    }
}
