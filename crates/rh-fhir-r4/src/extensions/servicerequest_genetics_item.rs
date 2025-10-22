use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Item
///
/// The specific diagnostic investigations that are requested as part of this request. Sometimes, there can only be one item per request, but in most contexts, more than one investigation can be requested.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-geneticsItem
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicerequestGeneticsItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ServicerequestGeneticsItem {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
