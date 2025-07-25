//! Code generation library for creating Rust types from FHIR StructureDefinitions
//!
//! This crate provides functionality to parse FHIR StructureDefinition JSON files
//! and generate corresponding Rust type definitions.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use reqwest;
use serde::{Deserialize, Serialize};
use syn::{parse_quote, Type};
use tar::Archive;
use flate2::read::GzDecoder;
use url::Url;
use tracing;

pub use common::{CommonError, Config};

/// Errors specific to code generation
#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
    #[error("Invalid FHIR type: {fhir_type}")]
    InvalidFhirType { fhir_type: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Code generation failed: {message}")]
    Generation { message: String },

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Package not found: {package}@{version}")]
    PackageNotFound { package: String, version: String },

    #[error("Invalid package manifest: {message}")]
    InvalidManifest { message: String },

    #[error("Archive extraction failed: {message}")]
    ArchiveError { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Common error: {0}")]
    Common(#[from] CommonError),
}

/// Result type for codegen operations
pub type CodegenResult<T> = std::result::Result<T, CodegenError>;

/// Configuration for the code generator
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodegenConfig {
    /// Output directory for generated files
    pub output_dir: String,
    /// Module name for generated types
    pub module_name: String,
    /// Whether to generate serde annotations
    pub with_serde: bool,
    /// Whether to generate documentation
    pub with_docs: bool,
    /// Custom type mappings from FHIR to Rust types
    pub type_mappings: HashMap<String, String>,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        let mut type_mappings = HashMap::new();

        // Common FHIR to Rust type mappings
        type_mappings.insert("string".to_string(), "String".to_string());
        type_mappings.insert("integer".to_string(), "i32".to_string());
        type_mappings.insert("boolean".to_string(), "bool".to_string());
        type_mappings.insert("decimal".to_string(), "f64".to_string());
        type_mappings.insert("uri".to_string(), "String".to_string());
        type_mappings.insert("url".to_string(), "String".to_string());
        type_mappings.insert("canonical".to_string(), "String".to_string());
        type_mappings.insert("code".to_string(), "String".to_string());
        type_mappings.insert("oid".to_string(), "String".to_string());
        type_mappings.insert("id".to_string(), "String".to_string());
        type_mappings.insert("markdown".to_string(), "String".to_string());
        type_mappings.insert("base64Binary".to_string(), "String".to_string());
        type_mappings.insert("instant".to_string(), "String".to_string()); // Could be chrono::DateTime
        type_mappings.insert("date".to_string(), "String".to_string()); // Could be chrono::NaiveDate
        type_mappings.insert("dateTime".to_string(), "String".to_string()); // Could be chrono::DateTime
        type_mappings.insert("time".to_string(), "String".to_string()); // Could be chrono::NaiveTime

        Self {
            output_dir: "src/generated".to_string(),
            module_name: "fhir_types".to_string(),
            with_serde: true,
            with_docs: true,
            type_mappings,
        }
    }
}

impl Config for CodegenConfig {}

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
            let auth_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|e| CodegenError::Generation { 
                    message: format!("Invalid auth token: {}", e) 
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
        
        tracing::info!("Successfully downloaded and extracted {}@{} to {}", 
                      package_name, version, extract_to.display());
        
        Ok(())
    }

    /// Get package manifest from registry
    async fn get_package_manifest(
        &self,
        package_name: &str,
        version: &str,
    ) -> CodegenResult<PackageManifest> {
        let registry_url = Url::parse(&self.config.registry_url)?;
        let package_url = registry_url.join(&format!("{}", package_name))?;
        
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
        let version_manifest = registry_response.versions.get(version)
            .ok_or_else(|| CodegenError::PackageNotFound {
                package: package_name.to_string(),
                version: version.to_string(),
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
        archive.unpack(extract_to).map_err(|e| CodegenError::ArchiveError {
            message: format!("Failed to extract tarball: {}", e),
        })?;
        
        Ok(())
    }
}

/// Represents a FHIR StructureDefinition element
#[derive(Debug, Deserialize, Clone)]
pub struct ElementDefinition {
    pub id: Option<String>,
    pub path: String,
    pub short: Option<String>,
    pub definition: Option<String>,
    pub min: Option<u32>,
    pub max: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<Vec<ElementType>>,
    pub fixed: Option<serde_json::Value>,
    pub pattern: Option<serde_json::Value>,
}

/// Represents a FHIR element type
#[derive(Debug, Deserialize, Clone)]
pub struct ElementType {
    pub code: String,
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,
}

/// Represents a FHIR StructureDefinition
#[derive(Debug, Deserialize)]
pub struct StructureDefinition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub url: String,
    pub name: String,
    pub title: Option<String>,
    pub status: String,
    pub kind: String,
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
    #[serde(rename = "type")]
    pub base_type: String,
    #[serde(rename = "baseDefinition")]
    pub base_definition: Option<String>,
    pub differential: Option<StructureDefinitionDifferential>,
    pub snapshot: Option<StructureDefinitionSnapshot>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionDifferential {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinitionSnapshot {
    pub element: Vec<ElementDefinition>,
}

/// Represents a generated Rust field
#[derive(Debug, Clone)]
pub struct RustField {
    pub name: String,
    pub rust_type: String,
    pub is_optional: bool,
    pub is_array: bool,
    pub documentation: Option<String>,
    pub serde_rename: Option<String>,
}

/// Represents a generated Rust struct
#[derive(Debug)]
pub struct RustStruct {
    pub name: String,
    pub fields: Vec<RustField>,
    pub documentation: Option<String>,
    pub derives: Vec<String>,
}

/// Main code generator struct
pub struct CodeGenerator {
    config: CodegenConfig,
    type_cache: HashMap<String, RustStruct>,
}

impl CodeGenerator {
    /// Create a new code generator with the given configuration
    pub fn new(config: CodegenConfig) -> Self {
        Self {
            config,
            type_cache: HashMap::new(),
        }
    }

    /// Load and parse a FHIR StructureDefinition from a JSON file
    pub fn load_structure_definition<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> CodegenResult<StructureDefinition> {
        let content = fs::read_to_string(&path).map_err(|e| CodegenError::Io(e))?;

        let structure_def: StructureDefinition =
            serde_json::from_str(&content).map_err(|e| CodegenError::Json(e))?;

        Ok(structure_def)
    }

    /// Generate a Rust struct from a FHIR StructureDefinition
    pub fn generate_struct(
        &mut self,
        structure_def: &StructureDefinition,
    ) -> CodegenResult<RustStruct> {
        let struct_name = self.to_rust_type_name(&structure_def.name);

        // Get elements from snapshot or differential
        let elements = if let Some(snapshot) = &structure_def.snapshot {
            &snapshot.element
        } else if let Some(differential) = &structure_def.differential {
            &differential.element
        } else {
            return Err(CodegenError::MissingField {
                field: "snapshot or differential".to_string(),
            });
        };

        let mut fields = Vec::new();
        let mut processed_paths = std::collections::HashSet::new();

        for element in elements {
            // Skip the root element (e.g., "Patient")
            if element.path == structure_def.base_type {
                continue;
            }

            // Extract field name from path (e.g., "Patient.name" -> "name")
            let field_parts: Vec<&str> = element.path.split('.').collect();
            if field_parts.len() < 2 {
                continue;
            }

            let field_name = field_parts[1];
            let field_path = format!("{}.{}", structure_def.base_type, field_name);

            // Skip if we've already processed this field
            if processed_paths.contains(&field_path) {
                continue;
            }
            processed_paths.insert(field_path);

            let rust_field = self.element_to_rust_field(element, field_name)?;
            fields.push(rust_field);
        }

        let mut derives = vec!["Debug".to_string(), "Clone".to_string()];
        if self.config.with_serde {
            derives.extend_from_slice(&["Serialize".to_string(), "Deserialize".to_string()]);
        }

        let rust_struct = RustStruct {
            name: struct_name,
            fields,
            documentation: structure_def
                .title
                .clone()
                .or_else(|| Some(format!("FHIR {} resource", structure_def.name))),
            derives,
        };

        Ok(rust_struct)
    }

    /// Convert an ElementDefinition to a RustField
    fn element_to_rust_field(
        &self,
        element: &ElementDefinition,
        field_name: &str,
    ) -> CodegenResult<RustField> {
        let rust_name = self.to_rust_field_name(field_name);
        let is_optional = element.min.unwrap_or(0) == 0;
        let is_array = element.max.as_deref() == Some("*")
            || element
                .max
                .as_deref()
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap_or(1)
                > 1;

        let rust_type = if let Some(element_types) = &element.element_type {
            if element_types.len() == 1 {
                self.fhir_type_to_rust_type(&element_types[0].code)?
            } else {
                // Multiple types - use an enum or serde_json::Value for now
                "serde_json::Value".to_string()
            }
        } else {
            // No type specified, might be a complex type
            "serde_json::Value".to_string()
        };

        let serde_rename = if rust_name != field_name && self.config.with_serde {
            Some(field_name.to_string())
        } else {
            None
        };

        Ok(RustField {
            name: rust_name,
            rust_type,
            is_optional,
            is_array,
            documentation: element.short.clone().or_else(|| element.definition.clone()),
            serde_rename,
        })
    }

    /// Convert FHIR type to Rust type
    fn fhir_type_to_rust_type(&self, fhir_type: &str) -> CodegenResult<String> {
        if let Some(mapped_type) = self.config.type_mappings.get(fhir_type) {
            Ok(mapped_type.clone())
        } else if fhir_type.chars().next().unwrap_or('a').is_uppercase() {
            // Likely a complex FHIR type, convert to PascalCase
            Ok(self.to_rust_type_name(fhir_type))
        } else {
            // Unknown primitive type, default to String
            Ok("String".to_string())
        }
    }

    /// Convert a name to Rust type name (PascalCase)
    fn to_rust_type_name(&self, name: &str) -> String {
        name.to_case(Case::Pascal)
    }

    /// Convert a name to Rust field name (snake_case)
    fn to_rust_field_name(&self, name: &str) -> String {
        name.to_case(Case::Snake)
    }

    /// Generate TokenStream for a RustStruct
    pub fn generate_tokens(&self, rust_struct: &RustStruct) -> TokenStream {
        let struct_name = format_ident!("{}", rust_struct.name);
        let derives: Vec<_> = rust_struct
            .derives
            .iter()
            .map(|d| format_ident!("{}", d))
            .collect();

        let fields: Vec<_> = rust_struct
            .fields
            .iter()
            .map(|field| {
                let field_name = format_ident!("{}", field.name);
                let field_type = self.build_field_type(field);

                let mut attrs = Vec::new();

                // Add serde rename if needed
                if let Some(rename) = &field.serde_rename {
                    attrs.push(quote! { #[serde(rename = #rename)] });
                }

                // Add documentation
                if let Some(doc) = &field.documentation {
                    attrs.push(quote! { #[doc = #doc] });
                }

                quote! {
                    #(#attrs)*
                    pub #field_name: #field_type
                }
            })
            .collect();

        let doc_attr = if let Some(doc) = &rust_struct.documentation {
            quote! { #[doc = #doc] }
        } else {
            quote! {}
        };

        quote! {
            #doc_attr
            #[derive(#(#derives),*)]
            pub struct #struct_name {
                #(#fields,)*
            }
        }
    }

    /// Build the complete type for a field (handling Option and Vec)
    fn build_field_type(&self, field: &RustField) -> Type {
        let base_type: Type = syn::parse_str(&field.rust_type).unwrap_or_else(|_| {
            parse_quote! { String }
        });

        let wrapped_type = if field.is_array {
            parse_quote! { Vec<#base_type> }
        } else {
            base_type
        };

        if field.is_optional {
            parse_quote! { Option<#wrapped_type> }
        } else {
            wrapped_type
        }
    }

    /// Generate code and write to file
    pub fn generate_to_file(
        &mut self,
        structure_def: &StructureDefinition,
        output_path: &Path,
    ) -> CodegenResult<()> {
        let rust_struct = self.generate_struct(structure_def)?;
        let tokens = self.generate_tokens(&rust_struct);

        // Create output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the generated code
        let code = if self.config.with_serde {
            format!(
                "use serde::{{Deserialize, Serialize}};\n\n{}",
                tokens.to_string()
            )
        } else {
            tokens.to_string()
        };

        fs::write(output_path, code)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CodegenConfig::default();
        assert_eq!(config.module_name, "fhir_types");
        assert!(config.with_serde);
        assert!(config.type_mappings.contains_key("string"));
    }

    #[test]
    fn test_rust_name_conversion() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        assert_eq!(generator.to_rust_type_name("Patient"), "Patient");
        assert_eq!(generator.to_rust_type_name("humanName"), "HumanName");
        assert_eq!(generator.to_rust_field_name("birthDate"), "birth_date");
        assert_eq!(
            generator.to_rust_field_name("resourceType"),
            "resource_type"
        );
    }

    #[test]
    fn test_fhir_type_mapping() {
        let config = CodegenConfig::default();
        let generator = CodeGenerator::new(config);

        assert_eq!(
            generator.fhir_type_to_rust_type("string").unwrap(),
            "String"
        );
        assert_eq!(generator.fhir_type_to_rust_type("integer").unwrap(), "i32");
        assert_eq!(generator.fhir_type_to_rust_type("boolean").unwrap(), "bool");
        assert_eq!(
            generator.fhir_type_to_rust_type("Patient").unwrap(),
            "Patient"
        );
    }
}
