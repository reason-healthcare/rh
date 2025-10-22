use crate::bindings::episode_of_care_status::EpisodeOfCareStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::resources::episode_of_care::EpisodeOfCareDiagnosis;
use crate::resources::episode_of_care::EpisodeOfCareStatushistory;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// EpisodeOfCare Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EpisodeOfCare
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EpisodeOfCareAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> EpisodeOfCareStatus;
    /// Returns a reference to the statusHistory field.
    fn status_history(&self) -> &[EpisodeOfCareStatushistory];
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the diagnosis field.
    fn diagnosis(&self) -> &[EpisodeOfCareDiagnosis];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the managingOrganization field.
    fn managing_organization(&self) -> Option<Reference>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the referralRequest field.
    fn referral_request(&self) -> &[Reference];
    /// Returns a reference to the careManager field.
    fn care_manager(&self) -> Option<Reference>;
    /// Returns a reference to the team field.
    fn team(&self) -> &[Reference];
    /// Returns a reference to the account field.
    fn account(&self) -> &[Reference];
}
/// EpisodeOfCare Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EpisodeOfCare
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EpisodeOfCareMutators: DomainResourceMutators {
    /// Create a new EpisodeOfCare with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::episode_of_care::EpisodeOfCare;
    /// use hl7_fhir_r4_core::traits::episode_of_care::EpisodeOfCareMutators;
    ///
    /// let resource = EpisodeOfCare::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EpisodeOfCareStatus) -> Self;
    /// Sets the statusHistory field and returns self for chaining.
    fn set_status_history(self, value: Vec<EpisodeOfCareStatushistory>) -> Self;
    /// Adds an item to the statusHistory field and returns self for chaining.
    fn add_status_history(self, item: EpisodeOfCareStatushistory) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the diagnosis field and returns self for chaining.
    fn set_diagnosis(self, value: Vec<EpisodeOfCareDiagnosis>) -> Self;
    /// Adds an item to the diagnosis field and returns self for chaining.
    fn add_diagnosis(self, item: EpisodeOfCareDiagnosis) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the managingOrganization field and returns self for chaining.
    fn set_managing_organization(self, value: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the referralRequest field and returns self for chaining.
    fn set_referral_request(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the referralRequest field and returns self for chaining.
    fn add_referral_request(self, item: Reference) -> Self;
    /// Sets the careManager field and returns self for chaining.
    fn set_care_manager(self, value: Reference) -> Self;
    /// Sets the team field and returns self for chaining.
    fn set_team(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the team field and returns self for chaining.
    fn add_team(self, item: Reference) -> Self;
    /// Sets the account field and returns self for chaining.
    fn set_account(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the account field and returns self for chaining.
    fn add_account(self, item: Reference) -> Self;
}
/// EpisodeOfCare Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EpisodeOfCare
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait EpisodeOfCareExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_history field is not empty.
    fn has_status_history(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the diagnosis field is not empty.
    fn has_diagnosis(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the managing_organization field is present (Some).
    fn has_managing_organization(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the referral_request field is not empty.
    fn has_referral_request(&self) -> bool;
    /// Returns true if the care_manager field is present (Some).
    fn has_care_manager(&self) -> bool;
    /// Returns true if the team field is not empty.
    fn has_team(&self) -> bool;
    /// Returns true if the account field is not empty.
    fn has_account(&self) -> bool;
}
