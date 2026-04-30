//! Scaffolds a new FHIR Package source directory (`rh package init`).

use crate::{manifest::PackageJson, Result};
use serde_json::json;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Options for initialising a new FHIR Package source directory.
#[derive(Debug, Clone)]
pub struct InitOptions {
    /// Package identifier in reverse-DNS NPM style (e.g. `com.example.fhir`). Required.
    pub name: String,

    /// Canonical URL base for resources (e.g. `https://example.org/fhir`). Required.
    pub canonical: String,

    /// SemVer package version. Defaults to `"0.1.0"`.
    pub version: String,

    /// FHIR version. Defaults to `"4.0.1"`.
    pub fhir_version: String,

    /// Human-readable title (optional; derived from `name` if absent).
    pub title: Option<String>,

    /// Package description (optional).
    pub description: Option<String>,

    /// Author / publisher name (optional).
    pub author: Option<String>,

    /// SPDX license identifier. Defaults to `"CC0-1.0"`.
    pub license: String,

    /// IG resource status (`draft`, `active`, `retired`). Defaults to `"draft"`.
    pub status: String,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            name: String::new(),
            canonical: String::new(),
            version: "0.1.0".to_string(),
            fhir_version: "4.0.1".to_string(),
            title: None,
            description: None,
            author: None,
            license: "CC0-1.0".to_string(),
            status: "draft".to_string(),
        }
    }
}

/// Scaffold a new FHIR Package source directory at `dir`.
///
/// Creates the target directory if it does not exist, then writes:
/// - `package.json` — FHIR package manifest
/// - `packager.toml` — hook configuration skeleton
/// - `ImplementationGuide.json` — minimal IG derived from manifest fields
///
/// # Returns
/// A list of paths that were created.
///
/// # Errors
/// Returns an error if `package.json` already exists in `dir`.
pub fn init_package(dir: &Path, opts: InitOptions) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;

    let pkg_json_path = dir.join("package.json");
    if pkg_json_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "package.json already exists — directory is already initialised",
        )
        .into());
    }

    let mut created: Vec<PathBuf> = Vec::new();

    // --- package.json ---
    let (base_dep_id, base_dep_version) = base_fhir_dependency(&opts.fhir_version);
    let mut dependencies = HashMap::new();
    dependencies.insert(base_dep_id.to_string(), base_dep_version.to_string());

    let pkg = PackageJson {
        name: opts.name.clone(),
        version: opts.version.clone(),
        fhir_versions: vec![opts.fhir_version.clone()],
        dependencies,
        url: Some(opts.canonical.clone()),
        description: opts.description.clone(),
        author: opts.author.clone(),
        license: Some(opts.license.clone()),
        extra: HashMap::new(),
    };
    let pkg_json = serde_json::to_string_pretty(&pkg)?;
    std::fs::write(&pkg_json_path, pkg_json)?;
    created.push(pkg_json_path);

    // --- packager.toml ---
    let toml_path = dir.join("packager.toml");
    if !toml_path.exists() {
        std::fs::write(&toml_path, packager_toml_template())?;
        created.push(toml_path);
    }

    // --- ImplementationGuide.json ---
    let ig_path = dir.join("ImplementationGuide.json");
    if !ig_path.exists() {
        let ig = build_implementation_guide(&opts, base_dep_id, base_dep_version);
        let ig_json = serde_json::to_string_pretty(&ig)?;
        std::fs::write(&ig_path, ig_json)?;
        created.push(ig_path);
    }

    Ok(created)
}

/// Returns `(package_id, version)` for the base FHIR core dependency implied
/// by the given FHIR version string.
fn base_fhir_dependency(fhir_version: &str) -> (&'static str, &'static str) {
    if fhir_version.starts_with("4.3") {
        ("hl7.fhir.r4b.core", "4.3.0")
    } else if fhir_version.starts_with("5.") {
        ("hl7.fhir.r5.core", "5.0.0")
    } else {
        ("hl7.fhir.r4.core", "4.0.1")
    }
}

