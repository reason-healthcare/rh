use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/codesystem-hierarchy-meaning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodesystemHierarchyMeaning {
    /// Grouped By
    #[serde(rename = "grouped-by")]
    GroupedBy,
    /// Is-A
    #[serde(rename = "is-a")]
    IsA,
    /// Part Of
    #[serde(rename = "part-of")]
    PartOf,
    /// Classified With
    #[serde(rename = "classified-with")]
    ClassifiedWith,
}
impl Default for CodesystemHierarchyMeaning {
    fn default() -> Self {
        Self::GroupedBy
    }
}
