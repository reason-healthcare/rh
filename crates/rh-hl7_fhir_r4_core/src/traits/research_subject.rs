use crate::bindings::research_subject_status::ResearchSubjectStatus;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ResearchSubject Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A physical entity which is the primary unit of operational and/or administrative interest in a study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchSubjectAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ResearchSubjectStatus;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the study field.
    fn study(&self) -> Reference;
    /// Returns a reference to the individual field.
    fn individual(&self) -> Reference;
    /// Returns a reference to the assignedArm field.
    fn assigned_arm(&self) -> Option<StringType>;
    /// Returns a reference to the actualArm field.
    fn actual_arm(&self) -> Option<StringType>;
    /// Returns a reference to the consent field.
    fn consent(&self) -> Option<Reference>;
}
/// ResearchSubject Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A physical entity which is the primary unit of operational and/or administrative interest in a study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchSubjectMutators: DomainResourceMutators {
    /// Create a new ResearchSubject with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::research_subject::ResearchSubject;
    /// use hl7_fhir_r4_core::traits::research_subject::ResearchSubjectMutators;
    ///
    /// let resource = ResearchSubject::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ResearchSubjectStatus) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the study field and returns self for chaining.
    fn set_study(self, value: Reference) -> Self;
    /// Sets the individual field and returns self for chaining.
    fn set_individual(self, value: Reference) -> Self;
    /// Sets the assignedArm field and returns self for chaining.
    fn set_assigned_arm(self, value: String) -> Self;
    /// Sets the actualArm field and returns self for chaining.
    fn set_actual_arm(self, value: String) -> Self;
    /// Sets the consent field and returns self for chaining.
    fn set_consent(self, value: Reference) -> Self;
}
/// ResearchSubject Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A physical entity which is the primary unit of operational and/or administrative interest in a study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchSubjectExistence: DomainResourceExistence {
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
    /// Returns true if the study field is present (Some).
    fn has_study(&self) -> bool;
    /// Returns true if the individual field is present (Some).
    fn has_individual(&self) -> bool;
    /// Returns true if the assigned_arm field is present (Some).
    fn has_assigned_arm(&self) -> bool;
    /// Returns true if the actual_arm field is present (Some).
    fn has_actual_arm(&self) -> bool;
    /// Returns true if the consent field is present (Some).
    fn has_consent(&self) -> bool;
}
