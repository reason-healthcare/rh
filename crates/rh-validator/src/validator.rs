//! FHIR validation implementation
//!
//! Provides structural validation via Rust's type system and serde deserialization,
//! plus FHIRPath-based invariant validation.

use crate::types::{IssueCode, Severity, ValidationIssue, ValidationResult, ValidatorError};
use crate::valuesets::ValueSetRegistry;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

/// Configuration for FHIR validator
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Maximum nesting depth for JSON documents
    pub max_depth: usize,
    /// Whether to report unknown fields as warnings
    pub warn_on_unknown_fields: bool,
    /// Whether to skip invariant validation (structural only)
    pub skip_invariants: bool,
    /// Whether to skip binding validation
    pub skip_bindings: bool,
    /// Whether to skip cardinality validation
    pub skip_cardinality: bool,
    /// Custom package directory for profiles (future use)
    pub package_dir: Option<PathBuf>,
    /// FHIR version to validate against
    pub fhir_version: String,
    /// ValueSet registry for binding validation
    pub valueset_registry: ValueSetRegistry,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            warn_on_unknown_fields: true,
            skip_invariants: false,
            skip_bindings: false,
            skip_cardinality: false,
            package_dir: None,
            fhir_version: "4.0.1".to_string(),
            valueset_registry: ValueSetRegistry::with_builtin(),
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

    /// Set whether to skip invariant validation
    pub fn with_skip_invariants(mut self, skip: bool) -> Self {
        self.skip_invariants = skip;
        self
    }

    /// Set whether to skip binding validation
    pub fn with_skip_bindings(mut self, skip: bool) -> Self {
        self.skip_bindings = skip;
        self
    }

    /// Set whether to skip cardinality validation
    pub fn with_skip_cardinality(mut self, skip: bool) -> Self {
        self.skip_cardinality = skip;
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

    /// Set ValueSet registry
    pub fn with_valueset_registry(mut self, registry: ValueSetRegistry) -> Self {
        self.valueset_registry = registry;
        self
    }
}

/// FHIR resource validator
///
/// Performs structural validation via Rust's type system and serde deserialization,
/// plus FHIRPath-based invariant validation using the embedded FHIRPath engine.
pub struct FhirValidator {
    config: ValidatorConfig,
    fhirpath_parser: FhirPathParser,
    fhirpath_evaluator: FhirPathEvaluator,
}

impl FhirValidator {
    /// Create a new FHIR validator with default configuration
    pub fn new() -> Result<Self, ValidatorError> {
        Ok(Self {
            config: ValidatorConfig::default(),
            fhirpath_parser: FhirPathParser::new(),
            fhirpath_evaluator: FhirPathEvaluator::new(),
        })
    }

    /// Create a validator with custom configuration
    pub fn with_config(config: ValidatorConfig) -> Result<Self, ValidatorError> {
        Ok(Self {
            config,
            fhirpath_parser: FhirPathParser::new(),
            fhirpath_evaluator: FhirPathEvaluator::new(),
        })
    }

