use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// HTTP Response header
///
/// In a transaction, every single interaction can have multiple HTTP response headers returned as a result of the interaction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/http-response-header
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPResponseHeader {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HTTPResponseHeader {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
