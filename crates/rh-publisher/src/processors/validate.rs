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
