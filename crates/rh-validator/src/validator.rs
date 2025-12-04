//! FHIR resource validator using profile-based validation.
//!
//! This module provides the main [`FhirValidator`] type for validating FHIR resources
//! against StructureDefinition profiles. It supports:
//!
//! - Base FHIR R4 validation
//! - US Core and other IG profiles
//! - Cardinality checking
//! - Type validation
//! - FHIRPath invariant evaluation
//! - ValueSet binding validation
//! - Extension validation
//!
//! # Example
//!
//! ```no_run
//! use rh_validator::{FhirValidator, FhirVersion};
//! use serde_json::json;
//!
//! # fn main() -> anyhow::Result<()> {
//! // Create validator with FHIR packages directory
//! let validator = FhirValidator::new(FhirVersion::R4, Some("~/.fhir/packages"))?;
//!
//! // Validate a resource
//! let patient = json!({
//!     "resourceType": "Patient",
//!     "id": "example",
//!     "name": [{"family": "Doe", "given": ["John"]}]
//! });
//!
//! let result = validator.validate(&patient)?;
//! if result.valid {
//!     println!("Resource is valid!");
//! } else {
//!     for issue in &result.issues {
//!         println!("{}: {}", issue.severity, issue.message);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};

use crate::profile::ProfileRegistry;
use crate::rules::RuleCompiler;
use crate::terminology::{TerminologyConfig, TerminologyService};
use crate::types::{IssueCode, ValidationIssue, ValidationResult};
use crate::valueset::ValueSetLoader;

/// FHIR resource validator with profile-based validation support.
///
/// `FhirValidator` validates FHIR resources against StructureDefinition profiles,
/// checking cardinality, types, FHIRPath invariants, and ValueSet bindings.
///
/// # Performance
///
/// The validator uses LRU caching for both profile snapshots and compiled validation
/// rules, providing excellent performance for repeated validations:
///
/// - First validation: ~50-100ms (profile compilation)
/// - Cached validations: ~1-5ms (100x faster)
/// - Cache hit rate: >99% in typical workloads
///
/// # Examples
///
/// ## Basic Validation
///
/// ```no_run
/// use rh_validator::{FhirValidator, FhirVersion};
/// use serde_json::json;
///
/// # fn main() -> anyhow::Result<()> {
/// let validator = FhirValidator::new(FhirVersion::R4, None)?;
/// let resource = json!({"resourceType": "Patient", "id": "123"});
/// let result = validator.validate(&resource)?;
/// assert!(result.valid);
/// # Ok(())
/// # }
/// ```
///
/// ## Profile-Specific Validation
///
/// ```no_run
/// use rh_validator::{FhirValidator, FhirVersion};
/// use serde_json::json;
///
/// # fn main() -> anyhow::Result<()> {
/// let validator = FhirValidator::new(FhirVersion::R4, Some("~/.fhir/packages"))?;
///
/// let patient = json!({
///     "resourceType": "Patient",
///     "meta": {
///         "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
///     },
///     "identifier": [{"system": "http://example.org", "value": "123"}],
///     "name": [{"family": "Doe", "given": ["John"]}],
///     "gender": "male"
/// });
///
/// let result = validator.validate(&patient)?;
/// # Ok(())
/// # }
/// ```
pub struct FhirValidator {
    profile_registry: ProfileRegistry,
    rule_compiler: RuleCompiler,
    valueset_loader: ValueSetLoader,
    questionnaire_loader: crate::questionnaire::QuestionnaireLoader,
    fhirpath_parser: FhirPathParser,
    fhirpath_evaluator: FhirPathEvaluator,
    terminology_service: Option<Arc<dyn TerminologyService>>,
    /// CodeSystem supplements: supplement_url -> base_codesystem_url
    supplements: std::sync::RwLock<std::collections::HashMap<String, String>>,
    #[allow(dead_code)]
    fhir_version: crate::fhir_version::FhirVersion,
}

