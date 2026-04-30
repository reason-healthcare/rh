//! Populates the `ImplementationGuide` resource with `dependsOn`, `definition.resource`,
//! and `definition.page` derived from the package context.
//!
//! Runs during the build pipeline after narrative is processed. The three sections are
//! **replaced** on each build so the published IG accurately reflects the package's actual
//! dependencies, resources, and pages. The rest of the IG (metadata, extensions, etc.) is
//! left untouched.
//!
//! ## `dependsOn`
//! Derived from `package.json` dependencies. Core FHIR packages (`hl7.fhir.r*.core`) are
//! excluded because they are already expressed via the IG's `fhirVersion` field.
//! Each entry includes `id` (slug from packageId), `packageId`, and `version`.
//! The `uri` field is omitted — it requires a registry lookup and is optional in R4.
//!
//! ## `definition.resource`
//! One entry per resource in `ctx.resources` (excluding the IG itself) and `ctx.examples`.
//! `exampleBoolean` is `false` for definitional resources and `true` for examples.
//!
//! ## `definition.page`
//! Populated when standalone markdown files exist in `docs/`. A root page (`index.md`) wraps
//! child entries — one per file — with `nameUrl` set to `other/<filename>.md` (matching the
//! output path in `package/other/`). The page `title` is parsed from the first `# Heading`
//! in the file, falling back to a humanised version of the filename stem.
//! If no standalone markdown files exist, `definition.page` is left as-is.

