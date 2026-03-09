//! Library source types.
//!
//! Provides [`LibrarySource`] for carrying CQL source code with metadata.

use super::identifiers::LibraryIdentifier;

/// CQL source code with metadata.
#[derive(Debug, Clone)]
pub struct LibrarySource {
    /// The library identifier.
    pub identifier: LibraryIdentifier,
    /// The CQL source code.
    pub source: String,
    /// Optional source location (file path or URI).
    pub location: Option<String>,
}

impl LibrarySource {
    /// Create a new library source.
    pub fn new(
        identifier: LibraryIdentifier,
        source: impl Into<String>,
        location: Option<impl Into<String>>,
    ) -> Self {
        Self {
            identifier,
            source: source.into(),
            location: location.map(|l| l.into()),
        }
    }
}
