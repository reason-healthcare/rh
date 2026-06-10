use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::research_subject::ResearchSubjectProgress;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ResearchSubject Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchSubjectAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the progress field.
    fn progress(&self) -> &[ResearchSubjectProgress];
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the study field.
    fn study(&self) -> Reference;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the assignedComparisonGroup field.
    fn assigned_comparison_group(&self) -> Option<StringType>;
    /// Returns a reference to the actualComparisonGroup field.
    fn actual_comparison_group(&self) -> Option<StringType>;
    /// Returns a reference to the consent field.
    fn consent(&self) -> &[Reference];
}
/// ResearchSubject Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::research_subject::ResearchSubject;
    /// use rh_hl7_fhir_r5_core::traits::research_subject::ResearchSubjectMutators;
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
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the progress field and returns self for chaining.
    fn set_progress(self, value: Vec<ResearchSubjectProgress>) -> Self;
    /// Adds an item to the progress field and returns self for chaining.
    fn add_progress(self, item: ResearchSubjectProgress) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the study field and returns self for chaining.
    fn set_study(self, value: Reference) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the assignedComparisonGroup field and returns self for chaining.
    fn set_assigned_comparison_group(self, value: String) -> Self;
    /// Sets the actualComparisonGroup field and returns self for chaining.
    fn set_actual_comparison_group(self, value: String) -> Self;
    /// Sets the consent field and returns self for chaining.
    fn set_consent(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the consent field and returns self for chaining.
    fn add_consent(self, item: Reference) -> Self;
}
/// ResearchSubject Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ResearchSubject
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ResearchSubjectExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the progress field is not empty.
    fn has_progress(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the study field is present (Some).
    fn has_study(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the assigned_comparison_group field is present (Some).
    fn has_assigned_comparison_group(&self) -> bool;
    /// Returns true if the actual_comparison_group field is present (Some).
    fn has_actual_comparison_group(&self) -> bool;
    /// Returns true if the consent field is not empty.
    fn has_consent(&self) -> bool;
}
