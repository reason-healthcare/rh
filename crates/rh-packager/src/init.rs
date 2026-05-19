//! Scaffolds a new FHIR Package source directory (`rh package init`).

use crate::Result;
use serde_json::json;
use std::path::{Path, PathBuf};

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
/// - `packager.toml` — package metadata and hook configuration
/// - `ImplementationGuide.json` — minimal IG derived from manifest fields
/// - `input/` — input directory with conventional subdirectories:
///   - `input/fsh/` — FHIR Shorthand source
///   - `input/cql/` — CQL source
///   - `input/narrative/` — resource-matched narrative markdown
///   - `input/docs/` — standalone pages
///   - `input/examples/` — example FHIR resources
///
/// The `package.json` is **not** created in the source directory; it is generated
/// into the output directory during `rh package build`.
///
/// # Returns
/// A list of paths that were created.
///
/// # Errors
/// Returns an error if `packager.toml` already exists in `dir`.
pub fn init_package(dir: &Path, opts: InitOptions) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;

    let toml_path = dir.join("packager.toml");
    if toml_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "packager.toml already exists — directory is already initialised",
        )
        .into());
    }

    let mut created: Vec<PathBuf> = Vec::new();

    let (base_dep_id, base_dep_version) = base_fhir_dependency(&opts.fhir_version);

    // --- packager.toml ---
    std::fs::write(
        &toml_path,
        build_packager_toml(&opts, base_dep_id, base_dep_version),
    )?;
    created.push(toml_path);

    // --- ImplementationGuide.json ---
    let ig_path = dir.join("ImplementationGuide.json");
    if !ig_path.exists() {
        let ig = build_implementation_guide(&opts, base_dep_id, base_dep_version);
        let ig_json = serde_json::to_string_pretty(&ig)?;
        std::fs::write(&ig_path, ig_json)?;
        created.push(ig_path);
    }

    // --- input/ directory structure ---
    for subdir in &["fsh", "cql", "narrative", "docs", "examples"] {
        let subdir_path = dir.join("input").join(subdir);
        std::fs::create_dir_all(&subdir_path)?;
        let gitkeep = subdir_path.join(".gitkeep");
        std::fs::write(&gitkeep, "")?;
        created.push(subdir_path);
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

fn build_packager_toml(opts: &InitOptions, base_dep_id: &str, base_dep_version: &str) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push("# packager.toml — rh package configuration".to_string());
    lines.push("# See docs/PACKAGER.md for full documentation.".to_string());
    lines.push(String::new());
    lines.push(format!("id           = {:?}", opts.name));
    lines.push(format!("version      = {:?}", opts.version));
    lines.push(format!("canonical    = {:?}", opts.canonical));
    lines.push(format!("fhir_version = {:?}", opts.fhir_version));
    lines.push(format!("license      = {:?}", opts.license));
    lines.push(format!("status       = {:?}", opts.status));

    if let Some(desc) = &opts.description {
        lines.push(format!("description  = {:?}", desc));
    }
    if let Some(author) = &opts.author {
        lines.push(format!("author       = {:?}", author));
    }

    lines.push(String::new());
    lines.push("[dependencies]".to_string());
    lines.push(format!("{:?} = {:?}", base_dep_id, base_dep_version));
    lines.push(String::new());
    lines.push("[hooks]".to_string());
    lines.push("# Built-in processors: \"fsh\", \"snapshot\", \"cql\", \"validate\"".to_string());
    lines.push("before_build = []".to_string());
    lines.push("after_build  = []".to_string());
    lines.push("before_pack  = []".to_string());
    lines.push("after_pack   = []".to_string());
    lines.push(String::new());
    lines.push("# Custom shell processors — uncomment to define your own:".to_string());
    lines.push("# [processors.my-script]".to_string());
    lines.push("# command = \"python3 scripts/my_script.py\"".to_string());
    lines.push(String::new());
    lines.push("# Input directory layout — defaults shown, uncomment to override:".to_string());
    lines.push("# [input]".to_string());
    lines.push("# dir          = \"input\"      # base input directory".to_string());
    lines.push("# fsh_dir      = \"fsh\"        # FSH source (relative to dir)".to_string());
    lines.push("# cql_dir      = \"cql\"        # CQL source (relative to dir)".to_string());
    lines.push("# narrative_dir = \"narrative\" # resource-matched narrative markdown".to_string());
    lines.push("# docs_dir     = \"docs\"       # standalone pages (relative to dir)".to_string());
    lines.push("# examples_dir = \"examples\"   # example resources (relative to dir)".to_string());
    lines.push(String::new());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn creates_expected_files_and_input_dirs() {
        let dir = TempDir::new().unwrap();
        let opts = InitOptions {
            name: "com.example.fhir".to_string(),
            canonical: "https://example.org/fhir".to_string(),
            ..Default::default()
        };
        let created = init_package(dir.path(), opts).unwrap();
        // 2 files + 5 input subdirs
        assert_eq!(created.len(), 7);
        assert!(!dir.path().join("package.json").exists());
        assert!(dir.path().join("packager.toml").exists());
        assert!(dir.path().join("ImplementationGuide.json").exists());
        for subdir in &["fsh", "cql", "narrative", "docs", "examples"] {
            assert!(
                dir.path().join("input").join(subdir).is_dir(),
                "Missing input/{subdir}"
            );
        }
    }

    #[test]
    fn packager_toml_has_correct_fields() {
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

        let toml_str = std::fs::read_to_string(dir.path().join("packager.toml")).unwrap();
        let cfg: crate::config::PublisherConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(cfg.id.as_deref(), Some("com.example.fhir"));
        assert_eq!(cfg.version.as_deref(), Some("1.0.0"));
        assert_eq!(cfg.canonical.as_deref(), Some("https://example.org/fhir"));
        assert_eq!(cfg.fhir_version.as_deref(), Some("4.0.1"));
        assert_eq!(cfg.description.as_deref(), Some("A test package"));
        assert_eq!(cfg.author.as_deref(), Some("Test Org"));
        assert_eq!(
            cfg.dependencies.get("hl7.fhir.r4.core").map(|s| s.as_str()),
            Some("4.0.1")
        );
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
    fn errors_if_packager_toml_already_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("packager.toml"), "").unwrap();
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
