use serde::{Deserialize, Serialize};
use crate::datatypes::backbone_element::BackboneElement;
/// ElementDefinition
///
/// Captures constraints on each element within the resource, profile, or extension.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: BackboneElement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
}
