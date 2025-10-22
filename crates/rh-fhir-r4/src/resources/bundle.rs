use crate::bindings::bundle_type::BundleType;
use crate::bindings::http_verb::HttpVerb;
use crate::bindings::search_entry_mode::SearchEntryMode;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::signature::Signature;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::resource::Resource;
use serde::{Deserialize, Serialize};
/// Bundle
///
/// A container for a collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Bundle
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Resource,
    /// Persistent identifier for the bundle
    pub identifier: Option<Identifier>,
    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection
    #[serde(rename = "type")]
    pub type_: BundleType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// When the bundle was assembled
    pub timestamp: Option<InstantType>,
    /// Extension element for the 'timestamp' primitive field. Contains metadata and extensions.
    pub _timestamp: Option<Element>,
    /// If search, the total number of matches
    pub total: Option<UnsignedIntType>,
    /// Extension element for the 'total' primitive field. Contains metadata and extensions.
    pub _total: Option<Element>,
    /// Links related to this Bundle
    pub link: Option<Vec<BundleLink>>,
    /// Entry in the bundle - will have a resource or information
    pub entry: Option<Vec<BundleEntry>>,
    /// Digital Signature
    pub signature: Option<Signature>,
}
/// BundleEntry nested structure for the 'request' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntryRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// GET | HEAD | POST | PUT | DELETE | PATCH
    pub method: HttpVerb,
    /// Extension element for the 'method' primitive field. Contains metadata and extensions.
    pub _method: Option<Element>,
    /// URL for HTTP equivalent of this entry
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// For managing cache currency
    #[serde(rename = "ifNoneMatch")]
    pub if_none_match: Option<StringType>,
    /// Extension element for the 'ifNoneMatch' primitive field. Contains metadata and extensions.
    #[serde(rename = "_ifNoneMatch")]
    pub _if_none_match: Option<Element>,
    /// For managing cache currency
    #[serde(rename = "ifModifiedSince")]
    pub if_modified_since: Option<InstantType>,
    /// Extension element for the 'ifModifiedSince' primitive field. Contains metadata and extensions.
    #[serde(rename = "_ifModifiedSince")]
    pub _if_modified_since: Option<Element>,
    /// For managing update contention
    #[serde(rename = "ifMatch")]
    pub if_match: Option<StringType>,
    /// Extension element for the 'ifMatch' primitive field. Contains metadata and extensions.
    #[serde(rename = "_ifMatch")]
    pub _if_match: Option<Element>,
    /// For conditional creates
    #[serde(rename = "ifNoneExist")]
    pub if_none_exist: Option<StringType>,
    /// Extension element for the 'ifNoneExist' primitive field. Contains metadata and extensions.
    #[serde(rename = "_ifNoneExist")]
    pub _if_none_exist: Option<Element>,
}
/// Bundle nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// See http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
    pub relation: StringType,
    /// Extension element for the 'relation' primitive field. Contains metadata and extensions.
    pub _relation: Option<Element>,
    /// Reference details for the link
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
}
/// BundleEntry nested structure for the 'response' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntryResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Status response code (text optional)
    pub status: StringType,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The location (if the operation returns a location)
    pub location: Option<StringType>,
    /// Extension element for the 'location' primitive field. Contains metadata and extensions.
    pub _location: Option<Element>,
    /// The Etag for the resource (if relevant)
    pub etag: Option<StringType>,
    /// Extension element for the 'etag' primitive field. Contains metadata and extensions.
    pub _etag: Option<Element>,
    /// Server's date time modified
    #[serde(rename = "lastModified")]
    pub last_modified: Option<InstantType>,
    /// Extension element for the 'lastModified' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastModified")]
    pub _last_modified: Option<Element>,
    /// OperationOutcome with hints and warnings (for batch/transaction)
    pub outcome: Option<Resource>,
}
/// BundleEntry nested structure for the 'search' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntrySearch {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// match | include | outcome - why this is in the result set
    pub mode: Option<SearchEntryMode>,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Search ranking (between 0 and 1)
    pub score: Option<DecimalType>,
    /// Extension element for the 'score' primitive field. Contains metadata and extensions.
    pub _score: Option<Element>,
}
/// Bundle nested structure for the 'entry' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Search related information
    pub search: Option<BundleEntrySearch>,
    /// Results of execution (transaction/batch/history)
    pub response: Option<BundleEntryResponse>,
    /// Additional execution information (transaction/batch/history)
    pub request: Option<BundleEntryRequest>,
    /// Links related to this entry
    pub link: Option<Vec<StringType>>,
    /// URI for resource (Absolute URL server address or URI for UUID/OID)
    #[serde(rename = "fullUrl")]
    pub full_url: Option<StringType>,
    /// Extension element for the 'fullUrl' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fullUrl")]
    pub _full_url: Option<Element>,
    /// A resource in the bundle
    pub resource: Option<Resource>,
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            base: Resource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            timestamp: Default::default(),
            _timestamp: Default::default(),
            total: Default::default(),
            _total: Default::default(),
            link: Default::default(),
            entry: Default::default(),
            signature: Default::default(),
        }
    }
}

