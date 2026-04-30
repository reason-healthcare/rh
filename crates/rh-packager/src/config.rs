use serde::Deserialize;
use std::collections::HashMap;

/// Top-level configuration loaded from `packager.toml`.
///
/// All sections are optional; absent sections use their `Default` implementations.
///
/// # Example
/// ```toml
/// # Shared packages cache for all processors (override per-processor as needed).
/// packages_dir = "/custom/.fhir/packages"
///
/// # Package / IG metadata — used by processors that generate or validate resources.
/// canonical    = "https://example.org/fhir"
/// status       = "active"
/// publisher    = "My Organization"
///
/// [hooks]
/// before_build = ["snapshot", "cql", "validate"]
/// after_build  = []
/// before_pack  = []
/// after_pack   = []
///
/// [validate]
/// terminology_server = "https://tx.fhir.org/r4"
/// # packages_dir = "/override/for/validate"  # overrides top-level packages_dir
///
/// [cql]
/// # packages_dir = "/override/for/cql"       # overrides top-level packages_dir
/// ```
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PublisherConfig {
    /// Default path to the local FHIR packages cache, shared by all processors.
    ///
    /// Defaults to `$HOME/.fhir/packages`. Per-processor `packages_dir` settings
    /// override this value when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_dir: Option<String>,

    /// Canonical base URL for all generated resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<String>,

    /// FHIR version string (e.g. `"4.0.1"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fhir_version: Option<String>,

    /// Package identifier in `<scope>.<name>` NPM-style format (e.g. `hl7.fhir.us.core`).
    /// Maps to `name` in the generated `package.json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Human-readable package name (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource status: `active`, `draft`, `retired`, or `unknown`.
    /// Defaults to `"draft"` when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Publisher name (organization or individual).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    /// Package version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Package dependencies: `{ "<package-id>": "<version>" }`.
    ///
    /// Use quoted keys in TOML because FHIR package IDs contain dots:
    /// ```toml
    /// [dependencies]
    /// "hl7.fhir.r4.core" = "4.0.1"
    /// "hl7.fhir.us.core" = "6.1.0"
    /// ```
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    /// Human-readable package description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Author or publisher name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// SPDX license identifier (e.g. `"CC0-1.0"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// Hook stage processor lists.
    #[serde(default)]
    pub hooks: HooksConfig,

    /// Configuration for the built-in `validate` hook processor.
    #[serde(default)]
    pub validate: ValidateConfig,

    /// Configuration for the built-in `cql` hook processor.
    #[serde(default)]
    pub cql: CqlConfig,

    /// Configuration for the built-in `fsh` hook processor.
    #[serde(default)]
    pub fsh: FshConfig,

    /// Named shell processors available to all hook stages.
    ///
    /// Each key is a processor name referenced in `[hooks]` stage lists.
    ///
    /// ```toml
    /// [processors.enrich]
    /// command = "python scripts/enrich.py"
    ///
    /// [processors.lint]
    /// command = "./scripts/lint.sh"
    /// working_dir = "."
    /// timeout_secs = 30
    ///
    /// [processors.lint.env]
    /// STRICT = "1"
    /// ```
    #[serde(default)]
    pub processors: HashMap<String, ShellProcessorConfig>,

    /// Input directory layout configuration.
    ///
    /// When an `input/` directory is present at the project root (or the path configured
    /// here), structured layout is used. Otherwise the legacy flat-root layout is assumed
    /// for backward compatibility.
    ///
    /// ```toml
    /// [input]
    /// dir          = "input"      # base input directory (default)
    /// fsh_dir      = "fsh"        # FHIR Shorthand source (relative to dir)
    /// cql_dir      = "cql"        # CQL source (relative to dir)
    /// narrative_dir = "narrative" # resource-matched narrative markdown (relative to dir)
    /// docs_dir     = "docs"       # standalone pages (relative to dir)
    /// examples_dir = "examples"   # example resources (relative to dir)
    /// ```
    #[serde(default)]
    pub input: InputConfig,
}

