//! CQL compiler options for controlling translation behavior.
//!
//! This module provides configuration options that control how CQL source
//! is translated to ELM, including annotation generation, semantic options,
//! and error handling.

use serde::{Deserialize, Serialize};

/// Individual compiler options that can be enabled or disabled.
///
/// These options control various aspects of the CQL-to-ELM translation process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CompilerOption {
    /// Enable date range optimization of retrieves.
    EnableDateRangeOptimization,
    /// Include source code annotations in the output ELM.
    EnableAnnotations,
    /// Include source code locators within output ELM.
    EnableLocators,
    /// Include result types in the output ELM.
    EnableResultTypes,
    /// Produce detailed error messages.
    EnableDetailedErrors,
    /// Disable traversal of paths on list-valued expressions.
    DisableListTraversal,
    /// Disable demotion of list-valued expressions to singletons.
    DisableListDemotion,
    /// Disable promotion of singletons to list-valued expressions.
    DisableListPromotion,
    /// Enable demotion of interval-valued expressions to points.
    EnableIntervalDemotion,
    /// Enable promotion of point-valued expressions to intervals.
    EnableIntervalPromotion,
    /// Disable method-style invocation support.
    DisableMethodInvocation,
    /// Require `from` keyword for all queries.
    RequireFromKeyword,
    /// Disable automatic loading of default model info.
    DisableDefaultModelInfoLoad,
    /// Disable implicit type conversions from ModelInfo (e.g., FHIR.Coding → System.Code).
    ///
    /// When enabled, the compiler will not automatically insert FHIRHelpers conversion
    /// function calls. This requires explicit conversion in CQL source code.
    DisableImplicitConversions,
    /// Treat missing conversion libraries as errors instead of warnings.
    ///
    /// When enabled, if a conversion is needed but the required library (e.g., FHIRHelpers)
    /// is not included, the compiler will emit an error instead of a warning.
    StrictConversionLibraryCheck,
}

/// Controls the level of signature information included in ELM output.
///
/// Signatures provide type information for function and operator invocations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SignatureLevel {
    /// Signatures will never be included in operator invocations.
    None,
    /// Signatures will only be included when the invocation signature
    /// differs from the declared signature.
    Differing,
    /// Signatures will be included when the operator or function has
    /// multiple overloads with the same number of arguments.
    #[default]
    Overloads,
    /// Signatures will always be included in invocations.
    All,
}

/// Minimum severity level for error reporting.
///
/// Messages below this severity will be filtered out.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ErrorSeverity {
    /// Report all messages including informational ones.
    #[default]
    Info,
    /// Report warnings and errors only.
    Warning,
    /// Report only errors.
    Error,
}

/// Output format for translated ELM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    /// JSON representation.
    #[serde(rename = "JSON")]
    Json,
    /// XML representation.
    #[serde(rename = "XML")]
    Xml,
}

/// Compiler options for CQL-to-ELM translation.
///
/// These options control various aspects of the translation process including
/// annotation generation, semantic behavior, and error reporting.
///
/// # Example
///
/// ```
/// use rh_cql::options::{CompilerOptions, CompilerOption, SignatureLevel, ErrorSeverity};
///
/// // Create options with default settings
/// let options = CompilerOptions::default();
///
/// // Create options with specific settings
/// let options = CompilerOptions::new()
///     .with_option(CompilerOption::EnableAnnotations)
///     .with_option(CompilerOption::EnableLocators)
///     .with_signature_level(SignatureLevel::All)
///     .with_error_level(ErrorSeverity::Warning);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    /// Set of enabled compiler options.
    #[serde(default)]
    pub options: std::collections::HashSet<CompilerOption>,

    /// Whether to validate unit strings in Quantity literals.
    #[serde(default = "default_true")]
    pub validate_units: bool,

    /// Whether to only verify the CQL without generating ELM.
    #[serde(default)]
    pub verify_only: bool,

    /// Whether to enable CQL-only mode (no model info).
    #[serde(default)]
    pub enable_cql_only: bool,

    /// CQL compatibility level (e.g., "1.5").
    #[serde(default = "default_compatibility_level")]
    pub compatibility_level: String,

    /// Minimum error severity to report.
    #[serde(default)]
    pub error_level: ErrorSeverity,

    /// Level of signature information to include.
    #[serde(default)]
    pub signature_level: SignatureLevel,

    /// Whether to analyze data requirements.
    #[serde(default)]
    pub analyze_data_requirements: bool,

    /// Whether to collapse data requirements.
    #[serde(default)]
    pub collapse_data_requirements: bool,
}

