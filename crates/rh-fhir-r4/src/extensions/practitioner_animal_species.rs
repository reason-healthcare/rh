use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// animalSpecies
///
/// This extension should be used to specifiy that a practioner or RelatedPerson resource is a service animal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/practitioner-animalSpecies
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerAnimalSpecies {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PractitionerAnimalSpecies {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
