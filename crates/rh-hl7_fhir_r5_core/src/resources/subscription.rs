use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::search_comparator::SearchComparator;
use crate::bindings::search_modifier_code::SearchModifierCode;
use crate::bindings::subscription_payload_content::SubscriptionPayloadContent;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use crate::resources::subscription_status::SubscriptionStatus;
use serde::{Deserialize, Serialize};
/// Subscription
///
/// The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Additional identifiers (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Human readable name for this subscription
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// requested | active | error | off | entered-in-error
    pub status: SubscriptionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reference to the subscription topic being subscribed to
    pub topic: StringType,
    /// Extension element for the 'topic' primitive field. Contains metadata and extensions.
    pub _topic: Option<Element>,
    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactPoint>,
    /// When to automatically delete the subscription
    pub end: Option<InstantType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Entity responsible for Subscription changes
    #[serde(rename = "managingEntity")]
    pub managing_entity: Option<Reference>,
    /// Description of why this subscription was created
    pub reason: Option<StringType>,
    /// Extension element for the 'reason' primitive field. Contains metadata and extensions.
    pub _reason: Option<Element>,
    /// Criteria for narrowing the subscription topic stream
    #[serde(rename = "filterBy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter_by: Vec<SubscriptionFilterby>,
    /// Channel type for notifications
    ///
    /// Binding: extensible (The type of method used to execute a subscription.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/subscription-channel-type
    #[serde(rename = "channelType")]
    pub channel_type: Coding,
    /// Where the channel points to
    pub endpoint: Option<StringType>,
    /// Extension element for the 'endpoint' primitive field. Contains metadata and extensions.
    pub _endpoint: Option<Element>,
    /// Channel type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<SubscriptionParameter>,
    /// Interval in seconds to send 'heartbeat' notification
    #[serde(rename = "heartbeatPeriod")]
    pub heartbeat_period: Option<UnsignedIntType>,
    /// Extension element for the 'heartbeatPeriod' primitive field. Contains metadata and extensions.
    #[serde(rename = "_heartbeatPeriod")]
    pub _heartbeat_period: Option<Element>,
    /// Timeout in seconds to attempt notification delivery
    pub timeout: Option<UnsignedIntType>,
    /// Extension element for the 'timeout' primitive field. Contains metadata and extensions.
    pub _timeout: Option<Element>,
    /// MIME type to send, or omit for no payload
    #[serde(rename = "contentType")]
    pub content_type: Option<Mimetypes>,
    /// Extension element for the 'contentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentType")]
    pub _content_type: Option<Element>,
    /// empty | id-only | full-resource
    pub content: Option<SubscriptionPayloadContent>,
    /// Extension element for the 'content' primitive field. Contains metadata and extensions.
    pub _content: Option<Element>,
    /// Maximum number of events that can be combined in a single notification
    #[serde(rename = "maxCount")]
    pub max_count: Option<PositiveIntType>,
    /// Extension element for the 'maxCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maxCount")]
    pub _max_count: Option<Element>,
}
/// Subscription nested structure for the 'filterBy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionFilterby {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Allowed Resource (reference to definition) for this Subscription filter
    ///
    /// Binding: extensible (A type of resource, or a Reference (from all versions))
    ///
    /// Available values:
    /// - `Reference`
    #[serde(rename = "resourceType")]
    pub resource_type: Option<StringType>,
    /// Extension element for the 'resourceType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_resourceType")]
    pub _resource_type: Option<Element>,
    /// Filter label defined in SubscriptionTopic
    #[serde(rename = "filterParameter")]
    pub filter_parameter: StringType,
    /// Extension element for the 'filterParameter' primitive field. Contains metadata and extensions.
    #[serde(rename = "_filterParameter")]
    pub _filter_parameter: Option<Element>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<SearchComparator>,
    /// Extension element for the 'comparator' primitive field. Contains metadata and extensions.
    pub _comparator: Option<Element>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<SearchModifierCode>,
    /// Extension element for the 'modifier' primitive field. Contains metadata and extensions.
    pub _modifier: Option<Element>,
    /// Literal value or resource path
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// Subscription nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name (key) of the parameter
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Value of the parameter to use or pass through
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}

impl Default for Subscription {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            status: SubscriptionStatus::default(),
            _status: Default::default(),
            topic: StringType::default(),
            _topic: Default::default(),
            contact: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            managing_entity: Default::default(),
            reason: Default::default(),
            _reason: Default::default(),
            filter_by: Default::default(),
            channel_type: Coding::default(),
            endpoint: Default::default(),
            _endpoint: Default::default(),
            parameter: Default::default(),
            heartbeat_period: Default::default(),
            _heartbeat_period: Default::default(),
            timeout: Default::default(),
            _timeout: Default::default(),
            content_type: Default::default(),
            _content_type: Default::default(),
            content: Default::default(),
            _content: Default::default(),
            max_count: Default::default(),
            _max_count: Default::default(),
        }
    }
}

