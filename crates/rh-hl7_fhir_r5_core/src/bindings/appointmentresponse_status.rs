use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/appointmentresponse-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppointmentresponseStatus {
    /// Accepted
    #[serde(rename = "accepted")]
    Accepted,
    /// Declined
    #[serde(rename = "declined")]
    Declined,
    /// Tentative
    #[serde(rename = "tentative")]
    Tentative,
    /// Needs Action
    #[serde(rename = "needs-action")]
    NeedsAction,
    /// Entered in error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for AppointmentresponseStatus {
    fn default() -> Self {
        Self::Accepted
    }
}