impl Default for BundleEntryRequest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            method: Default::default(),
            _method: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            if_none_match: Default::default(),
            _if_none_match: Default::default(),
            if_modified_since: Default::default(),
            _if_modified_since: Default::default(),
            if_match: Default::default(),
            _if_match: Default::default(),
            if_none_exist: Default::default(),
            _if_none_exist: Default::default(),
        }
    }
}

impl Default for BundleLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relation: StringType::default(),
            _relation: Default::default(),
            url: StringType::default(),
            _url: Default::default(),
        }
    }
}

impl Default for BundleEntryResponse {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            status: Default::default(),
            _status: Default::default(),
            location: Default::default(),
            _location: Default::default(),
            etag: Default::default(),
            _etag: Default::default(),
            last_modified: Default::default(),
            _last_modified: Default::default(),
            outcome: Default::default(),
        }
    }
}

impl Default for BundleEntrySearch {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: Default::default(),
            _mode: Default::default(),
            score: Default::default(),
            _score: Default::default(),
        }
    }
}

impl Default for BundleEntry {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            search: Default::default(),
            response: Default::default(),
            request: Default::default(),
            link: Default::default(),
            full_url: Default::default(),
            _full_url: Default::default(),
            resource: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Bundle {
    fn id(&self) -> Option<String> {
        self.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for Bundle {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Bundle {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
}

impl crate::traits::bundle::BundleAccessors for Bundle {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn type_(&self) -> BundleType {
        self.type_.clone()
    }
    fn timestamp(&self) -> Option<InstantType> {
        self.timestamp.clone()
    }
    fn total(&self) -> Option<UnsignedIntType> {
        self.total
    }
    fn link(&self) -> &[BundleLink] {
        self.link.as_deref().unwrap_or(&[])
    }
    fn entry(&self) -> &[BundleEntry] {
        self.entry.as_deref().unwrap_or(&[])
    }
    fn signature(&self) -> Option<Signature> {
        self.signature.clone()
    }
}

impl crate::traits::bundle::BundleMutators for Bundle {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_type_(self, value: BundleType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_timestamp(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.timestamp = Some(value);
        resource
    }
    fn set_total(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.total = Some(value);
        resource
    }
    fn set_link(self, value: Vec<BundleLink>) -> Self {
        let mut resource = self.clone();
        resource.link = Some(value);
        resource
    }
    fn add_link(self, item: BundleLink) -> Self {
        let mut resource = self.clone();
        resource.link.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_entry(self, value: Vec<BundleEntry>) -> Self {
        let mut resource = self.clone();
        resource.entry = Some(value);
        resource
    }
    fn add_entry(self, item: BundleEntry) -> Self {
        let mut resource = self.clone();
        resource.entry.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_signature(self, value: Signature) -> Self {
        let mut resource = self.clone();
        resource.signature = Some(value);
        resource
    }
}

impl crate::traits::bundle::BundleExistence for Bundle {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }
    fn has_total(&self) -> bool {
        self.total.is_some()
    }
    fn has_link(&self) -> bool {
        self.link.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_entry(&self) -> bool {
        self.entry.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_signature(&self) -> bool {
        self.signature.is_some()
    }
}
