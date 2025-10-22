use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// encounterClass
///
/// The class of encounter (inpatient, outpatient, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-encounterClass
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFEncounterClass {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFEncounterClass {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
