use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/filter-operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    /// Equals
    #[serde(rename = "=")]
    Equal,
    /// Is A (by subsumption)
    #[serde(rename = "is-a")]
    IsA,
    /// Descendent Of (by subsumption)
    #[serde(rename = "descendent-of")]
    DescendentOf,
    /// Not (Is A) (by subsumption)
    #[serde(rename = "is-not-a")]
    IsNotA,
    /// Regular Expression
    #[serde(rename = "regex")]
    Regex,
    /// In Set
    #[serde(rename = "in")]
    In,
    /// Not in Set
    #[serde(rename = "not-in")]
    NotIn,
    /// Generalizes (by Subsumption)
    #[serde(rename = "generalizes")]
    Generalizes,
    /// Exists
    #[serde(rename = "exists")]
    Exists,
}
impl Default for FilterOperator {
    fn default() -> Self {
        Self::Equal
    }
}
