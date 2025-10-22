use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::practitioner_role::PractitionerRoleAvailabletime;
use crate::resources::practitioner_role::PractitionerRoleNotavailable;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// PractitionerRole Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PractitionerRole
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PractitionerRoleAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the practitioner field.
    fn practitioner(&self) -> Option<Reference>;
    /// Returns a reference to the organization field.
    fn organization(&self) -> Option<Reference>;
    /// Returns a reference to the code field.
    fn code(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialty field.
    fn specialty(&self) -> &[CodeableConcept];
    /// Returns a reference to the location field.
    fn location(&self) -> &[Reference];
    /// Returns a reference to the healthcareService field.
    fn healthcare_service(&self) -> &[Reference];
    /// Returns a reference to the telecom field.
    fn telecom(&self) -> &[ContactPoint];
    /// Returns a reference to the availableTime field.
    fn available_time(&self) -> &[PractitionerRoleAvailabletime];
    /// Returns a reference to the notAvailable field.
    fn not_available(&self) -> &[PractitionerRoleNotavailable];
    /// Returns a reference to the availabilityExceptions field.
    fn availability_exceptions(&self) -> Option<StringType>;
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
}
/// PractitionerRole Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PractitionerRole
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PractitionerRoleMutators: DomainResourceMutators {
    /// Create a new PractitionerRole with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::practitioner_role::PractitionerRole;
    /// use hl7_fhir_r4_core::traits::practitioner_role::PractitionerRoleMutators;
    ///
    /// let resource = PractitionerRole::new();
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
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the practitioner field and returns self for chaining.
    fn set_practitioner(self, value: Reference) -> Self;
    /// Sets the organization field and returns self for chaining.
    fn set_organization(self, value: Reference) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: CodeableConcept) -> Self;
    /// Sets the specialty field and returns self for chaining.
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialty field and returns self for chaining.
    fn add_specialty(self, item: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: Reference) -> Self;
    /// Sets the healthcareService field and returns self for chaining.
    fn set_healthcare_service(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the healthcareService field and returns self for chaining.
    fn add_healthcare_service(self, item: Reference) -> Self;
    /// Sets the telecom field and returns self for chaining.
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the telecom field and returns self for chaining.
    fn add_telecom(self, item: ContactPoint) -> Self;
    /// Sets the availableTime field and returns self for chaining.
    fn set_available_time(self, value: Vec<PractitionerRoleAvailabletime>) -> Self;
    /// Adds an item to the availableTime field and returns self for chaining.
    fn add_available_time(self, item: PractitionerRoleAvailabletime) -> Self;
    /// Sets the notAvailable field and returns self for chaining.
    fn set_not_available(self, value: Vec<PractitionerRoleNotavailable>) -> Self;
    /// Adds an item to the notAvailable field and returns self for chaining.
    fn add_not_available(self, item: PractitionerRoleNotavailable) -> Self;
    /// Sets the availabilityExceptions field and returns self for chaining.
    fn set_availability_exceptions(self, value: String) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
}
/// PractitionerRole Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PractitionerRole
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PractitionerRoleExistence: DomainResourceExistence {
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
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the practitioner field is present (Some).
    fn has_practitioner(&self) -> bool;
    /// Returns true if the organization field is present (Some).
    fn has_organization(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the specialty field is not empty.
    fn has_specialty(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
    /// Returns true if the healthcare_service field is not empty.
    fn has_healthcare_service(&self) -> bool;
    /// Returns true if the telecom field is not empty.
    fn has_telecom(&self) -> bool;
    /// Returns true if the available_time field is not empty.
    fn has_available_time(&self) -> bool;
    /// Returns true if the not_available field is not empty.
    fn has_not_available(&self) -> bool;
    /// Returns true if the availability_exceptions field is present (Some).
    fn has_availability_exceptions(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
}
