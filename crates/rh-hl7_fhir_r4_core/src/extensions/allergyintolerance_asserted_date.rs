use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// assertedDate
///
/// The date on which the existence of the AllergyIntolerance was first asserted or acknowledged.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-assertedDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceAssertedDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceAssertedDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
