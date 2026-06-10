use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/chargeitem-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChargeitemStatus {
    /// Planned
    #[serde(rename = "planned")]
    Planned,
    /// Billable
    #[serde(rename = "billable")]
    Billable,
    /// Not billable
    #[serde(rename = "not-billable")]
    NotBillable,
    /// Aborted
    #[serde(rename = "aborted")]
    Aborted,
    /// Billed
    #[serde(rename = "billed")]
    Billed,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for ChargeitemStatus {
    fn default() -> Self {
        Self::Planned
    }
}
