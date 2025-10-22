use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ContactDetail
///
/// Base StructureDefinition for ContactDetail Type: Specifies contact information for a person or organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ContactDetail
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ContactDetail
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Name of an individual to contact
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Contact details for individual or organization
    pub telecom: Option<Vec<ContactPoint>>,
}

impl Default for ContactDetail {
    fn default() -> Self {
        Self {
            base: Element::default(),
            name: Default::default(),
            _name: Default::default(),
            telecom: Default::default(),
        }
    }
}
