//! Library source provider trait and re-exports.

pub mod composite;
pub mod file;
pub mod memory;

pub use composite::CompositeLibrarySourceProvider;
pub use file::FileLibrarySourceProvider;
pub use memory::MemoryLibrarySourceProvider;

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
