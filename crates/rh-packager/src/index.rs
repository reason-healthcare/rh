//! Generates `.index.json` per the FHIR Package Specification (index-version 2).

use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A single resource entry in the package index.
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
    let url = value
        .get("url")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let version = value
        .get("version")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    Some(IndexEntry {
        filename: format!("{stem}.json"),
        resource_type,
        id,
        url,
        version,
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
    fn serializes_with_index_version_field() {
        let idx = build_index(&HashMap::new()).unwrap();
        let json_str = serde_json::to_string(&idx).unwrap();
        assert!(json_str.contains("index-version"));
        assert!(json_str.contains("\"2\"") || json_str.contains(":2"));
    }
}
