//! Generates `.index.json` per the FHIR Package Specification (index-version 2).

use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A single resource entry in the package index.
///
/// All optional fields are populated directly from the matching string property of the
/// resource, irrespective of resource type, as specified in the FHIR Package spec §2.1.10.5.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexEntry {
    pub filename: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub resource_kind_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// The full `.index.json` document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageIndex {
    #[serde(rename = "index-version")]
    pub index_version: u32,
    pub files: Vec<IndexEntry>,
}

/// Build a `PackageIndex` from the provided resource map.
///
/// Call once for definitional resources (`package/*.json`) and optionally again
/// for example resources (`package/examples/*.json`) to get separate index documents.
pub fn build_index(resources: &HashMap<String, Value>) -> Result<PackageIndex> {
    let mut files: Vec<IndexEntry> = resources
        .iter()
        .filter_map(|(stem, value)| entry_from_resource(stem, value))
        .collect();

    // Stable ordering for reproducible output.
    files.sort_by(|a, b| a.filename.cmp(&b.filename));

    Ok(PackageIndex {
        index_version: 2,
        files,
    })
}

fn entry_from_resource(stem: &str, value: &Value) -> Option<IndexEntry> {
    let resource_type = value.get("resourceType")?.as_str()?.to_string();
    let id = value.get("id")?.as_str()?.to_string();

    let opt_str = |key: &str| -> Option<String> {
        value.get(key).and_then(|v| v.as_str()).map(str::to_string)
    };

    Some(IndexEntry {
        filename: format!("{stem}.json"),
        resource_type,
        id,
        url: opt_str("url"),
        version: opt_str("version"),
        kind: opt_str("kind"),
        resource_kind_type: opt_str("type"),
        supplements: opt_str("supplements"),
        content: opt_str("content"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn index_version_is_2() {
        let idx = build_index(&HashMap::new()).unwrap();
        assert_eq!(idx.index_version, 2);
    }

    #[test]
    fn generates_entry_for_each_resource() {
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","id":"foo","url":"http://example.org/foo","version":"1.0.0"}),
        );
        resources.insert(
            "ValueSet-bar".to_string(),
            json!({"resourceType":"ValueSet","id":"bar"}),
        );
        let idx = build_index(&resources).unwrap();
        assert_eq!(idx.files.len(), 2);
    }

    #[test]
    fn entry_filename_uses_stem_plus_json() {
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType":"StructureDefinition","id":"foo"}),
        );
        let idx = build_index(&resources).unwrap();
        assert_eq!(idx.files[0].filename, "StructureDefinition-foo.json");
    }

    #[test]
    fn url_and_version_are_optional() {
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-bar".to_string(),
            json!({"resourceType":"ValueSet","id":"bar"}),
        );
        let idx = build_index(&resources).unwrap();
        assert!(idx.files[0].url.is_none());
        assert!(idx.files[0].version.is_none());
    }

    #[test]
    fn captures_kind_type_supplements_content() {
        let mut resources = HashMap::new();
        resources.insert(
            "CodeSystem-cs".to_string(),
            json!({
                "resourceType": "CodeSystem",
                "id": "cs",
                "kind": "code-system",
                "type": "example",
                "supplements": "http://example.org/base",
                "content": "complete"
            }),
        );
        let idx = build_index(&resources).unwrap();
        let entry = &idx.files[0];
        assert_eq!(entry.kind.as_deref(), Some("code-system"));
        assert_eq!(entry.resource_kind_type.as_deref(), Some("example"));
        assert_eq!(entry.supplements.as_deref(), Some("http://example.org/base"));
        assert_eq!(entry.content.as_deref(), Some("complete"));
    }

    #[test]
    fn kind_type_supplements_content_absent_when_not_in_resource() {
        let mut resources = HashMap::new();
        resources.insert(
            "ValueSet-vs".to_string(),
            json!({"resourceType":"ValueSet","id":"vs","url":"http://example.org/vs"}),
        );
        let idx = build_index(&resources).unwrap();
        let entry = &idx.files[0];
        assert!(entry.kind.is_none());
        assert!(entry.resource_kind_type.is_none());
        assert!(entry.supplements.is_none());
        assert!(entry.content.is_none());
    }

    #[test]
    fn type_field_serializes_as_type_not_resource_kind_type() {
        let mut resources = HashMap::new();
        resources.insert(
            "CodeSystem-cs".to_string(),
            json!({"resourceType":"CodeSystem","id":"cs","type":"complete"}),
        );
        let idx = build_index(&resources).unwrap();
        let json_str = serde_json::to_string(&idx).unwrap();
        assert!(json_str.contains("\"type\""));
        assert!(!json_str.contains("resource_kind_type"));
    }

    #[test]
    fn serializes_with_index_version_field() {
        let idx = build_index(&HashMap::new()).unwrap();
        let json_str = serde_json::to_string(&idx).unwrap();
        assert!(json_str.contains("index-version"));
        assert!(json_str.contains("\"2\"") || json_str.contains(":2"));
    }
}
