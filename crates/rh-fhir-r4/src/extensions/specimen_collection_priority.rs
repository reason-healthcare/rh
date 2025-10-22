use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Collection Priority
///
/// The urgency of sample collection, such as STAT, ASAP, ASAP-ED, ROUTINE, ROUTINE-AM, etc….
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-collectionPriority
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenCollectionPriority {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for SpecimenCollectionPriority {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
