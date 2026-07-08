//! Validator that checks `ImplementationGuide.json` fields are consistent with `packager.toml`.
//!
//! The `ImplementationGuide` resource and `packager.toml` must stay in sync per the
//! FHIR Package Specification. This module compares the key fields and returns
//! descriptive errors for each mismatch found.

use crate::{context::PublishContext, PublisherError, Result};
use tracing::warn;

/// Validate that the `ImplementationGuide` resource in `ctx.resources` is consistent
/// with the package manifest (derived from `packager.toml`). All mismatches are
/// collected and returned as a single error with a newline-separated list.
///
/// # Checked Fields
/// | `packager.toml` field | IG field |
/// |---|---|
/// | `id` | `packageId` |
/// | `version` | `version` |
/// | `canonical` | `url` derived as `{canonical}/ImplementationGuide/{ImplementationGuide.id}` |
/// | `fhir_version` | `fhirVersion[0]` |
pub fn check_ig_sync(ctx: &PublishContext) -> Result<()> {
    let ig = find_ig(ctx)?;
    let pkg = &ctx.package_json;
    let mut mismatches: Vec<String> = Vec::new();

    check_field(
        &mut mismatches,
        "packageId",
        Some(pkg.name.as_str()),
        ig.get("packageId").and_then(|v| v.as_str()),
    );

    check_field(
        &mut mismatches,
        "version",
        Some(pkg.version.as_str()),
        ig.get("version").and_then(|v| v.as_str()),
    );

    if let Some(canonical_base) = pkg.url.as_deref() {
        if let Some(ig_id) = ig.get("id").and_then(|v| v.as_str()) {
            let expected_ig_url = crate::canonical::implementation_guide_url(canonical_base, ig_id);
            warn_field_mismatch(
                "url",
                Some(&expected_ig_url),
                ig.get("url").and_then(|v| v.as_str()),
            );
        } else {
            warn!("ImplementationGuide is missing id; cannot derive expected url");
        }
    }

    // Check first fhirVersion entry if present in both.
    let pkg_fhir = pkg.fhir_versions.first().map(|s| s.as_str());
    let ig_fhir = ig
        .get("fhirVersion")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.as_str());

    if pkg_fhir.is_some() || ig_fhir.is_some() {
        check_field(&mut mismatches, "fhirVersion[0]", pkg_fhir, ig_fhir);
    }

    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(PublisherError::IgSync(mismatches.join("\n")))
    }
}

fn find_ig(ctx: &PublishContext) -> Result<&serde_json::Value> {
    ctx.resources
        .values()
        .find(|v| {
            v.get("resourceType")
                .and_then(|rt| rt.as_str())
                .is_some_and(|rt| rt == "ImplementationGuide")
        })
        .ok_or_else(|| {
            PublisherError::MissingFile(
                "ImplementationGuide resource (e.g. ImplementationGuide.json)".to_string(),
            )
        })
}

fn check_field(
    mismatches: &mut Vec<String>,
    field: &str,
    expected: Option<&str>,
    actual: Option<&str>,
) {
    match (expected, actual) {
        (Some(exp), Some(act)) if exp != act => {
            mismatches.push(format!(
                "  {field}: packager.toml has \"{exp}\" but ImplementationGuide has \"{act}\""
            ));
        }
        (Some(exp), None) => {
            mismatches.push(format!(
                "  {field}: packager.toml has \"{exp}\" but ImplementationGuide is missing this field"
            ));
        }
        _ => {}
    }
}

