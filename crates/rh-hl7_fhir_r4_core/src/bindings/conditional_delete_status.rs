use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/conditional-delete-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalDeleteStatus {
    /// Not Supported
    #[serde(rename = "not-supported")]
    NotSupported,
    /// Single Deletes Supported
    #[serde(rename = "single")]
    Single,
    /// Multiple Deletes Supported
    #[serde(rename = "multiple")]
    Multiple,
}
impl Default for ConditionalDeleteStatus {
    fn default() -> Self {
        Self::NotSupported
    }
}