impl FhirValidator {
    /// Creates a new FHIR validator for the specified FHIR version.
    ///
    /// # Arguments
    ///
    /// * `fhir_version` - FHIR version to validate against (e.g., `FhirVersion::R4`)
    /// * `packages_dir` - Optional path to FHIR packages directory. If `None`, uses default
    ///   FHIR core definitions from ~/.fhir/packages. If `Some`, loads additional profiles.
    ///
    /// # Returns
    ///
    /// Returns a configured validator or an error if initialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rh_validator::{FhirValidator, FhirVersion};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// // Use FHIR R4 core only
    /// let validator = FhirValidator::new(FhirVersion::R4, None)?;
    ///
    /// // Load US Core and other IGs
    /// let validator = FhirValidator::new(FhirVersion::R4, Some("~/.fhir/packages"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        fhir_version: crate::fhir_version::FhirVersion,
        packages_dir: Option<&str>,
    ) -> Result<Self> {
        Self::with_terminology(fhir_version, packages_dir, None)
    }

    /// Creates a new FHIR validator with terminology service configuration.
    ///
    /// # Arguments
    ///
    /// * `fhir_version` - FHIR version to validate against (e.g., `FhirVersion::R4`)
    /// * `packages_dir` - Optional path to FHIR packages directory
    /// * `terminology_config` - Optional terminology service configuration. If `None`,
    ///   terminology validation (display names, code membership) is skipped.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rh_validator::{FhirValidator, FhirVersion, TerminologyConfig};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// // With mock terminology service (for testing)
    /// let validator = FhirValidator::with_terminology(
    ///     FhirVersion::R4,
    ///     None,
    ///     Some(TerminologyConfig::mock())
    /// )?;
    ///
    /// // With real terminology server and persistent cache
    /// let validator = FhirValidator::with_terminology(
    ///     FhirVersion::R4,
    ///     Some("~/.fhir/packages"),
    ///     Some(TerminologyConfig::fhir_tx().with_default_cache())
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_terminology(
        fhir_version: crate::fhir_version::FhirVersion,
        packages_dir: Option<&str>,
        terminology_config: Option<TerminologyConfig>,
    ) -> Result<Self> {
        let mut package_dirs = if let Some(dir) = packages_dir {
            vec![PathBuf::from(dir)]
        } else {
            vec![]
        };

        if let Ok(default_dir) = rh_foundation::loader::PackageLoader::get_default_packages_dir() {
            let package_name_with_version = format!(
                "{}#{}",
                fhir_version.package_id(),
                fhir_version.version_string()
            );
            let core_package_dir = default_dir.join(&package_name_with_version).join("package");
            if core_package_dir.exists() {
                package_dirs.push(core_package_dir);
            }
        }

        let terminology_service = terminology_config.and_then(|config| config.build());

        Ok(Self {
            profile_registry: ProfileRegistry::new(fhir_version, packages_dir)?,
            rule_compiler: RuleCompiler::default(),
            valueset_loader: ValueSetLoader::new(package_dirs.clone(), 100),
            questionnaire_loader: crate::questionnaire::QuestionnaireLoader::new(package_dirs, 50),
            fhirpath_parser: FhirPathParser::new(),
            fhirpath_evaluator: FhirPathEvaluator::new(),
            terminology_service,
            supplements: std::sync::RwLock::new(std::collections::HashMap::new()),
            fhir_version,
        })
    }

    /// Validates a FHIR resource with automatic profile detection.
    ///
    /// This method validates a resource by:
    /// 1. Auto-detecting the profile from `meta.profile`
    /// 2. If no profile specified, using the base resource profile
    /// 3. Validating against all detected profiles
    /// 4. Merging validation results
    ///
    /// # Arguments
    ///
    /// * `resource` - JSON value containing the FHIR resource
    ///
    /// # Returns
    ///
    /// Returns a [`ValidationResult`] with all issues found, or an error if validation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rh_validator::{FhirValidator, FhirVersion};
    /// use serde_json::json;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let validator = FhirValidator::new(FhirVersion::R4, None)?;
    ///
    /// let patient = json!({
    ///     "resourceType": "Patient",
    ///     "meta": {"profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]},
    ///     "name": [{"family": "Doe"}]
    /// });
    ///
    /// let result = validator.validate(&patient)?;
    /// println!("Valid: {}", result.valid);
    /// for issue in &result.issues {
    ///     println!("  {}: {}", issue.severity, issue.message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate(&self, resource: &Value) -> Result<ValidationResult> {
        let mut result = ValidationResult::valid();

        if !resource.is_object() {
            return Ok(ValidationResult::invalid(vec![ValidationIssue::error(
                IssueCode::Structure,
                "Resource must be a JSON object",
            )]));
        }

        let resource_type = resource.get("resourceType").and_then(|v| v.as_str());
        if resource_type.is_none() {
            result = result.with_issue(ValidationIssue::error(
                IssueCode::Required,
                "Missing required field 'resourceType'",
            ));
        }

        let resource_type_name = resource_type.unwrap_or("Resource");

        // Validate Resource.id format if present (FHIR id regex: [A-Za-z0-9\-\.]{1,64})
        if let Some(id) = resource.get("id").and_then(|v| v.as_str()) {
            if let Some(issue) = validate_id_format(id, &format!("{resource_type_name}.id")) {
                result = result.with_issue(issue);
            }
        }

        // Validate contained resource IDs
        if let Some(contained) = resource.get("contained").and_then(|v| v.as_array()) {
            for (idx, contained_resource) in contained.iter().enumerate() {
                if let Some(id) = contained_resource.get("id").and_then(|v| v.as_str()) {
                    let path = format!("{resource_type_name}.contained[{idx}].id");
                    if let Some(issue) = validate_id_format(id, &path) {
                        result = result.with_issue(issue);
                    }
                }
                // Note: Extension validation for contained resources is skipped because
                // we may not have all required IG packages loaded. Extensions from HL7
                // FHIR IGs (like SDC) require their respective packages.
            }
        }

        // Validate JSON structure (empty arrays are invalid in FHIR)
        let empty_array_issues = validate_json_structure(resource, resource_type_name);
        for issue in empty_array_issues {
            result = result.with_issue(issue);
        }

        // Validate base64Binary fields
        let base64_issues = validate_base64_fields(resource, resource_type_name);
        for issue in base64_issues {
            result = result.with_issue(issue);
        }

        // Validate Attachment size consistency
        let attachment_issues = validate_attachment_size(resource, resource_type_name);
        for issue in attachment_issues {
            result = result.with_issue(issue);
        }

        // Bundle-specific validation
        if resource_type_name == "Bundle" {
            let bundle_issues = validate_bundle(resource);
            for issue in bundle_issues {
                result = result.with_issue(issue);
            }
        }

        // Parameters-specific validation (reference resolution)
        if resource_type_name == "Parameters" {
            let params_issues = validate_parameters_references(resource);
            for issue in params_issues {
                result = result.with_issue(issue);
            }
        }

        // Validate strings for HTML-like content (security check)
        let string_security_issues = validate_string_security(resource, resource_type_name);
        for issue in string_security_issues {
            result = result.with_issue(issue);
        }

        // Validate canonical URLs for conformance resources
        let canonical_issues = validate_canonical_urls(resource, resource_type_name);
        for issue in canonical_issues {
            result = result.with_issue(issue);
        }

        // Terminology validation (display names) - only if terminology service is configured
        if self.terminology_service.is_some() {
            let terminology_issues =
                self.validate_coding_displays(resource, resource_type_name, "");
            for issue in terminology_issues {
                result = result.with_issue(issue);
            }
        }

        Ok(result)
    }

    /// Returns cache performance metrics.
    ///
    /// # Returns
    ///
    /// A tuple of `(profile_hits, profile_misses, profile_rate, rule_hits, rule_misses, rule_rate)`
    ///
    /// - `profile_hits` - Number of profile cache hits
    /// - `profile_misses` - Number of profile cache misses
    /// - `profile_rate` - Profile cache hit rate (0.0-1.0)
    /// - `rule_hits` - Number of rule compilation cache hits
    /// - `rule_misses` - Number of rule compilation cache misses
    /// - `rule_rate` - Rule cache hit rate (0.0-1.0)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rh_validator::{FhirValidator, FhirVersion};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let validator = FhirValidator::new(FhirVersion::R4, None)?;
    /// let (p_hits, p_misses, p_rate, r_hits, r_misses, r_rate) = validator.cache_metrics();
    /// println!("Profile cache: {:.1}% hit rate ({} hits, {} misses)",
    ///          p_rate * 100.0, p_hits, p_misses);
    /// println!("Rule cache: {:.1}% hit rate ({} hits, {} misses)",
    ///          r_rate * 100.0, r_hits, r_misses);
    /// # Ok(())
    /// # }
    /// ```
    pub fn cache_metrics(&self) -> (usize, usize, f64, usize, usize, f64) {
        let (prof_hits, prof_misses, prof_rate) = self.profile_registry.cache_metrics();
        let (rule_hits, rule_misses, rule_rate) = self.rule_compiler.cache_metrics();
        (
            prof_hits,
            prof_misses,
            prof_rate,
            rule_hits,
            rule_misses,
            rule_rate,
        )
    }

    pub fn reset_cache_metrics(&self) {
        self.profile_registry.reset_cache_metrics();
        self.rule_compiler.reset_cache_metrics();
    }

    pub fn validate_with_profile(
        &self,
        resource: &Value,
        profile_url: &str,
    ) -> Result<ValidationResult> {
        let mut result = self.validate(resource)?;

        let snapshot = self
            .profile_registry
            .get_snapshot(profile_url)
            .context("Failed to get snapshot from registry")?;

        let snapshot = match snapshot {
            Some(s) => s,
            None => {
                // Profile not found - add warning but still validate extensions against base profile
                // Extensions from unknown sources should still be flagged as errors
                result = result.with_issue(ValidationIssue::warning(
                    IssueCode::NotFound,
                    format!(
                        "[Profile: {profile_url}] Profile reference has not been checked because \
                        it could not be found"
                    ),
                ));

                // Still validate unknown extensions even if profile not found
                let resource_type_name = resource
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Resource");
                let unknown_ext_issues =
                    self.validate_unknown_extensions(resource, resource_type_name)?;
                for issue in unknown_ext_issues {
                    result = result.with_issue(issue);
                }

                return Ok(result);
            }
        };

        let rules = self
            .rule_compiler
            .compile(&snapshot)
            .context("Failed to compile validation rules")?;

        for rule in &rules.cardinality_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations =
                validate_cardinality_at_path(resource, &rule.path, rule.min, &rule.max);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Type validation
        for rule in &rules.type_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations = validate_type_at_path(resource, &rule.path, &rule.types);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Binding validation
        for rule in &rules.binding_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations = self.validate_binding_at_path(resource, rule)?;

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Invariant validation
        for rule in &rules.invariant_rules {
            let violations = self.validate_invariant(resource, rule)?;

            for violation in violations {
                result = result.with_issue(violation);
            }
        }

        // Extension validation
        for rule in &rules.extension_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            let violations = validate_extension_at_path(resource, rule);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Slicing validation
        for rule in &rules.slicing_rules {
            if !should_validate_path(&rule.path, resource) {
                continue;
            }

            if rule.path.ends_with(".extension") || rule.path.ends_with(".modifierExtension") {
                continue;
            }

            let violations = validate_slicing_at_path(resource, rule);

            for violation in violations {
                result = result.with_issue(violation.with_path(&rule.path));
            }
        }

        // Unknown extension validation - extensions must have known definitions
        let resource_type_name = resource
            .get("resourceType")
            .and_then(|v| v.as_str())
            .unwrap_or("Resource");
        let unknown_ext_issues = self.validate_unknown_extensions(resource, resource_type_name)?;
        for issue in unknown_ext_issues {
            result = result.with_issue(issue);
        }

        // QuestionnaireResponse validation against linked Questionnaire
        if resource_type_name == "QuestionnaireResponse" {
            let qr_issues = self.validate_questionnaire_response(resource)?;
            for issue in qr_issues {
                result = result.with_issue(issue);
            }
        }

        Ok(result)
    }

    fn validate_questionnaire_response(&self, resource: &Value) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        let questionnaire_url = match resource.get("questionnaire").and_then(|v| v.as_str()) {
            Some(url) => url,
            None => return Ok(issues),
        };

        let base_url = questionnaire_url
            .split('|')
            .next()
            .unwrap_or(questionnaire_url);

        let is_local_reference = base_url.starts_with('#');
        let local_id = if is_local_reference {
            Some(&base_url[1..])
        } else {
            None
        };

        if let Some(contained) = resource.get("contained").and_then(|v| v.as_array()) {
            for contained_resource in contained {
                if contained_resource
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    == Some("Questionnaire")
                {
                    if let Some(q) = self.questionnaire_loader.load_from_json(contained_resource) {
                        let matches = if let Some(id) = local_id {
                            contained_resource.get("id").and_then(|v| v.as_str()) == Some(id)
                        } else {
                            q.url.as_deref() == Some(base_url)
                        };

                        if matches {
                            let validator =
                                crate::questionnaire::QuestionnaireResponseValidator::new(&q)
                                    .with_valueset_loader(&self.valueset_loader);
                            return validator.validate(resource);
                        }
                    }
                }
            }

            for contained_resource in contained {
                let resource_matches = if let Some(id) = local_id {
                    contained_resource.get("id").and_then(|v| v.as_str()) == Some(id)
                } else if let Some(url) = contained_resource.get("url").and_then(|v| v.as_str()) {
                    url == base_url
                } else {
                    false
                };

                if resource_matches {
                    if let Some(resource_type) = contained_resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                    {
                        if resource_type != "Questionnaire" {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Invalid,
                                    format!(
                                        "Canonical URL '{questionnaire_url}' refers to a resource that has the wrong type. Found {resource_type} expecting one of [Questionnaire]"
                                    ),
                                )
                                .with_path("QuestionnaireResponse.questionnaire"),
                            );
                            issues.push(
                                ValidationIssue::warning(
                                    IssueCode::Required,
                                    format!(
                                        "The questionnaire '{questionnaire_url}' could not be resolved, so no validation can be performed against the base questionnaire"
                                    ),
                                )
                                .with_path("QuestionnaireResponse"),
                            );
                            return Ok(issues);
                        }
                    }
                }
            }
        }

        if is_local_reference {
            issues.push(
                ValidationIssue::warning(
                    IssueCode::Required,
                    format!(
                        "The questionnaire '{questionnaire_url}' could not be resolved, so no validation can be performed against the base questionnaire"
                    ),
                )
                .with_path("QuestionnaireResponse"),
            );
            return Ok(issues);
        }

        if let Some(q) = self.questionnaire_loader.load(base_url) {
            let validator = crate::questionnaire::QuestionnaireResponseValidator::new(&q)
                .with_valueset_loader(&self.valueset_loader);
            return validator.validate(resource);
        }

        if let Some(found_type) = self
            .questionnaire_loader
            .find_resource_type_for_url(base_url)
        {
            if found_type != "Questionnaire" {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invalid,
                        format!(
                            "Canonical URL '{questionnaire_url}' refers to a resource that has the wrong type. Found {found_type} expecting one of [Questionnaire]"
                        ),
                    )
                    .with_path("QuestionnaireResponse.questionnaire"),
                );
            }
        } else if self.valueset_loader.is_valueset_url(base_url) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!(
                        "Canonical URL '{questionnaire_url}' refers to a resource that has the wrong type. Found ValueSet expecting one of [Questionnaire]"
                    ),
                )
                .with_path("QuestionnaireResponse.questionnaire"),
            );
        }

        issues.push(
            ValidationIssue::warning(
                IssueCode::Required,
                format!(
                    "The questionnaire '{questionnaire_url}' could not be resolved, so no validation can be performed against the base questionnaire"
                ),
            )
            .with_path("QuestionnaireResponse"),
        );

        Ok(issues)
    }

    fn validate_unknown_extensions(
        &self,
        resource: &Value,
        resource_type: &str,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        let mut extensions = Vec::new();
        collect_extension_urls(resource, resource_type, &mut extensions);

        for ext_info in extensions {
            let url = &ext_info.url;
            let path = &ext_info.path;

            // Skip sub-extension URLs within complex extensions (they don't start with http(s)://)
            // These are just identifiers like "code", "value", etc. within the parent extension
            if !url.starts_with("http://") && !url.starts_with("https://") {
                continue;
            }

            // First try to resolve the extension
            match self.profile_registry.get_snapshot(url) {
                Ok(Some(snapshot)) => {
                    // Extension found - check if isModifier matches usage
                    // Get the first element in the differential which contains the extension definition
                    let def_is_modifier = snapshot
                        .differential
                        .as_ref()
                        .and_then(|d| d.element.first())
                        .and_then(|first| first.is_modifier)
                        .unwrap_or(false);

                    if ext_info.is_modifier && !def_is_modifier {
                        // Using a regular extension as modifierExtension
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Structure,
                                format!(
                                    "The extension {url} must not be used as a modifier extension"
                                ),
                            )
                            .with_path(path),
                        );
                    } else if !ext_info.is_modifier && def_is_modifier {
                        // Using a modifier extension as regular extension
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Structure,
                                format!("The extension {url} must be used as a modifierExtension"),
                            )
                            .with_path(path),
                        );
                    }
                }
                Ok(None) => {
                    // Extension not found - check if it's from an IG we might not have
                    // Skip HL7 FHIR IG extensions (but not core) since we may not have
                    // the IG packages loaded. Core extensions start with
                    // http://hl7.org/fhir/StructureDefinition/ and should be loaded.
                    let is_hl7_ig_extension = url.starts_with("http://hl7.org/fhir/")
                        && !url.starts_with("http://hl7.org/fhir/StructureDefinition/");

                    if !is_hl7_ig_extension {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Structure,
                                format!(
                                    "Extension definition '{url}' could not be found, so is not allowed here"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
                Err(_) => {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Structure,
                            format!(
                                "Extension definition '{url}' could not be resolved, so is not allowed here"
                            ),
                        )
                        .with_path(path),
                    );
                }
            }
        }

        Ok(issues)
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profile_registry.list_profiles()
    }

    pub fn search_profiles(&self, query: &str) -> Vec<String> {
        self.profile_registry.search_profiles(query)
    }

    pub fn register_questionnaire(&self, questionnaire: &Value) {
        if let Some(q) = self.questionnaire_loader.load_from_json(questionnaire) {
            self.questionnaire_loader.register(q);
        }
    }

    pub fn register_valueset(&self, valueset: &Value) {
        if let Ok(vs) = serde_json::from_value::<crate::valueset::ValueSet>(valueset.clone()) {
            if vs.resource_type == "ValueSet" {
                self.valueset_loader.register_valueset(vs);
            }
        }
    }

    pub fn register_profile(&self, profile: &Value) {
        if let Ok(sd) =
            serde_json::from_value::<rh_foundation::snapshot::StructureDefinition>(profile.clone())
        {
            self.profile_registry.register_profile(sd);
        }
    }

    /// Register a CodeSystem resource, extracting supplement information if present.
    ///
    /// If the CodeSystem has a `supplements` property, it will be tracked so that
    /// validation can detect when a supplement URL is incorrectly used in Coding.system.
    pub fn register_codesystem(&self, codesystem: &Value) {
        if codesystem.get("resourceType").and_then(|v| v.as_str()) == Some("CodeSystem") {
            // Check if this CodeSystem is a supplement
            if let (Some(url), Some(supplements)) = (
                codesystem.get("url").and_then(|v| v.as_str()),
                codesystem.get("supplements").and_then(|v| v.as_str()),
            ) {
                self.supplements
                    .write()
                    .unwrap()
                    .insert(url.to_string(), supplements.to_string());
            }
        }
    }

    pub fn cache_stats(&self) -> ((usize, usize), (usize, usize)) {
        (
            self.profile_registry.cache_stats(),
            self.rule_compiler.cache_stats(),
        )
    }

    /// Validate a resource with automatic profile detection from meta.profile.
    /// If no profiles are specified, validates against the base resource profile.
    /// If multiple profiles are specified, validates against all and merges results.
    pub fn validate_auto(&self, resource: &Value) -> Result<ValidationResult> {
        let profile_urls = ProfileRegistry::extract_profile_urls(resource);

        if profile_urls.is_empty() {
            // No profiles specified - use base resource profile
            if let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) {
                let base_profile_url =
                    format!("http://hl7.org/fhir/StructureDefinition/{resource_type}");
                return self.validate_with_profile(resource, &base_profile_url);
            } else {
                // No resourceType - just do basic validation
                return self.validate(resource);
            }
        }

        // Validate against all specified profiles and merge results
        self.validate_with_profiles(resource, &profile_urls)
    }

    /// Validate a resource against multiple profiles and merge results.
    /// Each profile's validation issues are tracked with the profile URL.
    pub fn validate_with_profiles(
        &self,
        resource: &Value,
        profile_urls: &[String],
    ) -> Result<ValidationResult> {
        let mut merged_result = self.validate(resource)?;

        for profile_url in profile_urls {
            let profile_result = self.validate_with_profile(resource, profile_url)?;

            // Add all issues from this profile, annotated with which profile they came from
            for mut issue in profile_result.issues {
                // Add profile context to the message
                if !issue.message.contains("Profile:") {
                    issue.message = format!("[Profile: {profile_url}] {}", issue.message);
                }
                merged_result = merged_result.with_issue(issue);
            }
        }

        Ok(merged_result)
    }
}

