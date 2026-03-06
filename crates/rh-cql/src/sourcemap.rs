use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDocument {
    pub id: String,
    pub uri: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElmNodeMeta {
    pub elm_node_id: String,
    pub elm_path: String,
    pub elm_kind: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MappingRole {
    Direct,
    ImplicitConversion,
    Desugared,
    Synthetic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceElmMapping {
    pub doc_id: String,
    pub span: SourceSpan,
    pub role: MappingRole,
    pub elm_node_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceMap {
    pub schema_version: String,
    pub options_fingerprint: String,
    pub source_documents: Vec<SourceDocument>,
    pub elm_node_metas: Vec<ElmNodeMeta>,
    pub mappings: Vec<SourceElmMapping>,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            schema_version: "rh-cql-sourcemap/v1".to_string(),
            options_fingerprint: "".to_string(),
            source_documents: Vec::new(),
            elm_node_metas: Vec::new(),
            mappings: Vec::new(),
        }
    }
}
