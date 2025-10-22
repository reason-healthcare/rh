use crate::bindings::fm_status::FmStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// PaymentNotice Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentNotice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentNoticeAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the request field.
    fn request(&self) -> Option<Reference>;
    /// Returns a reference to the response field.
    fn response(&self) -> Option<Reference>;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the provider field.
    fn provider(&self) -> Option<Reference>;
    /// Returns a reference to the payment field.
    fn payment(&self) -> Reference;
    /// Returns a reference to the paymentDate field.
    fn payment_date(&self) -> Option<StringType>;
    /// Returns a reference to the payee field.
    fn payee(&self) -> Option<Reference>;
    /// Returns a reference to the recipient field.
    fn recipient(&self) -> Reference;
    /// Returns a reference to the amount field.
    fn amount(&self) -> Money;
    /// Returns a reference to the paymentStatus field.
    fn payment_status(&self) -> Option<CodeableConcept>;
}
/// PaymentNotice Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentNotice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentNoticeMutators: DomainResourceMutators {
    /// Create a new PaymentNotice with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::payment_notice::PaymentNotice;
    /// use hl7_fhir_r4_core::traits::payment_notice::PaymentNoticeMutators;
    ///
    /// let resource = PaymentNotice::new();
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
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Reference) -> Self;
    /// Sets the response field and returns self for chaining.
    fn set_response(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the provider field and returns self for chaining.
    fn set_provider(self, value: Reference) -> Self;
    /// Sets the payment field and returns self for chaining.
    fn set_payment(self, value: Reference) -> Self;
    /// Sets the paymentDate field and returns self for chaining.
    fn set_payment_date(self, value: String) -> Self;
    /// Sets the payee field and returns self for chaining.
    fn set_payee(self, value: Reference) -> Self;
    /// Sets the recipient field and returns self for chaining.
    fn set_recipient(self, value: Reference) -> Self;
    /// Sets the amount field and returns self for chaining.
    fn set_amount(self, value: Money) -> Self;
    /// Sets the paymentStatus field and returns self for chaining.
    fn set_payment_status(self, value: CodeableConcept) -> Self;
}
/// PaymentNotice Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PaymentNotice
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PaymentNoticeExistence: DomainResourceExistence {
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
    /// Returns true if the request field is present (Some).
    fn has_request(&self) -> bool;
    /// Returns true if the response field is present (Some).
    fn has_response(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the provider field is present (Some).
    fn has_provider(&self) -> bool;
    /// Returns true if the payment field is present (Some).
    fn has_payment(&self) -> bool;
    /// Returns true if the payment_date field is present (Some).
    fn has_payment_date(&self) -> bool;
    /// Returns true if the payee field is present (Some).
    fn has_payee(&self) -> bool;
    /// Returns true if the recipient field is present (Some).
    fn has_recipient(&self) -> bool;
    /// Returns true if the amount field is present (Some).
    fn has_amount(&self) -> bool;
    /// Returns true if the payment_status field is present (Some).
    fn has_payment_status(&self) -> bool;
}
