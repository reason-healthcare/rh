use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::regulated_authorization::RegulatedAuthorizationCase;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// RegulatedAuthorization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RegulatedAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RegulatedAuthorizationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the region field.
    fn region(&self) -> &[CodeableConcept];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the validityPeriod field.
    fn validity_period(&self) -> Option<Period>;
    /// Returns a reference to the indication field.
    fn indication(&self) -> &[CodeableReference];
    /// Returns a reference to the intendedUse field.
    fn intended_use(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the basis field.
    fn basis(&self) -> &[CodeableConcept];
    /// Returns a reference to the holder field.
    fn holder(&self) -> Option<Reference>;
    /// Returns a reference to the regulator field.
    fn regulator(&self) -> Option<Reference>;
    /// Returns a reference to the attachedDocument field.
    fn attached_document(&self) -> &[Reference];
    /// Returns a reference to the case field.
    fn case(&self) -> Option<RegulatedAuthorizationCase>;
}
/// RegulatedAuthorization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RegulatedAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RegulatedAuthorizationMutators: DomainResourceMutators {
    /// Create a new RegulatedAuthorization with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::regulated_authorization::RegulatedAuthorization;
    /// use rh_hl7_fhir_r5_core::traits::regulated_authorization::RegulatedAuthorizationMutators;
    ///
    /// let resource = RegulatedAuthorization::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the region field and returns self for chaining.
    fn set_region(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the region field and returns self for chaining.
    fn add_region(self, item: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the validityPeriod field and returns self for chaining.
    fn set_validity_period(self, value: Period) -> Self;
    /// Sets the indication field and returns self for chaining.
    fn set_indication(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the indication field and returns self for chaining.
    fn add_indication(self, item: CodeableReference) -> Self;
    /// Sets the intendedUse field and returns self for chaining.
    fn set_intended_use(self, value: CodeableConcept) -> Self;
    /// Sets the basis field and returns self for chaining.
    fn set_basis(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the basis field and returns self for chaining.
    fn add_basis(self, item: CodeableConcept) -> Self;
    /// Sets the holder field and returns self for chaining.
    fn set_holder(self, value: Reference) -> Self;
    /// Sets the regulator field and returns self for chaining.
    fn set_regulator(self, value: Reference) -> Self;
    /// Sets the attachedDocument field and returns self for chaining.
    fn set_attached_document(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the attachedDocument field and returns self for chaining.
    fn add_attached_document(self, item: Reference) -> Self;
    /// Sets the case field and returns self for chaining.
    fn set_case(self, value: RegulatedAuthorizationCase) -> Self;
}
/// RegulatedAuthorization Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RegulatedAuthorization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RegulatedAuthorization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait RegulatedAuthorizationExistence: DomainResourceExistence {
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
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the region field is not empty.
    fn has_region(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the validity_period field is present (Some).
    fn has_validity_period(&self) -> bool;
    /// Returns true if the indication field is not empty.
    fn has_indication(&self) -> bool;
    /// Returns true if the intended_use field is present (Some).
    fn has_intended_use(&self) -> bool;
    /// Returns true if the basis field is not empty.
    fn has_basis(&self) -> bool;
    /// Returns true if the holder field is present (Some).
    fn has_holder(&self) -> bool;
    /// Returns true if the regulator field is present (Some).
    fn has_regulator(&self) -> bool;
    /// Returns true if the attached_document field is not empty.
    fn has_attached_document(&self) -> bool;
    /// Returns true if the case field is present (Some).
    fn has_case(&self) -> bool;
}
