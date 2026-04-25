//! FHIR Package Loading and Downloading Library
//!
//! This module provides functionality for downloading and loading FHIR packages from
//! npm-style registries such as packages.fhir.org. It supports authentication,
//! package extraction, and metadata management.
//!
//! # Features
//!
//! - Download FHIR packages from npm-style registries
//! - Support for authentication with bearer tokens
//! - Automatic tarball extraction and decompression
//! - Configurable timeout and retry mechanisms
//! - Comprehensive error handling and logging
//!
//! # Example
//!
//! ```rust,no_run
//! use rh_foundation::loader::{LoaderConfig, PackageLoader};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = LoaderConfig::default();
//!     let loader = PackageLoader::new(config)?;
//!
//!     let packages_dir = PackageLoader::get_default_packages_dir()?;
//!     loader
//!         .download_package("hl7.fhir.r4.core", "4.0.1", &packages_dir)
//!         .await?;
//!     Ok(())
//! }
//! ```

mod archive;
mod error;
mod models;
mod package_path;
mod registry;

use std::path::{Path, PathBuf};
use std::time::Duration;

pub use error::{LoaderError, LoaderResult};
pub use models::{LoaderConfig, PackageDist, PackageManifest, RegistryResponse};

use archive::{extract_tarball, verify_tarball_checksum};
use package_path::{default_packages_dir, package_directory};
use registry::{
    fetch_registry_response, latest_version as resolve_latest_version, package_manifest,
    sorted_versions,
};

/// FHIR package loader for downloading from npm-style registries.
pub struct PackageLoader {
    http_client: crate::http::HttpClient,
    config: LoaderConfig,
}

impl PackageLoader {
    /// Create a new package loader with the given configuration.
    pub fn new(config: LoaderConfig) -> LoaderResult<Self> {
        let mut builder = crate::http::HttpClient::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent("rh-loader/0.1.0")?;

        if let Some(token) = &config.auth_token {
            builder = builder.bearer_auth(token)?;
        }

        let http_client = builder.build()?;

        Ok(Self {
            http_client,
            config,
        })
    }

    pub fn is_package_downloaded(
        &self,
        package_name: &str,
        version: &str,
        output_dir: &Path,
    ) -> LoaderResult<bool> {
        let package_dir = self.get_package_directory(output_dir, package_name, version);
        Ok(package_dir.exists())
    }

    /// Download a FHIR package and extract it to the specified directory.
    pub async fn download_package(
        &self,
        package_name: &str,
        version: &str,
        extract_to: &Path,
    ) -> LoaderResult<PackageManifest> {
        let package_dir = self.get_package_directory(extract_to, package_name, version);

        if package_dir.exists() && !self.config.overwrite_existing {
            return Err(LoaderError::PackageExists {
                package: package_name.to_string(),
                version: version.to_string(),
                path: package_dir.display().to_string(),
            });
        }

        tracing::info!("Downloading FHIR package {}@{}", package_name, version);

        let manifest = self.get_package_manifest(package_name, version).await?;
        let tarball_data = self.download_tarball(&manifest.dist.tarball).await?;

        if self.config.verify_checksums {
            self.verify_tarball_checksum(&tarball_data, &manifest.dist)?;
        }

        self.extract_tarball(&tarball_data, &package_dir)?;

        tracing::info!(
            "Successfully downloaded and extracted {}@{} to {}",
            package_name,
            version,
            package_dir.display()
        );

        Ok(manifest)
    }

    /// Get the default FHIR packages directory (~/.fhir/packages).
    pub fn get_default_packages_dir() -> LoaderResult<PathBuf> {
        default_packages_dir()
    }

    fn get_package_directory(
        &self,
        base_path: &Path,
        package_name: &str,
        version: &str,
    ) -> PathBuf {
        package_directory(base_path, package_name, version)
    }

    /// List available versions for a package.
    pub async fn list_versions(&self, package_name: &str) -> LoaderResult<Vec<String>> {
        let registry_response = self.get_registry_response(package_name).await?;
        Ok(sorted_versions(&registry_response))
    }

    /// Get the latest version for a package.
    pub async fn get_latest_version(&self, package_name: &str) -> LoaderResult<String> {
        let registry_response = self.get_registry_response(package_name).await?;
        resolve_latest_version(package_name, &registry_response)
    }

    async fn get_registry_response(&self, package_name: &str) -> LoaderResult<RegistryResponse> {
        fetch_registry_response(&self.http_client, &self.config.registry_url, package_name).await
    }

    async fn get_package_manifest(
        &self,
        package_name: &str,
        version: &str,
    ) -> LoaderResult<PackageManifest> {
        let registry_response = self.get_registry_response(package_name).await?;
        package_manifest(package_name, version, &registry_response)
    }

    async fn download_tarball(&self, tarball_url: &str) -> LoaderResult<Vec<u8>> {
        tracing::debug!("Downloading tarball from: {}", tarball_url);

        crate::http::with_retry(
            || self.http_client.download(tarball_url),
            self.config.max_retries,
            1000,
        )
        .await
        .map_err(LoaderError::Foundation)
    }

    fn verify_tarball_checksum(&self, tarball_data: &[u8], dist: &PackageDist) -> LoaderResult<()> {
        verify_tarball_checksum(tarball_data, dist)
    }

