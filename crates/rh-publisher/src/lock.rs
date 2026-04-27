//! Canonical lock file (`fhir-lock.json`) — version pinning for external canonicals.
//!
//! Follows IGPublisher's `pin-all` pinning model:
//! - Scans source FHIR resources for canonical URL references
//! - Resolves each against locally installed dependency packages
//! - Records resolved versions in `fhir-lock.json`
//! - During build, applies pinning by appending `|version` to unversioned canonicals

use crate::{context::PublishContext, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};
use tracing::warn;

/// Well-known external code system URL prefixes that are never pinned.
const EXCLUDED_PREFIXES: &[&str] = &[
    "http://snomed.info/sct",
    "http://loinc.org",
    "http://www.nlm.nih.gov/research/umls/rxnorm",
    "http://hl7.org/fhir/sid/icd-10",
    "http://hl7.org/fhir/sid/icd-11",
    "http://terminology.hl7.org/CodeSystem/",
    "http://hl7.org/fhir/ValueSet/",
    "http://hl7.org/fhir/StructureDefinition/",
    "http://hl7.org/fhir/v2/",
    "http://hl7.org/fhir/v3/",
];

/// A single pinned canonical entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockedCanonical {
    /// The canonical URL (without version suffix).
    pub url: String,

    /// The resolved version string.
    #[serde(rename = "resolvedVersion")]
    pub resolved_version: String,

    /// The package that defines this canonical (`<name>#<version>`).
    #[serde(rename = "resolvedPackage")]
    pub resolved_package: String,

    /// Whether this canonical is pinned in build output.
    pub pinned: bool,
}

/// The full `fhir-lock.json` document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirLock {
    /// ISO 8601 timestamp of when the lock file was generated.
    pub generated: String,

    /// Pin mode: `"pin-all"` (default) or `"pin-none"` (opt-out).
    #[serde(rename = "pinMode")]
    pub pin_mode: String,

    /// Resolved canonical entries.
    pub canonicals: Vec<LockedCanonical>,
}

impl FhirLock {
    /// Build a lookup map: canonical URL → resolved version.
    pub fn to_pin_map(&self) -> HashMap<String, String> {
        self.canonicals
            .iter()
            .filter(|c| c.pinned)
            .map(|c| (c.url.clone(), c.resolved_version.clone()))
            .collect()
    }
}

/// Scan all resources in `ctx`, resolve canonicals against installed packages,
/// and write `fhir-lock.json` to the source directory. Returns the generated lock.
pub fn generate_lock(ctx: &PublishContext, packages_dir: &Path) -> Result<FhirLock> {
    let raw_urls: HashSet<String> = ctx
        .resources
        .values()
        .flat_map(|v| collect_canonicals(v))
        .filter(|url| !is_excluded(url) && !is_own_canonical(url, ctx))
        .collect();

    let mut canonicals: Vec<LockedCanonical> = Vec::new();

    for url in &raw_urls {
        match resolve_canonical(url, &ctx.package_json.dependencies, packages_dir) {
            Some(entry) => canonicals.push(entry),
            None => {
                warn!("Cannot resolve canonical: {url}");
            }
        }
    }

    canonicals.sort_by(|a, b| a.url.cmp(&b.url));

    let lock = FhirLock {
        generated: Utc::now().to_rfc3339(),
        pin_mode: "pin-all".to_string(),
        canonicals,
    };

    write_lock_file(ctx, &lock)?;
    Ok(lock)
}

/// Load `fhir-lock.json` from the source directory, if it exists.
pub fn load_lock(ctx: &PublishContext) -> Result<Option<FhirLock>> {
    let path = ctx.source_dir.join("fhir-lock.json");
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path)?;
    let lock: FhirLock = serde_json::from_str(&text)?;
    Ok(Some(lock))
}

/// Apply canonical pinning to all resources in `ctx.resources`.
/// Unversioned canonical URLs present in `pin_map` get `|version` appended.
pub fn apply_pinning(ctx: &mut PublishContext, pin_map: &HashMap<String, String>) {
    let keys: Vec<String> = ctx.resources.keys().cloned().collect();
    for key in keys {
        if let Some(resource) = ctx.resources.get_mut(&key) {
            pin_resource(resource, pin_map);
        }
    }
}

