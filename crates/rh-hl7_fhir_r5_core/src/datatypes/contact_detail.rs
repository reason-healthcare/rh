use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ContactDetail
///
/// ContactDetail Type: Specifies contact information for a person or organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ContactDetail
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: ContactDetail
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Name of an individual to contact
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Contact details for individual or organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<ContactPoint>,
}

impl Default for ContactDetail {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            name: Default::default(),
            _name: Default::default(),
            telecom: Default::default(),
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
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ContactDetail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ContactDetail.extension", 0, None),
            rh_foundation::ElementCardinality::new("ContactDetail.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ContactDetail.telecom", 0, None),
        ]
    });

impl crate::validation::ValidatableResource for ContactDetail {
    fn resource_type(&self) -> &'static str {
        "ContactDetail"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ContactDetail")
    }
}
