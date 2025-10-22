use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::bindings::search_comparator::SearchComparator;
use crate::bindings::search_modifier_code::SearchModifierCode;
use crate::bindings::search_param_type::SearchParamType;
use crate::bindings::search_xpath_usage::SearchXpathUsage;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::search_parameter::SearchParameterComponent;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SearchParameter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A search parameter that defines a named search item that can be used to search/filter on a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SearchParameter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SearchParameterAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> Option<StringType>;
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
    fn description(&self) -> StringType;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the code field.
    fn code(&self) -> StringType;
    /// Returns a reference to the base field.
    fn base_definition(&self) -> &[ResourceTypes];
    /// Returns a reference to the type field.
    fn type_(&self) -> SearchParamType;
    /// Returns a reference to the expression field.
    fn expression(&self) -> Option<StringType>;
    /// Returns a reference to the xpath field.
    fn xpath(&self) -> Option<StringType>;
    /// Returns a reference to the xpathUsage field.
    fn xpath_usage(&self) -> Option<SearchXpathUsage>;
    /// Returns a reference to the target field.
    fn target(&self) -> &[ResourceTypes];
    /// Returns a reference to the multipleOr field.
    fn multiple_or(&self) -> Option<BooleanType>;
    /// Returns a reference to the multipleAnd field.
    fn multiple_and(&self) -> Option<BooleanType>;
    /// Returns a reference to the comparator field.
    fn comparator(&self) -> &[SearchComparator];
    /// Returns a reference to the modifier field.
    fn modifier(&self) -> &[SearchModifierCode];
    /// Returns a reference to the chain field.
    fn chain(&self) -> &[StringType];
    /// Returns a reference to the component field.
    fn component(&self) -> &[SearchParameterComponent];
}
/// SearchParameter Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A search parameter that defines a named search item that can be used to search/filter on a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SearchParameter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SearchParameterMutators: DomainResourceMutators {
    /// Create a new SearchParameter with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::search_parameter::SearchParameter;
    /// use hl7_fhir_r4_core::traits::search_parameter::SearchParameterMutators;
    ///
    /// let resource = SearchParameter::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: String) -> Self;
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
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: String) -> Self;
    /// Sets the base field and returns self for chaining.
    fn set_base_definition(self, value: Vec<ResourceTypes>) -> Self;
    /// Adds an item to the base field and returns self for chaining.
    fn add_base_definition(self, item: ResourceTypes) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: SearchParamType) -> Self;
    /// Sets the expression field and returns self for chaining.
    fn set_expression(self, value: String) -> Self;
    /// Sets the xpath field and returns self for chaining.
    fn set_xpath(self, value: String) -> Self;
    /// Sets the xpathUsage field and returns self for chaining.
    fn set_xpath_usage(self, value: SearchXpathUsage) -> Self;
    /// Sets the target field and returns self for chaining.
    fn set_target(self, value: Vec<ResourceTypes>) -> Self;
    /// Adds an item to the target field and returns self for chaining.
    fn add_target(self, item: ResourceTypes) -> Self;
    /// Sets the multipleOr field and returns self for chaining.
    fn set_multiple_or(self, value: bool) -> Self;
    /// Sets the multipleAnd field and returns self for chaining.
    fn set_multiple_and(self, value: bool) -> Self;
    /// Sets the comparator field and returns self for chaining.
    fn set_comparator(self, value: Vec<SearchComparator>) -> Self;
    /// Adds an item to the comparator field and returns self for chaining.
    fn add_comparator(self, item: SearchComparator) -> Self;
    /// Sets the modifier field and returns self for chaining.
    fn set_modifier(self, value: Vec<SearchModifierCode>) -> Self;
    /// Adds an item to the modifier field and returns self for chaining.
    fn add_modifier(self, item: SearchModifierCode) -> Self;
    /// Sets the chain field and returns self for chaining.
    fn set_chain(self, value: Vec<String>) -> Self;
    /// Adds an item to the chain field and returns self for chaining.
    fn add_chain(self, item: String) -> Self;
    /// Sets the component field and returns self for chaining.
    fn set_component(self, value: Vec<SearchParameterComponent>) -> Self;
    /// Adds an item to the component field and returns self for chaining.
    fn add_component(self, item: SearchParameterComponent) -> Self;
}
/// SearchParameter Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A search parameter that defines a named search item that can be used to search/filter on a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SearchParameter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SearchParameterExistence: DomainResourceExistence {
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
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the derived_from field is present (Some).
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the base_definition field is not empty.
    fn has_base_definition(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the expression field is present (Some).
    fn has_expression(&self) -> bool;
    /// Returns true if the xpath field is present (Some).
    fn has_xpath(&self) -> bool;
    /// Returns true if the xpath_usage field is present (Some).
    fn has_xpath_usage(&self) -> bool;
    /// Returns true if the target field is not empty.
    fn has_target(&self) -> bool;
    /// Returns true if the multiple_or field is present (Some).
    fn has_multiple_or(&self) -> bool;
    /// Returns true if the multiple_and field is present (Some).
    fn has_multiple_and(&self) -> bool;
    /// Returns true if the comparator field is not empty.
    fn has_comparator(&self) -> bool;
    /// Returns true if the modifier field is not empty.
    fn has_modifier(&self) -> bool;
    /// Returns true if the chain field is not empty.
    fn has_chain(&self) -> bool;
    /// Returns true if the component field is not empty.
    fn has_component(&self) -> bool;
}
