use serde::Deserialize;

/// Top-level configuration loaded from `publisher.toml`.
///
/// All sections are optional; absent sections use their `Default` implementations.
///
/// # Example
/// ```toml
/// [hooks]
/// before_build = ["snapshot", "cql", "validate"]
/// after_build  = []
/// before_pack  = []
/// after_pack   = []
///
/// [validate]
/// terminology_server = "https://tx.fhir.org/r4"
/// skip_invariants = false
///
/// [cql]
/// packages_dir = "~/.fhir/packages"
/// ```
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PublisherConfig {
    /// Hook stage processor lists.
    #[serde(default)]
    pub hooks: HooksConfig,

    /// Configuration for the built-in `validate` hook processor.
    #[serde(default)]
    pub validate: ValidateConfig,

    /// Configuration for the built-in `cql` hook processor.
    #[serde(default)]
    pub cql: CqlConfig,
}

impl PublisherConfig {
    /// Parse a `publisher.toml` file from its string contents.
    pub fn from_toml_str(s: &str) -> crate::Result<Self> {
        Ok(toml::from_str(s)?)
    }
}

/// Stage-level hook lists declaring which processors to run and in which order.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct HooksConfig {
    /// Processors to run before the build stage.
    #[serde(default)]
    pub before_build: Vec<String>,

    /// Processors to run after the build stage.
    #[serde(default)]
    pub after_build: Vec<String>,

    /// Processors to run before packing to `.tgz`.
    #[serde(default)]
    pub before_pack: Vec<String>,

    /// Processors to run after packing to `.tgz`.
    #[serde(default)]
    pub after_pack: Vec<String>,
}

/// Configuration for the `validate` built-in hook processor.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ValidateConfig {
    /// Override path to the local FHIR packages cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_dir: Option<String>,

    /// Skip FHIRPath invariant checks.
    #[serde(default)]
    pub skip_invariants: bool,

    /// Skip terminology binding checks.
    #[serde(default)]
    pub skip_bindings: bool,

    /// URL of a FHIR terminology server for binding validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_server: Option<String>,
}

/// Configuration for the `cql` built-in hook processor.
#[derive(Debug, Clone, Deserialize)]
pub struct CqlConfig {
    /// Override path to the local FHIR packages cache used for CQL resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_dir: Option<String>,

    /// CQL model info to use during compilation (default: `"fhir"`).
    #[serde(default = "default_model_info")]
    pub model_info: String,
}

fn default_model_info() -> String {
    "fhir".to_string()
}

impl Default for CqlConfig {
    fn default() -> Self {
        Self {
            packages_dir: None,
            model_info: default_model_info(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_toml_uses_all_defaults() {
        let cfg = PublisherConfig::from_toml_str("").unwrap();
        assert!(cfg.hooks.before_build.is_empty());
        assert!(cfg.hooks.after_build.is_empty());
        assert!(cfg.hooks.before_pack.is_empty());
        assert!(cfg.hooks.after_pack.is_empty());
        assert!(!cfg.validate.skip_invariants);
        assert!(!cfg.validate.skip_bindings);
        assert!(cfg.validate.terminology_server.is_none());
        assert!(cfg.cql.packages_dir.is_none());
        assert_eq!(cfg.cql.model_info, "fhir");
    }

    #[test]
    fn parses_hooks_section() {
        let toml = r#"
[hooks]
before_build = ["snapshot", "cql", "validate"]
after_build  = []
before_pack  = ["validate"]
after_pack   = []
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.hooks.before_build, vec!["snapshot", "cql", "validate"]);
        assert!(cfg.hooks.after_build.is_empty());
        assert_eq!(cfg.hooks.before_pack, vec!["validate"]);
    }

    #[test]
    fn parses_validate_section() {
        let toml = r#"
[validate]
terminology_server = "https://tx.fhir.org/r4"
skip_invariants = true
skip_bindings = false
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(
            cfg.validate.terminology_server.as_deref(),
            Some("https://tx.fhir.org/r4")
        );
        assert!(cfg.validate.skip_invariants);
        assert!(!cfg.validate.skip_bindings);
    }

    #[test]
    fn parses_cql_section() {
        let toml = r#"
[cql]
packages_dir = "/custom/packages"
model_info = "fhir"
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.cql.packages_dir.as_deref(), Some("/custom/packages"));
        assert_eq!(cfg.cql.model_info, "fhir");
    }

    #[test]
    fn missing_cql_model_info_defaults_to_fhir() {
        let toml = "[cql]\npackages_dir = \"/tmp\"";
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.cql.model_info, "fhir");
    }

    #[test]
    fn absent_hooks_block_defaults_to_empty_lists() {
        let toml = "[validate]\nskip_bindings = true";
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert!(cfg.hooks.before_build.is_empty());
        assert!(cfg.validate.skip_bindings);
    }
}
