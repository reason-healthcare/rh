use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/examplescenario-actor-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExamplescenarioActorType {
    /// Person
    #[serde(rename = "person")]
    Person,
    /// System
    #[serde(rename = "system")]
    System,
}
impl Default for ExamplescenarioActorType {
    fn default() -> Self {
        Self::Person
    }
}
