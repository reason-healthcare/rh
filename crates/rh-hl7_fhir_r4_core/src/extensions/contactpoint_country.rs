use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Country Code
///
/// The country code as defined by the ITU. This extension is used when a system wishes to designate specific parts of a phone number (and potentially place constraints on which components must be present and how they're filled in).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/contactpoint-country
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactpointCountry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ContactpointCountry {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