/// Convert a dot/hyphen/underscore-separated package id to PascalCase (IG `name` field).
///
/// `"com.example.fhir"` → `"ComExampleFhir"`
fn to_pascal_case(s: &str) -> String {
    s.split(['.', '-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert a dot-separated package id to a hyphen-separated IG `id`.
///
/// `"com.example.fhir"` → `"com-example-fhir"`
fn to_ig_id(name: &str) -> String {
    name.replace('.', "-")
}

/// Derive a FHIR package name from a canonical URL following the FHIR naming convention.
///
/// The convention used by HL7 and most FHIR IGs is:
///
/// ```text
/// http://hl7.org/fhir/us/core  →  hl7.fhir.us.core
/// https://example.org/fhir     →  example.fhir
/// ```
///
/// Algorithm:
/// 1. Strip the URL scheme.
/// 2. Extract the second-level domain (SLD) from the host — e.g. `hl7` from `hl7.org`.
/// 3. Append path segments joined by `.`.
///
/// Returns `None` if the URL cannot be parsed.
pub fn name_from_canonical(canonical: &str) -> Option<String> {
    let without_scheme = canonical
        .strip_prefix("https://")
        .or_else(|| canonical.strip_prefix("http://"))?;

    let (host_with_port, path) = without_scheme
        .split_once('/')
        .unwrap_or((without_scheme, ""));

    // Strip port if present (e.g. "localhost:8080")
    let host = host_with_port.split(':').next().unwrap_or(host_with_port);

    // Second-level domain: second-to-last label, or the only label when there's just one.
    let host_labels: Vec<&str> = host.split('.').filter(|s| !s.is_empty()).collect();
    let sld = match host_labels.len() {
        0 => return None,
        1 => host_labels[0],
        n => host_labels[n - 2],
    };

    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    let mut parts = vec![sld];
    parts.extend_from_slice(&path_segments);

    Some(parts.join("."))
}

/// Build a minimal `ImplementationGuide` JSON value from `InitOptions`.
fn build_implementation_guide(
    opts: &InitOptions,
    base_dep_id: &str,
    base_dep_version: &str,
) -> serde_json::Value {
    let ig_id = to_ig_id(&opts.name);
    let ig_name = to_pascal_case(&opts.name);
    let title = opts.title.clone().unwrap_or_else(|| ig_name.clone());
    let canonical_url = format!(
        "{}/ImplementationGuide/{}",
        opts.canonical.trim_end_matches('/'),
        &opts.name
    );
    // Derive a clean id for the dependsOn entry (no dots or hyphens).
    let dep_entry_id = base_dep_id.replace(['.', '-'], "");

    let mut ig = json!({
        "resourceType": "ImplementationGuide",
        "id": ig_id,
        "url": canonical_url,
        "version": opts.version,
        "name": ig_name,
        "title": title,
        "status": opts.status,
        "packageId": opts.name,
        "fhirVersion": [opts.fhir_version],
        "dependsOn": [
            {
                "id": dep_entry_id,
                "packageId": base_dep_id,
                "version": base_dep_version,
                "uri": format!("http://hl7.org/fhir/{base_dep_version}")
            }
        ],
        "definition": {
            "resource": []
        }
    });

    if let Some(desc) = &opts.description {
        ig["description"] = json!(desc);
    }
    if let Some(author) = &opts.author {
        ig["publisher"] = json!(author);
    }

    ig
}

fn packager_toml_template() -> &'static str {
    r#"# packager.toml — rh package hook configuration
# See docs/PACKAGER.md for full documentation.

[hooks]
# Processors to run before the build stage.
# Built-in processors: "fsh", "snapshot", "cql", "validate"
before_build = []

# Processors to run after the build stage.
after_build = []

# Processors to run before packing to .tgz.
before_pack = []

# Processors to run after packing to .tgz.
after_pack = []

# Custom shell processors — uncomment to define your own:
# [processors.my-script]
# command = "python3 scripts/my_script.py"
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn creates_all_three_files() {
        let dir = TempDir::new().unwrap();
        let opts = InitOptions {
            name: "com.example.fhir".to_string(),
            canonical: "https://example.org/fhir".to_string(),
            ..Default::default()
        };
        let created = init_package(dir.path(), opts).unwrap();
        assert_eq!(created.len(), 3);
        assert!(dir.path().join("package.json").exists());
        assert!(dir.path().join("packager.toml").exists());
        assert!(dir.path().join("ImplementationGuide.json").exists());
    }

    #[test]
    fn package_json_has_correct_fields() {
        let dir = TempDir::new().unwrap();
        let opts = InitOptions {
            name: "com.example.fhir".to_string(),
            canonical: "https://example.org/fhir".to_string(),
            version: "1.0.0".to_string(),
            description: Some("A test package".to_string()),
            author: Some("Test Org".to_string()),
            ..Default::default()
        };
        init_package(dir.path(), opts).unwrap();

        let json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(dir.path().join("package.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(json["name"], "com.example.fhir");
        assert_eq!(json["version"], "1.0.0");
        assert_eq!(json["fhirVersions"][0], "4.0.1");
        assert_eq!(json["dependencies"]["hl7.fhir.r4.core"], "4.0.1");
        assert_eq!(json["description"], "A test package");
        assert_eq!(json["author"], "Test Org");
    }

    #[test]
    fn ig_json_has_correct_fields() {
        let dir = TempDir::new().unwrap();
        let opts = InitOptions {
            name: "com.example.fhir".to_string(),
            canonical: "https://example.org/fhir".to_string(),
            title: Some("Example FHIR Package".to_string()),
            ..Default::default()
        };
        init_package(dir.path(), opts).unwrap();

        let ig: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(dir.path().join("ImplementationGuide.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(ig["resourceType"], "ImplementationGuide");
        assert_eq!(ig["id"], "com-example-fhir");
        assert_eq!(ig["name"], "ComExampleFhir");
        assert_eq!(ig["title"], "Example FHIR Package");
        assert_eq!(ig["packageId"], "com.example.fhir");
        assert_eq!(
            ig["url"],
            "https://example.org/fhir/ImplementationGuide/com.example.fhir"
        );
    }

    #[test]
    fn errors_if_package_json_already_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("package.json"), "{}").unwrap();
        let opts = InitOptions {
            name: "com.example.fhir".to_string(),
            canonical: "https://example.org/fhir".to_string(),
            ..Default::default()
        };
        let result = init_package(dir.path(), opts);
        assert!(result.is_err());
    }

    #[test]
    fn r4b_dependency_for_4_3_fhir_version() {
        assert_eq!(
            base_fhir_dependency("4.3.0"),
            ("hl7.fhir.r4b.core", "4.3.0")
        );
    }

    #[test]
    fn r5_dependency_for_5_x_fhir_version() {
        assert_eq!(base_fhir_dependency("5.0.0"), ("hl7.fhir.r5.core", "5.0.0"));
    }

    #[test]
    fn to_pascal_case_handles_dots_and_hyphens() {
        assert_eq!(to_pascal_case("com.example.fhir"), "ComExampleFhir");
        assert_eq!(to_pascal_case("hl7.fhir.us.core"), "Hl7FhirUsCore");
        assert_eq!(to_pascal_case("my-package"), "MyPackage");
    }

    #[test]
    fn to_ig_id_replaces_dots_with_hyphens() {
        assert_eq!(to_ig_id("com.example.fhir"), "com-example-fhir");
    }

    #[test]
    fn name_from_canonical_follows_fhir_convention() {
        assert_eq!(
            name_from_canonical("http://hl7.org/fhir/us/core"),
            Some("hl7.fhir.us.core".to_string())
        );
        assert_eq!(
            name_from_canonical("http://hl7.org/fhir/uv/extensions"),
            Some("hl7.fhir.uv.extensions".to_string())
        );
        assert_eq!(
            name_from_canonical("https://example.org/fhir"),
            Some("example.fhir".to_string())
        );
        assert_eq!(
            name_from_canonical("https://example.org/fhir/my-ig"),
            Some("example.fhir.my-ig".to_string())
        );
        // Domain only (no path)
        assert_eq!(
            name_from_canonical("https://example.org"),
            Some("example".to_string())
        );
        // Subdomain — SLD is still the second-to-last label
        assert_eq!(
            name_from_canonical("https://build.fhir.org/ig/HL7/us-core"),
            Some("fhir.ig.HL7.us-core".to_string())
        );
        // No scheme → None
        assert_eq!(name_from_canonical("not-a-url"), None);
    }
}