    /// Create a validator with a custom package directory
    pub fn with_package_dir(package_dir: PathBuf) -> Result<Self, ValidatorError> {
        Ok(Self {
            config: ValidatorConfig::default().with_package_dir(package_dir),
            fhirpath_parser: FhirPathParser::new(),
            fhirpath_evaluator: FhirPathEvaluator::new(),
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

    /// Validate invariants for a resource that implements ValidatableResource
    ///
    /// This evaluates all FHIRPath invariants defined for the resource type.
    /// Returns validation issues for any invariants that fail.
    pub fn validate_invariants<T>(
        &self,
        resource: &T,
    ) -> Result<Vec<ValidationIssue>, ValidatorError>
    where
        T: Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        let mut issues = Vec::new();

        // Get all invariants for this resource type
        let invariants = T::invariants();

        // Serialize resource to JSON for FHIRPath evaluation
        let json_value = serde_json::to_value(resource)
            .map_err(|e| ValidatorError::Other(format!("Failed to serialize resource: {e}")))?;

        // Create evaluation context from the resource
        let context = EvaluationContext::new(json_value);

        // Evaluate each invariant
        for invariant in invariants {
            // Parse the FHIRPath expression
            let expression = match self.fhirpath_parser.parse(&invariant.expression) {
                Ok(expr) => expr,
                Err(e) => {
                    // FHIRPath parsing error
                    issues.push(ValidationIssue {
                        severity: Severity::Warning,
                        code: IssueCode::InvariantEvaluation,
                        details: format!(
                            "Failed to parse invariant {} expression: {}",
                            invariant.key, e
                        ),
                        location: Some(format!("Invariant: {}", invariant.key)),
                        expression: Some(invariant.expression.clone()),
                        invariant_key: Some(invariant.key.clone()),
                    });
                    continue;
                }
            };

            // Evaluate the expression
            match self.fhirpath_evaluator.evaluate(&expression, &context) {
                Ok(result) => {
                    // Check if invariant passed (result should be true)
                    let passed = match result {
                        FhirPathValue::Boolean(b) => b,
                        FhirPathValue::Collection(coll) => {
                            // Empty collection means false, non-empty means true
                            !coll.is_empty()
                        }
                        _ => {
                            // Other values are treated as true if they exist
                            true
                        }
                    };

                    if !passed {
                        // Invariant failed
                        issues.push(ValidationIssue {
                            severity: invariant.severity,
                            code: IssueCode::Invariant,
                            details: format!(
                                "Invariant {} failed: {}",
                                invariant.key, invariant.human
                            ),
                            location: Some(format!("Invariant: {}", invariant.key)),
                            expression: Some(invariant.expression.clone()),
                            invariant_key: Some(invariant.key.clone()),
                        });
                    }
                }
                Err(e) => {
                    // FHIRPath evaluation error
                    issues.push(ValidationIssue {
                        severity: Severity::Warning,
                        code: IssueCode::InvariantEvaluation,
                        details: format!("Failed to evaluate invariant {}: {}", invariant.key, e),
                        location: Some(format!("Invariant: {}", invariant.key)),
                        expression: Some(invariant.expression.clone()),
                        invariant_key: Some(invariant.key.clone()),
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Validate required bindings for coded elements
    ///
    /// This method checks that coded values in the resource conform to their
    /// required ValueSet bindings. Only "required" strength bindings are validated.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource instance to validate
    ///
    /// # Returns
    ///
    /// A vector of validation issues for any binding violations found
    pub fn validate_bindings<T>(&self, resource: &T) -> Result<Vec<ValidationIssue>, ValidatorError>
    where
        T: Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        let mut issues = Vec::new();

        // Get all required bindings for this resource type
        let bindings = T::bindings();

        // If no bindings, return early
        if bindings.is_empty() {
            return Ok(issues);
        }

        // Serialize resource to JSON for FHIRPath navigation
        let json_value = serde_json::to_value(resource)
            .map_err(|e| ValidatorError::Other(format!("Failed to serialize resource: {e}")))?;

        // For each binding, extract the coded value and validate
        for binding in bindings {
            // Extract the element path (e.g., "Patient.gender" -> "gender")
            let element_path = binding.path.split('.').next_back().unwrap_or(&binding.path);

            // Try to get the value from the JSON
            if let Some(value) = json_value.get(element_path) {
                // Handle different coding types
                match value {
                    // Simple code value (e.g., Patient.gender = "male")
                    serde_json::Value::String(code) => {
                        if !self.validate_code(&binding.value_set_url, None, code) {
                            issues.push(ValidationIssue {
                                severity: Severity::Error,
                                code: IssueCode::CodeInvalid,
                                details: format!(
                                    "Invalid code '{}' for element '{}'. Must be from ValueSet: {}",
                                    code, binding.path, binding.value_set_url
                                ),
                                location: Some(binding.path.clone()),
                                expression: None,
                                invariant_key: None,
                            });
                        }
                    }
                    // Could also be a Coding object with system and code
                    serde_json::Value::Object(obj) => {
                        if let Some(serde_json::Value::String(code)) = obj.get("code") {
                            let system = obj.get("system").and_then(|v| v.as_str());

                            if !self.validate_code(&binding.value_set_url, system, code) {
                                issues.push(ValidationIssue {
                                    severity: Severity::Error,
                                    code: IssueCode::CodeInvalid,
                                    details: format!(
                                        "Invalid code '{}' for element '{}'. Must be from ValueSet: {}",
                                        code, binding.path, binding.value_set_url
                                    ),
                                    location: Some(binding.path.clone()),
                                    expression: None,
                                    invariant_key: None,
                                });
                            }
                        }
                    }
                    _ => {
                        // Ignore other value types (arrays, nulls, etc.)
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Helper method to validate a code against a ValueSet
    ///
    /// Returns true if the code is valid, false otherwise
    fn validate_code(&self, value_set_url: &str, system: Option<&str>, code: &str) -> bool {
        // Look up the ValueSet in the registry
        if let Some(expansion) = self.config.valueset_registry.get(value_set_url) {
            expansion.contains_code(system, code)
        } else {
            // If ValueSet not found, we can't validate - treat as valid
            // (Could also be treated as an error depending on strictness)
            true
        }
    }

    /// Validate cardinality constraints for a resource
    ///
    /// This method checks that elements in the resource conform to their
    /// cardinality constraints (min..max). It validates:
    /// - Required elements (min > 0) are present
    /// - Array elements do not exceed max cardinality (if max is not unbounded)
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource instance to validate
    ///
    /// # Returns
    ///
    /// A vector of validation issues for any cardinality violations found
    pub fn validate_cardinality<T>(
        &self,
        resource: &T,
    ) -> Result<Vec<ValidationIssue>, ValidatorError>
    where
        T: Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        let mut issues = Vec::new();

        // Get cardinalities for this resource type
        let cardinalities = T::cardinalities();

        // If no cardinalities defined, return early
        if cardinalities.is_empty() {
            return Ok(issues);
        }

        // Serialize resource to JSON for element navigation
        let json_value = serde_json::to_value(resource)
            .map_err(|e| ValidatorError::Other(format!("Failed to serialize resource: {e}")))?;

        // For each cardinality constraint, check the element
        for cardinality in cardinalities {
            // Extract the element path (e.g., "Patient.gender" -> "gender")
            // For now, we only validate top-level fields
            let parts: Vec<&str> = cardinality.path.split('.').collect();
            if parts.len() != 2 {
                continue;
            }
            let element_name = parts[1];

            // Get the value from JSON
            let value = json_value.get(element_name);

            // Check minimum cardinality (required elements)
            if cardinality.is_required() {
                match value {
                    None => {
                        issues.push(ValidationIssue {
                            severity: Severity::Error,
                            code: IssueCode::Required,
                            details: format!(
                                "Required element '{}' is missing (cardinality: {})",
                                cardinality.path,
                                cardinality.to_fhir_notation()
                            ),
                            location: Some(cardinality.path.clone()),
                            expression: None,
                            invariant_key: None,
                        });
                    }
                    Some(serde_json::Value::Null) => {
                        issues.push(ValidationIssue {
                            severity: Severity::Error,
                            code: IssueCode::Required,
                            details: format!(
                                "Required element '{}' is null (cardinality: {})",
                                cardinality.path,
                                cardinality.to_fhir_notation()
                            ),
                            location: Some(cardinality.path.clone()),
                            expression: None,
                            invariant_key: None,
                        });
                    }
                    Some(_) => {}
                }
            }

            // Check maximum cardinality for arrays
            if let Some(serde_json::Value::Array(arr)) = value {
                if let Some(max) = cardinality.max {
                    if arr.len() > max {
                        issues.push(ValidationIssue {
                            severity: Severity::Error,
                            code: IssueCode::Cardinality,
                            details: format!(
                                "Element '{}' has {} items but maximum is {} (cardinality: {})",
                                cardinality.path,
                                arr.len(),
                                max,
                                cardinality.to_fhir_notation()
                            ),
                            location: Some(cardinality.path.clone()),
                            expression: None,
                            invariant_key: None,
                        });
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Validate both structure and invariants for a resource
    ///
    /// This performs comprehensive validation including:
    /// 1. Structural validation via deserialization
    /// 2. Invariant validation via FHIRPath expressions
    /// 3. Binding validation for required ValueSet bindings
    /// 4. Cardinality validation for min/max constraints
    ///
    /// Returns a ValidationResult with all issues found.
    pub fn validate_full<T>(&self, json: &str) -> Result<ValidationResult, ValidatorError>
    where
        T: DeserializeOwned + Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        // First perform structural validation
        let mut result = self.validate_json::<T>(json)?;

        // If structural validation failed with errors and we should skip all other validation, return early
        if !result.is_valid()
            && self.config.skip_invariants
            && self.config.skip_bindings
            && self.config.skip_cardinality
        {
            return Ok(result);
        }

        // Parse the resource once for all validation types
        if let Ok(resource) = serde_json::from_str::<T>(json) {
            // Validate invariants
            if !self.config.skip_invariants {
                let invariant_issues = self.validate_invariants(&resource)?;
                result.add_issues(invariant_issues);
            }

            // Validate bindings
            if !self.config.skip_bindings {
                let binding_issues = self.validate_bindings(&resource)?;
                result.add_issues(binding_issues);
            }

            // Validate cardinality
            if !self.config.skip_cardinality {
                let cardinality_issues = self.validate_cardinality(&resource)?;
                result.add_issues(cardinality_issues);
            }
        }

        Ok(result)
    }

    /// Validate a resource instance directly without JSON round-trip
    ///
    /// This validates invariants, bindings, and cardinality on an already-instantiated resource struct.
    /// More efficient than `validate_full()` for programmatically-built resources
    /// since it avoids the serialize → deserialize → serialize round-trip.
    ///
    /// Note: This assumes the resource is structurally valid (via Rust's type system).
    /// It validates invariants and bindings. For JSON validation, use `validate_full()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rh_validator::FhirValidator;
    /// use hl7_fhir_r4_core::resources::patient::Patient;
    ///
    /// let validator = FhirValidator::new()?;
    /// let mut patient = Patient::default();
    /// patient.base.base.id = Some("example".to_string());
    ///
    /// let result = validator.validate_resource(&patient)?;
    /// assert!(result.is_valid());
    /// # Ok::<(), rh_validator::ValidatorError>(())
    /// ```
    pub fn validate_resource<T>(&self, resource: &T) -> Result<ValidationResult, ValidatorError>
    where
        T: Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        // Create a ValidationResult with the correct resource type
        let resource_type = resource.resource_type();
        let mut result = ValidationResult::new(resource_type);

        // Validate invariants unless skipped
        if !self.config.skip_invariants {
            let invariant_issues = self.validate_invariants(resource)?;
            result.add_issues(invariant_issues);
        }

        // Validate bindings unless skipped
        if !self.config.skip_bindings {
            let binding_issues = self.validate_bindings(resource)?;
            result.add_issues(binding_issues);
        }

        // Validate cardinality unless skipped
        if !self.config.skip_cardinality {
            let cardinality_issues = self.validate_cardinality(resource)?;
            result.add_issues(cardinality_issues);
        }

        Ok(result)
    }

    /// Validate a batch of FHIR resources in parallel
    ///
    /// Uses Rayon to validate multiple JSON resources concurrently.
    /// This provides significant performance improvements for large batches.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rh_validator::FhirValidator;
    ///
    /// let validator = FhirValidator::new()?;
    /// let resources = vec![
    ///     r#"{"resourceType": "Patient", "id": "1"}"#,
    ///     r#"{"resourceType": "Patient", "id": "2"}"#,
    /// ];
    ///
    /// let results = validator.validate_batch::<Patient>(&resources)?;
    /// assert_eq!(results.len(), 2);
    /// # Ok::<(), rh_validator::ValidatorError>(())
    /// ```
    pub fn validate_batch<T>(
        &self,
        resources: &[&str],
    ) -> Result<Vec<ValidationResult>, ValidatorError>
    where
        T: DeserializeOwned + Serialize + hl7_fhir_r4_core::validation::ValidatableResource,
    {
        use rayon::prelude::*;

        // Clone config for thread-safe access
        let config = self.config.clone();

        // Validate all resources in parallel
        resources
            .par_iter()
            .map(|json| {
                // Create a validator instance for this thread
                let validator = FhirValidator::with_config(config.clone())?;
                validator.validate_full::<T>(json)
            })
            .collect()
    }

    /// Validate NDJSON (newline-delimited JSON) in parallel
    ///
    /// Processes a string containing multiple JSON objects separated by newlines.
    /// Empty lines and lines starting with # (comments) are skipped.
    /// Returns results with their line numbers for error reporting.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rh_validator::FhirValidator;
    ///
    /// let validator = FhirValidator::new()?;
    /// let ndjson = r#"{"resourceType": "Patient", "id": "1"}
    /// {"resourceType": "Patient", "id": "2"}
    /// {"resourceType": "Patient", "id": "3"}"#;
    ///
    /// let results = validator.validate_ndjson::<Patient>(ndjson)?;
    /// assert_eq!(results.len(), 3);
    /// # Ok::<(), rh_validator::ValidatorError>(())
    /// ```
    pub fn validate_ndjson<T>(
        &self,
        ndjson: &str,
    ) -> Result<Vec<(usize, ValidationResult)>, ValidatorError>
    where
        T: DeserializeOwned
            + Serialize
            + hl7_fhir_r4_core::validation::ValidatableResource
            + Send
            + Sync,
    {
        use rayon::prelude::*;

        // Clone config for thread-safe access
        let config = self.config.clone();

        // Parse lines with their line numbers
        let lines: Vec<(usize, &str)> = ndjson
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    None
                } else {
                    Some((idx + 1, trimmed))
                }
            })
            .collect();

        // Validate all lines in parallel
        lines
            .par_iter()
            .map(|(line_num, json)| {
                // Create a validator instance for this thread
                let validator = FhirValidator::with_config(config.clone())?;
                let result = validator.validate_full::<T>(json)?;
                Ok((*line_num, result))
            })
            .collect()
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
    pub fn validate_resource_json(
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

    #[test]
    fn test_validate_resource_direct() {
        use hl7_fhir_r4_core::resources::patient::Patient;

        let validator = FhirValidator::new().unwrap();
        let mut patient = Patient::default();
        patient.base.base.id = Some("test-direct".to_string());

        // Should validate successfully (minimal valid patient)
        let result = validator.validate_resource(&patient).unwrap();
        // Just verify the method works and returns a result
        let _ = result.error_count();
    }

    #[test]
    fn test_validate_resource_vs_full_equivalent() {
        use hl7_fhir_r4_core::resources::patient::Patient;

        let validator = FhirValidator::new().unwrap();
        let mut patient = Patient::default();
        patient.base.base.id = Some("test-equiv".to_string());

        // Validate directly
        let direct_result = validator.validate_resource(&patient).unwrap();

        // Validate via JSON
        let json = serde_json::to_string(&patient).unwrap();
        let json_result = validator.validate_full::<Patient>(&json).unwrap();

        // Should have same validation outcome
        assert_eq!(direct_result.is_valid(), json_result.is_valid());
        assert_eq!(direct_result.error_count(), json_result.error_count());
    }

    #[test]
    fn test_validate_resource_skip_invariants() {
        use hl7_fhir_r4_core::resources::patient::Patient;

        let config = ValidatorConfig::new().with_skip_invariants(true);
        let validator = FhirValidator::with_config(config).unwrap();
        let patient = Patient::default();

        // Should skip invariants and return empty result
        let result = validator.validate_resource(&patient).unwrap();
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 0);
    }
}
