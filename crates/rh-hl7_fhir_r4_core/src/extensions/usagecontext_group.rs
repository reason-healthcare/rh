use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Context Group
///
/// Defines the group in which this usage context is a member. Multiple groups are "OR'ed", contexts within a group are "AND'ed".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/usagecontext-group
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagecontextGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for UsagecontextGroup {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
