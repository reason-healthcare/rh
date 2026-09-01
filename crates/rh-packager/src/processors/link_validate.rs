//! `link-validate` hook processor — verifies the resource set is self-contained
//! and executable.
//!
//! Checks:
//!   1. All canonical references resolve to a resource in the map
//!   2. All ValueSets have expansions
//!   3. All StructureDefinitions have snapshots
//!   4. All Libraries have ELM attachments
//!   5. FHIRHelpers is present if any Library references it
//!   6. No circular Library dependencies

use crate::{context::PublishContext, hooks::HookProcessor, lock, PublisherError, Result};
use serde_json::Value;
use std::collections::HashSet;
use tracing::info;

/// Hook processor that validates executable bundle completeness.
pub struct LinkValidateProcessor;

impl HookProcessor for LinkValidateProcessor {
    fn name(&self) -> &str {
        "link-validate"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let mut errors: Vec<String> = Vec::new();

        // 1. Check all canonical references resolve.
        for (key, resource) in &ctx.resources {
            for url in lock::collect_canonicals(resource) {
                if lock::is_excluded(&url) {
                    continue;
                }
                if !canonical_in_resources(ctx, &url) {
                    errors.push(format!("Unresolved canonical reference: {url} (in {key})"));
                }
            }
        }

        // 2. Check all ValueSets are expanded.
        for (key, resource) in &ctx.resources {
            if resource.get("resourceType").and_then(|r| r.as_str()) == Some("ValueSet")
                && !has_expansion(resource)
            {
                errors.push(format!("Unexpanded ValueSet: {key}"));
            }
        }

        // 3. Check all StructureDefinitions have snapshots.
        for (key, resource) in &ctx.resources {
            if resource.get("resourceType").and_then(|r| r.as_str()) == Some("StructureDefinition")
                && !has_snapshot(resource)
            {
                errors.push(format!("Missing StructureDefinition snapshot: {key}"));
            }
        }

        // 4. Check all Libraries have ELM attachments.
        for (key, resource) in &ctx.resources {
            if resource.get("resourceType").and_then(|r| r.as_str()) == Some("Library")
                && !has_elm(resource)
            {
                errors.push(format!("Library missing ELM attachment: {key}"));
            }
        }

        // 5. FHIRHelpers check.
        if uses_fhir_helpers(ctx) && !has_fhir_helpers(ctx) {
            errors.push(
                "FHIRHelpers library is referenced by CQL but not present in the bundle"
                    .to_string(),
            );
        }

        // 6. Circular Library dependency check.
        if let Some(cycle) = detect_circular_library_deps(ctx) {
            errors.push(format!(
                "Circular Library dependency: {}",
                cycle.join(" -> ")
            ));
        }

        if !errors.is_empty() {
            return Err(PublisherError::LinkValidation(errors));
        }

        info!(
            "link-validate: bundle is self-contained ({} resources)",
            ctx.resources.len()
        );
        Ok(())
    }
}

/// Check if a canonical URL is present as a resource in the context.
fn canonical_in_resources(ctx: &PublishContext, url: &str) -> bool {
    ctx.resources
        .values()
        .any(|v| v.get("url").and_then(|u| u.as_str()) == Some(url))
}

/// Check if a ValueSet has a non-empty expansion.
fn has_expansion(vs: &Value) -> bool {
    vs.get("expansion")
        .and_then(|e| e.get("contains"))
        .and_then(|c| c.as_array())
        .is_some_and(|arr| !arr.is_empty())
}

/// Check if a StructureDefinition has a snapshot.
fn has_snapshot(sd: &Value) -> bool {
    sd.get("snapshot")
        .and_then(|s| s.get("element"))
        .and_then(|e| e.as_array())
        .is_some_and(|arr| !arr.is_empty())
}

/// Check if a Library has an ELM attachment.
fn has_elm(lib: &Value) -> bool {
    lib.get("content")
        .and_then(|c| c.as_array())
        .is_some_and(|content| {
            content.iter().any(|attachment| {
                attachment
                    .get("contentType")
                    .and_then(|t| t.as_str())
                    .is_some_and(|t| t == "application/elm+json")
            })
        })
}