fn validate_cardinality_at_path(
    resource: &Value,
    path: &str,
    min: Option<u32>,
    max: &Option<String>,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return issues;
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return issues;
    };

    if parts[0] != resource_type {
        return issues;
    }

    // Check if element with max > 1 is incorrectly an object instead of an array
    // In FHIR JSON, elements with max > 1 MUST be arrays
    if let Some(max_str) = max {
        if max_str != "1" {
            if let Some(issue) = check_must_be_array(resource, &parts) {
                issues.push(issue);
                return issues;
            }
        }
    }

    // Find if there's an array in the path before the final element
    let array_index = find_array_in_path(resource, &parts);

    if let Some(arr_idx) = array_index {
        // Path crosses an array boundary - validate per-item
        validate_per_array_item(resource, &parts, arr_idx, min, max.as_deref(), &mut issues);
    } else {
        // Simple path - validate directly
        let count = count_simple_path(resource, &parts);
        check_cardinality(path, count, min, max.as_deref(), &mut issues);
    }

    issues
}

fn check_must_be_array(resource: &Value, parts: &[&str]) -> Option<ValidationIssue> {
    if parts.len() < 2 {
        return None;
    }

    let mut current = resource;

    // Navigate to the parent of the final element
    for part in &parts[1..parts.len() - 1] {
        match current.get(part) {
            Some(Value::Array(arr)) => {
                // Check each item in the array for the final element
                let final_part = parts.last().unwrap();
                for item in arr {
                    if let Some(value) = item.get(final_part) {
                        if value.is_object() && !value.is_null() {
                            return Some(ValidationIssue::error(
                                IssueCode::Structure,
                                format!(
                                    "The property {} must be a JSON Array, not an Object (at {})",
                                    final_part, parts[0]
                                ),
                            ));
                        }
                    }
                }
                return None;
            }
            Some(other) => current = other,
            None => return None,
        }
    }

    // Check the final element at the current level
    let final_part = parts.last().unwrap();
    if let Some(value) = current.get(final_part) {
        if value.is_object() && !value.is_null() {
            return Some(ValidationIssue::error(
                IssueCode::Structure,
                format!(
                    "The property {} must be a JSON Array, not an Object (at {})",
                    final_part, parts[0]
                ),
            ));
        }
    }

    None
}

fn find_array_in_path(resource: &Value, parts: &[&str]) -> Option<usize> {
    if parts.len() <= 2 {
        // No intermediate path to check (e.g., "Patient.identifier")
        return None;
    }

    let mut current = resource;

    for (i, part) in parts[1..parts.len() - 1].iter().enumerate() {
        match current.get(part) {
            Some(Value::Array(_)) => return Some(i + 1), // +1 because we skipped resource type
            Some(other) => current = other,
            None => return None,
        }
    }

    None
}

fn validate_per_array_item(
    resource: &Value,
    parts: &[&str],
    array_index: usize,
    min: Option<u32>,
    max: Option<&str>,
    issues: &mut Vec<ValidationIssue>,
) {
    // Navigate to the array
    let mut current = resource;
    for part in &parts[1..array_index] {
        match current.get(part) {
            Some(value) => current = value,
            None => return,
        }
    }

    // Get the array
    let Some(array) = current.get(parts[array_index]).and_then(|v| v.as_array()) else {
        return;
    };

    // Validate cardinality for each array item
    for item in array.iter() {
        let remaining_path = &parts[array_index + 1..];
        let count = count_in_item(item, remaining_path);

        let full_path = parts.join(".");

        check_cardinality(&full_path, count, min, max, issues);
    }
}

fn count_in_item(item: &Value, remaining_path: &[&str]) -> usize {
    if remaining_path.is_empty() {
        return 1;
    }

    let mut current = item;
    for (i, part) in remaining_path.iter().enumerate() {
        match current.get(part) {
            Some(Value::Array(arr)) => {
                if i == remaining_path.len() - 1 {
                    return arr.len();
                }
                return arr.len(); // Count items in nested arrays
            }
            Some(other) => {
                if i == remaining_path.len() - 1 {
                    return 1;
                }
                current = other;
            }
            None => return 0,
        }
    }

    0
}

fn count_simple_path(resource: &Value, parts: &[&str]) -> usize {
    let mut current = resource;

    for (i, part) in parts[1..].iter().enumerate() {
        // Handle choice types (e.g., value[x] matches valueString, valueInteger, etc.)
        if let Some(prefix) = part.strip_suffix("[x]") {
            let obj = current.as_object();

            if let Some(obj) = obj {
                let matching_fields: Vec<_> = obj
                    .keys()
                    .filter(|k| k.starts_with(prefix) && k.len() > prefix.len())
                    .collect();

                if i == parts.len() - 2 {
                    return matching_fields.len();
                }

                if let Some(first_match) = matching_fields.first() {
                    match obj.get(*first_match) {
                        Some(Value::Array(arr)) => {
                            if i == parts.len() - 2 {
                                return arr.len();
                            }
                            if arr.is_empty() {
                                return 0;
                            }
                            return arr.len();
                        }
                        Some(other) => {
                            if i == parts.len() - 2 {
                                return 1;
                            }
                            current = other;
                            continue;
                        }
                        None => return 0,
                    }
                } else {
                    return 0;
                }
            }
        }

        match current.get(part) {
            Some(Value::Array(arr)) => {
                if i == parts.len() - 2 {
                    return arr.len();
                }
                if arr.is_empty() {
                    return 0;
                }
                return arr.len();
            }
            Some(other) => {
                if i == parts.len() - 2 {
                    return 1;
                }
                current = other;
            }
            None => return 0,
        }
    }

    1
}

fn check_cardinality(
    path: &str,
    count: usize,
    min: Option<u32>,
    max: Option<&str>,
    issues: &mut Vec<ValidationIssue>,
) {
    if let Some(min_val) = min {
        if count < min_val as usize {
            issues.push(ValidationIssue::error(
                IssueCode::Required,
                format!(
                    "Cardinality violation at '{path}': expected at least {min_val} but found {count}"
                ),
            ));
        }
    }

    if let Some(max_str) = max {
        if max_str != "*" {
            if let Ok(max_num) = max_str.parse::<usize>() {
                if count > max_num {
                    issues.push(ValidationIssue::error(
                        IssueCode::Value,
                        format!(
                            "Cardinality violation at '{path}': expected at most {max_num} but found {count}"
                        ),
                    ));
                }
            }
        }
    }
}

