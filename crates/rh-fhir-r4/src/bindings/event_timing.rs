use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/event-timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTiming {
    /// Morning
    #[serde(rename = "MORN")]
    MORN,
    /// Early Morning
    #[serde(rename = "MORN.early")]
    MORNEarly,
    /// Late Morning
    #[serde(rename = "MORN.late")]
    MORNLate,
    /// Noon
    #[serde(rename = "NOON")]
    NOON,
    /// Afternoon
    #[serde(rename = "AFT")]
    AFT,
    /// Early Afternoon
    #[serde(rename = "AFT.early")]
    AFTEarly,
    /// Late Afternoon
    #[serde(rename = "AFT.late")]
    AFTLate,
    /// Evening
    #[serde(rename = "EVE")]
    EVE,
    /// Early Evening
    #[serde(rename = "EVE.early")]
    EVEEarly,
    /// Late Evening
    #[serde(rename = "EVE.late")]
    EVELate,
    /// Night
    #[serde(rename = "NIGHT")]
    NIGHT,
    /// After Sleep
    #[serde(rename = "PHS")]
    PHS,
    #[serde(rename = "HS")]
    HS,
    #[serde(rename = "WAKE")]
    WAKE,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CV")]
    CV,
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "ACM")]
    ACM,
    #[serde(rename = "ACD")]
    ACD,
    #[serde(rename = "ACV")]
    ACV,
    #[serde(rename = "PC")]
    PC,
    #[serde(rename = "PCM")]
    PCM,
    #[serde(rename = "PCD")]
    PCD,
    #[serde(rename = "PCV")]
    PCV,
}
impl Default for EventTiming {
    fn default() -> Self {
        Self::MORN
    }
}
