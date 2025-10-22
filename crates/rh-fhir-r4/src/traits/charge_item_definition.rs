use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::charge_item_definition::ChargeItemDefinitionApplicability;
use crate::resources::charge_item_definition::ChargeItemDefinitionPropertygroup;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ChargeItemDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, therefore this resource gives only a rough structure and requires profiling for each type of billing code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFromUri field.
    fn derived_from_uri(&self) -> &[StringType];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[StringType];
    /// Returns a reference to the replaces field.
    fn replaces(&self) -> &[StringType];
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
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the instance field.
    fn instance(&self) -> &[Reference];
    /// Returns a reference to the applicability field.
    fn applicability(&self) -> &[ChargeItemDefinitionApplicability];
    /// Returns a reference to the propertyGroup field.
    fn property_group(&self) -> &[ChargeItemDefinitionPropertygroup];
}
/// ChargeItemDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, therefore this resource gives only a rough structure and requires profiling for each type of billing code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemDefinitionMutators: DomainResourceMutators {
    /// Create a new ChargeItemDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::charge_item_definition::ChargeItemDefinition;
    /// use hl7_fhir_r4_core::traits::charge_item_definition::ChargeItemDefinitionMutators;
    ///
    /// let resource = ChargeItemDefinition::new();
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
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the derivedFromUri field and returns self for chaining.
    fn set_derived_from_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFromUri field and returns self for chaining.
    fn add_derived_from_uri(self, item: String) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<String>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: String) -> Self;
    /// Sets the replaces field and returns self for chaining.
    fn set_replaces(self, value: Vec<String>) -> Self;
    /// Adds an item to the replaces field and returns self for chaining.
    fn add_replaces(self, item: String) -> Self;
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
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the instance field and returns self for chaining.
    fn add_instance(self, item: Reference) -> Self;
    /// Sets the applicability field and returns self for chaining.
    fn set_applicability(self, value: Vec<ChargeItemDefinitionApplicability>) -> Self;
    /// Adds an item to the applicability field and returns self for chaining.
    fn add_applicability(self, item: ChargeItemDefinitionApplicability) -> Self;
    /// Sets the propertyGroup field and returns self for chaining.
    fn set_property_group(self, value: Vec<ChargeItemDefinitionPropertygroup>) -> Self;
    /// Adds an item to the propertyGroup field and returns self for chaining.
    fn add_property_group(self, item: ChargeItemDefinitionPropertygroup) -> Self;
}
/// ChargeItemDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, therefore this resource gives only a rough structure and requires profiling for each type of billing code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ChargeItemDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ChargeItemDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ChargeItemDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the derived_from_uri field is not empty.
    fn has_derived_from_uri(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the replaces field is not empty.
    fn has_replaces(&self) -> bool;
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
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the instance field is not empty.
    fn has_instance(&self) -> bool;
    /// Returns true if the applicability field is not empty.
    fn has_applicability(&self) -> bool;
    /// Returns true if the property_group field is not empty.
    fn has_property_group(&self) -> bool;
}
