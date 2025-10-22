use crate::bindings::account_status::AccountStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::account::AccountCoverage;
use crate::resources::account::AccountGuarantor;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Account Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Account
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Account
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AccountAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> AccountStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the servicePeriod field.
    fn service_period(&self) -> Option<Period>;
    /// Returns a reference to the coverage field.
    fn coverage(&self) -> &[AccountCoverage];
    /// Returns a reference to the owner field.
    fn owner(&self) -> Option<Reference>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the guarantor field.
    fn guarantor(&self) -> &[AccountGuarantor];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> Option<Reference>;
}
/// Account Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Account
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Account
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AccountMutators: DomainResourceMutators {
    /// Create a new Account with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::account::Account;
    /// use hl7_fhir_r4_core::traits::account::AccountMutators;
    ///
    /// let resource = Account::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: AccountStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the servicePeriod field and returns self for chaining.
    fn set_service_period(self, value: Period) -> Self;
    /// Sets the coverage field and returns self for chaining.
    fn set_coverage(self, value: Vec<AccountCoverage>) -> Self;
    /// Adds an item to the coverage field and returns self for chaining.
    fn add_coverage(self, item: AccountCoverage) -> Self;
    /// Sets the owner field and returns self for chaining.
    fn set_owner(self, value: Reference) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the guarantor field and returns self for chaining.
    fn set_guarantor(self, value: Vec<AccountGuarantor>) -> Self;
    /// Adds an item to the guarantor field and returns self for chaining.
    fn add_guarantor(self, item: AccountGuarantor) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Reference) -> Self;
}
/// Account Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Account
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Account
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AccountExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the service_period field is present (Some).
    fn has_service_period(&self) -> bool;
    /// Returns true if the coverage field is not empty.
    fn has_coverage(&self) -> bool;
    /// Returns true if the owner field is present (Some).
    fn has_owner(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the guarantor field is not empty.
    fn has_guarantor(&self) -> bool;
    /// Returns true if the part_of field is present (Some).
    fn has_part_of(&self) -> bool;
}
