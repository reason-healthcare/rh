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

/// Binding strength for ValueSet bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BindingStrength {
    /// Value MUST come from the ValueSet
    Required,
    /// Value SHOULD come from the ValueSet (can use other codes if needed)
    Extensible,
    /// Value is recommended to come from the ValueSet
    Preferred,
    /// ValueSet is for illustration only
    Example,
}

impl BindingStrength {
    /// Parse from FHIR string representation
    pub fn from_fhir_str(s: &str) -> Option<Self> {
        match s {
            "required" => Some(BindingStrength::Required),
            "extensible" => Some(BindingStrength::Extensible),
            "preferred" => Some(BindingStrength::Preferred),
            "example" => Some(BindingStrength::Example),
            _ => None,
        }
    }
}

impl std::fmt::Display for BindingStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindingStrength::Required => write!(f, "required"),
            BindingStrength::Extensible => write!(f, "extensible"),
            BindingStrength::Preferred => write!(f, "preferred"),
            BindingStrength::Example => write!(f, "example"),
        }
    }
}

/// Element binding to a ValueSet
///
/// This structure is shared between the validator and code generator.
/// Generated resources embed these as static constants for elements with required bindings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ElementBinding {
    /// Element path (e.g., "Patient.gender")
    pub path: String,
    /// Binding strength
    pub strength: BindingStrength,
    /// ValueSet canonical URL
    pub value_set_url: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ElementBinding {
    /// Create a new element binding
    pub fn new(
        path: impl Into<String>,
        strength: BindingStrength,
        value_set_url: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            strength,
            value_set_url: value_set_url.into(),
            description: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
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

/// Element cardinality constraint
///
/// Defines the minimum and maximum occurrences allowed for a FHIR element.
/// This structure is shared between the validator and code generator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ElementCardinality {
    /// Element path (e.g., "Patient.identifier", "Patient.name")
    pub path: String,
    /// Minimum occurrences (0 = optional, 1+ = required)
    pub min: usize,
    /// Maximum occurrences (None = unbounded (*), Some(n) = up to n)
    pub max: Option<usize>,
}

impl ElementCardinality {
    /// Create a new cardinality constraint
    pub fn new(path: impl Into<String>, min: usize, max: Option<usize>) -> Self {
        Self {
            path: path.into(),
            min,
            max,
        }
    }

    /// Check if the cardinality allows unbounded occurrences
    pub fn is_unbounded(&self) -> bool {
        self.max.is_none()
    }

    /// Check if the element is required (min > 0)
    pub fn is_required(&self) -> bool {
        self.min > 0
    }

    /// Check if the element can be an array (max > 1 or unbounded)
    pub fn is_array(&self) -> bool {
        self.max.map_or(true, |m| m > 1)
    }

    /// Format cardinality as FHIR notation (e.g., "0..1", "1..*", "0..5")
    pub fn to_fhir_notation(&self) -> String {
        match self.max {
            None => format!("{}..*", self.min),
            Some(max) => format!("{}..{}", self.min, max),
        }
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
    }

    #[test]
    fn test_element_cardinality_creation() {
        let card = ElementCardinality::new("Patient.identifier", 0, None);
        assert_eq!(card.path, "Patient.identifier");
        assert_eq!(card.min, 0);
        assert!(card.max.is_none());
        assert!(card.is_unbounded());
        assert!(!card.is_required());
        assert!(card.is_array());
    }

    #[test]
    fn test_element_cardinality_required_single() {
        let card = ElementCardinality::new("Observation.code", 1, Some(1));
        assert!(card.is_required());
        assert!(!card.is_unbounded());
        assert!(!card.is_array());
        assert_eq!(card.to_fhir_notation(), "1..1");
    }

    #[test]
    fn test_element_cardinality_optional_array() {
        let card = ElementCardinality::new("Patient.name", 0, None);
        assert!(!card.is_required());
        assert!(card.is_unbounded());
        assert!(card.is_array());
        assert_eq!(card.to_fhir_notation(), "0..*");
    }

    #[test]
    fn test_element_cardinality_bounded_array() {
        let card = ElementCardinality::new("Patient.photo", 0, Some(5));
        assert!(!card.is_required());
        assert!(!card.is_unbounded());
        assert!(card.is_array());
        assert_eq!(card.to_fhir_notation(), "0..5");
    }

    #[test]
    fn test_invariant_with_xpath_value() {
        let inv = Invariant::new("obs-1", Severity::Warning, "Code required", "code.exists()")
            .with_xpath("f:code");
        assert_eq!(inv.xpath.unwrap(), "f:code");
    }

    #[test]
    fn test_invariant_serialization() {
        let inv = Invariant::new("test-1", Severity::Error, "Test", "true");
        let json = serde_json::to_string(&inv).unwrap();
        assert!(json.contains("test-1"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_binding_strength_parsing() {
        assert_eq!(
            BindingStrength::from_fhir_str("required"),
            Some(BindingStrength::Required)
        );
        assert_eq!(
            BindingStrength::from_fhir_str("extensible"),
            Some(BindingStrength::Extensible)
        );
        assert_eq!(
            BindingStrength::from_fhir_str("preferred"),
            Some(BindingStrength::Preferred)
        );
        assert_eq!(
            BindingStrength::from_fhir_str("example"),
            Some(BindingStrength::Example)
        );
        assert_eq!(BindingStrength::from_fhir_str("invalid"), None);
    }

    #[test]
    fn test_element_binding_creation() {
        let binding = ElementBinding::new(
            "Patient.gender",
            BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/administrative-gender",
        );
        assert_eq!(binding.path, "Patient.gender");
        assert_eq!(binding.strength, BindingStrength::Required);
        assert_eq!(
            binding.value_set_url,
            "http://hl7.org/fhir/ValueSet/administrative-gender"
        );
        assert!(binding.description.is_none());
    }

    #[test]
    fn test_element_binding_with_description() {
        let binding = ElementBinding::new(
            "Patient.gender",
            BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/administrative-gender",
        )
        .with_description("The gender of the patient");
        assert!(binding.description.is_some());
        assert_eq!(binding.description.unwrap(), "The gender of the patient");
    }
}
