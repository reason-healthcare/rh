use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::element::Element;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use serde::{Deserialize, Serialize};
/// Attachment
///
/// Base StructureDefinition for Attachment Type: For referring to data content defined in other formats.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Attachment
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Attachment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Mime type of the content, with charset etc.
    #[serde(rename = "contentType")]
    pub content_type: Option<Mimetypes>,
    /// Extension element for the 'contentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentType")]
    pub _content_type: Option<Element>,
    /// Human language of the content (BCP-47)
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `da`: Danish
    /// - `de`: German
    /// - `de-AT`: German (Austria)
    /// - `de-CH`: German (Switzerland)
    /// - `de-DE`: German (Germany)
    /// - `el`: Greek
    /// - `en`: English
    /// - ... and 46 more values
    pub language: Option<StringType>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Data inline, base64ed
    pub data: Option<Base64BinaryType>,
    /// Extension element for the 'data' primitive field. Contains metadata and extensions.
    pub _data: Option<Element>,
    /// Uri where the data can be found
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Number of bytes of content (if url provided)
    pub size: Option<UnsignedIntType>,
    /// Extension element for the 'size' primitive field. Contains metadata and extensions.
    pub _size: Option<Element>,
    /// Hash of the data (sha-1, base64ed)
    pub hash: Option<Base64BinaryType>,
    /// Extension element for the 'hash' primitive field. Contains metadata and extensions.
    pub _hash: Option<Element>,
    /// Label to display in place of the data
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Date attachment was first created
    pub creation: Option<DateTimeType>,
    /// Extension element for the 'creation' primitive field. Contains metadata and extensions.
    pub _creation: Option<Element>,
}

impl Default for Attachment {
    fn default() -> Self {
        Self {
            base: Element::default(),
            content_type: Default::default(),
            _content_type: Default::default(),
            language: Default::default(),
            _language: Default::default(),
            data: Default::default(),
            _data: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            size: Default::default(),
            _size: Default::default(),
            hash: Default::default(),
            _hash: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            creation: Default::default(),
            _creation: Default::default(),
        }
    }
}
