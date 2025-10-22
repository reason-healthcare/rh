use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// oauth-uris
///
/// Supports automated discovery of OAuth2 endpoints.
///
/// **Source:**
/// - URL: http://fhir-registry.smarthealthit.org/StructureDefinition/oauth-uris
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthURIs {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OauthURIs {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
