use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Adaptive Feeding Device
///
/// Materials used or needed to feed the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/nutritionorder-adaptiveFeedingDevice
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionorderAdaptiveFeedingDevice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for NutritionorderAdaptiveFeedingDevice {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
