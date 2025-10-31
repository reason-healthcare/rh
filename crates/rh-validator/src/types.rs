//! Core types for FHIR validation.
//!
//! This module provides the fundamental types used throughout the validation process:
//!
//! - [`ValidationResult`] - Result of validating a FHIR resource
//! - [`ValidationIssue`] - Individual validation issue
//! - [`Severity`] - Issue severity levels
//! - [`IssueCode`] - FHIR IssueType codes
//! - [`OperationOutcome`] - FHIR OperationOutcome resource representation

use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity level of a validation issue.
///
/// Maps to FHIR IssueSeverity codes (subset):
/// - `Error` - Issue prevents the resource from being accepted
/// - `Warning` - Issue may cause problems but doesn't prevent acceptance
/// - `Information` - Informational note about the resource
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical error that prevents resource acceptance
    Error,
    /// Warning that may cause issues
    Warning,
    /// Informational message
    Information,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Information => write!(f, "information"),
        }
    }
}

/// FHIR IssueType code classification.
///
/// Maps to FHIR IssueType ValueSet codes. Used to categorize validation issues
/// according to FHIR standards.
///
/// # Examples
///
/// ```
/// use rh_validator::IssueCode;
///
/// let code = IssueCode::Required;
/// assert_eq!(code.to_string(), "required");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueCode {
    Structure,
    Required,
    Value,
    Invariant,
    Invalid,
    CodeInvalid,
    Extension,
    Forbidden,
    Incomplete,
    TooCostly,
    BusinessRule,
    Conflict,
    NotSupported,
    Duplicate,
    NotFound,
    TooLong,
    CodeInvalidInValueSet,
    InvalidDisplay,
    Processing,
    NotAllowed,
    Informational,
}

impl fmt::Display for IssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueCode::Structure => write!(f, "structure"),
            IssueCode::Required => write!(f, "required"),
            IssueCode::Value => write!(f, "value"),
            IssueCode::Invariant => write!(f, "invariant"),
            IssueCode::Invalid => write!(f, "invalid"),
            IssueCode::CodeInvalid => write!(f, "code-invalid"),
            IssueCode::Extension => write!(f, "extension"),
            IssueCode::Forbidden => write!(f, "forbidden"),
            IssueCode::Incomplete => write!(f, "incomplete"),
            IssueCode::TooCostly => write!(f, "too-costly"),
            IssueCode::BusinessRule => write!(f, "business-rule"),
            IssueCode::Conflict => write!(f, "conflict"),
            IssueCode::NotSupported => write!(f, "not-supported"),
            IssueCode::Duplicate => write!(f, "duplicate"),
            IssueCode::NotFound => write!(f, "not-found"),
            IssueCode::TooLong => write!(f, "too-long"),
            IssueCode::CodeInvalidInValueSet => write!(f, "code-invalid-in-valueset"),
            IssueCode::InvalidDisplay => write!(f, "invalid-display"),
            IssueCode::Processing => write!(f, "processing"),
            IssueCode::NotAllowed => write!(f, "not-allowed"),
            IssueCode::Informational => write!(f, "informational"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub code: IssueCode,
    pub message: String,
    pub path: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl ValidationIssue {
    pub fn new(severity: Severity, code: IssueCode, message: impl Into<String>) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            path: None,
            location: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.location = Some(Location { line, column });
        self
    }

    pub fn error(code: IssueCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Error, code, message)
    }

    pub fn warning(code: IssueCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, code, message)
    }

    pub fn info(code: IssueCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Information, code, message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            issues: Vec::new(),
        }
    }

    pub fn invalid(issues: Vec<ValidationIssue>) -> Self {
        let valid = !issues.iter().any(|i| i.severity == Severity::Error);
        Self { valid, issues }
    }

    pub fn with_issue(mut self, issue: ValidationIssue) -> Self {
        if issue.severity == Severity::Error {
            self.valid = false;
        }
        self.issues.push(issue);
        self
    }

    pub fn merge(&mut self, other: ValidationResult) {
        if !other.valid {
            self.valid = false;
        }
        self.issues.extend(other.issues);
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }

    pub fn info_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Information)
            .count()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::valid()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationOutcome {
    pub resource_type: String,
    pub issue: Vec<OperationOutcomeIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationOutcomeIssue {
    pub severity: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Vec<String>>,
}

impl ValidationResult {
    pub fn to_operation_outcome(&self) -> OperationOutcome {
        let issues = self
            .issues
            .iter()
            .map(|issue| {
                let severity = match issue.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                    Severity::Information => "information",
                };

                let code = map_issue_code_to_fhir(&issue.code);

                let location = issue.path.as_ref().map(|p| vec![p.clone()]);
                let expression = issue.path.as_ref().map(|p| vec![p.clone()]);

                OperationOutcomeIssue {
                    severity: severity.to_string(),
                    code: code.to_string(),
                    diagnostics: Some(issue.message.clone()),
                    location,
                    expression,
                }
            })
            .collect();

        OperationOutcome {
            resource_type: "OperationOutcome".to_string(),
            issue: issues,
        }
    }
}

fn map_issue_code_to_fhir(code: &IssueCode) -> &'static str {
    match code {
        IssueCode::Structure => "structure",
        IssueCode::Required => "required",
        IssueCode::Value => "value",
        IssueCode::Invariant => "invariant",
        IssueCode::Invalid => "invalid",
        IssueCode::CodeInvalid => "code-invalid",
        IssueCode::Extension => "extension",
        IssueCode::Forbidden => "security",
        IssueCode::Incomplete => "incomplete",
        IssueCode::TooCostly => "too-costly",
        IssueCode::BusinessRule => "business-rule",
        IssueCode::Conflict => "conflict",
        IssueCode::NotSupported => "not-supported",
        IssueCode::Duplicate => "duplicate",
        IssueCode::NotFound => "not-found",
        IssueCode::TooLong => "too-long",
        IssueCode::CodeInvalidInValueSet => "code-invalid",
        IssueCode::InvalidDisplay => "invalid",
        IssueCode::Processing => "processing",
        IssueCode::NotAllowed => "business-rule",
        IssueCode::Informational => "informational",
    }
}
