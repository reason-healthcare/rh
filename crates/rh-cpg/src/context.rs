use std::sync::Arc;

use serde_json::Value;

use crate::resolver::ContentResolver;

pub struct ApplyContext {
    pub content_resolver: Arc<dyn ContentResolver>,
    /// Subject reference, e.g. "Patient/123".
    pub subject: String,
    pub encounter: Option<String>,
    pub practitioner: Option<String>,
    pub organization: Option<String>,
    /// FHIR Bundle of patient data (may be a collection bundle).
    pub data: Option<Value>,
}

impl ApplyContext {
    pub fn new(content_resolver: Arc<dyn ContentResolver>, subject: impl Into<String>) -> Self {
        Self {
            content_resolver,
            subject: subject.into(),
            encounter: None,
            practitioner: None,
            organization: None,
            data: None,
        }
    }

    pub fn data_resources(&self) -> Vec<Value> {
        self.data
            .as_ref()
            .map(|bundle| {
                bundle
                    .get("entry")
                    .and_then(Value::as_array)
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(|entry| entry.get("resource"))
                            .filter(|resource| resource.is_object())
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn resolve_context_resource(&self, reference: &str) -> Option<Value> {
        let canonical_reference = canonical_reference(reference);
        self.data_resources()
            .into_iter()
            .find(|resource| {
                matches_context_reference(
                    &canonical_reference,
                    resource.get("resourceType").and_then(Value::as_str),
                    resource.get("id").and_then(Value::as_str),
                )
            })
            .or_else(|| {
                self.content_resolver
                    .resolve_reference(reference)
                    .ok()
                    .flatten()
            })
    }
}

fn canonical_reference(reference: &str) -> String {
    reference
        .split(['?', '#'])
        .next()
        .unwrap_or(reference)
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/")
}

fn matches_context_reference(
    reference: &str,
    resource_type: Option<&str>,
    resource_id: Option<&str>,
) -> bool {
    let Some((resource_type, resource_id)) = resource_type.zip(resource_id) else {
        return false;
    };

    reference == format!("{resource_type}/{resource_id}")
}
