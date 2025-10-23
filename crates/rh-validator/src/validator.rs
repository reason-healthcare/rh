//! FHIR validation implementation
//!
//! Provides structural validation via Rust's type system and serde deserialization.

use crate::types::{IssueCode, Severity, ValidationIssue, ValidationResult, ValidatorError};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

/// Configuration for FHIR validator
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Maximum nesting depth for JSON documents
    pub max_depth: usize,
    /// Whether to report unknown fields as warnings
    pub warn_on_unknown_fields: bool,
    /// Custom package directory for profiles (future use)
    pub package_dir: Option<PathBuf>,
    /// FHIR version to validate against
    pub fhir_version: String,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            warn_on_unknown_fields: true,
            package_dir: None,
            fhir_version: "4.0.1".to_string(),
        }
    }
}

impl ValidatorConfig {
    /// Create a new configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum nesting depth
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Set whether to warn on unknown fields
    pub fn with_warn_on_unknown_fields(mut self, warn: bool) -> Self {
        self.warn_on_unknown_fields = warn;
        self
    }

    /// Set custom package directory
    pub fn with_package_dir(mut self, package_dir: PathBuf) -> Self {
        self.package_dir = Some(package_dir);
        self
    }

    /// Set FHIR version
    pub fn with_fhir_version(mut self, version: impl Into<String>) -> Self {
        self.fhir_version = version.into();
        self
    }
}

/// FHIR resource validator
///
/// Performs structural validation via Rust's type system and serde deserialization.
/// Future phases will add FHIRPath invariant validation and profile support.
#[derive(Debug, Clone, Default)]
pub struct FhirValidator {
    config: ValidatorConfig,
}

impl FhirValidator {
    /// Create a new FHIR validator with default configuration
    pub fn new() -> Result<Self, ValidatorError> {
        Ok(Self {
            config: ValidatorConfig::default(),
        })
    }

    /// Create a validator with custom configuration
    pub fn with_config(config: ValidatorConfig) -> Result<Self, ValidatorError> {
        Ok(Self { config })
    }

    /// Create a validator with a custom package directory
    pub fn with_package_dir(package_dir: PathBuf) -> Result<Self, ValidatorError> {
        Ok(Self {
            config: ValidatorConfig::default().with_package_dir(package_dir),
        })
    }

    /// Get the validator configuration
    pub fn config(&self) -> &ValidatorConfig {
        &self.config
    }

    /// Validate a FHIR resource from JSON string
    ///
    /// This performs structural validation by deserializing into the typed resource.
    /// Type parameter T should be a FHIR resource type from rh-hl7_fhir_r4_core.
    pub fn validate_json<T: DeserializeOwned>(
        &self,
        json: &str,
    ) -> Result<ValidationResult, ValidatorError> {
        // Parse JSON to determine resource type
        let json_value: serde_json::Value = serde_json::from_str(json)?;

        let resource_type = json_value
            .get("resourceType")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let mut result = ValidationResult::new(resource_type);

        // Attempt to deserialize into the typed resource
        match serde_json::from_str::<T>(json) {
            Ok(_resource) => {
                // Structural validation passed
                // Phase 3 will add invariant validation here
            }
            Err(e) => {
                // Convert serde error to validation issue
                let issue = convert_serde_error(&e, resource_type);
                result.add_issue(issue);
            }
        }

        Ok(result)
    }

    /// Validate a FHIR resource that's already been parsed
    ///
    /// This is useful when you already have a deserialized resource and want to validate it.
    pub fn validate<T: DeserializeOwned + serde::Serialize>(
        &self,
        resource: &T,
    ) -> Result<ValidationResult, ValidatorError> {
        // For now, we need to serialize then deserialize to validate structure
        // This is inefficient but will work for Phase 1
        // Phase 3 will optimize this
        let json = serde_json::to_string(resource)
            .map_err(|e| ValidatorError::Other(format!("Failed to serialize: {e}")))?;
        self.validate_json::<T>(&json)
    }

    /// Validate with specific FHIR version (compatibility method)
    pub fn validate_with_version(
        &self,
        json: &str,
        _version: &str,
    ) -> Result<ValidationResult, ValidatorError> {
        // For now, use serde_json::Value as generic type
        self.validate_json::<serde_json::Value>(json)
    }

    /// Validate a FHIR resource from JSON with explicit resource type
    pub fn validate_resource(
        &self,
        _resource_type: &str,
        json: &str,
    ) -> Result<ValidationResult, ValidatorError> {
        self.validate_json::<serde_json::Value>(json)
    }

    /// Validate multiple FHIR resources (NDJSON format)
    pub fn validate_multiple(
        &self,
        json: &str,
        _version: Option<&str>,
    ) -> Result<Vec<(usize, ValidationResult)>, ValidatorError> {
        let mut results = Vec::new();

        for (line_number, line) in json.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let result = self.validate_json::<serde_json::Value>(line)?;
            results.push((line_number + 1, result));
        }

        Ok(results)
    }
}

