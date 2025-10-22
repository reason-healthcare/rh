use crate::bindings::subscription_status::SubscriptionStatus;
use crate::datatypes::contact_point::ContactPoint;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::subscription::SubscriptionChannel;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Subscription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is created or updated, and if the resource matches the given criteria, it sends a message on the defined "channel" so that another system can take an appropriate action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Subscription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionAccessors: DomainResourceAccessors {
    /// Returns a reference to the status field.
    fn status(&self) -> SubscriptionStatus;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the end field.
    fn end(&self) -> Option<InstantType>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> StringType;
    /// Returns a reference to the criteria field.
    fn criteria(&self) -> StringType;
    /// Returns a reference to the error field.
    fn error(&self) -> Option<StringType>;
    /// Returns a reference to the channel field.
    fn channel(&self) -> SubscriptionChannel;
}
/// Subscription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is created or updated, and if the resource matches the given criteria, it sends a message on the defined "channel" so that another system can take an appropriate action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::subscription::Subscription;
    /// use hl7_fhir_r4_core::traits::subscription::SubscriptionMutators;
    ///
    /// let resource = Subscription::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SubscriptionStatus) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the end field and returns self for chaining.
    fn set_end(self, value: String) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: String) -> Self;
    /// Sets the criteria field and returns self for chaining.
    fn set_criteria(self, value: String) -> Self;
    /// Sets the error field and returns self for chaining.
    fn set_error(self, value: String) -> Self;
    /// Sets the channel field and returns self for chaining.
    fn set_channel(self, value: SubscriptionChannel) -> Self;
}
/// Subscription Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is created or updated, and if the resource matches the given criteria, it sends a message on the defined "channel" so that another system can take an appropriate action.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Subscription
/// - Version: 4.0.1
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the end field is present (Some).
    fn has_end(&self) -> bool;
    /// Returns true if the reason field is present (Some).
    fn has_reason(&self) -> bool;
    /// Returns true if the criteria field is present (Some).
    fn has_criteria(&self) -> bool;
    /// Returns true if the error field is present (Some).
    fn has_error(&self) -> bool;
    /// Returns true if the channel field is present (Some).
    fn has_channel(&self) -> bool;
}
