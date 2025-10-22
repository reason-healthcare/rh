use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Matching Grade
///
/// Assessment of resource match outcome - how likely this resource is to be a match.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/match-grade
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchGrade {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MatchGrade {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
