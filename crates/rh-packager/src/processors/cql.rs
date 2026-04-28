//! `cql` hook processor — compiles CQL files and embeds them into FHIR Library resources.
//!
//! For each `*.cql` file in the source directory:
//! 1. Compiles it to ELM using `rh-cql`
//! 2. Finds or auto-creates a matching `Library-<name>.json` in the resource map
//! 3. Embeds base64-encoded CQL source + ELM JSON in `Library.content[]`
//! 4. Sets `Library.type` to `logic-library` if absent
//!
//! Follows CQL IG guidance: https://build.fhir.org/ig/HL7/cql-ig/en/

use crate::{context::PublishContext, hooks::HookProcessor, PublisherError, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use rh_cql::compile_to_json;
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use tracing::{info, warn};

const LOGIC_LIBRARY_TYPE: &str = "logic-library";
const LIBRARY_TYPE_SYSTEM: &str = "http://terminology.hl7.org/CodeSystem/library-type";

/// Hook processor that compiles CQL files and embeds them in Library resources.
pub struct CqlProcessor;

impl HookProcessor for CqlProcessor {
    fn name(&self) -> &'static str {
        "cql"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let source_dir = ctx.source_dir.clone();

        // Discover .cql files.
        let cql_files: Vec<PathBuf> = fs::read_dir(&source_dir)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("cql"))
            .collect();

        for cql_path in cql_files {
            let cql_source = fs::read_to_string(&cql_path)?;

            // Derive library name from filename stem.
            let stem = cql_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();

            // Compile CQL → ELM JSON.
            let elm_json = compile_to_json(&cql_source, None, false)
                .map_err(|e| PublisherError::Cql(format!("{stem}: {e}")))?;

            // Base64-encode both artifacts.
            let cql_b64 = STANDARD.encode(cql_source.as_bytes());
            let elm_b64 = STANDARD.encode(elm_json.as_bytes());

            let content = json!([
                { "contentType": "text/cql", "data": cql_b64 },
                { "contentType": "application/elm+json", "data": elm_b64 }
            ]);

            let library_key = format!("Library-{stem}");
            match ctx.resources.entry(library_key) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    // Update existing Library.
                    let resource = e.get_mut();
                    resource["content"] = content;
                    ensure_logic_library_type(resource);
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    // Auto-create a minimal Library resource.
                    warn!(
                        "No Library resource found for '{stem}'; auto-creating Library-{stem}. \
                         Consider adding Library-{stem}.json to your source directory."
                    );
                    e.insert(create_minimal_library(&stem, content));
                }
            }

            info!("Embedded CQL + ELM for library '{stem}'");
        }

        Ok(())
    }
}

fn ensure_logic_library_type(resource: &mut Value) {
    let has_logic_library = resource
        .get("type")
        .and_then(|t| t.get("coding"))
        .and_then(|c| c.as_array())
        .is_some_and(|arr| {
            arr.iter()
                .any(|entry| entry.get("code").and_then(|c| c.as_str()) == Some(LOGIC_LIBRARY_TYPE))
        });

    if !has_logic_library {
        resource["type"] = logic_library_type_value();
    }
}

fn create_minimal_library(name: &str, content: Value) -> Value {
    json!({
        "resourceType": "Library",
        "id": name,
        "status": "draft",
        "type": logic_library_type_value(),
        "content": content
    })
}

fn logic_library_type_value() -> Value {
    json!({
        "coding": [{
            "system": LIBRARY_TYPE_SYSTEM,
            "code": LOGIC_LIBRARY_TYPE,
            "display": "Logic Library"
        }]
    })
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

    const SIMPLE_CQL: &str = "library SimpleLib version '1.0'\n\ndefine \"Always True\": true\n";

    #[test]
    fn auto_creates_library_when_missing() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("SimpleLib.cql"), SIMPLE_CQL).unwrap();

        let mut ctx = make_ctx(&tmp, HashMap::new());
        CqlProcessor.run(&mut ctx).unwrap();

        assert!(ctx.resources.contains_key("Library-SimpleLib"));
        let lib = ctx.resources.get("Library-SimpleLib").unwrap();
        assert_eq!(lib["resourceType"], "Library");
    }

    #[test]
    fn embeds_cql_and_elm_content_entries() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("SimpleLib.cql"), SIMPLE_CQL).unwrap();

        let mut ctx = make_ctx(&tmp, HashMap::new());
        CqlProcessor.run(&mut ctx).unwrap();

        let lib = ctx.resources.get("Library-SimpleLib").unwrap();
        let content = lib["content"].as_array().unwrap();
        assert_eq!(content.len(), 2);

        let content_types: Vec<&str> = content
            .iter()
            .filter_map(|c| c["contentType"].as_str())
            .collect();
        assert!(content_types.contains(&"text/cql"));
        assert!(content_types.contains(&"application/elm+json"));
    }

    #[test]
    fn sets_logic_library_type_on_auto_created() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("SimpleLib.cql"), SIMPLE_CQL).unwrap();

        let mut ctx = make_ctx(&tmp, HashMap::new());
        CqlProcessor.run(&mut ctx).unwrap();

        let lib = ctx.resources.get("Library-SimpleLib").unwrap();
        let coding = lib["type"]["coding"].as_array().unwrap();
        assert!(coding
            .iter()
            .any(|c| c["code"].as_str() == Some(LOGIC_LIBRARY_TYPE)));
    }

    #[test]
    fn preserves_existing_library_type() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("SimpleLib.cql"), SIMPLE_CQL).unwrap();

        let mut resources = HashMap::new();
        resources.insert(
            "Library-SimpleLib".to_string(),
            json!({
                "resourceType": "Library",
                "id": "SimpleLib",
                "status": "active",
                "type": {
                    "coding": [{"system": LIBRARY_TYPE_SYSTEM, "code": LOGIC_LIBRARY_TYPE}]
                }
            }),
        );
        let mut ctx = make_ctx(&tmp, resources);
        CqlProcessor.run(&mut ctx).unwrap();

        let lib = ctx.resources.get("Library-SimpleLib").unwrap();
        assert_eq!(lib["status"], "active"); // custom status preserved
    }

    #[test]
    fn compile_error_aborts_with_cql_error() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("BadLib.cql"), "this is not valid CQL").unwrap();

        let mut ctx = make_ctx(&tmp, HashMap::new());
        let err = CqlProcessor.run(&mut ctx).unwrap_err();
        assert!(matches!(err, PublisherError::Cql(_)));
    }
}
