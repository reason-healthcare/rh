use crate::bindings::compartment_type::CompartmentType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::compartment_definition::CompartmentDefinitionResource;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CompartmentDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A compartment definition that defines how resources are accessed on a server.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CompartmentDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CompartmentDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompartmentDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
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
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the code field.
    fn code(&self) -> CompartmentType;
    /// Returns a reference to the search field.
    fn search(&self) -> BooleanType;
    /// Returns a reference to the resource field.
    fn resource(&self) -> &[CompartmentDefinitionResource];
}
/// CompartmentDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A compartment definition that defines how resources are accessed on a server.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CompartmentDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CompartmentDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompartmentDefinitionMutators: DomainResourceMutators {
    /// Create a new CompartmentDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::compartment_definition::CompartmentDefinition;
    /// use hl7_fhir_r4_core::traits::compartment_definition::CompartmentDefinitionMutators;
    ///
    /// let resource = CompartmentDefinition::new();
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
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CompartmentType) -> Self;
    /// Sets the search field and returns self for chaining.
    fn set_search(self, value: bool) -> Self;
    /// Sets the resource field and returns self for chaining.
    fn set_resource(self, value: Vec<CompartmentDefinitionResource>) -> Self;
    /// Adds an item to the resource field and returns self for chaining.
    fn add_resource(self, item: CompartmentDefinitionResource) -> Self;
}
/// CompartmentDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A compartment definition that defines how resources are accessed on a server.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CompartmentDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CompartmentDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompartmentDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the search field is present (Some).
    fn has_search(&self) -> bool;
    /// Returns true if the resource field is not empty.
    fn has_resource(&self) -> bool;
}
