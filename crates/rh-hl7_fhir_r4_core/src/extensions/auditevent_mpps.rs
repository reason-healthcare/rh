use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// MPPS
///
/// An MPPS Instance UID associated with this entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-MPPS
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventMPPS {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventMPPS {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
