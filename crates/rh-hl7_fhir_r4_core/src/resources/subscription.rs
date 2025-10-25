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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "Subscription.channel.payload",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1",
            )
            .with_description("The mime type of an attachment. Any valid mime type is allowed."),
            rh_foundation::ElementBinding::new(
                "Subscription.channel.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-channel-type|4.0.1",
            )
            .with_description("The type of method used to execute a subscription."),
            rh_foundation::ElementBinding::new(
                "Subscription.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-status|4.0.1",
            )
            .with_description("The status of a subscription."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Subscription.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.contained", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.extension", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.contact", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.end", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.reason", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.criteria", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.error", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Subscription.channel.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Subscription.channel.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel.endpoint", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel.payload", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channel.header", 0, None),
        ]
    });

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

impl crate::validation::ValidatableResource for Subscription {
    fn resource_type(&self) -> &'static str {
        "Subscription"
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
        Some("http://hl7.org/fhir/StructureDefinition/Subscription")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::subscription::{
    SubscriptionAccessors, SubscriptionExistence, SubscriptionMutators,
};
