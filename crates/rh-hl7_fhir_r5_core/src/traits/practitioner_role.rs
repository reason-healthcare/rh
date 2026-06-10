use crate::datatypes::availability::Availability;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// PractitionerRole Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 5.0.0
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
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ExtendedContactDetail];
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[CodeableConcept];
    /// Returns a reference to the communication field.
    fn communication(&self) -> &[CodeableConcept];
    /// Returns a reference to the availability field.
    fn availability(&self) -> &[Availability];
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
}
/// PractitionerRole Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::practitioner_role::PractitionerRole;
    /// use rh_hl7_fhir_r5_core::traits::practitioner_role::PractitionerRoleMutators;
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
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ExtendedContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ExtendedContactDetail) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: CodeableConcept) -> Self;
    /// Sets the communication field and returns self for chaining.
    fn set_communication(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the communication field and returns self for chaining.
    fn add_communication(self, item: CodeableConcept) -> Self;
    /// Sets the availability field and returns self for chaining.
    fn set_availability(self, value: Vec<Availability>) -> Self;
    /// Adds an item to the availability field and returns self for chaining.
    fn add_availability(self, item: Availability) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
}
/// PractitionerRole Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PractitionerRole
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PractitionerRoleExistence: DomainResourceExistence {
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
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the communication field is not empty.
    fn has_communication(&self) -> bool;
    /// Returns true if the availability field is not empty.
    fn has_availability(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
}
