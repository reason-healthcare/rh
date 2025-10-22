use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// precondition
///
/// The condition or state of the patient, prior or during the diagnostic procedure or test, for example, fasting, at-rest, or post-operative. This captures circumstances that may influence the measured value and have bearing on the interpretation of the result.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-precondition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicerequestPrecondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ServicerequestPrecondition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
