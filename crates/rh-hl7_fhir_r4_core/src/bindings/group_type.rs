use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/group-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupType {
    /// Person
    #[serde(rename = "person")]
    Person,
    /// Animal
    #[serde(rename = "animal")]
    Animal,
    /// Practitioner
    #[serde(rename = "practitioner")]
    Practitioner,
    /// Device
    #[serde(rename = "device")]
    Device,
    /// Medication
    #[serde(rename = "medication")]
    Medication,
    /// Substance
    #[serde(rename = "substance")]
    Substance,
}
impl Default for GroupType {
    fn default() -> Self {
        Self::Person
    }
}