impl Default for SubscriptionFilterby {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            resource_type: Default::default(),
            _resource_type: Default::default(),
            filter_parameter: Default::default(),
            _filter_parameter: Default::default(),
            comparator: Default::default(),
            _comparator: Default::default(),
            modifier: Default::default(),
            _modifier: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for SubscriptionParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
            value: StringType::default(),
            _value: Default::default(),
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
    rh_foundation::Invariant::new("scr-1", rh_foundation::Severity::Error, "Subscription filters may only contain a modifier or a comparator", "(comparator.exists() and modifier.exists()).not()"),
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
                "Subscription.content",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-payload-content|5.0.0",
            )
            .with_description(
                "Codes to represent how much resource content to send in the notification payload.",
            ),
            rh_foundation::ElementBinding::new(
                "Subscription.contentType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "Subscription.filterBy.comparator",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/search-comparator|5.0.0",
            )
            .with_description("Search Comparator Codes applied to this filter."),
            rh_foundation::ElementBinding::new(
                "Subscription.filterBy.modifier",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/search-modifier-code|5.0.0",
            )
            .with_description("Search Modifier Code applied to this filter."),
            rh_foundation::ElementBinding::new(
                "Subscription.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Subscription.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/subscription-status|5.0.0",
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
            rh_foundation::ElementCardinality::new("Subscription.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.topic", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.contact", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.end", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.managingEntity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.reason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.filterBy", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.filterBy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.filterBy.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Subscription.filterBy.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Subscription.filterBy.resourceType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Subscription.filterBy.filterParameter",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Subscription.filterBy.comparator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.filterBy.modifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.filterBy.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.channelType", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.endpoint", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.parameter", 0, None),
            rh_foundation::ElementCardinality::new("Subscription.parameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.parameter.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Subscription.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Subscription.parameter.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.parameter.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.heartbeatPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.timeout", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.contentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.content", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Subscription.maxCount", 0, Some(1)),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for Subscription {
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

impl crate::traits::subscription::SubscriptionAccessors for Subscription {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn status(&self) -> SubscriptionStatus {
        self.status.clone()
    }
    fn topic(&self) -> StringType {
        self.topic.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_slice()
    }
    fn end(&self) -> Option<InstantType> {
        self.end.clone()
    }
    fn managing_entity(&self) -> Option<Reference> {
        self.managing_entity.clone()
    }
    fn reason(&self) -> Option<StringType> {
        self.reason.clone()
    }
    fn filter_by(&self) -> &[SubscriptionFilterby] {
        self.filter_by.as_slice()
    }
    fn channel_type(&self) -> Coding {
        self.channel_type.clone()
    }
    fn endpoint(&self) -> Option<StringType> {
        self.endpoint.clone()
    }
    fn parameter(&self) -> &[SubscriptionParameter] {
        self.parameter.as_slice()
    }
    fn heartbeat_period(&self) -> Option<UnsignedIntType> {
        self.heartbeat_period
    }
    fn timeout(&self) -> Option<UnsignedIntType> {
        self.timeout
    }
    fn content_type(&self) -> Option<Mimetypes> {
        self.content_type.clone()
    }
    fn content(&self) -> Option<SubscriptionPayloadContent> {
        self.content.clone()
    }
    fn max_count(&self) -> Option<PositiveIntType> {
        self.max_count
    }
}

impl crate::traits::subscription::SubscriptionMutators for Subscription {
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
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_status(self, value: SubscriptionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_topic(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.topic = value;
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
    fn set_end(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.end = Some(value);
        resource
    }
    fn set_managing_entity(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_entity = Some(value);
        resource
    }
    fn set_reason(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn set_filter_by(self, value: Vec<SubscriptionFilterby>) -> Self {
        let mut resource = self.clone();
        resource.filter_by = value;
        resource
    }
    fn add_filter_by(self, item: SubscriptionFilterby) -> Self {
        let mut resource = self.clone();
        resource.filter_by.push(item);
        resource
    }
    fn set_channel_type(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.channel_type = value;
        resource
    }
    fn set_endpoint(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.endpoint = Some(value);
        resource
    }
    fn set_parameter(self, value: Vec<SubscriptionParameter>) -> Self {
        let mut resource = self.clone();
        resource.parameter = value;
        resource
    }
    fn add_parameter(self, item: SubscriptionParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.push(item);
        resource
    }
    fn set_heartbeat_period(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.heartbeat_period = Some(value);
        resource
    }
    fn set_timeout(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.timeout = Some(value);
        resource
    }
    fn set_content_type(self, value: Mimetypes) -> Self {
        let mut resource = self.clone();
        resource.content_type = Some(value);
        resource
    }
    fn set_content(self, value: SubscriptionPayloadContent) -> Self {
        let mut resource = self.clone();
        resource.content = Some(value);
        resource
    }
    fn set_max_count(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.max_count = Some(value);
        resource
    }
}

impl crate::traits::subscription::SubscriptionExistence for Subscription {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_topic(&self) -> bool {
        true
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_end(&self) -> bool {
        self.end.is_some()
    }
    fn has_managing_entity(&self) -> bool {
        self.managing_entity.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.is_some()
    }
    fn has_filter_by(&self) -> bool {
        !self.filter_by.is_empty()
    }
    fn has_channel_type(&self) -> bool {
        true
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.is_some()
    }
    fn has_parameter(&self) -> bool {
        !self.parameter.is_empty()
    }
    fn has_heartbeat_period(&self) -> bool {
        self.heartbeat_period.is_some()
    }
    fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }
    fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }
    fn has_content(&self) -> bool {
        self.content.is_some()
    }
    fn has_max_count(&self) -> bool {
        self.max_count.is_some()
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
