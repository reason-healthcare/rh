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
//! - [`translator`]: Expression translation from CQL AST to ELM (deprecated — use `emit`)
//! - [`emit`]: Multi-stage ELM emitter (new preferred pipeline)
//! - [`eval`]: Runtime value model, three-valued logic, and eval engine
//! - [`semantics`]: Semantic analysis — `SemanticAnalyzer`, `TypedLibrary`, `ScopeManager`
//! - [`options`]: Compiler options for translation behavior
//! - [`output`]: ELM output generation (JSON serialization)
//! - [`reporting`]: Error reporting with source locations and severity levels

pub mod builder;
pub mod compiler;
pub mod conversion;
pub mod datatype;
pub mod elm;
pub mod emit;
pub mod eval;
pub mod error;
pub mod explain;
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
pub mod semantics;
pub mod sourcemap;
pub mod translator;
pub mod types;

// Primary public API - compile CQL to ELM
pub use compiler::{
    compile, compile_to_elm_with_sourcemap, compile_to_json, compile_with_model, explain_compile,
    explain_parse, validate, CompilationError, CompilationResult, SourceMapCompilationResult,
    ValidationResult,
};
pub use explain::explain_eval;

pub use builder::LibraryBuilder;
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
#[deprecated(
    since = "0.1.0",
    note = "Use the new three-stage pipeline: SemanticAnalyzer + ElmEmitter. \
            ExpressionTranslator will be removed in a future release."
)]
#[allow(deprecated)]
pub use translator::ExpressionTranslator;
#[allow(deprecated)]
pub use translator::{StatementTranslation, TranslatorError, TranslatorResult};

// New pipeline types — preferred public API
pub use emit::ElmEmitter;
pub use eval::value::{cql_equal, cql_equivalent, CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime, Value};
pub use eval::tvl::{tvl_and, tvl_implies, tvl_not, tvl_or, tvl_xor};
pub use semantics::analyzer::SemanticAnalyzer;
pub use semantics::scope::ScopeManager;
pub use semantics::typed_ast::{
    NodeId, SemanticMeta, SourceSpan, TypedExpression, TypedLibrary, TypedNode, TypedStatement,
};
pub use types::{TypeBuilder, TypeError, TypeInference, TypeResolver, TypeResult};

// Source-map public types
pub use sourcemap::{
    generate_doc_id, generate_elm_node_id, generate_mapping_id, Confidence, ElmNodeMeta,
    MappingRole, SourceDocument, SourceElmMapping, SourceMap,
};
