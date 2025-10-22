use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ParticipantObjectContainsStudy
///
/// A Study Instance ID, which may be used when the Entity type is not (110180, DCM, "Study Instance UID").
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-ParticipantObjectContainsStudy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventParticipantObjectContainsStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventParticipantObjectContainsStudy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
