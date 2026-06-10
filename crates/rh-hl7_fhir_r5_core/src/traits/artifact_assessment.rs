use crate::bindings::artifactassessment_disposition::ArtifactassessmentDisposition;
use crate::bindings::artifactassessment_workflow_status::ArtifactassessmentWorkflowStatus;
use crate::datatypes::identifier::Identifier;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::artifact_assessment::ArtifactAssessmentContent;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ArtifactAssessment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ArtifactAssessmentAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the content field.
    fn content(&self) -> &[ArtifactAssessmentContent];
    /// Returns a reference to the workflowStatus field.
    fn workflow_status(&self) -> Option<ArtifactassessmentWorkflowStatus>;
    /// Returns a reference to the disposition field.
    fn disposition(&self) -> Option<ArtifactassessmentDisposition>;
}
/// ArtifactAssessment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ArtifactAssessmentMutators: DomainResourceMutators {
    /// Create a new ArtifactAssessment with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::artifact_assessment::ArtifactAssessment;
    /// use rh_hl7_fhir_r5_core::traits::artifact_assessment::ArtifactAssessmentMutators;
    ///
    /// let resource = ArtifactAssessment::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the content field and returns self for chaining.
    fn set_content(self, value: Vec<ArtifactAssessmentContent>) -> Self;
    /// Adds an item to the content field and returns self for chaining.
    fn add_content(self, item: ArtifactAssessmentContent) -> Self;
    /// Sets the workflowStatus field and returns self for chaining.
    fn set_workflow_status(self, value: ArtifactassessmentWorkflowStatus) -> Self;
    /// Sets the disposition field and returns self for chaining.
    fn set_disposition(self, value: ArtifactassessmentDisposition) -> Self;
}
/// ArtifactAssessment Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ArtifactAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ArtifactAssessmentExistence: DomainResourceExistence {
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
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the cite_as field is present (Some).
    fn has_cite_as(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the artifact field is present (Some).
    fn has_artifact(&self) -> bool;
    /// Returns true if the content field is not empty.
    fn has_content(&self) -> bool;
    /// Returns true if the workflow_status field is present (Some).
    fn has_workflow_status(&self) -> bool;
    /// Returns true if the disposition field is present (Some).
    fn has_disposition(&self) -> bool;
}
