use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Location of Access restriction
///
/// Restricts this exception to only apply a specific location as defined.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-location
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConsentLocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
