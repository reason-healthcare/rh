use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Annotation
///
/// Base StructureDefinition for Annotation Type: A  text note which also  contains information about who made the statement and when.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Annotation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Annotation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Individual responsible for the annotation (Reference)
    #[serde(rename = "authorReference")]
    pub author_reference: Option<Reference>,
    /// Individual responsible for the annotation (string)
    #[serde(rename = "authorString")]
    pub author_string: Option<StringType>,
    /// When the annotation was made
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// The annotation  - text content (as markdown)
    pub text: StringType,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            base: Element::default(),
            author_reference: Default::default(),
            author_string: Default::default(),
            time: Default::default(),
            _time: Default::default(),
            text: StringType::default(),
            _text: Default::default(),
        }
    }
}
