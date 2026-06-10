use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/discriminator-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscriminatorType {
    /// Value
    #[serde(rename = "value")]
    Value,
    /// Exists
    #[serde(rename = "exists")]
    Exists,
    /// Pattern
    #[serde(rename = "pattern")]
    Pattern,
    /// Type
    #[serde(rename = "type")]
    TypeValue,
    /// Profile
    #[serde(rename = "profile")]
    Profile,
    /// Position
    #[serde(rename = "position")]
    Position,
}
impl Default for DiscriminatorType {
    fn default() -> Self {
        Self::Value
    }
}
