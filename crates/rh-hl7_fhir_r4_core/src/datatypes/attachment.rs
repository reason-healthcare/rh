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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "att-1",
                rh_foundation::Severity::Error,
                "If the Attachment has data, it SHALL have a contentType",
                "data.empty() or contentType.exists()",
            )
            .with_xpath("not(exists(f:data)) or exists(f:contentType)"),
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Attachment.contentType",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1",
        )
        .with_description("The mime type of an attachment. Any valid mime type is allowed.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Attachment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.extension", 0, None),
            rh_foundation::ElementCardinality::new("Attachment.contentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.data", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.size", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.hash", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Attachment.creation", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Attachment {
    fn resource_type(&self) -> &'static str {
        "Attachment"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Attachment")
    }
}
