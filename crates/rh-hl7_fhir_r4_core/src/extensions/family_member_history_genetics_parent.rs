use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// parent
///
/// Identifies a parent of the relative.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/family-member-history-genetics-parent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryGeneticsParent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilyMemberHistoryGeneticsParent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
