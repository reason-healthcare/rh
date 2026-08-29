//! `expand-valuesets` hook processor — pre-expands ValueSets for executable bundles.
//!
//! Finds ValueSet resources that have `compose` but no `expansion` (or an incomplete
//! one), and populates `expansion.contains` with the full code set.
//!
//! Terminology data sources (in priority order):
//!   1. Local terminology directory (`[link] terminology_dir`)
//!   2. Already-expanded ValueSets in the resource map (skip)
//!   3. FHIR terminology server (`[link] terminology_server`) — future, not yet implemented

use crate::{context::PublishContext, hooks::HookProcessor, Result};
use serde_json::Value;
use std::path::Path;
use tracing::{info, warn};

/// Hook processor that pre-expands ValueSets.
pub struct ExpandValueSetsProcessor;

impl HookProcessor for ExpandValueSetsProcessor {
    fn name(&self) -> &str {
        "expand-valuesets"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let terminology_dir = ctx.config.link.terminology_dir.as_deref();

        // Collect ValueSet keys that need expansion.
        let vs_keys: Vec<String> = ctx
            .resources
            .iter()
            .filter(|(_, v)| {
                v.get("resourceType").and_then(|r| r.as_str()) == Some("ValueSet")
                    && needs_expansion(v)
            })
            .map(|(k, _)| k.clone())
            .collect();

        if vs_keys.is_empty() {
            info!("expand-valuesets: no ValueSets need expansion");
            return Ok(());
        }

        let mut expanded_count = 0usize;

        for key in &vs_keys {
            let vs = ctx.resources[key].clone();
            let url = vs.get("url").and_then(|u| u.as_str()).unwrap_or("");

            // Try local terminology directory first.
            if let Some(dir) = terminology_dir {
                if let Some(expanded) = expand_from_dir(&vs, dir)? {
                    ctx.resources.insert(key.clone(), expanded);
                    expanded_count += 1;
                    info!("expand-valuesets: expanded {key} from terminology dir");
                    continue;
                }
            }

            // Try terminology server (future — not yet implemented).
            if ctx.config.link.terminology_server.is_some() {
                warn!(
                    "expand-valuesets: terminology_server expansion not yet implemented \
                     for {key} ({url})"
                );
            }

            // If the ValueSet has explicit concepts in compose.include, we can
            // build the expansion directly from those without external data.
            if let Some(expanded) = expand_from_compose(&vs) {
                ctx.resources.insert(key.clone(), expanded);
                expanded_count += 1;
                info!("expand-valuesets: expanded {key} from compose includes");
                continue;
            }

            warn!(
                "expand-valuesets: could not expand {key} ({url}) — \
                 no terminology source available"
            );
        }

        info!(
            "expand-valuesets: expanded {} of {} ValueSet(s)",
            expanded_count,
            vs_keys.len()
        );

        Ok(())
    }
}

/// Check if a ValueSet needs expansion (has compose but no/empty expansion).
fn needs_expansion(vs: &Value) -> bool {
    let has_compose = vs.get("compose").is_some();
    let has_expansion = vs
        .get("expansion")
        .and_then(|e| e.get("contains"))
        .and_then(|c| c.as_array())
        .is_some_and(|arr| !arr.is_empty());

    has_compose && !has_expansion
}

/// Try to expand a ValueSet from a local terminology directory.
///
/// Looks for a file matching the ValueSet URL or ID in the directory.
fn expand_from_dir(vs: &Value, dir: &str) -> Result<Option<Value>> {
    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        return Ok(None);
    }

    // Try to find by URL (convert slashes to underscores for filename).
    let url = vs.get("url").and_then(|u| u.as_str()).unwrap_or("");
    let id = vs.get("id").and_then(|i| i.as_str()).unwrap_or("");

    // Candidate filenames: ValueSet-<id>.json, or URL-based.
    let candidates = [
        format!("ValueSet-{id}.json"),
        url.replace("http://", "")
            .replace("https://", "")
            .replace('/', "_")
            + ".json",
    ];

    for candidate in &candidates {
        let path = dir_path.join(candidate);
        if path.exists() {
            let text = std::fs::read_to_string(&path)?;
            let expanded_vs: Value = serde_json::from_str(&text)?;

            // Verify it's a ValueSet with an expansion.
            if expanded_vs.get("resourceType").and_then(|r| r.as_str()) == Some("ValueSet")
                && expanded_vs
                    .get("expansion")
                    .and_then(|e| e.get("contains"))
                    .and_then(|c| c.as_array())
                    .is_some()
            {
                // Merge the expansion into the original ValueSet, preserving
                // the original's url, id, name, etc.
                let mut result = vs.clone();
                result["expansion"] = expanded_vs["expansion"].clone();
                return Ok(Some(result));
            }
        }
    }

    Ok(None)
}

