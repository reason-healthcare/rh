use std::collections::{HashMap, HashSet};

use serde_json::{json, Map, Value};

use crate::error::{CpgError, CpgResult};

/// Convert tolerant content input into a FHIR collection Bundle.
pub fn as_content_bundle(value: Value) -> CpgResult<Value> {
    if value.get("resourceType").and_then(Value::as_str) == Some("Bundle") {
        return Ok(value);
    }

    let resources = match value {
        Value::Array(resources) => resources,
        Value::Object(_) => vec![value],
        Value::Null => {
            return Err(CpgError::InvalidResource(
                "content JSON must contain resources".to_string(),
            ))
        }
        other => {
            return Err(CpgError::InvalidResource(format!(
                "content JSON must be a Bundle, resource object, or resource array; got {}",
                json_type_name(&other)
            )))
        }
    };

    let entries: Vec<Value> = resources
        .into_iter()
        .map(|resource| {
            let mut entry = Map::new();
            entry.insert("resource".to_string(), resource);
            Value::Object(entry)
        })
        .collect();

    Ok(json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": entries,
    }))
}

fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

pub trait ContentResolver: Send + Sync {
    /// Resolve a resource by canonical URL (e.g. "http://example.org/PlanDefinition/x|1.0").
    /// Version is optional; if given, match `url|version`; otherwise match `url` alone.
    fn resolve_canonical(&self, canonical: &str) -> CpgResult<Option<Value>>;
    /// Resolve a resource by reference (e.g. "Patient/123" or "http://server/Patient/123").
    fn resolve_reference(&self, reference: &str) -> CpgResult<Option<Value>>;
    /// Return all resources of a given resourceType (e.g. "Library"), optionally filtered by subject.
    fn all_by_type(&self, resource_type: &str) -> CpgResult<Vec<Value>>;
    /// Find a ValueSet by canonical URL.
    fn find_value_set(&self, url: &str) -> CpgResult<Option<Value>>;
}

#[derive(Debug, Default)]
pub struct BundleResolver {
    /// Resources keyed by their exact canonical identity. Versioned resources
    /// are only present as `url|version` so an unavailable requested version
    /// cannot silently resolve to a different artifact.
    canonical: HashMap<String, Value>,
    /// Unversioned lookup is available only when a URL identifies one resource.
    unversioned_canonical: HashMap<String, Value>,
    ambiguous_unversioned_canonical: HashSet<String>,
    references: HashMap<String, Value>,
    by_type: HashMap<String, Vec<Value>>,
}

impl BundleResolver {
    pub fn new(bundle: &Value) -> CpgResult<Self> {
        if bundle
            .as_object()
            .and_then(|object| object.get("resourceType"))
            .and_then(Value::as_str)
            != Some("Bundle")
        {
            return Err(CpgError::InvalidResource(
                "expected a FHIR Bundle object".to_string(),
            ));
        }

        let mut resolver = Self::default();
        for resource in bundle
            .get("entry")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.get("resource"))
            .filter(|resource| resource.is_object())
        {
            let Some(resource_type) = resource
                .get("resourceType")
                .and_then(Value::as_str)
                .map(str::to_string)
            else {
                continue;
            };

            if let Some(url) = resource.get("url").and_then(Value::as_str) {
                let exact_canonical = resource
                    .get("version")
                    .and_then(Value::as_str)
                    .map(|version| format!("{url}|{version}"))
                    .unwrap_or_else(|| url.to_string());
                if resolver
                    .canonical
                    .insert(exact_canonical.clone(), resource.clone())
                    .is_some()
                {
                    return Err(CpgError::InvalidResource(format!(
                        "duplicate canonical '{exact_canonical}' in content Bundle"
                    )));
                }

                if !resolver.ambiguous_unversioned_canonical.contains(url)
                    && resolver
                        .unversioned_canonical
                        .insert(url.to_string(), resource.clone())
                        .is_some()
                {
                    resolver.unversioned_canonical.remove(url);
                    resolver
                        .ambiguous_unversioned_canonical
                        .insert(url.to_string());
                }
            }

            if let Some(id) = resource.get("id").and_then(Value::as_str) {
                resolver
                    .references
                    .insert(format!("{resource_type}/{id}"), resource.clone());
            }

            resolver
                .by_type
                .entry(resource_type)
                .or_default()
                .push(resource.clone());
        }

        Ok(resolver)
    }
}

impl ContentResolver for BundleResolver {
    fn resolve_canonical(&self, canonical: &str) -> CpgResult<Option<Value>> {
        if canonical.contains('|') {
            return Ok(self.canonical.get(canonical).cloned());
        }

        if self.ambiguous_unversioned_canonical.contains(canonical) {
            return Err(CpgError::InvalidResource(format!(
                "ambiguous unversioned canonical '{canonical}' in content Bundle; request an explicit version"
            )));
        }

        Ok(self.unversioned_canonical.get(canonical).cloned())
    }

    fn resolve_reference(&self, reference: &str) -> CpgResult<Option<Value>> {
        let normalized = normalize_reference(reference);
        Ok(self.references.get(&normalized).cloned())
    }

