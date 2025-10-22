use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Last Review Date
///
/// The date on which the asset content was last reviewed. Review happens periodically after that, but doesn't change the original approval date.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-lastReviewDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLastReviewDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ResourceLastReviewDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
