use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Insurance
///
/// Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be relevant in delivering the requested service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/request-insurance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInsurance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RequestInsurance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
