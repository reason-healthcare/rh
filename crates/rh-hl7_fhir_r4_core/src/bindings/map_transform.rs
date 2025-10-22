use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/map-transform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapTransform {
    /// create
    #[serde(rename = "create")]
    Create,
    /// copy
    #[serde(rename = "copy")]
    Copy,
    /// truncate
    #[serde(rename = "truncate")]
    Truncate,
    /// escape
    #[serde(rename = "escape")]
    Escape,
    /// cast
    #[serde(rename = "cast")]
    Cast,
    /// append
    #[serde(rename = "append")]
    Append,
    /// translate
    #[serde(rename = "translate")]
    Translate,
    /// reference
    #[serde(rename = "reference")]
    Reference,
    /// dateOp
    #[serde(rename = "dateOp")]
    DateOp,
    /// uuid
    #[serde(rename = "uuid")]
    Uuid,
    /// pointer
    #[serde(rename = "pointer")]
    Pointer,
    /// evaluate
    #[serde(rename = "evaluate")]
    Evaluate,
    /// cc
    #[serde(rename = "cc")]
    Cc,
    /// c
    #[serde(rename = "c")]
    C,
    /// qty
    #[serde(rename = "qty")]
    Qty,
    /// id
    #[serde(rename = "id")]
    Id,
    /// cp
    #[serde(rename = "cp")]
    Cp,
}
impl Default for MapTransform {
    fn default() -> Self {
        Self::Create
    }
}
