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

const IG_RESOURCE_TYPE: &str = "ImplementationGuide";
const PUBLISHER_TOML: &str = "packager.toml";

/// Scanned source resources: (definitional_resources, example_resources, standalone_markdown).
type ScannedResources = (HashMap<String, Value>, HashMap<String, Value>, Vec<PathBuf>);

/// Scan `source_dir`, load `packager.toml`, populate a `PublishContext` with the
/// discovered FHIR resources and markdown files.
///
/// The `package.json` manifest is **generated** from `packager.toml` fields at load time;
/// it is not read from the source directory. If a `package.json` file is present in
/// `source_dir` it is silently ignored (though a warning is emitted).
///
/// ## Input Directory Resolution
///
/// If `source_dir/<config.input.dir>` (default: `source_dir/input/`) exists on disk,
/// structured layout is used and resources are scanned from there. Otherwise the legacy
/// flat-root layout is assumed for backward compatibility.
///
/// # Errors
/// - `PublisherError::MissingFile` if no `ImplementationGuide` resource is found.
pub fn load_source_dir(source_dir: &Path, output_dir: PathBuf) -> Result<PublishContext> {
    if source_dir.join("package.json").exists() {
        warn!(
            "Found package.json in source directory — it is no longer read. \
             All package metadata should be defined in packager.toml. \
             You can safely delete the source package.json."
        );
    }

    let config = load_publisher_config(source_dir)?;
    let package_json = PackageJson::from_config(&config);

    // Resolve the input directory: use structured layout if the input/ dir exists.
    let candidate = source_dir.join(&config.input.dir);
    let input_dir = if candidate.is_dir() {
        candidate
    } else {
        source_dir.to_path_buf()
    };

    let mut ctx = PublishContext::new(
        source_dir.to_path_buf(),
        input_dir.clone(),
        output_dir,
        package_json,
        config,
    );

    let (resources, examples, standalone_md) = scan_resources(
        &input_dir,
        &ctx.config.input.examples_dir,
        &ctx.config.input.docs_dir,
    )?;
    ctx.resources = resources;
    ctx.examples = examples;
    ctx.standalone_markdown = standalone_md;

    ensure_implementation_guide(&ctx)?;

    Ok(ctx)
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

/// Walk `scan_root` recursively, loading `*.json` FHIR resources into definitional
/// and example maps, and partitioning `*.md` files into resource-matched and standalone lists.
///
/// Resources found under `examples_dir_name` (at any depth) are placed in the examples map
/// and will be written to `package/examples/` in the output. All other resources are
/// definitional and go into `package/` flat.
///
/// JSON files inside `docs_dir_name` are ignored — that directory is reserved for
/// standalone narrative markdown (e.g. overview pages) copied to `package/other/`.
///
/// Returns `(resources, examples, standalone_markdown)`.
fn scan_resources(
    scan_root: &Path,
    examples_dir_name: &str,
    docs_dir_name: &str,
) -> Result<ScannedResources> {
    let mut resources: HashMap<String, Value> = HashMap::new();
    let mut examples: HashMap<String, Value> = HashMap::new();
    let mut md_stems: HashMap<String, PathBuf> = HashMap::new();

    let mut state = ScanState {
        examples_dir_name,
        docs_dir_name,
        resources: &mut resources,
        examples: &mut examples,
        md_stems: &mut md_stems,
    };
    scan_dir(scan_root, false, false, &mut state)?;

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

struct ScanState<'a> {
    examples_dir_name: &'a str,
    docs_dir_name: &'a str,
    resources: &'a mut HashMap<String, Value>,
    examples: &'a mut HashMap<String, Value>,
    md_stems: &'a mut HashMap<String, PathBuf>,
}

/// Recursively walk `dir`, collecting FHIR JSON resources and markdown files.
///
/// `in_examples` is true when scanning a path that descends from `examples_dir_name`,
/// which routes resources to the `examples` map instead of `resources`.
///
/// `in_docs` is true when scanning a path that descends from `docs_dir_name`.
/// JSON files in that subtree are ignored (the directory is for standalone narrative markdown).
fn scan_dir(dir: &Path, in_examples: bool, in_docs: bool, state: &mut ScanState<'_>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if path.is_dir() {
            if is_skip_dir(name) {
                continue;
            }
            let next_in_examples = in_examples || name == state.examples_dir_name;
            let next_in_docs = in_docs || name == state.docs_dir_name;
            scan_dir(&path, next_in_examples, next_in_docs, state)?;
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
            "json"
                if !in_docs
                    && stem != "package"
                    && stem != "fhir-lock"
                    && !stem.starts_with('.') =>
            {
                match load_json_resource(&path) {
                    Ok(value) => {
                        if in_examples {
                            state.examples.insert(stem.to_string(), value);
                        } else {
                            state.resources.insert(stem.to_string(), value);
                        }
                    }
                    Err(e) => {
                        warn!("Skipping {}: {}", path.display(), e);
                    }
                }
            }
            "md" => {
                state.md_stems.insert(stem.to_string(), path.clone());
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

    fn minimal_packager_toml() -> &'static str {
        r#"id = "example.pkg"
version = "1.0.0"
fhir_version = "4.0.1"
"#
    }

    #[test]
    fn fails_when_no_implementation_guide() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
        write_file(
            dir.path(),
            "ValueSet-foo.json",
            r#"{"resourceType":"ValueSet","id":"foo"}"#,
        );
        let result = load_source_dir(dir.path(), dir.path().join("output"));
        assert!(
            matches!(result, Err(crate::PublisherError::MissingFile(f)) if f.contains("ImplementationGuide"))
        );
    }

    #[test]
    fn loads_resources_and_partitions_markdown() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());
        // A leftover package.json from a prior layout should not be loaded as a resource.
        write_file(
            dir.path(),
            "package.json",
            r#"{"name":"example.pkg","version":"1.0.0","fhirVersions":["4.0.1"]}"#,
        );

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(!ctx.resources.contains_key("package"));
    }

    #[test]
    fn loads_resources_from_subdirectory() {
        let dir = TempDir::new().unwrap();
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "packager.toml", minimal_packager_toml());
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
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert!(ctx.config.hooks.before_build.is_empty());
    }

    #[test]
    fn derives_package_json_from_packager_toml() {
        let dir = TempDir::new().unwrap();
        write_file(
            dir.path(),
            "packager.toml",
            r#"id = "com.example.fhir"
version = "2.0.0"
canonical = "https://example.org/fhir"
fhir_version = "4.0.1"
description = "Example"

[dependencies]
"hl7.fhir.r4.core" = "4.0.1"
"#,
        );
        write_file(dir.path(), "ImplementationGuide.json", minimal_ig());

        let ctx = load_source_dir(dir.path(), dir.path().join("output")).unwrap();
        assert_eq!(ctx.package_json.name, "com.example.fhir");
        assert_eq!(ctx.package_json.version, "2.0.0");
        assert_eq!(
            ctx.package_json.url.as_deref(),
            Some("https://example.org/fhir")
        );
        assert_eq!(ctx.package_json.description.as_deref(), Some("Example"));
        assert_eq!(
            ctx.package_json
                .dependencies
                .get("hl7.fhir.r4.core")
                .map(|s| s.as_str()),
            Some("4.0.1")
        );
    }
}
