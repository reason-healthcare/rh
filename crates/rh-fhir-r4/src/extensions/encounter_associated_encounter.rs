use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// associatedEncounter
///
/// This encounter occurs within the scope of the referenced encounter.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/encounter-associatedEncounter
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterAssociatedEncounter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EncounterAssociatedEncounter {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
