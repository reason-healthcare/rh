use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/contact-point-system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactPointSystem {
    /// Phone
    #[serde(rename = "phone")]
    Phone,
    /// Fax
    #[serde(rename = "fax")]
    Fax,
    /// Email
    #[serde(rename = "email")]
    Email,
    /// Pager
    #[serde(rename = "pager")]
    Pager,
    /// URL
    #[serde(rename = "url")]
    Url,
    /// SMS
    #[serde(rename = "sms")]
    Sms,
    /// Other
    #[serde(rename = "other")]
    Other,
}
impl Default for ContactPointSystem {
    fn default() -> Self {
        Self::Phone
    }
}
