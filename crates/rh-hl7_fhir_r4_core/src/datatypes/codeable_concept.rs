use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// CodeableConcept
///
/// Base StructureDefinition for CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CodeableConcept
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: CodeableConcept
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Code defined by a terminology system
    pub coding: Option<Vec<Coding>>,
    /// Plain text representation of the concept
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
}

impl Default for CodeableConcept {
    fn default() -> Self {
        Self {
            base: Element::default(),
            coding: Default::default(),
            text: Default::default(),
            _text: Default::default(),
        }
    }
}
