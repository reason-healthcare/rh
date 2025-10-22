use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/days-of-week
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaysOfWeek {
    /// Monday
    #[serde(rename = "mon")]
    Mon,
    /// Tuesday
    #[serde(rename = "tue")]
    Tue,
    /// Wednesday
    #[serde(rename = "wed")]
    Wed,
    /// Thursday
    #[serde(rename = "thu")]
    Thu,
    /// Friday
    #[serde(rename = "fri")]
    Fri,
    /// Saturday
    #[serde(rename = "sat")]
    Sat,
    /// Sunday
    #[serde(rename = "sun")]
    Sun,
}
impl Default for DaysOfWeek {
    fn default() -> Self {
        Self::Mon
    }
}
