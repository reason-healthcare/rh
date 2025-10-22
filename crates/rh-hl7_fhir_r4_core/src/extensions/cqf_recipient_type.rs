use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// recipientType
///
/// The type of individual that will consume the response content. This may be different from the requesting user type (e.g. if a clinician is getting disease management guidance for provision to a patient). E.g. patient, healthcare provider or specific type of healthcare provider (physician, nurse, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-recipientType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFRecipientType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFRecipientType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
