//! FHIR setup module
//!
//! This module handles downloading FHIR packages and generating Rust types.

use anyhow::Result;
use rh_loader::{LoaderConfig, PackageLoader};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors specific to FHIR validation and setup
#[derive(Error, Debug)]
pub enum FhirValidationError {
    #[error("Invalid FHIR version: {version}")]
    InvalidVersion { version: String },

    #[error("Package not found: {package}")]
    PackageNotFound { package: String },

    #[error("Failed to download package: {message}")]
    PackageDownload { message: String },

    #[error("Failed to generate Rust types: {message}")]
    CodegenFailed { message: String },

    #[error("FHIR resource validation failed: {message}")]
    ResourceValidation { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Codegen error: {0}")]
    Codegen(#[from] rh_codegen::CodegenError),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Loader error: {0}")]
    Loader(#[from] rh_loader::LoaderError),
}

/// Mapping from FHIR version to core package information
#[derive(Debug, Clone)]
pub struct FhirPackageMapping {
    /// FHIR version (e.g., "4.0.1")
    pub version: String,
    /// Package name (e.g., "hl7.fhir.r4.core")
    pub package: String,
    /// Package version (usually same as FHIR version)
    pub package_version: String,
}

impl FhirPackageMapping {
    /// Get the package identifier string (package@version)
    pub fn package_id(&self) -> String {
        format!("{}@{}", self.package, self.package_version)
    }

    /// Get the directory name for this package (package#version)
    pub fn package_dir_name(&self) -> String {
        format!("{}#{}", self.package, self.package_version)
    }
}

/// FHIR setup manager for package downloads and Rust type generation
pub struct FhirSetup {
    /// Base directory for FHIR packages and generated types
    base_dir: PathBuf,
    /// Default FHIR version to use
    default_version: String,
    /// Mapping of FHIR versions to core packages
    version_mappings: HashMap<String, FhirPackageMapping>,
}

impl FhirSetup {
    /// Create a new FHIR setup manager with default settings
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            FhirValidationError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            ))
        })?;

        let base_dir = home_dir.join(".fhir");
        Self::with_base_dir(base_dir)
    }

    /// Create a new FHIR setup manager with custom base directory
    pub fn with_base_dir(base_dir: PathBuf) -> Result<Self> {
        // Ensure directories exist
        let packages_dir = base_dir.join("packages");
        let rust_dir = base_dir.join("rust");
        fs::create_dir_all(&packages_dir)?;
        fs::create_dir_all(&rust_dir)?;

        // Initialize version mappings
        let mut version_mappings = HashMap::new();

        // FHIR R4
        version_mappings.insert(
            "4.0.1".to_string(),
            FhirPackageMapping {
                version: "4.0.1".to_string(),
                package: "hl7.fhir.r4.core".to_string(),
                package_version: "4.0.1".to_string(),
            },
        );

        Ok(Self {
            base_dir,
            default_version: "4.0.1".to_string(),
            version_mappings,
        })
    }

    /// Get the base directory
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Get the default FHIR version
    pub fn default_version(&self) -> &str {
        &self.default_version
    }

    /// Get the package mapping for a FHIR version
    pub fn get_package_mapping(
        &self,
        version: &str,
    ) -> Result<&FhirPackageMapping, FhirValidationError> {
        self.version_mappings
            .get(version)
            .ok_or_else(|| FhirValidationError::InvalidVersion {
                version: version.to_string(),
            })
    }

    /// Check if a package is already downloaded
    pub fn is_package_downloaded(&self, mapping: &FhirPackageMapping) -> bool {
        let package_dir = self
            .base_dir
            .join("packages")
            .join(mapping.package_dir_name());

        package_dir.exists() && package_dir.join("package").exists()
    }

    /// Download a FHIR package if not already present
    pub async fn download_package(
        &self,
        mapping: &FhirPackageMapping,
    ) -> Result<(), FhirValidationError> {
        if self.is_package_downloaded(mapping) {
            return Ok(());
        }

        let download_config = LoaderConfig {
            registry_url: "https://packages.fhir.org".to_string(),
            auth_token: None,
            timeout_seconds: 30,
            max_retries: 3,
            verify_checksums: false,
            overwrite_existing: false,
        };

        let downloader =
            PackageLoader::new(download_config).map_err(FhirValidationError::Loader)?;
        let packages_dir = self.base_dir.join("packages");

        downloader
            .download_package(&mapping.package, &mapping.package_version, &packages_dir)
            .await
            .map_err(|e| FhirValidationError::PackageDownload {
                message: e.to_string(),
            })?;

        Ok(())
    }
}

impl Default for FhirSetup {
    fn default() -> Self {
        Self::new().expect("Failed to create default FHIR setup manager")
    }
}
