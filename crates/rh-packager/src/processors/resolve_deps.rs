//! `resolve-dependencies` hook processor — pulls in transitive dependencies
//! from installed FHIR packages so the resource set is self-contained.
//!
//! Walks all canonical references in the resource map (using the same
//! `walk_canonical_fields` logic from `lock.rs`), finds references that don't
//! resolve to any resource in the current map, and loads them from
//! dependency packages. Repeats until no new unresolved references are found
//! (fixpoint iteration).

use crate::{
    context::PublishContext, hooks::HookProcessor, lock, utils::resolve_packages_dir, Result,
};
use serde_json::Value;
use std::collections::HashSet;
use tracing::{info, warn};

/// Hook processor that resolves transitive dependencies from FHIR packages.
pub struct ResolveDependenciesProcessor;

impl HookProcessor for ResolveDependenciesProcessor {
    fn name(&self) -> &str {
        "resolve-dependencies"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let packages_dir = resolve_packages_dir(
            ctx.config.link.packages_dir.as_deref(),
            ctx.config.packages_dir.as_deref(),
        );

        let mut resolved_count = 0usize;
        let mut seen_urls: HashSet<String> = HashSet::new();

        // Fixpoint iteration: keep resolving until no new canonicals are found.
        loop {
            // Collect all canonical references from all resources in the map.
            let all_canonicals: HashSet<String> = ctx
                .resources
                .values()
                .flat_map(lock::collect_canonicals)
                .filter(|url| !lock::is_excluded(url))
                .collect();

            // Find canonicals that are not already in the resource map and not yet seen.
            let unresolved: Vec<String> = all_canonicals
                .iter()
                .filter(|url| !seen_urls.contains(*url))
                .filter(|url| !canonical_in_resources(ctx, url))
                .cloned()
                .collect();

            if unresolved.is_empty() {
                break;
            }

            for url in &unresolved {
                seen_urls.insert(url.clone());

                // Try to load from dependency packages.
                match load_from_packages(url, &ctx.package_json.dependencies, &packages_dir) {
                    Ok(Some(resource)) => {
                        let stem = resource_stem(&resource);
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ctx.resources.entry(stem)
                        {
                            e.insert(resource);
                            resolved_count += 1;
                        }
                    }
                    Ok(None) => {
                        // Not found in any dependency package.
                        // This will be caught by link-validate if it's a real problem.
                        warn!("Could not resolve canonical: {url}");
                    }
                    Err(e) => {
                        warn!("Error resolving canonical {url}: {e}");
                    }
                }
            }
        }

        info!("Resolved {} transitive dependencies", resolved_count);
        Ok(())
    }
}

/// Check if a canonical URL is already present as a resource in the context.
fn canonical_in_resources(ctx: &PublishContext, url: &str) -> bool {
    ctx.resources
        .values()
        .any(|v| v.get("url").and_then(|u| u.as_str()) == Some(url))
}

/// Derive a resource map key from a FHIR resource.
///
/// Returns `<ResourceType>-<id>` (preferred) or `<ResourceType>-<name>` as a
/// fallback, matching the naming convention used by the file loader.
fn resource_stem(resource: &Value) -> String {
    let rt = resource
        .get("resourceType")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    if let Some(id) = resource.get("id").and_then(|v| v.as_str()) {
        return format!("{rt}-{id}");
    }
    if let Some(name) = resource.get("name").and_then(|v| v.as_str()) {
        return format!("{rt}-{name}");
    }
    format!("{rt}-unknown")
}

/// Search all dependency packages for a resource matching the given canonical URL.
///
/// Returns the full resource JSON if found, or `None` if not found in any package.
fn load_from_packages(
    url: &str,
    dependencies: &std::collections::HashMap<String, String>,
    packages_dir: &std::path::Path,
) -> Result<Option<Value>> {
    for (pkg_name, pkg_version) in dependencies {
        let pkg_dir = packages_dir.join(format!("{pkg_name}#{pkg_version}"));
        if !pkg_dir.exists() {
            continue;
        }
        if let Some(resource) = search_package_for_resource(&pkg_dir, url)? {
            return Ok(Some(resource));
        }
    }
    Ok(None)
}

/// Scan a package directory for a resource with the given canonical URL.
fn search_package_for_resource(pkg_dir: &std::path::Path, url: &str) -> Result<Option<Value>> {
    let read_dir = std::fs::read_dir(pkg_dir)?;
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = std::fs::read_to_string(&path)?;
        let value: Value = serde_json::from_str(&text)?;
        if value.get("url").and_then(|v| v.as_str()) == Some(url) {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, context::PublishContext, hooks::HookProcessor,
        manifest::PackageJson,
    };
    use serde_json::json;
    use std::{collections::HashMap, fs};
    use tempfile::TempDir;

    fn make_ctx(
        tmp: &TempDir,
        resources: HashMap<String, Value>,
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
                url: Some("http://example.org/fhir".to_string()),
                canonical: Some("http://example.org/fhir".to_string()),
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
    fn resolves_value_set_from_dependency_package() {
        let tmp = TempDir::new().unwrap();
        let packages_dir = tmp.path().join("packages");

        // Create a fake dependency package with a ValueSet.
        let pkg_dir = packages_dir.join("dep.pkg#1.0.0");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(
            pkg_dir.join("ValueSet-test-codes.json"),
            r#"{"resourceType":"ValueSet","id":"test-codes","url":"http://dep.org/fhir/ValueSet/test-codes","version":"1.0.0","compose":{"include":[{"system":"http://snomed.info/sct"}]}}"#,
        )
        .unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "PlanDefinition-test".to_string(),
            json!({
                "resourceType": "PlanDefinition",
                "id": "test",
                "url": "http://example.org/fhir/PlanDefinition/test",
                "library": ["http://dep.org/fhir/ValueSet/test-codes"]
            }),
        );

        let mut deps = HashMap::new();
        deps.insert("dep.pkg".to_string(), "1.0.0".to_string());

        let mut ctx = make_ctx(&tmp, resources, deps);
        ctx.config.link.packages_dir = Some(packages_dir.to_string_lossy().to_string());

        ResolveDependenciesProcessor.run(&mut ctx).unwrap();

        assert!(
            ctx.resources.contains_key("ValueSet-test-codes"),
            "ValueSet should be resolved from dependency package"
        );
    }

    #[test]
    fn skips_already_present_resources() {
        let tmp = TempDir::new().unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-local".to_string(),
            json!({
                "resourceType": "ValueSet",
                "id": "local",
                "url": "http://example.org/fhir/ValueSet/local"
            }),
        );
        resources.insert(
            "PlanDefinition-test".to_string(),
            json!({
                "resourceType": "PlanDefinition",
                "id": "test",
                "url": "http://example.org/fhir/PlanDefinition/test",
                "library": ["http://example.org/fhir/ValueSet/local"]
            }),
        );

        let ctx = make_ctx(&tmp, resources, HashMap::new());

        // All canonicals already in resources — nothing to resolve.
        let count = ctx.resources.len();
        let mut ctx = ctx;
        ResolveDependenciesProcessor.run(&mut ctx).unwrap();
        assert_eq!(ctx.resources.len(), count);
    }

    #[test]
    fn resource_stem_uses_id_or_name() {
        assert_eq!(
            resource_stem(&json!({"resourceType": "ValueSet", "id": "foo"})),
            "ValueSet-foo"
        );
        assert_eq!(
            resource_stem(&json!({"resourceType": "ValueSet", "name": "MyVS"})),
            "ValueSet-MyVS"
        );
        assert_eq!(resource_stem(&json!({"id": "foo"})), "Unknown-foo");
    }
}
