//! Composite library source provider.

use super::LibrarySourceProvider;
use crate::library::identifiers::LibraryIdentifier;
use crate::library::sources::LibrarySource;

/// A composite provider that searches multiple providers in order.
///
/// Useful for layering providers, e.g., check in-memory first, then filesystem.
#[derive(Default)]
pub struct CompositeLibrarySourceProvider {
    providers: Vec<Box<dyn LibrarySourceProvider>>,
}

impl CompositeLibrarySourceProvider {
    /// Create a new empty composite provider.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Add a provider to the chain.
    pub fn add_provider(mut self, provider: impl LibrarySourceProvider + 'static) -> Self {
        self.providers.push(Box::new(provider));
        self
    }
}

impl LibrarySourceProvider for CompositeLibrarySourceProvider {
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        for provider in &self.providers {
            if let Some(source) = provider.get_source(identifier) {
                return Some(source);
            }
        }
        None
    }

    fn has_library(&self, identifier: &LibraryIdentifier) -> bool {
        self.providers.iter().any(|p| p.has_library(identifier))
    }

    fn list_libraries(&self) -> Vec<LibraryIdentifier> {
        let mut libraries = Vec::new();
        for provider in &self.providers {
            for id in provider.list_libraries() {
                if !libraries.contains(&id) {
                    libraries.push(id);
                }
            }
        }
        libraries
    }
}
