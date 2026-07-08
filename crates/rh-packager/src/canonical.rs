//! Canonical URL helpers for package metadata and generated resources.

use crate::context::PublishContext;
use serde_json::Value;
use tracing::warn;

const IMPLEMENTATION_GUIDE_SEGMENT: &str = "/ImplementationGuide/";

const CANONICAL_RESOURCE_TYPES: &[&str] = &[
    "ActivityDefinition",
    "CapabilityStatement",
    "CodeSystem",
    "CompartmentDefinition",
    "ConceptMap",
    "GraphDefinition",
    "ImplementationGuide",
    "Library",
    "Measure",
    "MessageDefinition",
    "NamingSystem",
    "OperationDefinition",
    "PlanDefinition",
    "Questionnaire",
    "SearchParameter",
    "StructureDefinition",
    "TerminologyCapabilities",
    "ValueSet",
];

fn likely_implementation_guide_resource_url(canonical: &str) -> Option<String> {
    let canonical = canonical.trim_end_matches('/');
    let (base, ig_id) = canonical.split_once(IMPLEMENTATION_GUIDE_SEGMENT)?;

    if base.is_empty() || ig_id.is_empty() || ig_id.contains('/') {
        return None;
    }

    Some(base.to_string())
}

pub(crate) fn warn_if_likely_implementation_guide_resource_url(canonical: Option<&str>) {
    let Some(canonical) = canonical else {
        return;
    };
    let Some(suggested_base) = likely_implementation_guide_resource_url(canonical) else {
        return;
    };

    warn!(
        canonical,
        suggested_base,
        "packager.toml canonical looks like an ImplementationGuide resource URL; \
         it will be used literally as the package canonical base. Did you mean \"{}\"?",
        suggested_base
    );
}

pub(crate) fn implementation_guide_url(canonical_base: &str, ig_id: &str) -> String {
    resource_canonical_url(canonical_base, "ImplementationGuide", ig_id)
}

pub(crate) fn resource_canonical_url(
    canonical_base: &str,
    resource_type: &str,
    id: &str,
) -> String {
    format!(
        "{}/{}/{}",
        canonical_base.trim_end_matches('/'),
        resource_type,
        id
    )
}

fn is_canonical_resource_type(resource_type: &str) -> bool {
    CANONICAL_RESOURCE_TYPES.contains(&resource_type)
}

pub(crate) fn warn_resource_canonical_url_mismatches(ctx: &PublishContext) {
    let Some(canonical_base) = ctx.package_json.url.as_deref() else {
        return;
    };

    for (label, resource) in ctx.resources.iter().chain(ctx.examples.iter()) {
        warn_resource_canonical_url_mismatch(label, resource, canonical_base);
    }
}

fn warn_resource_canonical_url_mismatch(label: &str, resource: &Value, canonical_base: &str) {
    let Some(resource_type) = resource.get("resourceType").and_then(|v| v.as_str()) else {
        return;
    };
    if resource_type == "ImplementationGuide" || !is_canonical_resource_type(resource_type) {
        return;
    }
    let Some(id) = resource.get("id").and_then(|v| v.as_str()) else {
        return;
    };
    let Some(actual_url) = resource.get("url").and_then(|v| v.as_str()) else {
        return;
    };

    let expected_url = resource_canonical_url(canonical_base, resource_type, id);
    if actual_url != expected_url {
        warn!(
            resource = label,
            resource_type,
            id,
            actual_url,
            expected_url,
            "Canonical resource url differs from value derived from packager.toml canonical"
        );
    }
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
    fn detects_likely_implementation_guide_url_canonical_base() {
        assert_eq!(
            likely_implementation_guide_resource_url(
                "http://example.org/fhir/ImplementationGuide/example.fhir"
            )
            .as_deref(),
            Some("http://example.org/fhir")
        );
    }

    #[test]
    fn does_not_warn_for_deeper_implementation_guide_path_canonical_base() {
        assert_eq!(
            likely_implementation_guide_resource_url(
                "http://example.org/fhir/ImplementationGuide/root/ns"
            ),
            None
        );
    }

    #[test]
    fn ignores_missing_non_ig_canonical_resource_url() {
        let mut resources = HashMap::new();
        resources.insert(
            "Library-logic".to_string(),
            json!({"resourceType":"Library","id":"logic","status":"draft"}),
        );
        let ctx = make_ctx(resources);

        warn_resource_canonical_url_mismatches(&ctx);
    }

    #[test]
    fn warns_for_nested_implementation_guide_resource_url() {
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
        let ctx = make_ctx(resources);

        warn_resource_canonical_url_mismatches(&ctx);
    }

    #[test]
    fn validates_against_implementation_guide_path_canonical_base_literally() {
        let mut resources = HashMap::new();
        resources.insert(
            "ActivityDefinition-assess".to_string(),
            json!({
                "resourceType":"ActivityDefinition",
                "id":"assess",
                "url":"http://example.org/fhir/ImplementationGuide/root/ActivityDefinition/assess",
                "status":"draft"
            }),
        );
        let mut ctx = make_ctx(resources);
        ctx.package_json.url = Some("http://example.org/fhir/ImplementationGuide/root".to_string());
        warn_resource_canonical_url_mismatches(&ctx);
    }

    #[test]
    fn warns_for_external_or_custom_url_for_own_canonical_resource() {
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-custom".to_string(),
            json!({
                "resourceType":"ValueSet",
                "id":"custom",
                "url":"http://custom.example/ValueSet/custom"
            }),
        );
        let ctx = make_ctx(resources);

        warn_resource_canonical_url_mismatches(&ctx);
    }
}