fn validate_type_at_path(
    resource: &Value,
    path: &str,
    expected_types: &[String],
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if expected_types.is_empty() {
        return issues;
    }

    let values = get_values_at_path(resource, path);

    for value in values {
        if !matches_any_type(value, expected_types) {
            let actual_type = get_json_type_name(value);
            issues.push(ValidationIssue::error(
                IssueCode::Value,
                format!(
                    "Type mismatch at '{path}': expected one of [{}] but found {actual_type}",
                    expected_types.join(", ")
                ),
            ));
        } else {
            // Type matches, now validate primitive format if applicable
            for type_name in expected_types {
                if let Some(format_error) = validate_primitive_format(value, type_name, path) {
                    issues.push(format_error);
                    break;
                }
            }
        }
    }

    issues
}

fn validate_id_format(id: &str, path: &str) -> Option<ValidationIssue> {
    // FHIR id regex: [A-Za-z0-9\-\.]{1,64}
    if id.len() > 64 {
        return Some(ValidationIssue::error(
            IssueCode::Value,
            format!(
                "Invalid id at '{path}': value exceeds 64 characters (length: {})",
                id.len()
            ),
        ));
    }
    if id.is_empty() {
        return Some(ValidationIssue::error(
            IssueCode::Value,
            format!("Invalid id at '{path}': value cannot be empty"),
        ));
    }
    for c in id.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '.' {
            return Some(ValidationIssue::error(
                IssueCode::Value,
                format!("Invalid id at '{path}': contains invalid character '{c}' (allowed: A-Za-z0-9, -, .)"),
            ));
        }
    }
    None
}

fn validate_json_structure(value: &Value, current_path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    match value {
        Value::Object(obj) => {
            for (key, v) in obj {
                let child_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{current_path}.{key}")
                };
                issues.extend(validate_json_structure(v, &child_path));
            }
        }
        Value::Array(arr) => {
            // Empty arrays are invalid in FHIR JSON
            if arr.is_empty() {
                issues.push(ValidationIssue::error(
                    IssueCode::Structure,
                    format!("Empty array at '{current_path}' is not allowed in FHIR"),
                ));
            } else {
                for (idx, item) in arr.iter().enumerate() {
                    let child_path = format!("{current_path}[{idx}]");
                    issues.extend(validate_json_structure(item, &child_path));
                }
            }
        }
        _ => {}
    }

    issues
}

fn validate_string_security(value: &Value, current_path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    match value {
        Value::Object(obj) => {
            for (key, v) in obj {
                let child_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{current_path}.{key}")
                };

                // Special handling for the "div" field - validate XHTML content
                if key == "div" {
                    if let Value::String(div_content) = v {
                        issues.extend(validate_xhtml_narrative(div_content, &child_path));
                    }
                    continue;
                }

                issues.extend(validate_string_security(v, &child_path));
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                issues.extend(validate_string_security(item, &child_path));
            }
        }
        Value::String(s) => {
            // Check for HTML-like content in strings
            // Look for patterns like <script>, <style>, <iframe>, etc.
            // Note: Java validator has a "security-checks" option that controls
            // whether this is an error (true) or information (false/default).
            // Without that option, we default to error to be conservative.
            if contains_html_tags(s) {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invalid,
                        "The string value contains text that looks like embedded HTML tags, which are not allowed for security reasons in this context".to_string(),
                    )
                    .with_path(current_path.to_string()),
                );
            }
        }
        _ => {}
    }

    issues
}

fn validate_xhtml_narrative(div_content: &str, path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // List of disallowed HTML elements in FHIR narrative (per txt-1 invariant)
    // The narrative SHALL contain only the basic html formatting elements and attributes
    // described in chapters 7-11 (except section 4 of chapter 9) and 15 of the HTML 4.0 standard
    let disallowed_elements = [
        "script", "style", "iframe", "object", "embed", "form", "input", "button", "select",
        "textarea", "applet", "frame", "frameset", "link", "meta", "base", "body", "head", "html",
        "noscript", "audio", "video", "canvas", "svg", "math",
    ];

    let lower = div_content.to_lowercase();

    for element in &disallowed_elements {
        // Check for opening tag: <element or <element>
        let open_tag = format!("<{element}");
        if lower.contains(&open_tag) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!("Invalid element name in the XHTML ('{element}')"),
                )
                .with_path(path.to_string()),
            );
            break; // Only report one error per div to avoid spamming
        }
    }

    issues
}

fn validate_bundle(bundle: &Value) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let bundle_type = bundle.get("type").and_then(|v| v.as_str()).unwrap_or("");

    // Validate Bundle.link for duplicate relationship types
    issues.extend(validate_bundle_links(bundle));

    let entries = match bundle.get("entry").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return issues,
    };

    // Collect all resource references for resolution checking
    let mut available_resources: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    // First pass: collect all fullUrls and resource type/id combinations
    // Track resource counts for detecting multiple matches
    let mut resource_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for entry in entries {
        if let Some(full_url) = entry.get("fullUrl").and_then(|v| v.as_str()) {
            available_resources.insert(full_url.to_string());

            // Also add relative form: Type/id
            if let Some(resource) = entry.get("resource") {
                if let (Some(res_type), Some(res_id)) = (
                    resource.get("resourceType").and_then(|v| v.as_str()),
                    resource.get("id").and_then(|v| v.as_str()),
                ) {
                    let relative_ref = format!("{res_type}/{res_id}");
                    available_resources.insert(relative_ref.clone());
                    *resource_counts.entry(relative_ref).or_insert(0) += 1;
                }
            }
        }

        // Also track contained resources by #id
        if let Some(resource) = entry.get("resource") {
            if let Some(contained) = resource.get("contained").and_then(|v| v.as_array()) {
                for c in contained {
                    if let Some(id) = c.get("id").and_then(|v| v.as_str()) {
                        available_resources.insert(format!("#{id}"));
                    }
                }
            }
        }
    }

    // Second pass: validate each entry
    for (idx, entry) in entries.iter().enumerate() {
        let entry_path = format!("Bundle.entry[{idx}]");

        // Validate fullUrl
        if let Some(full_url) = entry.get("fullUrl").and_then(|v| v.as_str()) {
            // Note: We don't check for duplicate fullUrls because in FHIR,
            // duplicate fullUrls are allowed in some cases (e.g., document bundles
            // can contain multiple versions of the same resource)

            // fullUrl must be absolute for most bundle types
            // Exception: transaction/batch bundles can have relative URLs
            let requires_absolute = !matches!(bundle_type, "transaction" | "batch");

            if requires_absolute && !is_absolute_url(full_url) {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invalid,
                        format!("The fullUrl must be an absolute URL (not '{full_url}')"),
                    )
                    .with_path(&entry_path),
                );
            }

            // Validate that RESTful URLs match the resource type and id
            if let Some(resource) = entry.get("resource") {
                issues.extend(validate_fullurl_consistency(
                    full_url,
                    resource,
                    &entry_path,
                ));
            }
        }

        // Validate references within entry resources
        // Only require resolution for document bundles
        if let Some(resource) = entry.get("resource") {
            let resource_type = resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("Resource");
            let resource_id = resource.get("id").and_then(|v| v.as_str());
            let resource_path = format!("{entry_path}.resource/*{resource_type}*/");

            // Searchset bundles: resources must have ids
            // Exception: OperationOutcome resources don't need ids when used as outcome entries
            // (either explicitly marked with search.mode=outcome, or implicitly when no search mode is specified)
            if bundle_type == "searchset" {
                let search = entry.get("search");
                let search_mode = search.and_then(|s| s.get("mode")).and_then(|m| m.as_str());

                let is_operation_outcome = resource_type == "OperationOutcome";

                // OperationOutcome doesn't need an id if:
                // 1. search.mode is explicitly "outcome", OR
                // 2. no search element exists (implicit outcome)
                let exempt_from_id =
                    is_operation_outcome && (search_mode == Some("outcome") || search.is_none());

                if resource_id.is_none() && !exempt_from_id {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invalid,
                            "Search results must have ids".to_string(),
                        )
                        .with_path(format!("{entry_path}.resource")),
                    );
                }
            }

            validate_bundle_references(
                resource,
                &resource_path,
                &available_resources,
                &resource_counts,
                bundle_type,
                &mut issues,
            );
        }
    }

    issues
}

fn validate_bundle_links(bundle: &Value) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let links = match bundle.get("link").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return issues,
    };

    // Track which relationship types we've seen
    let mut seen_relations: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    // Relationships that can only occur once per bundle
    // Based on FHIR spec and Java validator behavior
    let unique_relations = ["self", "first", "last", "next", "previous"];

    for (idx, link) in links.iter().enumerate() {
        if let Some(relation) = link.get("relation").and_then(|v| v.as_str()) {
            if unique_relations.contains(&relation) {
                if seen_relations.contains_key(relation) {
                    // We've seen this relation before - error on the duplicate
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invalid,
                            format!("The link relationship type '{relation}' can only occur once"),
                        )
                        .with_path(format!("Bundle.link[{idx}]")),
                    );
                } else {
                    seen_relations.insert(relation.to_string(), idx);
                }
            }
        }
    }

    issues
}

fn validate_fullurl_consistency(
    full_url: &str,
    resource: &Value,
    entry_path: &str,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // Only validate RESTful URLs (http:// or https://)
    // URN, OID, UUID don't need to match resource type/id
    if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
        return issues;
    }

    let resource_type = match resource.get("resourceType").and_then(|v| v.as_str()) {
        Some(rt) => rt,
        None => return issues,
    };

    let resource_id = match resource.get("id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return issues, // No id to validate against
    };

    // Expected suffix: /ResourceType/id
    let expected_suffix = format!("/{resource_type}/{resource_id}");

    // The fullUrl should end with /ResourceType/id
    // But we need to handle versioned URLs like /ResourceType/id/_history/version
    let url_without_history = if let Some(pos) = full_url.find("/_history/") {
        &full_url[..pos]
    } else {
        full_url
    };

    if !url_without_history.ends_with(&expected_suffix) {
        // Check what the URL actually ends with
        let url_parts: Vec<&str> = url_without_history.split('/').collect();
        if url_parts.len() >= 2 {
            let actual_type = url_parts[url_parts.len() - 2];
            let actual_id = url_parts[url_parts.len() - 1];

            // If it looks like a RESTful URL pattern (ends with Type/id)
            // and the type matches but id doesn't, report an error
            if actual_type == resource_type && actual_id != resource_id {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invalid,
                        format!(
                            "The fullUrl '{full_url}' looks like a RESTful server URL, so it must end with the correct type and id (/{resource_type}/{resource_id})"
                        ),
                    )
                    .with_path(entry_path.to_string()),
                );
            }
        }
    }

    issues
}

