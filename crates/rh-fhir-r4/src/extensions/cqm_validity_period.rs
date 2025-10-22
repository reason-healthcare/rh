use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ValidityPeriod
///
/// The period in which the catalog is valid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqm-ValidityPeriod
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CqmValidityPeriod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CqmValidityPeriod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
