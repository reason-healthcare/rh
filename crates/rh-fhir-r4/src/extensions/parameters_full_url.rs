use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// fullUrl for resource
///
/// This specifies the fullUrl for the resource in parameters.resource, if there is one. When fullUrl is provided, ithe [resource resolution method described for Bundle](bundle.html#references).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/parameters-fullUrl
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersFullURL {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ParametersFullURL {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
