//! ValueSet expansion handling for validation
//!
//! This module provides functionality for loading and querying FHIR ValueSet expansions
//! to validate coded elements against required bindings.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// A loaded ValueSet expansion
#[derive(Debug, Clone)]
pub struct ValueSetExpansion {
    /// Canonical URL of the ValueSet
    pub url: String,
    /// Valid codes in this ValueSet (system|code format for uniqueness)
    pub codes: HashSet<String>,
    /// Code details for better error messages (system|code -> display)
    pub code_details: HashMap<String, CodeInfo>,
}

/// Information about a code in a ValueSet
#[derive(Debug, Clone)]
pub struct CodeInfo {
    /// Code system URL
    pub system: String,
    /// Code value
    pub code: String,
    /// Display text (if available)
    pub display: Option<String>,
}

impl ValueSetExpansion {
    /// Create a new empty ValueSet expansion
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            codes: HashSet::new(),
            code_details: HashMap::new(),
        }
    }

    /// Add a code to the expansion
    pub fn add_code(
        &mut self,
        system: impl Into<String>,
        code: impl Into<String>,
        display: Option<String>,
    ) {
        let system = system.into();
        let code = code.into();
        let key = format!("{system}|{code}");

        self.codes.insert(key.clone());
        self.code_details.insert(
            key,
            CodeInfo {
                system,
                code,
                display,
            },
        );
    }

    /// Check if a code exists in this expansion
    ///
    /// For codes without a system, only checks the code value.
    /// For codes with a system, checks both system and code.
    pub fn contains_code(&self, system: Option<&str>, code: &str) -> bool {
        if let Some(sys) = system {
            let key = format!("{sys}|{code}");
            self.codes.contains(&key)
        } else {
            // If no system specified, check if code exists in any system
            self.code_details.values().any(|info| info.code == code)
        }
    }

    /// Get display text for a code
    pub fn get_display(&self, system: &str, code: &str) -> Option<&str> {
        let key = format!("{system}|{code}");
        self.code_details
            .get(&key)
            .and_then(|info| info.display.as_deref())
    }

    /// Load from FHIR ValueSet JSON
    pub fn from_fhir_json(json: &str) -> Result<Self, String> {
        let value_set: FhirValueSet =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse ValueSet: {e}"))?;

        let url = value_set
            .url
            .ok_or_else(|| "ValueSet missing URL".to_string())?;

        let mut expansion = ValueSetExpansion::new(url);

        // Extract codes from expansion
        if let Some(exp) = value_set.expansion {
            if let Some(contains) = exp.contains {
                for item in contains {
                    expansion.add_code(item.system.unwrap_or_default(), item.code, item.display);
                }
            }
        }

        Ok(expansion)
    }

    /// Load from embedded expansion (hardcoded common ValueSets)
    ///
    /// This is used for ValueSets that are commonly used and don't need
    /// to be loaded from external files.
    pub fn administrative_gender() -> Self {
        let mut expansion =
            ValueSetExpansion::new("http://hl7.org/fhir/ValueSet/administrative-gender");
        let system = "http://hl7.org/fhir/administrative-gender";

        expansion.add_code(system, "male", Some("Male".to_string()));
        expansion.add_code(system, "female", Some("Female".to_string()));
        expansion.add_code(system, "other", Some("Other".to_string()));
        expansion.add_code(system, "unknown", Some("Unknown".to_string()));

        expansion
    }
}

/// FHIR ValueSet JSON structure (minimal, for parsing)
#[derive(Debug, Deserialize)]
struct FhirValueSet {
    url: Option<String>,
    expansion: Option<FhirExpansion>,
}

#[derive(Debug, Deserialize)]
struct FhirExpansion {
    contains: Option<Vec<FhirContains>>,
}

#[derive(Debug, Deserialize)]
struct FhirContains {
    system: Option<String>,
    code: String,
    display: Option<String>,
}

/// Registry for ValueSet expansions
#[derive(Debug, Clone, Default)]
pub struct ValueSetRegistry {
    /// Loaded ValueSet expansions by canonical URL
    expansions: HashMap<String, ValueSetExpansion>,
}

