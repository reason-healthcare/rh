use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Bi-directional
///
/// Set to true if the concept map can be safely intepreted in reversse.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/concept-bidirectional
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptBidirectional {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConceptBidirectional {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
