//! Foundation crate providing common utilities and types shared across the workspace.
//!
//! This crate provides foundational functionality including:
//! - Error handling (`error` module)
//! - Configuration traits (`config` module)
//! - I/O utilities (`io` module)
//! - HTTP utilities (`http` module, with `http` feature)
//! - JSON helpers (`json` module)
//!
//! # Features
//! - `http`: Enables HTTP client utilities (requires `reqwest` and `tokio`)

pub mod config;
pub mod error;
pub mod io;
pub mod json;

#[cfg(feature = "http")]
pub mod http;

// Re-export commonly used types
pub use config::Config;
pub use error::{ErrorContext, ErrorWithMetadata, FoundationError, Result};