impl ValueSetRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new registry with common built-in ValueSets
    pub fn with_builtin() -> Self {
        let mut registry = Self::new();
        registry.register(ValueSetExpansion::administrative_gender());
        registry
    }

    /// Register a ValueSet expansion
    pub fn register(&mut self, expansion: ValueSetExpansion) {
        self.expansions.insert(expansion.url.clone(), expansion);
    }

    /// Get a ValueSet expansion by URL
    pub fn get(&self, url: &str) -> Option<&ValueSetExpansion> {
        self.expansions.get(url)
    }

    /// Load and register a ValueSet from JSON
    pub fn load_from_json(&mut self, json: &str) -> Result<(), String> {
        let expansion = ValueSetExpansion::from_fhir_json(json)?;
        self.register(expansion);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valueset_expansion_creation() {
        let mut expansion =
            ValueSetExpansion::new("http://hl7.org/fhir/ValueSet/administrative-gender");
        assert_eq!(
            expansion.url,
            "http://hl7.org/fhir/ValueSet/administrative-gender"
        );
        assert!(expansion.codes.is_empty());

        expansion.add_code(
            "http://hl7.org/fhir/admin-gender",
            "male",
            Some("Male".to_string()),
        );
        assert!(expansion.contains_code(Some("http://hl7.org/fhir/admin-gender"), "male"));
    }

    #[test]
    fn test_contains_code_with_system() {
        let mut expansion = ValueSetExpansion::new("http://test.com/vs");
        expansion.add_code("http://system1.com", "code1", None);
        expansion.add_code("http://system2.com", "code1", None);

        assert!(expansion.contains_code(Some("http://system1.com"), "code1"));
        assert!(expansion.contains_code(Some("http://system2.com"), "code1"));
        assert!(!expansion.contains_code(Some("http://system3.com"), "code1"));
    }

    #[test]
    fn test_contains_code_without_system() {
        let mut expansion = ValueSetExpansion::new("http://test.com/vs");
        expansion.add_code("http://system1.com", "code1", None);

        // Should find code regardless of system when no system specified
        assert!(expansion.contains_code(None, "code1"));
        assert!(!expansion.contains_code(None, "code2"));
    }

    #[test]
    fn test_administrative_gender_builtin() {
        let expansion = ValueSetExpansion::administrative_gender();

        assert_eq!(
            expansion.url,
            "http://hl7.org/fhir/ValueSet/administrative-gender"
        );
        assert!(expansion.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "male"));
        assert!(
            expansion.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "female")
        );
        assert!(expansion.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "other"));
        assert!(
            expansion.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "unknown")
        );
        assert!(
            !expansion.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "invalid")
        );
    }

    #[test]
    fn test_registry_with_builtin() {
        let registry = ValueSetRegistry::with_builtin();

        let vs = registry.get("http://hl7.org/fhir/ValueSet/administrative-gender");
        assert!(vs.is_some());

        let vs = vs.unwrap();
        assert!(vs.contains_code(Some("http://hl7.org/fhir/administrative-gender"), "male"));
    }

    #[test]
    fn test_registry_register() {
        let mut registry = ValueSetRegistry::new();
        assert!(registry.get("http://test.com/vs").is_none());

        let expansion = ValueSetExpansion::new("http://test.com/vs");
        registry.register(expansion);

        assert!(registry.get("http://test.com/vs").is_some());
    }

    #[test]
    fn test_from_fhir_json() {
        let json = r#"{
            "resourceType": "ValueSet",
            "url": "http://test.com/test-vs",
            "expansion": {
                "contains": [
                    {
                        "system": "http://test.com/codes",
                        "code": "code1",
                        "display": "Code 1"
                    },
                    {
                        "system": "http://test.com/codes",
                        "code": "code2",
                        "display": "Code 2"
                    }
                ]
            }
        }"#;

        let expansion = ValueSetExpansion::from_fhir_json(json).unwrap();
        assert_eq!(expansion.url, "http://test.com/test-vs");
        assert!(expansion.contains_code(Some("http://test.com/codes"), "code1"));
        assert!(expansion.contains_code(Some("http://test.com/codes"), "code2"));
        assert!(!expansion.contains_code(Some("http://test.com/codes"), "code3"));
    }
}
