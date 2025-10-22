use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Addendum Of
///
/// The supplements or provides additional information for the target report.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-addendumOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportAddendumOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportAddendumOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
