use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Summary Of
///
/// A summary report that points to subordinate target reports.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-summaryOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportSummaryOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportSummaryOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
