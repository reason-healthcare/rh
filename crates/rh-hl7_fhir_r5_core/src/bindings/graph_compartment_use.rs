use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/graph-compartment-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphCompartmentUse {
    /// Where
    #[serde(rename = "where")]
    WhereValue,
    /// requires
    #[serde(rename = "requires")]
    Requires,
}
impl Default for GraphCompartmentUse {
    fn default() -> Self {
        Self::WhereValue
    }
}
