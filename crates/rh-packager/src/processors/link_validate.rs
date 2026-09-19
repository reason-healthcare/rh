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
use base64::{engine::general_purpose::STANDARD, Engine as _};
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
            for url in lock::collect_canonical_references(resource) {
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

        // 4. Check all Libraries have valid, inline ELM attachments.
        for (key, resource) in &ctx.resources {
            if resource.get("resourceType").and_then(|r| r.as_str()) == Some("Library") {
                if let Err(message) = validate_elm(resource) {
                    errors.push(format!("Library {key} has invalid ELM: {message}"));
                }
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
        .any(|resource| lock::resource_matches_canonical(resource, url))
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

/// Require at least one inline, base64-encoded, parseable ELM JSON attachment.
fn validate_elm(lib: &Value) -> std::result::Result<(), String> {
    let elm_attachments: Vec<&Value> = lib
        .get("content")
        .and_then(|c| c.as_array())
        .into_iter()
        .flatten()
        .filter(|attachment| {
            attachment.get("contentType").and_then(Value::as_str) == Some("application/elm+json")
        })
        .collect();

    if elm_attachments.is_empty() {
        return Err("missing application/elm+json attachment".to_string());
    }

    for attachment in elm_attachments {
        decode_elm_attachment(attachment)?;
    }

    Ok(())
}

fn decode_elm_attachment(attachment: &Value) -> std::result::Result<rh_cql::elm::Library, String> {
    let encoded = attachment
        .get("data")
        .and_then(Value::as_str)
        .filter(|data| !data.is_empty())
        .ok_or_else(|| "ELM attachment must contain non-empty inline data".to_string())?;
    let decoded = STANDARD
        .decode(encoded)
        .map_err(|error| format!("ELM attachment is not valid base64: {error}"))?;
    let document: Value = serde_json::from_slice(&decoded)
        .map_err(|error| format!("ELM attachment is not valid JSON: {error}"))?;
    let library = document.get("library").cloned().unwrap_or(document);
    serde_json::from_value(library)
        .map_err(|error| format!("ELM attachment is not a valid ELM library: {error}"))
}

/// Check if any Library's CQL content imports FHIRHelpers.
fn uses_fhir_helpers(ctx: &PublishContext) -> bool {
    for resource in ctx.resources.values() {
        if resource.get("resourceType").and_then(|r| r.as_str()) != Some("Library") {
            continue;
        }
        if let Some(content) = resource.get("content").and_then(Value::as_array) {
            for attachment in content {
                let content_type = attachment.get("contentType").and_then(Value::as_str);
                let data = attachment.get("data").and_then(Value::as_str);
                if content_type == Some("text/cql")
                    && data
                        .and_then(|encoded| STANDARD.decode(encoded).ok())
                        .and_then(|decoded| String::from_utf8(decoded).ok())
                        .is_some_and(|cql| cql.lines().any(cql_line_imports_fhir_helpers))
                {
                    return true;
                }
                if content_type == Some("application/elm+json")
                    && decode_elm_attachment(attachment)
                        .ok()
                        .and_then(|elm| elm.includes)
                        .is_some_and(|includes| {
                            includes.defs.iter().any(|include| {
                                include.path.as_deref().is_some_and(is_fhir_helpers_name)
                            })
                        })
                {
                    return true;
                }
            }
        }
        // Check relatedArtifact for FHIRHelpers dependency.
        if let Some(related) = resource.get("relatedArtifact").and_then(|r| r.as_array()) {
            for art in related {
                let display = art.get("display").and_then(|d| d.as_str()).unwrap_or("");
                let url = art
                    .get("resource")
                    .or_else(|| art.get("url"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if display.contains("FHIRHelpers") || url.contains("FHIRHelpers") {
                    return true;
                }
            }
        }
    }
    false
}

fn cql_line_imports_fhir_helpers(line: &str) -> bool {
    let line = line.trim_start();
    line.starts_with("include ")
        && line
            .split_ascii_whitespace()
            .nth(1)
            .is_some_and(is_fhir_helpers_name)
}

fn is_fhir_helpers_name(name: &str) -> bool {
    name.split(['.', '/', '|'])
        .any(|segment| segment == "FHIRHelpers")
}

/// Check if a FHIRHelpers Library resource is present.
fn has_fhir_helpers(ctx: &PublishContext) -> bool {
    ctx.resources.values().any(|v| {
        v.get("resourceType").and_then(|r| r.as_str()) == Some("Library")
            && (v
                .get("name")
                .and_then(|n| n.as_str())
                .is_some_and(is_fhir_helpers_name)
                || v.get("id")
                    .and_then(|i| i.as_str())
                    .is_some_and(is_fhir_helpers_name)
                || v.get("url")
                    .and_then(Value::as_str)
                    .is_some_and(is_fhir_helpers_name))
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
                    if let Some(url) = art
                        .get("resource")
                        .or_else(|| art.get("url"))
                        .and_then(Value::as_str)
                    {
                        // Try to find the library name from the URL.
                        let (canonical, _) = lock::canonical_url_and_version(url);
                        if let Some(dep_name) = canonical.rsplit('/').next() {
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

    fn valid_elm_data() -> String {
        let library = serde_json::json!({"library": {"identifier": {"id": "TestLogic"}}});
        STANDARD.encode(serde_json::to_vec(&library).unwrap())
    }

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
                    {"contentType": "application/elm+json", "data": valid_elm_data()}
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
    fn fails_on_unresolved_versioned_canonical() {
        let mut resources = HashMap::new();
        resources.insert(
            "PlanDefinition-test".to_string(),
            json!({
                "resourceType": "PlanDefinition",
                "id": "test",
                "url": "http://example.org/fhir/PlanDefinition/test",
                "library": ["http://example.org/fhir/Library/TestLogic|2.0.0"]
            }),
        );
        resources.insert(
            "Library-TestLogic".to_string(),
            json!({
                "resourceType": "Library",
                "id": "TestLogic",
                "url": "http://example.org/fhir/Library/TestLogic",
                "version": "1.0.0",
                "content": [
                    {"contentType": "application/elm+json", "data": valid_elm_data()}
                ]
            }),
        );

        let mut ctx = make_ctx(resources);
        let err = LinkValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::LinkValidation(ref errs) if errs.iter().any(|e| e.contains("TestLogic|2.0.0")))
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
    fn elm_validation_checks_inline_content() {
        assert!(validate_elm(
            &json!({"content": [{"contentType": "application/elm+json", "data": valid_elm_data()}]})
        )
        .is_ok());
        assert!(validate_elm(
            &json!({"content": [{"contentType": "application/elm+json", "data": ""}]})
        )
        .is_err());
        assert!(validate_elm(
            &json!({"content": [{"contentType": "application/elm+json", "data": "bm90IGpzb24="}]})
        )
        .is_err());
        assert!(
            validate_elm(&json!({"content": [{"contentType": "text/cql", "data": ""}]})).is_err()
        );
        assert!(validate_elm(&json!({})).is_err());
    }

    #[test]
    fn detects_fhir_helpers_import_from_inline_cql() {
        let cql = STANDARD.encode(
            "library Test version '1.0.0'\ninclude FHIRHelpers version '4.0.1' called FHIRHelpers",
        );
        let resources = [(
            "Library-Test".to_string(),
            json!({
                "resourceType": "Library",
                "content": [{"contentType": "text/cql", "data": cql}]
            }),
        )]
        .into_iter()
        .collect();
        assert!(uses_fhir_helpers(&make_ctx(resources)));
    }
}
