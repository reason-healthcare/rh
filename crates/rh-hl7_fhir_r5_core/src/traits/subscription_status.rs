use crate::bindings::subscription_notification_type::SubscriptionNotificationType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::reference::Reference;
use crate::primitives::integer64::Integer64Type;
use crate::primitives::string::StringType;
use crate::resources::subscription_status::SubscriptionStatusNotificationevent;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubscriptionStatus Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionStatus
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionStatusAccessors: DomainResourceAccessors {
    /// Returns a reference to the status field.
    fn status(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> SubscriptionNotificationType;
    /// Returns a reference to the eventsSinceSubscriptionStart field.
    fn events_since_subscription_start(&self) -> Option<Integer64Type>;
    /// Returns a reference to the notificationEvent field.
    fn notification_event(&self) -> &[SubscriptionStatusNotificationevent];
    /// Returns a reference to the subscription field.
    fn subscription(&self) -> Reference;
    /// Returns a reference to the topic field.
    fn topic(&self) -> Option<StringType>;
    /// Returns a reference to the error field.
    fn error(&self) -> &[CodeableConcept];
}
/// SubscriptionStatus Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionStatus
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionStatusMutators: DomainResourceMutators {
    /// Create a new SubscriptionStatus with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::subscription_status::SubscriptionStatus;
    /// use rh_hl7_fhir_r5_core::traits::subscription_status::SubscriptionStatusMutators;
    ///
    /// let resource = SubscriptionStatus::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: StringType) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: SubscriptionNotificationType) -> Self;
    /// Sets the eventsSinceSubscriptionStart field and returns self for chaining.
    fn set_events_since_subscription_start(self, value: Integer64Type) -> Self;
    /// Sets the notificationEvent field and returns self for chaining.
    fn set_notification_event(self, value: Vec<SubscriptionStatusNotificationevent>) -> Self;
    /// Adds an item to the notificationEvent field and returns self for chaining.
    fn add_notification_event(self, item: SubscriptionStatusNotificationevent) -> Self;
    /// Sets the subscription field and returns self for chaining.
    fn set_subscription(self, value: Reference) -> Self;
    /// Sets the topic field and returns self for chaining.
    fn set_topic(self, value: String) -> Self;
    /// Sets the error field and returns self for chaining.
    fn set_error(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the error field and returns self for chaining.
    fn add_error(self, item: CodeableConcept) -> Self;
}
/// SubscriptionStatus Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionStatus
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionStatusExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the events_since_subscription_start field is present (Some).
    fn has_events_since_subscription_start(&self) -> bool;
    /// Returns true if the notification_event field is not empty.
    fn has_notification_event(&self) -> bool;
    /// Returns true if the subscription field is present (Some).
    fn has_subscription(&self) -> bool;
    /// Returns true if the topic field is present (Some).
    fn has_topic(&self) -> bool;
    /// Returns true if the error field is not empty.
    fn has_error(&self) -> bool;
}
