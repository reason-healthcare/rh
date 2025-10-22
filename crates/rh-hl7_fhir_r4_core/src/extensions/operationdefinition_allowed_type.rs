use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Allowed Type
///
/// When the base type is an abstract type (e.g. Resource or Element) then this extension defines which concrete types are allowed to be used for a parameter. In the absence of this extension, any type is allowed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/operationdefinition-allowed-type
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationdefinitionAllowedType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OperationdefinitionAllowedType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
