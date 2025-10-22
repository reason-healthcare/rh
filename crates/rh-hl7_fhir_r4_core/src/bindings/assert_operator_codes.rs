use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/assert-operator-codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssertOperatorCodes {
    /// equals
    #[serde(rename = "equals")]
    Equals,
    /// notEquals
    #[serde(rename = "notEquals")]
    NotEquals,
    /// in
    #[serde(rename = "in")]
    In,
    /// notIn
    #[serde(rename = "notIn")]
    NotIn,
    /// greaterThan
    #[serde(rename = "greaterThan")]
    GreaterThan,
    /// lessThan
    #[serde(rename = "lessThan")]
    LessThan,
    /// empty
    #[serde(rename = "empty")]
    Empty,
    /// notEmpty
    #[serde(rename = "notEmpty")]
    NotEmpty,
    /// contains
    #[serde(rename = "contains")]
    Contains,
    /// notContains
    #[serde(rename = "notContains")]
    NotContains,
    /// evaluate
    #[serde(rename = "eval")]
    Eval,
}
impl Default for AssertOperatorCodes {
    fn default() -> Self {
        Self::Equals
    }
}
