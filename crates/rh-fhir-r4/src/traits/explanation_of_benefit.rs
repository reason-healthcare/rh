use crate::bindings::claim_use::ClaimUse;
use crate::bindings::explanationofbenefit_status::ExplanationofbenefitStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitAccident;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitAdditem;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitBenefitbalance;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitCareteam;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitDiagnosis;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitInsurance;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitItem;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitPayee;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitPayment;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitProcedure;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitProcessnote;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitRelated;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitSupportinginfo;
use crate::resources::explanation_of_benefit::ExplanationOfBenefitTotal;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ExplanationOfBenefit Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ExplanationOfBenefit
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ExplanationOfBenefitAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ExplanationofbenefitStatus;
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
    fn insurer(&self) -> Reference;
    /// Returns a reference to the provider field.
    fn provider(&self) -> Reference;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the fundsReserveRequested field.
    fn funds_reserve_requested(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the fundsReserve field.
    fn funds_reserve(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the related field.
    fn related(&self) -> &[ExplanationOfBenefitRelated];
    /// Returns a reference to the prescription field.
    fn prescription(&self) -> Option<Reference>;
    /// Returns a reference to the originalPrescription field.
    fn original_prescription(&self) -> Option<Reference>;
    /// Returns a reference to the payee field.
    fn payee(&self) -> Option<ExplanationOfBenefitPayee>;
    /// Returns a reference to the referral field.
    fn referral(&self) -> Option<Reference>;
    /// Returns a reference to the facility field.
    fn facility(&self) -> Option<Reference>;
    /// Returns a reference to the claim field.
    fn claim(&self) -> Option<Reference>;
    /// Returns a reference to the claimResponse field.
    fn claim_response(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> RemittanceOutcome;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the preAuthRef field.
    fn pre_auth_ref(&self) -> &[StringType];
    /// Returns a reference to the preAuthRefPeriod field.
    fn pre_auth_ref_period(&self) -> &[Period];
    /// Returns a reference to the careTeam field.
    fn care_team(&self) -> &[ExplanationOfBenefitCareteam];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[ExplanationOfBenefitSupportinginfo];
    /// Returns a reference to the diagnosis field.
    fn diagnosis(&self) -> &[ExplanationOfBenefitDiagnosis];
    /// Returns a reference to the procedure field.
    fn procedure(&self) -> &[ExplanationOfBenefitProcedure];
    /// Returns a reference to the precedence field.
    fn precedence(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[ExplanationOfBenefitInsurance];
    /// Returns a reference to the accident field.
    fn accident(&self) -> Option<ExplanationOfBenefitAccident>;
    /// Returns a reference to the item field.
    fn item(&self) -> &[ExplanationOfBenefitItem];
    /// Returns a reference to the addItem field.
    fn add_item(&self) -> &[ExplanationOfBenefitAdditem];
    /// Returns a reference to the total field.
    fn total(&self) -> &[ExplanationOfBenefitTotal];
    /// Returns a reference to the payment field.
    fn payment(&self) -> Option<ExplanationOfBenefitPayment>;
    /// Returns a reference to the formCode field.
    fn form_code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the form field.
    fn form(&self) -> Option<Attachment>;
    /// Returns a reference to the processNote field.
    fn process_note(&self) -> &[ExplanationOfBenefitProcessnote];
    /// Returns a reference to the benefitPeriod field.
    fn benefit_period(&self) -> Option<Period>;
    /// Returns a reference to the benefitBalance field.
    fn benefit_balance(&self) -> &[ExplanationOfBenefitBenefitbalance];
}
/// ExplanationOfBenefit Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ExplanationOfBenefit
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ExplanationOfBenefitMutators: DomainResourceMutators {
    /// Create a new ExplanationOfBenefit with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::explanation_of_benefit::ExplanationOfBenefit;
    /// use hl7_fhir_r4_core::traits::explanation_of_benefit::ExplanationOfBenefitMutators;
    ///
    /// let resource = ExplanationOfBenefit::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ExplanationofbenefitStatus) -> Self;
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
    /// Sets the fundsReserveRequested field and returns self for chaining.
    fn set_funds_reserve_requested(self, value: CodeableConcept) -> Self;
    /// Sets the fundsReserve field and returns self for chaining.
    fn set_funds_reserve(self, value: CodeableConcept) -> Self;
    /// Sets the related field and returns self for chaining.
    fn set_related(self, value: Vec<ExplanationOfBenefitRelated>) -> Self;
    /// Adds an item to the related field and returns self for chaining.
    fn add_related(self, item: ExplanationOfBenefitRelated) -> Self;
    /// Sets the prescription field and returns self for chaining.
    fn set_prescription(self, value: Reference) -> Self;
    /// Sets the originalPrescription field and returns self for chaining.
    fn set_original_prescription(self, value: Reference) -> Self;
    /// Sets the payee field and returns self for chaining.
    fn set_payee(self, value: ExplanationOfBenefitPayee) -> Self;
    /// Sets the referral field and returns self for chaining.
    fn set_referral(self, value: Reference) -> Self;
    /// Sets the facility field and returns self for chaining.
    fn set_facility(self, value: Reference) -> Self;
    /// Sets the claim field and returns self for chaining.
    fn set_claim(self, value: Reference) -> Self;
    /// Sets the claimResponse field and returns self for chaining.
    fn set_claim_response(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: RemittanceOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the preAuthRef field and returns self for chaining.
    fn set_pre_auth_ref(self, value: Vec<String>) -> Self;
    /// Adds an item to the preAuthRef field and returns self for chaining.
    fn add_pre_auth_ref(self, item: String) -> Self;
    /// Sets the preAuthRefPeriod field and returns self for chaining.
    fn set_pre_auth_ref_period(self, value: Vec<Period>) -> Self;
    /// Adds an item to the preAuthRefPeriod field and returns self for chaining.
    fn add_pre_auth_ref_period(self, item: Period) -> Self;
    /// Sets the careTeam field and returns self for chaining.
    fn set_care_team(self, value: Vec<ExplanationOfBenefitCareteam>) -> Self;
    /// Adds an item to the careTeam field and returns self for chaining.
    fn add_care_team(self, item: ExplanationOfBenefitCareteam) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<ExplanationOfBenefitSupportinginfo>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: ExplanationOfBenefitSupportinginfo) -> Self;
    /// Sets the diagnosis field and returns self for chaining.
    fn set_diagnosis(self, value: Vec<ExplanationOfBenefitDiagnosis>) -> Self;
    /// Adds an item to the diagnosis field and returns self for chaining.
    fn add_diagnosis(self, item: ExplanationOfBenefitDiagnosis) -> Self;
    /// Sets the procedure field and returns self for chaining.
    fn set_procedure(self, value: Vec<ExplanationOfBenefitProcedure>) -> Self;
    /// Adds an item to the procedure field and returns self for chaining.
    fn add_procedure(self, item: ExplanationOfBenefitProcedure) -> Self;
    /// Sets the precedence field and returns self for chaining.
    fn set_precedence(self, value: i32) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<ExplanationOfBenefitInsurance>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: ExplanationOfBenefitInsurance) -> Self;
    /// Sets the accident field and returns self for chaining.
    fn set_accident(self, value: ExplanationOfBenefitAccident) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<ExplanationOfBenefitItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: ExplanationOfBenefitItem) -> Self;
    /// Sets the addItem field and returns self for chaining.
    fn set_add_item(self, value: Vec<ExplanationOfBenefitAdditem>) -> Self;
    /// Adds an item to the addItem field and returns self for chaining.
    fn add_add_item(self, item: ExplanationOfBenefitAdditem) -> Self;
    /// Sets the adjudication field and returns self for chaining.
    fn set_adjudication(self, value: Vec<String>) -> Self;
    /// Adds an item to the adjudication field and returns self for chaining.
    fn add_adjudication(self, item: String) -> Self;
    /// Sets the total field and returns self for chaining.
    fn set_total(self, value: Vec<ExplanationOfBenefitTotal>) -> Self;
    /// Adds an item to the total field and returns self for chaining.
    fn add_total(self, item: ExplanationOfBenefitTotal) -> Self;
    /// Sets the payment field and returns self for chaining.
    fn set_payment(self, value: ExplanationOfBenefitPayment) -> Self;
    /// Sets the formCode field and returns self for chaining.
    fn set_form_code(self, value: CodeableConcept) -> Self;
    /// Sets the form field and returns self for chaining.
    fn set_form(self, value: Attachment) -> Self;
    /// Sets the processNote field and returns self for chaining.
    fn set_process_note(self, value: Vec<ExplanationOfBenefitProcessnote>) -> Self;
    /// Adds an item to the processNote field and returns self for chaining.
    fn add_process_note(self, item: ExplanationOfBenefitProcessnote) -> Self;
    /// Sets the benefitPeriod field and returns self for chaining.
    fn set_benefit_period(self, value: Period) -> Self;
    /// Sets the benefitBalance field and returns self for chaining.
    fn set_benefit_balance(self, value: Vec<ExplanationOfBenefitBenefitbalance>) -> Self;
    /// Adds an item to the benefitBalance field and returns self for chaining.
    fn add_benefit_balance(self, item: ExplanationOfBenefitBenefitbalance) -> Self;
}
/// ExplanationOfBenefit Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ExplanationOfBenefit
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ExplanationOfBenefitExistence: DomainResourceExistence {
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
    /// Returns true if the funds_reserve_requested field is present (Some).
    fn has_funds_reserve_requested(&self) -> bool;
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
    /// Returns true if the claim field is present (Some).
    fn has_claim(&self) -> bool;
    /// Returns true if the claim_response field is present (Some).
    fn has_claim_response(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
    /// Returns true if the pre_auth_ref field is not empty.
    fn has_pre_auth_ref(&self) -> bool;
    /// Returns true if the pre_auth_ref_period field is not empty.
    fn has_pre_auth_ref_period(&self) -> bool;
    /// Returns true if the care_team field is not empty.
    fn has_care_team(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
    /// Returns true if the diagnosis field is not empty.
    fn has_diagnosis(&self) -> bool;
    /// Returns true if the procedure field is not empty.
    fn has_procedure(&self) -> bool;
    /// Returns true if the precedence field is present (Some).
    fn has_precedence(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the accident field is present (Some).
    fn has_accident(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
    /// Returns true if the add_item field is not empty.
    fn has_add_item(&self) -> bool;
    /// Returns true if the adjudication field is not empty.
    fn has_adjudication(&self) -> bool;
    /// Returns true if the total field is not empty.
    fn has_total(&self) -> bool;
    /// Returns true if the payment field is present (Some).
    fn has_payment(&self) -> bool;
    /// Returns true if the form_code field is present (Some).
    fn has_form_code(&self) -> bool;
    /// Returns true if the form field is present (Some).
    fn has_form(&self) -> bool;
    /// Returns true if the process_note field is not empty.
    fn has_process_note(&self) -> bool;
    /// Returns true if the benefit_period field is present (Some).
    fn has_benefit_period(&self) -> bool;
    /// Returns true if the benefit_balance field is not empty.
    fn has_benefit_balance(&self) -> bool;
}
