use crate::bindings::contributor_type::ContributorType;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Contributor
///
/// Base StructureDefinition for Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Contributor
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Contributor
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// author | editor | reviewer | endorser
    #[serde(rename = "type")]
    pub type_: ContributorType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Who contributed the content
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Contact details of the contributor
    pub contact: Option<Vec<ContactDetail>>,
}

impl Default for Contributor {
    fn default() -> Self {
        Self {
            base: Element::default(),
            type_: Default::default(),
            _type: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            contact: Default::default(),
        }
    }
}
