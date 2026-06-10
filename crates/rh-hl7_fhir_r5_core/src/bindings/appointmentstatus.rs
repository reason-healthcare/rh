use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/appointmentstatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Appointmentstatus {
    /// Proposed
    #[serde(rename = "proposed")]
    Proposed,
    /// Pending
    #[serde(rename = "pending")]
    Pending,
    /// Booked
    #[serde(rename = "booked")]
    Booked,
    /// Arrived
    #[serde(rename = "arrived")]
    Arrived,
    /// Fulfilled
    #[serde(rename = "fulfilled")]
    Fulfilled,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// No Show
    #[serde(rename = "noshow")]
    Noshow,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Checked In
    #[serde(rename = "checked-in")]
    CheckedIn,
    /// Waitlisted
    #[serde(rename = "waitlist")]
    Waitlist,
}
impl Default for Appointmentstatus {
    fn default() -> Self {
        Self::Proposed
    }
}
