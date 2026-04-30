//! `fsh` hook processor — compiles FHIR Shorthand (FSH) files into FHIR resources.
//!
//! Scans the source directory recursively for `*.fsh` files, compiles them into
//! FHIR resources using `rh-fsh`, and injects the results into the build context
//! before the core pipeline stages run.
//!
//! Register by name in `packager.toml`:
//! ```toml
//! [hooks]
//! before_build = ["fsh"]
//! ```
//!
//! All configuration is optional — values are inferred from `package.json` when absent:
//! ```toml
//! [fsh]
//! canonical  = "https://example.org/fhir"   # inferred from package.json url
//! status     = "active"
//! publisher  = "My Organization"
//! ```

use crate::{
    config::FshConfig, context::PublishContext, hooks::HookProcessor, PublisherError, Result,
};
use rh_fsh::{FhirDefs, FshExporter, FshParser, FshResolver, FshTank};
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;

/// Built-in processor that compiles FSH source files and injects the resulting
/// FHIR resources into the build context.
pub struct FshProcessor {
    config: FshConfig,
}

impl FshProcessor {
    pub fn new(config: FshConfig) -> Self {
        Self { config }
    }
}

impl HookProcessor for FshProcessor {
    fn name(&self) -> &str {
        "fsh"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let fsh_files = collect_fsh_files(&ctx.source_dir);

        if fsh_files.is_empty() {
            info!("fsh: no .fsh files found in {:?}, skipping", ctx.source_dir);
            return Ok(());
        }

        info!("fsh: compiling {} .fsh file(s)", fsh_files.len());

        let fsh_config = build_fsh_config(&self.config, ctx);

        let mut tank = FshTank::new();
        for path in &fsh_files {
            let content = std::fs::read_to_string(path).map_err(|e| {
                PublisherError::Other(anyhow::anyhow!(
                    "fsh: failed to read {}: {e}",
                    path.display()
                ))
            })?;
            let source_name = path.to_string_lossy().into_owned();
            let doc = FshParser::parse(&content, source_name).map_err(|e| {
                PublisherError::Other(anyhow::anyhow!(
                    "fsh: parse error in {}: {e}",
                    path.display()
                ))
            })?;
            tank.add_document(doc).map_err(|errs| {
                let msgs: Vec<_> = errs.iter().map(|e| e.to_string()).collect();
                PublisherError::Other(anyhow::anyhow!(
                    "fsh: document error(s): {}",
                    msgs.join("; ")
                ))
            })?;
        }

        FshResolver::resolve(&mut tank).map_err(|errs| {
            let msgs: Vec<_> = errs.iter().map(|e| e.to_string()).collect();
            PublisherError::Other(anyhow::anyhow!(
                "fsh: resolve error(s): {}",
                msgs.join("; ")
            ))
        })?;

        let defs = FhirDefs::r4();
        let package = FshExporter::export(&tank, defs, &fsh_config);

        for err in &package.errors {
            warn!("fsh: non-fatal compilation warning: {err}");
        }

        let mut injected = 0usize;
        for resource in package.resources {
            if let Some(stem) = resource_stem(&resource) {
                if ctx.resources.contains_key(&stem) {
                    warn!("fsh: overwriting existing resource '{stem}' with FSH-compiled version");
                }
                ctx.resources.insert(stem, resource);
                injected += 1;
            } else {
                warn!("fsh: compiled resource has no resourceType or id/name — skipping");
            }
        }

        info!("fsh: injected {injected} resource(s) into build context");
        Ok(())
    }
}

/// Build an `rh_fsh::FshConfig` by merging `[fsh]` overrides with values
/// derived from `package.json`.
fn build_fsh_config(cfg: &FshConfig, ctx: &PublishContext) -> rh_fsh::FshConfig {
    rh_fsh::FshConfig {
        canonical: cfg
            .canonical
            .clone()
            .or_else(|| ctx.package_json.url.clone()),
        fhir_version: cfg
            .fhir_version
            .clone()
            .or_else(|| ctx.package_json.fhir_versions.first().cloned()),
        id: cfg
            .id
            .clone()
            .or_else(|| Some(ctx.package_json.name.clone())),
        name: cfg
            .name
            .clone()
            .or_else(|| Some(ctx.package_json.name.clone())),
        status: cfg.status.clone().or_else(|| Some("draft".to_string())),
        publisher: cfg.publisher.clone(),
        version: cfg
            .version
            .clone()
            .or_else(|| Some(ctx.package_json.version.clone())),
    }
}

