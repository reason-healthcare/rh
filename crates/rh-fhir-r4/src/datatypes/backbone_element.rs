use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// BackboneElement
///
/// Base StructureDefinition for BackboneElement Type: Base definition for all elements that are defined inside a resource - but not those in a data type.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BackboneElement
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: BackboneElement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackboneElement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,
}

impl Default for BackboneElement {
    fn default() -> Self {
        Self {
            base: Element::default(),
            modifier_extension: Default::default(),
        }
    }
}
