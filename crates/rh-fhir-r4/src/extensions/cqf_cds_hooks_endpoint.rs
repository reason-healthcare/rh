use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// cdsHooksEndpoint
///
/// Specifies the URI of a CDS Hooks service that uses this PlanDefinition as its implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-cdsHooksEndpoint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFCdsHooksEndpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFCdsHooksEndpoint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