/// Build an expansion directly from `compose.include.concept` entries.
///
/// This works when the ValueSet explicitly lists all concepts (no filters,
/// no nested ValueSet references). The expansion is deterministic and needs
/// no external terminology data.
fn expand_from_compose(vs: &Value) -> Option<Value> {
    let compose = vs.get("compose")?;
    let includes = compose.get("include")?.as_array()?;

    let mut contains: Vec<Value> = Vec::new();

    for include in includes {
        // Skip if this include references other ValueSets or has filters
        // (those need external expansion).
        if include.get("valueSet").is_some() {
            return None;
        }
        if include.get("filter").is_some() {
            return None;
        }

        let system = include.get("system").and_then(|s| s.as_str())?;
        let version = include.get("version").and_then(|v| v.as_str());

        if let Some(concepts) = include.get("concept").and_then(|c| c.as_array()) {
            for concept in concepts {
                let code = concept.get("code").and_then(|c| c.as_str())?;
                let display = concept.get("display").and_then(|d| d.as_str());

                let mut entry = serde_json::json!({
                    "system": system,
                    "code": code,
                });

                if let Some(v) = version {
                    entry["version"] = Value::String(v.to_string());
                }
                if let Some(d) = display {
                    entry["display"] = Value::String(d.to_string());
                }

                contains.push(entry);
            }
        } else {
            // No explicit concepts — can't expand without a terminology server.
            return None;
        }
    }

    if contains.is_empty() {
        return None;
    }

    let mut result = vs.clone();
    result["expansion"] = serde_json::json!({
        "identifier": format!("urn:uuid:{}", uuid_like_id(vs)),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total": contains.len(),
        "offset": 0,
        "contains": contains
    });

    Some(result)
}

