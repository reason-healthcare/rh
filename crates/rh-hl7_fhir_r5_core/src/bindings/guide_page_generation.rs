use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/guide-page-generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuidePageGeneration {
    /// HTML
    #[serde(rename = "html")]
    Html,
    /// Markdown
    #[serde(rename = "markdown")]
    Markdown,
    /// XML
    #[serde(rename = "xml")]
    Xml,
    /// Generated
    #[serde(rename = "generated")]
    Generated,
}
impl Default for GuidePageGeneration {
    fn default() -> Self {
        Self::Html
    }
}