    fn all_by_type(&self, resource_type: &str) -> CpgResult<Vec<Value>> {
        Ok(self.by_type.get(resource_type).cloned().unwrap_or_default())
    }

    fn find_value_set(&self, url: &str) -> CpgResult<Option<Value>> {
        self.resolve_canonical(url)
    }
}

fn normalize_reference(reference: &str) -> String {
    let reference = reference.split(['?', '#']).next().unwrap_or(reference);
    let mut parts = reference
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if parts.len() >= 2 {
        parts.drain(..parts.len() - 2);
    }
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn bundle() -> Value {
        json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {
                    "resourceType": "Library",
                    "id": "library-1",
                    "url": "http://example.org/Library/library-1",
                    "version": "1.0"
                }},
                {"resource": {
                    "resourceType": "Patient",
                    "id": "123"
                }},
                {"resource": {
                    "resourceType": "ValueSet",
                    "id": "valueset-1",
                    "url": "http://example.org/ValueSet/valueset-1"
                }}
            ]
        })
    }

    fn resolver() -> BundleResolver {
        BundleResolver::new(&bundle()).expect("valid bundle")
    }

    #[test]
    fn resolves_canonical_exact_match() {
        let resolver = resolver();
        let resource = resolver
            .resolve_canonical("http://example.org/Library/library-1")
            .unwrap();
        assert_eq!(resource.unwrap().get("id").unwrap(), "library-1");
    }

    #[test]
    fn resolves_canonical_with_version_suffix() {
        let resolver = resolver();
        let resource = resolver
            .resolve_canonical("http://example.org/Library/library-1|1.0")
            .unwrap();
        assert_eq!(resource.unwrap().get("id").unwrap(), "library-1");
    }

    #[test]
    fn explicit_missing_version_does_not_fallback_to_unversioned_resource() {
        let resolver = resolver();
        let resource = resolver
            .resolve_canonical("http://example.org/ValueSet/valueset-1|1.0")
            .unwrap();
        assert!(resource.is_none());
    }

    #[test]
    fn constructor_rejects_duplicate_canonical_and_version() {
        let bundle = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {"resourceType": "Library", "url": "http://example.org/Library/x", "version": "1.0"}},
                {"resource": {"resourceType": "Library", "url": "http://example.org/Library/x", "version": "1.0"}}
            ]
        });

        let error = BundleResolver::new(&bundle).unwrap_err();
        assert!(
            matches!(error, CpgError::InvalidResource(message) if message.contains("duplicate canonical"))
        );
    }

    #[test]
    fn multiple_versions_require_an_explicit_version() {
        let bundle = json!({
            "resourceType": "Bundle",
            "entry": [
                {"resource": {"resourceType": "Library", "id": "one", "url": "http://example.org/Library/x", "version": "1.0"}},
                {"resource": {"resourceType": "Library", "id": "two", "url": "http://example.org/Library/x", "version": "2.0"}}
            ]
        });
        let resolver = BundleResolver::new(&bundle).expect("valid versioned resources");

        assert!(matches!(
            resolver.resolve_canonical("http://example.org/Library/x"),
            Err(CpgError::InvalidResource(message)) if message.contains("ambiguous unversioned canonical")
        ));
        let resource = resolver
            .resolve_canonical("http://example.org/Library/x|2.0")
            .unwrap()
            .expect("exact version");
        assert_eq!(resource.get("id").unwrap(), "two");
    }

    #[test]
    fn resolves_relative_reference() {
        let resolver = resolver();
        let resource = resolver.resolve_reference("Patient/123").unwrap();
        assert_eq!(resource.unwrap().get("id").unwrap(), "123");
    }

    #[test]
    fn resolves_absolute_reference() {
        let resolver = resolver();
        let resource = resolver
            .resolve_reference("https://server.example.org/Patient/123")
            .unwrap();
        assert_eq!(resource.unwrap().get("id").unwrap(), "123");
    }

    #[test]
    fn returns_all_by_type() {
        let resolver = resolver();
        let libraries = resolver.all_by_type("Library").unwrap();
        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].get("id").unwrap(), "library-1");
    }

    #[test]
    fn finds_value_set_by_url() {
        let resolver = resolver();
        let value_set = resolver
            .find_value_set("http://example.org/ValueSet/valueset-1")
            .unwrap();
        assert_eq!(value_set.unwrap().get("id").unwrap(), "valueset-1");
    }

    #[test]
    fn misses_return_none_or_empty_without_error() {
        let resolver = resolver();
        assert!(resolver
            .resolve_canonical("http://example.org/missing")
            .unwrap()
            .is_none());
        assert!(resolver
            .resolve_reference("Observation/missing")
            .unwrap()
            .is_none());
        assert!(resolver.all_by_type("Organization").unwrap().is_empty());
        assert!(resolver
            .find_value_set("http://example.org/missing")
            .unwrap()
            .is_none());
    }

    #[test]
    fn constructor_rejects_non_bundle() {
        let error = BundleResolver::new(&json!({"resourceType": "Patient"})).unwrap_err();
        assert!(matches!(error, CpgError::InvalidResource(_)));
    }
}
