use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// delta
///
/// The qualitative change in the value relative to the previous measurement. Usually only recorded if the change is clinically significant.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-delta
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDelta {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationDelta {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
