use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/research-subject-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchSubjectStatus {
    /// Candidate
    #[serde(rename = "candidate")]
    Candidate,
    /// Eligible
    #[serde(rename = "eligible")]
    Eligible,
    /// Follow-up
    #[serde(rename = "follow-up")]
    FollowUp,
    /// Ineligible
    #[serde(rename = "ineligible")]
    Ineligible,
    /// Not Registered
    #[serde(rename = "not-registered")]
    NotRegistered,
    /// Off-study
    #[serde(rename = "off-study")]
    OffStudy,
    /// On-study
    #[serde(rename = "on-study")]
    OnStudy,
    /// On-study-intervention
    #[serde(rename = "on-study-intervention")]
    OnStudyIntervention,
    /// On-study-observation
    #[serde(rename = "on-study-observation")]
    OnStudyObservation,
    /// Pending on-study
    #[serde(rename = "pending-on-study")]
    PendingOnStudy,
    /// Potential Candidate
    #[serde(rename = "potential-candidate")]
    PotentialCandidate,
    /// Screening
    #[serde(rename = "screening")]
    Screening,
    /// Withdrawn
    #[serde(rename = "withdrawn")]
    Withdrawn,
}
impl Default for ResearchSubjectStatus {
    fn default() -> Self {
        Self::Candidate
    }
}
