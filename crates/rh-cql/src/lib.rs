//! # rh-cql
//!
//! CQL (Clinical Quality Language) capabilities for the RH ecosystem.
//!
//! This crate provides:
//! - CQL-to-ELM translation
//! - CQL/ELM execution against FHIR data
//! - ModelInfo type definitions for data model resolution
//!
//! ## Status
//!
//! 🚧 Under development - API is not yet stable.
//!
//! ## Quick Start
//!
//! The simplest way to compile CQL to ELM:
//!
//! ```
//! use rh_cql::{compile, CompilerOptions};
//!
//! let source = r#"
//!     library Example version '1.0.0'
//!     define Greeting: 'Hello, CQL!'
//! "#;
//!
//! let result = compile(source, None).unwrap();
//! assert!(result.is_success());
//! println!("{}", result.to_json().unwrap());
//! ```
//!
//! ## Modules
//!
//! - [`compiler`]: Public compilation API (`compile`, `validate`, `compile_to_json`)
//! - [`elm`]: ELM (Expression Logical Model) type definitions
//! - [`modelinfo`]: ModelInfo type definitions for CQL data model resolution
//! - [`provider`]: Model information providers (in-memory, WASM-compatible)
//! - [`library`]: Library source providers and compiled library management
//! - [`datatype`]: Internal DataType system for type checking
//! - [`parser`]: CQL parser using nom combinators
//! - [`preprocessor`]: Preprocessor for extracting library info from AST
//! - [`builder`]: LibraryBuilder for semantic analysis and ELM generation
//! - [`types`]: Type resolution for CQL semantic analysis
//! - [`operators`]: Operator resolution for CQL semantic analysis
//! - [`translator`]: Expression translation from CQL AST to ELM
//! - [`options`]: Compiler options for translation behavior
//! - [`output`]: ELM output generation (JSON serialization)
//! - [`reporting`]: Error reporting with source locations and severity levels

pub mod builder;
pub mod compiler;
pub mod conversion;
pub mod datatype;
pub mod elm;
pub mod error;
pub mod library;
pub mod modelinfo;
mod modelinfo_xml;
pub mod operators;
pub mod options;
pub mod output;
pub mod parser;
pub mod preprocessor;
pub mod provider;
pub mod reporting;
pub mod translator;
pub mod types;

// Primary public API - compile CQL to ELM
pub use compiler::{
    compile, compile_to_json, compile_with_model, validate, CompilationError, CompilationResult,
    ValidationResult,
};

pub use builder::{
    BuilderError, BuilderResult, FunctionSignature, LibraryBuilder, ResolvedIdentifier, Scope,
    Symbol, SymbolKind,
};
pub use conversion::{
    conversion_key_to_datatype, datatype_to_conversion_key, needs_conversion, wrap_in_conversion,
    ConversionContext, ConversionEntry, ConversionRegistry, ConversionResult,
};
pub use datatype::{DataType, SystemType, TupleElement};
pub use error::{CqlError, Result};
pub use library::{
    CompiledLibrary, CompositeLibrarySourceProvider, DefinitionRef, FileLibrarySourceProvider,
    FunctionRef, LibraryError, LibraryIdentifier, LibraryManager, LibraryResult, LibrarySource,
    LibrarySourceProvider, MemoryLibrarySourceProvider,
};
pub use operators::{
    OperatorError, OperatorKind, OperatorResolver, OperatorResult, OperatorSignature,
    ResolvedOperator,
};
pub use options::{CompilerOption, CompilerOptions, ErrorSeverity, OutputFormat, SignatureLevel};
pub use output::{
    library_to_compact_json, library_to_json, library_to_json_with_options, ElmWriter, OutputError,
    TRANSLATOR_VERSION,
};
pub use parser::CqlParser;
pub use preprocessor::{
    CodeInfo, CodeSystemInfo, ConceptInfo, DefinitionKind, ExpressionInfo, FunctionInfo,
    LibraryDependency, LibraryInfo, ModelDependency, ParameterInfo, Preprocessor, ValueSetInfo,
};
pub use provider::{
    fhir_r4_model_info, fhir_r4_provider, fhir_r4_provider_from_package, get_default_packages_dir,
    get_package_dir, load_fhir_r4_modelinfo_from_package, load_modelinfo_from_package,
    MemoryModelInfoProvider, ModelInfoProvider,
};
pub use reporting::{
    CqlCompilerException, ExceptionCollector, ExceptionType, Severity, SourceLocator,
};
pub use translator::{
    ExpressionTranslator, StatementTranslation, TranslatorError, TranslatorResult,
};
pub use types::{TypeBuilder, TypeError, TypeInference, TypeResolver, TypeResult};
