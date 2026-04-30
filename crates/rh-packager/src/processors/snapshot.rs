//! `snapshot` hook processor — generates missing StructureDefinition snapshots.
//!
//! Finds StructureDefinition resources in the resource map that are missing
//! `snapshot.element`, loads dependency packages to resolve base definitions,
//! and runs `rh-foundation`'s `SnapshotGenerator` to fill them in.

use crate::{
    context::PublishContext, hooks::HookProcessor, utils::resolve_packages_dir, PublisherError,
    Result,
};
use rh_foundation::snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use serde_json::Value;
use tracing::info;

/// Hook processor that generates snapshot elements for StructureDefinitions.
pub struct SnapshotProcessor;

impl HookProcessor for SnapshotProcessor {
    fn name(&self) -> &str {
        "snapshot"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let packages_dir = resolve_packages_dir(None, ctx.config.packages_dir.as_deref());

        let mut generator = SnapshotGenerator::new();

        // Load SDs from dependency packages.
        for (pkg_name, pkg_version) in &ctx.package_json.dependencies {
            let pkg_dir = packages_dir.join(format!("{pkg_name}#{pkg_version}"));
            if !pkg_dir.exists() {
                return Err(PublisherError::MissingPackage(format!(
                    "{pkg_name}#{pkg_version}"
                )));
            }
            match StructureDefinitionLoader::load_from_package(pkg_name, pkg_version, &packages_dir)
            {
                Ok(sds) => generator.load_structure_definitions(sds),
                Err(e) => {
                    tracing::warn!("Could not load SDs from {pkg_name}#{pkg_version}: {e}");
                }
            }
        }

        // Also pre-load SDs from the current source (so profiles can reference each other).
        let source_sds: Vec<_> = ctx
            .resources
            .values()
            .filter(|v| {
                v.get("resourceType").and_then(|r| r.as_str()) == Some("StructureDefinition")
            })
            .filter_map(|v| {
                serde_json::from_value::<rh_foundation::snapshot::types::StructureDefinition>(
                    v.clone(),
                )
                .ok()
            })
            .collect();
        generator.load_structure_definitions(source_sds);

        // Generate snapshots for SDs that are missing them.
        let sd_keys: Vec<String> = ctx
            .resources
            .iter()
            .filter(|(_, v)| {
                v.get("resourceType").and_then(|r| r.as_str()) == Some("StructureDefinition")
                    && !has_snapshot(v)
            })
            .map(|(k, _)| k.clone())
            .collect();

        for key in sd_keys {
            let url = ctx.resources[&key]
                .get("url")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    PublisherError::IgSync(format!(
                        "StructureDefinition '{key}' is missing a 'url' field"
                    ))
                })?
                .to_string();

            let snapshot = generator.generate_snapshot(&url).map_err(|e| {
                PublisherError::Other(anyhow::anyhow!(
                    "Snapshot generation failed for '{url}': {e}"
                ))
            })?;

            // Inject snapshot.element into the resource value.
            if let Some(resource) = ctx.resources.get_mut(&key) {
                let elements_json = serde_json::to_value(&snapshot.element)?;
                resource["snapshot"] = serde_json::json!({ "element": elements_json });
                info!("Generated snapshot for {url}");
            }
        }

        Ok(())
    }
}

fn has_snapshot(resource: &Value) -> bool {
    resource
        .get("snapshot")
        .and_then(|s| s.get("element"))
        .and_then(|e| e.as_array())
        .is_some_and(|arr| !arr.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(resources: HashMap<String, Value>) -> PublishContext {
        let tmp = TempDir::new().unwrap();
        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: PackageJson {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: HashMap::new(),
                url: None,
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
    fn has_snapshot_returns_true_when_elements_present() {
        let resource = json!({
            "resourceType": "StructureDefinition",
            "snapshot": { "element": [{"id": "foo", "path": "foo"}] }
        });
        assert!(has_snapshot(&resource));
    }

    #[test]
    fn has_snapshot_returns_false_when_absent() {
        let resource = json!({"resourceType": "StructureDefinition", "id": "foo"});
        assert!(!has_snapshot(&resource));
    }

    #[test]
    fn has_snapshot_returns_false_when_empty_array() {
        let resource = json!({"resourceType": "StructureDefinition", "snapshot": {"element": []}});
        assert!(!has_snapshot(&resource));
    }

    #[test]
    fn processor_skips_already_snapshotted_resources() {
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({
                "resourceType": "StructureDefinition",
                "id": "foo",
                "url": "http://example.org/fhir/StructureDefinition/foo",
                "snapshot": {"element": [{"id": "foo", "path": "foo"}]}
            }),
        );
        // No dependency packages needed because the SD is already snapshotted.
        let mut ctx = make_ctx(resources);
        // Processor should complete without error (nothing to do).
        let processor = SnapshotProcessor;
        processor.run(&mut ctx).unwrap();
        // Snapshot should still be present.
        let resource = ctx.resources.get("StructureDefinition-foo").unwrap();
        assert!(has_snapshot(resource));
    }
}