/// Input directory layout configuration.
///
/// All subdirectory fields are relative to `dir`, which is itself relative to the project root.
/// Defaults match the conventional layout created by `rh package init`.
#[derive(Debug, Clone, Deserialize)]
pub struct InputConfig {
    /// Base input directory, relative to the project root.
    #[serde(default = "default_input_dir")]
    pub dir: String,

    /// Subdirectory containing FHIR Shorthand (`*.fsh`) source files.
    #[serde(default = "default_fsh_dir")]
    pub fsh_dir: String,

    /// Subdirectory containing CQL (`*.cql`) source files.
    #[serde(default = "default_cql_dir")]
    pub cql_dir: String,

    /// Subdirectory containing resource-matched narrative markdown files.
    ///
    /// A file `narrative/StructureDefinition-foo.md` is matched to the resource
    /// with stem `StructureDefinition-foo` and embedded as `resource.text`.
    #[serde(default = "default_narrative_dir")]
    pub narrative_dir: String,

    /// Subdirectory containing standalone markdown pages written to `package/other/`.
    #[serde(default = "default_docs_dir")]
    pub docs_dir: String,

    /// Subdirectory containing example FHIR resources written to `package/examples/`.
    #[serde(default = "default_examples_dir")]
    pub examples_dir: String,
}

fn default_input_dir() -> String {
    "input".to_string()
}
fn default_fsh_dir() -> String {
    "fsh".to_string()
}
fn default_cql_dir() -> String {
    "cql".to_string()
}
fn default_narrative_dir() -> String {
    "narrative".to_string()
}
fn default_docs_dir() -> String {
    "docs".to_string()
}
fn default_examples_dir() -> String {
    "examples".to_string()
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            dir: default_input_dir(),
            fsh_dir: default_fsh_dir(),
            cql_dir: default_cql_dir(),
            narrative_dir: default_narrative_dir(),
            docs_dir: default_docs_dir(),
            examples_dir: default_examples_dir(),
        }
    }
}

impl PublisherConfig {
    /// Parse a `packager.toml` file from its string contents.
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
    /// Falls back to the top-level `packages_dir` or `$HOME/.fhir/packages`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages_dir: Option<String>,

    /// Skip FHIRPath invariant checks.
    ///
    /// **Reserved for future use** — accepted in config but not yet wired to the
    /// underlying validator. Will take effect once `rh-validator` exposes the option.
    #[serde(default)]
    pub skip_invariants: bool,

    /// Skip terminology binding checks.
    ///
    /// **Reserved for future use** — accepted in config but not yet wired to the
    /// underlying validator. Will take effect once `rh-validator` exposes the option.
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
    /// Falls back to the top-level `packages_dir` or `$HOME/.fhir/packages`.
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

/// Configuration for the `fsh` built-in hook processor.
///
/// Package/IG metadata fields (`canonical`, `fhir_version`, `id`, `name`,
/// `status`, `publisher`, `version`) are root-level fields in `packager.toml`
/// and apply to all processors. This section is reserved for future
/// FSH-specific settings.
///
/// ```toml
/// [fsh]
/// # Future FSH-specific options go here.
/// ```
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FshConfig {}

/// Configuration for a named shell processor declared under `[processors.<name>]`.
///
/// Shell processors run an external command (bash, Python, Node.js, etc.) as a
/// pipeline stage. Resources are exchanged via a temporary working directory.
///
/// See `PROCESSORS.md` for the full execution contract and examples.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ShellProcessorConfig {
    /// Shell command to execute. Passed to `sh -c` on Unix or `cmd /C` on Windows.
    pub command: String,

    /// Working directory for the command, relative to the source directory.
    /// Defaults to the source directory when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// Maximum time in seconds the command may run before being killed.
    /// **Reserved for future use** — accepted in config but not yet enforced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,

