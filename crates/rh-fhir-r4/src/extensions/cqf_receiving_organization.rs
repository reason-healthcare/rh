use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// receivingOrganization
///
/// The organization that will receive the response.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-receivingOrganization
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFReceivingOrganization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFReceivingOrganization {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
