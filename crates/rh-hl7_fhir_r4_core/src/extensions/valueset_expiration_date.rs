use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expirationDate
///
/// The date when the value set version is no longer expected to be used to create new content. This is the first date-time when the value set version becomes Inactive, so this value SHALL present on all Inactive value set versions. The start Date_time is expected to be as of 0001 UTC of the Expiration Date.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expirationDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpirationDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpirationDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
