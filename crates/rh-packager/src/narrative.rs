//! Narrative processor — converts markdown files to FHIR `.text` (XHTML) and embeds them.
//!
//! Files named `<stem>.md` that match a `<stem>.json` FHIR resource in the resource map
//! have their content converted to XHTML and embedded as `resource.text`. Unmatched
//! markdown files (e.g. those in `docs/`) are routed to `package/other/` via
//! `ctx.standalone_markdown`.

use crate::{context::PublishContext, Result};
use pulldown_cmark::{html, Options, Parser};
use serde_json::{json, Value};
use std::path::Path;

/// Process all matched markdown narrative files in the source directory:
/// - Scans recursively through subdirectories (skipping hidden dirs, `output/`, etc.)
/// - Converts markdown → XHTML wrapped in a FHIR-compliant div
/// - Embeds the result as `resource.text = {status: "generated", div: "..."}`
///   for resources matched by filename stem in either `ctx.resources` or `ctx.examples`
///
/// Unmatched `.md` files have already been recorded in `ctx.standalone_markdown`
/// by the loader and are handled during output assembly.
pub fn process_narrative(ctx: &mut PublishContext) -> Result<()> {
    let source_dir = ctx.source_dir.clone();

    // Collect (stem, content) pairs without holding borrows on ctx.
    let mut resource_matched: Vec<(String, String)> = Vec::new();
    let mut example_matched: Vec<(String, String)> = Vec::new();

    collect_narrative_matches(
        &source_dir,
        &ctx.resources,
        &ctx.examples,
        &mut resource_matched,
        &mut example_matched,
    )?;

    for (stem, md_content) in resource_matched {
        let xhtml = markdown_to_xhtml(&md_content);
        if let Some(resource) = ctx.resources.get_mut(&stem) {
            embed_narrative(resource, xhtml);
        }
    }
    for (stem, md_content) in example_matched {
        let xhtml = markdown_to_xhtml(&md_content);
        if let Some(resource) = ctx.examples.get_mut(&stem) {
            embed_narrative(resource, xhtml);
        }
    }

    Ok(())
}

/// Recursively walk `dir` for `*.md` files and match stems against `resources` and `examples`.
fn collect_narrative_matches(
    dir: &Path,
    resources: &std::collections::HashMap<String, Value>,
    examples: &std::collections::HashMap<String, Value>,
    resource_matched: &mut Vec<(String, String)>,
    example_matched: &mut Vec<(String, String)>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if path.is_dir() {
            if name.starts_with('.') || matches!(name, "output" | "target" | "node_modules") {
                continue;
            }
            collect_narrative_matches(
                &path,
                resources,
                examples,
                resource_matched,
                example_matched,
            )?;
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

        if ext != "md" {
            continue;
        }

        if resources.contains_key(stem) {
            let content = std::fs::read_to_string(&path)?;
            resource_matched.push((stem.to_string(), content));
        } else if examples.contains_key(stem) {
            let content = std::fs::read_to_string(&path)?;
            example_matched.push((stem.to_string(), content));
        }
    }
    Ok(())
}

/// Convert a markdown string to a FHIR-compliant XHTML narrative `<div>`.
///
/// The output is wrapped in `<div xmlns="http://www.w3.org/1999/xhtml">...</div>`
/// per the FHIR narrative profile.
pub fn markdown_to_xhtml(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    format!(r#"<div xmlns="http://www.w3.org/1999/xhtml">{html_output}</div>"#)
}

/// Embed a FHIR `.text` narrative block into a resource value.
fn embed_narrative(resource: &mut Value, xhtml_div: String) {
    resource["text"] = json!({
        "status": "generated",
        "div": xhtml_div
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::{collections::HashMap, fs};
    use tempfile::TempDir;

    fn make_ctx(tmp: &TempDir, resources: HashMap<String, Value>) -> PublishContext {
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
    fn markdown_converts_to_xhtml_div() {
        let xhtml = markdown_to_xhtml("# Hello\n\nThis is **bold**.");
        assert!(xhtml.starts_with(r#"<div xmlns="http://www.w3.org/1999/xhtml">"#));
        assert!(xhtml.ends_with("</div>"));
        assert!(xhtml.contains("<h1>Hello</h1>"));
        assert!(xhtml.contains("<strong>bold</strong>"));
    }

    #[test]
    fn matched_markdown_embeds_into_resource_text() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("StructureDefinition-foo.md"),
            "# Foo\n\nDescription of Foo.",
        )
        .unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","id":"foo"}),
        );

        let mut ctx = make_ctx(&tmp, resources);
        process_narrative(&mut ctx).unwrap();

        let resource = ctx.resources.get("StructureDefinition-foo").unwrap();
        assert_eq!(resource["text"]["status"], "generated");
        let div = resource["text"]["div"].as_str().unwrap();
        assert!(div.contains("Foo"));
        assert!(div.contains(r#"xmlns="http://www.w3.org/1999/xhtml""#));
    }

    #[test]
    fn unmatched_markdown_not_embedded_in_resources() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("overview.md"), "# Overview").unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","id":"foo"}),
        );

        let mut ctx = make_ctx(&tmp, resources);
        // overview.md has no match, should be pre-placed in standalone_markdown by loader.
        process_narrative(&mut ctx).unwrap();

        // The StructureDefinition should NOT have .text (no matching md was found).
        let resource = ctx.resources.get("StructureDefinition-foo").unwrap();
        assert!(resource.get("text").is_none());
    }

    #[test]
    fn resource_without_narrative_unchanged() {
        let tmp = TempDir::new().unwrap();
        // No .md files at all.
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-bar".to_string(),
            json!({"resourceType":"ValueSet","id":"bar"}),
        );

        let mut ctx = make_ctx(&tmp, resources);
        process_narrative(&mut ctx).unwrap();

        let resource = ctx.resources.get("ValueSet-bar").unwrap();
        assert!(resource.get("text").is_none());
    }
}
