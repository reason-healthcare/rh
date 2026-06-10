use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/subscriptiontopic-cr-behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptiontopicCrBehavior {
    /// Test passes
    #[serde(rename = "test-passes")]
    TestPasses,
    /// Test fails
    #[serde(rename = "test-fails")]
    TestFails,
}
impl Default for SubscriptiontopicCrBehavior {
    fn default() -> Self {
        Self::TestPasses
    }
}