/// Check if any Library's CQL content imports FHIRHelpers.
fn uses_fhir_helpers(ctx: &PublishContext) -> bool {
    for resource in ctx.resources.values() {
        if resource.get("resourceType").and_then(|r| r.as_str()) != Some("Library") {
            continue;
        }
        if let Some(content) = resource.get("content").and_then(|c| c.as_array()) {
            for attachment in content {
                if attachment
                    .get("contentType")
                    .and_then(|t| t.as_str())
                    .is_some_and(|t| t == "text/cql")
                {
                    // Check if CQL source references FHIRHelpers.
                    // The data is base64-encoded; for now check relatedArtifact
                    // which is more reliable.
                }
            }
        }
        // Check relatedArtifact for FHIRHelpers dependency.
        if let Some(related) = resource.get("relatedArtifact").and_then(|r| r.as_array()) {
            for art in related {
                let display = art.get("display").and_then(|d| d.as_str()).unwrap_or("");
                let url = art.get("url").and_then(|u| u.as_str()).unwrap_or("");
                if display.contains("FHIRHelpers") || url.contains("FHIRHelpers") {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if a FHIRHelpers Library resource is present.
fn has_fhir_helpers(ctx: &PublishContext) -> bool {
    ctx.resources.values().any(|v| {
        v.get("resourceType").and_then(|r| r.as_str()) == Some("Library")
            && (v
                .get("name")
                .and_then(|n| n.as_str())
                .is_some_and(|n| n == "FHIRHelpers")
                || v.get("id")
                    .and_then(|i| i.as_str())
                    .is_some_and(|i| i == "FHIRHelpers"))
    })
}

/// Detect circular dependencies among Library resources.
///
/// Returns `Some(cycle)` with the list of library names forming the cycle,
/// or `None` if no cycles are found.
fn detect_circular_library_deps(ctx: &PublishContext) -> Option<Vec<String>> {
    // Build adjacency list: library name → list of dependency library names.
    let mut adj: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for resource in ctx.resources.values() {
        if resource.get("resourceType").and_then(|r| r.as_str()) != Some("Library") {
            continue;
        }
        let name = resource
            .get("name")
            .or_else(|| resource.get("id"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }

        let mut deps = Vec::new();
        if let Some(related) = resource.get("relatedArtifact").and_then(|r| r.as_array()) {
            for art in related {
                if art.get("type").and_then(|t| t.as_str()) == Some("depends-on") {
                    if let Some(url) = art.get("url").and_then(|u| u.as_str()) {
                        // Try to find the library name from the URL.
                        if let Some(dep_name) = url.rsplit('/').next() {
                            deps.push(dep_name.to_string());
                        }
                    }
                }
            }
        }
        adj.insert(name, deps);
    }

    // DFS-based cycle detection.
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: HashSet<String> = HashSet::new();
    let mut path: Vec<String> = Vec::new();

    for node in adj.keys() {
        if !visited.contains(node) {
            if let Some(cycle) = dfs_cycle(node, &adj, &mut visited, &mut stack, &mut path) {
                return Some(cycle);
            }
        }
    }

    None
}

fn dfs_cycle(
    node: &str,
    adj: &std::collections::HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    visited.insert(node.to_string());
    stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = adj.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if let Some(cycle) = dfs_cycle(neighbor, adj, visited, stack, path) {
                    return Some(cycle);
                }
            } else if stack.contains(neighbor) {
                // Found a cycle — extract it from the path.
                let start = path.iter().position(|n| n == neighbor).unwrap_or(0);
                let mut cycle = path[start..].to_vec();
                cycle.push(neighbor.to_string());
                return Some(cycle);
            }
        }
    }

    stack.remove(node);
    path.pop();
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, context::PublishContext, hooks::HookProcessor,
        manifest::PackageJson,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(resources: HashMap<String, Value>) -> PublishContext {
        let tmp = TempDir::new().unwrap();
        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            input_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: PackageJson {
                name: "test.pkg".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: HashMap::new(),
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
    fn passes_when_all_deps_resolved() {
        let mut resources = HashMap::new();
        resources.insert(
            "PlanDefinition-test".to_string(),
            json!({
                "resourceType": "PlanDefinition",
                "id": "test",
                "url": "http://example.org/fhir/PlanDefinition/test",
                "library": ["http://example.org/fhir/Library/TestLogic"]
            }),
        );
        resources.insert(
            "Library-TestLogic".to_string(),
            json!({
                "resourceType": "Library",
                "id": "TestLogic",
                "url": "http://example.org/fhir/Library/TestLogic",
                "content": [
                    {"contentType": "text/cql", "data": ""},
                    {"contentType": "application/elm+json", "data": ""}
                ]
            }),
        );

        let mut ctx = make_ctx(resources);
        LinkValidateProcessor.run(&mut ctx).unwrap();
    }

    #[test]
    fn fails_on_unresolved_canonical() {
        let mut resources = HashMap::new();
        resources.insert(
            "PlanDefinition-test".to_string(),
            json!({
                "resourceType": "PlanDefinition",
                "id": "test",
                "url": "http://example.org/fhir/PlanDefinition/test",
                "library": ["http://missing.org/Library/DoesNotExist"]
            }),
        );

        let mut ctx = make_ctx(resources);
        let err = LinkValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::LinkValidation(ref errs) if errs.iter().any(|e| e.contains("Unresolved canonical reference")))
        );
    }

    #[test]
    fn fails_on_unexpanded_valueset() {
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-test".to_string(),
            json!({
                "resourceType": "ValueSet",
                "id": "test",
                "url": "http://example.org/fhir/ValueSet/test",
                "compose": {"include": [{"system": "http://snomed.info/sct"}]}
            }),
        );

        let mut ctx = make_ctx(resources);
        let err = LinkValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::LinkValidation(ref errs) if errs.iter().any(|e| e.contains("Unexpanded ValueSet")))
        );
    }

    #[test]
    fn fails_on_missing_snapshot() {
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "id": "foo",
                "url": "http://example.org/fhir/StructureDefinition/foo",
                "differential": {"element": []}
            }),
        );

        let mut ctx = make_ctx(resources);
        let err = LinkValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::LinkValidation(ref errs) if errs.iter().any(|e| e.contains("Missing StructureDefinition snapshot")))
        );
    }

    #[test]
    fn fails_on_missing_elm() {
        let mut resources = HashMap::new();
        resources.insert(
            "Library-TestLogic".to_string(),
            json!({
                "resourceType": "Library",
                "id": "TestLogic",
                "url": "http://example.org/fhir/Library/TestLogic",
                "content": [
                    {"contentType": "text/cql", "data": ""}
                ]
            }),
        );

        let mut ctx = make_ctx(resources);
        let err = LinkValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::LinkValidation(ref errs) if errs.iter().any(|e| e.contains("ELM")))
        );
    }

    #[test]
    fn has_expansion_checks() {
        assert!(has_expansion(
            &json!({"expansion": {"contains": [{"code": "1"}]}})
        ));
        assert!(!has_expansion(&json!({"expansion": {"contains": []}})));
        assert!(!has_expansion(&json!({})));
    }

    #[test]
    fn has_snapshot_checks() {
        assert!(has_snapshot(
            &json!({"snapshot": {"element": [{"id": "foo"}]}})
        ));
        assert!(!has_snapshot(&json!({"snapshot": {"element": []}})));
        assert!(!has_snapshot(&json!({})));
    }

    #[test]
    fn has_elm_checks() {
        assert!(has_elm(
            &json!({"content": [{"contentType": "application/elm+json", "data": ""}]})
        ));
        assert!(!has_elm(
            &json!({"content": [{"contentType": "text/cql", "data": ""}]})
        ));
        assert!(!has_elm(&json!({})));
    }
}
