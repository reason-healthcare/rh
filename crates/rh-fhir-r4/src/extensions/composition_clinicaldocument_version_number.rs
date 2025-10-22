use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// versionNumber
///
/// Version specific identifier for the composition, assigned when each version is created/updated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-clinicaldocument-versionNumber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionClinicaldocumentVersionNumber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CompositionClinicaldocumentVersionNumber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
