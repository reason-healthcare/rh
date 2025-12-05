//! Foundation crate providing common utilities and types shared across the workspace.
//!
//! This crate provides foundational functionality including:
//! - Error handling (`error` module)
//! - Configuration traits (`config` module)
//! - I/O utilities (`io` module)
//! - HTTP utilities (`http` module, with `http` feature)
//! - JSON helpers (`json` module)
//! - In-memory storage (`memory` module) - WASM-compatible caching
//! - WASM utilities (`wasm` module, with `wasm` feature)
//! - Package loading (`loader` module, with `http` feature)
//! - Snapshot generation (`snapshot` module)
//!
//! # Features
//! - `http`: Enables HTTP client utilities (requires `reqwest` and `tokio`)
//! - `wasm`: Enables WebAssembly utilities (requires `wasm-bindgen`)

pub mod cli;
pub mod config;
pub mod error;
pub mod io;
pub mod json;
pub mod memory;
pub mod snapshot;
pub mod validation;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http")]
pub mod loader;

#[cfg(feature = "wasm")]
pub mod wasm;

// Re-export commonly used types
pub use config::Config;
pub use error::{ErrorContext, ErrorWithMetadata, FoundationError, Result};
pub use memory::{MemoryStore, MemoryStoreConfig, MemoryStoreStats};
pub use validation::{BindingStrength, ElementBinding, ElementCardinality, Invariant, Severity};
