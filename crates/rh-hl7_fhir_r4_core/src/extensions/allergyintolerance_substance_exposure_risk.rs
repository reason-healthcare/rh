use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// substanceExposureRisk
///
/// A complex extension allowing structured capture of the exposure risk of the patient for an adverse reaction (allergy or intolerance) to the specified substance/product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-substanceExposureRisk
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceSubstanceExposureRisk {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceSubstanceExposureRisk {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
