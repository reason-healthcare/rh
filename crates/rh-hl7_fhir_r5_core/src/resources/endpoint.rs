use crate::bindings::endpoint_status::EndpointStatus;
use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
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
/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Endpoint
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Endpoint
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifies this endpoint across multiple systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// active | suspended | error | off | entered-in-error | test
    pub status: EndpointStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Protocol/Profile/Standard to be used with this endpoint connection
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/endpoint-connection-type
    #[serde(rename = "connectionType")]
    pub connection_type: Vec<CodeableConcept>,
    /// A name that this endpoint can be identified by
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Additional details about the endpoint that could be displayed as further information to identify the description beyond its name
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The type of environment(s) exposed at this endpoint
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/endpoint-environment
    #[serde(rename = "environmentType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_type: Vec<CodeableConcept>,
    /// Organization that manages this endpoint (might not be the organization that exposes the endpoint)
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactPoint>,
    /// Interval the endpoint is expected to be operational
    pub period: Option<Period>,
    /// Set of payloads that are provided by this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<EndpointPayload>,
    /// The technical base address for connecting to this endpoint
    pub address: StringType,
    /// Extension element for the 'address' primitive field. Contains metadata and extensions.
    pub _address: Option<Element>,
    /// Usage depends on the channel type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header: Vec<StringType>,
    /// Extension element for the 'header' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _header: Vec<Element>,
}
/// Endpoint nested structure for the 'payload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointPayload {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of content that may be used at this endpoint (e.g. XDS Discharge summaries)
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/endpoint-payload-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
    /// Mimetype to send. If not specified, the content could be anything (including no payload, if the connectionType defined this)
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mime_type: Vec<Mimetypes>,
    /// Extension element for the 'mimeType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_mimeType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _mime_type: Vec<Element>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: EndpointStatus::default(),
            _status: Default::default(),
            connection_type: Vec::new(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            environment_type: Default::default(),
            managing_organization: Default::default(),
            contact: Default::default(),
            period: Default::default(),
            payload: Default::default(),
            address: StringType::default(),
            _address: Default::default(),
            header: Default::default(),
            _header: Default::default(),
        }
    }
}

impl Default for EndpointPayload {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            mime_type: Default::default(),
            _mime_type: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "Endpoint.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Endpoint.payload.mimeType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "Endpoint.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/endpoint-status|5.0.0",
            )
            .with_description("The status of the endpoint."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Endpoint.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.contained", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.extension", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.connectionType", 1, None),
            rh_foundation::ElementCardinality::new("Endpoint.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.environmentType", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.managingOrganization", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.contact", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.payload", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.payload.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.payload.extension", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.payload.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.payload.type", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.payload.mimeType", 0, None),
            rh_foundation::ElementCardinality::new("Endpoint.address", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Endpoint.header", 0, None),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Endpoint {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::endpoint::EndpointAccessors for Endpoint {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn status(&self) -> EndpointStatus {
        self.status.clone()
    }
    fn connection_type(&self) -> &[CodeableConcept] {
        &self.connection_type
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn environment_type(&self) -> &[CodeableConcept] {
        self.environment_type.as_slice()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_slice()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn payload(&self) -> &[EndpointPayload] {
        self.payload.as_slice()
    }
    fn address(&self) -> StringType {
        self.address.clone()
    }
    fn header(&self) -> &[StringType] {
        self.header.as_slice()
    }
}

impl crate::traits::endpoint::EndpointMutators for Endpoint {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_status(self, value: EndpointStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_connection_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.connection_type = value;
        resource
    }
    fn add_connection_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.connection_type.push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_environment_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.environment_type = value;
        resource
    }
    fn add_environment_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.environment_type.push(item);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_payload(self, value: Vec<EndpointPayload>) -> Self {
        let mut resource = self.clone();
        resource.payload = value;
        resource
    }
    fn add_payload(self, item: EndpointPayload) -> Self {
        let mut resource = self.clone();
        resource.payload.push(item);
        resource
    }
    fn set_address(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.address = value;
        resource
    }
    fn set_header(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.header = value;
        resource
    }
    fn add_header(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.header.push(item);
        resource
    }
}

impl crate::traits::endpoint::EndpointExistence for Endpoint {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_connection_type(&self) -> bool {
        !self.connection_type.is_empty()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_environment_type(&self) -> bool {
        !self.environment_type.is_empty()
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_payload(&self) -> bool {
        !self.payload.is_empty()
    }
    fn has_address(&self) -> bool {
        true
    }
    fn has_header(&self) -> bool {
        !self.header.is_empty()
    }
}

impl crate::validation::ValidatableResource for Endpoint {
    fn resource_type(&self) -> &'static str {
        "Endpoint"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Endpoint")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::endpoint::{EndpointAccessors, EndpointExistence, EndpointMutators};
