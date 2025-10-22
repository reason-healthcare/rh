use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/report-participant-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportParticipantType {
    /// Test Engine
    #[serde(rename = "test-engine")]
    TestEngine,
    /// Client
    #[serde(rename = "client")]
    Client,
    /// Server
    #[serde(rename = "server")]
    Server,
}
impl Default for ReportParticipantType {
    fn default() -> Self {
        Self::TestEngine
    }
}
