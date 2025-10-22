use crate::bindings::response_code::ResponseCode;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MessageHeader
///
/// The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MessageHeader
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Code for the event this message represents or link to event definition (Coding)
    #[serde(rename = "eventCoding")]
    pub event_coding: Coding,
    /// Code for the event this message represents or link to event definition (uri)
    #[serde(rename = "eventUri")]
    pub event_uri: StringType,
    /// Message destination application(s)
    pub destination: Option<Vec<MessageHeaderDestination>>,
    /// Real world sender of the message
    pub sender: Option<Reference>,
    /// The source of the data entry
    pub enterer: Option<Reference>,
    /// The source of the decision
    pub author: Option<Reference>,
    /// Message source application
    pub source: MessageHeaderSource,
    /// Final responsibility for event
    pub responsible: Option<Reference>,
    /// Cause of event
    ///
    /// Binding: example (Reason for event occurrence.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/message-reason-encounter
    pub reason: Option<CodeableConcept>,
    /// If this is a reply to prior message
    pub response: Option<MessageHeaderResponse>,
    /// The actual content of the message
    pub focus: Option<Vec<Reference>>,
    /// Link to the definition for this message
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
}
/// MessageHeader nested structure for the 'source' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeaderSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of system
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name of software running the system
    pub software: Option<StringType>,
    /// Extension element for the 'software' primitive field. Contains metadata and extensions.
    pub _software: Option<Element>,
    /// Version of software running
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Human contact for problems
    pub contact: Option<ContactPoint>,
    /// Actual message source address or id
    pub endpoint: StringType,
    /// Extension element for the 'endpoint' primitive field. Contains metadata and extensions.
    pub _endpoint: Option<Element>,
}
/// MessageHeader nested structure for the 'destination' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeaderDestination {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of system
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Particular delivery destination within the destination
    pub target: Option<Reference>,
    /// Actual destination address or id
    pub endpoint: StringType,
    /// Extension element for the 'endpoint' primitive field. Contains metadata and extensions.
    pub _endpoint: Option<Element>,
    /// Intended "real-world" recipient for the data
    pub receiver: Option<Reference>,
}
/// MessageHeader nested structure for the 'response' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeaderResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Id of original message
    pub identifier: StringType,
    /// Extension element for the 'identifier' primitive field. Contains metadata and extensions.
    pub _identifier: Option<Element>,
    /// ok | transient-error | fatal-error
    pub code: ResponseCode,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Specific list of hints/warnings/errors
    pub details: Option<Reference>,
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            event_coding: Default::default(),
            event_uri: Default::default(),
            destination: Default::default(),
            sender: Default::default(),
            enterer: Default::default(),
            author: Default::default(),
            source: MessageHeaderSource::default(),
            responsible: Default::default(),
            reason: Default::default(),
            response: Default::default(),
            focus: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
        }
    }
}

impl Default for MessageHeaderSource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            software: Default::default(),
            _software: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            contact: Default::default(),
            endpoint: StringType::default(),
            _endpoint: Default::default(),
        }
    }
}

impl Default for MessageHeaderDestination {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            target: Default::default(),
            endpoint: StringType::default(),
            _endpoint: Default::default(),
            receiver: Default::default(),
        }
    }
}

impl Default for MessageHeaderResponse {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: StringType::default(),
            _identifier: Default::default(),
            code: ResponseCode::default(),
            _code: Default::default(),
            details: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MessageHeader {
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

impl crate::traits::resource::ResourceMutators for MessageHeader {
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

impl crate::traits::resource::ResourceExistence for MessageHeader {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MessageHeader {
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

impl crate::traits::domain_resource::DomainResourceMutators for MessageHeader {
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

impl crate::traits::domain_resource::DomainResourceExistence for MessageHeader {
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

impl crate::traits::message_header::MessageHeaderAccessors for MessageHeader {
    fn destination(&self) -> &[MessageHeaderDestination] {
        self.destination.as_deref().unwrap_or(&[])
    }
    fn sender(&self) -> Option<Reference> {
        self.sender.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn source(&self) -> MessageHeaderSource {
        self.source.clone()
    }
    fn responsible(&self) -> Option<Reference> {
        self.responsible.clone()
    }
    fn reason(&self) -> Option<CodeableConcept> {
        self.reason.clone()
    }
    fn response(&self) -> Option<MessageHeaderResponse> {
        self.response.clone()
    }
    fn focus(&self) -> &[Reference] {
        self.focus.as_deref().unwrap_or(&[])
    }
    fn definition(&self) -> Option<StringType> {
        self.definition.clone()
    }
}

impl crate::traits::message_header::MessageHeaderMutators for MessageHeader {
    fn new() -> Self {
        Self::default()
    }
    fn set_destination(self, value: Vec<MessageHeaderDestination>) -> Self {
        let mut resource = self.clone();
        resource.destination = Some(value);
        resource
    }
    fn add_destination(self, item: MessageHeaderDestination) -> Self {
        let mut resource = self.clone();
        resource.destination.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_sender(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.sender = Some(value);
        resource
    }
    fn set_enterer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.enterer = Some(value);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_source(self, value: MessageHeaderSource) -> Self {
        let mut resource = self.clone();
        resource.source = value;
        resource
    }
    fn set_responsible(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.responsible = Some(value);
        resource
    }
    fn set_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn set_response(self, value: MessageHeaderResponse) -> Self {
        let mut resource = self.clone();
        resource.response = Some(value);
        resource
    }
    fn set_focus(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn add_focus(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_definition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
}

impl crate::traits::message_header::MessageHeaderExistence for MessageHeader {
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
    fn has_event(&self) -> bool {
        true
    }
    fn has_destination(&self) -> bool {
        self.destination.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_sender(&self) -> bool {
        self.sender.is_some()
    }
    fn has_enterer(&self) -> bool {
        self.enterer.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_source(&self) -> bool {
        true
    }
    fn has_responsible(&self) -> bool {
        self.responsible.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.is_some()
    }
    fn has_response(&self) -> bool {
        self.response.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_definition(&self) -> bool {
        self.definition.is_some()
    }
}
