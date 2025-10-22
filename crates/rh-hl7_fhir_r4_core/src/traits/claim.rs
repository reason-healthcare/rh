use crate::bindings::claim_use::ClaimUse;
use crate::bindings::fm_status::FmStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::claim::ClaimAccident;
use crate::resources::claim::ClaimCareteam;
use crate::resources::claim::ClaimDiagnosis;
use crate::resources::claim::ClaimInsurance;
use crate::resources::claim::ClaimItem;
use crate::resources::claim::ClaimPayee;
use crate::resources::claim::ClaimProcedure;
use crate::resources::claim::ClaimRelated;
use crate::resources::claim::ClaimSupportinginfo;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Claim Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Claim
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Claim
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> CodeableConcept;
    /// Returns a reference to the subType field.
    fn sub_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the use field.
    fn use_(&self) -> ClaimUse;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the billablePeriod field.
    fn billable_period(&self) -> Option<Period>;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the enterer field.
    fn enterer(&self) -> Option<Reference>;
    /// Returns a reference to the insurer field.
    fn insurer(&self) -> Option<Reference>;
    /// Returns a reference to the provider field.
    fn provider(&self) -> Reference;
    /// Returns a reference to the priority field.
    fn priority(&self) -> CodeableConcept;
    /// Returns a reference to the fundsReserve field.
    fn funds_reserve(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the related field.
    fn related(&self) -> &[ClaimRelated];
    /// Returns a reference to the prescription field.
    fn prescription(&self) -> Option<Reference>;
    /// Returns a reference to the originalPrescription field.
    fn original_prescription(&self) -> Option<Reference>;
    /// Returns a reference to the payee field.
    fn payee(&self) -> Option<ClaimPayee>;
    /// Returns a reference to the referral field.
    fn referral(&self) -> Option<Reference>;
    /// Returns a reference to the facility field.
    fn facility(&self) -> Option<Reference>;
    /// Returns a reference to the careTeam field.
    fn care_team(&self) -> &[ClaimCareteam];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[ClaimSupportinginfo];
    /// Returns a reference to the diagnosis field.
    fn diagnosis(&self) -> &[ClaimDiagnosis];
    /// Returns a reference to the procedure field.
    fn procedure(&self) -> &[ClaimProcedure];
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[ClaimInsurance];
    /// Returns a reference to the accident field.
    fn accident(&self) -> Option<ClaimAccident>;
    /// Returns a reference to the item field.
    fn item(&self) -> &[ClaimItem];
    /// Returns a reference to the total field.
    fn total(&self) -> Option<Money>;
}
/// Claim Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Claim
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Claim
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimMutators: DomainResourceMutators {
    /// Create a new Claim with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::claim::Claim;
    /// use hl7_fhir_r4_core::traits::claim::ClaimMutators;
    ///
    /// let resource = Claim::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subType field and returns self for chaining.
    fn set_sub_type(self, value: CodeableConcept) -> Self;
    /// Sets the use field and returns self for chaining.
    fn set_use_(self, value: ClaimUse) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the billablePeriod field and returns self for chaining.
    fn set_billable_period(self, value: Period) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the enterer field and returns self for chaining.
    fn set_enterer(self, value: Reference) -> Self;
    /// Sets the insurer field and returns self for chaining.
    fn set_insurer(self, value: Reference) -> Self;
    /// Sets the provider field and returns self for chaining.
    fn set_provider(self, value: Reference) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: CodeableConcept) -> Self;
    /// Sets the fundsReserve field and returns self for chaining.
    fn set_funds_reserve(self, value: CodeableConcept) -> Self;
    /// Sets the related field and returns self for chaining.
    fn set_related(self, value: Vec<ClaimRelated>) -> Self;
    /// Adds an item to the related field and returns self for chaining.
    fn add_related(self, item: ClaimRelated) -> Self;
    /// Sets the prescription field and returns self for chaining.
    fn set_prescription(self, value: Reference) -> Self;
    /// Sets the originalPrescription field and returns self for chaining.
    fn set_original_prescription(self, value: Reference) -> Self;
    /// Sets the payee field and returns self for chaining.
    fn set_payee(self, value: ClaimPayee) -> Self;
    /// Sets the referral field and returns self for chaining.
    fn set_referral(self, value: Reference) -> Self;
    /// Sets the facility field and returns self for chaining.
    fn set_facility(self, value: Reference) -> Self;
    /// Sets the careTeam field and returns self for chaining.
    fn set_care_team(self, value: Vec<ClaimCareteam>) -> Self;
    /// Adds an item to the careTeam field and returns self for chaining.
    fn add_care_team(self, item: ClaimCareteam) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<ClaimSupportinginfo>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: ClaimSupportinginfo) -> Self;
    /// Sets the diagnosis field and returns self for chaining.
    fn set_diagnosis(self, value: Vec<ClaimDiagnosis>) -> Self;
    /// Adds an item to the diagnosis field and returns self for chaining.
    fn add_diagnosis(self, item: ClaimDiagnosis) -> Self;
    /// Sets the procedure field and returns self for chaining.
    fn set_procedure(self, value: Vec<ClaimProcedure>) -> Self;
    /// Adds an item to the procedure field and returns self for chaining.
    fn add_procedure(self, item: ClaimProcedure) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<ClaimInsurance>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: ClaimInsurance) -> Self;
    /// Sets the accident field and returns self for chaining.
    fn set_accident(self, value: ClaimAccident) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<ClaimItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: ClaimItem) -> Self;
    /// Sets the total field and returns self for chaining.
    fn set_total(self, value: Money) -> Self;
}
/// Claim Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Claim
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Claim
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ClaimExistence: DomainResourceExistence {
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
    /// Returns true if the sub_type field is present (Some).
    fn has_sub_type(&self) -> bool;
    /// Returns true if the use_ field is present (Some).
    fn has_use_(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the billable_period field is present (Some).
    fn has_billable_period(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the enterer field is present (Some).
    fn has_enterer(&self) -> bool;
    /// Returns true if the insurer field is present (Some).
    fn has_insurer(&self) -> bool;
    /// Returns true if the provider field is present (Some).
    fn has_provider(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the funds_reserve field is present (Some).
    fn has_funds_reserve(&self) -> bool;
    /// Returns true if the related field is not empty.
    fn has_related(&self) -> bool;
    /// Returns true if the prescription field is present (Some).
    fn has_prescription(&self) -> bool;
    /// Returns true if the original_prescription field is present (Some).
    fn has_original_prescription(&self) -> bool;
    /// Returns true if the payee field is present (Some).
    fn has_payee(&self) -> bool;
    /// Returns true if the referral field is present (Some).
    fn has_referral(&self) -> bool;
    /// Returns true if the facility field is present (Some).
    fn has_facility(&self) -> bool;
    /// Returns true if the care_team field is not empty.
    fn has_care_team(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the diagnosis field is not empty.
    fn has_diagnosis(&self) -> bool;
    /// Returns true if the procedure field is not empty.
    fn has_procedure(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the accident field is present (Some).
    fn has_accident(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
    /// Returns true if the total field is present (Some).
    fn has_total(&self) -> bool;
}
