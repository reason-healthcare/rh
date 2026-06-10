use crate::traits::resource::ResourceExistence;
/// Subscription Notification bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a Subscription Notification Bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/subscription-notification-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SubscriptionNotificationBundleAccessors {}
/// Subscription Notification bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a Subscription Notification Bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/subscription-notification-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SubscriptionNotificationBundleMutators {
    /// Create a new SubscriptionNotificationBundle with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::profiles::subscription_notification_bundle::SubscriptionNotificationBundle;
    /// use rh_hl7_fhir_r5_core::traits::subscription_notification_bundle::SubscriptionNotificationBundleMutators;
    ///
    /// let resource = SubscriptionNotificationBundle::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// subscription-notification-bundle Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This profile holds all the requirements and constraints related to a Subscription Notification Bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/subscription-notification-bundle
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Bundle
pub trait SubscriptionNotificationBundleExistence: ResourceExistence {}