    /// Additional environment variables set for the command.
    /// These are merged on top of the inherited process environment, overriding
    /// any variables with the same name.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_top_level_packages_dir() {
        let toml = r#"packages_dir = "/shared/packages""#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.packages_dir.as_deref(), Some("/shared/packages"));
    }

    #[test]
    fn empty_toml_uses_all_defaults() {
        let cfg = PublisherConfig::from_toml_str("").unwrap();
        assert!(cfg.packages_dir.is_none());
        assert!(cfg.canonical.is_none());
        assert!(cfg.fhir_version.is_none());
        assert!(cfg.id.is_none());
        assert!(cfg.name.is_none());
        assert!(cfg.status.is_none());
        assert!(cfg.publisher.is_none());
        assert!(cfg.version.is_none());
        assert!(cfg.dependencies.is_empty());
        assert!(cfg.description.is_none());
        assert!(cfg.author.is_none());
        assert!(cfg.license.is_none());
        assert!(cfg.hooks.before_build.is_empty());
        assert!(cfg.hooks.after_build.is_empty());
        assert!(cfg.hooks.before_pack.is_empty());
        assert!(cfg.hooks.after_pack.is_empty());
        assert!(!cfg.validate.skip_invariants);
        assert!(!cfg.validate.skip_bindings);
        assert!(cfg.validate.terminology_server.is_none());
        assert!(cfg.cql.packages_dir.is_none());
        assert_eq!(cfg.cql.model_info, "fhir");
        assert_eq!(cfg.input.dir, "input");
        assert_eq!(cfg.input.fsh_dir, "fsh");
        assert_eq!(cfg.input.cql_dir, "cql");
        assert_eq!(cfg.input.narrative_dir, "narrative");
        assert_eq!(cfg.input.docs_dir, "docs");
        assert_eq!(cfg.input.examples_dir, "examples");
    }

    #[test]
    fn parses_input_section_overrides() {
        let toml = r#"
[input]
dir           = "src"
fsh_dir       = "shorthand"
cql_dir       = "logic"
narrative_dir = "text"
docs_dir      = "pages"
examples_dir  = "samples"
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.input.dir, "src");
        assert_eq!(cfg.input.fsh_dir, "shorthand");
        assert_eq!(cfg.input.cql_dir, "logic");
        assert_eq!(cfg.input.narrative_dir, "text");
        assert_eq!(cfg.input.docs_dir, "pages");
        assert_eq!(cfg.input.examples_dir, "samples");
    }

    #[test]
    fn parses_root_level_meta_fields() {
        let toml = r#"
canonical    = "https://example.org/fhir"
fhir_version = "4.0.1"
id           = "my.package"
name         = "MyPackage"
status       = "active"
publisher    = "My Org"
version      = "1.0.0"
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.canonical.as_deref(), Some("https://example.org/fhir"));
        assert_eq!(cfg.fhir_version.as_deref(), Some("4.0.1"));
        assert_eq!(cfg.id.as_deref(), Some("my.package"));
        assert_eq!(cfg.name.as_deref(), Some("MyPackage"));
        assert_eq!(cfg.status.as_deref(), Some("active"));
        assert_eq!(cfg.publisher.as_deref(), Some("My Org"));
        assert_eq!(cfg.version.as_deref(), Some("1.0.0"));
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

    #[test]
    fn parses_dependencies_with_dotted_keys() {
        let toml = r#"
[dependencies]
"hl7.fhir.r4.core" = "4.0.1"
"hl7.fhir.us.core" = "6.1.0"
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(
            cfg.dependencies.get("hl7.fhir.r4.core").map(|s| s.as_str()),
            Some("4.0.1")
        );
        assert_eq!(
            cfg.dependencies.get("hl7.fhir.us.core").map(|s| s.as_str()),
            Some("6.1.0")
        );
    }

    #[test]
    fn parses_optional_metadata_fields() {
        let toml = r#"
description = "A test package"
author      = "Test Org"
license     = "Apache-2.0"
"#;
        let cfg = PublisherConfig::from_toml_str(toml).unwrap();
        assert_eq!(cfg.description.as_deref(), Some("A test package"));
        assert_eq!(cfg.author.as_deref(), Some("Test Org"));
        assert_eq!(cfg.license.as_deref(), Some("Apache-2.0"));
    }
}
