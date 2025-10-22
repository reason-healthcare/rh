use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::subscription_channel_type::SubscriptionChannelType;
use crate::bindings::subscription_status::SubscriptionStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Subscription
///
/// The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is created or updated, and if the resource matches the given criteria, it sends a message on the defined "channel" so that another system can take an appropriate action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// requested | active | error | off
    pub status: SubscriptionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<ContactPoint>>,
    /// When to automatically delete the subscription
    pub end: Option<InstantType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Description of why this subscription was created
    pub reason: StringType,
    /// Extension element for the 'reason' primitive field. Contains metadata and extensions.
    pub _reason: Option<Element>,
    /// Rule for server push
    pub criteria: StringType,
    /// Extension element for the 'criteria' primitive field. Contains metadata and extensions.
    pub _criteria: Option<Element>,
    /// Latest error note
    pub error: Option<StringType>,
    /// Extension element for the 'error' primitive field. Contains metadata and extensions.
    pub _error: Option<Element>,
    /// The channel on which to report matches to the criteria
    pub channel: SubscriptionChannel,
}
/// Subscription nested structure for the 'channel' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionChannel {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// rest-hook | websocket | email | sms | message
    #[serde(rename = "type")]
    pub type_: SubscriptionChannelType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Where the channel points to
    pub endpoint: Option<StringType>,
    /// Extension element for the 'endpoint' primitive field. Contains metadata and extensions.
    pub _endpoint: Option<Element>,
    /// MIME type to send, or omit for no payload
    pub payload: Option<Mimetypes>,
    /// Extension element for the 'payload' primitive field. Contains metadata and extensions.
    pub _payload: Option<Element>,
    /// Usage depends on the channel type
    pub header: Option<Vec<StringType>>,
    /// Extension element for the 'header' primitive field. Contains metadata and extensions.
    pub _header: Option<Element>,
}

impl Default for Subscription {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            status: SubscriptionStatus::default(),
            _status: Default::default(),
            contact: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            reason: StringType::default(),
            _reason: Default::default(),
            criteria: StringType::default(),
            _criteria: Default::default(),
            error: Default::default(),
            _error: Default::default(),
            channel: SubscriptionChannel::default(),
        }
    }
}

impl Default for SubscriptionChannel {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            endpoint: Default::default(),
            _endpoint: Default::default(),
            payload: Default::default(),
            _payload: Default::default(),
            header: Default::default(),
            _header: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Subscription {
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

impl crate::traits::resource::ResourceMutators for Subscription {
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

impl crate::traits::resource::ResourceExistence for Subscription {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Subscription {
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

impl crate::traits::domain_resource::DomainResourceMutators for Subscription {
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

impl crate::traits::domain_resource::DomainResourceExistence for Subscription {
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

impl crate::traits::subscription::SubscriptionAccessors for Subscription {
    fn status(&self) -> SubscriptionStatus {
        self.status.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn end(&self) -> Option<InstantType> {
        self.end.clone()
    }
    fn reason(&self) -> StringType {
        self.reason.clone()
    }
    fn criteria(&self) -> StringType {
        self.criteria.clone()
    }
    fn error(&self) -> Option<StringType> {
        self.error.clone()
    }
    fn channel(&self) -> SubscriptionChannel {
        self.channel.clone()
    }
}

impl crate::traits::subscription::SubscriptionMutators for Subscription {
    fn new() -> Self {
        Self::default()
    }
    fn set_status(self, value: SubscriptionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
    fn set_end(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.end = Some(value);
        resource
    }
    fn set_reason(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn set_criteria(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.criteria = value;
        resource
    }
    fn set_error(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.error = Some(value);
        resource
    }
    fn set_channel(self, value: SubscriptionChannel) -> Self {
        let mut resource = self.clone();
        resource.channel = value;
        resource
    }
}

impl crate::traits::subscription::SubscriptionExistence for Subscription {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_end(&self) -> bool {
        self.end.is_some()
    }
    fn has_reason(&self) -> bool {
        true
    }
    fn has_criteria(&self) -> bool {
        true
    }
    fn has_error(&self) -> bool {
        self.error.is_some()
    }
    fn has_channel(&self) -> bool {
        true
    }
}
