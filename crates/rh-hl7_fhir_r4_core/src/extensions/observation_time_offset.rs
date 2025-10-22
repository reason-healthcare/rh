use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Time-offset
///
/// A specific offset time in milliseconds from the stated time in the Observation.appliesDateTime to allow for representation of sequential recording  of sampled data from the same lead or data stream.  For example, an ECG recorder may record sequentially 3 leads four time to receive 12-lead ECG, see [ISO 22077](https://www.iso.org/obp/ui/#iso:std:61871:en).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-timeOffset
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationTimeOffset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationTimeOffset {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
