use crate::bindings::administrative_gender::AdministrativeGender;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::resources::patient::PatientCommunication;
use crate::resources::patient::PatientContact;
use crate::resources::patient::PatientLink;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Patient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Demographics and other administrative information about an individual or animal receiving care or other health-related services.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
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
    /// Returns a reference to the maritalStatus field.
    fn marital_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the photo field.
    fn photo(&self) -> &[Attachment];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[PatientContact];
    /// Returns a reference to the communication field.
    fn communication(&self) -> &[PatientCommunication];
    /// Returns a reference to the generalPractitioner field.
    fn general_practitioner(&self) -> &[Reference];
    /// Returns a reference to the managingOrganization field.
    fn managing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the link field.
    fn link(&self) -> &[PatientLink];
}
/// Patient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Demographics and other administrative information about an individual or animal receiving care or other health-related services.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientMutators: DomainResourceMutators {
    /// Create a new Patient with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::patient::Patient;
    /// use hl7_fhir_r4_core::traits::patient::PatientMutators;
    ///
    /// let resource = Patient::new();
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
    /// Sets the maritalStatus field and returns self for chaining.
    fn set_marital_status(self, value: CodeableConcept) -> Self;
    /// Sets the photo field and returns self for chaining.
    fn set_photo(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the photo field and returns self for chaining.
    fn add_photo(self, item: Attachment) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<PatientContact>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: PatientContact) -> Self;
    /// Sets the communication field and returns self for chaining.
    fn set_communication(self, value: Vec<PatientCommunication>) -> Self;
    /// Adds an item to the communication field and returns self for chaining.
    fn add_communication(self, item: PatientCommunication) -> Self;
    /// Sets the generalPractitioner field and returns self for chaining.
    fn set_general_practitioner(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the generalPractitioner field and returns self for chaining.
    fn add_general_practitioner(self, item: Reference) -> Self;
    /// Sets the managingOrganization field and returns self for chaining.
    fn set_managing_organization(self, value: Reference) -> Self;
    /// Sets the link field and returns self for chaining.
    fn set_link(self, value: Vec<PatientLink>) -> Self;
    /// Adds an item to the link field and returns self for chaining.
    fn add_link(self, item: PatientLink) -> Self;
}
/// Patient Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Demographics and other administrative information about an individual or animal receiving care or other health-related services.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PatientExistence: DomainResourceExistence {
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
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the telecom field is not empty.
    fn has_telecom(&self) -> bool;
    /// Returns true if the gender field is present (Some).
    fn has_gender(&self) -> bool;
    /// Returns true if the birth_date field is present (Some).
    fn has_birth_date(&self) -> bool;
    /// Returns true if the deceased field is present (Some).
    fn has_deceased(&self) -> bool;
    /// Returns true if the address field is not empty.
    fn has_address(&self) -> bool;
    /// Returns true if the marital_status field is present (Some).
    fn has_marital_status(&self) -> bool;
    /// Returns true if the multiple_birth field is present (Some).
    fn has_multiple_birth(&self) -> bool;
    /// Returns true if the photo field is not empty.
    fn has_photo(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the communication field is not empty.
    fn has_communication(&self) -> bool;
    /// Returns true if the general_practitioner field is not empty.
    fn has_general_practitioner(&self) -> bool;
    /// Returns true if the managing_organization field is present (Some).
    fn has_managing_organization(&self) -> bool;
    /// Returns true if the link field is not empty.
    fn has_link(&self) -> bool;
}
