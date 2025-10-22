use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::payment_reconciliation::PaymentReconciliationDetail;
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
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentReconciliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentReconciliationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the paymentIssuer field.
    fn payment_issuer(&self) -> Option<Reference>;
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the requestor field.
    fn requestor(&self) -> Option<Reference>;
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<RemittanceOutcome>;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<StringType>;
    /// Returns a reference to the paymentDate field.
    fn payment_date(&self) -> StringType;
    /// Returns a reference to the paymentAmount field.
    fn payment_amount(&self) -> Money;
    /// Returns a reference to the paymentIdentifier field.
    fn payment_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the detail field.
    fn detail(&self) -> &[PaymentReconciliationDetail];
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
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::payment_reconciliation::PaymentReconciliation;
    /// use hl7_fhir_r4_core::traits::payment_reconciliation::PaymentReconciliationMutators;
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
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the paymentIssuer field and returns self for chaining.
    fn set_payment_issuer(self, value: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the requestor field and returns self for chaining.
    fn set_requestor(self, value: Reference) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: RemittanceOutcome) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: String) -> Self;
    /// Sets the paymentDate field and returns self for chaining.
    fn set_payment_date(self, value: String) -> Self;
    /// Sets the paymentAmount field and returns self for chaining.
    fn set_payment_amount(self, value: Money) -> Self;
    /// Sets the paymentIdentifier field and returns self for chaining.
    fn set_payment_identifier(self, value: Identifier) -> Self;
    /// Sets the detail field and returns self for chaining.
    fn set_detail(self, value: Vec<PaymentReconciliationDetail>) -> Self;
    /// Adds an item to the detail field and returns self for chaining.
    fn add_detail(self, item: PaymentReconciliationDetail) -> Self;
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
/// - Version: 4.0.1
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
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
    /// Returns true if the payment_date field is present (Some).
    fn has_payment_date(&self) -> bool;
    /// Returns true if the payment_amount field is present (Some).
    fn has_payment_amount(&self) -> bool;
    /// Returns true if the payment_identifier field is present (Some).
    fn has_payment_identifier(&self) -> bool;
    /// Returns true if the detail field is not empty.
    fn has_detail(&self) -> bool;
    /// Returns true if the form_code field is present (Some).
    fn has_form_code(&self) -> bool;
    /// Returns true if the process_note field is not empty.
    fn has_process_note(&self) -> bool;
}
