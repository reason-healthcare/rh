use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// SOPClass
///
/// Required if ParticipantObjectIDTypeCode is (110180, DCM, "Study Instance UID") and any of the optional fields (AccessionNumber, ContainsMPPS, NumberOfInstances, ContainsSOPInstances,Encrypted,Anonymized) are present in this Participant Object. May be present if ParticipantObjectIDTypeCode is (110180, DCM, "Study Instance UID") even though none of the optional fields are present.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-SOPClass
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventSOPClass {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventSOPClass {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
