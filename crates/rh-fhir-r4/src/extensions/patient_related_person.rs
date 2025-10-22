use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// relatedPerson
///
/// In some cases a Patient.contact will also be populated as a RelatedPerson resource. This linkage permits the linkage between the 2 resources to be able to accurately indicate a representation of the same individual, and updating details between could be appropriate.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-relatedPerson
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRelatedPerson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientRelatedPerson {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
