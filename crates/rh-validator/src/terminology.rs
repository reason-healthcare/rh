//! Terminology service client for FHIR validation.
//!
//! This module provides a trait-based terminology service abstraction that supports:
//! - CodeSystem/$validate-code - Validate a code exists in a CodeSystem
//! - ValueSet/$validate-code - Validate a code is in a ValueSet
//! - CodeSystem/$lookup - Look up display name and properties for a code
//!
//! The trait can be implemented by:
//! - `HttpTerminologyService` - Real HTTP client for production use
//! - `MockTerminologyService` - Mock implementation for testing
//! - `CachedTerminologyService` - Persistent disk cache wrapper for any service

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Result of a code validation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateCodeResult {
    /// True if the code is valid
    pub result: bool,
    /// Error or warning message
    pub message: Option<String>,
    /// The correct/recommended display for the code
    pub display: Option<String>,
}

/// Result of a code lookup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupResult {
    /// The display name for the code
    pub display: Option<String>,
    /// Whether this is an abstract code
    pub is_abstract: Option<bool>,
    /// Additional properties
    pub properties: HashMap<String, String>,
    /// Designations (alternative displays for different languages/uses)
    pub designations: Vec<Designation>,
}

/// A designation (alternative display) for a code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Designation {
    /// Language code (e.g., "en-US")
    pub language: Option<String>,
    /// Use context
    pub use_code: Option<String>,
    /// The display value
    pub value: String,
}

/// Error type for terminology operations
#[derive(Debug, Clone)]
pub enum TerminologyError {
    /// The code system or value set was not found
    NotFound(String),
    /// Network or connection error
    NetworkError(String),
    /// Invalid request parameters
    InvalidRequest(String),
    /// Server returned an error
    ServerError(String),
}

impl std::fmt::Display for TerminologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminologyError::NotFound(msg) => write!(f, "Not found: {msg}"),
            TerminologyError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            TerminologyError::InvalidRequest(msg) => write!(f, "Invalid request: {msg}"),
            TerminologyError::ServerError(msg) => write!(f, "Server error: {msg}"),
        }
    }
}

impl std::error::Error for TerminologyError {}

