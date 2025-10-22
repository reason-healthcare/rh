use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// strengthOfRecommendation
///
/// The strength of the recommendation assigned to this reference. The code system used specifies the rating scale used to rate this recommendation while the code specifies the actual recommendation rating (represented as a coded value) associated with this recommendation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-strengthOfRecommendation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFStrengthOfRecommendation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFStrengthOfRecommendation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
