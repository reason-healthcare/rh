//! CQL Library Builder for semantic analysis and ELM generation.
//!
//! The `LibraryBuilder` is the core engine for translating CQL AST to typed ELM.
//! It manages:
//! - **Context**: The current evaluation context (Patient, Practitioner, etc.)
//! - **Scopes**: Nested symbol tables for identifier resolution
//! - **Type Resolution**: Mapping type specifiers to internal DataTypes
//! - **Library Dependencies**: Access to included libraries and their definitions
//!
//! # Overview
//!
//! The builder operates in phases:
//! 1. **Setup**: Initialize with library info, model provider, and dependencies
//! 2. **Context Setting**: Establish the evaluation context
//! 3. **Definition Processing**: Translate each definition to ELM
//! 4. **Finalization**: Produce the complete ELM Library
//!
//! # Example
//!
//! ```
//! use rh_cql::builder::{LibraryBuilder, Scope, Symbol, SymbolKind};
//! use rh_cql::datatype::{DataType, SystemType};
//!
//! // Create a builder
//! let mut builder = LibraryBuilder::new();
//!
//! // Set the library name
//! builder.set_library_name("MyLibrary");
//! builder.set_library_version("1.0.0");
//!
//! // Set the context
//! builder.set_context("Patient");
//! assert_eq!(builder.context(), Some("Patient"));
//!
//! // Add a symbol to the current scope
//! let symbol = Symbol::new("Age", SymbolKind::Expression)
//!     .with_type(DataType::system(SystemType::Integer));
//! builder.define_symbol(symbol);
//!
//! // Resolve the symbol
//! let resolved = builder.resolve_identifier("Age");
//! assert!(resolved.is_some());
//! ```

use std::collections::HashMap;

use crate::datatype::DataType;
use crate::elm;
use crate::library::{CompiledLibrary, LibraryIdentifier};
use crate::parser::ast;
use crate::preprocessor::LibraryInfo;
use crate::provider::ModelInfoProvider;

// ============================================================================
// Symbol and Scope
// ============================================================================

/// The kind of symbol in a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// An expression definition.
    Expression,
    /// A function definition.
    Function,
    /// A parameter definition.
    Parameter,
    /// A code system definition.
    CodeSystem,
    /// A value set definition.
    ValueSet,
    /// A code definition.
    Code,
    /// A concept definition.
    Concept,
    /// A query alias (from query sources).
    QueryAlias,
    /// A let binding (within expressions).
    Let,
    /// A function operand/parameter.
    Operand,
    /// An included library alias.
    Library,
    /// A context variable (e.g., Patient).
    Context,
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Expression => write!(f, "expression"),
            SymbolKind::Function => write!(f, "function"),
            SymbolKind::Parameter => write!(f, "parameter"),
            SymbolKind::CodeSystem => write!(f, "codesystem"),
            SymbolKind::ValueSet => write!(f, "valueset"),
            SymbolKind::Code => write!(f, "code"),
            SymbolKind::Concept => write!(f, "concept"),
            SymbolKind::QueryAlias => write!(f, "alias"),
            SymbolKind::Let => write!(f, "let"),
            SymbolKind::Operand => write!(f, "operand"),
            SymbolKind::Library => write!(f, "library"),
            SymbolKind::Context => write!(f, "context"),
        }
    }
}

/// A symbol in the symbol table.
///
/// Symbols represent named entities that can be referenced in expressions:
/// - Definitions (expressions, functions, parameters)
/// - Terminology (code systems, value sets, codes, concepts)
/// - Query aliases and let bindings
/// - Included library references
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The symbol name.
    pub name: String,
    /// The kind of symbol.
    pub kind: SymbolKind,
    /// The resolved type (if known).
    pub result_type: Option<DataType>,
    /// The library containing this symbol (None for local symbols).
    pub library: Option<LibraryIdentifier>,
    /// Whether this symbol is public.
    pub is_public: bool,
}

impl Symbol {
    /// Create a new symbol with the given name and kind.
    pub fn new(name: impl Into<String>, kind: SymbolKind) -> Self {
        Self {
            name: name.into(),
            kind,
            result_type: None,
            library: None,
            is_public: true,
        }
    }

    /// Set the result type of this symbol.
    pub fn with_type(mut self, result_type: DataType) -> Self {
        self.result_type = Some(result_type);
        self
    }

