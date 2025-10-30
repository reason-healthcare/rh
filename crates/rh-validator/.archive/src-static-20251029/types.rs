//! Core validation types and structures

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// Re-export shared types from rh-foundation
pub use rh_foundation::{Invariant, Severity};

/// Type of validation issue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssueCode {
    /// Structural issue (missing field, wrong type, etc.)
    Structure,
    /// Required field is missing
    Required,
    /// Value doesn't match expected type
    ValueType,
    /// Invariant constraint failed
    Invariant,
    /// Error evaluating invariant expression
    InvariantEvaluation,
    /// Cardinality constraint violated
    Cardinality,
    /// Invalid code or coding
    CodeInvalid,
    /// Unknown or unexpected element
    Unknown,
    /// Business rule violation
    BusinessRule,
    /// Other validation issue
    Other,
}

impl fmt::Display for IssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueCode::Structure => write!(f, "structure"),
            IssueCode::Required => write!(f, "required"),
            IssueCode::ValueType => write!(f, "value-type"),
            IssueCode::Invariant => write!(f, "invariant"),
            IssueCode::InvariantEvaluation => write!(f, "invariant-evaluation"),
            IssueCode::Cardinality => write!(f, "cardinality"),
            IssueCode::CodeInvalid => write!(f, "code-invalid"),
            IssueCode::Unknown => write!(f, "unknown"),
            IssueCode::BusinessRule => write!(f, "business-rule"),
            IssueCode::Other => write!(f, "other"),
        }
    }
}

/// A single validation issue found during validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity of the issue
    pub severity: Severity,
    /// Type of issue
    pub code: IssueCode,
    /// Human-readable description
    pub details: String,
    /// FHIRPath expression (for invariant failures)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Location in the resource (JSON path or element path)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Invariant key (e.g., "pat-1")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invariant_key: Option<String>,
}

impl ValidationIssue {
    /// Create a new validation issue
    pub fn new(severity: Severity, code: IssueCode, details: impl Into<String>) -> Self {
        Self {
            severity,
            code,
            details: details.into(),
            expression: None,
            location: None,
            invariant_key: None,
        }
    }

    /// Add FHIRPath expression context
    pub fn with_expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }

    /// Add location context
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Add invariant key
    pub fn with_invariant_key(mut self, key: impl Into<String>) -> Self {
        self.invariant_key = Some(key.into());
        self
    }

    /// Check if this is an error (vs warning or info)
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    /// Check if this is a warning
    pub fn is_warning(&self) -> bool {
        self.severity == Severity::Warning
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.code, self.details)?;
        if let Some(ref loc) = self.location {
            write!(f, " at {loc}")?;
        }
        if let Some(ref key) = self.invariant_key {
            write!(f, " ({key})")?;
        }
        Ok(())
    }
}

/// Result of validating a FHIR resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Resource type that was validated
    pub resource_type: String,
    /// Resource ID (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// All validation issues found
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new(resource_type: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            resource_id: None,
            issues: Vec::new(),
        }
    }

    /// Set the resource ID
    pub fn with_resource_id(mut self, id: impl Into<String>) -> Self {
        self.resource_id = Some(id.into());
        self
    }

    /// Add a validation issue
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    /// Add multiple validation issues
    pub fn add_issues(&mut self, issues: impl IntoIterator<Item = ValidationIssue>) {
        self.issues.extend(issues);
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.is_error())
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.is_warning())
    }

    /// Get all error issues
    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.is_error())
    }

    /// Get all warning issues
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.is_warning())
    }

    /// Get count of errors
    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    /// Get count of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid() {
            write!(f, "✅ Validation passed for {}", self.resource_type)?;
            if let Some(ref id) = self.resource_id {
                write!(f, " (id: {id})")?;
            }
        } else {
            write!(
                f,
                "❌ Validation failed for {} ({} errors, {} warnings)",
                self.resource_type,
                self.error_count(),
                self.warning_count()
            )?;
            if let Some(ref id) = self.resource_id {
                write!(f, " (id: {id})")?;
            }
        }
        Ok(())
    }
}

/// Validator-specific errors
#[derive(Error, Debug)]
pub enum ValidatorError {
    /// JSON parsing error
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// FHIRPath evaluation error
    #[error("FHIRPath evaluation error: {0}")]
    FhirPath(String),

    /// Resource type not recognized
    #[error("Unknown resource type: {0}")]
    UnknownResourceType(String),

    /// Other error
    #[error("Validator error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Warning > Severity::Information);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Error.to_string(), "error");
        assert_eq!(Severity::Warning.to_string(), "warning");
        assert_eq!(Severity::Information.to_string(), "information");
    }

    #[test]
    fn test_issue_code_display() {
        assert_eq!(IssueCode::Structure.to_string(), "structure");
        assert_eq!(IssueCode::Invariant.to_string(), "invariant");
    }

    #[test]
    fn test_validation_issue_creation() {
        let issue = ValidationIssue::new(
            Severity::Error,
            IssueCode::Required,
            "Missing required field",
        )
        .with_location("Patient.name")
        .with_invariant_key("pat-1");

        assert!(issue.is_error());
        assert!(!issue.is_warning());
        assert_eq!(issue.location, Some("Patient.name".to_string()));
        assert_eq!(issue.invariant_key, Some("pat-1".to_string()));
    }

    #[test]
    fn test_validation_result_creation() {
        let mut result = ValidationResult::new("Patient").with_resource_id("patient-123");

        assert!(result.is_valid());
        assert!(!result.has_errors());
        assert!(!result.has_warnings());

        result.add_issue(ValidationIssue::new(
            Severity::Error,
            IssueCode::Required,
            "Missing name",
        ));

        assert!(!result.is_valid());
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 0);
    }

    #[test]
    fn test_validation_result_with_warnings() {
        let mut result = ValidationResult::new("Patient");

        result.add_issue(ValidationIssue::new(
            Severity::Warning,
            IssueCode::Unknown,
            "Unknown field",
        ));

        assert!(result.is_valid()); // Warnings don't fail validation
        assert!(!result.has_errors());
        assert!(result.has_warnings());
    }

    #[test]
    fn test_validation_result_display() {
        let result = ValidationResult::new("Patient").with_resource_id("pat-1");
        assert!(result.to_string().contains("✅"));
        assert!(result.to_string().contains("Patient"));
        assert!(result.to_string().contains("pat-1"));
    }

    #[test]
    fn test_validation_issue_json_serialization() {
        let issue = ValidationIssue::new(Severity::Error, IssueCode::Invariant, "Test error")
            .with_expression("Patient.name.exists()")
            .with_location("Patient.name");

        let json = serde_json::to_string(&issue).unwrap();
        let deserialized: ValidationIssue = serde_json::from_str(&json).unwrap();

        assert_eq!(issue.severity, deserialized.severity);
        assert_eq!(issue.code, deserialized.code);
        assert_eq!(issue.details, deserialized.details);
    }
}
