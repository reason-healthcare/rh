use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// exact
///
/// If true, indicates that the specified times, frequencies, periods are expected to be adhered to as precisely as possible.  If false, indicates that a typical degree of variability based on institutional and/or patient convenience is acceptable.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/timing-exact
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingExact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TimingExact {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
