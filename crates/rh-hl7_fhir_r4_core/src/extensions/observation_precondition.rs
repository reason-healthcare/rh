use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Precondition
///
/// Other preceding or concurrent observations that must be known to correctly interpret the the observation.  For example an fiO2 measure taken alongside of a SpO2 measurement.  See the [Observation notes](observation.html#notes) section for additional guidance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-precondition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationPrecondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationPrecondition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
