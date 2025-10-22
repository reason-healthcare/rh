use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Meta
///
/// Base StructureDefinition for Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Meta
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Meta
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Version specific identifier
    #[serde(rename = "versionId")]
    pub version_id: Option<StringType>,
    /// Extension element for the 'versionId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_versionId")]
    pub _version_id: Option<Element>,
    /// When the resource version last changed
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<InstantType>,
    /// Extension element for the 'lastUpdated' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastUpdated")]
    pub _last_updated: Option<Element>,
    /// Identifies where the resource comes from
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// Profiles this resource claims to conform to
    pub profile: Option<Vec<StringType>>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Security Labels applied to this resource
    ///
    /// Binding: extensible (Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-labels
    pub security: Option<Vec<Coding>>,
    /// Tags applied to this resource
    ///
    /// Binding: example (Codes that represent various types of tags, commonly workflow-related; e.g. "Needs review by Dr. Jones".)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/common-tags
    pub tag: Option<Vec<Coding>>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            base: Element::default(),
            version_id: Default::default(),
            _version_id: Default::default(),
            last_updated: Default::default(),
            _last_updated: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            security: Default::default(),
            tag: Default::default(),
        }
    }
}