fn warn_field_mismatch(field: &str, expected: Option<&str>, actual: Option<&str>) {
    match (expected, actual) {
        (Some(exp), Some(act)) if exp != act => {
            warn!(
                field,
                expected = exp,
                actual = act,
                "ImplementationGuide field differs from value derived from packager.toml"
            );
        }
        (Some(exp), None) => {
            warn!(
                field,
                expected = exp,
                "ImplementationGuide is missing field derived from packager.toml"
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::collections::HashMap;

    fn make_context(
        pkg_name: &str,
        pkg_version: &str,
        pkg_url: Option<&str>,
        pkg_fhir: &[&str],
        ig: serde_json::Value,
    ) -> PublishContext {
        let extra: HashMap<String, serde_json::Value> = HashMap::new();
        let pkg = PackageJson {
            name: pkg_name.to_string(),
            version: pkg_version.to_string(),
            fhir_versions: pkg_fhir.iter().map(|s| s.to_string()).collect(),
            dependencies: HashMap::new(),
            url: pkg_url.map(|s| s.to_string()),
            canonical: pkg_url.map(|s| s.to_string()),
            description: None,
            author: None,
            license: None,
            extra,
        };

        let mut resources = HashMap::new();
        resources.insert("ImplementationGuide".to_string(), ig);

        PublishContext {
            source_dir: std::path::PathBuf::from("/tmp/src"),
            input_dir: std::path::PathBuf::from("/tmp/src"),
            output_dir: std::path::PathBuf::from("/tmp/out"),
            package_json: pkg,
            resources,
            examples: HashMap::new(),
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        }
    }

    fn valid_ig(name: &str, version: &str, url: &str, fhir_ver: &str) -> serde_json::Value {
        json!({
            "resourceType": "ImplementationGuide",
            "id": "test-ig",
            "packageId": name,
            "version": version,
            "url": url,
            "fhirVersion": [fhir_ver],
            "status": "draft"
        })
    }

    #[test]
    fn passes_when_all_fields_match() {
        let ctx = make_context(
            "example.fhir.pkg",
            "1.0.0",
            Some("http://example.org/fhir"),
            &["4.0.1"],
            valid_ig(
                "example.fhir.pkg",
                "1.0.0",
                "http://example.org/fhir/ImplementationGuide/test-ig",
                "4.0.1",
            ),
        );
        assert!(check_ig_sync(&ctx).is_ok());
    }

    #[test]
    fn fails_on_package_id_mismatch() {
        let ctx = make_context(
            "example.fhir.pkg",
            "1.0.0",
            None,
            &[],
            json!({
                "resourceType": "ImplementationGuide",
                "packageId": "wrong.package.id",
                "version": "1.0.0",
                "status": "draft"
            }),
        );
        let err = check_ig_sync(&ctx).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("packageId"),
            "expected packageId mismatch in: {msg}"
        );
    }

    #[test]
    fn fails_on_version_mismatch() {
        let ctx = make_context(
            "example.fhir.pkg",
            "2.0.0",
            None,
            &[],
            json!({
                "resourceType": "ImplementationGuide",
                "packageId": "example.fhir.pkg",
                "version": "1.0.0",
                "status": "draft"
            }),
        );
        let err = check_ig_sync(&ctx).unwrap_err();
        assert!(err.to_string().contains("version"));
    }

    #[test]
    fn warns_but_passes_on_url_mismatch() {
        let ctx = make_context(
            "example.fhir.pkg",
            "1.0.0",
            Some("http://correct.org/fhir"),
            &[],
            json!({
                "resourceType": "ImplementationGuide",
                "id": "test-ig",
                "packageId": "example.fhir.pkg",
                "version": "1.0.0",
                "url": "http://wrong.org/fhir",
                "status": "draft"
            }),
        );
        assert!(check_ig_sync(&ctx).is_ok());
    }

    #[test]
    fn fails_on_fhir_version_mismatch() {
        let ctx = make_context(
            "example.fhir.pkg",
            "1.0.0",
            None,
            &["4.0.1"],
            json!({
                "resourceType": "ImplementationGuide",
                "packageId": "example.fhir.pkg",
                "version": "1.0.0",
                "fhirVersion": ["5.0.0"],
                "status": "draft"
            }),
        );
        let err = check_ig_sync(&ctx).unwrap_err();
        assert!(err.to_string().contains("fhirVersion"));
    }

    #[test]
    fn collects_multiple_mismatches() {
        let ctx = make_context(
            "example.fhir.pkg",
            "2.0.0",
            Some("http://correct.org/fhir"),
            &["4.0.1"],
            json!({
                "resourceType": "ImplementationGuide",
                "id": "test-ig",
                "packageId": "wrong.id",
                "version": "1.0.0",
                "url": "http://wrong.org/fhir",
                "fhirVersion": ["5.0.0"],
                "status": "draft"
            }),
        );
        let err = check_ig_sync(&ctx).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("packageId"));
        assert!(msg.contains("version"));
        assert!(msg.contains("fhirVersion"));
    }
}