    /// Set the library this symbol comes from.
    pub fn with_library(mut self, library: LibraryIdentifier) -> Self {
        self.library = Some(library);
        self
    }

    /// Set the access level of this symbol.
    pub fn with_access(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }

    /// Check if this symbol is local (not from an included library).
    pub fn is_local(&self) -> bool {
        self.library.is_none()
    }
}

/// A scope containing symbols.
///
/// Scopes are nested - inner scopes can shadow outer scopes.
/// The builder maintains a stack of scopes for:
/// - Library-level definitions
/// - Query sources (aliases)
/// - Let bindings
/// - Function parameters
#[derive(Debug, Clone, Default)]
pub struct Scope {
    /// Symbols in this scope, keyed by name.
    symbols: HashMap<String, Symbol>,
    /// Overloaded function signatures, keyed by name.
    /// Each entry contains all functions with that name (for overload resolution).
    functions: HashMap<String, Vec<FunctionSignature>>,
}

impl Scope {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a symbol in this scope.
    ///
    /// Returns the previous symbol with the same name, if any.
    pub fn define(&mut self, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(symbol.name.clone(), symbol)
    }

    /// Define a function signature (for overload resolution).
    pub fn define_function(&mut self, signature: FunctionSignature) {
        self.functions
            .entry(signature.name.clone())
            .or_default()
            .push(signature);
    }

    /// Look up a symbol by name.
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Look up function signatures by name.
    pub fn get_functions(&self, name: &str) -> Option<&[FunctionSignature]> {
        self.functions.get(name).map(|v| v.as_slice())
    }

    /// Check if a symbol exists in this scope.
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Get all symbol names in this scope.
    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.symbols.keys().map(|s| s.as_str())
    }

    /// Get all symbols in this scope.
    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }

    /// Get the number of symbols in this scope.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Check if the scope is empty.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

/// A function signature for overload resolution.
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// The function name.
    pub name: String,
    /// Parameter types.
    pub operand_types: Vec<DataType>,
    /// Return type.
    pub result_type: DataType,
    /// Whether this is a fluent function.
    pub is_fluent: bool,
    /// Whether this is an external function.
    pub is_external: bool,
    /// The library containing this function (None for local).
    pub library: Option<LibraryIdentifier>,
}

impl FunctionSignature {
    /// Create a new function signature.
    pub fn new(name: impl Into<String>, result_type: DataType) -> Self {
        Self {
            name: name.into(),
            operand_types: Vec::new(),
            result_type,
            is_fluent: false,
            is_external: false,
            library: None,
        }
    }

    /// Add parameter types.
    pub fn with_operands(mut self, operand_types: Vec<DataType>) -> Self {
        self.operand_types = operand_types;
        self
    }

    /// Set as fluent function.
    pub fn fluent(mut self) -> Self {
        self.is_fluent = true;
        self
    }

    /// Set as external function.
    pub fn external(mut self) -> Self {
        self.is_external = true;
        self
    }

    /// Set the library this function comes from.
    pub fn with_library(mut self, library: LibraryIdentifier) -> Self {
        self.library = Some(library);
        self
    }

    /// Get the arity (number of parameters).
    pub fn arity(&self) -> usize {
        self.operand_types.len()
    }
}

// ============================================================================
// Resolution Result
// ============================================================================

/// The result of resolving an identifier.
#[derive(Debug, Clone)]
pub struct ResolvedIdentifier {
    /// The symbol that was resolved.
    pub symbol: Symbol,
    /// The scope depth where the symbol was found (0 = current scope).
    pub scope_depth: usize,
}

impl ResolvedIdentifier {
    /// Check if this is a local resolution (current library).
    pub fn is_local(&self) -> bool {
        self.symbol.is_local()
    }

    /// Get the library qualifier if this is from an included library.
    pub fn library_name(&self) -> Option<&str> {
        self.symbol.library.as_ref().map(|id| id.name.as_str())
    }
}

// ============================================================================
// Builder Error
// ============================================================================