use crate::{context::PublishContext, PublisherError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Core FHIR package IDs expressed via `fhirVersion`, not `dependsOn`.
const CORE_FHIR_PACKAGES: &[&str] = &[
    "hl7.fhir.r2.core",
    "hl7.fhir.r3.core",
    "hl7.fhir.r4.core",
    "hl7.fhir.r4b.core",
    "hl7.fhir.r5.core",
    "hl7.fhir.r6.core",
];

/// Populate the `ImplementationGuide` resource in `ctx.resources` with `dependsOn`
/// and `definition.resource` derived from the current build context.
pub fn populate_ig(ctx: &mut PublishContext) -> Result<()> {
    let ig_stem = find_ig_stem(ctx)?;

    let depends_on = build_depends_on(&ctx.package_json.dependencies);
    let def_resources = build_definition_resources(&ctx.resources, &ctx.examples);

    let ig = ctx
        .resources
        .get_mut(&ig_stem)
        .ok_or_else(|| crate::PublisherError::MissingFile("ImplementationGuide".into()))?;

    if depends_on.is_empty() {
        if let Some(obj) = ig.as_object_mut() {
            obj.remove("dependsOn");
        }
    } else {
        ig["dependsOn"] = json!(depends_on);
    }

    // Preserve any existing definition extensions/pages/etc; only replace managed fields.
    if ig.get("definition").is_none_or(|d| d.is_null()) {
        ig["definition"] = json!({ "resource": [] });
    }
    ig["definition"]["resource"] = json!(def_resources);

    // Populate definition.page from standalone markdown files (docs/).
    if !ctx.standalone_markdown.is_empty() {
        let pkg_title = ctx.package_json.extra.get("title").and_then(|v| v.as_str());
        let page_tree =
            build_page_tree(&ctx.standalone_markdown, &ctx.package_json.name, pkg_title);
        ig["definition"]["page"] = page_tree;
    }

    Ok(())
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn find_ig_stem(ctx: &PublishContext) -> Result<String> {
    ctx.resources
        .iter()
        .find(|(_, v)| is_resource_type(v, "ImplementationGuide"))
        .map(|(stem, _)| stem.clone())
        .ok_or_else(|| {
            PublisherError::MissingFile(
                "ImplementationGuide resource (e.g. ImplementationGuide.json)".to_string(),
            )
        })
}

fn build_depends_on(dependencies: &HashMap<String, String>) -> Vec<Value> {
    let mut entries: Vec<Value> = dependencies
        .iter()
        .filter(|(pkg_id, _)| !CORE_FHIR_PACKAGES.contains(&pkg_id.as_str()))
        .map(|(pkg_id, version)| {
            json!({
                "id": package_id_to_slug(pkg_id),
                "packageId": pkg_id,
                "version": version,
            })
        })
        .collect();

    entries.sort_by(|a, b| {
        let a_id = a["packageId"].as_str().unwrap_or("");
        let b_id = b["packageId"].as_str().unwrap_or("");
        a_id.cmp(b_id)
    });

    entries
}

fn build_definition_resources(
    resources: &HashMap<String, Value>,
    examples: &HashMap<String, Value>,
) -> Vec<Value> {
    let mut entries: Vec<Value> = Vec::new();

    let mut resource_list: Vec<_> = resources
        .iter()
        .filter(|(_, v)| !is_resource_type(v, "ImplementationGuide"))
        .collect();
    resource_list.sort_by_key(|(stem, _)| stem.as_str());

    for (_, resource) in resource_list {
        if let Some(entry) = make_entry(resource, false) {
            entries.push(entry);
        }
    }

    let mut example_list: Vec<_> = examples.iter().collect();
    example_list.sort_by_key(|(stem, _)| stem.as_str());

    for (_, resource) in example_list {
        if let Some(entry) = make_entry(resource, true) {
            entries.push(entry);
        }
    }

    entries
}

fn make_entry(resource: &Value, is_example: bool) -> Option<Value> {
    let resource_type = resource.get("resourceType")?.as_str()?;
    let id = resource.get("id")?.as_str()?;

    let mut entry = json!({
        "reference": { "reference": format!("{resource_type}/{id}") },
        "exampleBoolean": is_example,
    });

    // name: prefer `name`, fall back to `title`
    let name = resource
        .get("name")
        .and_then(|v| v.as_str())
        .or_else(|| resource.get("title").and_then(|v| v.as_str()));
    if let Some(n) = name {
        entry["name"] = json!(n);
    }

    if let Some(desc) = resource.get("description").and_then(|v| v.as_str()) {
        entry["description"] = json!(desc);
    }

    Some(entry)
}

fn is_resource_type(v: &Value, rt: &str) -> bool {
    v.get("resourceType")
        .and_then(|r| r.as_str())
        .is_some_and(|r| r == rt)
}

/// Convert a package ID to a safe FHIR `id`-compliant slug.
/// e.g. `hl7.fhir.uv.smart-app-launch` → `hl7_fhir_uv_smart_app_launch`
fn package_id_to_slug(pkg_id: &str) -> String {
    pkg_id.replace(['.', '-'], "_")
}

/// Build a `definition.page` tree from standalone markdown files.
///
/// Structure:
/// ```json
/// { "nameUrl": "index.md", "title": "<pkg title>", "generation": "markdown",
///   "page": [ { "nameUrl": "other/overview.md", "title": "Overview", "generation": "markdown" }, ... ] }
/// ```
fn build_page_tree(markdown_files: &[PathBuf], pkg_name: &str, pkg_title: Option<&str>) -> Value {
    let root_title = pkg_title.unwrap_or(pkg_name);

    let mut pages: Vec<_> = markdown_files.iter().collect();
    pages.sort_by_key(|p| p.file_name().and_then(|n| n.to_str()).unwrap_or(""));

    let children: Vec<Value> = pages.iter().map(|p| make_page_entry(p)).collect();

    let mut root = json!({
        "nameUrl": "index.md",
        "title": root_title,
        "generation": "markdown",
    });

    if !children.is_empty() {
        root["page"] = json!(children);
    }

    root
}

fn make_page_entry(path: &Path) -> Value {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("page.md");

    let title = parse_markdown_title(path).unwrap_or_else(|| stem_to_title(path));

    json!({
        "nameUrl": format!("other/{filename}"),
        "title": title,
        "generation": "markdown",
    })
}

/// Parse the first ATX `# Heading` from a markdown file.
fn parse_markdown_title(path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    content.lines().find_map(|line| {
        line.trim()
            .strip_prefix("# ")
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
    })
}

/// Convert a filename stem to a human-readable title.
/// e.g. `blood-pressure_guide` → `"Blood Pressure Guide"`
fn stem_to_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Page")
        .split(['-', '_'])
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::collections::HashMap;

    fn make_ctx(
        deps: &[(&str, &str)],
        resources: &[(&str, Value)],
        examples: &[(&str, Value)],
    ) -> PublishContext {
        let mut dependencies = HashMap::new();
        for (k, v) in deps {
            dependencies.insert(k.to_string(), v.to_string());
        }
        let pkg = PackageJson {
            name: "test.fhir.pkg".to_string(),
            version: "1.0.0".to_string(),
            fhir_versions: vec!["4.0.1".to_string()],
            dependencies,
            url: None,
            description: None,
            author: None,
            license: None,
            extra: HashMap::new(),
        };

        let mut res_map = HashMap::new();
        res_map.insert(
            "ImplementationGuide".to_string(),
            json!({
                "resourceType": "ImplementationGuide",
                "id": "test.fhir.pkg",
                "packageId": "test.fhir.pkg",
                "version": "1.0.0",
                "status": "draft"
            }),
        );
        for (stem, v) in resources {
            res_map.insert(stem.to_string(), v.clone());
        }

        let mut ex_map = HashMap::new();
        for (stem, v) in examples {
            ex_map.insert(stem.to_string(), v.clone());
        }

        PublishContext {
            source_dir: std::path::PathBuf::from("/tmp/src"),
            output_dir: std::path::PathBuf::from("/tmp/out"),
            package_json: pkg,
            resources: res_map,
            examples: ex_map,
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        }
    }

    #[test]
    fn depends_on_excludes_core_fhir_package() {
        let mut ctx = make_ctx(
            &[
                ("hl7.fhir.r4.core", "4.0.1"),
                ("hl7.terminology.r4", "3.1.0"),
            ],
            &[],
            &[],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let deps = ig["dependsOn"].as_array().unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0]["packageId"], "hl7.terminology.r4");
    }

    #[test]
    fn depends_on_absent_when_only_core_dependencies() {
        let mut ctx = make_ctx(&[("hl7.fhir.r4.core", "4.0.1")], &[], &[]);
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        assert!(ig.get("dependsOn").is_none());
    }

    #[test]
    fn depends_on_id_slug_replaces_dots_and_hyphens() {
        let mut ctx = make_ctx(&[("hl7.fhir.uv.smart-app-launch", "2.0.0")], &[], &[]);
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let slug = ig["dependsOn"][0]["id"].as_str().unwrap();
        assert_eq!(slug, "hl7_fhir_uv_smart_app_launch");
    }

    #[test]
    fn definition_resource_excludes_implementation_guide() {
        let mut ctx = make_ctx(
            &[],
            &[(
                "ValueSet-vs1",
                json!({"resourceType":"ValueSet","id":"vs1","name":"VS1"}),
            )],
            &[],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let res = ig["definition"]["resource"].as_array().unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0]["reference"]["reference"], "ValueSet/vs1");
    }

    #[test]
    fn examples_have_example_boolean_true() {
        let mut ctx = make_ctx(
            &[],
            &[],
            &[(
                "Patient-example",
                json!({"resourceType":"Patient","id":"example"}),
            )],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let res = ig["definition"]["resource"].as_array().unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0]["exampleBoolean"], true);
        assert_eq!(res[0]["reference"]["reference"], "Patient/example");
    }

    #[test]
    fn definition_resource_uses_name_field() {
        let mut ctx = make_ctx(
            &[],
            &[(
                "StructureDefinition-bp",
                json!({"resourceType":"StructureDefinition","id":"bp","name":"BpProfile","title":"BP Profile"}),
            )],
            &[],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let res = ig["definition"]["resource"].as_array().unwrap();
        assert_eq!(res[0]["name"], "BpProfile");
    }

    #[test]
    fn definition_resource_falls_back_to_title() {
        let mut ctx = make_ctx(
            &[],
            &[(
                "StructureDefinition-bp",
                json!({"resourceType":"StructureDefinition","id":"bp","title":"BP Profile"}),
            )],
            &[],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let res = ig["definition"]["resource"].as_array().unwrap();
        assert_eq!(res[0]["name"], "BP Profile");
    }

    #[test]
    fn definition_resource_includes_description() {
        let mut ctx = make_ctx(
            &[],
            &[(
                "ValueSet-vs1",
                json!({"resourceType":"ValueSet","id":"vs1","description":"A value set"}),
            )],
            &[],
        );
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let res = ig["definition"]["resource"].as_array().unwrap();
        assert_eq!(res[0]["description"], "A value set");
    }

    #[test]
    fn preserves_existing_definition_extensions() {
        let mut ctx = make_ctx(&[], &[], &[]);
        // Pre-populate the IG with a definition that has extensions.
        let ig = ctx.resources.get_mut("ImplementationGuide").unwrap();
        ig["definition"] = json!({
            "extension": [{"url": "http://example.org/ext", "valueString": "kept"}],
            "resource": []
        });

        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let exts = ig["definition"]["extension"].as_array().unwrap();
        assert_eq!(exts.len(), 1, "extension should be preserved");
    }

    #[test]
    fn error_when_no_implementation_guide_resource() {
        let pkg = PackageJson {
            name: "test.fhir.pkg".to_string(),
            version: "1.0.0".to_string(),
            fhir_versions: vec![],
            dependencies: HashMap::new(),
            url: None,
            description: None,
            author: None,
            license: None,
            extra: HashMap::new(),
        };
        let mut ctx = PublishContext {
            source_dir: std::path::PathBuf::from("/tmp/src"),
            output_dir: std::path::PathBuf::from("/tmp/out"),
            package_json: pkg,
            resources: HashMap::new(),
            examples: HashMap::new(),
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        };
        let err = populate_ig(&mut ctx).unwrap_err();
        assert!(matches!(err, PublisherError::MissingFile(_)));
    }

    // ── page tests ──────────────────────────────────────────────────────────────

    #[test]
    fn page_not_set_when_no_standalone_markdown() {
        let mut ctx = make_ctx(&[], &[], &[]);
        populate_ig(&mut ctx).unwrap();
        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        assert!(ig["definition"].get("page").is_none());
    }

    #[test]
    fn page_set_when_standalone_markdown_present() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("overview.md");
        std::fs::write(&md, "# Overview\nSome content.").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        assert!(ig["definition"]["page"].is_object());
        assert_eq!(ig["definition"]["page"]["nameUrl"], "index.md");
        assert_eq!(ig["definition"]["page"]["generation"], "markdown");
    }

    #[test]
    fn page_child_uses_other_prefix_in_name_url() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("overview.md");
        std::fs::write(&md, "# Overview").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let child = &ig["definition"]["page"]["page"][0];
        assert_eq!(child["nameUrl"], "other/overview.md");
    }

    #[test]
    fn page_title_parsed_from_h1() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("intro.md");
        std::fs::write(&md, "# Introduction Guide\nContent here.").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let child = &ig["definition"]["page"]["page"][0];
        assert_eq!(child["title"], "Introduction Guide");
    }

    #[test]
    fn page_title_falls_back_to_humanised_stem() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("blood-pressure_guide.md");
        // No H1 heading
        std::fs::write(&md, "Some content without a heading.").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        let child = &ig["definition"]["page"]["page"][0];
        assert_eq!(child["title"], "Blood Pressure Guide");
    }

    #[test]
    fn page_root_uses_package_title_from_extra() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("overview.md");
        std::fs::write(&md, "# Overview").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.package_json
            .extra
            .insert("title".to_string(), json!("My Custom IG"));
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        assert_eq!(ig["definition"]["page"]["title"], "My Custom IG");
    }

    #[test]
    fn page_root_falls_back_to_package_name() {
        use tempfile::TempDir;
        let tmp = TempDir::new().unwrap();
        let md = tmp.path().join("overview.md");
        std::fs::write(&md, "# Overview").unwrap();

        let mut ctx = make_ctx(&[], &[], &[]);
        ctx.standalone_markdown.push(md);
        populate_ig(&mut ctx).unwrap();

        let ig = ctx.resources.get("ImplementationGuide").unwrap();
        assert_eq!(ig["definition"]["page"]["title"], "test.fhir.pkg");
    }

    #[test]
    fn stem_to_title_humanises_correctly() {
        use std::path::PathBuf;
        assert_eq!(
            stem_to_title(&PathBuf::from("blood-pressure.md")),
            "Blood Pressure"
        );
        assert_eq!(
            stem_to_title(&PathBuf::from("my_overview.md")),
            "My Overview"
        );
        assert_eq!(stem_to_title(&PathBuf::from("intro.md")), "Intro");
    }
}
