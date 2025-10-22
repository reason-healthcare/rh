use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// initiatingLocation
///
/// Location where the information being requested to be communicated happened.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/communicationrequest-initiatingLocation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationrequestInitiatingLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CommunicationrequestInitiatingLocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
