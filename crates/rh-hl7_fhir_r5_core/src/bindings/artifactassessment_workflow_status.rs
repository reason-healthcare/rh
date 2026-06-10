use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/artifactassessment-workflow-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactassessmentWorkflowStatus {
    /// Submitted
    #[serde(rename = "submitted")]
    Submitted,
    /// Triaged
    #[serde(rename = "triaged")]
    Triaged,
    /// Waiting for Input
    #[serde(rename = "waiting-for-input")]
    WaitingForInput,
    /// Resolved - No Change
    #[serde(rename = "resolved-no-change")]
    ResolvedNoChange,
    /// Resolved - Change Required
    #[serde(rename = "resolved-change-required")]
    ResolvedChangeRequired,
    /// Deferred
    #[serde(rename = "deferred")]
    Deferred,
    /// Duplicate
    #[serde(rename = "duplicate")]
    Duplicate,
    /// Applied
    #[serde(rename = "applied")]
    Applied,
    /// Published
    #[serde(rename = "published")]
    Published,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ArtifactassessmentWorkflowStatus {
    fn default() -> Self {
        Self::Submitted
    }
}
