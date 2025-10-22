use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::medicinal_product_authorization::MedicinalProductAuthorizationJurisdictionalauthorization;
use crate::resources::medicinal_product_authorization::MedicinalProductAuthorizationProcedure;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductAuthorization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The regulatory authorization of a medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductAuthorization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductAuthorizationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the country field.
    fn country(&self) -> &[CodeableConcept];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the restoreDate field.
    fn restore_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the validityPeriod field.
    fn validity_period(&self) -> Option<Period>;
    /// Returns a reference to the dataExclusivityPeriod field.
    fn data_exclusivity_period(&self) -> Option<Period>;
    /// Returns a reference to the dateOfFirstAuthorization field.
    fn date_of_first_authorization(&self) -> Option<DateTimeType>;
    /// Returns a reference to the internationalBirthDate field.
    fn international_birth_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the legalBasis field.
    fn legal_basis(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the jurisdictionalAuthorization field.
    fn jurisdictional_authorization(
        &self,
    ) -> &[MedicinalProductAuthorizationJurisdictionalauthorization];
    /// Returns a reference to the holder field.
    fn holder(&self) -> Option<Reference>;
    /// Returns a reference to the regulator field.
    fn regulator(&self) -> Option<Reference>;
    /// Returns a reference to the procedure field.
    fn procedure(&self) -> Option<MedicinalProductAuthorizationProcedure>;
}
/// MedicinalProductAuthorization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The regulatory authorization of a medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductAuthorization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductAuthorizationMutators: DomainResourceMutators {
    /// Create a new MedicinalProductAuthorization with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_authorization::MedicinalProductAuthorization;
    /// use hl7_fhir_r4_core::traits::medicinal_product_authorization::MedicinalProductAuthorizationMutators;
    ///
    /// let resource = MedicinalProductAuthorization::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the country field and returns self for chaining.
    fn set_country(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the country field and returns self for chaining.
    fn add_country(self, item: CodeableConcept) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the restoreDate field and returns self for chaining.
    fn set_restore_date(self, value: String) -> Self;
    /// Sets the validityPeriod field and returns self for chaining.
    fn set_validity_period(self, value: Period) -> Self;
    /// Sets the dataExclusivityPeriod field and returns self for chaining.
    fn set_data_exclusivity_period(self, value: Period) -> Self;
    /// Sets the dateOfFirstAuthorization field and returns self for chaining.
    fn set_date_of_first_authorization(self, value: String) -> Self;
    /// Sets the internationalBirthDate field and returns self for chaining.
    fn set_international_birth_date(self, value: String) -> Self;
    /// Sets the legalBasis field and returns self for chaining.
    fn set_legal_basis(self, value: CodeableConcept) -> Self;
    /// Sets the jurisdictionalAuthorization field and returns self for chaining.
    fn set_jurisdictional_authorization(
        self,
        value: Vec<MedicinalProductAuthorizationJurisdictionalauthorization>,
    ) -> Self;
    /// Adds an item to the jurisdictionalAuthorization field and returns self for chaining.
    fn add_jurisdictional_authorization(
        self,
        item: MedicinalProductAuthorizationJurisdictionalauthorization,
    ) -> Self;
    /// Sets the holder field and returns self for chaining.
    fn set_holder(self, value: Reference) -> Self;
    /// Sets the regulator field and returns self for chaining.
    fn set_regulator(self, value: Reference) -> Self;
    /// Sets the procedure field and returns self for chaining.
    fn set_procedure(self, value: MedicinalProductAuthorizationProcedure) -> Self;
}
/// MedicinalProductAuthorization Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The regulatory authorization of a medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductAuthorization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductAuthorizationExistence: DomainResourceExistence {
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
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the country field is not empty.
    fn has_country(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the restore_date field is present (Some).
    fn has_restore_date(&self) -> bool;
    /// Returns true if the validity_period field is present (Some).
    fn has_validity_period(&self) -> bool;
    /// Returns true if the data_exclusivity_period field is present (Some).
    fn has_data_exclusivity_period(&self) -> bool;
    /// Returns true if the date_of_first_authorization field is present (Some).
    fn has_date_of_first_authorization(&self) -> bool;
    /// Returns true if the international_birth_date field is present (Some).
    fn has_international_birth_date(&self) -> bool;
    /// Returns true if the legal_basis field is present (Some).
    fn has_legal_basis(&self) -> bool;
    /// Returns true if the jurisdictional_authorization field is not empty.
    fn has_jurisdictional_authorization(&self) -> bool;
    /// Returns true if the holder field is present (Some).
    fn has_holder(&self) -> bool;
    /// Returns true if the regulator field is present (Some).
    fn has_regulator(&self) -> bool;
    /// Returns true if the procedure field is present (Some).
    fn has_procedure(&self) -> bool;
}