/// Trait for terminology service operations
///
/// This trait defines the FHIR terminology operations needed for validation.
/// Implementations can be HTTP clients, mock services, or local caches.
pub trait TerminologyService: Send + Sync {
    /// Validate a code against a CodeSystem
    ///
    /// Implements CodeSystem/$validate-code
    ///
    /// # Arguments
    /// * `system` - The CodeSystem URL (e.g., "http://loinc.org")
    /// * `code` - The code to validate
    /// * `display` - Optional display to validate
    fn validate_code_in_codesystem(
        &self,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError>;

    /// Validate a code against a ValueSet
    ///
    /// Implements ValueSet/$validate-code
    ///
    /// # Arguments
    /// * `valueset_url` - The ValueSet canonical URL
    /// * `system` - The code system URL
    /// * `code` - The code to validate
    /// * `display` - Optional display to validate
    fn validate_code_in_valueset(
        &self,
        valueset_url: &str,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError>;

    /// Look up a code in a CodeSystem
    ///
    /// Implements CodeSystem/$lookup
    ///
    /// # Arguments
    /// * `system` - The CodeSystem URL
    /// * `code` - The code to look up
    fn lookup_code(&self, system: &str, code: &str) -> Result<LookupResult, TerminologyError>;

    /// Check if this service supports a given code system
    fn supports_code_system(&self, system: &str) -> bool;

    /// Check if this service supports a given value set
    fn supports_value_set(&self, valueset_url: &str) -> bool;

    /// Check if a CodeSystem URL is a supplement (cannot be used directly in Coding.system)
    ///
    /// # Arguments
    /// * `system` - The CodeSystem URL to check
    ///
    /// # Returns
    /// Some(base_url) if this is a supplement (with the URL of the base CodeSystem),
    /// None if it's not a supplement or unknown
    fn is_supplement(&self, system: &str) -> Option<String>;

    /// Register a CodeSystem as a supplement
    ///
    /// # Arguments
    /// * `system` - The CodeSystem URL
    /// * `supplements` - The URL of the CodeSystem this supplements
    fn register_supplement(&mut self, system: &str, supplements: &str);
}

/// A mock terminology service for testing
///
/// This service can be configured with known codes and their displays
/// to test validation logic without a real terminology server.
#[derive(Debug, Clone, Default)]
pub struct MockTerminologyService {
    /// Known codes: system -> code -> display
    codes: HashMap<String, HashMap<String, CodeInfo>>,
    /// Known value sets: valueset_url -> set of (system, code)
    value_sets: HashMap<String, Vec<(String, String)>>,
    /// CodeSystem supplements: supplement_url -> base_codesystem_url
    supplements: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct CodeInfo {
    display: String,
    designations: Vec<Designation>,
    is_abstract: bool,
}

impl MockTerminologyService {
    /// Create a new empty mock service
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a mock service pre-populated with common FHIR codes
    pub fn with_common_codes() -> Self {
        let mut service = Self::new();
        service.add_common_codes();
        service
    }

    /// Add a code to a code system
    pub fn add_code(&mut self, system: &str, code: &str, display: &str) -> &mut Self {
        self.codes.entry(system.to_string()).or_default().insert(
            code.to_string(),
            CodeInfo {
                display: display.to_string(),
                designations: vec![],
                is_abstract: false,
            },
        );
        self
    }

    /// Add a code with designations (alternative displays)
    pub fn add_code_with_designations(
        &mut self,
        system: &str,
        code: &str,
        display: &str,
        designations: Vec<(&str, &str)>, // (language, value)
    ) -> &mut Self {
        self.codes.entry(system.to_string()).or_default().insert(
            code.to_string(),
            CodeInfo {
                display: display.to_string(),
                designations: designations
                    .into_iter()
                    .map(|(lang, val)| Designation {
                        language: Some(lang.to_string()),
                        use_code: None,
                        value: val.to_string(),
                    })
                    .collect(),
                is_abstract: false,
            },
        );
        self
    }

    /// Add a value set with its member codes
    pub fn add_value_set(&mut self, url: &str, codes: Vec<(&str, &str)>) -> &mut Self {
        self.value_sets.insert(
            url.to_string(),
            codes
                .into_iter()
                .map(|(sys, code)| (sys.to_string(), code.to_string()))
                .collect(),
        );
        self
    }

    /// Add common FHIR terminology codes for testing
    fn add_common_codes(&mut self) {
        // Administrative Gender
        self.add_code("http://hl7.org/fhir/administrative-gender", "male", "Male");
        self.add_code(
            "http://hl7.org/fhir/administrative-gender",
            "female",
            "Female",
        );
        self.add_code(
            "http://hl7.org/fhir/administrative-gender",
            "other",
            "Other",
        );
        self.add_code(
            "http://hl7.org/fhir/administrative-gender",
            "unknown",
            "Unknown",
        );

        // Observation status
        self.add_code(
            "http://hl7.org/fhir/observation-status",
            "registered",
            "Registered",
        );
        self.add_code(
            "http://hl7.org/fhir/observation-status",
            "preliminary",
            "Preliminary",
        );
        self.add_code("http://hl7.org/fhir/observation-status", "final", "Final");
        self.add_code(
            "http://hl7.org/fhir/observation-status",
            "amended",
            "Amended",
        );

        // Some LOINC codes
        self.add_code_with_designations(
            "http://loinc.org",
            "8867-4",
            "Heart rate",
            vec![("en-US", "Heart rate"), ("en-US", "Pulse")],
        );
        self.add_code_with_designations(
            "http://loinc.org",
            "8310-5",
            "Body temperature",
            vec![("en-US", "Body temperature"), ("en-US", "Temp")],
        );
        self.add_code_with_designations(
            "http://loinc.org",
            "59408-5",
            "Oxygen saturation in Arterial blood by Pulse oximetry",
            vec![
                (
                    "en-US",
                    "Oxygen saturation in Arterial blood by Pulse oximetry",
                ),
                ("en-US", "SaO2 % BldA PulseOx"),
            ],
        );
        self.add_code_with_designations(
            "http://loinc.org",
            "85354-9",
            "Blood pressure panel with all children optional",
            vec![("en-US", "Blood pressure panel with all children optional")],
        );

        // Some SNOMED CT codes
        self.add_code(
            "http://snomed.info/sct",
            "271649006",
            "Systolic blood pressure",
        );
        self.add_code(
            "http://snomed.info/sct",
            "271650006",
            "Diastolic blood pressure",
        );
        self.add_code("http://snomed.info/sct", "386661006", "Fever");

        // V3 Role codes
        self.add_code(
            "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "MTH",
            "mother",
        );
        self.add_code(
            "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "FTH",
            "father",
        );
        self.add_code(
            "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "SIS",
            "sister",
        );
        self.add_code(
            "http://terminology.hl7.org/CodeSystem/v3-RoleCode",
            "BRO",
            "brother",
        );
        // Also add with old URL for backward compat
        self.add_code("http://hl7.org/fhir/v3/RoleCode", "MTH", "mother");
        self.add_code("http://hl7.org/fhir/v3/RoleCode", "FTH", "father");

        // CVX vaccine codes
        self.add_code_with_designations(
            "http://hl7.org/fhir/sid/cvx",
            "207",
            "COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose or 50 mcg/0.25mL dose",
            vec![(
                "en-US",
                "COVID-19, mRNA, LNP-S, PF, 100 mcg/0.5mL dose or 50 mcg/0.25mL dose",
            )],
        );
        self.add_code_with_designations(
            "http://hl7.org/fhir/sid/cvx",
            "208",
            "COVID-19, mRNA, LNP-S, PF, 30 mcg/0.3 mL dose",
            vec![("en-US", "COVID-19, mRNA, LNP-S, PF, 30 mcg/0.3 mL dose")],
        );

        // UCUM units
        self.add_code(
            "http://unitsofmeasure.org",
            "mm[Hg]",
            "millimeter of mercury",
        );
        self.add_code("http://unitsofmeasure.org", "/min", "per minute");
        self.add_code("http://unitsofmeasure.org", "Cel", "degree Celsius");
        self.add_code("http://unitsofmeasure.org", "%", "percent");
        self.add_code("http://unitsofmeasure.org", "kg", "kilogram");
        self.add_code("http://unitsofmeasure.org", "cm", "centimeter");
        self.add_code("http://unitsofmeasure.org", "m", "meter");
        self.add_code("http://unitsofmeasure.org", "a", "year");
        self.add_code("http://unitsofmeasure.org", "mo", "month");
        self.add_code("http://unitsofmeasure.org", "wk", "week");
        self.add_code("http://unitsofmeasure.org", "d", "day");
        self.add_code("http://unitsofmeasure.org", "h", "hour");
        self.add_code("http://unitsofmeasure.org", "min", "minute");
        self.add_code("http://unitsofmeasure.org", "s", "second");

        // Add some common value sets
        self.add_value_set(
            "http://hl7.org/fhir/ValueSet/administrative-gender",
            vec![
                ("http://hl7.org/fhir/administrative-gender", "male"),
                ("http://hl7.org/fhir/administrative-gender", "female"),
                ("http://hl7.org/fhir/administrative-gender", "other"),
                ("http://hl7.org/fhir/administrative-gender", "unknown"),
            ],
        );

        self.add_value_set(
            "http://hl7.org/fhir/ValueSet/observation-status",
            vec![
                ("http://hl7.org/fhir/observation-status", "registered"),
                ("http://hl7.org/fhir/observation-status", "preliminary"),
                ("http://hl7.org/fhir/observation-status", "final"),
                ("http://hl7.org/fhir/observation-status", "amended"),
                ("http://hl7.org/fhir/observation-status", "corrected"),
                ("http://hl7.org/fhir/observation-status", "cancelled"),
                ("http://hl7.org/fhir/observation-status", "entered-in-error"),
                ("http://hl7.org/fhir/observation-status", "unknown"),
            ],
        );

        // Age units value set
        self.add_value_set(
            "http://hl7.org/fhir/ValueSet/age-units",
            vec![
                ("http://unitsofmeasure.org", "a"),
                ("http://unitsofmeasure.org", "mo"),
                ("http://unitsofmeasure.org", "wk"),
                ("http://unitsofmeasure.org", "d"),
                ("http://unitsofmeasure.org", "h"),
                ("http://unitsofmeasure.org", "min"),
            ],
        );
    }

    /// Check if a display matches the code's display or any designation
    fn display_matches(&self, code_info: &CodeInfo, display: &str) -> bool {
        // Case-insensitive comparison
        if code_info.display.eq_ignore_ascii_case(display) {
            return true;
        }

        // Check designations
        for designation in &code_info.designations {
            if designation.value.eq_ignore_ascii_case(display) {
                return true;
            }
        }

        false
    }
}

impl TerminologyService for MockTerminologyService {
    fn validate_code_in_codesystem(
        &self,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let codes = self.codes.get(system).ok_or_else(|| {
            TerminologyError::NotFound(format!("CodeSystem '{system}' not found"))
        })?;

        let code_info = codes.get(code).ok_or_else(|| {
            TerminologyError::NotFound(format!("Code '{code}' not found in system '{system}'"))
        })?;

        // If display is provided, validate it
        if let Some(display) = display {
            if self.display_matches(code_info, display) {
                Ok(ValidateCodeResult {
                    result: true,
                    message: None,
                    display: Some(code_info.display.clone()),
                })
            } else {
                // Collect valid displays for error message
                let mut valid_displays = vec![code_info.display.clone()];
                for d in &code_info.designations {
                    if !valid_displays.contains(&d.value) {
                        valid_displays.push(d.value.clone());
                    }
                }

                let valid_str = if valid_displays.len() == 1 {
                    format!("'{}'", valid_displays[0])
                } else {
                    let choices: Vec<String> =
                        valid_displays.iter().map(|d| format!("'{d}'")).collect();
                    format!("one of {} choices: {}", choices.len(), choices.join(" or "))
                };

                Ok(ValidateCodeResult {
                    result: false,
                    message: Some(format!(
                        "Wrong Display Name '{display}' for {system}#{code}. Valid display is {valid_str}"
                    )),
                    display: Some(code_info.display.clone()),
                })
            }
        } else {
            Ok(ValidateCodeResult {
                result: true,
                message: None,
                display: Some(code_info.display.clone()),
            })
        }
    }

    fn validate_code_in_valueset(
        &self,
        valueset_url: &str,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let members = self.value_sets.get(valueset_url).ok_or_else(|| {
            TerminologyError::NotFound(format!("ValueSet '{valueset_url}' not found"))
        })?;

        // Check if the code is in the value set
        let in_valueset = members.iter().any(|(s, c)| s == system && c == code);

        if !in_valueset {
            return Ok(ValidateCodeResult {
                result: false,
                message: Some(format!(
                    "The code provided ({system}#{code}) was not found in the value set '{valueset_url}'"
                )),
                display: None,
            });
        }

        // If in value set, also validate display if provided
        if let Some(display) = display {
            self.validate_code_in_codesystem(system, code, Some(display))
        } else {
            // Just check code is valid in codesystem
            match self.validate_code_in_codesystem(system, code, None) {
                Ok(result) => Ok(result),
                Err(_) => {
                    // Code is in valueset but we don't have code system info
                    Ok(ValidateCodeResult {
                        result: true,
                        message: None,
                        display: None,
                    })
                }
            }
        }
    }

    fn lookup_code(&self, system: &str, code: &str) -> Result<LookupResult, TerminologyError> {
        let codes = self.codes.get(system).ok_or_else(|| {
            TerminologyError::NotFound(format!("CodeSystem '{system}' not found"))
        })?;

        let code_info = codes.get(code).ok_or_else(|| {
            TerminologyError::NotFound(format!("Code '{code}' not found in system '{system}'"))
        })?;

        Ok(LookupResult {
            display: Some(code_info.display.clone()),
            is_abstract: Some(code_info.is_abstract),
            properties: HashMap::new(),
            designations: code_info.designations.clone(),
        })
    }

    fn supports_code_system(&self, system: &str) -> bool {
        self.codes.contains_key(system)
    }

    fn supports_value_set(&self, valueset_url: &str) -> bool {
        self.value_sets.contains_key(valueset_url)
    }

    fn is_supplement(&self, system: &str) -> Option<String> {
        self.supplements.get(system).cloned()
    }

    fn register_supplement(&mut self, system: &str, supplements: &str) {
        self.supplements
            .insert(system.to_string(), supplements.to_string());
    }
}

/// HTTP-based terminology service client
///
/// Connects to a real FHIR terminology server (e.g., tx.fhir.org)
pub struct HttpTerminologyService {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl HttpTerminologyService {
    /// Create a new HTTP terminology service client
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create a client for the official FHIR terminology server
    pub fn fhir_tx() -> Self {
        Self::new("https://tx.fhir.org/r4")
    }
}

impl TerminologyService for HttpTerminologyService {
    fn validate_code_in_codesystem(
        &self,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let mut url = format!(
            "{}/CodeSystem/$validate-code?url={}&code={}",
            self.base_url,
            urlencoding::encode(system),
            urlencoding::encode(code)
        );

        if let Some(d) = display {
            url.push_str(&format!("&display={}", urlencoding::encode(d)));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .map_err(|e| TerminologyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TerminologyError::ServerError(format!(
                "Server returned status {}",
                response.status()
            )));
        }

        let params: serde_json::Value = response
            .json()
            .map_err(|e| TerminologyError::ServerError(e.to_string()))?;

        parse_validate_code_response(&params)
    }

    fn validate_code_in_valueset(
        &self,
        valueset_url: &str,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let mut url = format!(
            "{}/ValueSet/$validate-code?url={}&system={}&code={}",
            self.base_url,
            urlencoding::encode(valueset_url),
            urlencoding::encode(system),
            urlencoding::encode(code)
        );

        if let Some(d) = display {
            url.push_str(&format!("&display={}", urlencoding::encode(d)));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .map_err(|e| TerminologyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TerminologyError::ServerError(format!(
                "Server returned status {}",
                response.status()
            )));
        }

        let params: serde_json::Value = response
            .json()
            .map_err(|e| TerminologyError::ServerError(e.to_string()))?;

        parse_validate_code_response(&params)
    }

    fn lookup_code(&self, system: &str, code: &str) -> Result<LookupResult, TerminologyError> {
        let url = format!(
            "{}/CodeSystem/$lookup?system={}&code={}",
            self.base_url,
            urlencoding::encode(system),
            urlencoding::encode(code)
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .map_err(|e| TerminologyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TerminologyError::ServerError(format!(
                "Server returned status {}",
                response.status()
            )));
        }

        let params: serde_json::Value = response
            .json()
            .map_err(|e| TerminologyError::ServerError(e.to_string()))?;

        parse_lookup_response(&params)
    }

    fn supports_code_system(&self, _system: &str) -> bool {
        // HTTP service potentially supports all code systems
        true
    }

    fn supports_value_set(&self, _valueset_url: &str) -> bool {
        // HTTP service potentially supports all value sets
        true
    }

    fn is_supplement(&self, _system: &str) -> Option<String> {
        // HTTP service doesn't track supplements locally
        // Would need to query the server to find out
        None
    }

    fn register_supplement(&mut self, _system: &str, _supplements: &str) {
        // HTTP service doesn't store supplements locally
        // This would require server-side tracking
    }
}

/// Parse a FHIR Parameters resource from $validate-code response
fn parse_validate_code_response(
    params: &serde_json::Value,
) -> Result<ValidateCodeResult, TerminologyError> {
    let parameters = params
        .get("parameter")
        .and_then(|p| p.as_array())
        .ok_or_else(|| TerminologyError::ServerError("Invalid response format".to_string()))?;

    let mut result = false;
    let mut message = None;
    let mut display = None;

    for param in parameters {
        let name = param.get("name").and_then(|n| n.as_str());
        match name {
            Some("result") => {
                result = param
                    .get("valueBoolean")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
            }
            Some("message") => {
                message = param
                    .get("valueString")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            Some("display") => {
                display = param
                    .get("valueString")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            _ => {}
        }
    }

    Ok(ValidateCodeResult {
        result,
        message,
        display,
    })
}

/// Parse a FHIR Parameters resource from $lookup response
fn parse_lookup_response(params: &serde_json::Value) -> Result<LookupResult, TerminologyError> {
    let parameters = params
        .get("parameter")
        .and_then(|p| p.as_array())
        .ok_or_else(|| TerminologyError::ServerError("Invalid response format".to_string()))?;

    let mut display = None;
    let mut is_abstract = None;
    let mut properties = HashMap::new();
    let mut designations = Vec::new();

    for param in parameters {
        let name = param.get("name").and_then(|n| n.as_str());
        match name {
            Some("display") => {
                display = param
                    .get("valueString")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            Some("abstract") => {
                is_abstract = param.get("valueBoolean").and_then(|v| v.as_bool());
            }
            Some("property") => {
                // Parse property parts
                if let Some(parts) = param.get("part").and_then(|p| p.as_array()) {
                    let mut prop_code = None;
                    let mut prop_value = None;
                    for part in parts {
                        let part_name = part.get("name").and_then(|n| n.as_str());
                        match part_name {
                            Some("code") => {
                                prop_code = part.get("valueCode").and_then(|v| v.as_str());
                            }
                            Some("value") => {
                                prop_value = part
                                    .get("valueString")
                                    .or_else(|| part.get("valueCode"))
                                    .and_then(|v| v.as_str());
                            }
                            _ => {}
                        }
                    }
                    if let (Some(code), Some(value)) = (prop_code, prop_value) {
                        properties.insert(code.to_string(), value.to_string());
                    }
                }
            }
            Some("designation") => {
                if let Some(parts) = param.get("part").and_then(|p| p.as_array()) {
                    let mut language = None;
                    let mut use_code = None;
                    let mut value = None;
                    for part in parts {
                        let part_name = part.get("name").and_then(|n| n.as_str());
                        match part_name {
                            Some("language") => {
                                language = part
                                    .get("valueCode")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                            }
                            Some("use") => {
                                use_code = part
                                    .get("valueCoding")
                                    .and_then(|c| c.get("code"))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                            }
                            Some("value") => {
                                value = part
                                    .get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                            }
                            _ => {}
                        }
                    }
                    if let Some(v) = value {
                        designations.push(Designation {
                            language,
                            use_code,
                            value: v,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Ok(LookupResult {
        display,
        is_abstract,
        properties,
        designations,
    })
}

/// Configuration for terminology services
#[derive(Debug, Clone, Default)]
pub struct TerminologyConfig {
    /// Primary terminology server URL (None to disable)
    pub server_url: Option<String>,
    /// Whether to use the mock service for testing
    pub use_mock: bool,
    /// Path for persistent cache storage (None for in-memory only)
    pub cache_path: Option<PathBuf>,
}

impl TerminologyConfig {
    /// Create a config that uses the mock service
    pub fn mock() -> Self {
        Self {
            server_url: None,
            use_mock: true,
            cache_path: None,
        }
    }

    /// Create a config that uses a real terminology server
    pub fn with_server(url: &str) -> Self {
        Self {
            server_url: Some(url.to_string()),
            use_mock: false,
            cache_path: None,
        }
    }

    /// Create a config that uses the official FHIR tx server
    pub fn fhir_tx() -> Self {
        Self::with_server("https://tx.fhir.org/r4")
    }

    /// Set the cache path for persistent storage
    pub fn with_cache_path(mut self, path: PathBuf) -> Self {
        self.cache_path = Some(path);
        self
    }

    /// Use the default cache path (~/.fhir/terminology-cache)
    pub fn with_default_cache(mut self) -> Self {
        if let Some(home) = dirs::home_dir() {
            self.cache_path = Some(home.join(".fhir").join("terminology-cache"));
        }
        self
    }

    /// Build the terminology service from this config
    pub fn build(&self) -> Option<Arc<dyn TerminologyService>> {
        let base_service: Arc<dyn TerminologyService> = if self.use_mock {
            Arc::new(MockTerminologyService::with_common_codes())
        } else if let Some(ref url) = self.server_url {
            Arc::new(HttpTerminologyService::new(url))
        } else {
            return None;
        };

        // Wrap with cache if path is configured
        if let Some(ref cache_path) = self.cache_path {
            Some(Arc::new(CachedTerminologyService::new(
                base_service,
                cache_path.clone(),
            )))
        } else {
            // Use in-memory cache even without disk persistence
            Some(Arc::new(CachedTerminologyService::new_memory_only(
                base_service,
            )))
        }
    }
}

/// Cache entry for terminology lookups
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry<T> {
    value: T,
    #[allow(dead_code)]
    timestamp: u64,
}

/// Persistent disk cache for terminology service results
///
/// This service wraps any `TerminologyService` and caches results to disk,
/// significantly reducing API calls to external terminology servers.
pub struct CachedTerminologyService {
    inner: Arc<dyn TerminologyService>,
    cache_path: Option<PathBuf>,
    codesystem_cache: RwLock<HashMap<String, ValidateCodeResult>>,
    valueset_cache: RwLock<HashMap<String, ValidateCodeResult>>,
    lookup_cache: RwLock<HashMap<String, LookupResult>>,
    hits: RwLock<usize>,
    misses: RwLock<usize>,
}

impl CachedTerminologyService {
    /// Create a new cached terminology service with disk persistence
    pub fn new(inner: Arc<dyn TerminologyService>, cache_path: PathBuf) -> Self {
        let service = Self {
            inner,
            cache_path: Some(cache_path.clone()),
            codesystem_cache: RwLock::new(HashMap::new()),
            valueset_cache: RwLock::new(HashMap::new()),
            lookup_cache: RwLock::new(HashMap::new()),
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        };
        service.load_from_disk();
        service
    }

    /// Create a new cached terminology service with in-memory only caching
    pub fn new_memory_only(inner: Arc<dyn TerminologyService>) -> Self {
        Self {
            inner,
            cache_path: None,
            codesystem_cache: RwLock::new(HashMap::new()),
            valueset_cache: RwLock::new(HashMap::new()),
            lookup_cache: RwLock::new(HashMap::new()),
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        }
    }

    /// Get cache metrics (hits, misses, hit_rate)
    pub fn cache_metrics(&self) -> (usize, usize, f64) {
        let hits = *self.hits.read().unwrap();
        let misses = *self.misses.read().unwrap();
        let total = hits + misses;
        let rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        (hits, misses, rate)
    }

    fn cache_key_codesystem(system: &str, code: &str, display: Option<&str>) -> String {
        format!("cs:{}#{}:{}", system, code, display.unwrap_or(""))
    }

    fn cache_key_valueset(
        valueset_url: &str,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> String {
        format!(
            "vs:{}|{}#{}:{}",
            valueset_url,
            system,
            code,
            display.unwrap_or("")
        )
    }

    fn cache_key_lookup(system: &str, code: &str) -> String {
        format!("lookup:{system}#{code}")
    }

    fn load_from_disk(&self) {
        let Some(ref cache_path) = self.cache_path else {
            return;
        };

        // Load codesystem cache
        let cs_path = cache_path.join("codesystem_cache.json");
        if let Ok(data) = fs::read_to_string(&cs_path) {
            if let Ok(cache) = serde_json::from_str::<HashMap<String, ValidateCodeResult>>(&data) {
                *self.codesystem_cache.write().unwrap() = cache;
            }
        }

        // Load valueset cache
        let vs_path = cache_path.join("valueset_cache.json");
        if let Ok(data) = fs::read_to_string(&vs_path) {
            if let Ok(cache) = serde_json::from_str::<HashMap<String, ValidateCodeResult>>(&data) {
                *self.valueset_cache.write().unwrap() = cache;
            }
        }

        // Load lookup cache
        let lookup_path = cache_path.join("lookup_cache.json");
        if let Ok(data) = fs::read_to_string(&lookup_path) {
            if let Ok(cache) = serde_json::from_str::<HashMap<String, LookupResult>>(&data) {
                *self.lookup_cache.write().unwrap() = cache;
            }
        }
    }

    fn save_to_disk(&self) {
        let Some(ref cache_path) = self.cache_path else {
            return;
        };

        // Ensure directory exists
        if let Err(e) = fs::create_dir_all(cache_path) {
            eprintln!("Failed to create cache directory: {e}");
            return;
        }

        // Save codesystem cache
        let cs_path = cache_path.join("codesystem_cache.json");
        if let Ok(data) = serde_json::to_string(&*self.codesystem_cache.read().unwrap()) {
            let _ = fs::write(&cs_path, data);
        }

        // Save valueset cache
        let vs_path = cache_path.join("valueset_cache.json");
        if let Ok(data) = serde_json::to_string(&*self.valueset_cache.read().unwrap()) {
            let _ = fs::write(&vs_path, data);
        }

        // Save lookup cache
        let lookup_path = cache_path.join("lookup_cache.json");
        if let Ok(data) = serde_json::to_string(&*self.lookup_cache.read().unwrap()) {
            let _ = fs::write(&lookup_path, data);
        }
    }
}

impl Drop for CachedTerminologyService {
    fn drop(&mut self) {
        self.save_to_disk();
    }
}

impl TerminologyService for CachedTerminologyService {
    fn validate_code_in_codesystem(
        &self,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let key = Self::cache_key_codesystem(system, code, display);

        // Check cache
        if let Some(result) = self.codesystem_cache.read().unwrap().get(&key) {
            *self.hits.write().unwrap() += 1;
            return Ok(result.clone());
        }

        // Cache miss - call underlying service
        *self.misses.write().unwrap() += 1;
        let result = self
            .inner
            .validate_code_in_codesystem(system, code, display)?;

        // Store in cache
        self.codesystem_cache
            .write()
            .unwrap()
            .insert(key, result.clone());

        Ok(result)
    }

    fn validate_code_in_valueset(
        &self,
        valueset_url: &str,
        system: &str,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidateCodeResult, TerminologyError> {
        let key = Self::cache_key_valueset(valueset_url, system, code, display);

        // Check cache
        if let Some(result) = self.valueset_cache.read().unwrap().get(&key) {
            *self.hits.write().unwrap() += 1;
            return Ok(result.clone());
        }

        // Cache miss - call underlying service
        *self.misses.write().unwrap() += 1;
        let result = self
            .inner
            .validate_code_in_valueset(valueset_url, system, code, display)?;

        // Store in cache
        self.valueset_cache
            .write()
            .unwrap()
            .insert(key, result.clone());

        Ok(result)
    }

    fn lookup_code(&self, system: &str, code: &str) -> Result<LookupResult, TerminologyError> {
        let key = Self::cache_key_lookup(system, code);

        // Check cache
        if let Some(result) = self.lookup_cache.read().unwrap().get(&key) {
            *self.hits.write().unwrap() += 1;
            return Ok(result.clone());
        }

        // Cache miss - call underlying service
        *self.misses.write().unwrap() += 1;
        let result = self.inner.lookup_code(system, code)?;

        // Store in cache
        self.lookup_cache
            .write()
            .unwrap()
            .insert(key, result.clone());

        Ok(result)
    }

    fn supports_code_system(&self, system: &str) -> bool {
        self.inner.supports_code_system(system)
    }

    fn supports_value_set(&self, valueset_url: &str) -> bool {
        self.inner.supports_value_set(valueset_url)
    }

    fn is_supplement(&self, system: &str) -> Option<String> {
        self.inner.is_supplement(system)
    }

    fn register_supplement(&mut self, system: &str, supplements: &str) {
        // CachedTerminologyService wraps the inner service
        // For supplements, we need to delegate but the inner service is behind Arc
        // Since supplements are typically registered during setup, we can't modify
        // the inner service. This is a limitation of the current design.
        // For now, we'll skip this - supplements should be registered on the inner
        // service before wrapping with cache.
        let _ = (system, supplements);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_validate_code_success() {
        let service = MockTerminologyService::with_common_codes();

        let result = service
            .validate_code_in_codesystem(
                "http://hl7.org/fhir/administrative-gender",
                "male",
                Some("Male"),
            )
            .unwrap();

        assert!(result.result);
        assert!(result.message.is_none());
        assert_eq!(result.display, Some("Male".to_string()));
    }

    #[test]
    fn test_mock_validate_code_wrong_display() {
        let service = MockTerminologyService::with_common_codes();

        let result = service
            .validate_code_in_codesystem(
                "http://hl7.org/fhir/administrative-gender",
                "male",
                Some("Masculine"),
            )
            .unwrap();

        assert!(!result.result);
        assert!(result.message.is_some());
        assert!(result.message.unwrap().contains("Wrong Display Name"));
    }

    #[test]
    fn test_mock_validate_code_case_insensitive() {
        let service = MockTerminologyService::with_common_codes();

        let result = service
            .validate_code_in_codesystem(
                "http://hl7.org/fhir/administrative-gender",
                "male",
                Some("MALE"),
            )
            .unwrap();

        assert!(result.result);
    }

    #[test]
    fn test_mock_validate_designation() {
        let service = MockTerminologyService::with_common_codes();

        // LOINC has alternate displays
        let result = service
            .validate_code_in_codesystem("http://loinc.org", "8867-4", Some("Pulse"))
            .unwrap();

        assert!(result.result);
    }

    #[test]
    fn test_mock_valueset_validation() {
        let service = MockTerminologyService::with_common_codes();

        // Valid code in value set
        let result = service
            .validate_code_in_valueset(
                "http://hl7.org/fhir/ValueSet/administrative-gender",
                "http://hl7.org/fhir/administrative-gender",
                "male",
                None,
            )
            .unwrap();

        assert!(result.result);

        // Code not in age-units value set
        let result = service
            .validate_code_in_valueset(
                "http://hl7.org/fhir/ValueSet/age-units",
                "http://unitsofmeasure.org",
                "m", // meters not in age units
                None,
            )
            .unwrap();

        assert!(!result.result);
    }

    #[test]
    fn test_mock_lookup() {
        let service = MockTerminologyService::with_common_codes();

        let result = service.lookup_code("http://loinc.org", "8867-4").unwrap();

        assert_eq!(result.display, Some("Heart rate".to_string()));
        assert!(!result.designations.is_empty());
    }

    #[test]
    fn test_cvx_vaccine_display() {
        let service = MockTerminologyService::with_common_codes();

        // The wrong display that appears in test cases
        let result = service
            .validate_code_in_codesystem(
                "http://hl7.org/fhir/sid/cvx",
                "207",
                Some("SARS-COV-2 (COVID-19) vaccine, mRNA, spike protein, LNP, preservative free, 100 mcg/0.5mL dose"),
            )
            .unwrap();

        assert!(!result.result);
        assert!(result.message.unwrap().contains("Wrong Display Name"));
    }

    #[test]
    fn test_role_code_display() {
        let service = MockTerminologyService::with_common_codes();

        // Correct display
        let result = service
            .validate_code_in_codesystem("http://hl7.org/fhir/v3/RoleCode", "MTH", Some("mother"))
            .unwrap();
        assert!(result.result);

        // Wrong display (from test case bad-display)
        let result = service
            .validate_code_in_codesystem("http://hl7.org/fhir/v3/RoleCode", "MTH", Some("Cother"))
            .unwrap();
        assert!(!result.result);
        assert!(result.message.unwrap().contains("Wrong Display Name"));
    }
}
