use crate::datatypes::element::Element;
use serde::{Deserialize, Serialize};
/// Extension Data Type
///
/// Optional Extension Element - found in all resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Extension
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Element
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
}
