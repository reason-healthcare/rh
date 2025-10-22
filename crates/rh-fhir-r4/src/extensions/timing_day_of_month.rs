use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Day Of Month
///
/// When present, this extension indicate that the event actually only occurs on the specified days of the month, on the times as otherwise specified by the timing schedule.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/timing-dayOfMonth
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingDayOfMonth {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TimingDayOfMonth {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
