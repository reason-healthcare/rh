use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// receivingPerson
///
/// The person in the receiving organization that will receive the response.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-receivingPerson
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFReceivingPerson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFReceivingPerson {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
