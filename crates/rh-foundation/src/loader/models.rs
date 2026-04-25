use std::collections::HashMap;

use serde::Deserialize;

/// NPM-style package manifest structure.
#[derive(Debug, Clone, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dist: PackageDist,
    pub dependencies: Option<HashMap<String, String>>,
    pub keywords: Option<Vec<String>>,
}

/// Package distribution information.
#[derive(Debug, Clone, Deserialize)]
pub struct PackageDist {
    pub tarball: String,
    pub shasum: Option<String>,
    pub integrity: Option<String>,
}

/// NPM registry response for package versions.
#[derive(Debug, Deserialize)]
pub struct RegistryResponse {
    pub name: String,
    pub description: Option<String>,
    pub versions: HashMap<String, PackageManifest>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: Option<HashMap<String, String>>,
}

/// Configuration for package loading operations.
#[derive(Debug, Clone)]
pub struct LoaderConfig {
    /// Registry URL (defaults to https://packages.fhir.org).
    pub registry_url: String,
    /// Optional authentication token.
    pub auth_token: Option<String>,
    /// HTTP client timeout in seconds.
    pub timeout_seconds: u64,
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Whether to verify checksums when available.
    pub verify_checksums: bool,
    /// Whether to overwrite existing packages (defaults to false).
    pub overwrite_existing: bool,
}

impl Default for LoaderConfig {
    fn default() -> Self {
        Self {
            registry_url: "https://packages.fhir.org".to_string(),
            auth_token: None,
            timeout_seconds: 30,
            max_retries: 3,
            verify_checksums: false,
            overwrite_existing: false,
        }
    }
}
