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
//! ## Modules
//!
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

pub mod builder;
pub mod datatype;
pub mod elm;
pub mod error;
pub mod library;
pub mod modelinfo;
pub mod operators;
pub mod parser;
pub mod preprocessor;
pub mod provider;
pub mod translator;
pub mod types;

pub use builder::{
    BuilderError, BuilderResult, FunctionSignature, LibraryBuilder, ResolvedIdentifier, Scope,
    Symbol, SymbolKind,
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
pub use parser::CqlParser;
pub use preprocessor::{
    CodeInfo, CodeSystemInfo, ConceptInfo, DefinitionKind, ExpressionInfo, FunctionInfo,
    LibraryDependency, LibraryInfo, ModelDependency, ParameterInfo, Preprocessor, ValueSetInfo,
};
pub use provider::{
    fhir_r4_model_info, fhir_r4_provider, MemoryModelInfoProvider, ModelInfoProvider,
};
pub use translator::{ExpressionTranslator, TranslatorError, TranslatorResult};
pub use types::{TypeBuilder, TypeError, TypeInference, TypeResolver, TypeResult};
