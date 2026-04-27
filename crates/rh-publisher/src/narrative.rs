//! Narrative processor — converts markdown files to FHIR `.text` (XHTML) and embeds them.
//!
//! Files named `<stem>.md` that match a `<stem>.json` FHIR resource in the resource map
//! have their content converted to XHTML and embedded as `resource.text`. Unmatched
//! markdown files are routed to `package/other/` via `ctx.standalone_markdown`.

use crate::{context::PublishContext, Result};
use pulldown_cmark::{html, Options, Parser};
use serde_json::{json, Value};

/// Process all matched markdown narrative files in the source directory:
/// - Converts markdown → XHTML wrapped in a FHIR-compliant div
/// - Embeds the result as `resource.text = {status: "generated", div: "..."}`
///
/// Unmatched `.md` files have already been recorded in `ctx.standalone_markdown`
/// by the loader and are handled during output assembly.
pub fn process_narrative(ctx: &mut PublishContext) -> Result<()> {
    let source_dir = ctx.source_dir.clone();

    // Collect matched (stem, content) pairs to avoid borrow issues.
    let mut matched: Vec<(String, String)> = Vec::new();

    for entry in std::fs::read_dir(&source_dir)? {
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
        if ext != "md" {
            continue;
        }
        if ctx.resources.contains_key(stem) {
            let content = std::fs::read_to_string(&path)?;
            matched.push((stem.to_string(), content));
        }
    }

    for (stem, md_content) in matched {
        let xhtml = markdown_to_xhtml(&md_content);
        if let Some(resource) = ctx.resources.get_mut(&stem) {
            embed_narrative(resource, xhtml);
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

    format!(
        r#"<div xmlns="http://www.w3.org/1999/xhtml">{html_output}</div>"#
    )
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
    use crate::{
        config::PublisherConfig, context::PublishContext, manifest::PackageJson,
    };
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
