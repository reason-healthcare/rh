use crate::bindings::subscription_notification_type::SubscriptionNotificationType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::integer64::Integer64Type;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubscriptionStatus
///
/// The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionStatus
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// requested | active | error | off | entered-in-error
    pub status: Option<StringType>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// handshake | heartbeat | event-notification | query-status | query-event
    #[serde(rename = "type")]
    pub type_: SubscriptionNotificationType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Events since the Subscription was created
    #[serde(rename = "eventsSinceSubscriptionStart")]
    pub events_since_subscription_start: Option<Integer64Type>,
    /// Detailed information about any events relevant to this notification
    #[serde(rename = "notificationEvent")]
    pub notification_event: Option<Vec<SubscriptionStatusNotificationevent>>,
    /// Reference to the Subscription responsible for this notification
    pub subscription: Reference,
    /// Reference to the SubscriptionTopic this notification relates to
    pub topic: Option<StringType>,
    /// Extension element for the 'topic' primitive field. Contains metadata and extensions.
    pub _topic: Option<Element>,
    /// List of errors on the subscription
    ///
    /// Binding: example (Codes to represent subscription error details.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/subscription-error
    pub error: Option<Vec<CodeableConcept>>,
}
/// SubscriptionStatus nested structure for the 'notificationEvent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStatusNotificationevent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Sequencing index of this event
    #[serde(rename = "eventNumber")]
    pub event_number: Integer64Type,
    /// The instant this event occurred
    pub timestamp: Option<InstantType>,
    /// Extension element for the 'timestamp' primitive field. Contains metadata and extensions.
    pub _timestamp: Option<Element>,
    /// Reference to the primary resource or information of this event
    pub focus: Option<Reference>,
    /// References related to the focus resource and/or context of this event
    #[serde(rename = "additionalContext")]
    pub additional_context: Option<Vec<Reference>>,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            status: Default::default(),
            _status: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            events_since_subscription_start: Default::default(),
            notification_event: Default::default(),
            subscription: Reference::default(),
            topic: Default::default(),
            _topic: Default::default(),
            error: Default::default(),
        }
    }
}

impl Default for SubscriptionStatusNotificationevent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            event_number: Default::default(),
            timestamp: Default::default(),
            _timestamp: Default::default(),
            focus: Default::default(),
            additional_context: Default::default(),
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
    rh_foundation::Invariant::new("sst-1", rh_foundation::Severity::Error, "Event notifications must contain events", "(type = 'event-notification' or type = 'query-event') implies notificationEvent.exists()"),
    rh_foundation::Invariant::new("sst-2", rh_foundation::Severity::Error, "Status messages must contain status", "type = 'query-status' implies status.exists()"),
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
                "SubscriptionStatus.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "SubscriptionStatus.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-status|5.0.0",
            )
            .with_description(
                "The status of a subscription at the time this notification was generated.",
            ),
            rh_foundation::ElementBinding::new(
                "SubscriptionStatus.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-notification-type|5.0.0",
            )
            .with_description("The type of notification represented by the status message."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SubscriptionStatus.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.contained", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.extension", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.eventsSinceSubscriptionStart",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.notificationEvent", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.eventNumber",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.timestamp",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.focus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionStatus.notificationEvent.additionalContext",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.subscription", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.topic", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionStatus.error", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubscriptionStatus {
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

impl crate::traits::resource::ResourceMutators for SubscriptionStatus {
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

impl crate::traits::resource::ResourceExistence for SubscriptionStatus {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubscriptionStatus {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubscriptionStatus {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubscriptionStatus {
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

impl crate::traits::subscription_status::SubscriptionStatusAccessors for SubscriptionStatus {
    fn status(&self) -> Option<StringType> {
        self.status.clone()
    }
    fn type_(&self) -> SubscriptionNotificationType {
        self.type_.clone()
    }
    fn events_since_subscription_start(&self) -> Option<Integer64Type> {
        self.events_since_subscription_start.clone()
    }
    fn notification_event(&self) -> &[SubscriptionStatusNotificationevent] {
        self.notification_event.as_deref().unwrap_or(&[])
    }
    fn subscription(&self) -> Reference {
        self.subscription.clone()
    }
    fn topic(&self) -> Option<StringType> {
        self.topic.clone()
    }
    fn error(&self) -> &[CodeableConcept] {
        self.error.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::subscription_status::SubscriptionStatusMutators for SubscriptionStatus {
    fn new() -> Self {
        Self::default()
    }
    fn set_status(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_type_(self, value: SubscriptionNotificationType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_events_since_subscription_start(self, value: Integer64Type) -> Self {
        let mut resource = self.clone();
        resource.events_since_subscription_start = Some(value);
        resource
    }
    fn set_notification_event(self, value: Vec<SubscriptionStatusNotificationevent>) -> Self {
        let mut resource = self.clone();
        resource.notification_event = Some(value);
        resource
    }
    fn add_notification_event(self, item: SubscriptionStatusNotificationevent) -> Self {
        let mut resource = self.clone();
        resource
            .notification_event
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subscription(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subscription = value;
        resource
    }
    fn set_topic(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.topic = Some(value);
        resource
    }
    fn set_error(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.error = Some(value);
        resource
    }
    fn add_error(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.error.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::subscription_status::SubscriptionStatusExistence for SubscriptionStatus {
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_events_since_subscription_start(&self) -> bool {
        self.events_since_subscription_start.is_some()
    }
    fn has_notification_event(&self) -> bool {
        self.notification_event
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_subscription(&self) -> bool {
        true
    }
    fn has_topic(&self) -> bool {
        self.topic.is_some()
    }
    fn has_error(&self) -> bool {
        self.error.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SubscriptionStatus {
    fn resource_type(&self) -> &'static str {
        "SubscriptionStatus"
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
        Some("http://hl7.org/fhir/StructureDefinition/SubscriptionStatus")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::subscription_status::{
    SubscriptionStatusAccessors, SubscriptionStatusExistence, SubscriptionStatusMutators,
};