    fn extract_tarball(&self, tarball_data: &[u8], extract_to: &Path) -> LoaderResult<()> {
        extract_tarball(tarball_data, extract_to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_temp_test_dir(test_name: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir().join(format!(
            "rh_loader_test_{}_{}",
            std::process::id(),
            test_name
        ));
        fs::create_dir_all(&temp_dir).expect("Failed to create temp test directory");
        temp_dir
    }

    fn cleanup_test_dir(dir: &Path) {
        if dir.exists() {
            fs::remove_dir_all(dir).ok();
        }
    }

    #[test]
    fn test_loader_config_default() {
        let config = LoaderConfig::default();
        assert_eq!(config.registry_url, "https://packages.fhir.org");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert!(!config.verify_checksums);
        assert!(!config.overwrite_existing);
    }

    #[tokio::test]
    async fn test_package_loader_creation() {
        let config = LoaderConfig::default();
        let loader = PackageLoader::new(config);
        assert!(loader.is_ok());
    }

    #[tokio::test]
    async fn test_package_loader_with_auth() {
        let config = LoaderConfig {
            auth_token: Some("test-token".to_string()),
            ..LoaderConfig::default()
        };
        let loader = PackageLoader::new(config);
        assert!(loader.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_auth_token() {
        let config = LoaderConfig {
            auth_token: Some("invalid\ntoken".to_string()),
            ..LoaderConfig::default()
        };
        let loader = PackageLoader::new(config);
        assert!(loader.is_err());
    }

    #[test]
    fn test_package_manifest_deserialization() {
        let json = r#"{
            "name": "test-package",
            "version": "1.0.0",
            "description": "Test package",
            "dist": {
                "tarball": "https://example.com/test-1.0.0.tgz",
                "shasum": "abc123"
            },
            "dependencies": {
                "dep1": "^1.0.0"
            },
            "keywords": ["test", "example"]
        }"#;

        let manifest: PackageManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "test-package");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, Some("Test package".to_string()));
        assert_eq!(manifest.dist.tarball, "https://example.com/test-1.0.0.tgz");
        assert_eq!(manifest.dist.shasum, Some("abc123".to_string()));
    }

    #[test]
    fn test_registry_response_deserialization() {
        let json = r#"{
            "name": "test-package",
            "description": "Test package",
            "versions": {
                "1.0.0": {
                    "name": "test-package",
                    "version": "1.0.0",
                    "dist": {
                        "tarball": "https://example.com/test-1.0.0.tgz"
                    }
                }
            },
            "dist-tags": {
                "latest": "1.0.0"
            }
        }"#;

        let response: RegistryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.name, "test-package");
        assert!(response.versions.contains_key("1.0.0"));
        assert_eq!(
            response.dist_tags.unwrap().get("latest"),
            Some(&"1.0.0".to_string())
        );
    }

    #[test]
    fn test_get_default_packages_dir() {
        let dir = PackageLoader::get_default_packages_dir();
        assert!(dir.is_ok());
        let dir_path = dir.unwrap();
        assert!(dir_path.to_string_lossy().contains(".fhir"));
        assert!(dir_path.to_string_lossy().contains("packages"));

        let path_str = dir_path.to_string_lossy();
        assert!(path_str.ends_with(".fhir/packages") || path_str.ends_with(".fhir\\packages"));
    }

    #[test]
    fn test_get_package_directory() {
        let config = LoaderConfig::default();
        let loader = PackageLoader::new(config).unwrap();
        let base_path = Path::new("/tmp/packages");
        let package_dir = loader.get_package_directory(base_path, "hl7.fhir.r4.core", "4.0.1");

        assert_eq!(
            package_dir,
            Path::new("/tmp/packages/hl7.fhir.r4.core#4.0.1")
        );
    }

    #[test]
    fn test_package_directory_with_cleanup() {
        let temp_dir = create_temp_test_dir("package_directory_cleanup");

        let config = LoaderConfig::default();
        let loader = PackageLoader::new(config).unwrap();

        let package_dir = loader.get_package_directory(&temp_dir, "test.package", "1.0.0");
        let expected_dir = temp_dir.join("test.package#1.0.0");

        assert_eq!(package_dir, expected_dir);
        assert!(temp_dir.exists());

        fs::create_dir_all(&package_dir).unwrap();
        assert!(package_dir.exists());

        let is_downloaded = loader
            .is_package_downloaded("test.package", "1.0.0", &temp_dir)
            .unwrap();
        assert!(is_downloaded);

        cleanup_test_dir(&temp_dir);
        assert!(!temp_dir.exists());
    }

    #[test]
    fn test_default_packages_dir_with_package_directory() {
        let config = LoaderConfig::default();
        let loader = PackageLoader::new(config).unwrap();

        let default_dir = PackageLoader::get_default_packages_dir().unwrap();
        let package_dir = loader.get_package_directory(&default_dir, "hl7.fhir.r4.core", "4.0.1");

        let expected_suffix = if cfg!(windows) {
            ".fhir\\packages\\hl7.fhir.r4.core#4.0.1"
        } else {
            ".fhir/packages/hl7.fhir.r4.core#4.0.1"
        };

        assert!(package_dir.to_string_lossy().ends_with(expected_suffix));
    }

    #[test]
    fn test_extract_tarball_cleanup() {
        let temp_dir = create_temp_test_dir("extract_tarball_cleanup");
        let config = LoaderConfig::default();
        let loader = PackageLoader::new(config).unwrap();

        let tar_data = create_test_tarball();
        let result = loader.extract_tarball(&tar_data, &temp_dir);

        let _ = result;

        cleanup_test_dir(&temp_dir);
        assert!(!temp_dir.exists());
    }

    fn create_test_tarball() -> Vec<u8> {
        vec![0x1f, 0x8b, 0x08, 0x00]
    }
}
