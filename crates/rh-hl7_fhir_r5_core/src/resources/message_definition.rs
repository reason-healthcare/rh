use crate::bindings::message_significance_category::MessageSignificanceCategory;
use crate::bindings::messageheader_response_request::MessageheaderResponseRequest;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MessageDefinition
///
/// Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MessageDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The cannonical URL for a given MessageDefinition
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business Identifier for a given MessageDefinition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the message definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this message definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this message definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Takes the place of
    pub replaces: Option<Vec<StringType>>,
    /// Extension element for the 'replaces' primitive field. Contains metadata and extensions.
    pub _replaces: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the message definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for message definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this message definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// Definition this one is based on
    #[serde(rename = "base")]
    pub base_definition: Option<StringType>,
    /// Extension element for the 'base' primitive field. Contains metadata and extensions.
    pub _base: Option<Element>,
    /// Protocol/workflow this is part of
    pub parent: Option<Vec<StringType>>,
    /// Extension element for the 'parent' primitive field. Contains metadata and extensions.
    pub _parent: Option<Element>,
    /// Event code  or link to the EventDefinition (Coding)
    #[serde(rename = "eventCoding")]
    pub event_coding: Coding,
    /// Event code  or link to the EventDefinition (uri)
    #[serde(rename = "eventUri")]
    pub event_uri: StringType,
    /// consequence | currency | notification
    pub category: Option<MessageSignificanceCategory>,
    /// Extension element for the 'category' primitive field. Contains metadata and extensions.
    pub _category: Option<Element>,
    /// Resource(s) that are the subject of the event
    pub focus: Option<Vec<MessageDefinitionFocus>>,
    /// always | on-error | never | on-success
    #[serde(rename = "responseRequired")]
    pub response_required: Option<MessageheaderResponseRequest>,
    /// Extension element for the 'responseRequired' primitive field. Contains metadata and extensions.
    #[serde(rename = "_responseRequired")]
    pub _response_required: Option<Element>,
    /// Responses to this message
    #[serde(rename = "allowedResponse")]
    pub allowed_response: Option<Vec<MessageDefinitionAllowedresponse>>,
    /// Canonical reference to a GraphDefinition
    pub graph: Option<StringType>,
    /// Extension element for the 'graph' primitive field. Contains metadata and extensions.
    pub _graph: Option<Element>,
}
/// MessageDefinition nested structure for the 'focus' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDefinitionFocus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of resource
    pub code: ResourceTypes,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Profile that must be adhered to by focus
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Minimum number of focuses of this type
    pub min: UnsignedIntType,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Maximum number of focuses of this type
    pub max: Option<StringType>,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
}
/// MessageDefinition nested structure for the 'allowedResponse' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDefinitionAllowedresponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to allowed message definition response
    pub message: StringType,
    /// Extension element for the 'message' primitive field. Contains metadata and extensions.
    pub _message: Option<Element>,
    /// When should this response be used
    pub situation: Option<StringType>,
    /// Extension element for the 'situation' primitive field. Contains metadata and extensions.
    pub _situation: Option<Element>,
}

impl Default for MessageDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            replaces: Default::default(),
            _replaces: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            base_definition: Default::default(),
            _base: Default::default(),
            parent: Default::default(),
            _parent: Default::default(),
            event_coding: Default::default(),
            event_uri: Default::default(),
            category: Default::default(),
            _category: Default::default(),
            focus: Default::default(),
            response_required: Default::default(),
            _response_required: Default::default(),
            allowed_response: Default::default(),
            graph: Default::default(),
            _graph: Default::default(),
        }
    }
}

impl Default for MessageDefinitionFocus {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: ResourceTypes::default(),
            _code: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            min: UnsignedIntType::default(),
            _min: Default::default(),
            max: Default::default(),
            _max: Default::default(),
        }
    }
}

