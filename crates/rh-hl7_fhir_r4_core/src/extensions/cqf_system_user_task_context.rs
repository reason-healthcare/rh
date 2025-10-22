use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// systemUserTaskContext
///
/// The task the system user is performing, e.g. laboratory results review, medication list review, etc. This information can be used to tailor decision support outputs, such as recommended information resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-systemUserTaskContext
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFSystemUserTaskContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFSystemUserTaskContext {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
