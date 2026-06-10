use crate::datatypes::address::Address;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::data_type::DataType;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use serde::{Deserialize, Serialize};
/// ExtendedContactDetail
///
/// ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: ExtendedContactDetail
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedContactDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// The type of contact
    ///
    /// Binding: preferred (The purpose for which an extended contact detail should be used.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/contactentity-type
    pub purpose: Option<CodeableConcept>,
    /// Name of an individual to contact
    pub name: Option<Vec<HumanName>>,
    /// Contact details (e.g.phone/fax/url)
    pub telecom: Option<Vec<ContactPoint>>,
    /// Address for the contact
    pub address: Option<Address>,
    /// This contact detail is handled/monitored by a specific organization
    pub organization: Option<Reference>,
    /// Period that this contact was valid for usage
    pub period: Option<Period>,
}

impl Default for ExtendedContactDetail {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            purpose: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            address: Default::default(),
            organization: Default::default(),
            period: Default::default(),
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
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.extension", 0, None),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.name", 0, None),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.telecom", 0, None),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.address", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExtendedContactDetail.organization",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExtendedContactDetail.period", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ExtendedContactDetail {
    fn resource_type(&self) -> &'static str {
        "ExtendedContactDetail"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail")
    }
}
