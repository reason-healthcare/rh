use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/property-representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyRepresentation {
    /// XML Attribute
    #[serde(rename = "xmlAttr")]
    XmlAttr,
    /// XML Text
    #[serde(rename = "xmlText")]
    XmlText,
    /// Type Attribute
    #[serde(rename = "typeAttr")]
    TypeAttr,
    /// CDA Text Format
    #[serde(rename = "cdaText")]
    CdaText,
    /// XHTML
    #[serde(rename = "xhtml")]
    Xhtml,
}
impl Default for PropertyRepresentation {
    fn default() -> Self {
        Self::XmlAttr
    }
}
