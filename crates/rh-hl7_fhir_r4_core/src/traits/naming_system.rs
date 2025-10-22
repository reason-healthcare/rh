use crate::bindings::namingsystem_type::NamingsystemType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::naming_system::NamingSystemUniqueid;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemAccessors: DomainResourceAccessors {
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the kind field.
    fn kind(&self) -> NamingsystemType;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the responsible field.
    fn responsible(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the usage field.
    fn usage(&self) -> Option<StringType>;
    /// Returns a reference to the uniqueId field.
    fn unique_id(&self) -> &[NamingSystemUniqueid];
}
/// NamingSystem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemMutators: DomainResourceMutators {
    /// Create a new NamingSystem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::naming_system::NamingSystem;
    /// use hl7_fhir_r4_core::traits::naming_system::NamingSystemMutators;
    ///
    /// let resource = NamingSystem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: NamingsystemType) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the responsible field and returns self for chaining.
    fn set_responsible(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
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
    /// Sets the usage field and returns self for chaining.
    fn set_usage(self, value: String) -> Self;
    /// Sets the uniqueId field and returns self for chaining.
    fn set_unique_id(self, value: Vec<NamingSystemUniqueid>) -> Self;
    /// Adds an item to the uniqueId field and returns self for chaining.
    fn add_unique_id(self, item: NamingSystemUniqueid) -> Self;
}
/// NamingSystem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NamingSystemExistence: DomainResourceExistence {
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
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the responsible field is present (Some).
    fn has_responsible(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the usage field is present (Some).
    fn has_usage(&self) -> bool;
    /// Returns true if the unique_id field is not empty.
    fn has_unique_id(&self) -> bool;
}
