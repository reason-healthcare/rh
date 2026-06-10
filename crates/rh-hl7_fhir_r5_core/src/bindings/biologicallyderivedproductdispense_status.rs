use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderivedproductdispense-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiologicallyderivedproductdispenseStatus {
    /// Preparation
    #[serde(rename = "preparation")]
    Preparation,
    /// In Progress
    #[serde(rename = "in-progress")]
    InProgress,
    /// Allocated
    #[serde(rename = "allocated")]
    Allocated,
    /// Issued
    #[serde(rename = "issued")]
    Issued,
    /// Unfulfilled
    #[serde(rename = "unfulfilled")]
    Unfulfilled,
    /// Returned
    #[serde(rename = "returned")]
    Returned,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for BiologicallyderivedproductdispenseStatus {
    fn default() -> Self {
        Self::Preparation
    }
}