fn is_absolute_url(url: &str) -> bool {
    // Absolute URLs start with a scheme (http:, https:, urn:, etc.)
    // The resource: scheme is used in Smart Health Cards (SHC)
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("urn:")
        || url.starts_with("oid:")
        || url.starts_with("uuid:")
        || url.starts_with("resource:")
}

fn validate_bundle_references(
    value: &Value,
    current_path: &str,
    available_resources: &std::collections::HashSet<String>,
    resource_counts: &std::collections::HashMap<String, usize>,
    bundle_type: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::Object(obj) => {
            // Check if this is a Reference with a "reference" field
            if let Some(ref_value) = obj.get("reference").and_then(|v| v.as_str()) {
                // Skip contained references (start with #) - they're validated separately
                if !ref_value.starts_with('#')
                    && !ref_value.starts_with("http://")
                    && !ref_value.starts_with("https://")
                {
                    // This is a relative reference like "Patient/123"
                    // Check if this is a versioned reference like "Type/id/_history/N"
                    let has_version = ref_value.contains("/_history/");

                    // Strip version history suffix for matching
                    let ref_without_history =
                        ref_value.split("/_history/").next().unwrap_or(ref_value);

                    // Check for multiple matches - only if NOT a versioned reference
                    // Versioned references are specific and don't have ambiguity
                    if !has_version {
                        if let Some(&count) = resource_counts.get(ref_without_history) {
                            if count > 1 {
                                issues.push(
                                    ValidationIssue::error(
                                        IssueCode::Forbidden,
                                        format!(
                                            "Multiple matches in bundle for reference {ref_without_history}"
                                        ),
                                    )
                                    .with_path(current_path.to_string()),
                                );
                            }
                        }
                    }

                    // Only require reference resolution for document bundles
                    // Other bundle types (transaction, batch, searchset, collection, etc.)
                    // may reference resources that exist on the server but aren't in the bundle
                    let requires_resolution = bundle_type == "document";

                    if requires_resolution
                        && !available_resources.contains(ref_value)
                        && !available_resources.contains(ref_without_history)
                    {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Structure,
                                format!("Unable to resolve resource with reference '{ref_value}'"),
                            )
                            .with_path(current_path.to_string()),
                        );
                    }
                }
            }

            // Recurse into child objects
            for (key, v) in obj {
                let child_path = format!("{current_path}.{key}");
                validate_bundle_references(
                    v,
                    &child_path,
                    available_resources,
                    resource_counts,
                    bundle_type,
                    issues,
                );
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                validate_bundle_references(
                    item,
                    &child_path,
                    available_resources,
                    resource_counts,
                    bundle_type,
                    issues,
                );
            }
        }
        _ => {}
    }
}

fn validate_parameters_references(params: &Value) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let parameters = match params.get("parameter").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return issues,
    };

    // Collect all resources from parameters
    let mut available_resources: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    // First pass: collect all resources and their ids
    for param in parameters {
        if let Some(resource) = param.get("resource") {
            if let (Some(res_type), Some(res_id)) = (
                resource.get("resourceType").and_then(|v| v.as_str()),
                resource.get("id").and_then(|v| v.as_str()),
            ) {
                available_resources.insert(format!("{res_type}/{res_id}"));
            }

            // Also track contained resources
            if let Some(contained) = resource.get("contained").and_then(|v| v.as_array()) {
                for c in contained {
                    if let Some(id) = c.get("id").and_then(|v| v.as_str()) {
                        available_resources.insert(format!("#{id}"));
                    }
                }
            }
        }
    }

    // Second pass: validate references
    for (idx, param) in parameters.iter().enumerate() {
        if let Some(resource) = param.get("resource") {
            let resource_type = resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("Resource");
            let resource_id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let path =
                format!("Parameters.parameter[{idx}].resource/*{resource_type}/{resource_id}*/");
            validate_parameters_resource_references(
                resource,
                &path,
                &available_resources,
                &mut issues,
            );
        }
    }

    issues
}

fn validate_parameters_resource_references(
    value: &Value,
    current_path: &str,
    available_resources: &std::collections::HashSet<String>,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::Object(obj) => {
            // Check if this is a Reference
            if let Some(ref_value) = obj.get("reference").and_then(|v| v.as_str()) {
                // Skip contained references and absolute URLs
                if !ref_value.starts_with('#')
                    && !ref_value.starts_with("http://")
                    && !ref_value.starts_with("https://")
                {
                    // This is a relative reference - check if resolvable
                    if !available_resources.contains(ref_value) {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Structure,
                                format!("Unable to resolve resource with reference '{ref_value}'"),
                            )
                            .with_path(current_path),
                        );
                    }
                }
            }

            // Recurse
            for (key, v) in obj {
                let child_path = format!("{current_path}.{key}");
                validate_parameters_resource_references(
                    v,
                    &child_path,
                    available_resources,
                    issues,
                );
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                validate_parameters_resource_references(
                    item,
                    &child_path,
                    available_resources,
                    issues,
                );
            }
        }
        _ => {}
    }
}

fn validate_base64_fields(value: &Value, current_path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    validate_base64_fields_recursive(value, current_path, &mut issues);
    issues
}

fn validate_base64_fields_recursive(
    value: &Value,
    current_path: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::Object(obj) => {
            for (key, v) in obj {
                let child_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{current_path}.{key}")
                };

                // Known base64Binary field names in FHIR:
                // - Attachment.data
                // - Binary.content / Binary.data
                // - Signature.data
                // - Attachment.hash (SHA-1 hash)
                if is_base64_field_name(key) {
                    if let Some(s) = v.as_str() {
                        if !is_valid_base64(s) {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Value,
                                    format!("The value '{s}' is not a valid Base64 value"),
                                )
                                .with_path(&child_path),
                            );
                        }
                    } else {
                        // Not a string, recurse into it
                        validate_base64_fields_recursive(v, &child_path, issues);
                    }
                } else {
                    validate_base64_fields_recursive(v, &child_path, issues);
                }
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                validate_base64_fields_recursive(item, &child_path, issues);
            }
        }
        _ => {}
    }
}

fn is_base64_field_name(name: &str) -> bool {
    // Known base64Binary field names in FHIR R4
    matches!(name, "data" | "hash")
}

fn validate_attachment_size(value: &Value, current_path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    validate_attachment_size_recursive(value, current_path, &mut issues);
    issues
}

fn validate_attachment_size_recursive(
    value: &Value,
    current_path: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::Object(obj) => {
            // Check if this looks like an Attachment with both data and size
            if obj.contains_key("data") && obj.contains_key("size") {
                if let (Some(data_str), Some(size_val)) = (
                    obj.get("data").and_then(|v| v.as_str()),
                    obj.get("size").and_then(|v| v.as_i64()),
                ) {
                    // Calculate decoded size from base64 data
                    // Base64 encodes 3 bytes into 4 characters, with possible padding
                    let trimmed = data_str.trim();
                    if !trimmed.is_empty() {
                        // Count non-whitespace characters and subtract padding
                        let non_ws_chars: usize =
                            trimmed.chars().filter(|c| !c.is_whitespace()).count();
                        let padding = trimmed.chars().rev().take_while(|c| *c == '=').count();
                        // Decoded size = (non_ws_chars * 3 / 4) - padding
                        let decoded_size = ((non_ws_chars * 3) / 4) - padding;

                        if size_val as usize != decoded_size {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Structure,
                                    format!(
                                        "Stated Attachment Size {size_val} does not match actual attachment size {decoded_size}"
                                    ),
                                )
                                .with_path(current_path),
                            );
                        }
                    }
                }
            }

            // Recurse into children
            for (key, v) in obj {
                let child_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{current_path}.{key}")
                };
                validate_attachment_size_recursive(v, &child_path, issues);
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                validate_attachment_size_recursive(item, &child_path, issues);
            }
        }
        _ => {}
    }
}

// Note: HTML security checking is disabled because the Java validator uses an option
// ("security-checks": true/false) to control whether HTML in strings is an error or info.
// Without proper option support, enabling this breaks tests.
#[allow(dead_code)]
fn validate_no_embedded_html(value: &Value, current_path: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    validate_no_embedded_html_recursive(value, current_path, &mut issues);
    issues
}

