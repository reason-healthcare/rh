use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Effective Period
///
/// The period during which the resource content was or is planned to be effective.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-effectivePeriod
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEffectivePeriod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ResourceEffectivePeriod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
