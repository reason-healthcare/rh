//! FHIR package-based library source provider.
//!
//! Resolves CQL libraries from locally installed FHIR packages stored in the
//! standard `~/.fhir/packages/<package-name>#<version>/package/` layout.
//!
//! Each library is stored as a FHIR `Library` JSON resource whose `content`
//! array contains a `text/cql` entry with the CQL source base64-encoded.
//!
//! # Name resolution
//!
//! Two forms of include path are supported:
//!
//! - **Package-qualified** — `fhir.cqf.common.FHIRHelpers`:
//!   The last dot-separated segment that begins with an uppercase letter is
//!   treated as the library name; everything before it is the package name.
//!   The version from the `include ... version '...'` clause selects which
//!   installed package to use.
//!
//! - **Simple name** — `FHIRHelpers`:
//!   All installed package directories are scanned for a matching
//!   `Library-FHIRHelpers.json`. If `version` is given, only packages whose
//!   version matches are considered.
//!
//! # Example
//!
//! ```no_run
//! use rh_cql::library::{PackageLibrarySourceProvider, LibraryIdentifier, LibrarySourceProvider};
//!
//! let provider = PackageLibrarySourceProvider::new("~/.fhir/packages");
//!
//! // Resolves fhir.cqf.common#4.0.1/package/Library-FHIRHelpers.json
//! let id = LibraryIdentifier::new("fhir.cqf.common.FHIRHelpers", Some("4.0.1"));
//! let source = provider.get_source(&id);
//!
//! // Simple name — scans all packages
//! let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
//! let source = provider.get_source(&id);
//! ```

use std::path::{Path, PathBuf};

use super::LibrarySourceProvider;
use crate::library::identifiers::LibraryIdentifier;
use crate::library::sources::LibrarySource;
use crate::provider::decode_base64;

/// A library source provider that resolves CQL libraries from locally installed
/// FHIR packages.
///
/// Packages must be in the standard FHIR CLI layout:
/// `<packages_dir>/<package-name>#<version>/package/Library-<name>.json`
///
/// CQL is extracted from the `text/cql` content entry in the Library JSON.
#[derive(Debug, Clone)]
pub struct PackageLibrarySourceProvider {
    /// Root directory that contains installed FHIR package directories.
    packages_dir: PathBuf,
}

impl PackageLibrarySourceProvider {
    /// Create a provider rooted at `packages_dir`.
    pub fn new(packages_dir: impl AsRef<Path>) -> Self {
        Self {
            packages_dir: packages_dir.as_ref().to_path_buf(),
        }
    }

    /// Return the configured packages root directory.
    pub fn packages_dir(&self) -> &Path {
        &self.packages_dir
    }

    /// Split a possibly-qualified library name into `(package_name, lib_name)`.
    ///
    /// `"fhir.cqf.common.FHIRHelpers"` → `(Some("fhir.cqf.common"), "FHIRHelpers")`
    /// `"FHIRHelpers"`                  → `(None, "FHIRHelpers")`
    ///
    /// The last dot-separated segment that starts with an uppercase ASCII letter
    /// is treated as the library name; everything before it is the package name.
    fn parse_name(name: &str) -> (Option<&str>, &str) {
        if let Some(pos) = name.rfind('.') {
            let lib = &name[pos + 1..];
            if lib.chars().next().is_some_and(|c| c.is_uppercase()) {
                return (Some(&name[..pos]), lib);
            }
        }
        (None, name)
    }

    /// Extract the CQL source string from a FHIR Library JSON file.
    ///
    /// Returns `None` if the file cannot be read, parsed, or does not contain a
    /// `text/cql` content entry.
    fn extract_cql_from_library_json(path: &Path) -> Option<String> {
        let raw = std::fs::read_to_string(path).ok()?;
        let lib: serde_json::Value = serde_json::from_str(&raw).ok()?;
        let content = lib.get("content")?.as_array()?;
        for entry in content {
            if entry
                .get("contentType")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                == "text/cql"
            {
                let b64 = entry.get("data")?.as_str()?;
                let bytes = decode_base64(b64).ok()?;
                return String::from_utf8(bytes).ok();
            }
        }
        None
    }

    /// Try to load `lib_name` from a specific package directory.
    ///
    /// Looks for `<package_dir>/package/Library-<lib_name>.json` (and
    /// `<package_dir>/Library-<lib_name>.json` as a fallback).
    fn find_in_package_dir(&self, package_dir: &Path, lib_name: &str) -> Option<LibrarySource> {
        let pkg_content = package_dir.join("package");
        let search_dir = if pkg_content.exists() {
            pkg_content
        } else {
            package_dir.to_path_buf()
        };

        let filename = format!("Library-{lib_name}.json");
        let file_path = search_dir.join(&filename);

        if !file_path.exists() {
            return None;
        }

        let cql = Self::extract_cql_from_library_json(&file_path)?;

        // Parse the version from the package directory name: `pkg-name#version`
        let version = package_dir
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.split_once('#'))
            .map(|(_, v)| v.to_string());

