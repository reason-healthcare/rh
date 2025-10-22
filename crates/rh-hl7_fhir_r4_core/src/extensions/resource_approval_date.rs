use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Approval Date
///
/// The date on which the asset content was approved by the publisher. Approval happens once when the content is officially approved for usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-approvalDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceApprovalDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ResourceApprovalDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
