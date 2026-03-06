//! Source-map infrastructure for correlating CQL source text with ELM nodes.
//!
//! See PLAN.md §6.1 for the full schema specification.

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ---------------------------------------------------------------------------
// Location primitives
// ---------------------------------------------------------------------------

/// A single point in a source document (line/column are 1-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    /// Byte offset from the start of the document.
    pub offset: usize,
}

/// A contiguous span of source text (start inclusive, end exclusive).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

// ---------------------------------------------------------------------------
// Source document
// ---------------------------------------------------------------------------

/// Metadata about a CQL source document.
///
/// `doc_id` is the stable identifier used as the foreign key across the map.
/// It is produced by [`generate_doc_id`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDocument {
    /// Stable identifier for this document (hash-based).
    pub doc_id: String,
    /// Canonical URI / library identifier of the document.
    pub uri: String,
    /// Optional SHA-256 (or similar) checksum of the normalised source text.
    pub checksum: Option<String>,
    /// Optional compressed line-start byte offsets (for fast span look-ups).
    pub line_index: Option<Vec<u32>>,
}

// ---------------------------------------------------------------------------
// ELM node metadata
// ---------------------------------------------------------------------------

/// Metadata about a single ELM node produced during emission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElmNodeMeta {
    /// Stable identifier for this ELM node.
    pub elm_node_id: String,
    /// JSON-pointer-like path within the ELM document.
    pub elm_path: String,
    /// ELM kind string (e.g. `"Equal"`, `"Query"`, `"FunctionRef"`).
    pub elm_kind: String,
    /// `elm_node_id` of the containing node, if any.
    pub parent_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Mapping
// ---------------------------------------------------------------------------

/// The semantic role of a CQL-span → ELM-node mapping entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MappingRole {
    /// The ELM node directly represents the CQL span.
    Direct,
    /// The ELM node was inserted as an implicit type-conversion.
    ImplicitConversion,
    /// The ELM node is a desugared expansion of syntactic sugar.
    Desugared,
    /// The ELM node is synthetic (no direct CQL counterpart).
    Synthetic,
}

/// How precisely a mapping was determined.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    /// The mapping is exact — CQL span and ELM node correspond 1-to-1.
    Exact,
    /// The mapping is an approximation (e.g. span from a macro expansion).
    Approximate,
}

/// A mapping entry correlating one CQL span with one or more ELM nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceElmMapping {
    /// Stable identifier for this mapping entry.
    pub mapping_id: String,
    /// `doc_id` of the [`SourceDocument`] that contains the CQL span.
    pub doc_id: String,
    /// The CQL source span.
    pub span: SourceSpan,
    /// Semantic role of the mapping.
    pub role: MappingRole,
    /// ELM node IDs that this CQL span maps to.
    pub elm_node_ids: Vec<String>,
    /// Precision of the mapping.
    pub confidence: Option<Confidence>,
    /// Optional free-text note for debugging.
    pub note: Option<String>,
}

// ---------------------------------------------------------------------------
// SourceMap
// ---------------------------------------------------------------------------

