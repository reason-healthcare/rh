use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Reference
///
/// Base StructureDefinition for Reference Type: A reference from one resource to another.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Reference
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Reference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    pub _reference: Option<Element>,
    /// Type the reference refers to (e.g. "Patient")
    ///
    /// Binding: extensible (Aa resource (or, for logical models, the URI of the logical model).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/resource-types
    #[serde(rename = "type")]
    pub type_: Option<StringType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Logical reference, when literal reference is not known
    pub identifier: Option<Box<Identifier>>,
    /// Text alternative for the resource
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            base: Element::default(),
            reference: Default::default(),
            _reference: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            identifier: Default::default(),
            display: Default::default(),
            _display: Default::default(),
        }
    }
}
