use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Area Code
///
/// The area/zone/city code that, in some areas, may be omitted when dialing locally within the zone. This extension is used when a system wishes to designate specific parts of a phone number (and potentially place constraints on which components must be present and how they're filled in).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/contactpoint-area
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactpointArea {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ContactpointArea {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
