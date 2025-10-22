use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Patient Record
///
/// A link to one to more patient records for the relation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-patient-record
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilymemberhistoryPatientRecord {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilymemberhistoryPatientRecord {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
