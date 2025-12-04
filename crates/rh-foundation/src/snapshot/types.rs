use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinition {
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub base_definition: Option<String>,
    pub differential: Option<Differential>,
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Differential {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinition {
    pub path: String,
    pub id: Option<String>,
    pub min: Option<u32>,
    pub max: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<Vec<ElementType>>,
    pub binding: Option<ElementBinding>,
    pub constraint: Option<Vec<ElementConstraint>>,
    pub definition: Option<String>,
    pub short: Option<String>,
    pub comment: Option<String>,
    pub requirements: Option<String>,
    #[serde(rename = "mustSupport")]
    pub must_support: Option<bool>,
    #[serde(rename = "isSummary")]
    pub is_summary: Option<bool>,
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<bool>,
    #[serde(rename = "isModifierReason")]
    pub is_modifier_reason: Option<String>,
    pub slicing: Option<ElementSlicing>,
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementSlicing {
    pub discriminator: Option<Vec<ElementDiscriminator>>,
    pub rules: Option<String>,
    pub ordered: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDiscriminator {
    #[serde(rename = "type")]
    pub type_: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementType {
    pub code: String,
    pub profile: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementBinding {
    pub strength: String,
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementConstraint {
    pub key: String,
    pub severity: String,
    pub human: String,
    pub expression: Option<String>,
}
