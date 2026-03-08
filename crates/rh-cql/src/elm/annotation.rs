//! ELM annotation types for metadata and documentation.
//!
//! Annotations provide a mechanism to attach metadata to ELM elements,
//! including source locator information and custom tags.
//!
//! The primary annotation structure uses a recursive "s" (source) narrative
//! that captures the original CQL source text with references to localIds.

use serde::{Deserialize, Serialize};

/// Annotation attached to an ELM element.
///
/// Annotations provide a way to attach metadata to any ELM element,
/// such as documentation, source mapping, or custom information.
///
/// When EnableAnnotations is set, annotations contain an "s" field with
/// nested Narrative structures that capture the original CQL source text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// The annotation tag type (always "Annotation" for source annotations).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<String>,

    /// Source narrative structure containing original CQL text.
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub source: Option<Narrative>,
}

impl Annotation {
    /// Creates a source annotation with the given narrative.
    pub fn source(narrative: Narrative) -> Self {
        Self {
            tag_type: Some("Annotation".to_string()),
            source: Some(narrative),
        }
    }
}

/// Narrative content representing source CQL text.
///
/// The narrative structure is recursive, allowing nested segments
/// to represent different parts of the source text with their
/// associated localIds.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Narrative {
    /// Reference to the localId of the associated ELM element.
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub local_id_ref: Option<String>,

    /// Nested narrative segments.
    #[serde(rename = "s", default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<NarrativeSegment>,
}

impl Narrative {
    /// Creates a narrative with a localId reference.
    pub fn with_ref(local_id: impl Into<String>) -> Self {
        Self {
            local_id_ref: Some(local_id.into()),
            segments: Vec::new(),
        }
    }

    /// Creates a narrative with segments.
    pub fn with_segments(segments: Vec<NarrativeSegment>) -> Self {
        Self {
            local_id_ref: None,
            segments,
        }
    }

    /// Creates a narrative with a localId reference and segments.
    pub fn new(local_id: impl Into<String>, segments: Vec<NarrativeSegment>) -> Self {
        Self {
            local_id_ref: Some(local_id.into()),
            segments,
        }
    }
}

/// A segment within a narrative, either text values or nested narrative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NarrativeSegment {
    /// Text values - an array of string fragments.
    Value {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        value: Vec<String>,
    },
    /// Nested narrative with its own localId reference.
    Nested {
        #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
        local_id_ref: Option<String>,
        #[serde(rename = "s", default, skip_serializing_if = "Vec::is_empty")]
        segments: Vec<NarrativeSegment>,
    },
}

impl NarrativeSegment {
    /// Creates a value segment with the given text fragments.
    pub fn value(fragments: Vec<impl Into<String>>) -> Self {
        Self::Value {
            value: fragments.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates a nested segment with a localId reference.
    pub fn nested(local_id: impl Into<String>, segments: Vec<NarrativeSegment>) -> Self {
        Self::Nested {
            local_id_ref: Some(local_id.into()),
            segments,
        }
    }

    /// Creates a nested segment without a localId reference.
    pub fn nested_anonymous(segments: Vec<NarrativeSegment>) -> Self {
        Self::Nested {
            local_id_ref: None,
            segments,
        }
    }
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
    /// Type discriminator for the annotation.
    /// Always "CqlToElmInfo" for this type.
    #[serde(rename = "type", default = "CqlToElmInfo::type_name")]
    pub annotation_type: String,

    /// Version of the CQL-to-ELM translator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator_version: Option<String>,

    /// Translation options used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator_options: Option<String>,
}

impl CqlToElmInfo {
    /// Returns the type name for this annotation.
    pub fn type_name() -> String {
        "CqlToElmInfo".to_string()
    }

    /// Create a new CqlToElmInfo with required fields.
    pub fn new() -> Self {
        Self {
            annotation_type: Self::type_name(),
            translator_version: None,
            translator_options: None,
        }
    }
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
    fn test_source_annotation_serialization() {
        let annotation = Annotation::source(Narrative::new(
            "206",
            vec![NarrativeSegment::value(vec!["", "using ", "FHIR"])],
        ));
        let json = serde_json::to_string(&annotation).unwrap();
        assert!(json.contains("\"type\":\"Annotation\""));
        assert!(json.contains("\"r\":\"206\""));
        assert!(json.contains("\"value\":["));
    }

    #[test]
    fn test_narrative_with_nested_segments() {
        let narrative = Narrative::new(
            "208",
            vec![
                NarrativeSegment::value(vec!["", "include "]),
                NarrativeSegment::nested_anonymous(vec![NarrativeSegment::value(vec![
                    "FHIRHelpers",
                ])]),
                NarrativeSegment::value(vec![" version ", "'4.0.1'"]),
            ],
        );
        let json = serde_json::to_string(&narrative).unwrap();
        assert!(json.contains("\"r\":\"208\""));
        assert!(json.contains("\"include \""));
        assert!(json.contains("\"FHIRHelpers\""));
    }

    #[test]
    fn test_narrative_segment_value() {
        let segment = NarrativeSegment::value(vec!["define ", "\"TestDef\""]);
        let json = serde_json::to_string(&segment).unwrap();
        assert!(json.contains("\"value\":[\"define \",\"\\\"TestDef\\\"\"]"));
    }

    #[test]
    fn test_narrative_segment_nested() {
        let segment =
            NarrativeSegment::nested("10", vec![NarrativeSegment::value(vec!["Patient"])]);
        let json = serde_json::to_string(&segment).unwrap();
        assert!(json.contains("\"r\":\"10\""));
        assert!(json.contains("\"s\":["));
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
        let mut info = CqlToElmInfo::new();
        info.translator_version = Some("2.11.0".into());
        info.translator_options = Some("EnableAnnotations".into());
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"type\":\"CqlToElmInfo\""));
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

    #[test]
    fn test_deserialize_reference_annotation() {
        let json = r#"{"type":"Annotation","s":{"r":"206","s":[{"value":["","using "]},{"s":[{"value":["FHIR"]}]},{"value":[" version '4.0.1'"]}]}}"#;
        let annotation: Annotation = serde_json::from_str(json).unwrap();
        assert_eq!(annotation.tag_type, Some("Annotation".to_string()));
        let source = annotation.source.unwrap();
        assert_eq!(source.local_id_ref, Some("206".to_string()));
        assert_eq!(source.segments.len(), 3);
    }
}
