//! Source directory scanner — loads FHIR resources and discovers markdown files.

use crate::{
    config::PublisherConfig, context::PublishContext, manifest::PackageJson, PublisherError, Result,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::warn;

const PACKAGE_JSON: &str = "package.json";
const IG_RESOURCE_TYPE: &str = "ImplementationGuide";
const PUBLISHER_TOML: &str = "packager.toml";

/// Scanned source resources: (definitional_resources, example_resources, standalone_markdown).
type ScannedResources = (HashMap<String, Value>, HashMap<String, Value>, Vec<PathBuf>);

/// Scan `source_dir`, load `package.json` and `packager.toml`, populate a
/// `PublishContext` with the discovered FHIR resources and markdown files.
///
/// # Errors
/// - `PublisherError::MissingFile` if `package.json` is absent.
/// - `PublisherError::MissingFile` if no `ImplementationGuide` resource is found.
pub fn load_source_dir(source_dir: &Path, output_dir: PathBuf) -> Result<PublishContext> {
    let package_json = load_package_json(source_dir)?;
    let config = load_publisher_config(source_dir)?;

    let mut ctx = PublishContext::new(source_dir.to_path_buf(), output_dir, package_json, config);

    let (resources, examples, standalone_md) = scan_resources(source_dir)?;
    ctx.resources = resources;
    ctx.examples = examples;
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

/// Walk `source_dir` recursively, loading `*.json` FHIR resources into definitional
/// and example maps, and partitioning `*.md` files into resource-matched and standalone lists.
///
/// Resources found under an `examples/` subdirectory (at any depth) are placed in the
/// examples map and will be written to `package/examples/` in the output. All other
/// resources are definitional and go into `package/` flat.
///
/// JSON files inside a `docs/` subdirectory are ignored — `docs/` is reserved for
/// standalone narrative markdown (e.g. overview pages) that are copied to `package/other/`.
///
/// Returns `(resources, examples, standalone_markdown)`.
fn scan_resources(source_dir: &Path) -> Result<ScannedResources> {
    let mut resources: HashMap<String, Value> = HashMap::new();
    let mut examples: HashMap<String, Value> = HashMap::new();
    let mut md_stems: HashMap<String, PathBuf> = HashMap::new();

    scan_dir(
        source_dir,
        false,
        false,
        &mut resources,
        &mut examples,
        &mut md_stems,
    )?;

    // Partition markdown files: matched (have a corresponding JSON resource) vs standalone.
    let mut standalone_markdown: Vec<PathBuf> = Vec::new();
    for (stem, md_path) in md_stems {
        if !resources.contains_key(&stem) && !examples.contains_key(&stem) {
            standalone_markdown.push(md_path);
        }
        // Matched markdown files are handled by the narrative module later.
    }

    Ok((resources, examples, standalone_markdown))
}

/// Returns true if a directory entry should be skipped during source scanning.
fn is_skip_dir(name: &str) -> bool {
    name.starts_with('.') || matches!(name, "output" | "target" | "node_modules")
}

/// Recursively walk `dir`, collecting FHIR JSON resources and markdown files.
///
/// `in_examples` is true when scanning a path that descends from an `examples/` directory,
/// which routes resources to the `examples` map instead of `resources`.
///
/// `in_docs` is true when scanning a path that descends from a `docs/` directory.
/// JSON files in `docs/` are ignored (the directory is for standalone narrative markdown).
fn scan_dir(
    dir: &Path,
    in_examples: bool,
    in_docs: bool,
    resources: &mut HashMap<String, Value>,
    examples: &mut HashMap<String, Value>,
    md_stems: &mut HashMap<String, PathBuf>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if path.is_dir() {
            if is_skip_dir(name) {
                continue;
            }
            let next_in_examples = in_examples || name == "examples";
            let next_in_docs = in_docs || name == "docs";
            scan_dir(&path, next_in_examples, next_in_docs, resources, examples, md_stems)?;
            continue;
        }

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
            "json" if !in_docs && stem != "package" && stem != "fhir-lock" && !stem.starts_with('.') => {
                match load_json_resource(&path) {
                    Ok(value) => {
                        if in_examples {
                            examples.insert(stem.to_string(), value);
                        } else {
                            resources.insert(stem.to_string(), value);
                        }
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
    Ok(())
}

fn load_json_resource(path: &Path) -> Result<Value> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

fn ensure_implementation_guide(ctx: &PublishContext) -> Result<()> {
    let has_ig = ctx.resources.values().any(|v| {
        v.get("resourceType")
            .and_then(|rt| rt.as_str())
            .is_some_and(|rt| rt == IG_RESOURCE_TYPE)
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
        assert!(
            matches!(result, Err(PublisherError::MissingFile(f)) if f.contains("package.json"))
        );
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
        assert!(
            matches!(result, Err(PublisherError::MissingFile(f)) if f.contains("ImplementationGuide"))
        );
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
        // Standalone markdown in docs/
        let docs_dir = dir.path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        write_file(&docs_dir, "overview.md", "# Overview");

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();

        assert!(ctx.resources.contains_key("ImplementationGuide"));
        assert!(ctx.resources.contains_key("StructureDefinition-foo"));
        assert_eq!(ctx.standalone_markdown.len(), 1);
        assert!(ctx.standalone_markdown[0]
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("overview"));
    }

    #[test]
    fn docs_dir_markdown_becomes_standalone() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let docs_dir = dir.path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        write_file(&docs_dir, "introduction.md", "# Introduction");
        write_file(&docs_dir, "overview.md", "# Overview");

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert_eq!(ctx.standalone_markdown.len(), 2);
    }

    #[test]
    fn docs_dir_json_is_not_loaded_as_resource() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let docs_dir = dir.path().join("docs");
        fs::create_dir_all(&docs_dir).unwrap();
        // A JSON file in docs/ should be ignored — it's not a FHIR resource.
        write_file(
            &docs_dir,
            "ValueSet-stray.json",
            r#"{"resourceType":"ValueSet","id":"stray"}"#,
        );

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(!ctx.resources.contains_key("ValueSet-stray"));
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
    fn loads_resources_from_subdirectory() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let profiles_dir = dir.path().join("profiles");
        fs::create_dir_all(&profiles_dir).unwrap();
        write_file(
            &profiles_dir,
            "StructureDefinition-foo.json",
            r#"{"resourceType":"StructureDefinition","id":"foo"}"#,
        );

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(ctx.resources.contains_key("StructureDefinition-foo"));
        assert!(ctx.examples.is_empty());
    }

    #[test]
    fn routes_examples_subdir_to_examples_map() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let examples_dir = dir.path().join("examples");
        fs::create_dir_all(&examples_dir).unwrap();
        write_file(
            &examples_dir,
            "Patient-example.json",
            r#"{"resourceType":"Patient","id":"example"}"#,
        );

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(!ctx.resources.contains_key("Patient-example"));
        assert!(ctx.examples.contains_key("Patient-example"));
    }

    #[test]
    fn skips_output_and_hidden_directories() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        // Place a JSON file in output/ — should be ignored.
        let output_dir = dir.path().join("output");
        fs::create_dir_all(&output_dir).unwrap();
        write_file(
            &output_dir,
            "ValueSet-stray.json",
            r#"{"resourceType":"ValueSet","id":"stray"}"#,
        );

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(!ctx.resources.contains_key("ValueSet-stray"));
    }

    #[test]
    fn uses_default_config_when_packager_toml_absent() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "package.json", minimal_package_json());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(ctx.config.hooks.before_build.is_empty());
    }
}
