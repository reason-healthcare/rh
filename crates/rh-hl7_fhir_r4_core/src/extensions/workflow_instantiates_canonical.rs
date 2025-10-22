use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Definition
///
/// The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by the event or request resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-instantiatesCanonical
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstantiatesCanonical {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowInstantiatesCanonical {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
