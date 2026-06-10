use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::subscription_payload_content::SubscriptionPayloadContent;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::subscription::SubscriptionFilterby;
use crate::resources::subscription::SubscriptionParameter;
use crate::resources::subscription_status::SubscriptionStatus;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Subscription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> SubscriptionStatus;
    /// Returns a reference to the topic field.
    fn topic(&self) -> StringType;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the end field.
    fn end(&self) -> Option<InstantType>;
    /// Returns a reference to the managingEntity field.
    fn managing_entity(&self) -> Option<Reference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> Option<StringType>;
    /// Returns a reference to the filterBy field.
    fn filter_by(&self) -> &[SubscriptionFilterby];
    /// Returns a reference to the channelType field.
    fn channel_type(&self) -> Coding;
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> Option<StringType>;
    /// Returns a reference to the parameter field.
    fn parameter(&self) -> &[SubscriptionParameter];
    /// Returns a reference to the heartbeatPeriod field.
    fn heartbeat_period(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the timeout field.
    fn timeout(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the contentType field.
    fn content_type(&self) -> Option<Mimetypes>;
    /// Returns a reference to the content field.
    fn content(&self) -> Option<SubscriptionPayloadContent>;
    /// Returns a reference to the maxCount field.
    fn max_count(&self) -> Option<PositiveIntType>;
}
/// Subscription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionMutators: DomainResourceMutators {
    /// Create a new Subscription with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::subscription::Subscription;
    /// use rh_hl7_fhir_r5_core::traits::subscription::SubscriptionMutators;
    ///
    /// let resource = Subscription::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SubscriptionStatus) -> Self;
    /// Sets the topic field and returns self for chaining.
    fn set_topic(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the end field and returns self for chaining.
    fn set_end(self, value: String) -> Self;
    /// Sets the managingEntity field and returns self for chaining.
    fn set_managing_entity(self, value: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: String) -> Self;
    /// Sets the filterBy field and returns self for chaining.
    fn set_filter_by(self, value: Vec<SubscriptionFilterby>) -> Self;
    /// Adds an item to the filterBy field and returns self for chaining.
    fn add_filter_by(self, item: SubscriptionFilterby) -> Self;
    /// Sets the channelType field and returns self for chaining.
    fn set_channel_type(self, value: Coding) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: String) -> Self;
    /// Sets the parameter field and returns self for chaining.
    fn set_parameter(self, value: Vec<SubscriptionParameter>) -> Self;
    /// Adds an item to the parameter field and returns self for chaining.
    fn add_parameter(self, item: SubscriptionParameter) -> Self;
    /// Sets the heartbeatPeriod field and returns self for chaining.
    fn set_heartbeat_period(self, value: i32) -> Self;
    /// Sets the timeout field and returns self for chaining.
    fn set_timeout(self, value: i32) -> Self;
    /// Sets the contentType field and returns self for chaining.
    fn set_content_type(self, value: Mimetypes) -> Self;
    /// Sets the content field and returns self for chaining.
    fn set_content(self, value: SubscriptionPayloadContent) -> Self;
    /// Sets the maxCount field and returns self for chaining.
    fn set_max_count(self, value: i32) -> Self;
}
/// Subscription Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the topic field is present (Some).
    fn has_topic(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the end field is present (Some).
    fn has_end(&self) -> bool;
    /// Returns true if the managing_entity field is present (Some).
    fn has_managing_entity(&self) -> bool;
    /// Returns true if the reason field is present (Some).
    fn has_reason(&self) -> bool;
    /// Returns true if the filter_by field is not empty.
    fn has_filter_by(&self) -> bool;
    /// Returns true if the channel_type field is present (Some).
    fn has_channel_type(&self) -> bool;
    /// Returns true if the endpoint field is present (Some).
    fn has_endpoint(&self) -> bool;
    /// Returns true if the parameter field is not empty.
    fn has_parameter(&self) -> bool;
    /// Returns true if the heartbeat_period field is present (Some).
    fn has_heartbeat_period(&self) -> bool;
    /// Returns true if the timeout field is present (Some).
    fn has_timeout(&self) -> bool;
    /// Returns true if the content_type field is present (Some).
    fn has_content_type(&self) -> bool;
    /// Returns true if the content field is present (Some).
    fn has_content(&self) -> bool;
    /// Returns true if the max_count field is present (Some).
    fn has_max_count(&self) -> bool;
}
