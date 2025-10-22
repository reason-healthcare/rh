use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Period
///
/// The date range that this organization should be considered available.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/organization-period
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationPeriod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OrganizationPeriod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