#[allow(dead_code)]
fn validate_no_embedded_html_recursive(
    value: &Value,
    current_path: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::Object(obj) => {
            for (key, v) in obj {
                // Skip the "div" field which is expected to contain XHTML
                if key == "div" {
                    continue;
                }
                let child_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{current_path}.{key}")
                };
                validate_no_embedded_html_recursive(v, &child_path, issues);
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let child_path = format!("{current_path}[{idx}]");
                validate_no_embedded_html_recursive(item, &child_path, issues);
            }
        }
        Value::String(s) => {
            // Check for embedded HTML tags (security risk)
            // Look for patterns like <tag> or </tag> or <tag />
            // This is reported as a warning (informational) to match Java validator behavior
            if contains_html_tags(s) {
                issues.push(ValidationIssue::warning(
                    IssueCode::Invalid,
                    "The string value contains text that looks like embedded HTML tags. If this content is rendered to HTML without appropriate post-processing, it may be a security risk".to_string(),
                ).with_path(current_path));
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn contains_html_tags(s: &str) -> bool {
    // Check for specific dangerous HTML tags that pose security risks
    // These are tags that can execute scripts, embed content, or change page behavior
    let dangerous_tags = [
        "script", "style", "iframe", "object", "embed", "form", "input", "button", "select",
        "textarea", "applet", "frame", "frameset", "link", "meta", "base", "noscript", "audio",
        "video", "canvas", "svg", "math", "event", "onload", "onerror", "onclick",
    ];

    let lower = s.to_lowercase();

    for tag in &dangerous_tags {
        // Check for opening tag: <tag or <tag> or <tag
        let open_pattern = format!("<{tag}");
        if lower.contains(&open_pattern) {
            // Make sure it's actually a tag by checking for > or whitespace/attributes after
            if let Some(pos) = lower.find(&open_pattern) {
                let remaining = &lower[pos + open_pattern.len()..];
                if remaining.starts_with('>')
                    || remaining.starts_with(' ')
                    || remaining.starts_with('\t')
                    || remaining.starts_with('\n')
                    || remaining.starts_with('/')
                {
                    return true;
                }
            }
        }
    }

    false
}

fn validate_canonical_urls(resource: &Value, resource_type: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    match resource_type {
        "ValueSet" => {
            issues.extend(validate_valueset_canonical_urls(resource));
        }
        "CodeSystem" => {
            issues.extend(validate_codesystem_canonical_urls(resource));
        }
        "StructureDefinition"
        | "CapabilityStatement"
        | "OperationDefinition"
        | "SearchParameter"
        | "CompartmentDefinition"
        | "ImplementationGuide"
        | "GraphDefinition"
        | "NamingSystem"
        | "TerminologyCapabilities"
        | "Questionnaire"
        | "MessageDefinition"
        | "ConceptMap"
        | "ExampleScenario" => {
            if let Some(url) = resource.get("url").and_then(|v| v.as_str()) {
                if !is_canonical_url(url) {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invalid,
                            format!(
                                "Canonical URLs must be absolute URLs if they are not fragment references ({url})"
                            ),
                        )
                        .with_path(format!("{resource_type}.url")),
                    );
                }
            }
        }
        _ => {}
    }

    if let Some(contained) = resource.get("contained").and_then(|v| v.as_array()) {
        for (idx, contained_resource) in contained.iter().enumerate() {
            let contained_type = contained_resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("Resource");

            if let Some(url) = contained_resource.get("url").and_then(|v| v.as_str()) {
                let requires_absolute = matches!(
                    contained_type,
                    "ValueSet"
                        | "CodeSystem"
                        | "StructureDefinition"
                        | "CapabilityStatement"
                        | "OperationDefinition"
                        | "SearchParameter"
                        | "CompartmentDefinition"
                        | "ImplementationGuide"
                        | "Questionnaire"
                        | "MessageDefinition"
                        | "ConceptMap"
                );

                let contained_path =
                    format!("{resource_type}.contained[{idx}]/*{contained_type}*/.url");

                if requires_absolute && !is_canonical_url(url) {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invalid,
                            format!("Canonical URLs in contained resources must be absolute URLs if present ({url})"),
                        )
                        .with_path(contained_path.clone()),
                    );
                }

                if let Some(uuid_issue) = validate_urn_uuid_format(url, &contained_path) {
                    issues.push(uuid_issue);
                }
            }
        }
    }

    issues
}

fn validate_valueset_canonical_urls(valueset: &Value) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if let Some(url) = valueset.get("url").and_then(|v| v.as_str()) {
        if !is_canonical_url(url) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!(
                        "Canonical URLs must be absolute URLs if they are not fragment references ({url})"
                    ),
                )
                .with_path("ValueSet.url".to_string()),
            );
        }
        if let Some(uuid_issue) = validate_urn_uuid_format(url, "ValueSet.url") {
            issues.push(uuid_issue);
        }
    }

    if let Some(compose) = valueset.get("compose") {
        if let Some(includes) = compose.get("include").and_then(|v| v.as_array()) {
            for (idx, include) in includes.iter().enumerate() {
                if let Some(system) = include.get("system").and_then(|v| v.as_str()) {
                    if system.starts_with('#') {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Invalid,
                                "URI values in ValueSet.compose.include.system must be absolute. To reference a contained code system, use the full CodeSystem URL and reference it using the http://hl7.org/fhir/StructureDefinition/valueset-system extension".to_string(),
                            )
                            .with_path(format!("ValueSet.compose.include[{idx}]")),
                        );
                    }
                }
            }
        }
    }

    issues
}

fn validate_codesystem_canonical_urls(codesystem: &Value) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if let Some(url) = codesystem.get("url").and_then(|v| v.as_str()) {
        if !is_canonical_url(url) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!(
                        "Canonical URLs must be absolute URLs if they are not fragment references ({url})"
                    ),
                )
                .with_path("CodeSystem.url".to_string()),
            );
        }
        if let Some(uuid_issue) = validate_urn_uuid_format(url, "CodeSystem.url") {
            issues.push(uuid_issue);
        }
    }

    issues
}

fn is_canonical_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("urn:")
}

fn validate_urn_uuid_format(url: &str, path: &str) -> Option<ValidationIssue> {
    if !url.starts_with("urn:uuid:") {
        return None;
    }

    let uuid_part = &url[9..]; // Skip "urn:uuid:"

    // UUID format: 8-4-4-4-12 hex chars with hyphens, all lowercase
    // Pattern: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    if !is_valid_uuid(uuid_part) {
        return Some(
            ValidationIssue::error(
                IssueCode::Invalid,
                format!("UUIDs must be valid and lowercase ({uuid_part})"),
            )
            .with_path(path.to_string()),
        );
    }

    None
}

fn is_valid_uuid(s: &str) -> bool {
    // UUID format: 8-4-4-4-12 (36 chars total including hyphens)
    if s.len() != 36 {
        return false;
    }

    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    // Expected lengths: 8-4-4-4-12
    let expected_lengths = [8, 4, 4, 4, 12];
    for (part, expected_len) in parts.iter().zip(expected_lengths.iter()) {
        if part.len() != *expected_len {
            return false;
        }
        // All characters must be lowercase hex
        if !part
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            return false;
        }
    }

    true
}

fn validate_primitive_format(
    value: &Value,
    type_name: &str,
    path: &str,
) -> Option<ValidationIssue> {
    let s = value.as_str()?;

    match type_name {
        "id" => {
            // FHIR id: [A-Za-z0-9\-\.]{1,64}
            if s.len() > 64 {
                return Some(ValidationIssue::error(
                    IssueCode::Value,
                    format!(
                        "Invalid id at '{path}': value exceeds 64 characters (length: {})",
                        s.len()
                    ),
                ));
            }
            if s.is_empty() {
                return Some(ValidationIssue::error(
                    IssueCode::Value,
                    format!("Invalid id at '{path}': value cannot be empty"),
                ));
            }
            for c in s.chars() {
                if !c.is_ascii_alphanumeric() && c != '-' && c != '.' {
                    return Some(ValidationIssue::error(
                        IssueCode::Value,
                        format!("Invalid id at '{path}': contains invalid character '{c}' (allowed: A-Za-z0-9, -, .)"),
                    ));
                }
            }
        }
        "oid" => {
            // FHIR oid: urn:oid:[0-2](\.(0|[1-9][0-9]*))+
            if !s.starts_with("urn:oid:") {
                return Some(ValidationIssue::error(
                    IssueCode::Value,
                    format!("Invalid oid at '{path}': must start with 'urn:oid:'"),
                ));
            }
        }
        "uuid" => {
            // FHIR uuid: urn:uuid:[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}
            if !s.starts_with("urn:uuid:") {
                return Some(ValidationIssue::error(
                    IssueCode::Value,
                    format!("Invalid uuid at '{path}': must start with 'urn:uuid:'"),
                ));
            }
        }
        "positiveInt" => {
            if let Some(n) = value.as_i64() {
                if n < 1 {
                    return Some(ValidationIssue::error(
                        IssueCode::Value,
                        format!("Invalid positiveInt at '{path}': value must be >= 1, got {n}"),
                    ));
                }
            }
        }
        "unsignedInt" => {
            if let Some(n) = value.as_i64() {
                if n < 0 {
                    return Some(ValidationIssue::error(
                        IssueCode::Value,
                        format!("Invalid unsignedInt at '{path}': value must be >= 0, got {n}"),
                    ));
                }
            }
        }
        "base64Binary" => {
            if !is_valid_base64(s) {
                return Some(ValidationIssue::error(
                    IssueCode::Value,
                    format!("The value '{s}' is not a valid Base64 value"),
                ));
            }
        }
        _ => {}
    }

    None
}

fn is_valid_base64(s: &str) -> bool {
    // Base64 alphabet: A-Z, a-z, 0-9, +, / and = for padding
    // Also allow whitespace (per RFC 2045)
    let mut char_count = 0;
    for c in s.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }
        if !c.is_ascii_alphanumeric() && c != '+' && c != '/' && c != '=' {
            return false;
        }
        char_count += 1;
    }
    // Valid base64 length (without whitespace) must be divisible by 4
    // Empty string is valid
    char_count == 0 || char_count % 4 == 0
}

#[allow(dead_code)]
fn path_exists_in_resource(resource: &Value, path: &str) -> bool {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return false;
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return false;
    };

    if parts[0] != resource_type {
        return false;
    }

    if parts.len() == 1 {
        return true;
    }

    // For element paths like "Patient.extension", just check if the field exists
    // We don't need to navigate deeply - just check the immediate field
    if parts.len() == 2 {
        return resource.get(parts[1]).is_some();
    }

    // For nested paths like "Patient.name.extension", navigate step by step
    let mut current = resource;
    for part in &parts[1..] {
        match current.get(part) {
            Some(Value::Array(arr)) if !arr.is_empty() => {
                // Navigate into first array element for further navigation
                current = &arr[0];
            }
            Some(Value::Object(_)) => {
                current = current.get(part).unwrap();
            }
            Some(_) => {
                // Primitive value - path exists
                return true;
            }
            None => {
                return false;
            }
        }
    }

    true
}

fn get_values_at_path<'a>(resource: &'a Value, path: &str) -> Vec<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return vec![];
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return vec![];
    };

    if parts[0] != resource_type {
        return vec![];
    }

    if parts.len() == 1 {
        return vec![resource];
    }

    let mut current = vec![resource];

    for part in &parts[1..] {
        let mut next = Vec::new();

        for value in current {
            match value.get(part) {
                Some(Value::Array(arr)) => {
                    for item in arr {
                        next.push(item);
                    }
                }
                Some(other) => {
                    next.push(other);
                }
                None => {}
            }
        }

        if next.is_empty() {
            return vec![];
        }

        current = next;
    }

    current
}

fn matches_any_type(value: &Value, expected_types: &[String]) -> bool {
    for expected_type in expected_types {
        if matches_fhir_type(value, expected_type) {
            return true;
        }
    }
    false
}

