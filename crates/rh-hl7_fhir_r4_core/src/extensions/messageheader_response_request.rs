use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// MessageHeader response request
///
/// This extension enables the capability currently available through MSH-16 (Application Level acknowledgement) in HL7 Version 2 to declare at a message instance level whether a response is required or only upon error or success, or never.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/messageheader-response-request
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageheaderResponseRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MessageheaderResponseRequest {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
