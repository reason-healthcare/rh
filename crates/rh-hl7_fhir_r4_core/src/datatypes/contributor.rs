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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

impl crate::validation::ValidatableResource for Contributor {
    fn resource_type(&self) -> &'static str {
        "Contributor"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Contributor")
    }
}
