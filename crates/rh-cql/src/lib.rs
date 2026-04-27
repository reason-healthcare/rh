//! # rh-cql
//!
//! CQL (Clinical Quality Language) capabilities for the RH ecosystem.
//!
//! This crate provides:
//! - **Three-stage CQL compiler**: Parse (nom) → Semantic Analysis (Typed AST) → ELM Emission
//! - CQL/ELM execution against FHIR data via a pure-Rust evaluation engine
//! - ModelInfo type definitions for FHIR and custom data model resolution
//! - Source maps correlating CQL source positions to ELM IR nodes
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
//! ## Pipeline
//!
//! All public compile functions delegate to a shared internal pipeline:
//!
//! 1. **Parse** — `&str` → `ast::Library` via [`CqlParser`] (nom combinators)
//! 2. **Analyze** — `ast::Library` → [`TypedLibrary`] via [`SemanticAnalyzer`]
//! 3. **Emit** *(optional)* — [`TypedLibrary`] → `elm::Library` via [`ElmEmitter`]
//! 4. **Source map** *(optional)* — emitter side-channel during emit
//!
//! See [`ARCHITECTURE.md`](https://github.com/reason-healthcare/rh/blob/main/crates/rh-cql/ARCHITECTURE.md)
//! for full details.
//!
//! ## Modules
//!
//! ### Core pipeline
//! - [`compiler`]: Public compilation API (`compile`, `validate`, `compile_to_json`, …)
//! - [`parser`]: Stage 1 — CQL parser using nom combinators
//! - [`semantics`]: Stage 2 — `SemanticAnalyzer`, `TypedLibrary`, `ScopeManager`
//! - [`emit`]: Stage 3 — `ElmEmitter` (preferred ELM emission pipeline)
//! - [`sourcemap`]: Source-map types (`SourceMap`, `SourceElmMapping`)
//!
//! ### Supporting modules
//! - [`elm`]: ELM (Expression Logical Model) type definitions
//! - [`eval`]: Runtime value model, three-valued logic, and eval engine
//! - [`library`]: Library source providers and compiled library management
//! - [`modelinfo`]: ModelInfo type definitions for CQL data model resolution
//! - [`provider`]: Model information providers (in-memory, WASM-compatible)
//! - [`datatype`]: Internal DataType system for type checking
//! - [`types`]: Type resolution for CQL semantic analysis
//! - [`operators`]: Operator resolution for CQL semantic analysis
//! - [`options`]: Compiler options for translation behavior
//! - [`output`]: ELM output generation (JSON serialization)
//! - [`reporting`]: Error reporting with source locations and severity levels
//! - [`preprocessor`]: Preprocessor for extracting library info from AST

pub mod compiler;
pub mod conversion;
pub mod datatype;
pub mod elm;
pub mod emit;
pub mod error;
pub mod eval;
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
pub mod types;

// Primary public API - compile CQL to ELM
pub use compiler::{
    compile, compile_to_elm_with_sourcemap, compile_to_json, compile_with_libraries,
    compile_with_model, explain_compile, explain_parse, validate, CompilationContext,
    CompilationError, CompilationResult, CompileOutputWithLibs, SourceMapCompilationResult,
    ValidationResult,
};
pub use explain::explain_eval;

pub use conversion::{
    conversion_key_to_datatype, datatype_to_conversion_key, needs_conversion, wrap_in_conversion,
    ConversionContext, ConversionEntry, ConversionRegistry, ConversionResult,
};
pub use datatype::{DataType, SystemType, TupleElement};
pub use error::{CqlError, Result};
pub use library::{
    CompiledLibrary, CompositeLibrarySourceProvider, DefinitionRef, FileLibrarySourceProvider,
    FunctionRef, LibraryError, LibraryIdentifier, LibraryManager, LibraryResult, LibrarySource,
    LibrarySourceProvider, MemoryLibrarySourceProvider, PackageLibrarySourceProvider,
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
    CqlCompilerException, Diagnostic, DiagnosticCode, DiagnosticCollection, DiagnosticStage,
    ExceptionCollector, ExceptionType, Severity, SourceLocator,
};
// Pipeline types — public API
pub use emit::ElmEmitter;
pub use eval::context::{
    Clock, DataProvider, EvalContext, EvalContextBuilder, EvalError, FixedClock,
    InMemoryDataProvider, InMemoryTerminologyProvider, TerminologyProvider,
};
pub use eval::engine::{
    evaluate_elm, evaluate_elm_with_libraries, evaluate_elm_with_trace, TraceEvent,
};
pub use eval::tvl::{tvl_and, tvl_implies, tvl_not, tvl_or, tvl_xor};
pub use eval::value::{
    cql_equal, cql_equivalent, CqlCode, CqlConcept, CqlDate, CqlDateTime, CqlQuantity, CqlTime,
    Value,
};
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
