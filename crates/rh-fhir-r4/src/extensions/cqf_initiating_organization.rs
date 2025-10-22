use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// initiatingOrganization
///
/// The organization initiating the request.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-initiatingOrganization
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFInitiatingOrganization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFInitiatingOrganization {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
