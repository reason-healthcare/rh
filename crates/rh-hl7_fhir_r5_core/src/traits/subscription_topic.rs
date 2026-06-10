use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::subscription_topic::SubscriptionTopicCanfilterby;
use crate::resources::subscription_topic::SubscriptionTopicEventtrigger;
use crate::resources::subscription_topic::SubscriptionTopicNotificationshape;
use crate::resources::subscription_topic::SubscriptionTopicResourcetrigger;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubscriptionTopic Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionTopic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionTopicAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the copyrightLabel field.
    fn copyright_label(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the resourceTrigger field.
    fn resource_trigger(&self) -> &[SubscriptionTopicResourcetrigger];
    /// Returns a reference to the eventTrigger field.
    fn event_trigger(&self) -> &[SubscriptionTopicEventtrigger];
    /// Returns a reference to the canFilterBy field.
    fn can_filter_by(&self) -> &[SubscriptionTopicCanfilterby];
    /// Returns a reference to the notificationShape field.
    fn notification_shape(&self) -> &[SubscriptionTopicNotificationshape];
}
/// SubscriptionTopic Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionTopic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionTopicMutators: DomainResourceMutators {
    /// Create a new SubscriptionTopic with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::subscription_topic::SubscriptionTopic;
    /// use rh_hl7_fhir_r5_core::traits::subscription_topic::SubscriptionTopicMutators;
    ///
    /// let resource = SubscriptionTopic::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the copyrightLabel field and returns self for chaining.
    fn set_copyright_label(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the resourceTrigger field and returns self for chaining.
    fn set_resource_trigger(self, value: Vec<SubscriptionTopicResourcetrigger>) -> Self;
    /// Adds an item to the resourceTrigger field and returns self for chaining.
    fn add_resource_trigger(self, item: SubscriptionTopicResourcetrigger) -> Self;
    /// Sets the eventTrigger field and returns self for chaining.
    fn set_event_trigger(self, value: Vec<SubscriptionTopicEventtrigger>) -> Self;
    /// Adds an item to the eventTrigger field and returns self for chaining.
    fn add_event_trigger(self, item: SubscriptionTopicEventtrigger) -> Self;
    /// Sets the canFilterBy field and returns self for chaining.
    fn set_can_filter_by(self, value: Vec<SubscriptionTopicCanfilterby>) -> Self;
    /// Adds an item to the canFilterBy field and returns self for chaining.
    fn add_can_filter_by(self, item: SubscriptionTopicCanfilterby) -> Self;
    /// Sets the notificationShape field and returns self for chaining.
    fn set_notification_shape(self, value: Vec<SubscriptionTopicNotificationshape>) -> Self;
    /// Adds an item to the notificationShape field and returns self for chaining.
    fn add_notification_shape(self, item: SubscriptionTopicNotificationshape) -> Self;
}
/// SubscriptionTopic Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionTopic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubscriptionTopicExistence: DomainResourceExistence {
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the copyright_label field is present (Some).
    fn has_copyright_label(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the resource_trigger field is not empty.
    fn has_resource_trigger(&self) -> bool;
    /// Returns true if the event_trigger field is not empty.
    fn has_event_trigger(&self) -> bool;
    /// Returns true if the can_filter_by field is not empty.
    fn has_can_filter_by(&self) -> bool;
    /// Returns true if the notification_shape field is not empty.
    fn has_notification_shape(&self) -> bool;
}