impl Default for MessageDefinitionAllowedresponse {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            message: Default::default(),
            _message: Default::default(),
            situation: Default::default(),
            _situation: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("md-1", rh_foundation::Severity::Error, "Max must be postive int or *", "max='*' or (max.toInteger() > 0)"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("MessageDefinition.category", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/message-significance-category|5.0.0").with_description("The impact of the content of a message."),
    rh_foundation::ElementBinding::new("MessageDefinition.focus.code", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-types|5.0.0").with_description("One of the resource types defined as part of this version of FHIR."),
    rh_foundation::ElementBinding::new("MessageDefinition.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("MessageDefinition.responseRequired", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/messageheader-response-request|5.0.0").with_description("This enables the capability currently available through MSH-16 (Application Level acknowledgement) in HL7 Version 2 to declare at a message definition level whether a response is required or only upon error or success, or never."),
    rh_foundation::ElementBinding::new("MessageDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MessageDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MessageDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.replaces", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.base", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.parent", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.event[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus", 0, None),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.focus.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.profile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.min", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MessageDefinition.focus.max", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.responseRequired",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MessageDefinition.allowedResponse", 0, None),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.allowedResponse.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.allowedResponse.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.allowedResponse.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.allowedResponse.message",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MessageDefinition.allowedResponse.situation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MessageDefinition.graph", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MessageDefinition {
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

impl crate::traits::resource::ResourceMutators for MessageDefinition {
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

impl crate::traits::resource::ResourceExistence for MessageDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MessageDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for MessageDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for MessageDefinition {
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

impl crate::traits::message_definition::MessageDefinitionAccessors for MessageDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn replaces(&self) -> &[StringType] {
        self.replaces.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn base_definition(&self) -> Option<StringType> {
        self.base_definition.clone()
    }
    fn parent(&self) -> &[StringType] {
        self.parent.as_deref().unwrap_or(&[])
    }
    fn category(&self) -> Option<MessageSignificanceCategory> {
        self.category.clone()
    }
    fn focus(&self) -> &[MessageDefinitionFocus] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn response_required(&self) -> Option<MessageheaderResponseRequest> {
        self.response_required.clone()
    }
    fn allowed_response(&self) -> &[MessageDefinitionAllowedresponse] {
        self.allowed_response.as_deref().unwrap_or(&[])
    }
    fn graph(&self) -> Option<StringType> {
        self.graph.clone()
    }
}

impl crate::traits::message_definition::MessageDefinitionMutators for MessageDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
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
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_replaces(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.replaces = Some(value);
        resource
    }
    fn add_replaces(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.replaces.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_base_definition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base_definition = Some(value);
        resource
    }
    fn set_parent(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
    fn add_parent(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.parent.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_category(self, value: MessageSignificanceCategory) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn set_focus(self, value: Vec<MessageDefinitionFocus>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: MessageDefinitionFocus) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_response_required(self, value: MessageheaderResponseRequest) -> Self {
        let mut resource = self.clone();
        resource.response_required = Some(value);
        resource
    }
    fn set_allowed_response(self, value: Vec<MessageDefinitionAllowedresponse>) -> Self {
        let mut resource = self.clone();
        resource.allowed_response = Some(value);
        resource
    }
    fn add_allowed_response(self, item: MessageDefinitionAllowedresponse) -> Self {
        let mut resource = self.clone();
        resource
            .allowed_response
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_graph(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.graph = Some(value);
        resource
    }
}

impl crate::traits::message_definition::MessageDefinitionExistence for MessageDefinition {
    fn has_event(&self) -> bool {
        true
    }
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_base_definition(&self) -> bool {
        self.base_definition.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_category(&self) -> bool {
        self.category.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_response_required(&self) -> bool {
        self.response_required.is_some()
    }
    fn has_allowed_response(&self) -> bool {
        self.allowed_response
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_graph(&self) -> bool {
        self.graph.is_some()
    }
}

impl crate::validation::ValidatableResource for MessageDefinition {
    fn resource_type(&self) -> &'static str {
        "MessageDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/MessageDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::message_definition::{
    MessageDefinitionAccessors, MessageDefinitionExistence, MessageDefinitionMutators,
};
