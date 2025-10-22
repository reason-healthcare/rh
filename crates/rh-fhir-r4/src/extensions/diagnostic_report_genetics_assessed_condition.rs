use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// AssessedCondition
///
/// Used to denote condition context for genetic testing, which may influence reported variants and interpretation for large genomic testing panels e.g. lung cancer or familial breast cancer.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsAssessedCondition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsAssessedCondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportGeneticsAssessedCondition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