/// Derive a resource map key from a compiled FHIR resource.
///
/// Returns `<ResourceType>-<id>` (preferred) or `<ResourceType>-<name>` as a
/// fallback, matching the naming convention used by the file loader.
fn resource_stem(resource: &serde_json::Value) -> Option<String> {
    let rt = resource.get("resourceType")?.as_str()?;
    if let Some(id) = resource.get("id").and_then(|v| v.as_str()) {
        return Some(format!("{rt}-{id}"));
    }
    if let Some(name) = resource.get("name").and_then(|v| v.as_str()) {
        return Some(format!("{rt}-{name}"));
    }
    None
}

/// Collect all `*.fsh` files under `dir` recursively.
fn collect_fsh_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("fsh"))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FshConfig, PublisherConfig};
    use serde_json::json;
    use std::collections::HashMap;

    fn make_ctx(source_dir: PathBuf) -> PublishContext {
        use crate::manifest::PackageJson;
        use std::collections::HashMap as HM;
        PublishContext {
            source_dir,
            output_dir: PathBuf::from("/tmp/out"),
            package_json: PackageJson {
                name: "test.pkg".to_string(),
                version: "0.1.0".to_string(),
                url: Some("https://example.org/fhir".to_string()),
                fhir_versions: vec!["4.0.1".to_string()],
                dependencies: HM::new(),
                description: None,
                author: None,
                license: None,
                extra: HM::new(),
            },
            resources: HashMap::new(),
            examples: HashMap::new(),
            standalone_markdown: Vec::new(),
            config: PublisherConfig::default(),
        }
    }

    #[test]
    fn test_name() {
        let p = FshProcessor::new(FshConfig::default());
        assert_eq!(p.name(), "fsh");
    }

    #[test]
    fn test_no_fsh_files_is_ok() {
        let dir = tempfile::tempdir().unwrap();
        let mut ctx = make_ctx(dir.path().to_path_buf());
        let p = FshProcessor::new(FshConfig::default());
        assert!(p.run(&mut ctx).is_ok());
        assert!(ctx.resources.is_empty());
    }

    #[test]
    fn test_compile_simple_profile() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("MyPatient.fsh"),
            r#"
Profile: MyPatient
Parent: Patient
Id: my-patient
Title: "My Patient"
Description: "A simple patient profile."
"#,
        )
        .unwrap();

        let mut ctx = make_ctx(dir.path().to_path_buf());
        let p = FshProcessor::new(FshConfig::default());
        p.run(&mut ctx).unwrap();

        assert!(
            ctx.resources.contains_key("StructureDefinition-my-patient"),
            "Expected StructureDefinition-my-patient in resources, got: {:?}",
            ctx.resources.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_resource_stem() {
        assert_eq!(
            resource_stem(&json!({"resourceType": "StructureDefinition", "id": "foo"})),
            Some("StructureDefinition-foo".to_string())
        );
        assert_eq!(
            resource_stem(&json!({"resourceType": "ValueSet", "name": "MyVS"})),
            Some("ValueSet-MyVS".to_string())
        );
        assert_eq!(resource_stem(&json!({"id": "foo"})), None);
    }

    #[test]
    fn test_config_infers_from_package_json() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = make_ctx(dir.path().to_path_buf());
        let fsh_cfg = build_fsh_config(&FshConfig::default(), &ctx);
        assert_eq!(
            fsh_cfg.canonical,
            Some("https://example.org/fhir".to_string())
        );
        assert_eq!(fsh_cfg.version, Some("0.1.0".to_string()));
        assert_eq!(fsh_cfg.id, Some("test.pkg".to_string()));
        assert_eq!(fsh_cfg.fhir_version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_config_overrides_take_precedence() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = make_ctx(dir.path().to_path_buf());
        let cfg = FshConfig {
            canonical: Some("https://override.org".to_string()),
            status: Some("active".to_string()),
            ..Default::default()
        };
        let fsh_cfg = build_fsh_config(&cfg, &ctx);
        assert_eq!(fsh_cfg.canonical, Some("https://override.org".to_string()));
        assert_eq!(fsh_cfg.status, Some("active".to_string()));
        // Non-overridden fields still come from package.json
        assert_eq!(fsh_cfg.version, Some("0.1.0".to_string()));
    }
}
