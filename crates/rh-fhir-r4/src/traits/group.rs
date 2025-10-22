use crate::bindings::group_type::GroupType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::group::GroupCharacteristic;
use crate::resources::group::GroupMember;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Group Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Group
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GroupAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> GroupType;
    /// Returns a reference to the actual field.
    fn actual(&self) -> BooleanType;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the managingEntity field.
    fn managing_entity(&self) -> Option<Reference>;
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[GroupCharacteristic];
    /// Returns a reference to the member field.
    fn member(&self) -> &[GroupMember];
}
/// Group Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Group
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GroupMutators: DomainResourceMutators {
    /// Create a new Group with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::group::Group;
    /// use hl7_fhir_r4_core::traits::group::GroupMutators;
    ///
    /// let resource = Group::new();
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
    fn set_type_(self, value: GroupType) -> Self;
    /// Sets the actual field and returns self for chaining.
    fn set_actual(self, value: bool) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: i32) -> Self;
    /// Sets the managingEntity field and returns self for chaining.
    fn set_managing_entity(self, value: Reference) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<GroupCharacteristic>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: GroupCharacteristic) -> Self;
    /// Sets the member field and returns self for chaining.
    fn set_member(self, value: Vec<GroupMember>) -> Self;
    /// Adds an item to the member field and returns self for chaining.
    fn add_member(self, item: GroupMember) -> Self;
}
/// Group Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Group
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Group
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GroupExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the actual field is present (Some).
    fn has_actual(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the managing_entity field is present (Some).
    fn has_managing_entity(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the member field is not empty.
    fn has_member(&self) -> bool;
}
