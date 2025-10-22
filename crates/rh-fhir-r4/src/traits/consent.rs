use crate::bindings::consent_state_codes::ConsentStateCodes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::consent::ConsentPolicy;
use crate::resources::consent::ConsentProvision;
use crate::resources::consent::ConsentVerification;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Consent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConsentAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ConsentStateCodes;
    /// Returns a reference to the scope field.
    fn scope(&self) -> CodeableConcept;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the dateTime field.
    fn date_time(&self) -> Option<DateTimeType>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[Reference];
    /// Returns a reference to the organization field.
    fn organization(&self) -> &[Reference];
    /// Returns a reference to the policy field.
    fn policy(&self) -> &[ConsentPolicy];
    /// Returns a reference to the policyRule field.
    fn policy_rule(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the verification field.
    fn verification(&self) -> &[ConsentVerification];
    /// Returns a reference to the provision field.
    fn provision(&self) -> Option<ConsentProvision>;
}
/// Consent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConsentMutators: DomainResourceMutators {
    /// Create a new Consent with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::consent::Consent;
    /// use hl7_fhir_r4_core::traits::consent::ConsentMutators;
    ///
    /// let resource = Consent::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ConsentStateCodes) -> Self;
    /// Sets the scope field and returns self for chaining.
    fn set_scope(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the dateTime field and returns self for chaining.
    fn set_date_time(self, value: String) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: Reference) -> Self;
    /// Sets the organization field and returns self for chaining.
    fn set_organization(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the organization field and returns self for chaining.
    fn add_organization(self, item: Reference) -> Self;
    /// Sets the policy field and returns self for chaining.
    fn set_policy(self, value: Vec<ConsentPolicy>) -> Self;
    /// Adds an item to the policy field and returns self for chaining.
    fn add_policy(self, item: ConsentPolicy) -> Self;
    /// Sets the policyRule field and returns self for chaining.
    fn set_policy_rule(self, value: CodeableConcept) -> Self;
    /// Sets the verification field and returns self for chaining.
    fn set_verification(self, value: Vec<ConsentVerification>) -> Self;
    /// Adds an item to the verification field and returns self for chaining.
    fn add_verification(self, item: ConsentVerification) -> Self;
    /// Sets the provision field and returns self for chaining.
    fn set_provision(self, value: ConsentProvision) -> Self;
}
/// Consent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConsentExistence: DomainResourceExistence {
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
    /// Returns true if the scope field is present (Some).
    fn has_scope(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the date_time field is present (Some).
    fn has_date_time(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the organization field is not empty.
    fn has_organization(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the policy field is not empty.
    fn has_policy(&self) -> bool;
    /// Returns true if the policy_rule field is present (Some).
    fn has_policy_rule(&self) -> bool;
    /// Returns true if the verification field is not empty.
    fn has_verification(&self) -> bool;
    /// Returns true if the provision field is present (Some).
    fn has_provision(&self) -> bool;
}
