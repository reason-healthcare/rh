use crate::datatypes::address::Address;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::organization::OrganizationContact;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Organization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Organization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Organization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OrganizationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the alias field.
    fn alias(&self) -> &[StringType];
    /// Returns a reference to the telecom field.
    fn telecom(&self) -> &[ContactPoint];
    /// Returns a reference to the address field.
    fn address(&self) -> &[Address];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[OrganizationContact];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
}
/// Organization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Organization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Organization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OrganizationMutators: DomainResourceMutators {
    /// Create a new Organization with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::organization::Organization;
    /// use hl7_fhir_r4_core::traits::organization::OrganizationMutators;
    ///
    /// let resource = Organization::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the active field and returns self for chaining.
    fn set_active(self, value: bool) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the alias field and returns self for chaining.
    fn set_alias(self, value: Vec<String>) -> Self;
    /// Adds an item to the alias field and returns self for chaining.
    fn add_alias(self, item: String) -> Self;
    /// Sets the telecom field and returns self for chaining.
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the telecom field and returns self for chaining.
    fn add_telecom(self, item: ContactPoint) -> Self;
    /// Sets the address field and returns self for chaining.
    fn set_address(self, value: Vec<Address>) -> Self;
    /// Adds an item to the address field and returns self for chaining.
    fn add_address(self, item: Address) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<OrganizationContact>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: OrganizationContact) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
}
/// Organization Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Organization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Organization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait OrganizationExistence: DomainResourceExistence {
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
    /// Returns true if the active field is present (Some).
    fn has_active(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the alias field is not empty.
    fn has_alias(&self) -> bool;
    /// Returns true if the telecom field is not empty.
    fn has_telecom(&self) -> bool;
    /// Returns true if the address field is not empty.
    fn has_address(&self) -> bool;
    /// Returns true if the part_of field is present (Some).
    fn has_part_of(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
}
