use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Special handling
///
/// Special handling during the collection, transport, or storage of the specimen.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-specialHandling
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenSpecialHandling {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for SpecimenSpecialHandling {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