/// The complete source map produced by one compilation run.
///
/// Serializes to / deserializes from `*.elm.sourcemap.json` sidecar files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceMap {
    /// Schema version — always `"rh-cql-sourcemap/v1"`.
    pub schema_version: String,
    /// Fingerprint of the [`CompilerOptions`] that produced this map.
    pub options_fingerprint: String,
    /// Source documents referenced by this map.
    pub source_documents: Vec<SourceDocument>,
    /// Metadata for every ELM node that was emitted.
    pub elm_node_metas: Vec<ElmNodeMeta>,
    /// Span ↔ ELM-node correlation entries.
    pub mappings: Vec<SourceElmMapping>,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceMap {
    /// Create an empty source map with the current schema version.
    pub fn new() -> Self {
        Self {
            schema_version: "rh-cql-sourcemap/v1".to_string(),
            options_fingerprint: String::new(),
            source_documents: Vec::new(),
            elm_node_metas: Vec::new(),
            mappings: Vec::new(),
        }
    }

    /// Serialize the source map to a pretty-printed JSON string.
    ///
    /// This is the format written to `*.elm.sourcemap.json` sidecar files.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a source map from a JSON string (sidecar file content).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// ---------------------------------------------------------------------------
// Stable ID generation (task 7.2)
// ---------------------------------------------------------------------------

/// Generate a stable, hex-prefixed identifier by hashing the given inputs.
///
/// Uses [`DefaultHasher`] — suitable for within-compilation correlation.
/// For cross-version persistence, replace with a cryptographic hash.
fn hash_inputs(inputs: &[&str]) -> String {
    let mut hasher = DefaultHasher::new();
    for s in inputs {
        s.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

/// Produce a stable `doc_id` for a CQL source document.
///
/// The ID is derived from the normalised library identifier, version, and URI.
/// Pass empty strings for absent fields.
pub fn generate_doc_id(library_id: &str, version: &str, uri: &str) -> String {
    format!("doc:{}", &hash_inputs(&[library_id, version, uri])[..8])
}

/// Produce a stable `elm_node_id` for an ELM node.
///
/// The ID is derived from the ELM kind, the JSON-pointer path in the ELM
/// document, and an optional operator signature.
pub fn generate_elm_node_id(elm_kind: &str, elm_path: &str, operator_sig: &str) -> String {
    format!("elm:{}", &hash_inputs(&[elm_kind, elm_path, operator_sig])[..6])
}

/// Produce a stable `mapping_id` for a [`SourceElmMapping`] entry.
///
/// The ID is derived from `doc_id`, span bytes, sorted ELM node IDs, and role.
pub fn generate_mapping_id(
    doc_id: &str,
    start_byte: usize,
    end_byte: usize,
    elm_node_ids: &[String],
    role: &str,
) -> String {
    let byte_range = format!("{}:{}", start_byte, end_byte);
    let mut sorted_ids = elm_node_ids.to_vec();
    sorted_ids.sort();
    let ids_str = sorted_ids.join(",");
    format!("map:{}", &hash_inputs(&[doc_id, &byte_range, &ids_str, role])[..6])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_map() -> SourceMap {
        let doc_id = generate_doc_id("MyLib", "1.0.0", "library:MyLib:1.0.0");
        let elm_node_id = generate_elm_node_id("Equal", "/statements/def[0]/expression", "");
        let mapping_id = generate_mapping_id(&doc_id, 312, 327, &[elm_node_id.clone()], "direct");

        let mut map = SourceMap::new();
        map.source_documents.push(SourceDocument {
            doc_id: doc_id.clone(),
            uri: "library:MyLib:1.0.0".to_string(),
            checksum: Some("sha256:abc123".to_string()),
            line_index: None,
        });
        map.elm_node_metas.push(ElmNodeMeta {
            elm_node_id: elm_node_id.clone(),
            elm_path: "/statements/def[0]/expression".to_string(),
            elm_kind: "Equal".to_string(),
            parent_id: None,
        });
        map.mappings.push(SourceElmMapping {
            mapping_id,
            doc_id,
            span: SourceSpan {
                start: SourceLocation { line: 14, column: 9, offset: 312 },
                end: SourceLocation { line: 14, column: 24, offset: 327 },
            },
            role: MappingRole::Direct,
            elm_node_ids: vec![elm_node_id],
            confidence: Some(Confidence::Exact),
            note: None,
        });
        map
    }

    // Task 7.7 — JSON round-trip
    #[test]
    fn test_sourcemap_json_round_trip() {
        let original = sample_map();
        let json = original.to_json().expect("serialization failed");

        // JSON must contain expected values
        assert!(json.contains("rh-cql-sourcemap/v1"));
        assert!(json.contains("doc:"));
        assert!(json.contains("elm:"));
        assert!(json.contains("map:"));
        assert!(json.contains("direct"));
        assert!(json.contains("exact"));

        // Round-trip must be lossless
        let recovered = SourceMap::from_json(&json).expect("deserialization failed");
        assert_eq!(original, recovered);
    }

    // Task 7.7 — empty map round-trip
    #[test]
    fn test_empty_sourcemap_round_trip() {
        let map = SourceMap::new();
        let json = map.to_json().unwrap();
        let recovered = SourceMap::from_json(&json).unwrap();
        assert_eq!(map, recovered);
    }

    // Task 7.2 — stable ID generation
    #[test]
    fn test_generate_doc_id_stability() {
        let id1 = generate_doc_id("MyLib", "1.0.0", "library:MyLib:1.0.0");
        let id2 = generate_doc_id("MyLib", "1.0.0", "library:MyLib:1.0.0");
        assert_eq!(id1, id2, "doc_id must be stable");
        assert!(id1.starts_with("doc:"), "doc_id must have 'doc:' prefix");
    }

    #[test]
    fn test_generate_elm_node_id_stability() {
        let id1 = generate_elm_node_id("Equal", "/statements/def[0]/expression", "");
        let id2 = generate_elm_node_id("Equal", "/statements/def[0]/expression", "");
        assert_eq!(id1, id2, "elm_node_id must be stable");
        assert!(id1.starts_with("elm:"), "elm_node_id must have 'elm:' prefix");
    }

    #[test]
    fn test_generate_mapping_id_stability() {
        let doc_id = generate_doc_id("L", "1", "uri");
        let elm_ids = vec![generate_elm_node_id("Equal", "/path", "")];
        let id1 = generate_mapping_id(&doc_id, 10, 20, &elm_ids, "direct");
        let id2 = generate_mapping_id(&doc_id, 10, 20, &elm_ids, "direct");
        assert_eq!(id1, id2, "mapping_id must be stable");
        assert!(id1.starts_with("map:"));
    }

    #[test]
    fn test_different_inputs_produce_different_ids() {
        let id1 = generate_doc_id("LibA", "1.0", "uri");
        let id2 = generate_doc_id("LibB", "1.0", "uri");
        assert_ne!(id1, id2);
    }

    // Task 7.3 — role variant serialization
    #[test]
    fn test_mapping_role_serialization() {
        let cases = [
            (MappingRole::Direct, "direct"),
            (MappingRole::ImplicitConversion, "implicit-conversion"),
            (MappingRole::Desugared, "desugared"),
            (MappingRole::Synthetic, "synthetic"),
        ];
        for (role, expected) in cases {
            let json = serde_json::to_string(&role).unwrap();
            assert_eq!(json, format!("\"{expected}\""));
            let recovered: MappingRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, recovered);
        }
    }

    // Task 7.8 — CQL span → ELM node correlation sanity check
    #[test]
    fn test_span_elm_correlation() {
        let map = sample_map();

        // There must be exactly one mapping
        assert_eq!(map.mappings.len(), 1);
        let m = &map.mappings[0];

        // The mapping must reference a known ELM node
        assert!(!m.elm_node_ids.is_empty());
        let elm_id = &m.elm_node_ids[0];
        let meta = map.elm_node_metas.iter().find(|n| &n.elm_node_id == elm_id);
        assert!(meta.is_some(), "ELM node referenced by mapping must exist in elm_node_metas");

        // Span offsets must be consistent
        assert!(m.span.start.offset < m.span.end.offset);

        // The doc_id must reference a known source document
        let doc = map.source_documents.iter().find(|d| d.doc_id == m.doc_id);
        assert!(doc.is_some(), "doc_id in mapping must reference a source document");
    }
}
