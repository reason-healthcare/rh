use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Accession
///
/// An Accession Number associated with this participant object.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-Accession
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventAccession {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventAccession {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
