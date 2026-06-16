use crate::datatypes::element::Element;
use serde::{Deserialize, Serialize};
/// HumanName
///
/// A complex-type for golden testing
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HumanName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: HumanName
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
}


impl Default for HumanName {
    fn default() -> Self {
        Self {
            base: Element::default()
        }
    }
}

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> = once_cell::sync::Lazy::new(Vec::new);
