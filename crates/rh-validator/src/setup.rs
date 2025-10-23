//! FHIR setup module
//!
//! This module handles downloading FHIR packages and generating Rust types.

use rh_loader::{LoaderConfig, PackageLoader};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result as ValidatorResult, ValidatorError};

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
    pub fn new() -> ValidatorResult<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            ValidatorError::Foundation(rh_foundation::FoundationError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            )))
        })?;

        let base_dir = home_dir.join(".fhir");
        Self::with_base_dir(base_dir)
    }

    /// Create a new FHIR setup manager with custom base directory
    pub fn with_base_dir(base_dir: PathBuf) -> ValidatorResult<Self> {
        let packages_dir = base_dir.join("packages");
        let rust_dir = base_dir.join("rust");
        fs::create_dir_all(&packages_dir)?;
        fs::create_dir_all(&rust_dir)?;

        let mut version_mappings = HashMap::new();
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
    pub fn get_package_mapping(&self, version: &str) -> ValidatorResult<&FhirPackageMapping> {
        self.version_mappings
            .get(version)
            .ok_or_else(|| ValidatorError::InvalidVersion {
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
    pub async fn download_package(&self, mapping: &FhirPackageMapping) -> ValidatorResult<()> {
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

        let downloader = PackageLoader::new(download_config)?;
        let packages_dir = self.base_dir.join("packages");

        downloader
            .download_package(&mapping.package, &mapping.package_version, &packages_dir)
            .await
            .map_err(|e| ValidatorError::PackageDownload {
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