        let id = LibraryIdentifier::new(lib_name, version.as_deref());
        let location = file_path.to_string_lossy().to_string();
        Some(LibrarySource::new(id, cql, Some(location)))
    }

    /// Return all package directories whose name matches `<package_name>#<version>`.
    ///
    /// If `version` is `None`, all versions of `package_name` are returned.
    fn matching_package_dirs(&self, package_name: &str, version: Option<&str>) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        let Ok(entries) = std::fs::read_dir(&self.packages_dir) else {
            return dirs;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if let Some((pkg, ver)) = dir_name.split_once('#') {
                    if pkg == package_name && version.is_none_or(|v| v == ver) {
                        dirs.push(path);
                    }
                }
            }
        }
        dirs
    }

    /// Scan every installed package for a library whose name matches `lib_name`,
    /// optionally filtering by `version`.
    fn find_in_any_package(&self, lib_name: &str, version: Option<&str>) -> Option<LibrarySource> {
        let Ok(entries) = std::fs::read_dir(&self.packages_dir) else {
            return None;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if let Some(source) = self.find_in_package_dir(&path, lib_name) {
                // Skip if the caller requested a specific version that doesn't match.
                if let Some(req_ver) = version {
                    if source.identifier.version.as_deref() != Some(req_ver) {
                        continue;
                    }
                }
                return Some(source);
            }
        }
        None
    }
}

impl LibrarySourceProvider for PackageLibrarySourceProvider {
    fn get_source(&self, identifier: &LibraryIdentifier) -> Option<LibrarySource> {
        if !self.packages_dir.exists() {
            return None;
        }

        let (package_name, lib_name) = Self::parse_name(&identifier.name);
        let version = identifier.version.as_deref();

        if let Some(pkg) = package_name {
            // Qualified name — target a specific package.
            for pkg_dir in self.matching_package_dirs(pkg, version) {
                if let Some(source) = self.find_in_package_dir(&pkg_dir, lib_name) {
                    return Some(source);
                }
            }
            None
        } else {
            // Simple name — scan all installed packages.
            self.find_in_any_package(lib_name, version)
        }
    }

    fn list_libraries(&self) -> Vec<LibraryIdentifier> {
        let mut libs = Vec::new();
        let Ok(pkg_entries) = std::fs::read_dir(&self.packages_dir) else {
            return libs;
        };
        for pkg_entry in pkg_entries.flatten() {
            let pkg_dir = pkg_entry.path();
            if !pkg_dir.is_dir() {
                continue;
            }
            let version = pkg_dir
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.split_once('#'))
                .map(|(_, v)| v.to_string());

            let content_dir = pkg_dir.join("package");
            let search_dir = if content_dir.exists() {
                content_dir
            } else {
                pkg_dir
            };

            let Ok(entries) = std::fs::read_dir(&search_dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(lib_name) = name
                        .strip_prefix("Library-")
                        .and_then(|n| n.strip_suffix(".json"))
                    {
                        libs.push(LibraryIdentifier::new(lib_name, version.as_deref()));
                    }
                }
            }
        }
        libs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name_qualified() {
        let (pkg, lib) = PackageLibrarySourceProvider::parse_name("fhir.cqf.common.FHIRHelpers");
        assert_eq!(pkg, Some("fhir.cqf.common"));
        assert_eq!(lib, "FHIRHelpers");
    }

    #[test]
    fn test_parse_name_simple() {
        let (pkg, lib) = PackageLibrarySourceProvider::parse_name("FHIRHelpers");
        assert_eq!(pkg, None);
        assert_eq!(lib, "FHIRHelpers");
    }

    #[test]
    fn test_parse_name_lowercase_last_segment_not_split() {
        // "some.package.name" — last segment is lowercase, treat as simple name
        let (pkg, lib) = PackageLibrarySourceProvider::parse_name("some.package.name");
        assert_eq!(pkg, None);
        assert_eq!(lib, "some.package.name");
    }

    #[test]
    fn test_nonexistent_packages_dir_returns_none() {
        let provider = PackageLibrarySourceProvider::new("/nonexistent/packages");
        let id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
        assert!(provider.get_source(&id).is_none());
    }
}
