//! Canonical URL helpers for package metadata and generated resources.

use crate::{context::PublishContext, PublisherError, Result};
use rh_hl7_fhir_r4_core::metadata::FHIR_TYPE_REGISTRY;
use serde_json::{json, Value};

const IMPLEMENTATION_GUIDE_SEGMENT: &str = "/ImplementationGuide/";

pub fn validate_canonical_base(canonical_base: Option<&str>) -> Result<()> {
    let Some(canonical_base) = canonical_base else {
        return Ok(());
    };

    if canonical_base.contains(IMPLEMENTATION_GUIDE_SEGMENT) {
        return Err(PublisherError::IgSync(format!(
            "canonical must be the package canonical base URL, not an ImplementationGuide URL: \
             \"{canonical_base}\". Use the base URL before /ImplementationGuide/."
        )));
    }

    Ok(())
}

pub fn implementation_guide_url(canonical_base: &str, ig_id: &str) -> String {
    resource_canonical_url(canonical_base, "ImplementationGuide", ig_id)
}

pub fn resource_canonical_url(canonical_base: &str, resource_type: &str, id: &str) -> String {
    format!(
        "{}/{}/{}",
        canonical_base.trim_end_matches('/'),
        resource_type,
        id
    )
}

pub fn is_canonical_resource_type(resource_type: &str) -> bool {
    FHIR_TYPE_REGISTRY
        .get(resource_type)
        .is_some_and(|fields| fields.contains_key("url"))
}

pub fn normalize_resource_canonical_urls(ctx: &mut PublishContext) {
    let Some(canonical_base) = ctx.package_json.url.as_deref() else {
        return;
    };

    for resource in ctx.resources.values_mut().chain(ctx.examples.values_mut()) {
        normalize_resource_canonical_url(resource, canonical_base);
    }
}

fn normalize_resource_canonical_url(resource: &mut Value, canonical_base: &str) {
    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return;
    };
    if !is_canonical_resource_type(resource_type) {
        return;
    }
    let Some(id) = resource.get("id").and_then(|v| v.as_str()) else {
        return;
    };

    let expected = if resource_type == "ImplementationGuide" {
        implementation_guide_url(canonical_base, id)
    } else {
        resource_canonical_url(canonical_base, resource_type, id)
    };

    let current = resource.get("url").and_then(|v| v.as_str());
    if current.is_none_or(|url| should_replace_url(url, canonical_base, resource_type, id)) {
        resource["url"] = json!(expected);
    }
}

fn should_replace_url(
    current_url: &str,
    canonical_base: &str,
    resource_type: &str,
    id: &str,
) -> bool {
    if current_url.is_empty() {
        return true;
    }

    let ig_prefix = format!(
        "{}/ImplementationGuide/",
        canonical_base.trim_end_matches('/')
    );
    current_url.starts_with(&ig_prefix) && current_url.ends_with(&format!("/{resource_type}/{id}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::PublisherConfig, context::PublishContext, manifest::PackageJson};
    use serde_json::json;
    use std::collections::HashMap;

    fn make_ctx(resources: HashMap<String, Value>) -> PublishContext {
        PublishContext {
            source_dir: std::path::PathBuf::from("/tmp/src"),
            input_dir: std::path::PathBuf::from("/tmp/src"),
            output_dir: std::path::PathBuf::from("/tmp/out"),
            package_json: PackageJson {
                name: "example.fhir".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec![],
                dependencies: HashMap::new(),
                url: Some("http://example.org/fhir".to_string()),
                canonical: Some("http://example.org/fhir".to_string()),
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
    fn rejects_implementation_guide_url_as_canonical_base() {
        let err = validate_canonical_base(Some(
            "http://example.org/fhir/ImplementationGuide/example.fhir",
        ))
        .unwrap_err();
        assert!(err.to_string().contains("canonical must be"));
    }

    #[test]
    fn canonical_resource_detection_uses_r4_url_metadata() {
        assert!(is_canonical_resource_type("PlanDefinition"));
        assert!(is_canonical_resource_type("ValueSet"));
        assert!(!is_canonical_resource_type("Patient"));
    }

    #[test]
    fn fills_missing_canonical_resource_url_from_base() {
        let mut resources = HashMap::new();
        resources.insert(
            "Library-logic".to_string(),
            json!({"resourceType":"Library","id":"logic","status":"draft"}),
        );
        let mut ctx = make_ctx(resources);

        normalize_resource_canonical_urls(&mut ctx);

        assert_eq!(
            ctx.resources["Library-logic"]["url"],
            "http://example.org/fhir/Library/logic"
        );
    }

    #[test]
    fn rewrites_nested_implementation_guide_resource_url() {
        let mut resources = HashMap::new();
        resources.insert(
            "ActivityDefinition-assess".to_string(),
            json!({
                "resourceType":"ActivityDefinition",
                "id":"assess",
                "url":"http://example.org/fhir/ImplementationGuide/example.fhir/ActivityDefinition/assess",
                "status":"draft"
            }),
        );
        let mut ctx = make_ctx(resources);

        normalize_resource_canonical_urls(&mut ctx);

        assert_eq!(
            ctx.resources["ActivityDefinition-assess"]["url"],
            "http://example.org/fhir/ActivityDefinition/assess"
        );
    }

    #[test]
    fn preserves_external_or_custom_resource_url() {
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-custom".to_string(),
            json!({
                "resourceType":"ValueSet",
                "id":"custom",
                "url":"http://custom.example/ValueSet/custom"
            }),
        );
        let mut ctx = make_ctx(resources);

        normalize_resource_canonical_urls(&mut ctx);

        assert_eq!(
            ctx.resources["ValueSet-custom"]["url"],
            "http://custom.example/ValueSet/custom"
        );
    }
}
