use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// VirtualServiceDetail
///
/// VirtualServiceDetail Type: Virtual Service Contact Details.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: VirtualServiceDetail
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualServiceDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Channel Type
    ///
    /// Binding: example (The purpose for which an extended contact detail should be used.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/virtual-service-type
    #[serde(rename = "channelType")]
    pub channel_type: Option<Coding>,
    /// Contact address/number (url)
    #[serde(rename = "addressUrl")]
    pub address_url: Option<StringType>,
    /// Contact address/number (string)
    #[serde(rename = "addressString")]
    pub address_string: Option<StringType>,
    /// Contact address/number (ContactPoint)
    #[serde(rename = "addressContactPoint")]
    pub address_contact_point: Option<ContactPoint>,
    /// Contact address/number (ExtendedContactDetail)
    #[serde(rename = "addressExtendedContactDetail")]
    pub address_extended_contact_detail: Option<ExtendedContactDetail>,
    /// Address to see alternative connection details
    #[serde(rename = "additionalInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<StringType>,
    /// Extension element for the 'additionalInfo' primitive field. Contains metadata and extensions.
    #[serde(rename = "_additionalInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _additional_info: Vec<Element>,
    /// Maximum number of participants supported by the virtual service
    #[serde(rename = "maxParticipants")]
    pub max_participants: Option<PositiveIntType>,
    /// Extension element for the 'maxParticipants' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maxParticipants")]
    pub _max_participants: Option<Element>,
    /// Session Key required by the virtual service
    #[serde(rename = "sessionKey")]
    pub session_key: Option<StringType>,
    /// Extension element for the 'sessionKey' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sessionKey")]
    pub _session_key: Option<Element>,
}

impl Default for VirtualServiceDetail {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            channel_type: Default::default(),
            address_url: Default::default(),
            address_string: Default::default(),
            address_contact_point: Default::default(),
            address_extended_contact_detail: Default::default(),
            additional_info: Default::default(),
            _additional_info: Default::default(),
            max_participants: Default::default(),
            _max_participants: Default::default(),
            session_key: Default::default(),
            _session_key: Default::default(),
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
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.extension", 0, None),
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.channelType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.address[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.additionalInfo", 0, None),
            rh_foundation::ElementCardinality::new(
                "VirtualServiceDetail.maxParticipants",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("VirtualServiceDetail.sessionKey", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for VirtualServiceDetail {
    fn resource_type(&self) -> &'static str {
        "VirtualServiceDetail"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/VirtualServiceDetail")
    }
}