/// Generate a pseudo-unique identifier for the expansion from the ValueSet URL/id.
fn uuid_like_id(vs: &Value) -> String {
    let url = vs.get("url").and_then(|u| u.as_str()).unwrap_or("unknown");
    // Simple hash-like derivation — not a real UUID but sufficient for expansion identifier.
    format!(
        "{:x}",
        url.bytes()
            .fold(0u128, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u128))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{LinkConfig, PublisherConfig},
        context::PublishContext,
        hooks::HookProcessor,
        manifest::PackageJson,
    };
    use serde_json::json;
    use std::{collections::HashMap, fs};
    use tempfile::TempDir;

    fn make_ctx(
        tmp: &TempDir,
        resources: HashMap<String, Value>,
        link_config: LinkConfig,
    ) -> PublishContext {
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
            config: PublisherConfig {
                link: link_config,
                ..Default::default()
            },
            standalone_markdown: Vec::new(),
        }
    }

    #[test]
    fn needs_expansion_detects_compose_without_expansion() {
        let vs = json!({
            "resourceType": "ValueSet",
            "compose": {"include": [{"system": "http://snomed.info/sct"}]}
        });
        assert!(needs_expansion(&vs));
    }

    #[test]
    fn needs_expansion_false_when_expansion_present() {
        let vs = json!({
            "resourceType": "ValueSet",
            "compose": {"include": [{"system": "http://snomed.info/sct"}]},
            "expansion": {"contains": [{"system": "http://snomed.info/sct", "code": "123"}]}
        });
        assert!(!needs_expansion(&vs));
    }

    #[test]
    fn needs_expansion_false_when_no_compose() {
        let vs = json!({"resourceType": "ValueSet"});
        assert!(!needs_expansion(&vs));
    }

    #[test]
    fn expands_from_compose_with_explicit_concepts() {
        let tmp = TempDir::new().unwrap();
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-test".to_string(),
            json!({
                "resourceType": "ValueSet",
                "id": "test",
                "url": "http://example.org/fhir/ValueSet/test",
                "compose": {
                    "include": [{
                        "system": "http://snomed.info/sct",
                        "concept": [
                            {"code": "123", "display": "Foo"},
                            {"code": "456", "display": "Bar"}
                        ]
                    }]
                }
            }),
        );

        let mut ctx = make_ctx(&tmp, resources, LinkConfig::default());
        ExpandValueSetsProcessor.run(&mut ctx).unwrap();

        let vs = ctx.resources.get("ValueSet-test").unwrap();
        let contains = vs["expansion"]["contains"].as_array().unwrap();
        assert_eq!(contains.len(), 2);
        assert_eq!(contains[0]["code"], "123");
        assert_eq!(contains[0]["display"], "Foo");
        assert_eq!(contains[1]["code"], "456");
    }

    #[test]
    fn cannot_expand_compose_with_valueSet_references() {
        let vs = json!({
            "resourceType": "ValueSet",
            "compose": {
                "include": [{
                    "valueSet": ["http://other.org/ValueSet/nested"]
                }]
            }
        });
        assert!(expand_from_compose(&vs).is_none());
    }

    #[test]
    fn cannot_expand_compose_with_filters() {
        let vs = json!({
            "resourceType": "ValueSet",
            "compose": {
                "include": [{
                    "system": "http://snomed.info/sct",
                    "filter": [{"property": "concept", "op": "is-a", "value": "123"}]
                }]
            }
        });
        assert!(expand_from_compose(&vs).is_none());
    }

    #[test]
    fn expands_from_terminology_dir() {
        let tmp = TempDir::new().unwrap();
        let term_dir = tmp.path().join("terminology");
        fs::create_dir_all(&term_dir).unwrap();

        // Write a pre-expanded ValueSet to the terminology directory.
        fs::write(
            term_dir.join("ValueSet-test-codes.json"),
            r#"{"resourceType":"ValueSet","expansion":{"contains":[{"system":"http://snomed.info/sct","code":"123","display":"Foo"},{"system":"http://snomed.info/sct","code":"456","display":"Bar"}]}}"#,
        )
        .unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-test-codes".to_string(),
            json!({
                "resourceType": "ValueSet",
                "id": "test-codes",
                "url": "http://example.org/fhir/ValueSet/test-codes",
                "compose": {"include": [{"system": "http://snomed.info/sct"}]}
            }),
        );

        let mut link_config = LinkConfig::default();
        link_config.terminology_dir = Some(term_dir.to_string_lossy().to_string());

        let mut ctx = make_ctx(&tmp, resources, link_config);
        ExpandValueSetsProcessor.run(&mut ctx).unwrap();

        let vs = ctx.resources.get("ValueSet-test-codes").unwrap();
        let contains = vs["expansion"]["contains"].as_array().unwrap();
        assert_eq!(contains.len(), 2);
        // Original metadata preserved
        assert_eq!(vs["url"], "http://example.org/fhir/ValueSet/test-codes");
    }

    #[test]
    fn skips_already_expanded_valuesets() {
        let tmp = TempDir::new().unwrap();
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-already".to_string(),
            json!({
                "resourceType": "ValueSet",
                "id": "already",
                "url": "http://example.org/fhir/ValueSet/already",
                "compose": {"include": [{"system": "http://snomed.info/sct"}]},
                "expansion": {"contains": [{"system": "http://snomed.info/sct", "code": "123"}]}
            }),
        );

        let mut ctx = make_ctx(&tmp, resources, LinkConfig::default());
        ExpandValueSetsProcessor.run(&mut ctx).unwrap();

        // Should still have 1 code — not modified.
        let vs = ctx.resources.get("ValueSet-already").unwrap();
        let contains = vs["expansion"]["contains"].as_array().unwrap();
        assert_eq!(contains.len(), 1);
    }

    #[test]
    fn no_valuesets_is_ok() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp, HashMap::new(), LinkConfig::default());
        ExpandValueSetsProcessor.run(&mut ctx).unwrap();
    }
}
