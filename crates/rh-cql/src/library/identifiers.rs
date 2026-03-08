//! Library identifier types.
//!
//! Provides [`LibraryIdentifier`] used as a key for library lookup and
//! dependency resolution.

/// Identifier for a CQL library (name + optional version).
///
/// Used as a key for library lookup and dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryIdentifier {
    /// The library name/path.
    pub name: String,
    /// The library version (optional).
    pub version: Option<String>,
}

impl LibraryIdentifier {
    /// Create a new library identifier.
    pub fn new(name: impl Into<String>, version: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            version: version.map(|v| v.into()),
        }
    }

    /// Create an identifier without a version.
    pub fn unversioned(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    /// Create a key string for storage.
    pub fn to_key(&self) -> String {
        match &self.version {
            Some(v) => format!("{}|{}", self.name, v),
            None => self.name.clone(),
        }
    }

    /// Parse a key string back into an identifier.
    pub fn from_key(key: &str) -> Self {
        match key.split_once('|') {
            Some((name, version)) => Self::new(name, Some(version)),
            None => Self::unversioned(key),
        }
    }

    /// Check if this identifier matches another, considering version compatibility.
    ///
    /// A request without a version matches any version of the same library.
    /// A request with a version only matches that exact version.
    pub fn matches(&self, other: &LibraryIdentifier) -> bool {
        if self.name != other.name {
            return false;
        }

        match (&self.version, &other.version) {
            // Both have versions - must match exactly
            (Some(v1), Some(v2)) => v1 == v2,
            // Request has no version - matches any
            (None, _) => true,
            // Request has version but candidate doesn't - no match
            (Some(_), None) => false,
        }
    }
}

impl std::fmt::Display for LibraryIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            Some(v) => write!(f, "{} version '{}'", self.name, v),
            None => write!(f, "{}", self.name),
        }
    }
}
