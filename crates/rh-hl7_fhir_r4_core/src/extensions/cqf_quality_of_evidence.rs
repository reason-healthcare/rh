use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// qualityOfEvidence
///
/// The quality of the evidence described. The code system used specifies the quality scale used to grade this evidence source while the code specifies the actual quality score (represented as a coded value) associated with the evidence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-qualityOfEvidence
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFQualityOfEvidence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFQualityOfEvidence {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
