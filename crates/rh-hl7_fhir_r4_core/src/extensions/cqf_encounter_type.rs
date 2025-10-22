use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// encounterType
///
/// The type of the encounter.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-encounterType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFEncounterType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFEncounterType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
