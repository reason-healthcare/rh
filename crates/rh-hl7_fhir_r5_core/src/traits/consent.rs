use crate::bindings::consent_provision_type::ConsentProvisionType;
use crate::bindings::consent_state_codes::ConsentStateCodes;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date::DateType;
use crate::resources::consent::ConsentPolicybasis;
use crate::resources::consent::ConsentProvision;
use crate::resources::consent::ConsentVerification;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Consent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConsentAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ConsentStateCodes;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateType>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the grantor field.
    fn grantor(&self) -> &[Reference];
    /// Returns a reference to the grantee field.
    fn grantee(&self) -> &[Reference];
    /// Returns a reference to the manager field.
    fn manager(&self) -> &[Reference];
    /// Returns a reference to the controller field.
    fn controller(&self) -> &[Reference];
    /// Returns a reference to the sourceAttachment field.
    fn source_attachment(&self) -> &[Attachment];
    /// Returns a reference to the sourceReference field.
    fn source_reference(&self) -> &[Reference];
    /// Returns a reference to the regulatoryBasis field.
    fn regulatory_basis(&self) -> &[CodeableConcept];
    /// Returns a reference to the policyBasis field.
    fn policy_basis(&self) -> Option<ConsentPolicybasis>;
    /// Returns a reference to the policyText field.
    fn policy_text(&self) -> &[Reference];
    /// Returns a reference to the verification field.
    fn verification(&self) -> &[ConsentVerification];
    /// Returns a reference to the decision field.
    fn decision(&self) -> Option<ConsentProvisionType>;
    /// Returns a reference to the provision field.
    fn provision(&self) -> &[ConsentProvision];
}
/// Consent Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::consent::Consent;
    /// use rh_hl7_fhir_r5_core::traits::consent::ConsentMutators;
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
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the grantor field and returns self for chaining.
    fn set_grantor(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the grantor field and returns self for chaining.
    fn add_grantor(self, item: Reference) -> Self;
    /// Sets the grantee field and returns self for chaining.
    fn set_grantee(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the grantee field and returns self for chaining.
    fn add_grantee(self, item: Reference) -> Self;
    /// Sets the manager field and returns self for chaining.
    fn set_manager(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manager field and returns self for chaining.
    fn add_manager(self, item: Reference) -> Self;
    /// Sets the controller field and returns self for chaining.
    fn set_controller(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the controller field and returns self for chaining.
    fn add_controller(self, item: Reference) -> Self;
    /// Sets the sourceAttachment field and returns self for chaining.
    fn set_source_attachment(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the sourceAttachment field and returns self for chaining.
    fn add_source_attachment(self, item: Attachment) -> Self;
    /// Sets the sourceReference field and returns self for chaining.
    fn set_source_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the sourceReference field and returns self for chaining.
    fn add_source_reference(self, item: Reference) -> Self;
    /// Sets the regulatoryBasis field and returns self for chaining.
    fn set_regulatory_basis(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the regulatoryBasis field and returns self for chaining.
    fn add_regulatory_basis(self, item: CodeableConcept) -> Self;
    /// Sets the policyBasis field and returns self for chaining.
    fn set_policy_basis(self, value: ConsentPolicybasis) -> Self;
    /// Sets the policyText field and returns self for chaining.
    fn set_policy_text(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the policyText field and returns self for chaining.
    fn add_policy_text(self, item: Reference) -> Self;
    /// Sets the verification field and returns self for chaining.
    fn set_verification(self, value: Vec<ConsentVerification>) -> Self;
    /// Adds an item to the verification field and returns self for chaining.
    fn add_verification(self, item: ConsentVerification) -> Self;
    /// Sets the decision field and returns self for chaining.
    fn set_decision(self, value: ConsentProvisionType) -> Self;
    /// Sets the provision field and returns self for chaining.
    fn set_provision(self, value: Vec<ConsentProvision>) -> Self;
    /// Adds an item to the provision field and returns self for chaining.
    fn add_provision(self, item: ConsentProvision) -> Self;
}
/// Consent Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Consent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Consent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConsentExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the grantor field is not empty.
    fn has_grantor(&self) -> bool;
    /// Returns true if the grantee field is not empty.
    fn has_grantee(&self) -> bool;
    /// Returns true if the manager field is not empty.
    fn has_manager(&self) -> bool;
    /// Returns true if the controller field is not empty.
    fn has_controller(&self) -> bool;
    /// Returns true if the source_attachment field is not empty.
    fn has_source_attachment(&self) -> bool;
    /// Returns true if the source_reference field is not empty.
    fn has_source_reference(&self) -> bool;
    /// Returns true if the regulatory_basis field is not empty.
    fn has_regulatory_basis(&self) -> bool;
    /// Returns true if the policy_basis field is present (Some).
    fn has_policy_basis(&self) -> bool;
    /// Returns true if the policy_text field is not empty.
    fn has_policy_text(&self) -> bool;
    /// Returns true if the verification field is not empty.
    fn has_verification(&self) -> bool;
    /// Returns true if the decision field is present (Some).
    fn has_decision(&self) -> bool;
    /// Returns true if the provision field is not empty.
    fn has_provision(&self) -> bool;
}
