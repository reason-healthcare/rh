//! FHIR package downloading functionality
//!
//! This module contains functionality for downloading FHIR packages from npm-style registries.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use flate2::read::GzDecoder;
use reqwest;
use serde::Deserialize;
use tar::Archive;
use tracing;
use url::Url;

use crate::{CodegenError, CodegenResult};

/// NPM-style package manifest structure
#[derive(Debug, Clone, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub dist: PackageDist,
}

/// Package distribution information
#[derive(Debug, Clone, Deserialize)]
pub struct PackageDist {
    pub tarball: String,
    pub shasum: Option<String>,
}

/// NPM registry response for package versions
#[derive(Debug, Deserialize)]
pub struct RegistryResponse {
    pub versions: HashMap<String, PackageManifest>,
}

/// Configuration for package downloading
#[derive(Debug, Clone)]
pub struct PackageDownloadConfig {
    /// Registry URL (defaults to https://packages.fhir.org)
    pub registry_url: String,
    /// Optional authentication token
    pub auth_token: Option<String>,
    /// HTTP client timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for PackageDownloadConfig {
    fn default() -> Self {
        Self {
            registry_url: "https://packages.fhir.org".to_string(),
            auth_token: None,
            timeout_seconds: 30,
        }
    }
}

/// FHIR package downloader
pub struct PackageDownloader {
    client: reqwest::Client,
    config: PackageDownloadConfig,
}

impl PackageDownloader {
    /// Create a new package downloader with the given configuration
    pub fn new(config: PackageDownloadConfig) -> CodegenResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Add authentication header if token is provided
        if let Some(token) = &config.auth_token {
            let auth_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|e| CodegenError::Generation {
                    message: format!("Invalid auth token: {e}"),
                })?;
            headers.insert(reqwest::header::AUTHORIZATION, auth_value);
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .default_headers(headers)
            .build()?;

        Ok(Self { client, config })
    }

    /// Download a FHIR package and extract it to the specified directory
    pub async fn download_package(
        &self,
        package_name: &str,
        version: &str,
        extract_to: &Path,
    ) -> CodegenResult<()> {
        tracing::info!("Downloading FHIR package {}@{}", package_name, version);

        // Get package manifest
        let manifest = self.get_package_manifest(package_name, version).await?;

        // Download tarball
        let tarball_data = self.download_tarball(&manifest.dist.tarball).await?;

        // Extract tarball
        self.extract_tarball(&tarball_data, extract_to)?;

        tracing::info!(
            "Successfully downloaded and extracted {}@{} to {}",
            package_name,
            version,
            extract_to.display()
        );

        Ok(())
    }

    /// Get package manifest from registry
    async fn get_package_manifest(
        &self,
        package_name: &str,
        version: &str,
    ) -> CodegenResult<PackageManifest> {
        let registry_url = Url::parse(&self.config.registry_url)?;
        let package_url = registry_url.join(package_name)?;

        tracing::debug!("Fetching package manifest from: {}", package_url);

        let response = self.client.get(package_url).send().await?;

        if !response.status().is_success() {
            return Err(CodegenError::PackageNotFound {
                package: package_name.to_string(),
                version: version.to_string(),
            });
        }

        let registry_response: RegistryResponse = response.json().await?;

        // Find the requested version
        let version_manifest = registry_response.versions.get(version).ok_or_else(|| {
            CodegenError::PackageNotFound {
                package: package_name.to_string(),
                version: version.to_string(),
            }
        })?;

        // Create PackageManifest from the registry response
        Ok(PackageManifest {
            name: package_name.to_string(),
            version: version.to_string(),
            dist: version_manifest.dist.clone(),
        })
    }

    /// Download tarball from URL
    async fn download_tarball(&self, tarball_url: &str) -> CodegenResult<Vec<u8>> {
        tracing::debug!("Downloading tarball from: {}", tarball_url);

        let response = self.client.get(tarball_url).send().await?;

        if !response.status().is_success() {
            return Err(CodegenError::Generation {
                message: format!("Failed to download tarball: HTTP {}", response.status()),
            });
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Extract gzipped tarball to directory
    fn extract_tarball(&self, tarball_data: &[u8], extract_to: &Path) -> CodegenResult<()> {
        tracing::debug!("Extracting tarball to: {}", extract_to.display());

        // Create extraction directory
        fs::create_dir_all(extract_to)?;

        // Decompress gzip
        let tar_decoder = GzDecoder::new(tarball_data);
        let mut archive = Archive::new(tar_decoder);

        // Extract all files
        archive
            .unpack(extract_to)
            .map_err(|e| CodegenError::ArchiveError {
                message: format!("Failed to extract tarball: {e}"),
            })?;

        Ok(())
    }
}
