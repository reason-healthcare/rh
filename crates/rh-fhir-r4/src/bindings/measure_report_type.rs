use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/measure-report-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasureReportType {
    /// Individual
    #[serde(rename = "individual")]
    Individual,
    /// Subject List
    #[serde(rename = "subject-list")]
    SubjectList,
    /// Summary
    #[serde(rename = "summary")]
    Summary,
    /// Data Collection
    #[serde(rename = "data-collection")]
    DataCollection,
}
impl Default for MeasureReportType {
    fn default() -> Self {
        Self::Individual
    }
}
