use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/guide-parameter-code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuideParameterCode {
    /// Apply Metadata Value
    #[serde(rename = "apply")]
    Apply,
    /// Resource Path
    #[serde(rename = "path-resource")]
    PathResource,
    /// Pages Path
    #[serde(rename = "path-pages")]
    PathPages,
    /// Terminology Cache Path
    #[serde(rename = "path-tx-cache")]
    PathTxCache,
    /// Expansion Profile
    #[serde(rename = "expansion-parameter")]
    ExpansionParameter,
    /// Broken Links Rule
    #[serde(rename = "rule-broken-links")]
    RuleBrokenLinks,
    /// Generate XML
    #[serde(rename = "generate-xml")]
    GenerateXml,
    /// Generate JSON
    #[serde(rename = "generate-json")]
    GenerateJson,
    /// Generate Turtle
    #[serde(rename = "generate-turtle")]
    GenerateTurtle,
    /// HTML Template
    #[serde(rename = "html-template")]
    HtmlTemplate,
}
impl Default for GuideParameterCode {
    fn default() -> Self {
        Self::Apply
    }
}
