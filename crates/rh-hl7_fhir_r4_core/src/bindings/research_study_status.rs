use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/research-study-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchStudyStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Administratively Completed
    #[serde(rename = "administratively-completed")]
    AdministrativelyCompleted,
    /// Approved
    #[serde(rename = "approved")]
    Approved,
    /// Closed to Accrual
    #[serde(rename = "closed-to-accrual")]
    ClosedToAccrual,
    /// Closed to Accrual and Intervention
    #[serde(rename = "closed-to-accrual-and-intervention")]
    ClosedToAccrualAndIntervention,
    /// Completed
    #[serde(rename = "completed")]
    Completed,
    /// Disapproved
    #[serde(rename = "disapproved")]
    Disapproved,
    /// In Review
    #[serde(rename = "in-review")]
    InReview,
    /// Temporarily Closed to Accrual
    #[serde(rename = "temporarily-closed-to-accrual")]
    TemporarilyClosedToAccrual,
    /// Temporarily Closed to Accrual and Intervention
    #[serde(rename = "temporarily-closed-to-accrual-and-intervention")]
    TemporarilyClosedToAccrualAndIntervention,
    /// Withdrawn
    #[serde(rename = "withdrawn")]
    Withdrawn,
}
impl Default for ResearchStudyStatus {
    fn default() -> Self {
        Self::Active
    }
}