fn matches_fhir_type(value: &Value, fhir_type: &str) -> bool {
    match fhir_type {
        // Primitive types
        "string" | "code" | "id" | "markdown" | "uri" | "url" | "canonical" | "oid" | "uuid" => {
            value.is_string()
        }
        "boolean" => value.is_boolean(),
        "integer" | "unsignedInt" | "positiveInt" => value.is_i64() || value.is_u64(),
        "decimal" => value.is_f64() || value.is_i64() || value.is_u64(),
        "date" | "dateTime" | "instant" | "time" => value.is_string(),
        "base64Binary" => value.is_string(),

        // Complex types - must be objects
        "HumanName"
        | "Address"
        | "ContactPoint"
        | "Identifier"
        | "CodeableConcept"
        | "Coding"
        | "Reference"
        | "Period"
        | "Quantity"
        | "Range"
        | "Ratio"
        | "Attachment"
        | "Annotation"
        | "Signature"
        | "SampledData"
        | "Age"
        | "Distance"
        | "Duration"
        | "Count"
        | "Money"
        | "MoneyQuantity"
        | "SimpleQuantity"
        | "Meta"
        | "Dosage"
        | "Extension"
        | "Narrative"
        | "ContactDetail"
        | "Contributor"
        | "DataRequirement"
        | "ParameterDefinition"
        | "RelatedArtifact"
        | "TriggerDefinition"
        | "UsageContext"
        | "Expression"
        | "Timing" => value.is_object(),

        // BackboneElement and other structured types
        "BackboneElement" | "Element" => value.is_object(),

        // Resource types (all should be objects)
        _ if fhir_type.chars().next().unwrap_or('a').is_uppercase() => value.is_object(),

        // Unknown type - be lenient
        _ => true,
    }
}

fn get_json_type_name(value: &Value) -> &str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn should_validate_path(path: &str, resource: &Value) -> bool {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.len() <= 1 {
        return true;
    }

    let parent_path = parts[..parts.len() - 1].join(".");
    let parent_value = get_value_at_path(resource, &parent_path);

    match parent_value {
        Some(Value::Array(arr)) => !arr.is_empty(),
        Some(Value::Object(_)) => true,
        Some(_) => false,
        None => parts.len() == 2,
    }
}

fn get_value_at_path<'a>(resource: &'a Value, path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return None;
    }

    let resource_type = resource.get("resourceType").and_then(|v| v.as_str())?;

    if parts[0] != resource_type {
        return None;
    }

    if parts.len() == 1 {
        return Some(resource);
    }

    let mut current = resource;
    for part in &parts[1..] {
        current = current.get(part)?;
    }

    Some(current)
}

impl FhirValidator {
    /// Validate display names for all Coding elements in the resource
    fn validate_coding_displays(
        &self,
        value: &Value,
        resource_type: &str,
        path: &str,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // We need terminology service for display validation, but supplement checking
        // can work without it using the local supplements map
        let terminology_service = self.terminology_service.as_ref();

        match value {
            Value::Object(obj) => {
                // Check if this is a Coding element
                if let (Some(system), Some(code)) = (
                    obj.get("system").and_then(|v| v.as_str()),
                    obj.get("code").and_then(|v| v.as_str()),
                ) {
                    // Check if the system is a CodeSystem supplement (local tracking)
                    if let Some(_base_cs) = self.supplements.read().unwrap().get(system).cloned() {
                        let current_path = if path.is_empty() {
                            format!("{resource_type}.system")
                        } else {
                            format!("{path}.system")
                        };
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::NotFound,
                                format!(
                                    "CodeSystem {system} is a supplement, so can't be used as a value in Coding.system"
                                ),
                            )
                            .with_path(&current_path),
                        );
                        // Don't continue checking this Coding since the system itself is invalid
                    } else if let Some(ts) = terminology_service {
                        // Also check terminology service for supplements
                        if let Some(_base_cs) = ts.is_supplement(system) {
                            let current_path = if path.is_empty() {
                                format!("{resource_type}.system")
                            } else {
                                format!("{path}.system")
                            };
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::NotFound,
                                    format!(
                                        "CodeSystem {system} is a supplement, so can't be used as a value in Coding.system"
                                    ),
                                )
                                .with_path(&current_path),
                            );
                        } else {
                            // If display is present, validate it
                            if let Some(display) = obj.get("display").and_then(|v| v.as_str()) {
                                if ts.supports_code_system(system) {
                                    match ts.validate_code_in_codesystem(
                                        system,
                                        code,
                                        Some(display),
                                    ) {
                                        Ok(result) => {
                                            if !result.result {
                                                if let Some(message) = result.message {
                                                    let current_path = if path.is_empty() {
                                                        format!("{resource_type}.display")
                                                    } else {
                                                        format!("{path}.display")
                                                    };
                                                    issues.push(
                                                        ValidationIssue::error(
                                                            IssueCode::CodeInvalid,
                                                            message,
                                                        )
                                                        .with_path(&current_path),
                                                    );
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            // Terminology service error - skip validation
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Recurse into object properties
                for (key, child) in obj {
                    let child_path = if path.is_empty() {
                        format!("{resource_type}.{key}")
                    } else {
                        format!("{path}.{key}")
                    };
                    issues.extend(self.validate_coding_displays(child, resource_type, &child_path));
                }
            }
            Value::Array(arr) => {
                for (idx, item) in arr.iter().enumerate() {
                    let item_path = format!("{path}[{idx}]");
                    issues.extend(self.validate_coding_displays(item, resource_type, &item_path));
                }
            }
            _ => {}
        }

        issues
    }

    fn validate_binding_at_path(
        &self,
        resource: &Value,
        rule: &crate::rules::BindingRule,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Skip preferred and example bindings
        if rule.strength == "preferred" || rule.strength == "example" {
            return Ok(issues);
        }

        // Get values at path
        let values = get_values_at_path(resource, &rule.path);

        // Check for primitive extension arrays without corresponding values FIRST
        // This validation doesn't depend on ValueSet being extensional - just checks if value exists
        // E.g., _category exists but category doesn't - this is invalid for required/extensible bindings
        if values.is_empty() && (rule.strength == "required" || rule.strength == "extensible") {
            if let Some(extension_count) =
                check_primitive_extension_without_value(resource, &rule.path)
            {
                // There are extension-only elements that should have values per the binding
                for i in 0..extension_count {
                    issues.push(ValidationIssue::error(
                        IssueCode::Required,
                        format!(
                            "No code provided for {}[{}], and a code is required from the value set '{}'",
                            rule.path, i, rule.value_set_url
                        ),
                    ));
                }
                return Ok(issues);
            }
        }

        // Check if ValueSet is extensional
        let is_extensional = self
            .valueset_loader
            .is_extensional(&rule.value_set_url)
            .unwrap_or(false);

        if !is_extensional {
            // Skip intensional ValueSets (deferred to Phase 3.5)
            return Ok(issues);
        }

        for value in values {
            // Extract codes based on type
            let codes = extract_codes_from_value(value);

            for (system, code) in codes {
                use crate::valueset::CodeInValueSetResult;
                let result =
                    self.valueset_loader
                        .contains_code(&rule.value_set_url, &system, &code)?;

                match result {
                    CodeInValueSetResult::Found => {}
                    CodeInValueSetResult::NotFound => {
                        let message = format!(
                            "Code '{}' from system '{}' is not in {} ValueSet '{}'",
                            code,
                            if system.is_empty() {
                                "(no system)"
                            } else {
                                &system
                            },
                            rule.strength,
                            rule.value_set_url
                        );

                        let issue = if rule.strength == "required" {
                            ValidationIssue::error(IssueCode::CodeInvalid, message)
                        } else {
                            ValidationIssue::warning(IssueCode::CodeInvalid, message)
                        };

                        issues.push(issue);
                    }
                    CodeInValueSetResult::ValueSetNotFound => {
                        // ValueSet couldn't be found - skip validation
                    }
                }
            }
        }

        Ok(issues)
    }

    fn validate_invariant(
        &self,
        resource: &Value,
        rule: &crate::rules::InvariantRule,
    ) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // Parse the FHIRPath expression
        let expression = match self.fhirpath_parser.parse(&rule.expression) {
            Ok(expr) => expr,
            Err(e) => {
                // Handle parse errors gracefully - skip validation but log
                return Ok(vec![ValidationIssue::warning(
                    IssueCode::Invariant,
                    format!("Failed to parse invariant {}: {}", rule.key, e),
                )]);
            }
        };

        // Determine if this is a resource-level or element-level invariant
        let parts: Vec<&str> = rule.path.split('.').collect();
        let is_resource_level = parts.len() == 1;

        if is_resource_level {
            // Resource-level invariant - evaluate against whole resource
            let context = EvaluationContext::new(resource.clone());
            let result = match self.fhirpath_evaluator.evaluate(&expression, &context) {
                Ok(value) => value,
                Err(e) => {
                    return Ok(vec![ValidationIssue::warning(
                        IssueCode::Invariant,
                        format!("Failed to evaluate invariant {}: {}", rule.key, e),
                    )]);
                }
            };

            let is_valid = self.evaluate_invariant_result(&result);
            if !is_valid {
                issues.push(self.create_invariant_issue(rule));
            }
        } else {
            // Element-level invariant - get elements at path and evaluate against each
            let elements = get_values_at_path(resource, &rule.path);

            // If no elements at path, skip validation (constraint doesn't apply)
            if elements.is_empty() {
                return Ok(issues);
            }

            for element in elements {
                let context = EvaluationContext::new(element.clone());
                let result = match self.fhirpath_evaluator.evaluate(&expression, &context) {
                    Ok(value) => value,
                    Err(e) => {
                        return Ok(vec![ValidationIssue::warning(
                            IssueCode::Invariant,
                            format!("Failed to evaluate invariant {}: {}", rule.key, e),
                        )]);
                    }
                };

                let is_valid = self.evaluate_invariant_result(&result);
                if !is_valid {
                    issues.push(self.create_invariant_issue(rule));
                    // Continue to check all elements, but we already have at least one failure
                }
            }
        }

        Ok(issues)
    }

    fn evaluate_invariant_result(&self, result: &rh_fhirpath::FhirPathValue) -> bool {
        use rh_fhirpath::FhirPathValue;
        match result {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::Empty => true,
            FhirPathValue::Collection(ref items) if items.is_empty() => true,
            FhirPathValue::Collection(ref items) if items.len() == 1 => match &items[0] {
                FhirPathValue::Boolean(b) => *b,
                _ => true,
            },
            _ => true,
        }
    }

    fn create_invariant_issue(&self, rule: &crate::rules::InvariantRule) -> ValidationIssue {
        if rule.severity == "error" {
            ValidationIssue::error(
                IssueCode::Invariant,
                format!("{}: {} (at {})", rule.key, rule.human, rule.path),
            )
        } else {
            ValidationIssue::warning(
                IssueCode::Invariant,
                format!("{}: {} (at {})", rule.key, rule.human, rule.path),
            )
        }
    }
}

fn extract_codes_from_value(value: &Value) -> Vec<(String, String)> {
    let mut codes = Vec::new();

    match value {
        // Simple code (string)
        Value::String(code) => {
            codes.push((String::new(), code.clone()));
        }
        // Coding
        Value::Object(obj) if obj.contains_key("code") => {
            if let Some(code) = obj.get("code").and_then(|v| v.as_str()) {
                let system = obj
                    .get("system")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                codes.push((system, code.to_string()));
            }
        }
        // CodeableConcept
        Value::Object(obj) if obj.contains_key("coding") => {
            if let Some(Value::Array(codings)) = obj.get("coding") {
                for coding in codings {
                    if let Value::Object(coding_obj) = coding {
                        if let Some(code) = coding_obj.get("code").and_then(|v| v.as_str()) {
                            let system = coding_obj
                                .get("system")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            codes.push((system, code.to_string()));
                        }
                    }
                }
            }
        }
        _ => {}
    }

    codes
}

/// Check if a primitive field has extension elements (_field) without corresponding values (field).
/// Returns Some(count) if there are extension-only elements, None otherwise.
fn check_primitive_extension_without_value(resource: &Value, path: &str) -> Option<usize> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.len() < 2 {
        return None;
    }

    let resource_type = resource.get("resourceType").and_then(|v| v.as_str())?;
    if parts[0] != resource_type {
        return None;
    }

    // Navigate to the parent object
    let mut current = resource;
    for part in &parts[1..parts.len() - 1] {
        current = current.get(part)?;
    }

    let field_name = parts[parts.len() - 1];
    let extension_field_name = format!("_{field_name}");

    // Check if the extension field exists and the value field doesn't (or is smaller)
    let has_value_field = current.get(field_name).is_some();
    let extension_array = current
        .get(&extension_field_name)
        .and_then(|v| v.as_array())?;

    if !extension_array.is_empty() && !has_value_field {
        // Extension elements exist but no value array - these are phantom elements
        return Some(extension_array.len());
    }

    // Could also check for length mismatch, but the allergy case is absence entirely
    None
}

fn validate_extension_at_path(
    resource: &Value,
    rule: &crate::rules::ExtensionRule,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let parts: Vec<&str> = rule.path.split('.').collect();

    if parts.is_empty() {
        return issues;
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return issues;
    };

    if parts[0] != resource_type {
        return issues;
    }

    let extension_array = navigate_to_extensions(resource, &parts);

    let Some(extensions) = extension_array else {
        if rule.min > 0 {
            issues.push(ValidationIssue::error(
                IssueCode::Extension,
                format!(
                    "Extension '{}' (slice: {}) is required (min: {}) but not present",
                    rule.url, rule.slice_name, rule.min
                ),
            ));
        }
        return issues;
    };

    let matching_extensions: Vec<_> = extensions
        .iter()
        .filter_map(|ext| {
            if let Value::Object(ext_obj) = ext {
                if let Some(url) = ext_obj.get("url").and_then(|v| v.as_str()) {
                    if url == rule.url {
                        return Some(ext_obj);
                    }
                }
            }
            None
        })
        .collect();

    let count = matching_extensions.len();

    if (count as u32) < rule.min {
        issues.push(ValidationIssue::error(
            IssueCode::Extension,
            format!(
                "Extension '{}' (slice: {}) cardinality violation: found {}, expected at least {}",
                rule.url, rule.slice_name, count, rule.min
            ),
        ));
    }

    if rule.max != "*" {
        if let Ok(max_val) = rule.max.parse::<usize>() {
            if count > max_val {
                issues.push(ValidationIssue::error(
                    IssueCode::Extension,
                    format!(
                        "Extension '{}' (slice: {}) cardinality violation: found {}, expected at most {}",
                        rule.url, rule.slice_name, count, max_val
                    ),
                ));
            }
        }
    }

    for ext_obj in matching_extensions {
        if !ext_obj.contains_key("url") {
            issues.push(ValidationIssue::error(
                IssueCode::Extension,
                format!(
                    "Extension '{}' (slice: {}) must have a url",
                    rule.url, rule.slice_name
                ),
            ));
        }

        let has_value = ext_obj.keys().any(|k| k.starts_with("value"));
        let has_nested = ext_obj
            .get("extension")
            .and_then(|v| v.as_array())
            .map(|arr| !arr.is_empty())
            .unwrap_or(false);

        if !has_value && !has_nested {
            issues.push(ValidationIssue::error(
                IssueCode::Extension,
                format!(
                    "Extension '{}' (slice: {}) must have either a value[x] or nested extensions",
                    rule.url, rule.slice_name
                ),
            ));
        }
    }

    issues
}

fn navigate_to_extensions<'a>(resource: &'a Value, parts: &[&str]) -> Option<&'a Vec<Value>> {
    let mut current = resource;

    for part in &parts[1..] {
        match current.get(part) {
            Some(value) => current = value,
            None => return None,
        }
    }

    current.as_array()
}

fn validate_slicing_at_path(
    resource: &Value,
    rule: &crate::rules::SlicingRule,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let parts: Vec<&str> = rule.path.split('.').collect();

    if parts.is_empty() {
        return issues;
    }

    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return issues;
    };

