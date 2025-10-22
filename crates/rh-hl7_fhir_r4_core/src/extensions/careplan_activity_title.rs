use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Title
///
/// Human-friendly name for the activity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/careplan-activity-title
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareplanActivityTitle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CareplanActivityTitle {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
