use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// sequenceNumber
///
/// An assigned number on the specimen denoting the order of collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-sequenceNumber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenSequenceNumber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for SpecimenSequenceNumber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
