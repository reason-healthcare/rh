//! Canonical lock file (`fhir-lock.json`) — version pinning for canonical references.
//!
//! Follows IGPublisher's `pin-all` pinning model:
//! - Scans source FHIR resources for canonical URL references in typed canonical fields
//! - Resolves each against source resources (own package) and locally installed dependency packages
//! - Records resolved versions in `fhir-lock.json`
//! - During build, applies pinning by appending `|version` to unversioned canonicals

use crate::{context::PublishContext, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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

/// JSON field names whose values are of FHIR type `canonical`.
///
/// Pinning is only applied to these well-known fields to avoid incorrectly versioning
/// non-canonical URLs such as `url`, `system`, or `reference`.
const CANONICAL_FIELDS: &[&str] = &[
    "baseDefinition",        // StructureDefinition.baseDefinition
    "valueSet", // ElementDefinition.binding.valueSet, ValueSet.compose.include.valueSet
    "profile", // ElementDefinition.type.profile, meta.profile, CapabilityStatement.rest.resource.profile
    "targetProfile", // ElementDefinition.type.targetProfile
    "supportedProfile", // CapabilityStatement.rest.resource.supportedProfile
    "imports", // CapabilityStatement.imports
    "instantiatesCanonical", // Task, CarePlan, RequestGroup, etc.
    "library", // Measure, PlanDefinition, ActivityDefinition
    "derivedFrom", // SearchParameter, Questionnaire, StructureDefinition
];

/// A canonical reference found in a FHIR resource, with its pinning status.
#[derive(Debug, Clone)]
pub struct CanonicalRef {
    /// The canonical URL, without any `|version` suffix.
    pub url: String,

    /// The version string when the reference is already pinned (contains `|version`).
    pub pinned_version: Option<String>,

    /// Resource map key (`ResourceType-id`) where this reference was found.
    pub resource_key: String,

    /// Dot-notation path to the field within the resource (e.g. `binding.valueSet`).
    pub field_path: String,
}

/// Canonical pinning status report produced by [`lock_status`].
#[derive(Debug)]
pub struct LockReport {
    /// References that are already pinned (have a `|version` suffix).
    pub pinned: Vec<CanonicalRef>,

    /// References that are present but lack a version suffix.
    pub unpinned: Vec<CanonicalRef>,
}

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

/// Scan all resources in `ctx`, resolve canonicals against source resources and installed
/// packages, and write `fhir-lock.json` to the source directory. Returns the generated lock.
///
/// All canonical references are included — both references to resources in the same package
/// and references to external dependency packages.
pub fn generate_lock(ctx: &PublishContext, packages_dir: &Path) -> Result<FhirLock> {
    let raw_urls: HashSet<String> = ctx
        .resources
        .values()
        .flat_map(collect_canonicals)
        .filter(|url| !is_excluded(url))
        .collect();

    let mut canonicals: Vec<LockedCanonical> = Vec::new();

    for url in &raw_urls {
        let entry = if is_own_canonical(url, ctx) {
            resolve_own_canonical(url, ctx)
        } else {
            resolve_canonical(url, &ctx.package_json.dependencies, packages_dir)
        };
        match entry {
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

/// Scan all resources in `ctx` and report which canonical references are pinned
/// (have `|version`) and which are unversioned.
///
/// Only fields of FHIR type `canonical` (from [`CANONICAL_FIELDS`]) are inspected.
pub fn lock_status(ctx: &PublishContext) -> LockReport {
    let mut pinned = Vec::new();
    let mut unpinned = Vec::new();

    for (resource_key, resource) in &ctx.resources {
        walk_canonical_fields(resource, "", &mut |path, url| {
            if let Some(pipe_idx) = url.find('|') {
                pinned.push(CanonicalRef {
                    url: url[..pipe_idx].to_string(),
                    pinned_version: Some(url[pipe_idx + 1..].to_string()),
                    resource_key: resource_key.clone(),
                    field_path: path.to_string(),
                });
            } else {
                unpinned.push(CanonicalRef {
                    url: url.to_string(),
                    pinned_version: None,
                    resource_key: resource_key.clone(),
                    field_path: path.to_string(),
                });
            }
        });
    }

    pinned.sort_by(|a, b| a.url.cmp(&b.url).then(a.resource_key.cmp(&b.resource_key)));
    unpinned.sort_by(|a, b| a.url.cmp(&b.url).then(a.resource_key.cmp(&b.resource_key)));

    LockReport { pinned, unpinned }
}

fn pin_resource(value: &mut Value, pin_map: &HashMap<String, String>) {
    if let Value::Object(map) = value {
        pin_object(map, pin_map);
    }
}

fn pin_object(map: &mut Map<String, Value>, pin_map: &HashMap<String, String>) {
    for (key, val) in map.iter_mut() {
        if is_canonical_field(key) {
            pin_canonical_value(val, pin_map);
        } else {
            match val {
                Value::Object(m) => pin_object(m, pin_map),
                Value::Array(arr) => {
                    for item in arr.iter_mut() {
                        if let Value::Object(m) = item {
                            pin_object(m, pin_map);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn pin_canonical_value(val: &mut Value, pin_map: &HashMap<String, String>) {
    match val {
        Value::String(s) if !s.contains('|') => {
            if let Some(version) = pin_map.get(s.as_str()) {
                *s = format!("{s}|{version}");
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                pin_canonical_value(item, pin_map);
            }
        }
        _ => {}
    }
}

/// Collect unversioned canonical URL strings from canonical-typed fields in a resource.
fn collect_canonicals(value: &Value) -> Vec<String> {
    let mut urls = Vec::new();
    walk_canonical_fields(value, "", &mut |_path, url| {
        if !url.contains('|') {
            urls.push(url.to_string());
        }
    });
    urls
}

/// Walk a JSON value, invoking `visitor(field_path, url)` for each string value found in a
/// canonical-typed field (see [`CANONICAL_FIELDS`]).
///
/// `path` is the dot-notation path to the current position (empty at root, bracket-indexed for
/// arrays). Only object keys listed in [`CANONICAL_FIELDS`] trigger visitor calls; other keys
/// are recursed into to discover nested canonical fields.
fn walk_canonical_fields<F>(value: &Value, path: &str, visitor: &mut F)
where
    F: FnMut(&str, &str),
{
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let child = join_path(path, key);
                if is_canonical_field(key) {
                    visit_canonical_value(val, &child, visitor);
                } else {
                    walk_canonical_fields(val, &child, visitor);
                }
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                walk_canonical_fields(item, &format!("{path}[{i}]"), visitor);
            }
        }
        _ => {}
    }
}

/// Within a value known to be in a canonical-typed field, invoke `visitor` for each string
/// that looks like a canonical URL (versioned or unversioned).
fn visit_canonical_value<F>(value: &Value, path: &str, visitor: &mut F)
where
    F: FnMut(&str, &str),
{
    match value {
        Value::String(s) if looks_like_canonical_any(s) => visitor(path, s),
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                visit_canonical_value(item, &format!("{path}[{i}]"), visitor);
            }
        }
        _ => {}
    }
}

fn join_path(parent: &str, key: &str) -> String {
    if parent.is_empty() {
        key.to_string()
    } else {
        format!("{parent}.{key}")
    }
}

fn looks_like_canonical(s: &str) -> bool {
    (s.starts_with("http://") || s.starts_with("https://")) && s.contains('/') && !s.ends_with('/')
}

/// Same as `looks_like_canonical` but also accepts already-versioned strings (`url|version`).
fn looks_like_canonical_any(s: &str) -> bool {
    let base = s.split('|').next().unwrap_or(s);
    looks_like_canonical(base)
}

fn is_canonical_field(key: &str) -> bool {
    CANONICAL_FIELDS.contains(&key)
}

fn is_excluded(url: &str) -> bool {
    EXCLUDED_PREFIXES
        .iter()
        .any(|prefix| url.starts_with(prefix))
}

fn is_own_canonical(url: &str, ctx: &PublishContext) -> bool {
    if let Some(base_url) = &ctx.package_json.url {
        url.starts_with(base_url.as_str())
    } else {
        false
    }
}

/// Resolve a canonical URL against source resources in the same package.
fn resolve_own_canonical(url: &str, ctx: &PublishContext) -> Option<LockedCanonical> {
    for resource in ctx.resources.values() {
        if resource.get("url").and_then(|v| v.as_str()) == Some(url) {
            let resolved_version = resource
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or(&ctx.package_json.version)
                .to_string();
            return Some(LockedCanonical {
                url: url.to_string(),
                resolved_version,
                resolved_package: format!("{}#{}", ctx.package_json.name, ctx.package_json.version),
                pinned: true,
            });
        }
    }
    None
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
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(
        tmp: &TempDir,
        resources: HashMap<String, Value>,
        pkg_url: Option<&str>,
        deps: HashMap<String, String>,
    ) -> PublishContext {
        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            input_dir: tmp.path().to_path_buf(),
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
            examples: HashMap::new(),
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
    fn is_own_canonical_detects_own_package_urls() {
        let tmp = TempDir::new().unwrap();
        let ctx = make_ctx(
            &tmp,
            HashMap::new(),
            Some("http://mypackage.org/fhir"),
            HashMap::new(),
        );
        assert!(is_own_canonical(
            "http://mypackage.org/fhir/StructureDefinition/foo",
            &ctx
        ));
        assert!(!is_own_canonical(
            "http://other.org/fhir/StructureDefinition/bar",
            &ctx
        ));
    }

    #[test]
    fn already_versioned_urls_not_collected() {
        // `url` is not a canonical-typed field, so nothing is collected regardless.
        let value = json!({
            "resourceType": "StructureDefinition",
            "baseDefinition": "http://example.org/fhir/StructureDefinition/base|1.0.0"
        });
        let urls = collect_canonicals(&value);
        assert!(
            urls.is_empty(),
            "already-versioned canonicals should not be collected"
        );
    }

    #[test]
    fn non_canonical_fields_not_collected() {
        // `url` and `system` are not canonical-typed fields and should be ignored.
        let value = json!({
            "url": "http://example.org/fhir/StructureDefinition/foo",
            "system": "http://snomed.info/sct",
            "description": "http://example.org/not-a-canonical"
        });
        let urls = collect_canonicals(&value);
        assert!(
            urls.is_empty(),
            "non-canonical fields should not be collected"
        );
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
        assert_eq!(
            base,
            "http://example.org/fhir/StructureDefinition/base|2.0.0"
        );
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
        assert_eq!(
            base,
            "http://example.org/fhir/StructureDefinition/base|1.5.0"
        );
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

    #[test]
    fn generate_lock_includes_own_canonicals() {
        let tmp = TempDir::new().unwrap();
        let packages_dir = tmp.path().join("packages");
        fs::create_dir_all(&packages_dir).unwrap();

        let mut resources = HashMap::new();
        // Own resource that is referenced by another own resource.
        resources.insert(
            "StructureDefinition-base".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "url": "http://mypackage.org/fhir/StructureDefinition/base",
                "version": "1.0.0"
            }),
        );
        resources.insert(
            "StructureDefinition-child".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "baseDefinition": "http://mypackage.org/fhir/StructureDefinition/base"
            }),
        );

        let ctx = make_ctx(
            &tmp,
            resources,
            Some("http://mypackage.org/fhir"),
            HashMap::new(),
        );
        let lock = generate_lock(&ctx, &packages_dir).unwrap();

        let entry = lock.canonicals.iter().find(|c| c.url.contains("base"));
        assert!(entry.is_some(), "own canonical should be included in lock");
        assert_eq!(entry.unwrap().resolved_version, "1.0.0");
        assert_eq!(entry.unwrap().resolved_package, "test.pkg#1.0.0");
    }

    #[test]
    fn lock_status_classifies_pinned_and_unpinned() {
        let tmp = TempDir::new().unwrap();
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "baseDefinition": "http://example.org/fhir/StructureDefinition/base|1.0.0",
                "snapshot": {
                    "element": [{
                        "binding": {
                            "valueSet": "http://example.org/fhir/ValueSet/unversioned"
                        }
                    }]
                }
            }),
        );
        let ctx = make_ctx(&tmp, resources, None, HashMap::new());
        let report = lock_status(&ctx);

        assert_eq!(report.pinned.len(), 1);
        assert_eq!(
            report.pinned[0].url,
            "http://example.org/fhir/StructureDefinition/base"
        );
        assert_eq!(report.pinned[0].pinned_version.as_deref(), Some("1.0.0"));

        assert_eq!(report.unpinned.len(), 1);
        assert_eq!(
            report.unpinned[0].url,
            "http://example.org/fhir/ValueSet/unversioned"
        );
        assert!(report.unpinned[0].pinned_version.is_none());
    }
}
