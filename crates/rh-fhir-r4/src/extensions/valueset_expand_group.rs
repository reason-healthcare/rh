use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expand-group
///
/// This extension declares a group of concepts that is generated into the ValueSet.expansion.contains hierarchy when the expansion is generated for a UI. THere is no inherent assigned meaning to the hierarchy; it is used to help the user navigate the concepts. Each group has a display and/or a code, and a list of members, which are either concepts in the value set, or other groups (by code).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expand-group
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpandGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpandGroup {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
