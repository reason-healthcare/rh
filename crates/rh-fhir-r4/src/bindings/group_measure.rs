use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/group-measure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupMeasure {
    /// Mean
    #[serde(rename = "mean")]
    Mean,
    /// Median
    #[serde(rename = "median")]
    Median,
    /// Mean of Study Means
    #[serde(rename = "mean-of-mean")]
    MeanOfMean,
    /// Mean of Study Medins
    #[serde(rename = "mean-of-median")]
    MeanOfMedian,
    /// Median of Study Means
    #[serde(rename = "median-of-mean")]
    MedianOfMean,
    /// Median of Study Medians
    #[serde(rename = "median-of-median")]
    MedianOfMedian,
}
impl Default for GroupMeasure {
    fn default() -> Self {
        Self::Mean
    }
}