fn pin_resource(value: &mut Value, pin_map: &HashMap<String, String>) {
    match value {
        Value::String(s) => {
            if !s.contains('|') {
                if let Some(version) = pin_map.get(s.as_str()) {
                    *s = format!("{s}|{version}");
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                pin_resource(item, pin_map);
            }
        }
        Value::Object(map) => {
            for val in map.values_mut() {
                pin_resource(val, pin_map);
            }
        }
        _ => {}
    }
}

/// Walk a JSON value tree and collect all canonical URL candidates.
fn collect_canonicals(value: &Value) -> Vec<String> {
    let mut urls = Vec::new();
    collect_canonical_values(value, &mut urls);
    urls
}

fn collect_canonical_values(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            if looks_like_canonical(s) && !s.contains('|') {
                out.push(s.clone());
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_canonical_values(item, out);
            }
        }
        Value::Object(map) => {
            for val in map.values() {
                collect_canonical_values(val, out);
            }
        }
        _ => {}
    }
}

fn looks_like_canonical(s: &str) -> bool {
    (s.starts_with("http://") || s.starts_with("https://"))
        && s.contains('/')
        && !s.ends_with('/')
}

fn is_excluded(url: &str) -> bool {
    EXCLUDED_PREFIXES.iter().any(|prefix| url.starts_with(prefix))
}

fn is_own_canonical(url: &str, ctx: &PublishContext) -> bool {
    if let Some(base_url) = &ctx.package_json.url {
        url.starts_with(base_url.as_str())
    } else {
        false
    }
}

/// Try to resolve a canonical URL by scanning all dependency package directories.
fn resolve_canonical(
    url: &str,
    dependencies: &HashMap<String, String>,
    packages_dir: &Path,
) -> Option<LockedCanonical> {
    for (pkg_name, pkg_version) in dependencies {
        let pkg_dir = packages_dir.join(format!("{pkg_name}#{pkg_version}"));
        if !pkg_dir.exists() {
            continue;
        }
        if let Some(entry) = search_package_for_canonical(&pkg_dir, url, pkg_name, pkg_version) {
            return Some(entry);
        }
    }
    None
}

fn search_package_for_canonical(
    pkg_dir: &Path,
    url: &str,
    pkg_name: &str,
    pkg_version: &str,
) -> Option<LockedCanonical> {
    let read_dir = fs::read_dir(pkg_dir).ok()?;
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = fs::read_to_string(&path).ok()?;
        let value: Value = serde_json::from_str(&text).ok()?;
        if value.get("url").and_then(|v| v.as_str()) == Some(url) {
            let resolved_version = value
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or(pkg_version)
                .to_string();
            return Some(LockedCanonical {
                url: url.to_string(),
                resolved_version,
                resolved_package: format!("{pkg_name}#{pkg_version}"),
                pinned: true,
            });
        }
    }
    None
}

