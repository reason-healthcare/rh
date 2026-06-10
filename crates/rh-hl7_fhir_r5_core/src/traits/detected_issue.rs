use crate::bindings::detectedissue_severity::DetectedissueSeverity;
use crate::bindings::detectedissue_status::DetectedissueStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::detected_issue::DetectedIssueEvidence;
use crate::resources::detected_issue::DetectedIssueMitigation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DetectedIssue Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DetectedIssue
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DetectedIssueAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> DetectedissueStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the severity field.
    fn severity(&self) -> Option<DetectedissueSeverity>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the implicated field.
    fn implicated(&self) -> &[Reference];
    /// Returns a reference to the evidence field.
    fn evidence(&self) -> &[DetectedIssueEvidence];
    /// Returns a reference to the detail field.
    fn detail(&self) -> Option<StringType>;
    /// Returns a reference to the reference field.
    fn reference(&self) -> Option<StringType>;
    /// Returns a reference to the mitigation field.
    fn mitigation(&self) -> &[DetectedIssueMitigation];
}
/// DetectedIssue Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DetectedIssue
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DetectedIssueMutators: DomainResourceMutators {
    /// Create a new DetectedIssue with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::detected_issue::DetectedIssue;
    /// use rh_hl7_fhir_r5_core::traits::detected_issue::DetectedIssueMutators;
    ///
    /// let resource = DetectedIssue::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DetectedissueStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the severity field and returns self for chaining.
    fn set_severity(self, value: DetectedissueSeverity) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the implicated field and returns self for chaining.
    fn set_implicated(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the implicated field and returns self for chaining.
    fn add_implicated(self, item: Reference) -> Self;
    /// Sets the evidence field and returns self for chaining.
    fn set_evidence(self, value: Vec<DetectedIssueEvidence>) -> Self;
    /// Adds an item to the evidence field and returns self for chaining.
    fn add_evidence(self, item: DetectedIssueEvidence) -> Self;
    /// Sets the detail field and returns self for chaining.
    fn set_detail(self, value: String) -> Self;
    /// Sets the reference field and returns self for chaining.
    fn set_reference(self, value: String) -> Self;
    /// Sets the mitigation field and returns self for chaining.
    fn set_mitigation(self, value: Vec<DetectedIssueMitigation>) -> Self;
    /// Adds an item to the mitigation field and returns self for chaining.
    fn add_mitigation(self, item: DetectedIssueMitigation) -> Self;
}
/// DetectedIssue Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DetectedIssue
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DetectedIssueExistence: DomainResourceExistence {
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
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the severity field is present (Some).
    fn has_severity(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the identified field is present (Some).
    fn has_identified(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the implicated field is not empty.
    fn has_implicated(&self) -> bool;
    /// Returns true if the evidence field is not empty.
    fn has_evidence(&self) -> bool;
    /// Returns true if the detail field is present (Some).
    fn has_detail(&self) -> bool;
    /// Returns true if the reference field is present (Some).
    fn has_reference(&self) -> bool;
    /// Returns true if the mitigation field is not empty.
    fn has_mitigation(&self) -> bool;
}
