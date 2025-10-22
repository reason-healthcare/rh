use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// observation
///
/// Allows capturing risk-relevant observations about the relative that aren't themselves a specific health condition; e.g. Certain ethnic ancestries that are disease-relevant, presence of particular genetic markers, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/family-member-history-genetics-observation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberHistoryGeneticsObservation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilyMemberHistoryGeneticsObservation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