fn write_lock_file(ctx: &PublishContext, lock: &FhirLock) -> Result<()> {
    let path = ctx.source_dir.join("fhir-lock.json");
    let content = serde_json::to_string_pretty(lock)?;
    fs::write(&path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, context::PublishContext, manifest::PackageJson,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(tmp: &TempDir, resources: HashMap<String, Value>, pkg_url: Option<&str>, deps: HashMap<String, String>) -> PublishContext {
        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: PackageJson {
                name: "test.pkg".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: deps,
                url: pkg_url.map(|s| s.to_string()),
                description: None,
                author: None,
                license: None,
                extra: HashMap::new(),
            },
            resources,
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        }
    }

    #[test]
    fn collects_canonical_urls_from_resource() {
        let value = json!({
            "resourceType": "StructureDefinition",
            "baseDefinition": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
            "binding": {
                "valueSet": "http://example.org/fhir/ValueSet/my-vs"
            }
        });
        let urls = collect_canonicals(&value);
        assert!(urls.iter().any(|u| u.contains("us-core-patient")));
        assert!(urls.iter().any(|u| u.contains("my-vs")));
    }

    #[test]
    fn excludes_well_known_code_systems() {
        assert!(is_excluded("http://snomed.info/sct"));
        assert!(is_excluded("http://loinc.org"));
        assert!(is_excluded("http://www.nlm.nih.gov/research/umls/rxnorm"));
        assert!(is_excluded("http://hl7.org/fhir/sid/icd-10-cm"));
        assert!(!is_excluded("http://example.org/my-canonical"));
    }

    #[test]
    fn excludes_own_package_canonicals() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(
            &tmp,
            HashMap::new(),
            Some("http://mypackage.org/fhir"),
            HashMap::new(),
        );
        assert!(is_own_canonical("http://mypackage.org/fhir/StructureDefinition/foo", &ctx));
        assert!(!is_own_canonical("http://other.org/fhir/StructureDefinition/bar", &ctx));
    }

    #[test]
    fn already_versioned_urls_not_collected() {
        let value = json!({"url": "http://example.org/fhir/ValueSet/foo|1.0.0"});
        let urls = collect_canonicals(&value);
        assert!(urls.is_empty());
    }

    #[test]
    fn apply_pinning_appends_version() {
        let tmp = TempDir::new().unwrap();
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "baseDefinition": "http://example.org/fhir/StructureDefinition/base"
            }),
        );
        let mut ctx = make_ctx(&tmp, resources, None, HashMap::new());
        let mut pin_map = HashMap::new();
        pin_map.insert(
            "http://example.org/fhir/StructureDefinition/base".to_string(),
            "2.0.0".to_string(),
        );

        apply_pinning(&mut ctx, &pin_map);

        let resource = ctx.resources.get("StructureDefinition-foo").unwrap();
        let base = resource["baseDefinition"].as_str().unwrap();
        assert_eq!(base, "http://example.org/fhir/StructureDefinition/base|2.0.0");
    }

    #[test]
    fn apply_pinning_skips_already_versioned() {
        let tmp = TempDir::new().unwrap();
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({
                "baseDefinition": "http://example.org/fhir/StructureDefinition/base|1.5.0"
            }),
        );
        let mut ctx = make_ctx(&tmp, resources, None, HashMap::new());
        let mut pin_map = HashMap::new();
        pin_map.insert(
            "http://example.org/fhir/StructureDefinition/base".to_string(),
            "2.0.0".to_string(),
        );

        apply_pinning(&mut ctx, &pin_map);

        let resource = ctx.resources.get("StructureDefinition-foo").unwrap();
        let base = resource["baseDefinition"].as_str().unwrap();
        // Should remain at the author-specified version, not overridden.
        assert_eq!(base, "http://example.org/fhir/StructureDefinition/base|1.5.0");
    }

    #[test]
    fn generate_lock_resolves_from_local_package() {
        let tmp = TempDir::new().unwrap();
        let packages_dir = tmp.path().join("packages");

        // Create a fake local package containing a resource with a matching URL.
        let pkg_dir = packages_dir.join("dep.pkg#1.0.0");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(
            pkg_dir.join("StructureDefinition-base.json"),
            r#"{"resourceType":"StructureDefinition","url":"http://dep.org/fhir/StructureDefinition/base","version":"1.0.0"}"#,
        ).unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","baseDefinition":"http://dep.org/fhir/StructureDefinition/base"}),
        );

        let mut deps = HashMap::new();
        deps.insert("dep.pkg".to_string(), "1.0.0".to_string());

        let ctx = make_ctx(&tmp, resources, None, deps);
        let lock = generate_lock(&ctx, &packages_dir).unwrap();

        assert_eq!(lock.pin_mode, "pin-all");
        let entry = lock.canonicals.iter().find(|c| c.url.contains("base"));
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().resolved_version, "1.0.0");
        assert_eq!(entry.unwrap().resolved_package, "dep.pkg#1.0.0");
    }

    #[test]
    fn load_lock_returns_none_when_absent() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(&tmp, HashMap::new(), None, HashMap::new());
        let result = load_lock(&ctx).unwrap();
        assert!(result.is_none());
    }
}
