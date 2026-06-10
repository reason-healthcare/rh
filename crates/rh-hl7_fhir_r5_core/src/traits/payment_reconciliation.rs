use crate::bindings::fm_status::FmStatus;
use crate::bindings::payment_outcome::PaymentOutcome;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::payment_reconciliation::PaymentReconciliationAllocation;
use crate::resources::payment_reconciliation::PaymentReconciliationProcessnote;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// PaymentReconciliation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the details including amount of a payment and allocates the payment items being paid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentReconciliationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> CodeableConcept;
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the kind field.
    fn kind(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the enterer field.
    fn enterer(&self) -> Option<Reference>;
    /// Returns a reference to the issuerType field.
    fn issuer_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the paymentIssuer field.
    fn payment_issuer(&self) -> Option<Reference>;
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the requestor field.
    fn requestor(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<PaymentOutcome>;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the date field.
    fn date(&self) -> DateType;
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the method field.
    fn method(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the cardBrand field.
    fn card_brand(&self) -> Option<StringType>;
    /// Returns a reference to the accountNumber field.
    fn account_number(&self) -> Option<StringType>;
    /// Returns a reference to the expirationDate field.
    fn expiration_date(&self) -> Option<DateType>;
    /// Returns a reference to the processor field.
    fn processor(&self) -> Option<StringType>;
    /// Returns a reference to the referenceNumber field.
    fn reference_number(&self) -> Option<StringType>;
    /// Returns a reference to the authorization field.
    fn authorization(&self) -> Option<StringType>;
    /// Returns a reference to the tenderedAmount field.
    fn tendered_amount(&self) -> Option<Money>;
    /// Returns a reference to the returnedAmount field.
    fn returned_amount(&self) -> Option<Money>;
    /// Returns a reference to the amount field.
    fn amount(&self) -> Money;
    /// Returns a reference to the paymentIdentifier field.
    fn payment_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the allocation field.
    fn allocation(&self) -> &[PaymentReconciliationAllocation];
    /// Returns a reference to the formCode field.
    fn form_code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the processNote field.
    fn process_note(&self) -> &[PaymentReconciliationProcessnote];
}
/// PaymentReconciliation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the details including amount of a payment and allocates the payment items being paid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentReconciliationMutators: DomainResourceMutators {
    /// Create a new PaymentReconciliation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::payment_reconciliation::PaymentReconciliation;
    /// use rh_hl7_fhir_r5_core::traits::payment_reconciliation::PaymentReconciliationMutators;
    ///
    /// let resource = PaymentReconciliation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: CodeableConcept) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the enterer field and returns self for chaining.
    fn set_enterer(self, value: Reference) -> Self;
    /// Sets the issuerType field and returns self for chaining.
    fn set_issuer_type(self, value: CodeableConcept) -> Self;
    /// Sets the paymentIssuer field and returns self for chaining.
    fn set_payment_issuer(self, value: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the requestor field and returns self for chaining.
    fn set_requestor(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: PaymentOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the method field and returns self for chaining.
    fn set_method(self, value: CodeableConcept) -> Self;
    /// Sets the cardBrand field and returns self for chaining.
    fn set_card_brand(self, value: String) -> Self;
    /// Sets the accountNumber field and returns self for chaining.
    fn set_account_number(self, value: String) -> Self;
    /// Sets the expirationDate field and returns self for chaining.
    fn set_expiration_date(self, value: String) -> Self;
    /// Sets the processor field and returns self for chaining.
    fn set_processor(self, value: String) -> Self;
    /// Sets the referenceNumber field and returns self for chaining.
    fn set_reference_number(self, value: String) -> Self;
    /// Sets the authorization field and returns self for chaining.
    fn set_authorization(self, value: String) -> Self;
    /// Sets the tenderedAmount field and returns self for chaining.
    fn set_tendered_amount(self, value: Money) -> Self;
    /// Sets the returnedAmount field and returns self for chaining.
    fn set_returned_amount(self, value: Money) -> Self;
    /// Sets the amount field and returns self for chaining.
    fn set_amount(self, value: Money) -> Self;
    /// Sets the paymentIdentifier field and returns self for chaining.
    fn set_payment_identifier(self, value: Identifier) -> Self;
    /// Sets the allocation field and returns self for chaining.
    fn set_allocation(self, value: Vec<PaymentReconciliationAllocation>) -> Self;
    /// Adds an item to the allocation field and returns self for chaining.
    fn add_allocation(self, item: PaymentReconciliationAllocation) -> Self;
    /// Sets the formCode field and returns self for chaining.
    fn set_form_code(self, value: CodeableConcept) -> Self;
    /// Sets the processNote field and returns self for chaining.
    fn set_process_note(self, value: Vec<PaymentReconciliationProcessnote>) -> Self;
    /// Adds an item to the processNote field and returns self for chaining.
    fn add_process_note(self, item: PaymentReconciliationProcessnote) -> Self;
}
/// PaymentReconciliation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides the details including amount of a payment and allocates the payment items being paid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentReconciliationExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the enterer field is present (Some).
    fn has_enterer(&self) -> bool;
    /// Returns true if the issuer_type field is present (Some).
    fn has_issuer_type(&self) -> bool;
    /// Returns true if the payment_issuer field is present (Some).
    fn has_payment_issuer(&self) -> bool;
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the requestor field is present (Some).
    fn has_requestor(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the method field is present (Some).
    fn has_method(&self) -> bool;
    /// Returns true if the card_brand field is present (Some).
    fn has_card_brand(&self) -> bool;
    /// Returns true if the account_number field is present (Some).
    fn has_account_number(&self) -> bool;
    /// Returns true if the expiration_date field is present (Some).
    fn has_expiration_date(&self) -> bool;
    /// Returns true if the processor field is present (Some).
    fn has_processor(&self) -> bool;
    /// Returns true if the reference_number field is present (Some).
    fn has_reference_number(&self) -> bool;
    /// Returns true if the authorization field is present (Some).
    fn has_authorization(&self) -> bool;
    /// Returns true if the tendered_amount field is present (Some).
    fn has_tendered_amount(&self) -> bool;
    /// Returns true if the returned_amount field is present (Some).
    fn has_returned_amount(&self) -> bool;
    /// Returns true if the amount field is present (Some).
    fn has_amount(&self) -> bool;
    /// Returns true if the payment_identifier field is present (Some).
    fn has_payment_identifier(&self) -> bool;
    /// Returns true if the allocation field is not empty.
    fn has_allocation(&self) -> bool;
    /// Returns true if the form_code field is present (Some).
    fn has_form_code(&self) -> bool;
    /// Returns true if the process_note field is not empty.
    fn has_process_note(&self) -> bool;
}
