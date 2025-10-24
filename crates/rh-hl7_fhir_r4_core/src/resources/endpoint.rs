use crate::bindings::endpoint_status::EndpointStatus;
use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Endpoint
///
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b or a REST endpoint for another FHIR server. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifies this endpoint across multiple systems
    pub identifier: Option<Vec<Identifier>>,
    /// active | suspended | error | off | entered-in-error | test
    pub status: EndpointStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Protocol/Profile/Standard to be used with this endpoint connection
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/endpoint-connection-type
    #[serde(rename = "connectionType")]
    pub connection_type: Coding,
    /// A name that this endpoint can be identified by
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Organization that manages this endpoint (might not be the organization that exposes the endpoint)
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<ContactPoint>>,
    /// Interval the endpoint is expected to be operational
    pub period: Option<Period>,
    /// The type of content that may be used at this endpoint (e.g. XDS Discharge summaries)
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `urn:ihe:pcc:handp:2008`: History and Physical Specification
    /// - `urn:ihe:pcc:xphr:2007`: HL7 CCD Document
    /// - `urn:ihe:pcc:aps:2007`: IHE Antepartum Summary
    /// - `urn:ihe:pcc:xds-ms:2007`: XDS Medical Summaries
    /// - `urn:ihe:pcc:xphr:2007`: Personal Health Records
    /// - `urn:ihe:pcc:edr:2007`: Emergency Department Referral (EDR)
    /// - `urn:ihe:pcc:edes:2007`: Emergency Department Encounter Summary (EDES)
    /// - `urn:ihe:pcc:apr:handp:2008`: Antepartum Record (APR) - History and Physical
    /// - `urn:ihe:pcc:apr:lab:2008`: Antepartum Record (APR) - Laboratory
    /// - `urn:ihe:pcc:apr:edu:2008`: Antepartum Record (APR) - Education
    /// - ... and 62 more values
    #[serde(rename = "payloadType")]
    pub payload_type: Vec<CodeableConcept>,
    /// Mimetype to send. If not specified, the content could be anything (including no payload, if the connectionType defined this)
    #[serde(rename = "payloadMimeType")]
    pub payload_mime_type: Option<Vec<Mimetypes>>,
    /// Extension element for the 'payloadMimeType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_payloadMimeType")]
    pub _payload_mime_type: Option<Element>,
    /// The technical base address for connecting to this endpoint
    pub address: StringType,
    /// Extension element for the 'address' primitive field. Contains metadata and extensions.
    pub _address: Option<Element>,
    /// Usage depends on the channel type
    pub header: Option<Vec<StringType>>,
    /// Extension element for the 'header' primitive field. Contains metadata and extensions.
    pub _header: Option<Element>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: EndpointStatus::default(),
            _status: Default::default(),
            connection_type: Coding::default(),
            name: Default::default(),
            _name: Default::default(),
            managing_organization: Default::default(),
            contact: Default::default(),
            period: Default::default(),
            payload_type: Vec::new(),
            payload_mime_type: Default::default(),
            _payload_mime_type: Default::default(),
            address: StringType::default(),
            _address: Default::default(),
            header: Default::default(),
            _header: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Endpoint {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for Endpoint {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Endpoint {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for Endpoint {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for Endpoint {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Endpoint {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::endpoint::EndpointAccessors for Endpoint {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EndpointStatus {
        self.status.clone()
    }
    fn connection_type(&self) -> Coding {
        self.connection_type.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn payload_type(&self) -> &[CodeableConcept] {
        &self.payload_type
    }
    fn payload_mime_type(&self) -> &[Mimetypes] {
        self.payload_mime_type.as_deref().unwrap_or(&[])
    }
    fn address(&self) -> StringType {
        self.address.clone()
    }
    fn header(&self) -> &[StringType] {
        self.header.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::endpoint::EndpointMutators for Endpoint {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: EndpointStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_connection_type(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.connection_type = value;
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_payload_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.payload_type = value;
        resource
    }
    fn add_payload_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.payload_type.push(item);
        resource
    }
    fn set_payload_mime_type(self, value: Vec<Mimetypes>) -> Self {
        let mut resource = self.clone();
        resource.payload_mime_type = Some(value);
        resource
    }
    fn add_payload_mime_type(self, item: Mimetypes) -> Self {
        let mut resource = self.clone();
        resource
            .payload_mime_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_address(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.address = value;
        resource
    }
    fn set_header(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.header = Some(value);
        resource
    }
    fn add_header(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.header.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::endpoint::EndpointExistence for Endpoint {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_connection_type(&self) -> bool {
        true
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_payload_type(&self) -> bool {
        !self.payload_type.is_empty()
    }
    fn has_payload_mime_type(&self) -> bool {
        self.payload_mime_type
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_address(&self) -> bool {
        true
    }
    fn has_header(&self) -> bool {
        self.header.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Endpoint {
    fn resource_type(&self) -> &'static str {
        "Endpoint"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Endpoint")
    }
}