/// Errors that can occur during library building.
#[derive(Debug, Clone, PartialEq)]
pub enum BuilderError {
    /// An identifier could not be resolved.
    UnresolvedIdentifier {
        name: String,
        context: Option<String>,
    },
    /// A qualified identifier could not be resolved.
    UnresolvedQualifiedIdentifier { qualifier: String, name: String },
    /// Duplicate symbol definition.
    DuplicateSymbol { name: String, kind: SymbolKind },
    /// Type mismatch.
    TypeMismatch {
        expected: String,
        found: String,
        context: Option<String>,
    },
    /// Invalid context.
    InvalidContext { name: String },
    /// Library not found.
    LibraryNotFound { identifier: String },
    /// Model not found.
    ModelNotFound { name: String },
    /// Access violation (private symbol accessed from outside).
    AccessViolation { name: String, library: String },
    /// Function overload not found.
    NoMatchingOverload {
        name: String,
        arg_types: Vec<String>,
    },
    /// Ambiguous function call (multiple overloads match).
    AmbiguousOverload {
        name: String,
        candidates: Vec<String>,
    },
    /// Invalid expression.
    InvalidExpression { message: String },
    /// Unsupported feature.
    Unsupported { feature: String },
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::UnresolvedIdentifier { name, context } => {
                if let Some(ctx) = context {
                    write!(f, "Could not resolve identifier '{name}' in {ctx}")
                } else {
                    write!(f, "Could not resolve identifier '{name}'")
                }
            }
            BuilderError::UnresolvedQualifiedIdentifier { qualifier, name } => {
                write!(f, "Could not resolve '{qualifier}.{name}'")
            }
            BuilderError::DuplicateSymbol { name, kind } => {
                write!(f, "Duplicate {kind} definition: '{name}'")
            }
            BuilderError::TypeMismatch {
                expected,
                found,
                context,
            } => {
                if let Some(ctx) = context {
                    write!(
                        f,
                        "Type mismatch in {ctx}: expected {expected}, found {found}"
                    )
                } else {
                    write!(f, "Type mismatch: expected {expected}, found {found}")
                }
            }
            BuilderError::InvalidContext { name } => {
                write!(f, "Invalid context: '{name}'")
            }
            BuilderError::LibraryNotFound { identifier } => {
                write!(f, "Library not found: {identifier}")
            }
            BuilderError::ModelNotFound { name } => {
                write!(f, "Model not found: '{name}'")
            }
            BuilderError::AccessViolation { name, library } => {
                write!(
                    f,
                    "Cannot access private definition '{name}' from library '{library}'"
                )
            }
            BuilderError::NoMatchingOverload { name, arg_types } => {
                write!(
                    f,
                    "No matching overload for '{name}' with arguments ({})",
                    arg_types.join(", ")
                )
            }
            BuilderError::AmbiguousOverload { name, candidates } => {
                write!(
                    f,
                    "Ambiguous call to '{name}', candidates: {}",
                    candidates.join(", ")
                )
            }
            BuilderError::InvalidExpression { message } => {
                write!(f, "Invalid expression: {message}")
            }
            BuilderError::Unsupported { feature } => {
                write!(f, "Unsupported feature: {feature}")
            }
        }
    }
}

impl std::error::Error for BuilderError {}

/// Result type for builder operations.
pub type BuilderResult<T> = Result<T, BuilderError>;

// ============================================================================
// LibraryBuilder
// ============================================================================

/// The main builder for translating CQL to ELM.
///
/// The `LibraryBuilder` manages:
/// - Symbol tables (scopes) for identifier resolution
/// - The current evaluation context
/// - Access to included libraries
/// - Type resolution through model providers
///
/// # Scope Stack
///
/// The builder maintains a stack of scopes:
/// - **Level 0**: Library-level definitions (expressions, functions, parameters)
/// - **Level 1+**: Query aliases, let bindings, function parameters
///
/// Inner scopes shadow outer scopes, allowing local names to override library-level names.
pub struct LibraryBuilder<'a> {
    /// Library name.
    library_name: Option<String>,
    /// Library version.
    library_version: Option<String>,
    /// The current evaluation context (e.g., "Patient").
    context: Option<String>,
    /// Stack of scopes for identifier resolution.
    scopes: Vec<Scope>,
    /// Preprocessed library info.
    library_info: Option<LibraryInfo>,
    /// Model information provider.
    model_provider: Option<&'a dyn ModelInfoProvider>,
    /// Included libraries, keyed by local alias.
    included_libraries: HashMap<String, CompiledLibrary>,
    /// Using models, keyed by local name.
    using_models: HashMap<String, String>,
    /// Errors collected during building.
    errors: Vec<BuilderError>,
    /// Warnings collected during building.
    warnings: Vec<String>,
}

