use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Profile
///
/// Identifies a profile structure or implementation Guide that applies to the datatype this element refers to. If any profiles are specified, then the content must conform to at least one of them. The URL can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the type SHALL conform to at least one profile defined in the implementation guide.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/operationdefinition-profile
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationdefinitionProfile {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OperationdefinitionProfile {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
