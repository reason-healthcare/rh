//! Library source provider trait and re-exports.

pub mod composite;
#[cfg(feature = "fs")]
pub mod file;
pub mod memory;
#[cfg(feature = "fs")]
pub mod package;

pub use composite::CompositeLibrarySourceProvider;
#[cfg(feature = "fs")]
pub use file::FileLibrarySourceProvider;
pub use memory::MemoryLibrarySourceProvider;
#[cfg(feature = "fs")]
pub use package::PackageLibrarySourceProvider;

use super::identifiers::LibraryIdentifier;
use super::sources::LibrarySource;

/// A provider for CQL library source code.
///
/// Implementations of this trait provide access to CQL source files
/// for compilation and dependency resolution.
pub trait LibrarySourceProvider: Send + Sync {
    /// Get the source code for a library.
    ///
    /// Returns `Some(source)` if the library is found, `None` otherwise.
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource>;

    /// Load a version-pinned, precompiled ELM dependency when the provider
    /// supports it. Implementations must return an error for a discovered
    /// compiled artifact that cannot be trusted (for example, malformed JSON
    /// or an identifier mismatch), rather than silently falling back to CQL
    /// source.
    ///
    /// This is deliberately separate from [`Self::get_source`]: root CQL is
    /// still parsed and compiled locally, while a pinned external dependency
    /// can contribute its already-translated symbols and function bodies.
    fn get_precompiled_elm(
        &self,
        _identifier: &LibraryIdentifier,
    ) -> Result<Option<crate::elm::Library>, String> {
        Ok(None)
    }

    /// Check if a library is available.
    fn has_library(&self, identifier: &LibraryIdentifier) -> bool {
        self.get_source(identifier).is_some()
    }

    /// List all available library identifiers.
    fn list_libraries(&self) -> Vec<LibraryIdentifier>;

    /// Find libraries by name (any version).
    fn find_by_name(&self, name: &str) -> Vec<LibraryIdentifier> {
        self.list_libraries()
            .into_iter()
            .filter(|id| id.name == name)
            .collect()
    }
}
