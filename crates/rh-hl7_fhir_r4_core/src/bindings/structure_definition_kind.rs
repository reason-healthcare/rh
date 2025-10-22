use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/structure-definition-kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureDefinitionKind {
    /// Primitive Data Type
    #[serde(rename = "primitive-type")]
    PrimitiveType,
    /// Complex Data Type
    #[serde(rename = "complex-type")]
    ComplexType,
    /// Resource
    #[serde(rename = "resource")]
    Resource,
    /// Logical
    #[serde(rename = "logical")]
    Logical,
}
impl Default for StructureDefinitionKind {
    fn default() -> Self {
        Self::PrimitiveType
    }
}
