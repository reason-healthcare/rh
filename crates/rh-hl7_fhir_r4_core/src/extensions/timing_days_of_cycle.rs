use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Days of Cycle
///
/// Days of a possibly repeating cycle on which the action is to be performed. The cycle is defined by the first action with a timing element that is a parent of the daysOfCycle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/timing-daysOfCycle
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingDaysOfCycle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TimingDaysOfCycle {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
