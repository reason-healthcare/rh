//! ELM annotation types for metadata and documentation.
//!
//! Annotations provide a mechanism to attach metadata to ELM elements,
//! including source locator information and custom tags.

use serde::{Deserialize, Serialize};

/// Annotation attached to an ELM element.
///
/// Annotations provide a way to attach metadata to any ELM element,
/// such as documentation, source mapping, or custom information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// The annotation tag type (e.g., "description", "author").
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<String>,

    /// Custom attributes on this annotation.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

/// Narrative content for documentation purposes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Narrative {
    /// The narrative text content (may contain XHTML).
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Tag element for CQL annotation pragmas.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// Tag name.
    pub name: String,

    /// Tag value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// CqlToElmInfo annotation with translator version information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CqlToElmInfo {
    /// Version of the CQL-to-ELM translator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator_version: Option<String>,

    /// Translation options used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator_options: Option<String>,
}

/// Error information from CQL translation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CqlToElmError {
    /// Error message.
    pub message: String,

    /// Error type/code.
    #[serde(rename = "errorType", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,

    /// Error severity (error, warning, info).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_severity: Option<String>,

    /// Start line in source CQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,

    /// Start character position in source CQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_char: Option<i32>,

    /// End line in source CQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// End character position in source CQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_char: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_serialization() {
        let annotation = Annotation {
            tag_type: Some("description".into()),
            attributes: None,
        };
        let json = serde_json::to_string(&annotation).unwrap();
        assert!(json.contains("\"type\":\"description\""));
    }

    #[test]
    fn test_tag_serialization() {
        let tag = Tag {
            name: "author".into(),
            value: Some("Test Author".into()),
        };
        let json = serde_json::to_string(&tag).unwrap();
        assert!(json.contains("\"name\":\"author\""));
        assert!(json.contains("\"value\":\"Test Author\""));
    }

    #[test]
    fn test_cql_to_elm_info_serialization() {
        let info = CqlToElmInfo {
            translator_version: Some("2.11.0".into()),
            translator_options: Some("EnableAnnotations".into()),
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"translatorVersion\":\"2.11.0\""));
    }

    #[test]
    fn test_cql_to_elm_error_serialization() {
        let error = CqlToElmError {
            message: "Undefined identifier".into(),
            error_type: Some("semantic".into()),
            error_severity: Some("error".into()),
            start_line: Some(10),
            start_char: Some(5),
            end_line: Some(10),
            end_char: Some(15),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"message\":\"Undefined identifier\""));
        assert!(json.contains("\"startLine\":10"));
    }
}