/// Convert a serde JSON error to a ValidationIssue
fn convert_serde_error(error: &serde_json::Error, resource_type: &str) -> ValidationIssue {
    let error_msg = error.to_string();

    // Determine issue code based on error message patterns
    let (code, severity) = if error_msg.contains("missing field") {
        (IssueCode::Required, Severity::Error)
    } else if error_msg.contains("invalid type") {
        (IssueCode::ValueType, Severity::Error)
    } else if error_msg.contains("unknown field") {
        (IssueCode::Unknown, Severity::Warning)
    } else {
        (IssueCode::Structure, Severity::Error)
    };

    // Extract location information from error
    let location = extract_location_from_error(&error_msg, resource_type);

    // Build detailed error message with line/column if available
    let details = if error.line() > 0 {
        format!(
            "{} at line {}, column {}",
            error_msg,
            error.line(),
            error.column()
        )
    } else {
        error_msg
    };

    let mut issue = ValidationIssue::new(severity, code, details);

    if let Some(loc) = location {
        issue = issue.with_location(loc);
    }

    issue
}

/// Extract location path from serde error message
fn extract_location_from_error(error_msg: &str, resource_type: &str) -> Option<String> {
    // Try to extract field names from error messages
    if let Some(field_start) = error_msg.find("field `") {
        if let Some(field_end) = error_msg[field_start + 7..].find('`') {
            let field_name = &error_msg[field_start + 7..field_start + 7 + field_end];
            return Some(format!("{resource_type}.{field_name}"));
        }
    }

    // Default to just resource type
    Some(resource_type.to_string())
}

/// JSON syntax validator
///
/// Validates JSON syntax without requiring typed resources.
#[derive(Debug, Clone)]
pub struct JsonValidator {
    max_depth: usize,
}

impl JsonValidator {
    /// Create a validator with custom max depth
    pub fn with_max_depth(max_depth: usize) -> Self {
        Self { max_depth }
    }

    /// Validate JSON syntax
    pub fn validate(&self, json: &str) -> ValidationResult {
        let mut result = ValidationResult::new("JSON");

        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(value) => {
                // Check nesting depth
                let depth = calculate_depth(&value);
                if depth > self.max_depth {
                    result.add_issue(ValidationIssue::new(
                        Severity::Error,
                        IssueCode::Structure,
                        format!(
                            "JSON nesting depth {} exceeds maximum {}",
                            depth, self.max_depth
                        ),
                    ));
                }
            }
            Err(e) => {
                let details = if e.line() > 0 {
                    format!(
                        "JSON syntax error at line {}, column {}: {}",
                        e.line(),
                        e.column(),
                        e
                    )
                } else {
                    e.to_string()
                };

                result.add_issue(ValidationIssue::new(
                    Severity::Error,
                    IssueCode::Structure,
                    details,
                ));
            }
        }

        result
    }

    /// Validate multiple JSON documents (NDJSON format)
    pub fn validate_multiple(&self, json: &str) -> Vec<(usize, ValidationResult)> {
        json.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(idx, line)| (idx + 1, self.validate(line)))
            .collect()
    }
}

impl Default for JsonValidator {
    fn default() -> Self {
        Self { max_depth: 100 }
    }
}

/// Calculate the maximum nesting depth of a JSON value
fn calculate_depth(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => 1 + map.values().map(calculate_depth).max().unwrap_or(0),
        serde_json::Value::Array(arr) => 1 + arr.iter().map(calculate_depth).max().unwrap_or(0),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_config_builder() {
        let config = ValidatorConfig::new()
            .with_max_depth(50)
            .with_warn_on_unknown_fields(false)
            .with_fhir_version("4.0.1");

        assert_eq!(config.max_depth, 50);
        assert!(!config.warn_on_unknown_fields);
        assert_eq!(config.fhir_version, "4.0.1");
    }

    #[test]
    fn test_fhir_validator_creation() {
        let validator = FhirValidator::new().unwrap();
        assert_eq!(validator.config().max_depth, 100);

        let validator =
            FhirValidator::with_config(ValidatorConfig::new().with_max_depth(50)).unwrap();
        assert_eq!(validator.config().max_depth, 50);
    }

    #[test]
    fn test_json_validator_valid_json() {
        let validator = JsonValidator::default();
        let result = validator.validate(r#"{"resourceType": "Patient", "id": "123"}"#);
        assert!(result.is_valid());
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn test_json_validator_invalid_json() {
        let validator = JsonValidator::default();
        let result = validator.validate(r#"{"resourceType": "Patient", "id": 123"#);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn test_json_validator_depth_limit() {
        let validator = JsonValidator::with_max_depth(2);
        let deep_json = r#"{"a": {"b": {"c": {"d": 1}}}}"#;
        let result = validator.validate(deep_json);
        assert!(!result.is_valid());
        assert!(result.errors().any(|e| e.details.contains("nesting depth")));
    }

    #[test]
    fn test_calculate_depth() {
        let shallow = serde_json::json!({"a": 1, "b": 2});
        assert_eq!(calculate_depth(&shallow), 1);

        let nested = serde_json::json!({"a": {"b": {"c": 1}}});
        assert_eq!(calculate_depth(&nested), 3);

        let array = serde_json::json!([1, 2, [3, 4, [5]]]);
        assert_eq!(calculate_depth(&array), 3);
    }

    #[test]
    fn test_convert_serde_error() {
        let json = r#"{"resourceType": "Patient"}"#;
        let error: serde_json::Error = serde_json::from_str::<serde_json::Value>(json)
            .err()
            .unwrap_or_else(|| {
                // Force an error for testing
                serde_json::from_str::<()>(json).unwrap_err()
            });

        // This test just ensures the function doesn't panic
        let _issue = convert_serde_error(&error, "Patient");
    }
}
