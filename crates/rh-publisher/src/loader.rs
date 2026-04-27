//! Source directory scanner — loads FHIR resources and discovers markdown files.

use crate::{
    config::PublisherConfig,
    context::PublishContext,
    manifest::PackageJson,
    PublisherError, Result,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::warn;

const PACKAGE_JSON: &str = "package.json";
const IG_RESOURCE_TYPE: &str = "ImplementationGuide";
const PUBLISHER_TOML: &str = "publisher.toml";

/// Scan `source_dir`, load `package.json` and `publisher.toml`, populate a
/// `PublishContext` with the discovered FHIR resources and markdown files.
///
/// # Errors
/// - `PublisherError::MissingFile` if `package.json` is absent.
/// - `PublisherError::MissingFile` if no `ImplementationGuide` resource is found.
pub fn load_source_dir(source_dir: &Path, output_dir: PathBuf) -> Result<PublishContext> {
    let package_json = load_package_json(source_dir)?;
    let config = load_publisher_config(source_dir)?;

    let mut ctx = PublishContext::new(
        source_dir.to_path_buf(),
        output_dir,
        package_json,
        config,
    );

    let (resources, standalone_md) = scan_resources(source_dir)?;
    ctx.resources = resources;
    ctx.standalone_markdown = standalone_md;

    ensure_implementation_guide(&ctx)?;

    Ok(ctx)
}

fn load_package_json(source_dir: &Path) -> Result<PackageJson> {
    let path = source_dir.join(PACKAGE_JSON);
    if !path.exists() {
        return Err(PublisherError::MissingFile(PACKAGE_JSON.to_string()));
    }
    let text = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&text)?)
}

fn load_publisher_config(source_dir: &Path) -> Result<PublisherConfig> {
    let path = source_dir.join(PUBLISHER_TOML);
    if path.exists() {
        let text = std::fs::read_to_string(&path)?;
        PublisherConfig::from_toml_str(&text)
    } else {
        Ok(PublisherConfig::default())
    }
}

/// Walk `source_dir`, loading `*.json` FHIR resources into a map and
/// partitioning `*.md` files into resource-matched and standalone lists.
///
/// Returns `(resource_map, standalone_markdown)` where the resource map is
/// keyed by the file's stem (without extension), e.g. `"StructureDefinition-foo"`.
fn scan_resources(
    source_dir: &Path,
) -> Result<(HashMap<String, Value>, Vec<PathBuf>)> {
    let mut resources: HashMap<String, Value> = HashMap::new();
    let mut md_stems: HashMap<String, PathBuf> = HashMap::new();

    for entry in std::fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        match ext {
            "json" if stem != "package" && stem != "fhir-lock" => {
                match load_json_resource(&path) {
                    Ok(value) => {
                        resources.insert(stem.to_string(), value);
                    }
                    Err(e) => {
                        warn!("Skipping {}: {}", path.display(), e);
                    }
                }
            }
            "md" => {
                md_stems.insert(stem.to_string(), path.clone());
            }
            _ => {}
        }
    }

    // Partition markdown files: matched (have a corresponding JSON resource) vs standalone.
    let mut standalone_markdown: Vec<PathBuf> = Vec::new();
    for (stem, md_path) in md_stems {
        if !resources.contains_key(&stem) {
            standalone_markdown.push(md_path);
        }
        // Matched markdown files are handled by the narrative module later.
    }

    Ok((resources, standalone_markdown))
}

fn load_json_resource(path: &Path) -> Result<Value> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

fn ensure_implementation_guide(ctx: &PublishContext) -> Result<()> {
    let has_ig = ctx.resources.values().any(|v| {
        v.get("resourceType")
            .and_then(|rt| rt.as_str())
            .map_or(false, |rt| rt == IG_RESOURCE_TYPE)
    });

    if !has_ig {
        return Err(PublisherError::MissingFile(
            "ImplementationGuide resource (e.g. ImplementationGuide.json)".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_file(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).unwrap();
    }

    fn minimal_ig() -> &'static str {
        r#"{"resourceType":"ImplementationGuide","id":"my-ig","version":"1.0.0","packageId":"example.pkg","url":"http://example.org/fhir","fhirVersion":["4.0.1"],"status":"draft"}"#
    }

    fn minimal_package_json() -> &'static str {
        r#"{"name":"example.pkg","version":"1.0.0","fhirVersions":["4.0.1"]}"#
    }

    #[test]
    fn fails_when_package_json_absent() {
        let dir = TempDir::new().unwrap();
        let result = load_source_dir(dir.path(), dir.path().join("output"));
        assert!(matches!(result, Err(PublisherError::MissingFile(f)) if f.contains("package.json")));
    }

    #[test]
    fn fails_when_no_implementation_guide() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(
            dir.path(),
            "ValueSet-foo.json",
            r#"{"resourceType":"ValueSet","id":"foo"}"#,
        );
        let result = load_source_dir(dir.path(), dir.path().join("output"));
        assert!(matches!(result, Err(PublisherError::MissingFile(f)) if f.contains("ImplementationGuide")));
    }

    #[test]
    fn loads_resources_and_partitions_markdown() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());
        write_file(
            dir.path(),
            "StructureDefinition-foo.json",
            r#"{"resourceType":"StructureDefinition","id":"foo"}"#,
        );
        // Matched markdown (has a JSON counterpart)
        write_file(dir.path(), "StructureDefinition-foo.md", "# Foo narrative");
        // Standalone markdown (no JSON counterpart)
        write_file(dir.path(), "overview.md", "# Overview");

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();

        assert!(ctx.resources.contains_key("ImplementationGuide"));
        assert!(ctx.resources.contains_key("StructureDefinition-foo"));
        assert_eq!(ctx.standalone_markdown.len(), 1);
        assert!(ctx
            .standalone_markdown[0]
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("overview"));
    }

    #[test]
    fn skips_package_json_from_resource_map() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(!ctx.resources.contains_key("package"));
    }

    #[test]
    fn uses_default_config_when_publisher_toml_absent() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(ctx.config.hooks.before_build.is_empty());
    }
}
