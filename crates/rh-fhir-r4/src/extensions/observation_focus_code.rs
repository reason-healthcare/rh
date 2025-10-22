use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Focal Subject Code
///
/// A code representing the  focus of an observation when the focus is not the patient of record.  In other words, the focus of the observation is different from `Observation.subject`.   An example use case would be using the *Observation* resource to capture whether the mother is trained to change her child's tracheostomy tube.  In this example, the child is the patient of record and the mother is focal subject referenced using this extension.  Other example focal subjects include spouses, related persons, feti, or  donors.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-focusCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationFocusCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationFocusCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
