use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// NumberOfInstances
///
/// The Number of SOP Instances referred to by this entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-NumberOfInstances
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventNumberOfInstances {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventNumberOfInstances {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
