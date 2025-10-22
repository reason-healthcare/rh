use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Do Not Perfom
///
/// If true indicates that the request is asking for the specified action to not occur.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/request-doNotPerform
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoNotPerform {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RequestDoNotPerform {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
