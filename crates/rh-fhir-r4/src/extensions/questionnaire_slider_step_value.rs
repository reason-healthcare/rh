use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// sliderStepValue
///
/// For slider-based controls, indicates the step size to use when toggling the control up or down.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-sliderStepValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSliderStepValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireSliderStepValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
