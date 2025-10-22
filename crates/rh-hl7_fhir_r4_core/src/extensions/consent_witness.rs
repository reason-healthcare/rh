use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Witness
///
/// Any witness to the consent.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-Witness
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentWitness {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConsentWitness {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
