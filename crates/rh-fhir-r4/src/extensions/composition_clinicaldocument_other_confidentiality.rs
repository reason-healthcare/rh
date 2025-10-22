use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// otherConfidentiality
///
/// Carries additional confidentiality codes beyond the base fixed code specified in the CDA document.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-clinicaldocument-otherConfidentiality
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionClinicaldocumentOtherConfidentiality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CompositionClinicaldocumentOtherConfidentiality {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
