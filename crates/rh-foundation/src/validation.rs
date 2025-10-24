//! FHIR validation types
//!
//! Shared types for FHIR validation that are used across multiple crates.

use serde::{Deserialize, Serialize};

/// Severity level of a validation issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Fatal error - validation failed
    Error,
    /// Warning - best practice violation
    Warning,
    /// Informational message
    Information,
}

impl Severity {
    fn rank(&self) -> u8 {
        match self {
            Severity::Error => 2,
            Severity::Warning => 1,
            Severity::Information => 0,
        }
    }
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Information => write!(f, "information"),
        }
    }
}

/// Invariant constraint from FHIR specification
///
/// This structure is shared between the validator and code generator.
/// Generated resources embed these as static constants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invariant {
    /// Invariant key (e.g., "pat-1", "obs-3")
    pub key: String,
    /// Severity level
    pub severity: Severity,
    /// Human-readable description
    pub human: String,
    /// FHIRPath expression to evaluate
    pub expression: String,
    /// XPath expression (legacy, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<String>,
}

impl Invariant {
    /// Create a new invariant
    pub fn new(
        key: impl Into<String>,
        severity: Severity,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            severity,
            human: human.into(),
            expression: expression.into(),
            xpath: None,
        }
    }

    /// Add XPath expression (legacy support)
    pub fn with_xpath(mut self, xpath: impl Into<String>) -> Self {
        self.xpath = Some(xpath.into());
        self
    }
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
    fn test_invariant_creation() {
        let inv = Invariant::new(
            "pat-1",
            Severity::Error,
            "Name is required",
            "name.exists()",
        );
        assert_eq!(inv.key, "pat-1");
        assert_eq!(inv.severity, Severity::Error);
        assert_eq!(inv.human, "Name is required");
        assert_eq!(inv.expression, "name.exists()");
        assert!(inv.xpath.is_none());
    }

    #[test]
    fn test_invariant_with_xpath() {
        let inv = Invariant::new("obs-1", Severity::Warning, "Code required", "code.exists()")
            .with_xpath("f:code");
        assert!(inv.xpath.is_some());
        assert_eq!(inv.xpath.unwrap(), "f:code");
    }

    #[test]
    fn test_invariant_serialization() {
        let inv = Invariant::new("test-1", Severity::Error, "Test", "true");
        let json = serde_json::to_string(&inv).unwrap();
        assert!(json.contains("test-1"));
        assert!(json.contains("error"));
    }
}
