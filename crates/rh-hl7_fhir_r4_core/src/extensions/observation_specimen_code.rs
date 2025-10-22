use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Specimen Code
///
/// A code representing the the type of specimen used for this observation.  Should only be used if not implicit in the code found in `Observation.code`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-specimenCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSpecimenCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationSpecimenCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