fn default_true() -> bool {
    true
}

fn default_compatibility_level() -> String {
    "1.5".to_string()
}

impl Default for CompilerOptions {
    /// Creates compiler options with recommended default settings.
    ///
    /// Default options include:
    /// - EnableAnnotations
    /// - EnableLocators
    /// - DisableListDemotion
    /// - DisableListPromotion
    /// - ErrorSeverity::Info
    /// - SignatureLevel::Overloads
    fn default() -> Self {
        let mut options = std::collections::HashSet::new();
        options.insert(CompilerOption::EnableAnnotations);
        options.insert(CompilerOption::EnableLocators);
        options.insert(CompilerOption::DisableListDemotion);
        options.insert(CompilerOption::DisableListPromotion);

        Self {
            options,
            validate_units: true,
            verify_only: false,
            enable_cql_only: false,
            compatibility_level: "1.5".to_string(),
            error_level: ErrorSeverity::Info,
            signature_level: SignatureLevel::Overloads,
            analyze_data_requirements: false,
            collapse_data_requirements: false,
        }
    }
}

impl CompilerOptions {
    /// Create new compiler options with no options enabled.
    pub fn new() -> Self {
        Self {
            options: std::collections::HashSet::new(),
            validate_units: true,
            verify_only: false,
            enable_cql_only: false,
            compatibility_level: "1.5".to_string(),
            error_level: ErrorSeverity::Info,
            signature_level: SignatureLevel::None,
            analyze_data_requirements: false,
            collapse_data_requirements: false,
        }
    }

    /// Create strict options that disable implicit conversions.
    ///
    /// Strict mode is equivalent to enabling:
    /// - DisableListTraversal
    /// - DisableListDemotion
    /// - DisableListPromotion
    /// - DisableMethodInvocation
    /// - DisableImplicitConversions
    /// - StrictConversionLibraryCheck
    pub fn strict() -> Self {
        let mut options = std::collections::HashSet::new();
        options.insert(CompilerOption::DisableListTraversal);
        options.insert(CompilerOption::DisableListDemotion);
        options.insert(CompilerOption::DisableListPromotion);
        options.insert(CompilerOption::DisableMethodInvocation);
        options.insert(CompilerOption::DisableImplicitConversions);
        options.insert(CompilerOption::StrictConversionLibraryCheck);

        Self {
            options,
            validate_units: true,
            verify_only: false,
            enable_cql_only: false,
            compatibility_level: "1.5".to_string(),
            error_level: ErrorSeverity::Info,
            signature_level: SignatureLevel::None,
            analyze_data_requirements: false,
            collapse_data_requirements: false,
        }
    }

    /// Create debug options that enable all annotation-related options.
    ///
    /// Debug mode enables:
    /// - EnableAnnotations
    /// - EnableLocators
    /// - EnableResultTypes
    pub fn debug() -> Self {
        let mut options = std::collections::HashSet::new();
        options.insert(CompilerOption::EnableAnnotations);
        options.insert(CompilerOption::EnableLocators);
        options.insert(CompilerOption::EnableResultTypes);

        Self {
            options,
            validate_units: true,
            verify_only: false,
            enable_cql_only: false,
            compatibility_level: "1.5".to_string(),
            error_level: ErrorSeverity::Info,
            signature_level: SignatureLevel::None,
            analyze_data_requirements: false,
            collapse_data_requirements: false,
        }
    }

    /// Add a compiler option.
    pub fn with_option(mut self, option: CompilerOption) -> Self {
        self.options.insert(option);
        self
    }

    /// Remove a compiler option.
    pub fn without_option(mut self, option: CompilerOption) -> Self {
        self.options.remove(&option);
        self
    }

    /// Set the signature level.
    pub fn with_signature_level(mut self, level: SignatureLevel) -> Self {
        self.signature_level = level;
        self
    }

    /// Set the error severity level.
    pub fn with_error_level(mut self, level: ErrorSeverity) -> Self {
        self.error_level = level;
        self
    }

    /// Set the compatibility level.
    pub fn with_compatibility_level(mut self, level: impl Into<String>) -> Self {
        self.compatibility_level = level.into();
        self
    }