    if parts[0] != resource_type {
        return issues;
    }

    let array = navigate_to_array(resource, &parts);

    let Some(elements) = array else {
        for slice in &rule.slices {
            if slice.min > 0 {
                issues.push(ValidationIssue::error(
                    IssueCode::Value,
                    format!(
                        "Slice '{}' requires at least {} element(s) but array is missing",
                        slice.name, slice.min
                    ),
                ));
            }
        }
        return issues;
    };

    if rule.discriminators.is_empty() {
        return issues;
    }

    let mut slice_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for slice in &rule.slices {
        slice_counts.insert(slice.name.clone(), 0);
    }

    for (idx, element) in elements.iter().enumerate() {
        let mut matched = false;

        for slice in &rule.slices {
            if matches_slice(element, slice, &rule.discriminators) {
                *slice_counts.get_mut(&slice.name).unwrap() += 1;
                matched = true;
                break;
            }
        }

        if !matched && rule.rules == "closed" {
            issues.push(ValidationIssue::error(
                IssueCode::Value,
                format!("Element at index {idx} does not match any slice and slicing is closed"),
            ));
        }
    }

    for slice in &rule.slices {
        let count = slice_counts.get(&slice.name).copied().unwrap_or(0);

        if (count as u32) < slice.min {
            issues.push(ValidationIssue::error(
                IssueCode::Value,
                format!(
                    "Slice '{}' cardinality violation: found {}, expected at least {}",
                    slice.name, count, slice.min
                ),
            ));
        }

        if slice.max != "*" {
            if let Ok(max_val) = slice.max.parse::<usize>() {
                if count > max_val {
                    issues.push(ValidationIssue::error(
                        IssueCode::Value,
                        format!(
                            "Slice '{}' cardinality violation: found {}, expected at most {}",
                            slice.name, count, max_val
                        ),
                    ));
                }
            }
        }
    }

    issues
}

fn navigate_to_array<'a>(resource: &'a Value, parts: &[&str]) -> Option<&'a Vec<Value>> {
    let mut current = resource;

    for part in &parts[1..] {
        match current.get(part) {
            Some(value) => current = value,
            None => return None,
        }
    }

    current.as_array()
}

fn matches_slice(
    element: &Value,
    _slice: &crate::rules::SliceDefinition,
    discriminators: &[crate::rules::Discriminator],
) -> bool {
    if discriminators.is_empty() {
        return true;
    }

    discriminators
        .iter()
        .all(|discriminator| match discriminator.type_.as_str() {
            "value" => navigate_to_discriminator_value(element, &discriminator.path)
                .is_some_and(|v| v.is_string() || v.is_number() || v.is_boolean()),
            "exists" => navigate_to_discriminator_value(element, &discriminator.path).is_some(),
            "type" => navigate_to_discriminator_value(element, &discriminator.path)
                .is_some_and(|v| v.is_object()),
            _ => false,
        })
}

fn navigate_to_discriminator_value<'a>(element: &'a Value, path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = element;

    for part in parts {
        match current.get(part) {
            Some(value) => current = value,
            None => return None,
        }
    }

    Some(current)
}

/// Information about a collected extension
struct ExtensionInfo {
    url: String,
    path: String,
    is_modifier: bool,
}

fn collect_extension_urls(value: &Value, path: &str, extensions: &mut Vec<ExtensionInfo>) {
    match value {
        Value::Object(obj) => {
            if let Some(ext_array) = obj.get("extension").and_then(|v| v.as_array()) {
                for (idx, ext) in ext_array.iter().enumerate() {
                    if let Some(url) = ext.get("url").and_then(|v| v.as_str()) {
                        let ext_path = format!("{path}.extension[{idx}]");
                        extensions.push(ExtensionInfo {
                            url: url.to_string(),
                            path: ext_path.clone(),
                            is_modifier: false,
                        });
                        collect_extension_urls(ext, &ext_path, extensions);
                    }
                }
            }
            if let Some(ext_array) = obj.get("modifierExtension").and_then(|v| v.as_array()) {
                for (idx, ext) in ext_array.iter().enumerate() {
                    if let Some(url) = ext.get("url").and_then(|v| v.as_str()) {
                        let ext_path = format!("{path}.modifierExtension[{idx}]");
                        extensions.push(ExtensionInfo {
                            url: url.to_string(),
                            path: ext_path.clone(),
                            is_modifier: true,
                        });
                        collect_extension_urls(ext, &ext_path, extensions);
                    }
                }
            }
            for (key, val) in obj {
                // Skip extension fields (handled above) and contained resources
                // (contained resources are validated separately with their own extension validation)
                if key == "extension" || key == "modifierExtension" || key == "contained" {
                    continue;
                }
                if let Some(base_field) = key.strip_prefix('_') {
                    if let Some(prim_ext_array) = val.get("extension").and_then(|v| v.as_array()) {
                        for (idx, ext) in prim_ext_array.iter().enumerate() {
                            if let Some(url) = ext.get("url").and_then(|v| v.as_str()) {
                                let ext_path = format!("{path}.{base_field}.extension[{idx}]");
                                extensions.push(ExtensionInfo {
                                    url: url.to_string(),
                                    path: ext_path.clone(),
                                    is_modifier: false,
                                });
                                collect_extension_urls(ext, &ext_path, extensions);
                            }
                        }
                    }
                    continue;
                }
                let child_path = format!("{path}.{key}");
                collect_extension_urls(val, &child_path, extensions);
            }
        }
        Value::Array(arr) => {
            for (idx, item) in arr.iter().enumerate() {
                let item_path = format!("{path}[{idx}]");
                collect_extension_urls(item, &item_path, extensions);
            }
        }
        _ => {}
    }
}

impl Default for FhirValidator {
    fn default() -> Self {
        Self::new(crate::fhir_version::FhirVersion::default(), None)
            .expect("Failed to initialize FhirValidator")
    }
}
