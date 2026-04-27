//! `validate` hook processor — validates FHIR resources using `rh-validator`.

use crate::{context::PublishContext, hooks::HookProcessor, PublisherError, Result};
use rh_validator::{FhirValidator, FhirVersion, Severity};
use std::path::PathBuf;
use tracing::{info, warn};

fn default_packages_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".fhir").join("packages")
}

/// Hook processor that validates all FHIR resources using `rh-validator`.
pub struct ValidateProcessor;

impl HookProcessor for ValidateProcessor {
    fn name(&self) -> &'static str {
        "validate"
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        let packages_dir = ctx
            .config
            .validate
            .packages_dir
            .as_ref()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| {
                default_packages_dir().to_string_lossy().to_string()
            });

        // Verify dependency packages exist before proceeding.
        let base_dir = PathBuf::from(&packages_dir);
        for (pkg_name, pkg_version) in &ctx.package_json.dependencies {
            let pkg_dir = base_dir.join(format!("{pkg_name}#{pkg_version}"));
            if !pkg_dir.exists() {
                return Err(PublisherError::MissingPackage(format!(
                    "{pkg_name}#{pkg_version}"
                )));
            }
        }

        let validator = FhirValidator::new(FhirVersion::R4, Some(&packages_dir))
            .map_err(|e| PublisherError::ValidationFailed(e.to_string()))?;

        let mut failed = false;
        for (stem, resource) in &ctx.resources {
            let result = validator
                .validate(resource)
                .map_err(|e| PublisherError::ValidationFailed(e.to_string()))?;

            for issue in &result.issues {
                match issue.severity {
                    Severity::Warning | Severity::Information => {
                        warn!("[{stem}] {}: {}", issue.severity, issue.message);
                    }
                    Severity::Error => {
                        tracing::error!("[{stem}] {}: {}", issue.severity, issue.message);
                        failed = true;
                    }
                }
            }
        }

        if failed {
            return Err(PublisherError::ValidationFailed(
                "One or more resources failed validation (see errors above)".to_string(),
            ));
        }

        info!("All {} resource(s) passed validation", ctx.resources.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, context::PublishContext, hooks::HookProcessor, manifest::PackageJson,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(resources: HashMap<String, serde_json::Value>, deps: HashMap<String, String>) -> (PublishContext, TempDir) {
        let tmp = TempDir::new().unwrap();
        let ctx = PublishContext {
            source_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: PackageJson {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: deps,
                url: None,
                description: None,
                author: None,
                license: None,
                extra: HashMap::new(),
            },
            resources,
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        };
        (ctx, tmp)
    }

    #[test]
    fn missing_dependency_package_returns_error() {
        let mut deps = HashMap::new();
        deps.insert("hl7.fhir.us.core".to_string(), "6.1.0".to_string());

        let (mut ctx, tmp) = make_ctx(HashMap::new(), deps);
        // Override packages_dir to a temp dir that definitely doesn't have the package.
        ctx.config.validate.packages_dir = Some(tmp.path().to_string_lossy().to_string());

        let err = ValidateProcessor.run(&mut ctx).unwrap_err();
        assert!(
            matches!(err, PublisherError::MissingPackage(ref s) if s.contains("hl7.fhir.us.core")),
            "Expected MissingPackage error, got: {err:?}"
        );
    }

    #[test]
    fn passes_on_valid_resource_with_no_deps() {
        // A minimal Patient resource should pass with no dependency packages needed.
        let mut resources = HashMap::new();
        resources.insert(
            "Patient-example".to_string(),
            json!({ "resourceType": "Patient", "id": "example" }),
        );

        let (mut ctx, tmp) = make_ctx(resources, HashMap::new());
        // Point packages_dir at an empty temp dir (no deps needed since no deps declared).
        ctx.config.validate.packages_dir = Some(tmp.path().to_string_lossy().to_string());

        // This may succeed or fail depending on whether core FHIR packages are installed,
        // but it must NOT return MissingPackage (since there are no declared deps).
        let result = ValidateProcessor.run(&mut ctx);
        if let Err(e) = &result {
            assert!(
                !matches!(e, PublisherError::MissingPackage(_)),
                "Should not get MissingPackage when no deps declared"
            );
        }
    }
}
