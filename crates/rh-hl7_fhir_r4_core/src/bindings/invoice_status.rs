use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/invoice-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    /// draft
    #[serde(rename = "draft")]
    Draft,
    /// issued
    #[serde(rename = "issued")]
    Issued,
    /// balanced
    #[serde(rename = "balanced")]
    Balanced,
    /// cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for InvoiceStatus {
    fn default() -> Self {
        Self::Draft
    }
}
