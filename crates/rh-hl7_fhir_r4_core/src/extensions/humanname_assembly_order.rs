use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// assembly-order
///
/// A code that represents the preferred display order of the components of this human name.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/humanname-assembly-order
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumannameAssemblyOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HumannameAssemblyOrder {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
