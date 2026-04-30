use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed representation of a FHIR `package.json` manifest.
///
/// Follows the [FHIR Package Specification](https://hl7.org/fhir/packages.html).
/// Unknown fields are preserved in `extra` so a round-trip write produces an
/// equivalent file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    /// Package identifier in `<scope>.<name>` NPM-style format (e.g. `hl7.fhir.us.core`).
    pub name: String,

    /// SemVer package version.
    pub version: String,

    /// FHIR versions this package is compatible with (e.g. `["4.0.1"]`).
    #[serde(rename = "fhirVersions", default)]
    pub fhir_versions: Vec<String>,

    /// Map of package dependencies: `{ "<package-id>": "<version>" }`.
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    /// Canonical base URL for resources in this package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Human-readable package description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Author or publisher name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// SPDX license identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// Additional fields not explicitly modelled, preserved for round-trip fidelity.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl PackageJson {
    /// Construct a `PackageJson` from a [`PublisherConfig`].
    ///
    /// This is the primary way `package.json` is produced — derived from `packager.toml`
    /// rather than read from a file in the source directory.
    pub fn from_config(config: &crate::config::PublisherConfig) -> Self {
        PackageJson {
            name: config.id.clone().unwrap_or_default(),
            version: config
                .version
                .clone()
                .unwrap_or_else(|| "0.1.0".to_string()),
            fhir_versions: config
                .fhir_version
                .as_ref()
                .map(|v| vec![v.clone()])
                .unwrap_or_default(),
            dependencies: config.dependencies.clone(),
            url: config.canonical.clone(),
            description: config.description.clone(),
            author: config.author.clone(),
            license: config.license.clone(),
            extra: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_JSON: &str = r#"{
        "name": "example.fhir.package",
        "version": "1.0.0",
        "fhirVersions": ["4.0.1"],
        "dependencies": {
            "hl7.fhir.r4.core": "4.0.1"
        }
    }"#;

    const FULL_JSON: &str = r#"{
        "name": "example.fhir.package",
        "version": "1.0.0",
        "fhirVersions": ["4.0.1"],
        "dependencies": { "hl7.fhir.r4.core": "4.0.1" },
        "url": "http://example.org/fhir",
        "description": "An example package",
        "author": "Example Author",
        "license": "CC0-1.0",
        "someExtension": "extra-value"
    }"#;

    #[test]
    fn deserializes_minimal_package_json() {
        let pkg: PackageJson = serde_json::from_str(MINIMAL_JSON).unwrap();
        assert_eq!(pkg.name, "example.fhir.package");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.fhir_versions, vec!["4.0.1"]);
        assert_eq!(pkg.dependencies.get("hl7.fhir.r4.core").unwrap(), "4.0.1");
        assert!(pkg.url.is_none());
        assert!(pkg.description.is_none());
    }

    #[test]
    fn deserializes_full_package_json() {
        let pkg: PackageJson = serde_json::from_str(FULL_JSON).unwrap();
        assert_eq!(pkg.url.as_deref(), Some("http://example.org/fhir"));
        assert_eq!(pkg.description.as_deref(), Some("An example package"));
        assert_eq!(pkg.author.as_deref(), Some("Example Author"));
        assert_eq!(pkg.license.as_deref(), Some("CC0-1.0"));
        assert!(pkg.extra.contains_key("someExtension"));
    }

    #[test]
    fn round_trips_package_json() {
        let pkg: PackageJson = serde_json::from_str(FULL_JSON).unwrap();
        let serialized = serde_json::to_string(&pkg).unwrap();
        let pkg2: PackageJson = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pkg.name, pkg2.name);
        assert_eq!(pkg.version, pkg2.version);
        assert_eq!(pkg.url, pkg2.url);
        assert_eq!(
            pkg.extra.get("someExtension"),
            pkg2.extra.get("someExtension")
        );
    }

    #[test]
    fn missing_fhir_versions_defaults_to_empty() {
        let json = r#"{"name": "test", "version": "1.0.0"}"#;
        let pkg: PackageJson = serde_json::from_str(json).unwrap();
        assert!(pkg.fhir_versions.is_empty());
    }

    #[test]
    fn missing_dependencies_defaults_to_empty() {
        let json = r#"{"name": "test", "version": "1.0.0"}"#;
        let pkg: PackageJson = serde_json::from_str(json).unwrap();
        assert!(pkg.dependencies.is_empty());
    }

    #[test]
    fn from_config_maps_all_fields() {
        use crate::config::PublisherConfig;
        let toml = r#"
id           = "hl7.fhir.us.core"
version      = "6.1.0"
canonical    = "http://hl7.org/fhir/us/core"
fhir_version = "4.0.1"
description  = "US Core IG"
author       = "HL7 US Realm"
license      = "CC0-1.0"

[dependencies]
"hl7.fhir.r4.core" = "4.0.1"
"#;
        let config = PublisherConfig::from_toml_str(toml).unwrap();
        let pkg = PackageJson::from_config(&config);
        assert_eq!(pkg.name, "hl7.fhir.us.core");
        assert_eq!(pkg.version, "6.1.0");
        assert_eq!(pkg.fhir_versions, vec!["4.0.1"]);
        assert_eq!(pkg.url.as_deref(), Some("http://hl7.org/fhir/us/core"));
        assert_eq!(pkg.description.as_deref(), Some("US Core IG"));
        assert_eq!(pkg.author.as_deref(), Some("HL7 US Realm"));
        assert_eq!(pkg.license.as_deref(), Some("CC0-1.0"));
        assert_eq!(
            pkg.dependencies.get("hl7.fhir.r4.core").map(|s| s.as_str()),
            Some("4.0.1")
        );
        assert!(pkg.extra.is_empty());
    }

    #[test]
    fn from_config_uses_defaults_for_absent_fields() {
        use crate::config::PublisherConfig;
        let config = PublisherConfig::default();
        let pkg = PackageJson::from_config(&config);
        assert_eq!(pkg.name, "");
        assert_eq!(pkg.version, "0.1.0");
        assert!(pkg.fhir_versions.is_empty());
        assert!(pkg.dependencies.is_empty());
        assert!(pkg.url.is_none());
    }
}
