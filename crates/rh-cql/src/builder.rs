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

use crate::conversion::{ConversionContext, ConversionRegistry, ConversionResult};
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
    /// Generic semantic error.
    SemanticError { message: String },
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
            BuilderError::SemanticError { message } => {
                write!(f, "{message}")
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
    /// Compiler options.
    options: crate::options::CompilerOptions,
    /// Errors collected during building.
    errors: Vec<BuilderError>,
    /// Warnings collected during building.
    warnings: Vec<String>,
    /// Type conversion context for implicit conversions.
    conversion_context: ConversionContext,
    /// Tracks inferred result types for expression definitions.
    /// Maps definition name to FHIR type string (e.g., "FHIR.Quantity").
    expression_result_types: HashMap<String, String>,
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
            .field("options", &self.options)
            .field("errors", &self.errors)
            .field("warnings", &self.warnings)
            .field(
                "conversion_context",
                &format!("{} conversions", self.conversion_context.registry().len()),
            )
            .field(
                "expression_result_types",
                &self.expression_result_types.len(),
            )
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
            options: crate::options::CompilerOptions::default(),
            errors: Vec::new(),
            warnings: Vec::new(),
            conversion_context: ConversionContext::new(),
            expression_result_types: HashMap::new(),
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

    /// Set the compiler options.
    pub fn set_options(&mut self, options: crate::options::CompilerOptions) {
        self.options = options;
    }

    /// Get the compiler options.
    pub fn options(&self) -> &crate::options::CompilerOptions {
        &self.options
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
    // Type Conversion Management
    // ========================================================================

    /// Set the conversion registry from a ModelInfo.
    ///
    /// This loads all conversion definitions from the ModelInfo, enabling
    /// automatic implicit conversions during translation.
    pub fn set_conversion_registry(&mut self, registry: ConversionRegistry) {
        self.conversion_context = ConversionContext::with_registry(registry);
    }

    /// Get the conversion context.
    pub fn conversion_context(&self) -> &ConversionContext {
        &self.conversion_context
    }

    /// Get a mutable reference to the conversion context.
    pub fn conversion_context_mut(&mut self) -> &mut ConversionContext {
        &mut self.conversion_context
    }

    /// Register a library as available for conversions.
    ///
    /// Call this when an `include` statement is processed to enable
    /// conversions that require that library (e.g., FHIRHelpers).
    pub fn register_conversion_library(&mut self, alias: &str, library_name: &str) {
        self.conversion_context
            .add_included_library(alias, library_name);
    }

    /// Check if a library required for conversions is available.
    pub fn is_conversion_library_available(&self, library_name: &str) -> bool {
        self.conversion_context.is_library_included(library_name)
    }

    /// Try to apply an implicit conversion to an expression.
    ///
    /// If the actual type doesn't match the expected type and a conversion
    /// is defined in the ModelInfo, this wraps the expression in a FunctionRef
    /// to the appropriate converter function.
    ///
    /// This method respects compiler options:
    /// - `DisableImplicitConversions`: If set, no conversions are applied
    /// - `StrictConversionLibraryCheck`: If set, missing library is an error not warning
    ///
    /// # Arguments
    ///
    /// * `operand` - The expression to potentially convert.
    /// * `actual_type` - The actual type of the operand.
    /// * `expected_type` - The expected/target type.
    /// * `translator` - The expression translator (for generating element fields).
    ///
    /// # Returns
    ///
    /// The potentially converted expression and a boolean indicating if conversion was applied.
    pub fn apply_implicit_conversion(
        &mut self,
        operand: elm::Expression,
        actual_type: &DataType,
        expected_type: &DataType,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> (elm::Expression, bool) {
        // Check if implicit conversions are disabled
        if !self.options.implicit_conversions_enabled() {
            return (operand, false);
        }

        let element_fields = translator.element_fields();

        match self.conversion_context.try_convert(
            operand.clone(),
            actual_type,
            expected_type,
            element_fields,
        ) {
            ConversionResult::NotNeeded => (operand, false),
            ConversionResult::Applied { expression, .. } => (*expression, true),
            ConversionResult::LibraryNotIncluded {
                entry,
                required_library,
            } => {
                let message = format!(
                    "Implicit conversion from {} to {} requires {} to be included",
                    entry.from_type, entry.to_type, required_library
                );

                // Check if strict mode is enabled - emit error instead of warning
                if self.options.strict_conversion_library_check() {
                    self.errors.push(BuilderError::SemanticError {
                        message: message.clone(),
                    });
                } else {
                    self.warnings.push(message);
                }
                (operand, false)
            }
            ConversionResult::NoConversionDefined { from_type, to_type } => {
                // Type mismatch with no known conversion - this might be an error
                // depending on context, but we don't emit here since it might be
                // valid for other reasons (e.g., subtype relationship, etc.)
                // Only emit as a debug-level message, not a warning
                let _ = (from_type, to_type); // suppress unused variable warning
                (operand, false)
            }
        }
    }

    /// Look up a property type from ModelInfo.
    ///
    /// Given a FHIR class name (e.g., "Observation") and property name (e.g., "status"),
    /// returns the property type (e.g., "FHIR.ObservationStatus").
    ///
    /// Returns None if:
    /// - No model provider is configured
    /// - The class is not found in ModelInfo
    /// - The property is not found on the class
    pub fn lookup_property_type(
        &self,
        model_name: &str,
        class_name: &str,
        property_name: &str,
    ) -> Option<String> {
        let provider = self.model_provider?;
        let class_info = provider.resolve_class(model_name, None, class_name)?;

        // Look through the elements to find the property
        for element in &class_info.element {
            if element.name.as_deref() == Some(property_name) {
                // Return element_type if available, otherwise type_name
                return element
                    .element_type
                    .clone()
                    .or_else(|| element.type_name.clone());
            }
        }

        // Property not found on this class - could check base classes but ModelInfo
        // typically flattens inherited properties. For now, return None.
        None
    }

    /// Attempt to apply a FHIR-to-System conversion on a property access expression.
    ///
    /// This is called for scoped property access (e.g., `ER.status` in a query).
    /// If the property type is a FHIR primitive (like `FHIR.code`) and we have
    /// FHIRHelpers included, wraps the property in the appropriate conversion
    /// function (e.g., `FHIRHelpers.ToString`).
    ///
    /// Returns the original expression unchanged if:
    /// - The alias type is unknown
    /// - The property type is unknown
    /// - No conversion is defined for the property type
    /// - The required converter library is not included
    fn maybe_apply_property_conversion(
        &self,
        property_expr: elm::Expression,
        alias_name: &str,
        property_name: &str,
        translator: &crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        // Get the alias type (FHIR class name like "Observation")
        let class_name = match translator.get_query_alias_type(alias_name) {
            Some(t) => t,
            None => return property_expr,
        };

        // Look up the property type from ModelInfo
        let property_type = match self.lookup_property_type("FHIR", class_name, property_name) {
            Some(t) => t,
            None => return property_expr,
        };

        // Find a conversion from this FHIR type to a System type
        // We look for any conversion from this property type
        let conversions = self
            .conversion_context
            .registry()
            .find_conversions_from(&property_type);

        // Find a conversion to a System type if FHIRHelpers is available
        for entry in conversions {
            // Accept both System.* types and Interval<System.*> types
            if entry.to_type.starts_with("System.") || entry.to_type.starts_with("Interval<System.")
            {
                // Check if the required library is included
                if let Some(lib) = &entry.library_name {
                    if self.conversion_context.is_library_included(lib) {
                        // Apply the conversion
                        return crate::conversion::wrap_in_conversion(
                            property_expr,
                            entry,
                            elm::ElementFields::default(),
                        );
                    }
                }
            }
        }

        // No applicable conversion found
        property_expr
    }

    /// Infer the FHIR result type from an AST expression.
    ///
    /// Returns the type string (e.g., "FHIR.Quantity") if it can be determined.
    /// Currently handles:
    /// - `as` type casts (e.g., `.value as Quantity`)
    /// - First/Last on typed queries (unwraps list to element type)
    fn infer_fhir_result_type(&self, expr: &ast::Expression) -> Option<String> {
        match expr {
            // Handle `as` type cast - the explicit type annotation
            ast::Expression::TypeExpression(type_expr) => {
                if type_expr.operator == ast::TypeOperator::As {
                    // Extract the type from the type specifier
                    return self.type_specifier_to_fhir_type(&type_expr.type_specifier);
                }
                None
            }

            // Handle First/Last - unwraps list to element type
            ast::Expression::FunctionInvocation(func) => {
                if (func.name == "First" || func.name == "Last") && func.arguments.len() == 1 {
                    // Try to get the element type of the list argument
                    // For now, we propagate if the argument is an ExpressionRef
                    if let ast::Expression::IdentifierRef(id_ref) = &func.arguments[0] {
                        // Look up the referenced definition's type
                        if let Some(ref_type) = self.expression_result_types.get(&id_ref.name) {
                            // If it's a list type, return the element type
                            // For simplicity, just return as-is since FHIR types don't use List<>
                            return Some(ref_type.clone());
                        }
                    }
                }
                None
            }

            // Handle query - try to infer from the return clause or source
            ast::Expression::Query(query) => {
                // If there's an explicit return clause, try to infer from that
                if let Some(ret) = &query.return_clause {
                    return self.infer_fhir_result_type(&ret.expression);
                }
                // Otherwise, query returns list of source type
                None
            }

            // Recursively handle parenthesized expressions
            ast::Expression::Parenthesized(inner) => self.infer_fhir_result_type(inner),

            _ => None,
        }
    }

    /// Convert a type specifier to a FHIR type string.
    fn type_specifier_to_fhir_type(&self, ts: &ast::TypeSpecifier) -> Option<String> {
        match ts {
            ast::TypeSpecifier::Named(named) => {
                let namespace = named.namespace.as_deref().unwrap_or("FHIR");
                Some(format!("{}.{}", namespace, named.name))
            }
            _ => None,
        }
    }

    /// Register the result type of an expression definition.
    fn register_expression_result_type(&mut self, name: &str, fhir_type: &str) {
        self.expression_result_types
            .insert(name.to_string(), fhir_type.to_string());
    }

    /// Get the registered result type of an expression definition.
    pub fn get_expression_result_type(&self, name: &str) -> Option<&str> {
        self.expression_result_types.get(name).map(|s| s.as_str())
    }

    /// Apply conversion to an ExpressionRef if needed based on the referenced
    /// expression's result type.
    fn maybe_apply_expression_ref_conversion(
        &self,
        expr_ref: elm::Expression,
        def_name: &str,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        // Look up the result type of the referenced definition
        let fhir_type = match self.get_expression_result_type(def_name) {
            Some(t) => t,
            None => return expr_ref,
        };

        // Find a conversion from this FHIR type to a System type
        let conversions = self
            .conversion_context
            .registry()
            .find_conversions_from(fhir_type);

        // Look for a conversion to a System type with FHIRHelpers available
        for entry in conversions {
            if entry.to_type.starts_with("System.") {
                if let Some(lib) = &entry.library_name {
                    if self.conversion_context.is_library_included(lib) {
                        return crate::conversion::wrap_in_conversion(
                            expr_ref,
                            entry,
                            translator.element_fields(),
                        );
                    }
                }
            }
        }

        expr_ref
    }

    /// Check if an AST expression is an identifier reference and apply
    /// type conversion if the referenced expression has a known FHIR type.
    ///
    /// This is used for comparison operators where operands may need
    /// FHIRHelpers conversions (e.g., FHIR.Quantity → System.Quantity).
    fn maybe_convert_expression_ref(
        &self,
        ast_expr: &ast::Expression,
        elm_expr: elm::Expression,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        // Check if this is an identifier reference to a definition
        if let ast::Expression::IdentifierRef(id_ref) = ast_expr {
            // Check if it resolves to an expression definition
            if let Some(resolved) = self.resolve_identifier(&id_ref.name) {
                if resolved.symbol.kind == SymbolKind::Expression {
                    return self.maybe_apply_expression_ref_conversion(
                        elm_expr,
                        &id_ref.name,
                        translator,
                    );
                }
            }
        }
        elm_expr
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

    /// Map a SymbolKind to a ResolvedRefKind.
    fn symbol_kind_to_ref_kind(kind: SymbolKind) -> crate::translator::ResolvedRefKind {
        use crate::translator::ResolvedRefKind;
        match kind {
            SymbolKind::Expression => ResolvedRefKind::Expression,
            SymbolKind::Function => ResolvedRefKind::Expression, // Functions are expression-like
            SymbolKind::Parameter => ResolvedRefKind::Parameter,
            SymbolKind::CodeSystem => ResolvedRefKind::CodeSystem,
            SymbolKind::ValueSet => ResolvedRefKind::ValueSet,
            SymbolKind::Code => ResolvedRefKind::Code,
            SymbolKind::Concept => ResolvedRefKind::Concept,
            SymbolKind::QueryAlias => ResolvedRefKind::QueryAlias,
            SymbolKind::Let => ResolvedRefKind::Let,
            SymbolKind::Operand => ResolvedRefKind::Operand,
            SymbolKind::Library => ResolvedRefKind::Unknown, // Library refs need qualified handling
            SymbolKind::Context => ResolvedRefKind::Context,
        }
    }

    /// Translate an expression with symbol resolution.
    ///
    /// This method translates an AST expression to ELM, resolving identifier
    /// references using the builder's symbol table.
    pub fn translate_expression_with_resolution(
        &self,
        expr: &ast::Expression,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        self.translate_expr_recursive(expr, translator)
    }

    /// Recursive helper for expression translation with resolution.
    fn translate_expr_recursive(
        &self,
        expr: &ast::Expression,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        use crate::translator::ResolvedRefKind;

        match expr {
            // Handle identifier references with resolution
            ast::Expression::IdentifierRef(id_ref) => {
                if let Some(resolved) = self.resolve_identifier(&id_ref.name) {
                    let ref_kind = Self::symbol_kind_to_ref_kind(resolved.symbol.kind);
                    translator.translate_identifier_ref(&id_ref.name, ref_kind, None)
                } else {
                    // Unresolved - keep as IdentifierRef
                    translator.translate_identifier_ref(
                        &id_ref.name,
                        ResolvedRefKind::Unknown,
                        None,
                    )
                }
            }

            // Handle qualified identifier references
            ast::Expression::QualifiedIdentifierRef(qid_ref) => {
                // Check if qualifier is a known alias in scope (query alias, let binding)
                if let Some(resolved) = self.resolve_identifier(&qid_ref.qualifier) {
                    match resolved.symbol.kind {
                        SymbolKind::QueryAlias | SymbolKind::Let => {
                            // This is property access on an alias - use scope
                            elm::Expression::Property(elm::Property {
                                element: translator.element_fields(),
                                source: None,
                                path: Some(qid_ref.name.clone()),
                                scope: Some(qid_ref.qualifier.clone()),
                            })
                        }
                        _ => {
                            // Fall through to generic handling
                            translator.translate_identifier_ref(
                                &qid_ref.name,
                                ResolvedRefKind::Unknown,
                                None,
                            )
                        }
                    }
                } else {
                    // Unknown qualifier
                    elm::Expression::IdentifierRef(elm::IdentifierRef {
                        element: translator.element_fields(),
                        name: Some(qid_ref.name.clone()),
                        library_name: Some(qid_ref.qualifier.clone()),
                    })
                }
            }

            // For all other expressions, delegate to translator but recursively
            // handle sub-expressions with resolution
            ast::Expression::Literal(lit) => translator.translate_literal(lit),

            ast::Expression::UnaryExpression(unary) => {
                let operand = self.translate_expr_recursive(&unary.operand, translator);
                translator.translate_unary_operator(unary.operator, operand, None)
            }

            ast::Expression::BinaryExpression(binary) => {
                let mut left = self.translate_expr_recursive(&binary.left, translator);
                let mut right = self.translate_expr_recursive(&binary.right, translator);

                // For comparison operators, apply ExpressionRef type conversions
                // This handles cases like: "Most Recent Tumor Size Quantity" > 1 'cm'
                // where the left side is a FHIR.Quantity that needs FHIRHelpers.ToQuantity
                if binary.operator.is_comparison() {
                    left = self.maybe_convert_expression_ref(&binary.left, left, translator);
                    right = self.maybe_convert_expression_ref(&binary.right, right, translator);
                }

                translator.translate_binary_operator(
                    binary.operator,
                    left,
                    right,
                    None,
                    binary.precision.as_ref(),
                )
            }

            ast::Expression::TernaryExpression(ternary) => {
                let first = self.translate_expr_recursive(&ternary.first, translator);
                let second = self.translate_expr_recursive(&ternary.second, translator);
                let third = self.translate_expr_recursive(&ternary.third, translator);
                translator.translate_ternary_operator(ternary.operator, first, second, third, None)
            }

            ast::Expression::DateTimeComponentFrom(dtc) => translator
                .translate_datetime_component_from(dtc, |t, e| self.translate_expr_recursive(e, t)),

            ast::Expression::IfThenElse(if_expr) => {
                let condition = self.translate_expr_recursive(&if_expr.condition, translator);
                let then_expr = self.translate_expr_recursive(&if_expr.then_expr, translator);
                let else_expr = self.translate_expr_recursive(&if_expr.else_expr, translator);
                elm::Expression::If(elm::IfExpr {
                    element: translator.element_fields(),
                    condition: Some(Box::new(condition)),
                    then_expr: Some(Box::new(then_expr)),
                    else_expr: Some(Box::new(else_expr)),
                })
            }

            ast::Expression::Query(query) => {
                self.translate_query_with_resolution(query, translator)
            }

            ast::Expression::Retrieve(retrieve) => {
                self.translate_retrieve_with_resolution(retrieve, translator)
            }

            ast::Expression::FunctionInvocation(func) => {
                let operands: Vec<elm::Expression> = func
                    .arguments
                    .iter()
                    .map(|arg| self.translate_expr_recursive(arg, translator))
                    .collect();

                elm::Expression::FunctionRef(elm::FunctionRef {
                    element: translator.element_fields(),
                    name: Some(func.name.clone()),
                    library_name: func.library.clone(),
                    operand: operands,
                    signature: Vec::new(),
                })
            }

            ast::Expression::MemberInvocation(member) => {
                // Check if source is a simple identifier that's a query alias
                if let ast::Expression::IdentifierRef(ident_ref) = &*member.source {
                    if translator.is_query_alias(&ident_ref.name) {
                        // Create the Property expression with scope attribute
                        let property_expr = elm::Expression::Property(elm::Property {
                            element: translator.element_fields(),
                            source: None,
                            path: Some(member.name.clone()),
                            scope: Some(ident_ref.name.clone()),
                        });

                        // Try to apply FHIR-to-System type conversion if needed
                        return self.maybe_apply_property_conversion(
                            property_expr,
                            &ident_ref.name,
                            &member.name,
                            translator,
                        );
                    }
                }

                // Default: translate source expression normally
                let source = self.translate_expr_recursive(&member.source, translator);
                elm::Expression::Property(elm::Property {
                    element: translator.element_fields(),
                    source: Some(Box::new(source)),
                    path: Some(member.name.clone()),
                    scope: None,
                })
            }

            ast::Expression::IndexInvocation(index) => {
                let source = self.translate_expr_recursive(&index.source, translator);
                let idx = self.translate_expr_recursive(&index.index, translator);
                elm::Expression::Indexer(elm::BinaryExpression {
                    element: translator.element_fields(),
                    operand: vec![source, idx],
                    signature: Vec::new(),
                })
            }

            ast::Expression::TypeExpression(type_expr) => translator
                .translate_type_expression(type_expr, |t, e| self.translate_expr_recursive(e, t)),

            ast::Expression::Case(case_expr) => translator.translate_case(
                case_expr,
                |t, e| self.translate_expr_recursive(e, t),
                None,
            ),

            ast::Expression::IntervalExpression(interval) => {
                let low = interval
                    .low
                    .as_ref()
                    .map(|e| Box::new(self.translate_expr_recursive(e, translator)));
                let high = interval
                    .high
                    .as_ref()
                    .map(|e| Box::new(self.translate_expr_recursive(e, translator)));
                elm::Expression::Interval(elm::IntervalExpr {
                    element: translator.element_fields(),
                    low,
                    high,
                    low_closed: Some(interval.low_closed),
                    high_closed: Some(interval.high_closed),
                    low_closed_expression: None,
                    high_closed_expression: None,
                })
            }

            ast::Expression::ListExpression(list) => {
                let elements: Vec<elm::Expression> = list
                    .elements
                    .iter()
                    .map(|e| self.translate_expr_recursive(e, translator))
                    .collect();
                elm::Expression::List(elm::ListExpr {
                    element: translator.element_fields(),
                    elements,
                    type_specifier: list
                        .type_specifier
                        .as_ref()
                        .map(|ts| translator.type_specifier_to_elm(ts)),
                })
            }

            ast::Expression::TupleExpression(tuple) => {
                let elements: Vec<elm::TupleElement> = tuple
                    .elements
                    .iter()
                    .map(|e| elm::TupleElement {
                        name: Some(e.name.clone()),
                        value: Some(Box::new(
                            self.translate_expr_recursive(&e.value, translator),
                        )),
                    })
                    .collect();
                elm::Expression::Tuple(elm::TupleExpr {
                    element: translator.element_fields(),
                    elements,
                })
            }

            ast::Expression::Instance(instance) => {
                let elements: Vec<elm::InstanceElement> = instance
                    .elements
                    .iter()
                    .map(|e| elm::InstanceElement {
                        name: Some(e.name.clone()),
                        value: Some(Box::new(
                            self.translate_expr_recursive(&e.value, translator),
                        )),
                    })
                    .collect();
                elm::Expression::Instance(elm::Instance {
                    element: translator.element_fields(),
                    class_type: Some(translator.type_specifier_to_qname(&instance.class_type)),
                    elements,
                })
            }

            ast::Expression::Let(let_clause) => {
                // Let expression creates a scope
                let value = self.translate_expr_recursive(&let_clause.expression, translator);
                // Note: proper let handling would need to add the binding to scope
                // For now, we translate it directly
                elm::Expression::Query(elm::Query {
                    element: translator.element_fields(),
                    source: Vec::new(),
                    let_clause: vec![elm::LetClause {
                        identifier: Some(let_clause.identifier.clone()),
                        expression: Some(Box::new(value)),
                    }],
                    relationship: Vec::new(),
                    where_clause: None,
                    return_clause: None,
                    aggregate: None,
                    sort: None,
                })
            }

            ast::Expression::Parenthesized(inner) => {
                self.translate_expr_recursive(inner, translator)
            }
        }
    }

    /// Translate a query expression with symbol resolution.
    ///
    /// Extracts type information from Retrieve source expressions and passes
    /// it to the translator for alias type tracking. This enables implicit
    /// type conversions on property access (e.g., FHIR.code to System.String).
    fn translate_query_with_resolution(
        &self,
        query: &ast::Query,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        // Extract alias types from source expressions
        let alias_types = self.extract_query_alias_types(query);

        translator.translate_query_with_types(
            query,
            |t, e| self.translate_expr_recursive(e, t),
            None,
            &alias_types,
        )
    }

    /// Extract alias-to-type mappings from query source expressions.
    ///
    /// When a source expression is a Retrieve, extracts the FHIR type name
    /// (e.g., "Observation" from `[Observation]`).
    fn extract_query_alias_types(
        &self,
        query: &ast::Query,
    ) -> std::collections::HashMap<String, String> {
        let mut alias_types = std::collections::HashMap::new();

        for source in &query.sources {
            if let Some(type_name) = self.extract_type_from_expression(&source.expression) {
                alias_types.insert(source.alias.clone(), type_name);
            }
        }

        alias_types
    }

    /// Extract the FHIR type name from an expression if it's a Retrieve.
    fn extract_type_from_expression(&self, expr: &ast::Expression) -> Option<String> {
        match expr {
            ast::Expression::Retrieve(retrieve) => {
                // Extract type name from Retrieve's data_type
                match &retrieve.data_type {
                    ast::TypeSpecifier::Named(named) => Some(named.name.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Translate a retrieve expression with symbol resolution and model info.
    ///
    /// This method enhances the basic retrieve translation with:
    /// - `templateId` from model info (profile URL/identifier)
    /// - `codeProperty` from model info (primary code path) if not specified
    /// - `codeComparator` set to "~" (equivalent) when codes are present
    fn translate_retrieve_with_resolution(
        &self,
        retrieve: &ast::Retrieve,
        translator: &mut crate::translator::ExpressionTranslator,
    ) -> elm::Expression {
        let element = translator.element_fields();

        // Convert the data type specifier to a QName
        let data_type = crate::translator::type_specifier_to_qname(&retrieve.data_type);

        // Try to look up model info to get templateId and codeProperty
        let (template_id, model_code_property) =
            self.lookup_retrieve_model_info(&retrieve.data_type);

        // Translate context expression if present
        let context = retrieve
            .context
            .as_ref()
            .map(|c| Box::new(self.translate_expr_recursive(c, translator)));

        // Translate codes expression if present
        // For ValueSetRef, use directly (it's already a list-like reference)
        // For CodeRef, wrap in ToList (single code needs to be converted to list)
        let (codes, is_valueset_ref) = match retrieve.codes.as_ref() {
            Some(c) => {
                let translated = self.translate_expr_recursive(c, translator);
                match &translated {
                    elm::Expression::ValueSetRef(_) => {
                        // ValueSetRef is used directly; codeComparator should be "in"
                        (Some(Box::new(translated)), true)
                    }
                    elm::Expression::CodeRef(_) => {
                        // CodeRef needs ToList wrapping; codeComparator should be "~"
                        let wrapped = elm::Expression::ToList(elm::UnaryExpression {
                            element: translator.element_fields(),
                            operand: Some(Box::new(translated)),
                            signature: Vec::new(),
                        });
                        (Some(Box::new(wrapped)), false)
                    }
                    _ => (Some(Box::new(translated)), false),
                }
            }
            None => (None, false),
        };

        // Translate date range expression if present
        let date_range = retrieve
            .date_range
            .as_ref()
            .map(|d| Box::new(self.translate_expr_recursive(d, translator)));

        // Determine code property: use explicit from CQL, or fall back to model default
        let code_property = retrieve.code_path.clone().or(model_code_property);

        // Set codeComparator based on codes type:
        // - "in" for ValueSetRef (membership test)
        // - "~" for CodeRef or other expressions (equivalent)
        let code_comparator = if codes.is_some() {
            Some(if is_valueset_ref { "in" } else { "~" }.to_string())
        } else {
            None
        };

        elm::Expression::Retrieve(elm::Retrieve {
            element,
            data_type: Some(data_type),
            template_id,
            context,
            code_property,
            code_comparator,
            codes,
            date_property: retrieve.date_path.clone(),
            date_range,
            // Include empty arrays for conformance with reference translator
            include: Vec::new(),
            code_filter: Vec::new(),
            date_filter: Vec::new(),
            other_filter: Vec::new(),
        })
    }

    /// Look up model info for a retrieve data type.
    ///
    /// Returns (template_id, primary_code_path) if model info is available.
    fn lookup_retrieve_model_info(
        &self,
        data_type: &ast::TypeSpecifier,
    ) -> (Option<String>, Option<String>) {
        // Extract model name and type name from the type specifier
        let (model_name, type_name) = match data_type {
            ast::TypeSpecifier::Named(named) => {
                let model = named.namespace.as_deref().unwrap_or("FHIR");
                (model, named.name.as_str())
            }
            _ => return (None, None),
        };

        // Look up the model provider
        let provider = match self.model_provider {
            Some(p) => p,
            None => return (None, None),
        };

        // Resolve the class from model info
        let class_info = match provider.resolve_class(model_name, None, type_name) {
            Some(ci) => ci,
            None => return (None, None),
        };

        // Extract template_id (profile URL/identifier) and primary_code_path
        let template_id = class_info.identifier.clone();
        let code_property = class_info.primary_code_path.clone();

        (template_id, code_property)
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

    // ========================================================================
    // Library Building (Phase 6.4d)
    // ========================================================================

    /// Build a complete ELM Library from an AST Library.
    ///
    /// This is the main entry point for translating a complete CQL library
    /// to ELM. It translates all definitions and assembles them into an
    /// ELM Library structure.
    ///
    /// # Example
    ///
    /// ```
    /// use rh_cql::parser::CqlParser;
    /// use rh_cql::builder::LibraryBuilder;
    ///
    /// let source = r#"
    ///     library TestLib version '1.0'
    ///     using FHIR version '4.0.1'
    ///     context Patient
    ///     define X: true
    /// "#;
    ///
    /// let ast = CqlParser::new().parse(source).unwrap();
    /// let mut builder = LibraryBuilder::new();
    /// let elm_library = builder.build(&ast);
    /// assert_eq!(elm_library.identifier.as_ref().unwrap().id, Some("TestLib".to_string()));
    /// ```
    pub fn build(&mut self, ast_library: &ast::Library) -> elm::Library {
        use crate::translator::ExpressionTranslator;

        let mut translator = ExpressionTranslator::new();

        // Build library identifier
        let identifier = ast_library
            .identifier
            .as_ref()
            .map(|id| elm::VersionedIdentifier {
                id: Some(id.name.clone()),
                system: None,
                version: id.version.clone(),
            });

        // Set library info
        if let Some(id) = &ast_library.identifier {
            self.library_name = Some(id.name.clone());
            self.library_version = id.version.clone();
        }

        // ========================================
        // Phase 1: Populate symbol table
        // ========================================
        // Register all definitions in the symbol table BEFORE translating
        // expression bodies, so identifier references can be properly resolved.

        // Register code systems
        for cs in &ast_library.codesystems {
            let symbol = Symbol::new(&cs.name, SymbolKind::CodeSystem);
            self.define_library_symbol(symbol);
        }

        // Register value sets
        for vs in &ast_library.valuesets {
            let symbol = Symbol::new(&vs.name, SymbolKind::ValueSet);
            self.define_library_symbol(symbol);
        }

        // Register codes
        for code in &ast_library.codes {
            let symbol = Symbol::new(&code.name, SymbolKind::Code);
            self.define_library_symbol(symbol);
        }

        // Register concepts
        for concept in &ast_library.concepts {
            let symbol = Symbol::new(&concept.name, SymbolKind::Concept);
            self.define_library_symbol(symbol);
        }

        // Register parameters
        for param in &ast_library.parameters {
            let symbol = Symbol::new(&param.name, SymbolKind::Parameter);
            self.define_library_symbol(symbol);
        }

        // Register contexts
        for ctx in &ast_library.contexts {
            let symbol = Symbol::new(&ctx.name, SymbolKind::Context);
            self.define_library_symbol(symbol);
        }

        // Register expression definitions (forward declarations for mutual recursion)
        for stmt in &ast_library.statements {
            if let ast::Statement::ExpressionDef(expr_def) = stmt {
                let symbol = Symbol::new(&expr_def.name, SymbolKind::Expression);
                self.define_library_symbol(symbol);
            }
        }

        // ========================================
        // Phase 2: Translate definitions
        // ========================================

        // Translate using definitions with implicit System using
        let usings = {
            // Always include the implicit System using first
            let system_using = elm::UsingDef {
                local_identifier: Some("System".to_string()),
                uri: Some("urn:hl7-org:elm-types:r1".to_string()),
                version: None,
            };
            let mut defs = vec![system_using];

            // Add explicit usings from the source
            defs.extend(
                ast_library
                    .usings
                    .iter()
                    .map(|u| translator.translate_using_def(u)),
            );
            Some(elm::UsingDefs { defs })
        };

        // Translate include definitions and register with conversion context
        let includes = if ast_library.includes.is_empty() {
            None
        } else {
            let defs: Vec<elm::IncludeDef> = ast_library
                .includes
                .iter()
                .map(|i| {
                    // Register the include with the conversion context
                    // The alias (or path if no alias) is the local reference name
                    let local_name = i.alias.as_ref().unwrap_or(&i.path);
                    self.conversion_context
                        .add_included_library(local_name, &i.path);

                    translator.translate_include_def(i)
                })
                .collect();
            Some(elm::IncludeDefs { defs })
        };

        // Translate code system definitions
        let code_systems = if ast_library.codesystems.is_empty() {
            None
        } else {
            let defs: Vec<elm::CodeSystemDef> = ast_library
                .codesystems
                .iter()
                .map(|cs| translator.translate_codesystem_def(cs))
                .collect();
            Some(elm::CodeSystemDefs { defs })
        };

        // Translate value set definitions
        let value_sets = if ast_library.valuesets.is_empty() {
            None
        } else {
            let defs: Vec<elm::ValueSetDef> = ast_library
                .valuesets
                .iter()
                .map(|vs| translator.translate_valueset_def(vs))
                .collect();
            Some(elm::ValueSetDefs { defs })
        };

        // Translate code definitions
        let codes = if ast_library.codes.is_empty() {
            None
        } else {
            let defs: Vec<elm::CodeDef> = ast_library
                .codes
                .iter()
                .map(|c| translator.translate_code_def(c))
                .collect();
            Some(elm::CodeDefs { defs })
        };

        // Translate concept definitions
        let concepts = if ast_library.concepts.is_empty() {
            None
        } else {
            let defs: Vec<elm::ConceptDef> = ast_library
                .concepts
                .iter()
                .map(|c| translator.translate_concept_def(c))
                .collect();
            Some(elm::ConceptDefs { defs })
        };

        // Translate parameter definitions
        let parameters = if ast_library.parameters.is_empty() {
            None
        } else {
            let defs: Vec<elm::ParameterDef> = ast_library
                .parameters
                .iter()
                .map(|p| translator.translate_parameter_def(p))
                .collect();
            Some(elm::ParameterDefs { defs })
        };

        // Translate context definitions
        let contexts = if ast_library.contexts.is_empty() {
            None
        } else {
            let defs: Vec<elm::ContextDef> = ast_library
                .contexts
                .iter()
                .map(|c| translator.translate_context_def(c))
                .collect();
            Some(elm::ContextDefs { defs })
        };

        // Determine current context for statements
        let current_context = ast_library.contexts.last().map(|c| c.name.as_str());

        // Translate statements (expression and function definitions)
        // Use resolution-aware translation for expression bodies
        let statements = if ast_library.statements.is_empty() {
            None
        } else {
            let mut expression_defs = Vec::new();

            // Generate implicit context expression definition (e.g., Patient definition for Patient context)
            // This matches reference translator behavior which generates:
            // define "Patient": SingletonFrom([Patient])
            if let Some(ctx_name) = current_context {
                // Create Retrieve for the context type
                let retrieve_local_id = translator.next_local_id();
                let retrieve = elm::Retrieve {
                    element: elm::ElementFields {
                        local_id: retrieve_local_id.clone(),
                        locator: None,
                        result_type_name: None,
                        result_type_specifier: None,
                    },
                    data_type: Some(format!("{{http://hl7.org/fhir}}{ctx_name}")),
                    template_id: Some(format!(
                        "http://hl7.org/fhir/StructureDefinition/{ctx_name}"
                    )),
                    context: None,
                    code_property: None,
                    code_comparator: None,
                    codes: None,
                    date_property: None,
                    date_range: None,
                    include: Vec::new(),
                    code_filter: Vec::new(),
                    date_filter: Vec::new(),
                    other_filter: Vec::new(),
                };

                // Wrap in SingletonFrom
                let singleton_local_id = translator.next_local_id();
                let singleton = elm::UnaryExpression {
                    element: elm::ElementFields {
                        local_id: singleton_local_id,
                        locator: None,
                        result_type_name: None,
                        result_type_specifier: None,
                    },
                    signature: Vec::new(),
                    operand: Some(Box::new(elm::Expression::Retrieve(retrieve))),
                };

                // Create ExpressionDef for the context
                let def_local_id = translator.next_local_id();

                // Generate annotation if enabled
                let annotation = if self.options.annotations_enabled() {
                    if let Some(ref id) = def_local_id {
                        vec![elm::Annotation::source(elm::Narrative::with_ref(
                            id.clone(),
                        ))]
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };

                expression_defs.push(elm::ExpressionDef {
                    local_id: def_local_id,
                    locator: None,
                    name: Some(ctx_name.to_string()),
                    context: Some(ctx_name.to_string()),
                    access_level: None, // Implicit definition has no explicit access modifier
                    result_type_name: None,
                    result_type_specifier: None,
                    expression: Some(Box::new(elm::Expression::SingletonFrom(singleton))),
                    annotation,
                });
            }

            for stmt in &ast_library.statements {
                match stmt {
                    ast::Statement::ExpressionDef(expr_def) => {
                        // Try to infer the result type before translation
                        if let Some(fhir_type) = self.infer_fhir_result_type(&expr_def.expression) {
                            self.register_expression_result_type(&expr_def.name, &fhir_type);
                        }

                        // Translate expression body with symbol resolution
                        let expression = Some(Box::new(self.translate_expression_with_resolution(
                            &expr_def.expression,
                            &mut translator,
                        )));

                        let access_level = match expr_def.access {
                            ast::AccessModifier::Public => Some(elm::AccessModifier::Public),
                            ast::AccessModifier::Private => Some(elm::AccessModifier::Private),
                        };

                        // Generate locator if enabled
                        let locator = if self.options.locators_enabled() {
                            expr_def
                                .location
                                .as_ref()
                                .map(|loc| format!("{}:{}", loc.line, loc.column))
                        } else {
                            None
                        };

                        // Generate localId (always needed for annotations)
                        let local_id = translator.next_local_id();

                        // Generate annotation if enabled
                        let annotation = if self.options.annotations_enabled() {
                            if let Some(ref id) = local_id {
                                vec![elm::Annotation::source(elm::Narrative::with_ref(
                                    id.clone(),
                                ))]
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        };

                        // Context defaults to "Unfiltered" when no explicit context is defined
                        let context = Some(
                            current_context
                                .map(String::from)
                                .unwrap_or_else(|| "Unfiltered".to_string()),
                        );

                        expression_defs.push(elm::ExpressionDef {
                            local_id,
                            locator,
                            name: Some(expr_def.name.clone()),
                            context,
                            access_level,
                            result_type_name: None,
                            result_type_specifier: None,
                            expression,
                            annotation,
                        });
                    }
                    ast::Statement::FunctionDef(func_def) => {
                        // Functions are translated separately
                        // For now, we skip them as ELM Library.statements only holds ExpressionDefs
                        // In a full implementation, functions would go in a separate field
                        let _ = translator.translate_function_def(func_def, |t, e| {
                            self.translate_expr_recursive(e, t)
                        });
                    }
                }
            }

            if expression_defs.is_empty() {
                None
            } else {
                Some(elm::ExpressionDefs {
                    defs: expression_defs,
                })
            }
        };

        // Get localId for the library (Library extends Element)
        let local_id = translator.next_local_id();

        elm::Library {
            local_id,
            identifier,
            schema_identifier: Some(elm::VersionedIdentifier {
                id: Some("urn:hl7-org:elm".to_string()),
                system: None,
                version: Some("r1".to_string()),
            }),
            usings,
            includes,
            parameters,
            code_systems,
            value_sets,
            codes,
            concepts,
            contexts,
            statements,
            annotation: Vec::new(),
        }
    }

    /// Build a library with compiler options.
    ///
    /// This variant allows specifying compiler options that control
    /// translation behavior.
    pub fn build_with_options(
        &mut self,
        ast_library: &ast::Library,
        options: &crate::options::CompilerOptions,
    ) -> elm::Library {
        use crate::translator::ExpressionTranslator;

        let mut translator = ExpressionTranslator::with_options(options.clone());

        // Build library identifier
        let identifier = ast_library
            .identifier
            .as_ref()
            .map(|id| elm::VersionedIdentifier {
                id: Some(id.name.clone()),
                system: None,
                version: id.version.clone(),
            });

        // Set library info
        if let Some(id) = &ast_library.identifier {
            self.library_name = Some(id.name.clone());
            self.library_version = id.version.clone();
        }

        // Translate using definitions with implicit System using
        let usings = {
            // Always include the implicit System using first
            let system_using = elm::UsingDef {
                local_identifier: Some("System".to_string()),
                uri: Some("urn:hl7-org:elm-types:r1".to_string()),
                version: None,
            };
            let mut defs = vec![system_using];

            // Add explicit usings from the source
            defs.extend(
                ast_library
                    .usings
                    .iter()
                    .map(|u| translator.translate_using_def(u)),
            );
            Some(elm::UsingDefs { defs })
        };

        // Translate include definitions and register with conversion context
        let includes = if ast_library.includes.is_empty() {
            None
        } else {
            let defs: Vec<elm::IncludeDef> = ast_library
                .includes
                .iter()
                .map(|i| {
                    // Register the include with the conversion context
                    // The alias (or path if no alias) is the local reference name
                    let local_name = i.alias.as_ref().unwrap_or(&i.path);
                    self.conversion_context
                        .add_included_library(local_name, &i.path);

                    translator.translate_include_def(i)
                })
                .collect();
            Some(elm::IncludeDefs { defs })
        };

        // Translate code system definitions
        let code_systems = if ast_library.codesystems.is_empty() {
            None
        } else {
            let defs: Vec<elm::CodeSystemDef> = ast_library
                .codesystems
                .iter()
                .map(|cs| translator.translate_codesystem_def(cs))
                .collect();
            Some(elm::CodeSystemDefs { defs })
        };

        // Translate value set definitions
        let value_sets = if ast_library.valuesets.is_empty() {
            None
        } else {
            let defs: Vec<elm::ValueSetDef> = ast_library
                .valuesets
                .iter()
                .map(|vs| translator.translate_valueset_def(vs))
                .collect();
            Some(elm::ValueSetDefs { defs })
        };

        // Translate code definitions
        let codes = if ast_library.codes.is_empty() {
            None
        } else {
            let defs: Vec<elm::CodeDef> = ast_library
                .codes
                .iter()
                .map(|c| translator.translate_code_def(c))
                .collect();
            Some(elm::CodeDefs { defs })
        };

        // Translate concept definitions
        let concepts = if ast_library.concepts.is_empty() {
            None
        } else {
            let defs: Vec<elm::ConceptDef> = ast_library
                .concepts
                .iter()
                .map(|c| translator.translate_concept_def(c))
                .collect();
            Some(elm::ConceptDefs { defs })
        };

        // Translate parameter definitions
        let parameters = if ast_library.parameters.is_empty() {
            None
        } else {
            let defs: Vec<elm::ParameterDef> = ast_library
                .parameters
                .iter()
                .map(|p| translator.translate_parameter_def(p))
                .collect();
            Some(elm::ParameterDefs { defs })
        };

        // Translate context definitions
        let contexts = if ast_library.contexts.is_empty() {
            None
        } else {
            let defs: Vec<elm::ContextDef> = ast_library
                .contexts
                .iter()
                .map(|c| translator.translate_context_def(c))
                .collect();
            Some(elm::ContextDefs { defs })
        };

        // Determine current context for statements
        let current_context = ast_library.contexts.last().map(|c| c.name.as_str());

        // Translate statements (expression and function definitions)
        let statements = if ast_library.statements.is_empty() {
            None
        } else {
            let mut expression_defs = Vec::new();

            for stmt in &ast_library.statements {
                match stmt {
                    ast::Statement::ExpressionDef(expr_def) => {
                        expression_defs
                            .push(translator.translate_expression_def(expr_def, current_context));
                    }
                    ast::Statement::FunctionDef(func_def) => {
                        let _ = translator
                            .translate_function_def(func_def, |t, e| t.translate_expression(e));
                    }
                }
            }

            if expression_defs.is_empty() {
                None
            } else {
                Some(elm::ExpressionDefs {
                    defs: expression_defs,
                })
            }
        };

        // Get localId for the library (Library extends Element)
        let local_id = translator.next_local_id();

        elm::Library {
            local_id,
            identifier,
            schema_identifier: Some(elm::VersionedIdentifier {
                id: Some("urn:hl7-org:elm".to_string()),
                system: None,
                version: Some("r1".to_string()),
            }),
            usings,
            includes,
            parameters,
            code_systems,
            value_sets,
            codes,
            concepts,
            contexts,
            statements,
            annotation: Vec::new(),
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

    // ========================================================================
    // Build Tests (Phase 6.4d)
    // ========================================================================

    #[test]
    fn test_build_basic_library() {
        use crate::parser::CqlParser;

        let source = r#"
            library TestLib version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define X: true
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        // Check library identifier
        assert!(elm.identifier.is_some());
        let id = elm.identifier.unwrap();
        assert_eq!(id.id, Some("TestLib".to_string()));
        assert_eq!(id.version, Some("1.0".to_string()));

        // Check schema identifier
        assert!(elm.schema_identifier.is_some());
        let schema = elm.schema_identifier.unwrap();
        assert_eq!(schema.id, Some("urn:hl7-org:elm".to_string()));

        // Check usings - System is always first (implicit), then explicit usings
        assert!(elm.usings.is_some());
        let usings = elm.usings.unwrap();
        assert_eq!(usings.defs.len(), 2);
        assert_eq!(usings.defs[0].local_identifier, Some("System".to_string()));
        assert_eq!(
            usings.defs[0].uri,
            Some("urn:hl7-org:elm-types:r1".to_string())
        );
        assert_eq!(usings.defs[1].local_identifier, Some("FHIR".to_string()));

        // Check contexts
        assert!(elm.contexts.is_some());
        let contexts = elm.contexts.unwrap();
        assert_eq!(contexts.defs.len(), 1);
        assert_eq!(contexts.defs[0].name, Some("Patient".to_string()));

        // Check statements
        // Reference translator generates an implicit "Patient" definition for Patient context
        assert!(elm.statements.is_some());
        let statements = elm.statements.unwrap();
        assert_eq!(statements.defs.len(), 2); // Implicit Patient + X

        // First definition is implicit Patient
        assert_eq!(statements.defs[0].name, Some("Patient".to_string()));
        assert_eq!(statements.defs[0].context, Some("Patient".to_string()));

        // Second definition is our explicit X
        assert_eq!(statements.defs[1].name, Some("X".to_string()));
        assert_eq!(statements.defs[1].context, Some("Patient".to_string()));
    }

    #[test]
    fn test_build_library_with_terminology() {
        use crate::parser::CqlParser;

        let source = r#"
            library TermLib version '1.0'
            
            codesystem "LOINC": 'http://loinc.org'
            valueset "BP Codes": 'http://example.org/vs/bp'
            code "Systolic BP": '8480-6' from "LOINC" display 'Systolic blood pressure'
            concept "Blood Pressure": { "Systolic BP" } display 'Blood Pressure'
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        // Check code systems
        assert!(elm.code_systems.is_some());
        let code_systems = elm.code_systems.unwrap();
        assert_eq!(code_systems.defs.len(), 1);
        assert_eq!(code_systems.defs[0].name, Some("LOINC".to_string()));
        assert_eq!(
            code_systems.defs[0].id,
            Some("http://loinc.org".to_string())
        );

        // Check value sets
        assert!(elm.value_sets.is_some());
        let value_sets = elm.value_sets.unwrap();
        assert_eq!(value_sets.defs.len(), 1);
        assert_eq!(value_sets.defs[0].name, Some("BP Codes".to_string()));

        // Check codes
        assert!(elm.codes.is_some());
        let codes = elm.codes.unwrap();
        assert_eq!(codes.defs.len(), 1);
        assert_eq!(codes.defs[0].name, Some("Systolic BP".to_string()));
        assert_eq!(codes.defs[0].id, Some("8480-6".to_string()));

        // Check concepts
        assert!(elm.concepts.is_some());
        let concepts = elm.concepts.unwrap();
        assert_eq!(concepts.defs.len(), 1);
        assert_eq!(concepts.defs[0].name, Some("Blood Pressure".to_string()));
    }

    #[test]
    fn test_build_library_with_parameters() {
        use crate::parser::CqlParser;

        let source = r#"
            library ParamLib version '1.0'
            
            parameter "Measurement Period" Interval<DateTime>
            parameter "Max Count" Integer default 10
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        // Check parameters
        assert!(elm.parameters.is_some());
        let parameters = elm.parameters.unwrap();
        assert_eq!(parameters.defs.len(), 2);
        assert_eq!(
            parameters.defs[0].name,
            Some("Measurement Period".to_string())
        );
        assert_eq!(parameters.defs[1].name, Some("Max Count".to_string()));
        assert!(parameters.defs[1].default_value.is_some());
    }

    #[test]
    fn test_build_library_with_includes() {
        use crate::parser::CqlParser;

        let source = r#"
            library IncludeLib version '1.0'
            
            include FHIRHelpers version '4.0.1' called Helpers
            include CommonLib
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        // Check includes
        assert!(elm.includes.is_some());
        let includes = elm.includes.unwrap();
        assert_eq!(includes.defs.len(), 2);
        assert_eq!(
            includes.defs[0].local_identifier,
            Some("Helpers".to_string())
        );
        assert_eq!(includes.defs[0].path, Some("FHIRHelpers".to_string()));
        assert_eq!(
            includes.defs[1].local_identifier,
            Some("CommonLib".to_string())
        );
    }

    #[test]
    fn test_build_empty_library() {
        use crate::parser::CqlParser;

        let source = "library EmptyLib version '1.0'";

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        assert!(elm.identifier.is_some());
        // System using is always implicitly added
        assert!(elm.usings.is_some());
        let usings = elm.usings.unwrap();
        assert_eq!(usings.defs.len(), 1);
        assert_eq!(usings.defs[0].local_identifier, Some("System".to_string()));
        assert_eq!(
            usings.defs[0].uri,
            Some("urn:hl7-org:elm-types:r1".to_string())
        );
        assert!(elm.includes.is_none());
        assert!(elm.parameters.is_none());
        assert!(elm.code_systems.is_none());
        assert!(elm.value_sets.is_none());
        assert!(elm.codes.is_none());
        assert!(elm.concepts.is_none());
        assert!(elm.contexts.is_none());
        assert!(elm.statements.is_none());
    }

    #[test]
    fn test_build_library_multiple_expressions() {
        use crate::parser::CqlParser;

        let source = r#"
            library MultiExpr version '1.0'
            using FHIR version '4.0.1'
            context Patient
            
            define "Is Adult": AgeInYears() >= 18
            define "Is Minor": not "Is Adult"
            define "Helper": true
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        assert!(elm.statements.is_some());
        let statements = elm.statements.unwrap();
        // 4 = implicit Patient + 3 explicit definitions
        assert_eq!(statements.defs.len(), 4);

        // All should have Patient context
        for def in &statements.defs {
            assert_eq!(def.context, Some("Patient".to_string()));
        }

        // Check names (first is implicit Patient)
        assert_eq!(statements.defs[0].name, Some("Patient".to_string()));
        assert_eq!(statements.defs[1].name, Some("Is Adult".to_string()));
        assert_eq!(statements.defs[2].name, Some("Is Minor".to_string()));
        assert_eq!(statements.defs[3].name, Some("Helper".to_string()));
    }

    // ========================================================================
    // Phase 6.5 High Priority Tests - Symbol Resolution
    // ========================================================================

    #[test]
    fn test_expression_ref_resolution() {
        use crate::parser::CqlParser;

        let source = r#"
            library RefTest version '1.0'
            define "Base": 42
            define "Reference": "Base"
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        assert_eq!(statements.defs.len(), 2);

        // The second expression should reference the first via ExpressionRef
        let ref_expr = &statements.defs[1].expression;
        assert!(ref_expr.is_some());

        // Check it's an ExpressionRef, not IdentifierRef
        let json = serde_json::to_string(ref_expr.as_ref().unwrap()).unwrap();
        assert!(
            json.contains(r#""type":"ExpressionRef""#),
            "Expected ExpressionRef, got: {json}"
        );
        assert!(json.contains(r#""name":"Base""#));
    }

    #[test]
    fn test_valueset_ref_resolution() {
        use crate::parser::CqlParser;

        let source = r#"
            library VSRefTest version '1.0'
            using FHIR version '4.0.1'
            valueset "My VS": 'http://example.org/vs'
            context Patient
            define "Test": [Condition: "My VS"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is Test
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Should have ValueSetRef in the Retrieve codes
        assert!(
            json.contains(r#""type":"ValueSetRef""#),
            "Expected ValueSetRef in: {json}"
        );
        assert!(json.contains(r#""name":"My VS""#));
    }

    #[test]
    fn test_code_ref_resolution() {
        use crate::parser::CqlParser;

        let source = r#"
            library CodeRefTest version '1.0'
            using FHIR version '4.0.1'
            codesystem "LOINC": 'http://loinc.org'
            code "BP Code": '55284-4' from "LOINC"
            context Patient
            define "Test": [Observation: "BP Code"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is Test
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Should have CodeRef in the Retrieve codes
        assert!(
            json.contains(r#""type":"CodeRef""#),
            "Expected CodeRef in: {json}"
        );
        assert!(json.contains(r#""name":"BP Code""#));
    }

    #[test]
    fn test_query_alias_scope_attribute() {
        use crate::parser::CqlParser;

        let source = r#"
            library QueryAliasTest version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define "Test": 
                [Condition] C where C.id = 'test'
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is Test
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Query alias property access should use scope attribute, not source+IdentifierRef
        // This matches the reference Java translator output
        assert!(
            json.contains(r#""scope":"C""#),
            "Expected scope attribute for query alias in: {json}"
        );
        assert!(json.contains(r#""path":"id""#));
        // Should NOT have IdentifierRef as source for the property
        assert!(
            !json.contains(r#""source":{"type":"IdentifierRef""#),
            "Should use scope, not source+IdentifierRef: {json}"
        );
    }

    #[test]
    fn test_mixed_references() {
        use crate::parser::CqlParser;

        let source = r#"
            library MixedRefTest version '1.0'
            using FHIR version '4.0.1'
            valueset "Conditions VS": 'http://example.org/vs'
            codesystem "SNOMED": 'http://snomed.info/sct'
            code "Diabetes": '73211009' from "SNOMED"
            context Patient
            define "HasCondition": exists [Condition: "Conditions VS"]
            define "Final": "HasCondition" and exists [Observation: "Diabetes"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();

        // defs[0] is implicit Patient, defs[1] is HasCondition, defs[2] is Final
        // First explicit expression should have ValueSetRef
        let expr1 = statements.defs[1].expression.as_ref().unwrap();
        let json1 = serde_json::to_string(expr1).unwrap();
        assert!(json1.contains(r#""type":"ValueSetRef""#));

        // Second explicit expression should have ExpressionRef and CodeRef
        let expr2 = statements.defs[2].expression.as_ref().unwrap();
        let json2 = serde_json::to_string(expr2).unwrap();
        assert!(json2.contains(r#""type":"ExpressionRef""#));
        assert!(json2.contains(r#""type":"CodeRef""#));
    }

    // ========================================================================
    // Phase 6.5 High Priority Tests - Retrieve Enhancement
    // ========================================================================

    #[test]
    fn test_retrieve_with_model_info() {
        use crate::parser::CqlParser;
        use crate::provider::fhir_r4_provider;

        let source = r#"
            library RetrieveTest version '1.0'
            using FHIR version '4.0.1'
            valueset "Test VS": 'http://example.org/vs'
            context Patient
            define "Conditions": [Condition: "Test VS"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let provider = fhir_r4_provider();
        let mut builder = LibraryBuilder::new();
        builder.set_model_provider(&provider);
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is Conditions
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Should have templateId from model info
        assert!(
            json.contains(r#""templateId":"http://hl7.org/fhir/StructureDefinition/Condition""#),
            "Expected templateId in: {json}"
        );

        // Should have codeProperty from model info
        assert!(
            json.contains(r#""codeProperty":"code""#),
            "Expected codeProperty in: {json}"
        );

        // Should have codeComparator "in" when using ValueSetRef
        assert!(
            json.contains(r#""codeComparator":"in""#),
            "Expected codeComparator 'in' for ValueSetRef in: {json}"
        );
    }

    #[test]
    fn test_retrieve_observation_model_info() {
        use crate::parser::CqlParser;
        use crate::provider::fhir_r4_provider;

        let source = r#"
            library RetrieveObsTest version '1.0'
            using FHIR version '4.0.1'
            codesystem "LOINC": 'http://loinc.org'
            code "BP": '55284-4' from "LOINC"
            context Patient
            define "BPs": [Observation: "BP"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let provider = fhir_r4_provider();
        let mut builder = LibraryBuilder::new();
        builder.set_model_provider(&provider);
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is BPs
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Observation should have its own templateId
        assert!(
            json.contains(r#""templateId":"http://hl7.org/fhir/StructureDefinition/Observation""#)
        );
        assert!(json.contains(r#""codeProperty":"code""#));
    }

    #[test]
    fn test_retrieve_without_codes_no_comparator() {
        use crate::parser::CqlParser;
        use crate::provider::fhir_r4_provider;

        let source = r#"
            library RetrieveNoCodes version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define "AllConditions": [Condition]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let provider = fhir_r4_provider();
        let mut builder = LibraryBuilder::new();
        builder.set_model_provider(&provider);
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is AllConditions
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Should have templateId even without codes
        assert!(
            json.contains(r#""templateId":"http://hl7.org/fhir/StructureDefinition/Condition""#)
        );

        // Should NOT have codeComparator when no codes
        assert!(
            !json.contains(r#""codeComparator""#),
            "Should not have codeComparator without codes: {json}"
        );
    }

    #[test]
    fn test_retrieve_without_model_provider() {
        use crate::parser::CqlParser;

        let source = r#"
            library RetrieveNoModel version '1.0'
            using FHIR version '4.0.1'
            valueset "Test VS": 'http://example.org/vs'
            context Patient
            define "Conditions": [Condition: "Test VS"]
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        // Don't set model provider
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        // defs[0] is implicit Patient, defs[1] is Conditions
        let expr = statements.defs[1].expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();

        // Should have codeComparator "in" for ValueSetRef
        assert!(json.contains(r#""codeComparator":"in""#));

        // But no templateId or codeProperty without model info
        assert!(!json.contains(r#""templateId""#));
    }

    #[test]
    fn test_enable_locators() {
        use crate::options::{CompilerOption, CompilerOptions};
        use crate::parser::CqlParser;

        let source = r#"
            library LocatorTest version '1.0'
            using FHIR version '4.0.1'
            define "TestExpr": 42
        "#;

        let ast = CqlParser::new().parse(source).unwrap();

        // With locators enabled (debug mode)
        let opts = CompilerOptions::debug();
        let mut builder = LibraryBuilder::new();
        builder.set_options(opts);
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        let locator = statements.defs[0].locator.as_ref();
        assert!(locator.is_some(), "Should have locator when enabled");
        assert!(
            locator.unwrap().contains(":"),
            "Locator should have line:column format"
        );

        // Without locators (explicitly disabled)
        let opts_no_loc = CompilerOptions::default().without_option(CompilerOption::EnableLocators);
        let mut builder2 = LibraryBuilder::new();
        builder2.set_options(opts_no_loc);
        let elm2 = builder2.build(&ast);

        let statements2 = elm2.statements.unwrap();
        let locator2 = statements2.defs[0].locator.as_ref();
        assert!(locator2.is_none(), "Should not have locator when disabled");
    }

    #[test]
    fn test_enable_annotations() {
        use crate::options::{CompilerOption, CompilerOptions};
        use crate::parser::CqlParser;

        let source = r#"
            library AnnotationTest version '1.0'
            using FHIR version '4.0.1'
            define "TestExpr": 42
        "#;

        let ast = CqlParser::new().parse(source).unwrap();

        // With annotations enabled (debug mode)
        let opts = CompilerOptions::debug();
        let mut builder = LibraryBuilder::new();
        builder.set_options(opts);
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();
        let def = &statements.defs[0];

        // Should have localId
        assert!(
            def.local_id.is_some(),
            "Should have localId when annotations enabled"
        );
        let local_id = def.local_id.as_ref().unwrap();

        // Should have annotation
        assert!(
            !def.annotation.is_empty(),
            "Should have annotation when enabled"
        );
        let annotation = &def.annotation[0];
        assert_eq!(annotation.tag_type, Some("Annotation".to_string()));

        // Annotation should have source with reference to localId
        let source = annotation.source.as_ref().expect("Should have source");
        assert_eq!(source.local_id_ref, Some(local_id.clone()));

        // Without annotations (explicitly disabled)
        let opts_no_annot =
            CompilerOptions::default().without_option(CompilerOption::EnableAnnotations);
        let mut builder2 = LibraryBuilder::new();
        builder2.set_options(opts_no_annot);
        let elm2 = builder2.build(&ast);

        let statements2 = elm2.statements.unwrap();
        let def2 = &statements2.defs[0];

        // Should not have annotation
        assert!(
            def2.annotation.is_empty(),
            "Should not have annotation when disabled"
        );
    }

    #[test]
    fn test_implicit_patient_definition() {
        use crate::parser::CqlParser;

        let source = r#"
            library ImplicitPatientTest version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define "TestExpr": true
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();

        // First definition should be implicit Patient
        assert_eq!(statements.defs.len(), 2);
        let patient_def = &statements.defs[0];
        assert_eq!(patient_def.name, Some("Patient".to_string()));
        assert_eq!(patient_def.context, Some("Patient".to_string()));
        assert!(
            patient_def.access_level.is_none(),
            "Implicit def has no access level"
        );

        // Should have SingletonFrom([Patient]) expression
        let expr = patient_def.expression.as_ref().unwrap();
        let json = serde_json::to_string(expr).unwrap();
        assert!(json.contains(r#""type":"SingletonFrom""#));
        assert!(json.contains(r#""type":"Retrieve""#));
        assert!(json.contains(r#"{http://hl7.org/fhir}Patient"#));
        assert!(json.contains(r#""templateId":"http://hl7.org/fhir/StructureDefinition/Patient""#));

        // Second definition should be the explicit one
        let test_def = &statements.defs[1];
        assert_eq!(test_def.name, Some("TestExpr".to_string()));
    }

    #[test]
    fn test_no_implicit_definition_without_context() {
        use crate::parser::CqlParser;

        let source = r#"
            library NoContextTest version '1.0'
            define "TestExpr": true
        "#;

        let ast = CqlParser::new().parse(source).unwrap();
        let mut builder = LibraryBuilder::new();
        let elm = builder.build(&ast);

        let statements = elm.statements.unwrap();

        // Should only have one definition (no implicit Patient)
        assert_eq!(statements.defs.len(), 1);
        assert_eq!(statements.defs[0].name, Some("TestExpr".to_string()));
    }

    #[test]
    fn test_apply_implicit_conversion_disabled() {
        use crate::options::{CompilerOption, CompilerOptions};
        use crate::translator::ExpressionTranslator;

        // Create builder with implicit conversions disabled
        let opts = CompilerOptions::new().with_option(CompilerOption::DisableImplicitConversions);
        let mut builder = LibraryBuilder::new();
        builder.set_options(opts);

        // Set up a conversion registry with a conversion
        let mut registry = ConversionRegistry::new();
        registry.register(crate::conversion::ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });
        builder.set_conversion_registry(registry);
        builder.register_conversion_library("FHIRHelpers", "FHIRHelpers");

        let mut translator = ExpressionTranslator::new();

        // Create a test operand
        let operand = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let fhir_coding = DataType::model("FHIR", "Coding");
        let system_code = DataType::system(SystemType::Code);

        // With conversions disabled, should NOT apply conversion
        let (result, was_converted) =
            builder.apply_implicit_conversion(operand, &fhir_coding, &system_code, &mut translator);

        assert!(!was_converted, "Should not convert when disabled");
        assert!(matches!(result, elm::Expression::Literal(_)));
    }

    #[test]
    fn test_apply_implicit_conversion_strict_mode() {
        use crate::options::{CompilerOption, CompilerOptions};
        use crate::translator::ExpressionTranslator;

        // Create builder with strict conversion library check
        let opts = CompilerOptions::new().with_option(CompilerOption::StrictConversionLibraryCheck);
        let mut builder = LibraryBuilder::new();
        builder.set_options(opts);

        // Set up a conversion registry but DON'T register FHIRHelpers
        let mut registry = ConversionRegistry::new();
        registry.register(crate::conversion::ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });
        builder.set_conversion_registry(registry);
        // Note: NOT registering FHIRHelpers library

        let mut translator = ExpressionTranslator::new();

        let operand = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let fhir_coding = DataType::model("FHIR", "Coding");
        let system_code = DataType::system(SystemType::Code);

        // With strict mode, should emit ERROR when library not included
        let (result, was_converted) =
            builder.apply_implicit_conversion(operand, &fhir_coding, &system_code, &mut translator);

        assert!(!was_converted, "Should not convert when library missing");
        assert!(matches!(result, elm::Expression::Literal(_)));

        // Should have an error in strict mode
        assert!(
            !builder.errors.is_empty(),
            "Should have error in strict mode"
        );
        let error_msg = builder.errors[0].to_string();
        assert!(
            error_msg.contains("FHIRHelpers"),
            "Error should mention FHIRHelpers: {error_msg}"
        );
    }

    #[test]
    fn test_apply_implicit_conversion_warning_mode() {
        use crate::options::CompilerOptions;
        use crate::translator::ExpressionTranslator;

        // Create builder with default options (warnings, not errors)
        let mut builder = LibraryBuilder::new();
        builder.set_options(CompilerOptions::default());

        // Set up a conversion registry but DON'T register FHIRHelpers
        let mut registry = ConversionRegistry::new();
        registry.register(crate::conversion::ConversionEntry {
            from_type: "FHIR.Coding".to_string(),
            to_type: "System.Code".to_string(),
            function_name: "FHIRHelpers.ToCode".to_string(),
            library_name: Some("FHIRHelpers".to_string()),
            function_simple_name: "ToCode".to_string(),
        });
        builder.set_conversion_registry(registry);
        // Note: NOT registering FHIRHelpers library

        let mut translator = ExpressionTranslator::new();

        let operand = elm::Expression::Literal(elm::Literal {
            element: elm::ElementFields::default(),
            value_type: None,
            value: Some("test".to_string()),
        });

        let fhir_coding = DataType::model("FHIR", "Coding");
        let system_code = DataType::system(SystemType::Code);

        // With default mode, should emit WARNING when library not included
        let (result, was_converted) =
            builder.apply_implicit_conversion(operand, &fhir_coding, &system_code, &mut translator);

        assert!(!was_converted, "Should not convert when library missing");
        assert!(matches!(result, elm::Expression::Literal(_)));

        // Should NOT have an error (it's a warning)
        assert!(
            builder.errors.is_empty(),
            "Should not have error in warning mode"
        );

        // Should have a warning
        assert!(
            !builder.warnings.is_empty(),
            "Should have warning when library missing"
        );
        assert!(
            builder.warnings[0].contains("FHIRHelpers"),
            "Warning should mention FHIRHelpers"
        );
    }
}
