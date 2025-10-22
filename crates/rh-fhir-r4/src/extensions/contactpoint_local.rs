use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Local Number
///
/// The local number that must be dialed to connect within the area/city/zone. This extension is used when a system wishes to designate specific parts of a phone number (and potentially place constraints on which components must be present and how they're filled in).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/contactpoint-local
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactpointLocal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ContactpointLocal {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
