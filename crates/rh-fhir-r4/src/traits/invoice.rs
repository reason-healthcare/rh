use crate::bindings::invoice_status::InvoiceStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::invoice::InvoiceLineitem;
use crate::resources::invoice::InvoiceParticipant;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Invoice Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Invoice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Invoice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InvoiceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> InvoiceStatus;
    /// Returns a reference to the cancelledReason field.
    fn cancelled_reason(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the recipient field.
    fn recipient(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[InvoiceParticipant];
    /// Returns a reference to the issuer field.
    fn issuer(&self) -> Option<Reference>;
    /// Returns a reference to the account field.
    fn account(&self) -> Option<Reference>;
    /// Returns a reference to the lineItem field.
    fn line_item(&self) -> &[InvoiceLineitem];
    /// Returns a reference to the totalNet field.
    fn total_net(&self) -> Option<Money>;
    /// Returns a reference to the totalGross field.
    fn total_gross(&self) -> Option<Money>;
    /// Returns a reference to the paymentTerms field.
    fn payment_terms(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// Invoice Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Invoice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Invoice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InvoiceMutators: DomainResourceMutators {
    /// Create a new Invoice with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::invoice::Invoice;
    /// use hl7_fhir_r4_core::traits::invoice::InvoiceMutators;
    ///
    /// let resource = Invoice::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: InvoiceStatus) -> Self;
    /// Sets the cancelledReason field and returns self for chaining.
    fn set_cancelled_reason(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the recipient field and returns self for chaining.
    fn set_recipient(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<InvoiceParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: InvoiceParticipant) -> Self;
    /// Sets the issuer field and returns self for chaining.
    fn set_issuer(self, value: Reference) -> Self;
    /// Sets the account field and returns self for chaining.
    fn set_account(self, value: Reference) -> Self;
    /// Sets the lineItem field and returns self for chaining.
    fn set_line_item(self, value: Vec<InvoiceLineitem>) -> Self;
    /// Adds an item to the lineItem field and returns self for chaining.
    fn add_line_item(self, item: InvoiceLineitem) -> Self;
    /// Sets the totalPriceComponent field and returns self for chaining.
    fn set_total_price_component(self, value: Vec<String>) -> Self;
    /// Adds an item to the totalPriceComponent field and returns self for chaining.
    fn add_total_price_component(self, item: String) -> Self;
    /// Sets the totalNet field and returns self for chaining.
    fn set_total_net(self, value: Money) -> Self;
    /// Sets the totalGross field and returns self for chaining.
    fn set_total_gross(self, value: Money) -> Self;
    /// Sets the paymentTerms field and returns self for chaining.
    fn set_payment_terms(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// Invoice Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Invoice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Invoice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InvoiceExistence: DomainResourceExistence {
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
    /// Returns true if the cancelled_reason field is present (Some).
    fn has_cancelled_reason(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the recipient field is present (Some).
    fn has_recipient(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the issuer field is present (Some).
    fn has_issuer(&self) -> bool;
    /// Returns true if the account field is present (Some).
    fn has_account(&self) -> bool;
    /// Returns true if the line_item field is not empty.
    fn has_line_item(&self) -> bool;
    /// Returns true if the total_price_component field is not empty.
    fn has_total_price_component(&self) -> bool;
    /// Returns true if the total_net field is present (Some).
    fn has_total_net(&self) -> bool;
    /// Returns true if the total_gross field is present (Some).
    fn has_total_gross(&self) -> bool;
    /// Returns true if the payment_terms field is present (Some).
    fn has_payment_terms(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