    /// Set verify-only mode.
    pub fn with_verify_only(mut self, verify_only: bool) -> Self {
        self.verify_only = verify_only;
        self
    }

    /// Set unit validation.
    pub fn with_validate_units(mut self, validate: bool) -> Self {
        self.validate_units = validate;
        self
    }

    /// Check if an option is enabled.
    pub fn has_option(&self, option: CompilerOption) -> bool {
        self.options.contains(&option)
    }

    /// Check if annotations are enabled.
    pub fn annotations_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableAnnotations)
    }

    /// Check if locators are enabled.
    pub fn locators_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableLocators)
    }

    /// Check if result types are enabled.
    pub fn result_types_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableResultTypes)
    }

    /// Check if detailed errors are enabled.
    pub fn detailed_errors_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableDetailedErrors)
    }

    /// Check if list traversal is enabled.
    pub fn list_traversal_enabled(&self) -> bool {
        !self.has_option(CompilerOption::DisableListTraversal)
    }

    /// Check if list demotion is enabled.
    pub fn list_demotion_enabled(&self) -> bool {
        !self.has_option(CompilerOption::DisableListDemotion)
    }

    /// Check if list promotion is enabled.
    pub fn list_promotion_enabled(&self) -> bool {
        !self.has_option(CompilerOption::DisableListPromotion)
    }

    /// Check if interval demotion is enabled.
    pub fn interval_demotion_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableIntervalDemotion)
    }

    /// Check if interval promotion is enabled.
    pub fn interval_promotion_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableIntervalPromotion)
    }

    /// Check if method invocation is enabled.
    pub fn method_invocation_enabled(&self) -> bool {
        !self.has_option(CompilerOption::DisableMethodInvocation)
    }

    /// Check if date range optimization is enabled.
    pub fn date_range_optimization_enabled(&self) -> bool {
        self.has_option(CompilerOption::EnableDateRangeOptimization)
    }

    /// Check if `from` keyword is required.
    pub fn from_keyword_required(&self) -> bool {
        self.has_option(CompilerOption::RequireFromKeyword)
    }

    /// Check if implicit type conversions are enabled.
    ///
    /// When enabled (default), the compiler will automatically insert FHIRHelpers
    /// conversion function calls when type mismatches occur that can be resolved
    /// by ModelInfo-defined conversions.
    pub fn implicit_conversions_enabled(&self) -> bool {
        !self.has_option(CompilerOption::DisableImplicitConversions)
    }

    /// Check if strict conversion library checking is enabled.
    ///
    /// When enabled, missing conversion libraries (e.g., FHIRHelpers not included
    /// but FHIR.Coding → System.Code conversion needed) will result in errors
    /// instead of warnings.
    pub fn strict_conversion_library_check(&self) -> bool {
        self.has_option(CompilerOption::StrictConversionLibraryCheck)
    }

    /// Convert enabled options to a comma-separated string for serialization.
    ///
    /// This format matches the reference implementation's `translatorOptions` field.
    pub fn options_to_string(&self) -> String {
        let mut option_names: Vec<&str> = self
            .options
            .iter()
            .map(|opt| match opt {
                CompilerOption::EnableDateRangeOptimization => "EnableDateRangeOptimization",
                CompilerOption::EnableAnnotations => "EnableAnnotations",
                CompilerOption::EnableLocators => "EnableLocators",
                CompilerOption::EnableResultTypes => "EnableResultTypes",
                CompilerOption::EnableDetailedErrors => "EnableDetailedErrors",
                CompilerOption::DisableListTraversal => "DisableListTraversal",
                CompilerOption::DisableListDemotion => "DisableListDemotion",
                CompilerOption::DisableListPromotion => "DisableListPromotion",
                CompilerOption::EnableIntervalDemotion => "EnableIntervalDemotion",
                CompilerOption::EnableIntervalPromotion => "EnableIntervalPromotion",
                CompilerOption::DisableMethodInvocation => "DisableMethodInvocation",
                CompilerOption::RequireFromKeyword => "RequireFromKeyword",
                CompilerOption::DisableDefaultModelInfoLoad => "DisableDefaultModelInfoLoad",
                CompilerOption::DisableImplicitConversions => "DisableImplicitConversions",
                CompilerOption::StrictConversionLibraryCheck => "StrictConversionLibraryCheck",
            })
            .collect();
        option_names.sort();
        option_names.join(",")
    }

    /// Parse options from a comma-separated string.
    ///
    /// This parses the format used by the reference implementation's `translatorOptions` field.
    pub fn parse_options(options_str: &str) -> std::collections::HashSet<CompilerOption> {
        options_str
            .split(',')
            .filter_map(|s| {
                let s = s.trim();
                match s {
                    "EnableDateRangeOptimization" => {
                        Some(CompilerOption::EnableDateRangeOptimization)
                    }
                    "EnableAnnotations" => Some(CompilerOption::EnableAnnotations),
                    "EnableLocators" => Some(CompilerOption::EnableLocators),
                    "EnableResultTypes" => Some(CompilerOption::EnableResultTypes),
                    "EnableDetailedErrors" => Some(CompilerOption::EnableDetailedErrors),
                    "DisableListTraversal" => Some(CompilerOption::DisableListTraversal),
                    "DisableListDemotion" => Some(CompilerOption::DisableListDemotion),
                    "DisableListPromotion" => Some(CompilerOption::DisableListPromotion),
                    "EnableIntervalDemotion" => Some(CompilerOption::EnableIntervalDemotion),
                    "EnableIntervalPromotion" => Some(CompilerOption::EnableIntervalPromotion),
                    "DisableMethodInvocation" => Some(CompilerOption::DisableMethodInvocation),
                    "RequireFromKeyword" => Some(CompilerOption::RequireFromKeyword),
                    "DisableDefaultModelInfoLoad" => {
                        Some(CompilerOption::DisableDefaultModelInfoLoad)
                    }
                    "DisableImplicitConversions" => {
                        Some(CompilerOption::DisableImplicitConversions)
                    }
                    "StrictConversionLibraryCheck" => {
                        Some(CompilerOption::StrictConversionLibraryCheck)
                    }
                    _ => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = CompilerOptions::default();

        assert!(options.annotations_enabled());
        assert!(options.locators_enabled());
        assert!(!options.list_demotion_enabled());
        assert!(!options.list_promotion_enabled());
        assert_eq!(options.error_level, ErrorSeverity::Info);
        assert_eq!(options.signature_level, SignatureLevel::Overloads);
    }

    #[test]
    fn test_new_options() {
        let options = CompilerOptions::new();

        assert!(!options.annotations_enabled());
        assert!(!options.locators_enabled());
        assert!(options.list_demotion_enabled());
        assert!(options.list_promotion_enabled());
    }

    #[test]
    fn test_strict_options() {
        let options = CompilerOptions::strict();

        assert!(!options.list_traversal_enabled());
        assert!(!options.list_demotion_enabled());
        assert!(!options.list_promotion_enabled());
        assert!(!options.method_invocation_enabled());
    }

    #[test]
    fn test_debug_options() {
        let options = CompilerOptions::debug();

        assert!(options.annotations_enabled());
        assert!(options.locators_enabled());
        assert!(options.result_types_enabled());
    }

    #[test]
    fn test_builder_pattern() {
        let options = CompilerOptions::new()
            .with_option(CompilerOption::EnableAnnotations)
            .with_option(CompilerOption::EnableLocators)
            .with_signature_level(SignatureLevel::All)
            .with_error_level(ErrorSeverity::Warning);

        assert!(options.annotations_enabled());
        assert!(options.locators_enabled());
        assert_eq!(options.signature_level, SignatureLevel::All);
        assert_eq!(options.error_level, ErrorSeverity::Warning);
    }

    #[test]
    fn test_without_option() {
        let options = CompilerOptions::default().without_option(CompilerOption::EnableAnnotations);

        assert!(!options.annotations_enabled());
        assert!(options.locators_enabled());
    }

    #[test]
    fn test_options_to_string() {
        let options = CompilerOptions::new()
            .with_option(CompilerOption::EnableAnnotations)
            .with_option(CompilerOption::EnableLocators);

        let s = options.options_to_string();
        assert!(s.contains("EnableAnnotations"));
        assert!(s.contains("EnableLocators"));
    }

    #[test]
    fn test_parse_options() {
        let parsed =
            CompilerOptions::parse_options("EnableAnnotations,EnableLocators,DisableListDemotion");

        assert!(parsed.contains(&CompilerOption::EnableAnnotations));
        assert!(parsed.contains(&CompilerOption::EnableLocators));
        assert!(parsed.contains(&CompilerOption::DisableListDemotion));
        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn test_parse_options_with_spaces() {
        let parsed = CompilerOptions::parse_options("EnableAnnotations, EnableLocators");

        assert!(parsed.contains(&CompilerOption::EnableAnnotations));
        assert!(parsed.contains(&CompilerOption::EnableLocators));
    }

    #[test]
    fn test_serialization() {
        let options = CompilerOptions::default();
        let json = serde_json::to_string(&options).unwrap();

        assert!(json.contains("errorLevel"));
        assert!(json.contains("signatureLevel"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "options": ["EnableAnnotations", "EnableLocators"],
            "errorLevel": "Warning",
            "signatureLevel": "All"
        }"#;

        let options: CompilerOptions = serde_json::from_str(json).unwrap();
        assert!(options.annotations_enabled());
        assert!(options.locators_enabled());
        assert_eq!(options.error_level, ErrorSeverity::Warning);
        assert_eq!(options.signature_level, SignatureLevel::All);
    }

    #[test]
    fn test_signature_level_variants() {
        assert_eq!(SignatureLevel::default(), SignatureLevel::Overloads);

        let options = CompilerOptions::new().with_signature_level(SignatureLevel::None);
        assert_eq!(options.signature_level, SignatureLevel::None);

        let options = CompilerOptions::new().with_signature_level(SignatureLevel::Differing);
        assert_eq!(options.signature_level, SignatureLevel::Differing);

        let options = CompilerOptions::new().with_signature_level(SignatureLevel::All);
        assert_eq!(options.signature_level, SignatureLevel::All);
    }

    #[test]
    fn test_error_severity_variants() {
        assert_eq!(ErrorSeverity::default(), ErrorSeverity::Info);

        let options = CompilerOptions::new().with_error_level(ErrorSeverity::Warning);
        assert_eq!(options.error_level, ErrorSeverity::Warning);

        let options = CompilerOptions::new().with_error_level(ErrorSeverity::Error);
        assert_eq!(options.error_level, ErrorSeverity::Error);
    }

    #[test]
    fn test_compatibility_level() {
        let options = CompilerOptions::new().with_compatibility_level("1.4");
        assert_eq!(options.compatibility_level, "1.4");

        let options = CompilerOptions::default();
        assert_eq!(options.compatibility_level, "1.5");
    }

    #[test]
    fn test_verify_only() {
        let options = CompilerOptions::new().with_verify_only(true);
        assert!(options.verify_only);

        let options = CompilerOptions::default();
        assert!(!options.verify_only);
    }

    #[test]
    fn test_validate_units() {
        let options = CompilerOptions::new().with_validate_units(false);
        assert!(!options.validate_units);

        let options = CompilerOptions::default();
        assert!(options.validate_units);
    }

    #[test]
    fn test_implicit_conversions_enabled_by_default() {
        let options = CompilerOptions::default();
        assert!(options.implicit_conversions_enabled());
        assert!(!options.strict_conversion_library_check());
    }

    #[test]
    fn test_disable_implicit_conversions() {
        let options =
            CompilerOptions::new().with_option(CompilerOption::DisableImplicitConversions);

        assert!(!options.implicit_conversions_enabled());
    }

    #[test]
    fn test_strict_conversion_library_check() {
        let options =
            CompilerOptions::new().with_option(CompilerOption::StrictConversionLibraryCheck);

        assert!(options.strict_conversion_library_check());
    }

    #[test]
    fn test_strict_mode_includes_conversion_options() {
        let options = CompilerOptions::strict();

        assert!(!options.implicit_conversions_enabled());
        assert!(options.strict_conversion_library_check());
    }

    #[test]
    fn test_parse_conversion_options() {
        let parsed = CompilerOptions::parse_options(
            "DisableImplicitConversions,StrictConversionLibraryCheck",
        );

        assert!(parsed.contains(&CompilerOption::DisableImplicitConversions));
        assert!(parsed.contains(&CompilerOption::StrictConversionLibraryCheck));
    }

    #[test]
    fn test_conversion_options_to_string() {
        let options = CompilerOptions::new()
            .with_option(CompilerOption::DisableImplicitConversions)
            .with_option(CompilerOption::StrictConversionLibraryCheck);

        let s = options.options_to_string();
        assert!(s.contains("DisableImplicitConversions"));
        assert!(s.contains("StrictConversionLibraryCheck"));
    }
}
