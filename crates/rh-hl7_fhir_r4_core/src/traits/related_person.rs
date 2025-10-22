use crate::bindings::administrative_gender::AdministrativeGender;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::resources::related_person::RelatedPersonCommunication;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// RelatedPerson Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Information about a person that is involved in the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RelatedPerson
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RelatedPersonAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the relationship field.
    fn relationship(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> &[HumanName];
    /// Returns a reference to the telecom field.
    fn telecom(&self) -> &[ContactPoint];
    /// Returns a reference to the gender field.
    fn gender(&self) -> Option<AdministrativeGender>;
    /// Returns a reference to the birthDate field.
    fn birth_date(&self) -> Option<DateType>;
    /// Returns a reference to the address field.
    fn address(&self) -> &[Address];
    /// Returns a reference to the photo field.
    fn photo(&self) -> &[Attachment];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the communication field.
    fn communication(&self) -> &[RelatedPersonCommunication];
}
/// RelatedPerson Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Information about a person that is involved in the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RelatedPerson
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RelatedPersonMutators: DomainResourceMutators {
    /// Create a new RelatedPerson with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::related_person::RelatedPerson;
    /// use hl7_fhir_r4_core::traits::related_person::RelatedPersonMutators;
    ///
    /// let resource = RelatedPerson::new();
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
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the relationship field and returns self for chaining.
    fn set_relationship(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the relationship field and returns self for chaining.
    fn add_relationship(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<HumanName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: HumanName) -> Self;
    /// Sets the telecom field and returns self for chaining.
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the telecom field and returns self for chaining.
    fn add_telecom(self, item: ContactPoint) -> Self;
    /// Sets the gender field and returns self for chaining.
    fn set_gender(self, value: AdministrativeGender) -> Self;
    /// Sets the birthDate field and returns self for chaining.
    fn set_birth_date(self, value: String) -> Self;
    /// Sets the address field and returns self for chaining.
    fn set_address(self, value: Vec<Address>) -> Self;
    /// Adds an item to the address field and returns self for chaining.
    fn add_address(self, item: Address) -> Self;
    /// Sets the photo field and returns self for chaining.
    fn set_photo(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the photo field and returns self for chaining.
    fn add_photo(self, item: Attachment) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the communication field and returns self for chaining.
    fn set_communication(self, value: Vec<RelatedPersonCommunication>) -> Self;
    /// Adds an item to the communication field and returns self for chaining.
    fn add_communication(self, item: RelatedPersonCommunication) -> Self;
}
/// RelatedPerson Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Information about a person that is involved in the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RelatedPerson
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RelatedPersonExistence: DomainResourceExistence {
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
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the relationship field is not empty.
    fn has_relationship(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the telecom field is not empty.
    fn has_telecom(&self) -> bool;
    /// Returns true if the gender field is present (Some).
    fn has_gender(&self) -> bool;
    /// Returns true if the birth_date field is present (Some).
    fn has_birth_date(&self) -> bool;
    /// Returns true if the address field is not empty.
    fn has_address(&self) -> bool;
    /// Returns true if the photo field is not empty.
    fn has_photo(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the communication field is not empty.
    fn has_communication(&self) -> bool;
}
