use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Variant
///
/// Variant information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsVariant
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsVariant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsVariant {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