impl std::fmt::Debug for LibraryBuilder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryBuilder")
            .field("library_name", &self.library_name)
            .field("library_version", &self.library_version)
            .field("context", &self.context)
            .field("scopes", &self.scopes)
            .field("library_info", &self.library_info)
            .field("model_provider", &self.model_provider.is_some())
            .field(
                "included_libraries",
                &self.included_libraries.keys().collect::<Vec<_>>(),
            )
            .field("using_models", &self.using_models)
            .field("errors", &self.errors)
            .field("warnings", &self.warnings)
            .finish()
    }
}

impl Default for LibraryBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> LibraryBuilder<'a> {
    /// Create a new library builder with default settings.
    pub fn new() -> Self {
        Self {
            library_name: None,
            library_version: None,
            context: None,
            scopes: vec![Scope::new()], // Start with library-level scope
            library_info: None,
            model_provider: None,
            included_libraries: HashMap::new(),
            using_models: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a builder initialized from preprocessed library info.
    pub fn from_library_info(info: LibraryInfo) -> Self {
        let mut builder = Self::new();
        builder.library_name = info.name().map(String::from);
        builder.library_version = info.version().map(String::from);
        builder.library_info = Some(info);
        builder
    }

    // ========================================================================
    // Library Identity
    // ========================================================================

    /// Set the library name.
    pub fn set_library_name(&mut self, name: impl Into<String>) {
        self.library_name = Some(name.into());
    }

    /// Get the library name.
    pub fn library_name(&self) -> Option<&str> {
        self.library_name.as_deref()
    }

    /// Set the library version.
    pub fn set_library_version(&mut self, version: impl Into<String>) {
        self.library_version = Some(version.into());
    }

    /// Get the library version.
    pub fn library_version(&self) -> Option<&str> {
        self.library_version.as_deref()
    }

    /// Get the library identifier.
    pub fn library_identifier(&self) -> Option<LibraryIdentifier> {
        self.library_name
            .as_ref()
            .map(|n| LibraryIdentifier::new(n, self.library_version.as_deref()))
    }

    // ========================================================================
    // Context Management
    // ========================================================================

    /// Set the current evaluation context.
    ///
    /// The context determines the implicit target for retrieve expressions
    /// and provides access to the context variable (e.g., Patient).
    pub fn set_context(&mut self, context: impl Into<String>) {
        let ctx = context.into();

        // Add the context as a symbol if not already present
        if self.resolve_identifier(&ctx).is_none() {
            let symbol = Symbol::new(&ctx, SymbolKind::Context);
            self.define_symbol(symbol);
        }

        self.context = Some(ctx);
    }

    /// Get the current evaluation context.
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    /// Clear the current context.
    pub fn clear_context(&mut self) {
        self.context = None;
    }

    // ========================================================================
    // Model Management
    // ========================================================================

    /// Set the model information provider.
    pub fn set_model_provider(&mut self, provider: &'a dyn ModelInfoProvider) {
        self.model_provider = Some(provider);
    }

    /// Get the model information provider.
    pub fn model_provider(&self) -> Option<&dyn ModelInfoProvider> {
        self.model_provider
    }

    /// Register a using declaration.
    ///
    /// Maps a local model name (e.g., "FHIR") to its URI.
    pub fn add_using(&mut self, local_name: impl Into<String>, uri: impl Into<String>) {
        self.using_models.insert(local_name.into(), uri.into());
    }

    /// Check if a model is registered.
    pub fn has_model(&self, name: &str) -> bool {
        self.using_models.contains_key(name)
    }

    /// Get the URI for a model.
    pub fn model_uri(&self, name: &str) -> Option<&str> {
        self.using_models.get(name).map(|s| s.as_str())
    }

    // ========================================================================
    // Library Dependencies
    // ========================================================================

    /// Register an included library.
    ///
    /// The library is registered by its local alias (or name if no alias).
    pub fn add_included_library(
        &mut self,
        local_name: impl Into<String>,
        library: CompiledLibrary,
    ) {
        let local = local_name.into();

        // Add a symbol for the library reference
        let symbol = Symbol::new(&local, SymbolKind::Library);
        self.define_symbol(symbol);

        self.included_libraries.insert(local, library);
    }

    /// Get an included library by local name.
    pub fn get_included_library(&self, local_name: &str) -> Option<&CompiledLibrary> {
        self.included_libraries.get(local_name)
    }

    /// Check if a library is included.
    pub fn has_included_library(&self, local_name: &str) -> bool {
        self.included_libraries.contains_key(local_name)
    }

    /// Get all included library names.
    pub fn included_library_names(&self) -> impl Iterator<Item = &str> {
        self.included_libraries.keys().map(|s| s.as_str())
    }

    // ========================================================================
    // Scope Management
    // ========================================================================

    /// Push a new scope onto the stack.
    ///
    /// New scopes are used for queries, let bindings, and function parameters.
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Pop the current scope from the stack.
    ///
    /// Returns the popped scope, or None if at library level.
    pub fn pop_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    /// Get the current scope depth (0 = library level).
    pub fn scope_depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Get a reference to the current (innermost) scope.
    pub fn current_scope(&self) -> &Scope {
        self.scopes.last().expect("scope stack should not be empty")
    }

    /// Get a mutable reference to the current scope.
    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes
            .last_mut()
            .expect("scope stack should not be empty")
    }

    /// Get a reference to the library-level scope.
    pub fn library_scope(&self) -> &Scope {
        &self.scopes[0]
    }

    /// Get a mutable reference to the library-level scope.
    pub fn library_scope_mut(&mut self) -> &mut Scope {
        &mut self.scopes[0]
    }

    // ========================================================================
    // Symbol Definition
    // ========================================================================

    /// Define a symbol in the current scope.
    ///
    /// Returns a duplicate error if a symbol with the same name exists.
    pub fn define_symbol(&mut self, symbol: Symbol) -> Option<BuilderError> {
        let scope = self.current_scope_mut();
        if let Some(existing) = scope.get(&symbol.name) {
            return Some(BuilderError::DuplicateSymbol {
                name: symbol.name.clone(),
                kind: existing.kind,
            });
        }
        scope.define(symbol);
        None
    }

    /// Define a symbol in the library-level scope.
    ///
    /// Use this for expressions, functions, parameters, and terminology definitions.
    pub fn define_library_symbol(&mut self, symbol: Symbol) -> Option<BuilderError> {
        let scope = self.library_scope_mut();
        if let Some(existing) = scope.get(&symbol.name) {
            return Some(BuilderError::DuplicateSymbol {
                name: symbol.name.clone(),
                kind: existing.kind,
            });
        }
        scope.define(symbol);
        None
    }

    /// Define a function signature for overload resolution.
    pub fn define_function(&mut self, signature: FunctionSignature) {
        self.library_scope_mut().define_function(signature);
    }

    // ========================================================================
    // Identifier Resolution
    // ========================================================================

    /// Resolve an identifier by searching scopes from innermost to outermost.
    ///
    /// Returns the resolved symbol and scope depth if found.
    pub fn resolve_identifier(&self, name: &str) -> Option<ResolvedIdentifier> {
        // Search from innermost scope to outermost
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(symbol) = scope.get(name) {
                return Some(ResolvedIdentifier {
                    symbol: symbol.clone(),
                    scope_depth: depth,
                });
            }
        }
        None
    }

    /// Resolve a qualified identifier (library.name or type.member).
    ///
    /// First checks if the qualifier is an included library, then checks
    /// if it's a model namespace.
    pub fn resolve_qualified_identifier(
        &self,
        qualifier: &str,
        name: &str,
    ) -> BuilderResult<ResolvedIdentifier> {
        // Check if qualifier is an included library
        if let Some(library) = self.included_libraries.get(qualifier) {
            // Look up the definition in the included library
            if library.has_definition(name) {
                let symbol = Symbol::new(name, SymbolKind::Expression)
                    .with_library(library.to_library_identifier());
                return Ok(ResolvedIdentifier {
                    symbol,
                    scope_depth: 0,
                });
            }
            return Err(BuilderError::UnresolvedQualifiedIdentifier {
                qualifier: qualifier.to_string(),
                name: name.to_string(),
            });
        }

        // Check if qualifier is a model namespace
        if self.using_models.contains_key(qualifier) {
            // This would resolve to a model type or property
            let symbol = Symbol::new(name, SymbolKind::Expression);
            return Ok(ResolvedIdentifier {
                symbol,
                scope_depth: 0,
            });
        }

        // Check if qualifier is a query alias in scope
        if let Some(resolved) = self.resolve_identifier(qualifier) {
            if matches!(
                resolved.symbol.kind,
                SymbolKind::QueryAlias | SymbolKind::Let
            ) {
                // This is a property access on an alias
                let symbol = Symbol::new(name, SymbolKind::Expression);
                return Ok(ResolvedIdentifier {
                    symbol,
                    scope_depth: resolved.scope_depth,
                });
            }
        }

        Err(BuilderError::UnresolvedQualifiedIdentifier {
            qualifier: qualifier.to_string(),
            name: name.to_string(),
        })
    }

    /// Resolve function overloads by name.
    ///
    /// Returns all function signatures that match the given name.
    pub fn resolve_function(&self, name: &str) -> Vec<&FunctionSignature> {
        let mut results = Vec::new();

        // Check local functions
        for scope in self.scopes.iter().rev() {
            if let Some(signatures) = scope.get_functions(name) {
                results.extend(signatures);
            }
        }

        // Check included libraries
        for library in self.included_libraries.values() {
            // Note: In a full implementation, we'd need to get function signatures
            // from the compiled library. For now, we just check if the function exists.
            if library.has_definition(name) {
                // Would need more info to create proper signatures
            }
        }

        results
    }

    /// Check if an identifier is defined in any scope.
    pub fn is_defined(&self, name: &str) -> bool {
        self.resolve_identifier(name).is_some()
    }

    // ========================================================================
    // Error Handling
    // ========================================================================

    /// Add an error to the error list.
    pub fn add_error(&mut self, error: BuilderError) {
        self.errors.push(error);
    }

    /// Add a warning to the warning list.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Get all errors.
    pub fn errors(&self) -> &[BuilderError] {
        &self.errors
    }

    /// Get all warnings.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Take all errors, clearing the error list.
    pub fn take_errors(&mut self) -> Vec<BuilderError> {
        std::mem::take(&mut self.errors)
    }

    /// Take all warnings, clearing the warning list.
    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }

    // ========================================================================
    // Library Info Access
    // ========================================================================

    /// Get the preprocessed library info.
    pub fn library_info(&self) -> Option<&LibraryInfo> {
        self.library_info.as_ref()
    }

    // ========================================================================
    // Building Helpers
    // ========================================================================

    /// Execute a function with a new scope.
    ///
    /// The scope is automatically popped after the function completes.
    pub fn with_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.push_scope();
        let result = f(self);
        self.pop_scope();
        result
    }

    /// Convert an AST access modifier to an ELM access modifier.
    pub fn convert_access_modifier(access: ast::AccessModifier) -> elm::AccessModifier {
        match access {
            ast::AccessModifier::Public => elm::AccessModifier::Public,
            ast::AccessModifier::Private => elm::AccessModifier::Private,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::SystemType;

    #[test]
    fn test_symbol_kind_display() {
        assert_eq!(SymbolKind::Expression.to_string(), "expression");
        assert_eq!(SymbolKind::Function.to_string(), "function");
        assert_eq!(SymbolKind::QueryAlias.to_string(), "alias");
    }

    #[test]
    fn test_symbol_new() {
        let symbol = Symbol::new("Age", SymbolKind::Expression);
        assert_eq!(symbol.name, "Age");
        assert_eq!(symbol.kind, SymbolKind::Expression);
        assert!(symbol.result_type.is_none());
        assert!(symbol.is_local());
        assert!(symbol.is_public);
    }

    #[test]
    fn test_symbol_with_type() {
        let symbol = Symbol::new("Age", SymbolKind::Expression)
            .with_type(DataType::system(SystemType::Integer));
        assert_eq!(
            symbol.result_type,
            Some(DataType::system(SystemType::Integer))
        );
    }

    #[test]
    fn test_symbol_with_library() {
        let lib_id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let symbol = Symbol::new("ToQuantity", SymbolKind::Function).with_library(lib_id.clone());
        assert!(!symbol.is_local());
        assert_eq!(symbol.library, Some(lib_id));
    }

    #[test]
    fn test_symbol_with_access() {
        let symbol = Symbol::new("Helper", SymbolKind::Expression).with_access(false);
        assert!(!symbol.is_public);
    }

    #[test]
    fn test_scope_new() {
        let scope = Scope::new();
        assert!(scope.is_empty());
        assert_eq!(scope.len(), 0);
    }

    #[test]
    fn test_scope_define_and_get() {
        let mut scope = Scope::new();
        let symbol = Symbol::new("X", SymbolKind::Expression);
        scope.define(symbol);

        assert!(scope.contains("X"));
        assert!(!scope.contains("Y"));

        let resolved = scope.get("X");
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().name, "X");
    }

    #[test]
    fn test_scope_symbol_names() {
        let mut scope = Scope::new();
        scope.define(Symbol::new("A", SymbolKind::Expression));
        scope.define(Symbol::new("B", SymbolKind::Function));

        let names: Vec<&str> = scope.symbol_names().collect();
        assert!(names.contains(&"A"));
        assert!(names.contains(&"B"));
        assert_eq!(scope.len(), 2);
    }

    #[test]
    fn test_scope_function_signatures() {
        let mut scope = Scope::new();
        let sig1 = FunctionSignature::new("ToInteger", DataType::system(SystemType::Integer))
            .with_operands(vec![DataType::system(SystemType::String)]);
        let sig2 = FunctionSignature::new("ToInteger", DataType::system(SystemType::Integer))
            .with_operands(vec![DataType::system(SystemType::Decimal)]);

        scope.define_function(sig1);
        scope.define_function(sig2);

        let funcs = scope.get_functions("ToInteger");
        assert!(funcs.is_some());
        assert_eq!(funcs.unwrap().len(), 2);
    }

    #[test]
    fn test_function_signature_new() {
        let sig = FunctionSignature::new("Add", DataType::system(SystemType::Integer))
            .with_operands(vec![
                DataType::system(SystemType::Integer),
                DataType::system(SystemType::Integer),
            ])
            .fluent();

        assert_eq!(sig.name, "Add");
        assert_eq!(sig.arity(), 2);
        assert!(sig.is_fluent);
        assert!(!sig.is_external);
    }

    #[test]
    fn test_builder_new() {
        let builder = LibraryBuilder::new();
        assert!(builder.library_name().is_none());
        assert!(builder.context().is_none());
        assert_eq!(builder.scope_depth(), 0);
    }

    #[test]
    fn test_builder_library_identity() {
        let mut builder = LibraryBuilder::new();
        builder.set_library_name("TestLib");
        builder.set_library_version("1.0.0");

        assert_eq!(builder.library_name(), Some("TestLib"));
        assert_eq!(builder.library_version(), Some("1.0.0"));

        let id = builder.library_identifier();
        assert!(id.is_some());
        assert_eq!(id.unwrap().name, "TestLib");
    }

    #[test]
    fn test_builder_context() {
        let mut builder = LibraryBuilder::new();
        assert!(builder.context().is_none());

        builder.set_context("Patient");
        assert_eq!(builder.context(), Some("Patient"));

        // Context should be added as a symbol
        assert!(builder.is_defined("Patient"));

        builder.clear_context();
        assert!(builder.context().is_none());
    }

    #[test]
    fn test_builder_using_models() {
        let mut builder = LibraryBuilder::new();
        builder.add_using("FHIR", "http://hl7.org/fhir");

        assert!(builder.has_model("FHIR"));
        assert!(!builder.has_model("QDM"));
        assert_eq!(builder.model_uri("FHIR"), Some("http://hl7.org/fhir"));
    }

    #[test]
    fn test_builder_scope_stack() {
        let mut builder = LibraryBuilder::new();
        assert_eq!(builder.scope_depth(), 0);

        builder.push_scope();
        assert_eq!(builder.scope_depth(), 1);

        builder.push_scope();
        assert_eq!(builder.scope_depth(), 2);

        builder.pop_scope();
        assert_eq!(builder.scope_depth(), 1);

        builder.pop_scope();
        assert_eq!(builder.scope_depth(), 0);

        // Cannot pop below library level
        let result = builder.pop_scope();
        assert!(result.is_none());
        assert_eq!(builder.scope_depth(), 0);
    }

    #[test]
    fn test_builder_define_symbol() {
        let mut builder = LibraryBuilder::new();
        let symbol = Symbol::new("X", SymbolKind::Expression);

        let error = builder.define_symbol(symbol);
        assert!(error.is_none());

        // Duplicate should return error
        let dup = Symbol::new("X", SymbolKind::Expression);
        let error = builder.define_symbol(dup);
        assert!(error.is_some());
        assert!(matches!(error, Some(BuilderError::DuplicateSymbol { .. })));
    }

    #[test]
    fn test_builder_resolve_identifier() {
        let mut builder = LibraryBuilder::new();

        // Define in library scope
        builder.define_symbol(Symbol::new("LibLevel", SymbolKind::Expression));

        builder.push_scope();
        builder.define_symbol(Symbol::new("QueryAlias", SymbolKind::QueryAlias));

        // Can resolve both
        let lib_resolved = builder.resolve_identifier("LibLevel");
        assert!(lib_resolved.is_some());
        assert_eq!(lib_resolved.unwrap().scope_depth, 1);

        let alias_resolved = builder.resolve_identifier("QueryAlias");
        assert!(alias_resolved.is_some());
        assert_eq!(alias_resolved.unwrap().scope_depth, 0);

        // Unknown returns None
        let unknown = builder.resolve_identifier("Unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_builder_scope_shadowing() {
        let mut builder = LibraryBuilder::new();

        // Define X in library scope
        builder.define_symbol(
            Symbol::new("X", SymbolKind::Expression)
                .with_type(DataType::system(SystemType::Integer)),
        );

        builder.push_scope();

        // Shadow X in inner scope
        builder.define_symbol(
            Symbol::new("X", SymbolKind::QueryAlias)
                .with_type(DataType::system(SystemType::String)),
        );

        // Should resolve to inner scope
        let resolved = builder.resolve_identifier("X").unwrap();
        assert_eq!(resolved.symbol.kind, SymbolKind::QueryAlias);
        assert_eq!(resolved.scope_depth, 0);

        builder.pop_scope();

        // Should now resolve to library scope
        let resolved = builder.resolve_identifier("X").unwrap();
        assert_eq!(resolved.symbol.kind, SymbolKind::Expression);
    }

    #[test]
    fn test_builder_with_scope() {
        let mut builder = LibraryBuilder::new();
        assert_eq!(builder.scope_depth(), 0);

        let result = builder.with_scope(|b| {
            assert_eq!(b.scope_depth(), 1);
            b.define_symbol(Symbol::new("Temp", SymbolKind::Let));
            42
        });

        assert_eq!(result, 42);
        assert_eq!(builder.scope_depth(), 0);
        // Temp should not be visible
        assert!(!builder.is_defined("Temp"));
    }

    #[test]
    fn test_builder_errors() {
        let mut builder = LibraryBuilder::new();
        assert!(!builder.has_errors());

        builder.add_error(BuilderError::UnresolvedIdentifier {
            name: "X".to_string(),
            context: None,
        });

        assert!(builder.has_errors());
        assert_eq!(builder.errors().len(), 1);

        let errors = builder.take_errors();
        assert_eq!(errors.len(), 1);
        assert!(!builder.has_errors());
    }

    #[test]
    fn test_builder_warnings() {
        let mut builder = LibraryBuilder::new();
        builder.add_warning("Consider using X instead of Y");
        builder.add_warning("Deprecated feature");

        assert_eq!(builder.warnings().len(), 2);

        let warnings = builder.take_warnings();
        assert_eq!(warnings.len(), 2);
        assert!(builder.warnings().is_empty());
    }

    #[test]
    fn test_builder_error_display() {
        let err = BuilderError::UnresolvedIdentifier {
            name: "X".to_string(),
            context: Some("query".to_string()),
        };
        assert!(err.to_string().contains("X"));
        assert!(err.to_string().contains("query"));

        let err = BuilderError::DuplicateSymbol {
            name: "Y".to_string(),
            kind: SymbolKind::Expression,
        };
        assert!(err.to_string().contains("Duplicate"));
        assert!(err.to_string().contains("expression"));
    }

    #[test]
    fn test_builder_from_library_info() {
        use crate::parser::CqlParser;
        use crate::preprocessor::Preprocessor;

        let source = r#"
            library TestLib version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define X: true
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let info = Preprocessor::process(&ast);

        let builder = LibraryBuilder::from_library_info(info);
        assert_eq!(builder.library_name(), Some("TestLib"));
        assert_eq!(builder.library_version(), Some("1.0"));
    }

    #[test]
    fn test_resolved_identifier() {
        let lib_id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        let symbol = Symbol::new("ToQuantity", SymbolKind::Function).with_library(lib_id);
        let resolved = ResolvedIdentifier {
            symbol,
            scope_depth: 0,
        };

        assert!(!resolved.is_local());
        assert_eq!(resolved.library_name(), Some("FHIRHelpers"));
    }

    #[test]
    fn test_convert_access_modifier() {
        assert_eq!(
            LibraryBuilder::convert_access_modifier(ast::AccessModifier::Public),
            elm::AccessModifier::Public
        );
        assert_eq!(
            LibraryBuilder::convert_access_modifier(ast::AccessModifier::Private),
            elm::AccessModifier::Private
        );
    }
}
