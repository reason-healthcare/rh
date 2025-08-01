//! FHIR validation module
//!
//! This module handles the actual FHIR resource validation logic.

use crate::setup::{FhirSetup, FhirValidationError};
use crate::{ValidationError, ValidationResult};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// FHIR validator that performs resource validation
pub struct FhirValidator {
    /// Setup manager for handling packages and Rust type generation
    setup: FhirSetup,
}

impl FhirValidator {
    /// Create a new FHIR validator with default settings
    pub fn new() -> Result<Self, FhirValidationError> {
        Ok(Self {
            setup: FhirSetup::new()?,
        })
    }

    /// Create a new FHIR validator with custom base directory
    pub fn with_base_dir(base_dir: PathBuf) -> Result<Self, FhirValidationError> {
        Ok(Self {
            setup: FhirSetup::with_base_dir(base_dir)?,
        })
    }

    /// Create a new FHIR validator with a specific package directory
    pub fn with_package_dir(package_dir: &Path) -> Self {
        // For now, fall back to default if we can't create with the specific directory
        Self::with_base_dir(package_dir.to_path_buf()).unwrap_or_else(|_| {
            Self::new().unwrap_or_else(|_| {
                // Panic is probably not the best, but for now this maintains API compatibility
                panic!("Failed to create FHIR validator")
            })
        })
    }

    /// Set the default FHIR version for validation (builder pattern)
    pub fn with_default_version(self, _version: &str) -> Self {
        // TODO: Store the version in the validator for default use
        self
    }

    /// Get access to the setup manager
    pub fn setup(&self) -> &FhirSetup {
        &self.setup
    }

    /// Validate multiple FHIR resources with an optional version
    pub async fn validate_multiple(
        &self,
        content: &str,
        version: Option<&str>,
    ) -> Result<Vec<(usize, ValidationResult)>, FhirValidationError> {
        let mut results = Vec::new();
        let default_version = "4.0.1"; // Default FHIR version
        let version_to_use = version.unwrap_or(default_version);

        // Split content by lines and try to parse each line as a separate JSON resource
        for (line_number, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Try to parse and validate each line as a FHIR resource
            match self.validate_with_version(line, version_to_use).await {
                Ok(validation_result) => {
                    results.push((line_number + 1, validation_result)); // 1-indexed line numbers
                }
                Err(e) => {
                    // Convert validation errors to Invalid results
                    let error = ValidationError::Schema {
                        message: format!("Line {}: {}", line_number + 1, e),
                    };
                    results.push((line_number + 1, ValidationResult::Invalid(vec![error])));
                }
            }
        }

        Ok(results)
    }

    /// Validate a FHIR resource with a specific version
    pub async fn validate_with_version(
        &self,
        content: &str,
        version: &str,
    ) -> Result<ValidationResult, FhirValidationError> {
        // Parse the JSON content
        let resource: Value = serde_json::from_str(content)?;

        // Perform basic structure validation
        self.validate_basic_structure(&resource, version)
    }

    /// Perform basic FHIR resource structure validation
    fn validate_basic_structure(
        &self,
        resource: &Value,
        _version: &str,
    ) -> Result<ValidationResult, FhirValidationError> {
        let mut errors = Vec::new();

        // Check if it's an object
        if !resource.is_object() {
            errors.push(ValidationError::Schema {
                message: "FHIR resource must be a JSON object".to_string(),
            });
            return Ok(ValidationResult::Invalid(errors));
        }

        let obj = resource.as_object().unwrap();

        // Check for required resourceType field
        match obj.get("resourceType") {
            Some(Value::String(resource_type)) => {
                if resource_type.is_empty() {
                    errors.push(ValidationError::Schema {
                        message: "resourceType cannot be empty".to_string(),
                    });
                }
            }
            Some(_) => {
                errors.push(ValidationError::Schema {
                    message: "resourceType must be a string".to_string(),
                });
            }
            None => {
                errors.push(ValidationError::Schema {
                    message: "Missing required field: resourceType".to_string(),
                });
            }
        }

        if errors.is_empty() {
            Ok(ValidationResult::Valid)
        } else {
            Ok(ValidationResult::Invalid(errors))
        }
    }
}

impl Default for FhirValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default FHIR validator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_valid_fhir_resource() {
        let validator = FhirValidator::new().unwrap();
        let valid_json = r#"{"resourceType": "Patient", "id": "example"}"#;

        let result = validator
            .validate_with_version(valid_json, "4.0.1")
            .await
            .unwrap();
        assert!(matches!(result, ValidationResult::Valid));
    }

    #[tokio::test]
    async fn test_validate_invalid_json() {
        let validator = FhirValidator::new().unwrap();
        let invalid_json = r#"{"resourceType": "Patient", "id": "example""#; // Missing closing brace

        let result = validator.validate_with_version(invalid_json, "4.0.1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_missing_resource_type() {
        let validator = FhirValidator::new().unwrap();
        let no_resource_type = r#"{"id": "example"}"#;

        let result = validator
            .validate_with_version(no_resource_type, "4.0.1")
            .await
            .unwrap();
        if let ValidationResult::Invalid(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(errors[0]
                .to_string()
                .contains("Missing required field: resourceType"));
        } else {
            panic!("Expected Invalid result");
        }
    }

    #[tokio::test]
    async fn test_validate_empty_resource_type() {
        let validator = FhirValidator::new().unwrap();
        let empty_resource_type = r#"{"resourceType": "", "id": "example"}"#;

        let result = validator
            .validate_with_version(empty_resource_type, "4.0.1")
            .await
            .unwrap();
        if let ValidationResult::Invalid(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(errors[0]
                .to_string()
                .contains("resourceType cannot be empty"));
        } else {
            panic!("Expected Invalid result");
        }
    }

    #[tokio::test]
    async fn test_validate_multiple_resources() {
        let validator = FhirValidator::new().unwrap();
        let multiple_resources = r#"{"resourceType": "Patient", "id": "1"}
{"resourceType": "Observation", "id": "2"}
{"id": "3"}"#; // This one is missing resourceType

        let results = validator
            .validate_multiple(multiple_resources, Some("4.0.1"))
            .await
            .unwrap();
        assert_eq!(results.len(), 3);

        // First two should be valid
        assert!(matches!(results[0].1, ValidationResult::Valid));
        assert!(matches!(results[1].1, ValidationResult::Valid));

        // Third should be invalid
        assert!(matches!(results[2].1, ValidationResult::Invalid(_)));
    }
}
