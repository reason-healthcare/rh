use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// References
///
/// Additional bibliographic reference information about genetics, medications, clinical trials, etc. associated with knowledge-based information on genetics/genetic condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsReferences
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsReferences {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportGeneticsReferences {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
